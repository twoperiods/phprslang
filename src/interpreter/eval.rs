use crate::parser::*;
use crate::interpreter::value::*;
use std::collections::HashMap;
use sha1::{Digest, Sha1};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

enum SocketWrapper {
    Tcp(TcpStream),
    Tls(Box<native_tls::TlsStream<TcpStream>>),
}

impl Read for SocketWrapper {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            SocketWrapper::Tcp(s) => s.read(buf),
            SocketWrapper::Tls(s) => s.read(buf),
        }
    }
}

impl Write for SocketWrapper {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            SocketWrapper::Tcp(s) => s.write(buf),
            SocketWrapper::Tls(s) => s.write(buf),
        }
    }
    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            SocketWrapper::Tcp(s) => s.flush(),
            SocketWrapper::Tls(s) => s.flush(),
        }
    }
}

#[derive(Clone)]
struct AsyncCurlResponse {
    status: i64,
    headers: String,
    body: String,
    error: String,
}

struct AsyncCurlState {
    done: bool,
    result: Option<AsyncCurlResponse>,
}

pub struct Interpreter {
    pub env: Environment,
    pub functions: Vec<FunctionValue>,
    sockets: HashMap<i64, SocketWrapper>,
    socket_counter: i64,
    listeners: HashMap<i64, TcpListener>,
    listener_counter: i64,
    async_handles: HashMap<i64, Arc<Mutex<AsyncCurlState>>>,
    async_counter: i64,
    cors_origin: String,
    cors_methods: String,
    cors_headers: String,
}

// Helper: convert a Value to a string key for hashing/comparison
fn value_to_key(val: &Value) -> String {
    match val {
        Value::Int(n) => format!("i:{}", n),
        Value::Float(f) => format!("f:{}", f),
        Value::String_(s) => format!("s:{}", s),
        Value::Bool(b) => format!("b:{}", b),
        Value::Null => "n:null".to_string(),
        Value::Array(_) => "a:array".to_string(),
        Value::Dict(_) => "d:dict".to_string(),
        Value::Function(_) => "u:function".to_string(),
        Value::NativeFunction(_) => "u:native".to_string(),
    }
}

impl Interpreter {
    pub fn new() -> Self {
        let mut interpreter = Self {
            env: Environment::new(),
            functions: Vec::new(),
            sockets: HashMap::new(),
            socket_counter: 0,
            listeners: HashMap::new(),
            listener_counter: 0,
            async_handles: HashMap::new(),
            async_counter: 0,
            cors_origin: "*".to_string(),
            cors_methods: "GET,POST,PUT,DELETE,PATCH,OPTIONS".to_string(),
            cors_headers: "Content-Type,Authorization".to_string(),
        };
        interpreter.register_builtins();
        interpreter
    }

    fn register_builtins(&mut self) {
        // strlen
        self.functions.push(FunctionValue {
            name: "strlen".into(),
            params: vec![FnParam { name: "s".into(), ty: Some(TypeAnnotation::String_), by_ref: false, by_mut_ref: false }],
            body: Box::new(Stmt::Block(vec![])),
            closure_env: None,
        });
        // count (for arrays)
        self.functions.push(FunctionValue {
            name: "count".into(),
            params: vec![FnParam { name: "arr".into(), ty: None, by_ref: false, by_mut_ref: false }],
            body: Box::new(Stmt::Block(vec![])),
            closure_env: None,
        });
        // array_push
        self.functions.push(FunctionValue {
            name: "array_push".into(),
            params: vec![
                FnParam { name: "arr".into(), ty: None, by_ref: false, by_mut_ref: false },
                FnParam { name: "val".into(), ty: None, by_ref: false, by_mut_ref: false },
            ],
            body: Box::new(Stmt::Block(vec![])),
            closure_env: None,
        });
        // curl - sync HTTP client
        self.functions.push(FunctionValue {
            name: "curl".into(),
            params: vec![
                FnParam { name: "url".into(), ty: Some(TypeAnnotation::String_), by_ref: false, by_mut_ref: false },
                FnParam { name: "options".into(), ty: None, by_ref: false, by_mut_ref: false },
            ],
            body: Box::new(Stmt::Block(vec![])),
            closure_env: None,
        });
        // curl_async - async HTTP client
        self.functions.push(FunctionValue {
            name: "curl_async".into(),
            params: vec![
                FnParam { name: "url".into(), ty: Some(TypeAnnotation::String_), by_ref: false, by_mut_ref: false },
                FnParam { name: "options".into(), ty: None, by_ref: false, by_mut_ref: false },
            ],
            body: Box::new(Stmt::Block(vec![])),
            closure_env: None,
        });
        // curl_wait - wait for async curl
        self.functions.push(FunctionValue {
            name: "curl_wait".into(),
            params: vec![
                FnParam { name: "handle".into(), ty: Some(TypeAnnotation::Int), by_ref: false, by_mut_ref: false },
            ],
            body: Box::new(Stmt::Block(vec![])),
            closure_env: None,
        });
        // curl_is_done - check if async curl is done
        self.functions.push(FunctionValue {
            name: "curl_is_done".into(),
            params: vec![
                FnParam { name: "handle".into(), ty: Some(TypeAnnotation::Int), by_ref: false, by_mut_ref: false },
            ],
            body: Box::new(Stmt::Block(vec![])),
            closure_env: None,
        });
        // chr
        self.functions.push(FunctionValue {
            name: "chr".into(),
            params: vec![FnParam { name: "codepoint".into(), ty: Some(TypeAnnotation::Int), by_ref: false, by_mut_ref: false }],
            body: Box::new(Stmt::Block(vec![])),
            closure_env: None,
        });
        // ord
        self.functions.push(FunctionValue {
            name: "ord".into(),
            params: vec![FnParam { name: "char".into(), ty: Some(TypeAnnotation::String_), by_ref: false, by_mut_ref: false }],
            body: Box::new(Stmt::Block(vec![])),
            closure_env: None,
        });
        // addslashes
        self.functions.push(FunctionValue {
            name: "addslashes".into(),
            params: vec![FnParam { name: "str".into(), ty: Some(TypeAnnotation::String_), by_ref: false, by_mut_ref: false }],
            body: Box::new(Stmt::Block(vec![])),
            closure_env: None,
        });
        // stripslashes
        self.functions.push(FunctionValue {
            name: "stripslashes".into(),
            params: vec![FnParam { name: "str".into(), ty: Some(TypeAnnotation::String_), by_ref: false, by_mut_ref: false }],
            body: Box::new(Stmt::Block(vec![])),
            closure_env: None,
        });
        // copy
        self.functions.push(FunctionValue {
            name: "copy".into(),
            params: vec![
                FnParam { name: "source".into(), ty: Some(TypeAnnotation::String_), by_ref: false, by_mut_ref: false },
                FnParam { name: "dest".into(), ty: Some(TypeAnnotation::String_), by_ref: false, by_mut_ref: false },
            ],
            body: Box::new(Stmt::Block(vec![])),
            closure_env: None,
        });
        // rename
        self.functions.push(FunctionValue {
            name: "rename".into(),
            params: vec![
                FnParam { name: "old".into(), ty: Some(TypeAnnotation::String_), by_ref: false, by_mut_ref: false },
                FnParam { name: "new".into(), ty: Some(TypeAnnotation::String_), by_ref: false, by_mut_ref: false },
            ],
            body: Box::new(Stmt::Block(vec![])),
            closure_env: None,
        });
        // filesize
        self.functions.push(FunctionValue {
            name: "filesize".into(),
            params: vec![FnParam { name: "path".into(), ty: Some(TypeAnnotation::String_), by_ref: false, by_mut_ref: false }],
            body: Box::new(Stmt::Block(vec![])),
            closure_env: None,
        });
        // filemtime
        self.functions.push(FunctionValue {
            name: "filemtime".into(),
            params: vec![FnParam { name: "path".into(), ty: Some(TypeAnnotation::String_), by_ref: false, by_mut_ref: false }],
            body: Box::new(Stmt::Block(vec![])),
            closure_env: None,
        });
        // pathinfo
        self.functions.push(FunctionValue {
            name: "pathinfo".into(),
            params: vec![FnParam { name: "path".into(), ty: Some(TypeAnnotation::String_), by_ref: false, by_mut_ref: false }],
            body: Box::new(Stmt::Block(vec![])),
            closure_env: None,
        });
        // move_uploaded_file
        self.functions.push(FunctionValue {
            name: "move_uploaded_file".into(),
            params: vec![
                FnParam { name: "tmp".into(), ty: Some(TypeAnnotation::String_), by_ref: false, by_mut_ref: false },
                FnParam { name: "dest".into(), ty: Some(TypeAnnotation::String_), by_ref: false, by_mut_ref: false },
            ],
            body: Box::new(Stmt::Block(vec![])),
            closure_env: None,
        });
        // password_hash
        self.functions.push(FunctionValue {
            name: "password_hash".into(),
            params: vec![
                FnParam { name: "password".into(), ty: Some(TypeAnnotation::String_), by_ref: false, by_mut_ref: false },
                FnParam { name: "algo".into(), ty: Some(TypeAnnotation::String_), by_ref: false, by_mut_ref: false },
            ],
            body: Box::new(Stmt::Block(vec![])),
            closure_env: None,
        });
        // password_verify
        self.functions.push(FunctionValue {
            name: "password_verify".into(),
            params: vec![
                FnParam { name: "password".into(), ty: Some(TypeAnnotation::String_), by_ref: false, by_mut_ref: false },
                FnParam { name: "hash".into(), ty: Some(TypeAnnotation::String_), by_ref: false, by_mut_ref: false },
            ],
            body: Box::new(Stmt::Block(vec![])),
            closure_env: None,
        });
        // random_bytes
        self.functions.push(FunctionValue {
            name: "random_bytes".into(),
            params: vec![FnParam { name: "length".into(), ty: Some(TypeAnnotation::Int), by_ref: false, by_mut_ref: false }],
            body: Box::new(Stmt::Block(vec![])),
            closure_env: None,
        });
        // random_int
        self.functions.push(FunctionValue {
            name: "random_int".into(),
            params: vec![
                FnParam { name: "min".into(), ty: Some(TypeAnnotation::Int), by_ref: false, by_mut_ref: false },
                FnParam { name: "max".into(), ty: Some(TypeAnnotation::Int), by_ref: false, by_mut_ref: false },
            ],
            body: Box::new(Stmt::Block(vec![])),
            closure_env: None,
        });
        // array_chunk
        self.functions.push(FunctionValue {
            name: "array_chunk".into(),
            params: vec![
                FnParam { name: "arr".into(), ty: None, by_ref: false, by_mut_ref: false },
                FnParam { name: "size".into(), ty: Some(TypeAnnotation::Int), by_ref: false, by_mut_ref: false },
                FnParam { name: "preserve_keys".into(), ty: None, by_ref: false, by_mut_ref: false },
            ],
            body: Box::new(Stmt::Block(vec![])),
            closure_env: None,
        });
        // array_count_values
        self.functions.push(FunctionValue {
            name: "array_count_values".into(),
            params: vec![FnParam { name: "arr".into(), ty: None, by_ref: false, by_mut_ref: false }],
            body: Box::new(Stmt::Block(vec![])),
            closure_env: None,
        });
        // array_product
        self.functions.push(FunctionValue {
            name: "array_product".into(),
            params: vec![FnParam { name: "arr".into(), ty: None, by_ref: false, by_mut_ref: false }],
            body: Box::new(Stmt::Block(vec![])),
            closure_env: None,
        });
        // array_intersect
        self.functions.push(FunctionValue {
            name: "array_intersect".into(),
            params: vec![
                FnParam { name: "arr1".into(), ty: None, by_ref: false, by_mut_ref: false },
                FnParam { name: "arr2".into(), ty: None, by_ref: false, by_mut_ref: false },
            ],
            body: Box::new(Stmt::Block(vec![])),
            closure_env: None,
        });
        // intval
        self.functions.push(FunctionValue {
            name: "intval".into(),
            params: vec![FnParam { name: "val".into(), ty: None, by_ref: false, by_mut_ref: false }],
            body: Box::new(Stmt::Block(vec![])),
            closure_env: None,
        });
        // floatval
        self.functions.push(FunctionValue {
            name: "floatval".into(),
            params: vec![FnParam { name: "val".into(), ty: None, by_ref: false, by_mut_ref: false }],
            body: Box::new(Stmt::Block(vec![])),
            closure_env: None,
        });
        // strval
        self.functions.push(FunctionValue {
            name: "strval".into(),
            params: vec![FnParam { name: "val".into(), ty: None, by_ref: false, by_mut_ref: false }],
            body: Box::new(Stmt::Block(vec![])),
            closure_env: None,
        });
        // boolval
        self.functions.push(FunctionValue {
            name: "boolval".into(),
            params: vec![FnParam { name: "val".into(), ty: None, by_ref: false, by_mut_ref: false }],
            body: Box::new(Stmt::Block(vec![])),
            closure_env: None,
        });
        // str_pad
        self.functions.push(FunctionValue {
            name: "str_pad".into(),
            params: vec![
                FnParam { name: "input".into(), ty: Some(TypeAnnotation::String_), by_ref: false, by_mut_ref: false },
                FnParam { name: "length".into(), ty: Some(TypeAnnotation::Int), by_ref: false, by_mut_ref: false },
            ],
            body: Box::new(Stmt::Block(vec![])),
            closure_env: None,
        });
        // wordwrap
        self.functions.push(FunctionValue {
            name: "wordwrap".into(),
            params: vec![FnParam { name: "str".into(), ty: Some(TypeAnnotation::String_), by_ref: false, by_mut_ref: false }],
            body: Box::new(Stmt::Block(vec![])),
            closure_env: None,
        });
        // str_word_count
        self.functions.push(FunctionValue {
            name: "str_word_count".into(),
            params: vec![FnParam { name: "str".into(), ty: Some(TypeAnnotation::String_), by_ref: false, by_mut_ref: false }],
            body: Box::new(Stmt::Block(vec![])),
            closure_env: None,
        });
        // chunk_split
        self.functions.push(FunctionValue {
            name: "chunk_split".into(),
            params: vec![FnParam { name: "body".into(), ty: Some(TypeAnnotation::String_), by_ref: false, by_mut_ref: false }],
            body: Box::new(Stmt::Block(vec![])),
            closure_env: None,
        });
        // array_splice
        self.functions.push(FunctionValue {
            name: "array_splice".into(),
            params: vec![
                FnParam { name: "arr".into(), ty: None, by_ref: false, by_mut_ref: false },
                FnParam { name: "offset".into(), ty: Some(TypeAnnotation::Int), by_ref: false, by_mut_ref: false },
                FnParam { name: "length".into(), ty: Some(TypeAnnotation::Int), by_ref: false, by_mut_ref: false },
            ],
            body: Box::new(Stmt::Block(vec![])),
            closure_env: None,
        });
        // array_pad
        self.functions.push(FunctionValue {
            name: "array_pad".into(),
            params: vec![
                FnParam { name: "arr".into(), ty: None, by_ref: false, by_mut_ref: false },
                FnParam { name: "size".into(), ty: Some(TypeAnnotation::Int), by_ref: false, by_mut_ref: false },
                FnParam { name: "value".into(), ty: None, by_ref: false, by_mut_ref: false },
            ],
            body: Box::new(Stmt::Block(vec![])),
            closure_env: None,
        });
        // array_key_first
        self.functions.push(FunctionValue {
            name: "array_key_first".into(),
            params: vec![FnParam { name: "arr".into(), ty: None, by_ref: false, by_mut_ref: false }],
            body: Box::new(Stmt::Block(vec![])),
            closure_env: None,
        });
        // array_key_last
        self.functions.push(FunctionValue {
            name: "array_key_last".into(),
            params: vec![FnParam { name: "arr".into(), ty: None, by_ref: false, by_mut_ref: false }],
            body: Box::new(Stmt::Block(vec![])),
            closure_env: None,
        });
        // array_is_list
        self.functions.push(FunctionValue {
            name: "array_is_list".into(),
            params: vec![FnParam { name: "arr".into(), ty: None, by_ref: false, by_mut_ref: false }],
            body: Box::new(Stmt::Block(vec![])),
            closure_env: None,
        });
        // fmod
        self.functions.push(FunctionValue {
            name: "fmod".into(),
            params: vec![
                FnParam { name: "x".into(), ty: Some(TypeAnnotation::Float), by_ref: false, by_mut_ref: false },
                FnParam { name: "y".into(), ty: Some(TypeAnnotation::Float), by_ref: false, by_mut_ref: false },
            ],
            body: Box::new(Stmt::Block(vec![])),
            closure_env: None,
        });
        // intdiv
        self.functions.push(FunctionValue {
            name: "intdiv".into(),
            params: vec![
                FnParam { name: "a".into(), ty: Some(TypeAnnotation::Int), by_ref: false, by_mut_ref: false },
                FnParam { name: "b".into(), ty: Some(TypeAnnotation::Int), by_ref: false, by_mut_ref: false },
            ],
            body: Box::new(Stmt::Block(vec![])),
            closure_env: None,
        });
        // checkdate
        self.functions.push(FunctionValue {
            name: "checkdate".into(),
            params: vec![
                FnParam { name: "month".into(), ty: Some(TypeAnnotation::Int), by_ref: false, by_mut_ref: false },
                FnParam { name: "day".into(), ty: Some(TypeAnnotation::Int), by_ref: false, by_mut_ref: false },
                FnParam { name: "year".into(), ty: Some(TypeAnnotation::Int), by_ref: false, by_mut_ref: false },
            ],
            body: Box::new(Stmt::Block(vec![])),
            closure_env: None,
        });
        // mktime
        self.functions.push(FunctionValue {
            name: "mktime".into(),
            params: vec![
                FnParam { name: "hour".into(), ty: Some(TypeAnnotation::Int), by_ref: false, by_mut_ref: false },
                FnParam { name: "min".into(), ty: Some(TypeAnnotation::Int), by_ref: false, by_mut_ref: false },
                FnParam { name: "sec".into(), ty: Some(TypeAnnotation::Int), by_ref: false, by_mut_ref: false },
                FnParam { name: "month".into(), ty: Some(TypeAnnotation::Int), by_ref: false, by_mut_ref: false },
                FnParam { name: "day".into(), ty: Some(TypeAnnotation::Int), by_ref: false, by_mut_ref: false },
                FnParam { name: "year".into(), ty: Some(TypeAnnotation::Int), by_ref: false, by_mut_ref: false },
            ],
            body: Box::new(Stmt::Block(vec![])),
            closure_env: None,
        });
        // printf
        self.functions.push(FunctionValue {
            name: "printf".into(),
            params: vec![FnParam { name: "format".into(), ty: Some(TypeAnnotation::String_), by_ref: false, by_mut_ref: false }],
            body: Box::new(Stmt::Block(vec![])),
            closure_env: None,
        });
        // str_starts_with
        self.functions.push(FunctionValue {
            name: "str_starts_with".into(),
            params: vec![
                FnParam { name: "haystack".into(), ty: Some(TypeAnnotation::String_), by_ref: false, by_mut_ref: false },
                FnParam { name: "needle".into(), ty: Some(TypeAnnotation::String_), by_ref: false, by_mut_ref: false },
            ],
            body: Box::new(Stmt::Block(vec![])),
            closure_env: None,
        });
        // str_ends_with
        self.functions.push(FunctionValue {
            name: "str_ends_with".into(),
            params: vec![
                FnParam { name: "haystack".into(), ty: Some(TypeAnnotation::String_), by_ref: false, by_mut_ref: false },
                FnParam { name: "needle".into(), ty: Some(TypeAnnotation::String_), by_ref: false, by_mut_ref: false },
            ],
            body: Box::new(Stmt::Block(vec![])),
            closure_env: None,
        });
        // phprs_client_ip
        self.functions.push(FunctionValue {
            name: "phprs_client_ip".into(),
            params: vec![FnParam { name: "fd".into(), ty: Some(TypeAnnotation::Int), by_ref: false, by_mut_ref: false }],
            body: Box::new(Stmt::Block(vec![])),
            closure_env: None,
        });
    }

    fn call_builtin(&mut self, name: &str, args: Vec<Value>) -> Result<Value, String> {
        match name {
            // ---- User-facing builtins ----
            "strlen" => {
                if args.len() != 1 { return Err("strlen() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(s) => Ok(Value::Int(s.chars().count() as i64)),
                    _ => Err("strlen() expects a string".into()),
                }
            }
            "count" => {
                if args.len() != 1 { return Err("count() expects 1 argument".into()); }
                match &args[0] {
                    Value::Array(a) => Ok(Value::Int(a.len() as i64)),
                    Value::Dict(d) => Ok(Value::Int(d.len() as i64)),
                    _ => Err("count() expects an array or dict".into()),
                }
            }
            "array_push" => {
                if args.len() < 2 { return Err("array_push() expects at least 2 arguments".into()); }
                match &args[0] {
                    Value::Array(items) => {
                        let mut new_items = items.clone();
                        new_items.extend(args[1..].iter().cloned());
                        Ok(Value::Array(new_items))
                    }
                    _ => Err("array_push() expects an array as first argument".into()),
                }
            }
            "array_pop" => {
                if args.len() != 1 { return Err("array_pop() expects 1 argument".into()); }
                match &args[0] {
                    Value::Array(items) => {
                        if items.is_empty() { return Ok(Value::Null); }
                        let mut new_items = items.clone();
                        let last = new_items.pop().unwrap();
                        Ok(last)
                    }
                    _ => Err("array_pop() expects an array".into()),
                }
            }
            "array_shift" => {
                if args.len() != 1 { return Err("array_shift() expects 1 argument".into()); }
                match &args[0] {
                    Value::Array(items) => {
                        if items.is_empty() { return Ok(Value::Null); }
                        Ok(items[0].clone())
                    }
                    _ => Err("array_shift() expects an array".into()),
                }
            }
            "array_unshift" => {
                if args.len() < 2 { return Err("array_unshift() expects at least 2 arguments".into()); }
                match &args[0] {
                    Value::Array(items) => {
                        let mut new_front: Vec<Value> = args[1..].iter().cloned().collect();
                        new_front.extend(items.clone());
                        Ok(Value::Array(new_front))
                    }
                    _ => Err("array_unshift() expects an array as first argument".into()),
                }
            }
            "array_keys" => {
                if args.len() != 1 { return Err("array_keys() expects 1 argument".into()); }
                match &args[0] {
                    Value::Dict(map) => {
                        Ok(Value::Array(map.keys().map(|k| Value::String_(k.clone())).collect()))
                    }
                    Value::Array(items) => {
                        Ok(Value::Array(items.iter().enumerate()
                            .map(|(i, _)| Value::Int(i as i64)).collect()))
                    }
                    _ => Err("array_keys() expects an array or dict".into()),
                }
            }
            "array_values" => {
                if args.len() != 1 { return Err("array_values() expects 1 argument".into()); }
                match &args[0] {
                    Value::Dict(map) => {
                        Ok(Value::Array(map.values().cloned().collect()))
                    }
                    Value::Array(items) => Ok(Value::Array(items.clone())),
                    _ => Err("array_values() expects an array or dict".into()),
                }
            }
            "array_merge" => {
                if args.len() < 2 { return Err("array_merge() expects at least 2 arguments".into()); }
                let mut result = Vec::new();
                for a in &args {
                    match a {
                        Value::Array(items) => result.extend(items.clone()),
                        _ => result.push(a.clone()),
                    }
                }
                Ok(Value::Array(result))
            }
            "array_flip" => {
                if args.len() != 1 { return Err("array_flip() expects 1 argument".into()); }
                match &args[0] {
                    Value::Array(items) => {
                        let mut map = HashMap::new();
                        for (i, item) in items.iter().enumerate() {
                            map.insert(format!("{}", item), Value::Int(i as i64));
                        }
                        Ok(Value::Dict(map))
                    }
                    Value::Dict(map) => {
                        let mut flipped = HashMap::new();
                        for (k, v) in map.iter() {
                            flipped.insert(format!("{}", v), Value::String_(k.clone()));
                        }
                        Ok(Value::Dict(flipped))
                    }
                    _ => Err("array_flip() expects an array or dict".into()),
                }
            }
            "in_array" => {
                if args.len() < 2 || args.len() > 3 { return Err("in_array() expects 2-3 arguments".into()); }
                let strict = args.len() >= 3 && args[2].is_truthy();
                match &args[1] {
                    Value::Array(items) => {
                        let found = if strict {
                            items.iter().any(|v| v.strict_eq(&args[0]))
                        } else {
                            items.iter().any(|v| *v == args[0])
                        };
                        Ok(Value::Bool(found))
                    }
                    _ => Err("in_array() expects an array as second argument".into()),
                }
            }
            "array_search" => {
                if args.len() < 2 || args.len() > 3 { return Err("array_search() expects 2-3 arguments".into()); }
                let strict = args.len() >= 3 && args[2].is_truthy();
                match &args[1] {
                    Value::Array(items) => {
                        let pos = if strict {
                            items.iter().position(|v| v.strict_eq(&args[0]))
                        } else {
                            items.iter().position(|v| *v == args[0])
                        };
                        match pos {
                            Some(i) => Ok(Value::Int(i as i64)),
                            None => Ok(Value::Bool(false)),
                        }
                    }
                    Value::Dict(map) => {
                        let pos = map.iter().position(|(_, v)| *v == args[0]);
                        match pos {
                            Some(i) => Ok(Value::Int(i as i64)),
                            None => Ok(Value::Bool(false)),
                        }
                    }
                    _ => Err("array_search() expects an array as second argument".into()),
                }
            }
            "array_key_exists" => {
                if args.len() != 2 { return Err("array_key_exists() expects 2 arguments".into()); }
                match (&args[0], &args[1]) {
                    (key, Value::Dict(map)) => {
                        let key_str = format!("{}", key);
                        Ok(Value::Bool(map.contains_key(&key_str)))
                    }
                    (Value::Int(i), Value::Array(items)) => {
                        Ok(Value::Bool(*i >= 0 && (*i as usize) < items.len()))
                    }
                    _ => Err("array_key_exists() expects (key, array_or_dict)".into()),
                }
            }
            "array_slice" => {
                if args.len() < 2 || args.len() > 3 { return Err("array_slice() expects 2-3 arguments".into()); }
                match (&args[0], &args[1]) {
                    (Value::Array(items), Value::Int(offset)) => {
                        let len = if args.len() >= 3 {
                            match &args[2] { Value::Int(n) => *n as usize, _ => return Err("array_slice() length must be int".into()) }
                        } else { items.len() };
                        let start = if *offset >= 0 { *offset as usize } else { items.len().saturating_sub((-offset) as usize) };
                        let end = std::cmp::min(start + len, items.len());
                        if start < items.len() {
                            Ok(Value::Array(items[start..end].to_vec()))
                        } else {
                            Ok(Value::Array(vec![]))
                        }
                    }
                    _ => Err("array_slice() expects (array, int)".into()),
                }
            }
            "array_sum" => {
                if args.len() != 1 { return Err("array_sum() expects 1 argument".into()); }
                match &args[0] {
                    Value::Array(items) => {
                        let mut sum = 0.0_f64;
                        let mut has_float = false;
                        for item in items {
                            match item {
                                Value::Int(n) => sum += *n as f64,
                                Value::Float(n) => { sum += n; has_float = true; }
                                _ => {}
                            }
                        }
                        if has_float {
                            Ok(Value::Float(sum))
                        } else {
                            Ok(Value::Int(sum as i64))
                        }
                    }
                    _ => Err("array_sum() expects an array".into()),
                }
            }
            "array_unique" => {
                if args.len() != 1 { return Err("array_unique() expects 1 argument".into()); }
                match &args[0] {
                    Value::Array(items) => {
                        let mut seen = Vec::new();
                        for item in items {
                            if !seen.contains(item) {
                                seen.push(item.clone());
                            }
                        }
                        Ok(Value::Array(seen))
                    }
                    _ => Err("array_unique() expects an array".into()),
                }
            }
            "array_reverse" => {
                if args.len() != 1 { return Err("array_reverse() expects 1 argument".into()); }
                match &args[0] {
                    Value::Array(items) => {
                        let mut reversed = items.clone();
                        reversed.reverse();
                        Ok(Value::Array(reversed))
                    }
                    _ => Err("array_reverse() expects an array".into()),
                }
            }
            "array_filter" => {
                if args.len() != 1 { return Err("array_filter() expects 1 argument".into()); }
                match &args[0] {
                    Value::Array(items) => {
                        Ok(Value::Array(items.iter()
                            .filter(|v| v.is_truthy())
                            .cloned()
                            .collect()))
                    }
                    _ => Err("array_filter() expects an array".into()),
                }
            }
            "array_map" => {
                if args.len() != 2 { return Err("array_map() expects 2 arguments".into()); }
                match (&args[0], &args[1]) {
                    (Value::Function(fv), Value::Array(items)) => {
                        let mut results = Vec::new();
                        for item in items {
                            let result = self.call_builtin(&fv.name, vec![item.clone()])?;
                            results.push(result);
                        }
                        Ok(Value::Array(results))
                    }
                    _ => Err("array_map() expects (callback, array)".into()),
                }
            }
            "array_reduce" => {
                if args.len() < 2 || args.len() > 3 { return Err("array_reduce() expects 2-3 arguments".into()); }
                match &args[0] {
                    Value::Array(items) => {
                        let mut carry = if args.len() >= 3 { args[2].clone() } else { Value::Null };
                        match &args[1] {
                            Value::Function(fv) => {
                                for item in items {
                                    carry = self.call_builtin(&fv.name, vec![carry.clone(), item.clone()])?;
                                }
                                Ok(carry)
                            }
                            _ => Err("array_reduce() expects a callback function".into()),
                        }
                    }
                    _ => Err("array_reduce() expects an array".into()),
                }
            }
            "range" => {
                if args.len() < 2 || args.len() > 3 { return Err("range() expects 2-3 arguments".into()); }
                match (&args[0], &args[1]) {
                    (Value::Int(start), Value::Int(end)) => {
                        let step = if args.len() >= 3 {
                            match &args[2] { Value::Int(n) => *n, _ => 1 }
                        } else { 1 };
                        let mut result = Vec::new();
                        if step > 0 {
                            let mut i = *start;
                            while i <= *end { result.push(Value::Int(i)); i += step; }
                        } else if step < 0 {
                            let mut i = *start;
                            while i >= *end { result.push(Value::Int(i)); i += step; }
                        }
                        Ok(Value::Array(result))
                    }
                    _ => Err("range() expects (int, int)".into()),
                }
            }
            "sort" => {
                if args.len() != 1 { return Err("sort() expects 1 argument".into()); }
                match &args[0] {
                    Value::Array(items) => {
                        let mut sorted = items.clone();
                        sorted.sort_by(|a, b| format!("{}", a).cmp(&format!("{}", b)));
                        Ok(Value::Array(sorted))
                    }
                    _ => Err("sort() expects an array".into()),
                }
            }
            "rsort" => {
                if args.len() != 1 { return Err("rsort() expects 1 argument".into()); }
                match &args[0] {
                    Value::Array(items) => {
                        let mut sorted = items.clone();
                        sorted.sort_by(|a, b| format!("{}", b).cmp(&format!("{}", a)));
                        Ok(Value::Array(sorted))
                    }
                    _ => Err("rsort() expects an array".into()),
                }
            }
            "array_diff" => {
                if args.len() < 2 { return Err("array_diff() expects at least 2 arguments".into()); }
                let base = match &args[0] {
                    Value::Array(items) => items.clone(),
                    _ => return Err("array_diff() expects arrays".into()),
                };
                let mut exclude: Vec<String> = Vec::new();
                for i in 1..args.len() {
                    if let Value::Array(items) = &args[i] {
                        for item in items {
                            exclude.push(format!("{}", item));
                        }
                    }
                }
                let result: Vec<Value> = base.into_iter()
                    .filter(|v| !exclude.contains(&format!("{}", v)))
                    .collect();
                Ok(Value::Array(result))
            }
            "array_combine" => {
                if args.len() != 2 { return Err("array_combine() expects 2 arguments".into()); }
                match (&args[0], &args[1]) {
                    (Value::Array(keys), Value::Array(vals)) => {
                        let mut map = std::collections::HashMap::new();
                        for (i, key) in keys.iter().enumerate() {
                            let val = vals.get(i).cloned().unwrap_or(Value::Null);
                            map.insert(format!("{}", key), val);
                        }
                        Ok(Value::Dict(map))
                    }
                    _ => Err("array_combine() expects (array, array)".into()),
                }
            }
            "array_column" => {
                if args.len() != 2 { return Err("array_column() expects 2 arguments".into()); }
                match (&args[0], &args[1]) {
                    (Value::Array(rows), Value::String_(col)) => {
                        let result: Vec<Value> = rows.iter().filter_map(|row| {
                            match row {
                                Value::Dict(map) => map.get(col).cloned(),
                                _ => None,
                            }
                        }).collect();
                        Ok(Value::Array(result))
                    }
                    _ => Err("array_column() expects (array, string)".into()),
                }
            }
            "array_fill" => {
                if args.len() != 3 { return Err("array_fill() expects 3 arguments".into()); }
                let start = match &args[0] { Value::Int(n) => *n as usize, _ => return Err("array_fill() start must be int".into()) };
                let count = match &args[1] { Value::Int(n) => *n as usize, _ => return Err("array_fill() count must be int".into()) };
                let val = args[2].clone();
                let mut result = Vec::new();
                for _ in 0..count {
                    result.push(val.clone());
                }
                if start > 0 {
                    let mut padded = vec![Value::Null; start];
                    padded.append(&mut result);
                    Ok(Value::Array(padded))
                } else {
                    Ok(Value::Array(result))
                }
            }
            "array_rand" => {
                if args.is_empty() || args.len() > 2 { return Err("array_rand() expects 1-2 arguments".into()); }
                let count = if args.len() >= 2 {
                    match &args[1] { Value::Int(n) => *n as usize, _ => return Err("array_rand() count must be int".into()) }
                } else { 1 };
                match &args[0] {
                    Value::Array(items) => {
                        let n = std::cmp::min(count, items.len());
                        // Fisher-Yates partial shuffle using LCG
                        let mut indices: Vec<usize> = (0..items.len()).collect();
                        let mut seed = 12345u64;
                        for i in 0..n {
                            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
                            let j = i + (seed as usize % (indices.len() - i));
                            indices.swap(i, j);
                        }
                        let result: Vec<Value> = indices[..n].iter().map(|&i| items[i].clone()).collect();
                        if count == 1 {
                            Ok(result.into_iter().next().unwrap_or(Value::Null))
                        } else {
                            Ok(Value::Array(result))
                        }
                    }
                    Value::Dict(map) => {
                        let keys: Vec<&String> = map.keys().collect();
                        let n = std::cmp::min(count, keys.len());
                        let mut indices: Vec<usize> = (0..keys.len()).collect();
                        let mut seed = 12345u64;
                        for i in 0..n {
                            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
                            let j = i + (seed as usize % (indices.len() - i));
                            indices.swap(i, j);
                        }
                        let result: Vec<Value> = indices[..n].iter().map(|&i| Value::String_(keys[i].clone())).collect();
                        if count == 1 {
                            Ok(result.into_iter().next().unwrap_or(Value::Null))
                        } else {
                            Ok(Value::Array(result))
                        }
                    }
                    _ => Err("array_rand() expects an array or dict".into()),
                }
            }
            "str_contains" => {
                if args.len() != 2 { return Err("str_contains() expects 2 arguments".into()); }
                match (&args[0], &args[1]) {
                    (Value::String_(h), Value::String_(n)) => Ok(Value::Bool(h.contains(n.as_str()))),
                    _ => Err("str_contains() expects two strings".into()),
                }
            }
            "trim" => {
                if args.len() != 1 { return Err("trim() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(s) => Ok(Value::String_(s.trim().to_string())),
                    _ => Err("trim() expects a string".into()),
                }
            }
            // ---- Type Checking ----
            "is_null" => {
                if args.len() != 1 { return Err("is_null() expects 1 argument".into()); }
                Ok(Value::Bool(matches!(args[0], Value::Null)))
            }
            "is_int" => {
                if args.len() != 1 { return Err("is_int() expects 1 argument".into()); }
                Ok(Value::Bool(matches!(args[0], Value::Int(_))))
            }
            "is_string" => {
                if args.len() != 1 { return Err("is_string() expects 1 argument".into()); }
                Ok(Value::Bool(matches!(args[0], Value::String_(_))))
            }
            "is_bool" => {
                if args.len() != 1 { return Err("is_bool() expects 1 argument".into()); }
                Ok(Value::Bool(matches!(args[0], Value::Bool(_))))
            }
            "is_float" => {
                if args.len() != 1 { return Err("is_float() expects 1 argument".into()); }
                Ok(Value::Bool(matches!(args[0], Value::Float(_))))
            }
            "is_array" => {
                if args.len() != 1 { return Err("is_array() expects 1 argument".into()); }
                Ok(Value::Bool(matches!(args[0], Value::Array(_) | Value::Dict(_))))
            }
            "gettype" => {
                if args.len() != 1 { return Err("gettype() expects 1 argument".into()); }
                Ok(Value::String_(args[0].type_name().to_string()))
            }
            "isset" => {
                if args.len() != 1 { return Err("isset() expects 1 argument".into()); }
                Ok(Value::Bool(!matches!(args[0], Value::Null)))
            }
            "empty" => {
                if args.len() != 1 { return Err("empty() expects 1 argument".into()); }
                let is_empty = match &args[0] {
                    Value::Null => true,
                    Value::Bool(b) => !*b,
                    Value::Int(n) => *n == 0,
                    Value::Float(n) => *n == 0.0,
                    Value::String_(s) => s.is_empty() || s == "0",
                    Value::Array(a) => a.is_empty(),
                    Value::Dict(d) => d.is_empty(),
                    _ => false,
                };
                Ok(Value::Bool(is_empty))
            }
            "unset" => {
                Ok(Value::Null)
            }
            "var_dump" => {
                if args.is_empty() { return Err("var_dump() expects at least 1 argument".into()); }
                let output = value_var_dump(&args[0], 0);
                print!("{}", output);
                Ok(Value::Null)
            }
            "print_r" => {
                if args.is_empty() { return Err("print_r() expects at least 1 argument".into()); }
                let output = value_print_r(&args[0], 0);
                print!("{}\n", output);
                Ok(Value::Null)
            }
            // ---- PHP String Functions ----
            "substr" => {
                if args.len() < 2 || args.len() > 3 { return Err("substr() expects 2-3 arguments".into()); }
                match (&args[0], &args[1]) {
                    (Value::String_(s), Value::Int(start)) => {
                        let len = if args.len() >= 3 {
                            match &args[2] { Value::Int(n) => *n as usize, _ => return Err("substr() length must be int".into()) }
                        } else { s.len() };
                        let start_idx = if *start >= 0 { *start as usize } else { s.len().saturating_sub((-start) as usize) };
                        let end = std::cmp::min(start_idx + len, s.len());
                        if start_idx < s.len() {
                            Ok(Value::String_(s[start_idx..end].to_string()))
                        } else {
                            Ok(Value::String_("".into()))
                        }
                    }
                    _ => Err("substr() expects (string, int)".into()),
                }
            }
            "strpos" => {
                if args.len() != 2 { return Err("strpos() expects 2 arguments".into()); }
                match (&args[0], &args[1]) {
                    (Value::String_(h), Value::String_(n)) => {
                        Ok(Value::Int(h.find(n.as_str()).map(|i| i as i64).unwrap_or(-1)))
                    }
                    _ => Err("strpos() expects two strings".into()),
                }
            }
            "stripos" => {
                if args.len() != 2 { return Err("stripos() expects 2 arguments".into()); }
                match (&args[0], &args[1]) {
                    (Value::String_(h), Value::String_(n)) => {
                        let hl = h.to_lowercase();
                        let nl = n.to_lowercase();
                        Ok(Value::Int(hl.find(&nl).map(|i| i as i64).unwrap_or(-1)))
                    }
                    _ => Err("stripos() expects two strings".into()),
                }
            }
            "explode" => {
                if args.len() != 2 { return Err("explode() expects 2 arguments".into()); }
                match (&args[0], &args[1]) {
                    (Value::String_(d), Value::String_(s)) => {
                        let parts: Vec<Value> = s.split(d.as_str())
                            .map(|p| Value::String_(p.to_string()))
                            .collect();
                        Ok(Value::Array(parts))
                    }
                    _ => Err("explode() expects two strings".into()),
                }
            }
            "implode" | "join" => {
                if args.len() != 2 { return Err("implode() expects 2 arguments".into()); }
                match (&args[0], &args[1]) {
                    (Value::String_(glue), Value::Array(items)) => {
                        let parts: Vec<String> = items.iter().map(|v| format!("{}", v)).collect();
                        Ok(Value::String_(parts.join(glue.as_str())))
                    }
                    _ => Err("implode() expects (string, array)".into()),
                }
            }
            "str_repeat" => {
                if args.len() != 2 { return Err("str_repeat() expects 2 arguments".into()); }
                match (&args[0], &args[1]) {
                    (Value::String_(s), Value::Int(n)) => {
                        Ok(Value::String_(s.repeat(*n as usize)))
                    }
                    _ => Err("str_repeat() expects (string, int)".into()),
                }
            }
            "strtolower" => {
                if args.len() != 1 { return Err("strtolower() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(s) => Ok(Value::String_(s.to_lowercase())),
                    _ => Err("strtolower() expects a string".into()),
                }
            }
            "strtoupper" => {
                if args.len() != 1 { return Err("strtoupper() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(s) => Ok(Value::String_(s.to_uppercase())),
                    _ => Err("strtoupper() expects a string".into()),
                }
            }
            "htmlspecialchars" => {
                if args.len() != 1 { return Err("htmlspecialchars() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(s) => {
                        let escaped = s.replace('&', "&amp;")
                            .replace('<', "&lt;")
                            .replace('>', "&gt;")
                            .replace('"', "&quot;")
                            .replace('\'', "&#039;");
                        Ok(Value::String_(escaped))
                    }
                    _ => Err("htmlspecialchars() expects a string".into()),
                }
            }
            "strip_tags" => {
                if args.len() != 1 { return Err("strip_tags() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(s) => {
                        let mut result = String::new();
                        let mut in_tag = false;
                        for ch in s.chars() {
                            if ch == '<' { in_tag = true; continue; }
                            if ch == '>' { in_tag = false; continue; }
                            if !in_tag { result.push(ch); }
                        }
                        Ok(Value::String_(result))
                    }
                    _ => Err("strip_tags() expects a string".into()),
                }
            }
            "nl2br" => {
                if args.len() != 1 { return Err("nl2br() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(s) => Ok(Value::String_(s.replace('\n', "<br />\n"))),
                    _ => Err("nl2br() expects a string".into()),
                }
            }
            "str_replace" => {
                if args.len() != 3 { return Err("str_replace() expects 3 arguments".into()); }
                match (&args[0], &args[1], &args[2]) {
                    (Value::String_(search), Value::String_(replace), Value::String_(subject)) => {
                        Ok(Value::String_(subject.replace(search.as_str(), replace.as_str())))
                    }
                    _ => Err("str_replace() expects (string, string, string)".into()),
                }
            }
            "ltrim" => {
                if args.len() != 1 { return Err("ltrim() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(s) => Ok(Value::String_(s.trim_start().to_string())),
                    _ => Err("ltrim() expects a string".into()),
                }
            }
            "rtrim" => {
                if args.len() != 1 { return Err("rtrim() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(s) => Ok(Value::String_(s.trim_end().to_string())),
                    _ => Err("rtrim() expects a string".into()),
                }
            }
            "strrpos" => {
                if args.len() != 2 { return Err("strrpos() expects 2 arguments".into()); }
                match (&args[0], &args[1]) {
                    (Value::String_(haystack), Value::String_(needle)) => {
                        Ok(Value::Int(haystack.rfind(needle.as_str()).map(|i| i as i64).unwrap_or(-1)))
                    }
                    _ => Err("strrpos() expects (string, string)".into()),
                }
            }
            "ucfirst" => {
                if args.len() != 1 { return Err("ucfirst() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(s) => {
                        let mut chars = s.chars();
                        match chars.next() {
                            Some(first) => {
                                let rest: String = chars.collect();
                                Ok(Value::String_(format!("{}{}", first.to_uppercase(), rest)))
                            }
                            None => Ok(Value::String_(String::new())),
                        }
                    }
                    _ => Err("ucfirst() expects a string".into()),
                }
            }
            "sprintf" => {
                if args.len() < 1 { return Err("sprintf() expects at least 1 argument".into()); }
                match &args[0] {
                    Value::String_(fmt) => {
                        let mut result = fmt.clone();
                        let mut arg_idx = 1;
                        // Simple %s / %d / %f substitution
                        let mut i = 0;
                        while i < result.len() {
                            if result.as_bytes().get(i) == Some(&b'%') && i + 1 < result.len() {
                                let spec = result.as_bytes()[i + 1];
                                if spec == b'%' {
                                    result.replace_range(i..i+2, "%");
                                    i += 1;
                                    continue;
                                }
                                let replacement = if arg_idx < args.len() {
                                    match &args[arg_idx] {
                                        Value::String_(s) => s.clone(),
                                        Value::Int(n) => n.to_string(),
                                        Value::Float(n) => n.to_string(),
                                        Value::Bool(b) => (if *b { "true" } else { "false" }).to_string(),
                                        Value::Null => "null".to_string(),
                                        _ => format!("{}", args[arg_idx]),
                                    }
                                } else { String::new() };
                                result.replace_range(i..i+2, &replacement);
                                i += replacement.len();
                                arg_idx += 1;
                                continue;
                            }
                            i += 1;
                        }
                        Ok(Value::String_(result))
                    }
                    _ => Err("sprintf() expects a format string".into()),
                }
            }
            "number_format" => {
                if args.is_empty() || args.len() > 2 { return Err("number_format() expects 1-2 arguments".into()); }
                let decimals = if args.len() >= 2 {
                    match &args[1] { Value::Int(n) => *n as usize, _ => return Err("number_format() decimals must be int".into()) }
                } else { 0 };
                let num = match &args[0] {
                    Value::Int(n) => *n as f64,
                    Value::Float(n) => *n,
                    _ => return Err("number_format() expects a number".into()),
                };
                Ok(Value::String_(format!("{:.dec$}", num, dec = decimals)))
            }
            // ---- Math Functions ----
            "abs" => {
                if args.len() != 1 { return Err("abs() expects 1 argument".into()); }
                match &args[0] {
                    Value::Int(n) => Ok(Value::Int(n.abs())),
                    Value::Float(n) => Ok(Value::Float(n.abs())),
                    _ => Err("abs() expects a number".into()),
                }
            }
            "ceil" => {
                if args.len() != 1 { return Err("ceil() expects 1 argument".into()); }
                match &args[0] {
                    Value::Float(n) => Ok(Value::Int(n.ceil() as i64)),
                    Value::Int(n) => Ok(Value::Int(*n)),
                    _ => Err("ceil() expects a number".into()),
                }
            }
            "floor" => {
                if args.len() != 1 { return Err("floor() expects 1 argument".into()); }
                match &args[0] {
                    Value::Float(n) => Ok(Value::Int(n.floor() as i64)),
                    Value::Int(n) => Ok(Value::Int(*n)),
                    _ => Err("floor() expects a number".into()),
                }
            }
            "round" => {
                if args.len() < 1 || args.len() > 2 { return Err("round() expects 1-2 arguments".into()); }
                let precision = if args.len() >= 2 {
                    match &args[1] { Value::Int(n) => *n as i32, _ => return Err("round() precision must be int".into()) }
                } else { 0 };
                match &args[0] {
                    Value::Float(n) => {
                        let factor = 10_f64.powi(precision);
                        Ok(Value::Float((n * factor).round() / factor))
                    }
                    Value::Int(n) => Ok(Value::Int(*n)),
                    _ => Err("round() expects a number".into()),
                }
            }
            "max" => {
                if args.len() < 2 { return Err("max() expects at least 2 arguments".into()); }
                let mut max_val = &args[0];
                for a in &args[1..] {
                    match (max_val, a) {
                        (Value::Int(x), Value::Int(y)) if y > x => max_val = a,
                        (Value::Float(x), Value::Float(y)) if y > x => max_val = a,
                        (Value::Int(x), Value::Float(y)) if *y > *x as f64 => max_val = a,
                        (Value::Float(x), Value::Int(y)) if *y as f64 > *x => max_val = a,
                        _ => {}
                    }
                }
                Ok(max_val.clone())
            }
            "min" => {
                if args.len() < 2 { return Err("min() expects at least 2 arguments".into()); }
                let mut min_val = &args[0];
                for a in &args[1..] {
                    match (min_val, a) {
                        (Value::Int(x), Value::Int(y)) if y < x => min_val = a,
                        (Value::Float(x), Value::Float(y)) if y < x => min_val = a,
                        (Value::Int(x), Value::Float(y)) if *y < *x as f64 => min_val = a,
                        (Value::Float(x), Value::Int(y)) if (*y as f64) < *x => min_val = a,
                        _ => {}
                    }
                }
                Ok(min_val.clone())
            }
            "rand" | "mt_rand" => {
                if args.len() != 2 { return Err("rand() expects 2 arguments".into()); }
                match (&args[0], &args[1]) {
                    (Value::Int(min), Value::Int(max)) => {
                        if min > max { return Err("rand() min must be <= max".into()); }
                        // Simple LCG for deterministic random
                        let val = ((min + max + 12345) * 1103515245 + 12345) as i64;
                        let range = max - min + 1;
                        Ok(Value::Int(if range > 0 { min + (val & 0x7FFFFFFF) % range } else { *min }))
                    }
                    _ => Err("rand() expects (int, int)".into()),
                }
            }
            "pow" => {
                if args.len() != 2 { return Err("pow() expects 2 arguments".into()); }
                let base = match &args[0] { Value::Int(n) => *n as f64, Value::Float(n) => *n, _ => return Err("pow() expects numbers".into()) };
                let exp = match &args[1] { Value::Int(n) => *n as f64, Value::Float(n) => *n, _ => return Err("pow() expects numbers".into()) };
                Ok(Value::Float(base.powf(exp)))
            }
            // ---- Hash Functions ----
            "md5" => {
                if args.len() != 1 { return Err("md5() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(s) => {
                        Ok(Value::String_(md5_hash(s)))
                    }
                    _ => Err("md5() expects a string".into()),
                }
            }
            "sha1" => {
                if args.len() != 1 { return Err("sha1() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(s) => {
                        Ok(Value::String_(sha1_hash(s)))
                    }
                    _ => Err("sha1() expects a string".into()),
                }
            }
            // ---- Misc Functions ----
            "uniqid" => {
                let prefix = if args.is_empty() {
                    String::new()
                } else {
                    match &args[0] {
                        Value::String_(s) => s.clone(),
                        _ => return Err("uniqid() prefix must be a string".into()),
                    }
                };
                let ts = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default();
                let id = format!("{}{:08x}{:05x}", prefix, ts.as_secs(), ts.subsec_micros());
                Ok(Value::String_(id))
            }
            "sleep" => {
                if args.len() != 1 { return Err("sleep() expects 1 argument".into()); }
                match &args[0] {
                    Value::Int(n) if *n >= 0 => {
                        std::thread::sleep(std::time::Duration::from_secs(*n as u64));
                        Ok(Value::Null)
                    }
                    _ => Err("sleep() expects a non-negative integer".into()),
                }
            }
            "usleep" => {
                if args.len() != 1 { return Err("usleep() expects 1 argument".into()); }
                match &args[0] {
                    Value::Int(n) if *n >= 0 => {
                        std::thread::sleep(std::time::Duration::from_micros(*n as u64));
                        Ok(Value::Null)
                    }
                    _ => Err("usleep() expects a non-negative integer".into()),
                }
            }
            "realpath" => {
                if args.len() != 1 { return Err("realpath() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(path) => {
                        std::fs::canonicalize(path)
                            .map(|p| Value::String_(p.to_string_lossy().to_string()))
                            .map_err(|e| format!("__throw__s:{}", e))
                    }
                    _ => Err("realpath() expects a string".into()),
                }
            }
            "is_file" => {
                if args.len() != 1 { return Err("is_file() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(path) => Ok(Value::Bool(std::path::Path::new(path).is_file())),
                    _ => Err("is_file() expects a string".into()),
                }
            }
            "getcwd" => {
                if !args.is_empty() { return Err("getcwd() expects 0 arguments".into()); }
                std::env::current_dir()
                    .map(|p| Value::String_(p.to_string_lossy().to_string()))
                    .map_err(|e| format!("getcwd(): {}", e))
            }
            // ---- URL & Encoding Functions ----
            "urlencode" => {
                if args.len() != 1 { return Err("urlencode() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(s) => {
                        let encoded: String = s.bytes().map(|b| {
                            if b.is_ascii_alphanumeric() || b == b'-' || b == b'_' || b == b'.' || b == b'~' {
                                format!("{}", b as char)
                            } else if b == b' ' {
                                "+".to_string()
                            } else {
                                format!("%{:02X}", b)
                            }
                        }).collect();
                        Ok(Value::String_(encoded))
                    }
                    _ => Err("urlencode() expects a string".into()),
                }
            }
            "urldecode" => {
                if args.len() != 1 { return Err("urldecode() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(s) => {
                        let s = s.replace('+', " ");
                        let mut result = String::new();
                        let mut chars = s.chars();
                        while let Some(c) = chars.next() {
                            if c == '%' {
                                let hex: String = chars.by_ref().take(2).collect();
                                if hex.len() == 2 {
                                    if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                                        result.push(byte as char);
                                    } else {
                                        result.push('%');
                                        result.push_str(&hex);
                                    }
                                } else {
                                    result.push('%');
                                    result.push_str(&hex);
                                }
                            } else {
                                result.push(c);
                            }
                        }
                        Ok(Value::String_(result))
                    }
                    _ => Err("urldecode() expects a string".into()),
                }
            }
            "parse_url" => {
                if args.len() != 1 { return Err("parse_url() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(url) => {
                        let mut map = std::collections::HashMap::new();
                        // Find scheme
                        if let Some(scheme_end) = url.find("://") {
                            let scheme = &url[..scheme_end];
                            map.insert("scheme".to_string(), Value::String_(scheme.to_string()));
                            let rest = &url[scheme_end + 3..];
                            // Find host (up to / or : or end)
                            let host_end = rest.find(|c: char| c == '/' || c == ':' || c == '?').unwrap_or(rest.len());
                            let host = &rest[..host_end];
                            map.insert("host".to_string(), Value::String_(host.to_string()));
                            let after_host = &rest[host_end..];
                            // Port
                            if after_host.starts_with(':') {
                                let port_end = after_host.find(|c: char| c == '/' || c == '?').unwrap_or(after_host.len());
                                let port_str = &after_host[1..port_end];
                                if let Ok(port) = port_str.parse::<i64>() {
                                    map.insert("port".to_string(), Value::Int(port));
                                }
                                let path_start = after_host.find('/').unwrap_or(after_host.len());
                                let query_start = after_host.find('?');
                                if let Some(qs) = query_start {
                                    let path = if path_start < qs { &after_host[path_start..qs] } else { "/" };
                                    map.insert("path".to_string(), Value::String_(path.to_string()));
                                    map.insert("query".to_string(), Value::String_(after_host[qs + 1..].to_string()));
                                } else if path_start < after_host.len() {
                                    map.insert("path".to_string(), Value::String_(after_host[path_start..].to_string()));
                                }
                            } else {
                                if let Some(qs) = after_host.find('?') {
                                    let path = &after_host[..qs];
                                    map.insert("path".to_string(), Value::String_(if path.is_empty() { "/" } else { path }.to_string()));
                                    map.insert("query".to_string(), Value::String_(after_host[qs + 1..].to_string()));
                                } else {
                                    map.insert("path".to_string(), Value::String_(if after_host.is_empty() { "/" } else { after_host }.to_string()));
                                }
                            }
                        }
                        Ok(Value::Dict(map))
                    }
                    _ => Err("parse_url() expects a string".into()),
                }
            }
            "http_build_query" => {
                if args.len() != 1 { return Err("http_build_query() expects 1 argument".into()); }
                match &args[0] {
                    Value::Dict(map) => {
                        let parts: Vec<String> = map.iter().map(|(k, v)| {
                            let val_str = match v {
                                Value::String_(s) => s.clone(),
                                Value::Int(n) => n.to_string(),
                                Value::Float(n) => n.to_string(),
                                Value::Bool(b) => (if *b { "1" } else { "0" }).to_string(),
                                _ => String::new(),
                            };
                            format!("{}={}", k, val_str)
                        }).collect();
                        Ok(Value::String_(parts.join("&")))
                    }
                    _ => Err("http_build_query() expects a dict".into()),
                }
            }
            "base64_encode" => {
                if args.len() != 1 { return Err("base64_encode() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(s) => {
                        const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
                        let bytes = s.as_bytes();
                        let mut result = String::new();
                        for chunk in bytes.chunks(3) {
                            let b0 = chunk[0] as u32;
                            let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
                            let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
                            let triple = (b0 << 16) | (b1 << 8) | b2;
                            result.push(CHARS[((triple >> 18) & 0x3F) as usize] as char);
                            result.push(CHARS[((triple >> 12) & 0x3F) as usize] as char);
                            result.push(if chunk.len() > 1 { CHARS[((triple >> 6) & 0x3F) as usize] as char } else { '=' });
                            result.push(if chunk.len() > 2 { CHARS[(triple & 0x3F) as usize] as char } else { '=' });
                        }
                        Ok(Value::String_(result))
                    }
                    _ => Err("base64_encode() expects a string".into()),
                }
            }
            "base64_decode" => {
                if args.len() != 1 { return Err("base64_decode() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(s) => {
                        let s = s.trim_end_matches('=');
                        let mut result = Vec::new();
                        let mut buffer: u32 = 0;
                        let mut bits_collected = 0;
                        for c in s.chars() {
                            let val = match c {
                                'A'..='Z' => c as u32 - 'A' as u32,
                                'a'..='z' => c as u32 - 'a' as u32 + 26,
                                '0'..='9' => c as u32 - '0' as u32 + 52,
                                '+' => 62,
                                '/' => 63,
                                _ => continue,
                            };
                            buffer = (buffer << 6) | val;
                            bits_collected += 6;
                            if bits_collected >= 8 {
                                bits_collected -= 8;
                                result.push(((buffer >> bits_collected) & 0xFF) as u8);
                            }
                        }
                        Ok(Value::String_(String::from_utf8_lossy(&result).to_string()))
                    }
                    _ => Err("base64_decode() expects a string".into()),
                }
            }
            // ---- File System Functions ----
            "file_get_contents" => {
                if args.len() != 1 { return Err("file_get_contents() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(path) => {
                        std::fs::read_to_string(path)
                            .map(Value::String_)
                            .map_err(|e| format!("__throw__s:{}", e))
                    }
                    _ => Err("file_get_contents() expects a string path".into()),
                }
            }
            "file_put_contents" => {
                if args.len() != 2 { return Err("file_put_contents() expects 2 arguments".into()); }
                match (&args[0], &args[1]) {
                    (Value::String_(path), Value::String_(content)) => {
                        std::fs::write(path, content)
                            .map(|_| Value::Int(content.len() as i64))
                            .map_err(|e| format!("__throw__s:{}", e))
                    }
                    _ => Err("file_put_contents() expects (string, string)".into()),
                }
            }
            "file_exists" => {
                if args.len() != 1 { return Err("file_exists() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(path) => Ok(Value::Bool(std::path::Path::new(path).exists())),
                    _ => Err("file_exists() expects a string".into()),
                }
            }
            "is_dir" => {
                if args.len() != 1 { return Err("is_dir() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(path) => Ok(Value::Bool(std::path::Path::new(path).is_dir())),
                    _ => Err("is_dir() expects a string".into()),
                }
            }
            "mkdir" => {
                if args.len() != 1 { return Err("mkdir() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(path) => {
                        std::fs::create_dir_all(path)
                            .map(|_| Value::Bool(true))
                            .map_err(|e| format!("__throw__s:{}", e))
                    }
                    _ => Err("mkdir() expects a string".into()),
                }
            }
            "unlink" => {
                if args.len() != 1 { return Err("unlink() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(path) => {
                        std::fs::remove_file(path)
                            .map(|_| Value::Bool(true))
                            .map_err(|e| format!("__throw__s:{}", e))
                    }
                    _ => Err("unlink() expects a string".into()),
                }
            }
            "basename" => {
                if args.len() != 1 { return Err("basename() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(path) => {
                        let p = std::path::Path::new(path);
                        let name = p.file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("");
                        Ok(Value::String_(name.to_string()))
                    }
                    _ => Err("basename() expects a string".into()),
                }
            }
            "dirname" => {
                if args.len() != 1 { return Err("dirname() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(path) => {
                        let p = std::path::Path::new(path);
                        let dir = p.parent()
                            .and_then(|d| d.to_str())
                            .unwrap_or(".");
                        Ok(Value::String_(dir.to_string()))
                    }
                    _ => Err("dirname() expects a string".into()),
                }
            }
            "scandir" => {
                if args.len() != 1 { return Err("scandir() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(path) => {
                        let entries: std::fs::ReadDir = std::fs::read_dir(path)
                            .map_err(|e| format!("__throw__s:{}", e))?;
                        let mut result = Vec::new();
                        for entry in entries {
                            if let Ok(e) = entry {
                                if let Some(name) = e.file_name().to_str() {
                                    result.push(Value::String_(name.to_string()));
                                }
                            }
                        }
                        Ok(Value::Array(result))
                    }
                    _ => Err("scandir() expects a string".into()),
                }
            }
            "json_encode" => {
                if args.len() != 1 { return Err("json_encode() expects 1 argument".into()); }
                Ok(Value::String_(value_to_json(&args[0])))
            }
            "json_decode" => {
                if args.len() < 1 || args.len() > 2 { return Err("json_decode() expects 1-2 arguments".into()); }
                match &args[0] {
                    Value::String_(s) => json_parse(s),
                    _ => Err("json_decode() expects a string".into()),
                }
            }
            "sqrt" => {
                if args.len() != 1 { return Err("sqrt() expects 1 argument".into()); }
                let n = match &args[0] { Value::Int(n) => *n as f64, Value::Float(n) => *n, _ => return Err("sqrt() expects a number".into()) };
                Ok(Value::Float(n.sqrt()))
            }
            // ---- Date/Time Functions ----
            "time" => {
                if !args.is_empty() { return Err("time() expects 0 arguments".into()); }
                let ts = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs() as i64;
                Ok(Value::Int(ts))
            }
            "date" => {
                if args.len() < 1 || args.len() > 2 { return Err("date() expects 1-2 arguments".into()); }
                match &args[0] {
                    Value::String_(format) => {
                        let ts = if args.len() >= 2 {
                            match &args[1] { Value::Int(n) => *n, _ => return Err("date() timestamp must be int".into()) }
                        } else {
                            std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap_or_default()
                                .as_secs() as i64
                        };
                        // Simple date format implementation
                        let secs = ts;
                        let days = secs / 86400;
                        let time_of_day = secs % 86400;
                        let hours = (time_of_day / 3600) % 24;
                        let minutes = (time_of_day / 60) % 60;
                        let seconds = time_of_day % 60;
                        // Days since Unix epoch (1970-01-01)
                        let total_days = days + 719528; // days from 0000-00-00 to 1970-01-01
                        let era = if total_days >= 0 { total_days / 146097 } else { (total_days - 146096) / 146097 };
                        let doe = total_days - era * 146097; // day of era [0, 146096]
                        let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365; // year of era [0, 399]
                        let year = yoe + era * 400;
                        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // day of year [0, 365]
                        let mp = (5 * doy + 2) / 153; // month [0, 11]
                        let day = doy - (153 * mp + 2) / 5 + 1;
                        let month = if mp < 10 { mp + 3 } else { mp - 9 };
                        let final_year = if month <= 2 { year + 1 } else { year };

                        let result = format
                            .replace("Y", &format!("{:04}", final_year))
                            .replace("y", &format!("{:02}", final_year % 100))
                            .replace("m", &format!("{:02}", month))
                            .replace("d", &format!("{:02}", day))
                            .replace("H", &format!("{:02}", hours))
                            .replace("i", &format!("{:02}", minutes))
                            .replace("s", &format!("{:02}", seconds));
                        Ok(Value::String_(result))
                    }
                    _ => Err("date() expects a format string".into()),
                }
            }
            "strtotime" => {
                if args.len() != 1 { return Err("strtotime() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(s) => {
                        // Basic strtotime implementation
                        let now = std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs() as i64;
                        if s == "now" {
                            Ok(Value::Int(now))
                        } else if s == "tomorrow" {
                            Ok(Value::Int(now + 86400))
                        } else if s == "yesterday" {
                            Ok(Value::Int(now - 86400))
                        } else if s.starts_with('+') && s.ends_with(" day") {
                            let n: i64 = s[1..s.len()-4].trim().parse().unwrap_or(0);
                            Ok(Value::Int(now + n * 86400))
                        } else if s.starts_with('-') && s.ends_with(" day") {
                            let n: i64 = s[1..s.len()-4].trim().parse().unwrap_or(0);
                            Ok(Value::Int(now - n * 86400))
                        } else {
                            // Try YYYY-MM-DD format
                            let parts: Vec<&str> = s.split('-').collect();
                            if parts.len() == 3 {
                                let y: i64 = parts[0].parse().unwrap_or(1970);
                                let m: i64 = parts[1].parse().unwrap_or(1);
                                let d: i64 = parts[2].parse().unwrap_or(1);
                                let mut days = (y - 1970) * 365;
                                days += (y - 1969) / 4 - (y - 1901) / 100 + (y - 1601) / 400;
                                let month_days = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
                                days += month_days[(m - 1) as usize] + d - 1;
                                if m > 2 && (y % 4 == 0 && (y % 100 != 0 || y % 400 == 0)) {
                                    days += 1;
                                }
                                Ok(Value::Int(days * 86400))
                            } else {
                                Ok(Value::Bool(false))
                            }
                        }
                    }
                    _ => Err("strtotime() expects a string".into()),
                }
            }
            "microtime" => {
                if !args.is_empty() { return Err("microtime() expects 0 arguments".into()); }
                let dur = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default();
                let micros = dur.as_micros() % 1_000_000;
                Ok(Value::String_(format!("0.{:06} {}", micros, dur.as_secs())))
            }
            // ---- String Helpers ----
            "phprs_str_replace" => {
                if args.len() != 3 { return Err("phprs_str_replace() expects 3 arguments".into()); }
                match (&args[0], &args[1], &args[2]) {
                    (Value::String_(s), Value::String_(from), Value::String_(to)) =>
                        Ok(Value::String_(s.replace(from.as_str(), to.as_str()))),
                    _ => Err("phprs_str_replace() expects three strings".into()),
                }
            }
            "phprs_str_contains" => {
                if args.len() != 2 { return Err("phprs_str_contains() expects 2 arguments".into()); }
                match (&args[0], &args[1]) {
                    (Value::String_(h), Value::String_(n)) => Ok(Value::Int(if h.contains(n.as_str()) { 1 } else { 0 })),
                    _ => Err("phprs_str_contains() expects two strings".into()),
                }
            }
            "phprs_str_split" => {
                if args.len() != 3 { return Err("phprs_str_split() expects 3 arguments".into()); }
                match (&args[0], &args[1], &args[2]) {
                    (Value::String_(s), Value::String_(delim), Value::Int(idx)) => {
                        let parts: Vec<&str> = s.split(delim.as_str()).collect();
                        let i = *idx as usize;
                        Ok(Value::String_(parts.get(i).unwrap_or(&"").to_string()))
                    }
                    _ => Err("phprs_str_split() expects (string, string, int)".into()),
                }
            }
            "phprs_str_starts_with" => {
                if args.len() != 2 { return Err("phprs_str_starts_with() expects 2 arguments".into()); }
                match (&args[0], &args[1]) {
                    (Value::String_(s), Value::String_(prefix)) => Ok(Value::Int(if s.starts_with(prefix.as_str()) { 1 } else { 0 })),
                    _ => Err("phprs_str_starts_with() expects two strings".into()),
                }
            }
            "phprs_str_ends_with" => {
                if args.len() != 2 { return Err("phprs_str_ends_with() expects 2 arguments".into()); }
                match (&args[0], &args[1]) {
                    (Value::String_(s), Value::String_(suffix)) => Ok(Value::Int(if s.ends_with(suffix.as_str()) { 1 } else { 0 })),
                    _ => Err("phprs_str_ends_with() expects two strings".into()),
                }
            }
            "phprs_str_upper" => {
                if args.len() != 1 { return Err("phprs_str_upper() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(s) => Ok(Value::String_(s.to_uppercase())),
                    _ => Err("phprs_str_upper() expects a string".into()),
                }
            }
            "phprs_str_lower" => {
                if args.len() != 1 { return Err("phprs_str_lower() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(s) => Ok(Value::String_(s.to_lowercase())),
                    _ => Err("phprs_str_lower() expects a string".into()),
                }
            }
            // ---- HTTP Parsing ----
            "phprs_http_method" => {
                if args.len() != 1 { return Err("phprs_http_method() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(raw) => {
                        let end = raw.find(' ').unwrap_or(raw.len());
                        Ok(Value::String_(raw[..end].to_string()))
                    }
                    _ => Err("phprs_http_method() expects a string".into()),
                }
            }
            "phprs_http_path" => {
                if args.len() != 1 { return Err("phprs_http_path() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(raw) => {
                        // Guard against empty or malformed input (e.g., browser preflight / empty read)
                        if raw.len() < 2 {
                            return Ok(Value::String_("/".to_string()));
                        }
                        let after_method = raw.find(' ').unwrap_or(0);
                        if after_method + 1 >= raw.len() {
                            return Ok(Value::String_("/".to_string()));
                        }
                        let rest = &raw[after_method + 1..];
                        let end = rest.find(' ').unwrap_or(rest.len());
                        Ok(Value::String_(rest[..end].to_string()))
                    }
                    _ => Err("phprs_http_path() expects a string".into()),
                }
            }
            "phprs_http_header" => {
                if args.len() != 2 { return Err("phprs_http_header() expects 2 arguments".into()); }
                match (&args[0], &args[1]) {
                    (Value::String_(raw), Value::String_(name)) => {
                        let search = format!("\r\n{}: ", name);
                        if let Some(start) = raw.find(&search) {
                            let val_start = start + search.len();
                            let val_end = raw[val_start..].find("\r\n").unwrap_or(raw.len() - val_start);
                            Ok(Value::String_(raw[val_start..val_start + val_end].to_string()))
                        } else {
                            Ok(Value::String_(String::new()))
                        }
                    }
                    _ => Err("phprs_http_header() expects two strings".into()),
                }
            }
            "phprs_http_body" => {
                if args.len() != 1 { return Err("phprs_http_body() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(raw) => {
                        if let Some(pos) = raw.find("\r\n\r\n") {
                            Ok(Value::String_(raw[pos + 4..].to_string()))
                        } else {
                            Ok(Value::String_(String::new()))
                        }
                    }
                    _ => Err("phprs_http_body() expects a string".into()),
                }
            }
            "phprs_request_parse" => {
                if args.len() != 1 { return Err("phprs_request_parse() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(raw) => {
                        let mut result = String::new();
                        let append = |result: &mut String, k: &str, v: &str| {
                            if !result.is_empty() { result.push('&'); }
                            result.push_str(k);
                            result.push('=');
                            result.push_str(v);
                        };
                        // 1. Method
                        let method_end = raw.find(' ').unwrap_or(raw.len());
                        let method = &raw[..method_end];
                        append(&mut result, "method", method);
                        // 2. Path (full path with query string)
                        let after_method = if method_end + 1 < raw.len() { &raw[method_end + 1..] } else { "" };
                        let path_end = after_method.find(' ').unwrap_or(after_method.len());
                        let full_path = &after_method[..path_end];
                        // Split path and query
                        let (path_only, query_str) = if let Some(qm) = full_path.find('?') {
                            (&full_path[..qm], &full_path[qm + 1..])
                        } else {
                            (full_path, "")
                        };
                        append(&mut result, "path", path_only);
                        // 3. Parse query string into flat params
                        for pair in query_str.split('&') {
                            if pair.is_empty() { continue; }
                            if let Some(eq) = pair.find('=') {
                                append(&mut result, &pair[..eq], &pair[eq + 1..]);
                            }
                        }
                        // 4. Body
                        let body = if let Some(pos) = raw.find("\r\n\r\n") {
                            &raw[pos + 4..]
                        } else {
                            ""
                        };
                        append(&mut result, "body", body);
                        // 5. Content-Type header
                        let content_type = if let Some(pos) = raw.find("\r\nContent-Type: ") {
                            let start = pos + 18;
                            let end = raw[start..].find("\r\n").unwrap_or(raw.len() - start);
                            &raw[start..start + end]
                        } else if let Some(pos) = raw.find("\r\ncontent-type: ") {
                            let start = pos + 18;
                            let end = raw[start..].find("\r\n").unwrap_or(raw.len() - start);
                            &raw[start..start + end]
                        } else {
                            ""
                        };
                        append(&mut result, "content_type", content_type);
                        // 6. Host header
                        let host = if let Some(pos) = raw.find("\r\nHost: ") {
                            let start = pos + 8;
                            let end = raw[start..].find("\r\n").unwrap_or(raw.len() - start);
                            &raw[start..start + end]
                        } else if let Some(pos) = raw.find("\r\nhost: ") {
                            let start = pos + 8;
                            let end = raw[start..].find("\r\n").unwrap_or(raw.len() - start);
                            &raw[start..start + end]
                        } else {
                            ""
                        };
                        append(&mut result, "host", host);
                        // 7. If body is form-urlencoded, parse and merge params
                        if content_type.contains("x-www-form-urlencoded") && !body.is_empty() {
                            for pair in body.split('&') {
                                if pair.is_empty() { continue; }
                                if let Some(eq) = pair.find('=') {
                                    append(&mut result, &pair[..eq], &pair[eq + 1..]);
                                }
                            }
                        }
                        Ok(Value::String_(result))
                    }
                    _ => Err("phprs_request_parse() expects a string".into()),
                }
            }
            "phprs_url_decode" => {
                if args.len() != 1 { return Err("phprs_url_decode() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(s) => Ok(Value::String_(url_decode(&s))),
                    _ => Err("phprs_url_decode() expects a string".into()),
                }
            }
            "phprs_http_response" => {
                if args.len() != 3 { return Err("phprs_http_response() expects 3 arguments".into()); }
                match (&args[0], &args[1], &args[2]) {
                    (Value::Int(code), Value::String_(content_type), Value::String_(body)) => {
                        let status_text = match code {
                            200 => "OK", 201 => "Created", 204 => "No Content",
                            301 => "Moved Permanently", 302 => "Found",
                            400 => "Bad Request", 401 => "Unauthorized", 403 => "Forbidden",
                            404 => "Not Found", 405 => "Method Not Allowed",
                            500 => "Internal Server Error", 502 => "Bad Gateway", 503 => "Service Unavailable",
                            _ => "Unknown",
                        };
                        // Sanitize content_type to prevent HTTP response splitting (CRLF injection)
                        let safe_ct: String = content_type.chars().filter(|&c| c != '\r' && c != '\n').collect();
                        Ok(Value::String_(format!(
                            "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
                            code, status_text, safe_ct, body.len(), body
                        )))
                    }
                    _ => Err("phprs_http_response() expects (int, string, string)".into()),
                }
            }
            // ---- HTTP Client ----
            "phprs_dns_resolve" => {
                if args.len() != 1 { return Err("phprs_dns_resolve() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(hostname) => {
                        use std::net::ToSocketAddrs;
                        match format!("{}:0", hostname).to_socket_addrs() {
                            Ok(mut addrs) => {
                                match addrs.next() {
                                    Some(addr) => Ok(Value::String_(addr.ip().to_string())),
                                    None => Ok(Value::String_(String::new())),
                                }
                            }
                            Err(_) => Ok(Value::String_(String::new())),
                        }
                    }
                    _ => Err("phprs_dns_resolve() expects a string".into()),
                }
            }
            "phprs_tcp_connect" => {
                if args.len() != 2 { return Err("phprs_tcp_connect() expects 2 arguments".into()); }
                match (&args[0], &args[1]) {
                    (Value::String_(host), Value::Int(port)) => {
                        let addr = format!("{}:{}", host, port);
                        match TcpStream::connect(&addr) {
                            Ok(stream) => {
                                let _ = stream.set_read_timeout(Some(std::time::Duration::from_secs(3)));
                                self.socket_counter += 1;
                                let fd = self.socket_counter;
                                self.sockets.insert(fd, SocketWrapper::Tcp(stream));
                                Ok(Value::Int(fd))
                            }
                            Err(_) => Ok(Value::Int(-1)),
                        }
                    }
                    _ => Err("phprs_tcp_connect() expects (string, int)".into()),
                }
            }
            "phprs_tls_connect" => {
                if args.len() != 2 { return Err("phprs_tls_connect() expects 2 arguments".into()); }
                match (&args[0], &args[1]) {
                    (Value::String_(host), Value::Int(port)) => {
                        let addr = format!("{}:{}", host, port);
                        let tcp = match TcpStream::connect(&addr) {
                            Ok(s) => s,
                            Err(_) => return Ok(Value::Int(-1)),
                        };
                        let _ = tcp.set_read_timeout(Some(std::time::Duration::from_secs(30)));
                        let connector = match native_tls::TlsConnector::builder()
                            .danger_accept_invalid_certs(true)
                            .danger_accept_invalid_hostnames(true)
                            .build()
                        {
                            Ok(c) => c,
                            Err(_) => return Ok(Value::Int(-1)),
                        };
                        match connector.connect(host, tcp) {
                            Ok(tls_stream) => {
                                self.socket_counter += 1;
                                let fd = self.socket_counter;
                                self.sockets.insert(fd, SocketWrapper::Tls(Box::new(tls_stream)));
                                Ok(Value::Int(fd))
                            }
                            Err(_) => Ok(Value::Int(-1)),
                        }
                    }
                    _ => Err("phprs_tls_connect() expects (string, int)".into()),
                }
            }
            "phprs_socket_read" => {
                if args.len() != 2 { return Err("phprs_socket_read() expects 2 arguments".into()); }
                match (&args[0], &args[1]) {
                    (Value::Int(fd), Value::Int(max_size)) => {
                        match self.sockets.get_mut(fd) {
                            Some(stream) => {
                                let mut buf = vec![0u8; *max_size as usize];
                                match stream.read(&mut buf) {
                                    Ok(n) => {
                                        buf.truncate(n);
                                        Ok(Value::String_(String::from_utf8_lossy(&buf).to_string()))
                                    }
                                    Err(_) => Ok(Value::String_(String::new())),
                                }
                            }
                            None => Ok(Value::String_(String::new())),
                        }
                    }
                    _ => Err("phprs_socket_read() expects (int, int)".into()),
                }
            }
            "phprs_socket_write" => {
                if args.len() != 2 { return Err("phprs_socket_write() expects 2 arguments".into()); }
                match (&args[0], &args[1]) {
                    (Value::Int(fd), Value::String_(data)) => {
                        match self.sockets.get_mut(fd) {
                            Some(stream) => {
                                match stream.write_all(data.as_bytes()) {
                                    Ok(()) => Ok(Value::Int(data.len() as i64)),
                                    Err(_) => Ok(Value::Int(-1)),
                                }
                            }
                            None => Ok(Value::Int(-1)),
                        }
                    }
                    _ => Err("phprs_socket_write() expects (int, string)".into()),
                }
            }
            "phprs_socket_close" => {
                if args.len() != 1 { return Err("phprs_socket_close() expects 1 argument".into()); }
                match &args[0] {
                    Value::Int(fd) => {
                        // Try closing a connected socket first, then a listener
                        if self.sockets.remove(fd).is_some() {
                            return Ok(Value::Null);
                        }
                        if self.listeners.remove(fd).is_some() {
                            return Ok(Value::Null);
                        }
                        Ok(Value::Null)
                    }
                    _ => Err("phprs_socket_close() expects an int".into()),
                }
            }
            "phprs_socket_read_all" => {
                if args.len() != 1 { return Err("phprs_socket_read_all() expects 1 argument".into()); }
                match &args[0] {
                    Value::Int(fd) => {
                        match self.sockets.get_mut(fd) {
                            Some(stream) => {
                                let mut buf = Vec::new();
                                let mut chunk = [0u8; 16384];

                                // Phase 1: Read until we have the full HTTP headers (\r\n\r\n)
                                loop {
                                    match stream.read(&mut chunk) {
                                        Ok(0) => break,
                                        Ok(n) => {
                                            buf.extend_from_slice(&chunk[..n]);
                                            // Check for end of headers
                                            if buf.windows(4).any(|w| w == b"\r\n\r\n") {
                                                break;
                                            }
                                        }
                                        Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                                        Err(_) => break,
                                    }
                                }

                                // Phase 2: Parse Content-Length or detect chunked encoding
                                if let Some(header_end) = buf.windows(4).position(|w| w == b"\r\n\r\n") {
                                    let headers = &buf[..header_end + 4];
                                    let headers_str = String::from_utf8_lossy(headers);
                                    let body_start = header_end + 4;
                                    let _body_so_far = buf.len() - body_start;

                                    // Try Content-Length first
                                    let content_length = headers_str.lines()
                                        .find(|l| l.to_lowercase().starts_with("content-length:"))
                                        .and_then(|l| l.split(':').nth(1))
                                        .and_then(|v| v.trim().parse::<usize>().ok());

                                    // Check for chunked transfer encoding
                                    let is_chunked = headers_str.lines()
                                        .any(|l| l.to_lowercase().contains("transfer-encoding:") && l.contains("chunked"));

                                    if let Some(cl) = content_length {
                                        let needed = body_start + cl;
                                        while buf.len() < needed {
                                            match stream.read(&mut chunk) {
                                                Ok(0) => break,
                                                Ok(n) => buf.extend_from_slice(&chunk[..n]),
                                                Err(_) => break,
                                            }
                                        }
                                    } else if is_chunked {
                                        let mut all_body = buf[body_start..].to_vec();
                                        // Read more if body_so_far doesn't contain the terminating chunk
                                        while !all_body.windows(5).any(|w| w == b"0\r\n\r\n") && !all_body.ends_with(b"0\r\n\r\n") {
                                            match stream.read(&mut chunk) {
                                                Ok(0) => break,
                                                Ok(n) => {
                                                    buf.extend_from_slice(&chunk[..n]);
                                                    all_body = buf[body_start..].to_vec();
                                                }
                                                Err(_) => break,
                                            }
                                        }
                                    }
                                    // If neither Content-Length nor chunked, we already have what we can
                                }

                                Ok(Value::String_(String::from_utf8_lossy(&buf).to_string()))
                            }
                            None => Ok(Value::String_(String::new())),
                        }
                    }
                    _ => Err("phprs_socket_read_all() expects an int".into()),
                }
            }
            "phprs_http_build_request" => {
                if args.len() != 5 { return Err("phprs_http_build_request() expects 5 arguments".into()); }
                match (&args[0], &args[1], &args[2], &args[3], &args[4]) {
                    (Value::String_(method), Value::String_(host), Value::String_(path),
                     Value::String_(headers), Value::String_(body)) => {
                        let mut req = format!("{} {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n", method, path, host);
                        if !headers.is_empty() {
                            req.push_str(headers);
                        }
                        if !body.is_empty() {
                            req.push_str(&format!("Content-Length: {}\r\n", body.len()));
                        }
                        req.push_str("\r\n");
                        req.push_str(body);
                        Ok(Value::String_(req))
                    }
                    _ => Err("phprs_http_build_request() expects five strings".into()),
                }
            }
            "phprs_http_response_status" => {
                if args.len() != 1 { return Err("phprs_http_response_status() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(raw) => {
                        let after_space = raw.find(' ').unwrap_or(0);
                        let rest = &raw[after_space + 1..];
                        let end = rest.find(' ').unwrap_or(rest.len());
                        match rest[..end].parse::<i64>() {
                            Ok(n) => Ok(Value::Int(n)),
                            Err(_) => Ok(Value::Int(0)),
                        }
                    }
                    _ => Err("phprs_http_response_status() expects a string".into()),
                }
            }
            "phprs_http_response_body" => {
                if args.len() != 1 { return Err("phprs_http_response_body() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(raw) => {
                        if let Some(pos) = raw.find("\r\n\r\n") {
                            Ok(Value::String_(raw[pos + 4..].to_string()))
                        } else {
                            Ok(Value::String_(String::new()))
                        }
                    }
                    _ => Err("phprs_http_response_body() expects a string".into()),
                }
            }
            // ---- curl: High-level HTTP client ----
            "curl" => {
                if args.len() < 1 || args.len() > 2 { return Err("curl() expects 1-2 arguments".into()); }
                let url = match &args[0] { Value::String_(s) => s.clone(), _ => return Err("curl() expects a URL string".into()) };
                let options = if args.len() >= 2 { args[1].clone() } else { Value::Dict(HashMap::new()) };
                self.curl_exec(&url, &options)
            }
            "curl_async" => {
                if args.len() < 1 || args.len() > 2 { return Err("curl_async() expects 1-2 arguments".into()); }
                let url = match &args[0] { Value::String_(s) => s.clone(), _ => return Err("curl_async() expects a URL string".into()) };
                let options = if args.len() >= 2 { args[1].clone() } else { Value::Dict(HashMap::new()) };
                self.curl_async_exec(&url, &options)
            }
            "curl_wait" => {
                if args.len() != 1 { return Err("curl_wait() expects 1 argument".into()); }
                match &args[0] {
                    Value::Int(handle) => self.curl_wait_exec(*handle),
                    _ => Err("curl_wait() expects an int handle".into()),
                }
            }
            "curl_is_done" => {
                if args.len() != 1 { return Err("curl_is_done() expects 1 argument".into()); }
                match &args[0] {
                    Value::Int(handle) => {
                        match self.async_handles.get(handle) {
                            Some(state) => Ok(Value::Bool(state.lock().unwrap().done)),
                            None => Ok(Value::Bool(false)),
                        }
                    }
                    _ => Err("curl_is_done() expects an int handle".into()),
                }
            }
            // ---- File I/O ----
            "phprs_file_read" => {
                if args.len() != 1 { return Err("phprs_file_read() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(path) => {
                        match std::fs::read_to_string(path) {
                            Ok(content) => Ok(Value::String_(content)),
                            Err(_) => Ok(Value::String_(String::new())),
                        }
                    }
                    _ => Err("phprs_file_read() expects a string".into()),
                }
            }
            "phprs_file_write" => {
                if args.len() != 2 { return Err("phprs_file_write() expects 2 arguments".into()); }
                match (&args[0], &args[1]) {
                    (Value::String_(path), Value::String_(content)) => {
                        match std::fs::write(path, content) {
                            Ok(()) => Ok(Value::Int(content.len() as i64)),
                            Err(_) => Ok(Value::Int(-1)),
                        }
                    }
                    _ => Err("phprs_file_write() expects (string, string)".into()),
                }
            }
            "phprs_file_exists" => {
                if args.len() != 1 { return Err("phprs_file_exists() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(path) => {
                        Ok(Value::Int(if std::path::Path::new(path).exists() { 1 } else { 0 }))
                    }
                    _ => Err("phprs_file_exists() expects a string".into()),
                }
            }
            // ---- JSON Helpers ----
            "phprs_json_get_string" => {
                if args.len() != 2 { return Err("phprs_json_get_string() expects 2 arguments".into()); }
                match (&args[0], &args[1]) {
                    (Value::String_(json), Value::String_(key)) => {
                        Ok(Value::String_(simple_json_get(json, key)))
                    }
                    _ => Err("phprs_json_get_string() expects (string, string)".into()),
                }
            }
            "phprs_json_get_int" => {
                if args.len() != 2 { return Err("phprs_json_get_int() expects 2 arguments".into()); }
                match (&args[0], &args[1]) {
                    (Value::String_(json), Value::String_(key)) => {
                        let s = simple_json_get(json, key);
                        match s.parse::<i64>() {
                            Ok(n) => Ok(Value::Int(n)),
                            Err(_) => Ok(Value::Int(0)),
                        }
                    }
                    _ => Err("phprs_json_get_int() expects (string, string)".into()),
                }
            }
            // ---- WebSocket (basic implementations for interpreter) ----
            "phprs_is_websocket_upgrade" => {
                if args.len() != 1 { return Err("phprs_is_websocket_upgrade() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(raw) => {
                        let has_upgrade = raw.to_lowercase().contains("upgrade: websocket");
                        Ok(Value::Int(if has_upgrade { 1 } else { 0 }))
                    }
                    _ => Err("phprs_is_websocket_upgrade() expects a string".into()),
                }
            }
            "phprs_ws_handshake_response" => {
                if args.len() != 1 { return Err("phprs_ws_handshake_response() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(raw) => {
                        // Extract Sec-WebSocket-Key
                        let key = raw.lines()
                            .find(|l| l.to_lowercase().starts_with("sec-websocket-key:"))
                            .map(|l| l.split(':').nth(1).unwrap_or("").trim().to_string())
                            .unwrap_or_default();
                        if key.is_empty() {
                            return Ok(Value::String_(
                                "HTTP/1.1 400 Bad Request\r\nContent-Type: text/plain\r\n\r\nMissing Sec-WebSocket-Key".into()
                            ));
                        }
                        // Compute accept: base64(sha1(key + magic))
                        let magic = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";
                        let combined = format!("{}{}", key, magic);
                        let mut hasher = Sha1::new();
                        hasher.update(combined.as_bytes());
                        let hash = hasher.finalize();
                        let accept = base64_encode(&hash);
                        Ok(Value::String_(format!(
                            "HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: {}\r\n\r\n",
                            accept
                        )))
                    }
                    _ => Err("phprs_ws_handshake_response() expects a string".into()),
                }
            }
            "phprs_ws_read_frame" => {
                // Simplified: returns empty for now
                Ok(Value::String_(String::new()))
            }
            "phprs_ws_write_frame" => {
                if args.len() != 3 { return Err("phprs_ws_write_frame() expects 3 arguments".into()); }
                match &args[0] {
                    Value::Int(fd) => {
                        match self.sockets.get_mut(fd) {
                            Some(stream) => {
                                match &args[1] {
                                    Value::String_(payload) => {
                                        let mut frame = Vec::new();
                                        frame.push(0x81); // FIN + text opcode
                                        let len = payload.len();
                                        if len < 126 {
                                            frame.push(len as u8);
                                        } else if len < 65536 {
                                            frame.push(126);
                                            frame.extend_from_slice(&(len as u16).to_be_bytes());
                                        } else {
                                            frame.push(127);
                                            frame.extend_from_slice(&(len as u64).to_be_bytes());
                                        }
                                        frame.extend_from_slice(payload.as_bytes());
                                        match stream.write_all(&frame) {
                                            Ok(()) => Ok(Value::Int(payload.len() as i64)),
                                            Err(_) => Ok(Value::Int(-1)),
                                        }
                                    }
                                    _ => Ok(Value::Int(-1)),
                                }
                            }
                            None => Ok(Value::Int(-1)),
                        }
                    }
                    _ => Ok(Value::Int(-1)),
                }
            }
            "phprs_ws_send_pong" => {
                if args.len() != 2 { return Err("phprs_ws_send_pong() expects 2 arguments".into()); }
                match &args[0] {
                    Value::Int(fd) => {
                        match self.sockets.get_mut(fd) {
                            Some(stream) => {
                                match &args[1] {
                                    Value::String_(payload) => {
                                        let mut frame = Vec::new();
                                        frame.push(0x8A); // FIN + pong opcode
                                        frame.push(payload.len() as u8);
                                        frame.extend_from_slice(payload.as_bytes());
                                        match stream.write_all(&frame) {
                                            Ok(()) => Ok(Value::Int(payload.len() as i64)),
                                            Err(_) => Ok(Value::Int(-1)),
                                        }
                                    }
                                    _ => Ok(Value::Int(-1)),
                                }
                            }
                            None => Ok(Value::Int(-1)),
                        }
                    }
                    _ => Ok(Value::Int(-1)),
                }
            }
            "phprs_ws_close" => {
                if args.len() != 1 { return Err("phprs_ws_close() expects 1 argument".into()); }
                match &args[0] {
                    Value::Int(fd) => {
                        self.sockets.remove(fd);
                        Ok(Value::Null)
                    }
                    _ => Err("phprs_ws_close() expects an int".into()),
                }
            }
            // ---- Socket Primitives (server) ----
            "phprs_server_new" => {
                if args.len() != 1 { return Err("phprs_server_new() expects 1 argument".into()); }
                match &args[0] {
                    Value::Int(port) => {
                        let addr = format!("0.0.0.0:{}", port);
                        match TcpListener::bind(&addr) {
                            Ok(listener) => {
                                // Make non-blocking so accept() can be interrupted
                                listener.set_nonblocking(true).ok();
                                let fd = self.listener_counter;
                                self.listener_counter += 1;
                                self.listeners.insert(fd, listener);
                                Ok(Value::Int(fd))
                            }
                            Err(e) => {
                                eprintln!("phprs_server_new: bind failed: {}", e);
                                Ok(Value::Int(-1))
                            }
                        }
                    }
                    _ => Err("phprs_server_new() expects an int".into()),
                }
            }
            "phprs_server_accept" => {
                if args.len() != 1 { return Err("phprs_server_accept() expects 1 argument".into()); }
                match &args[0] {
                    Value::Int(fd) => {
                        match self.listeners.get_mut(fd) {
                            Some(listener) => {
                                // Poll until a connection arrives (non-blocking with sleep)
                                // Give up after ~3 seconds to avoid hanging forever
                                let mut attempts = 0;
                                loop {
                                    match listener.accept() {
                                        Ok((stream, _addr)) => {
                                            // Client socket stays blocking — read() waits for data
                                            let _ = stream.set_nonblocking(false);
                                            let client_fd = self.socket_counter;
                                            self.socket_counter += 1;
                                            self.sockets.insert(client_fd, SocketWrapper::Tcp(stream));
                                            return Ok(Value::Int(client_fd));
                                        }
                                        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                                            attempts += 1;
                                            if attempts > 300 {
                                                // No connection after ~3s, yield to let caller stop
                                                std::thread::sleep(std::time::Duration::from_millis(10));
                                                attempts = 0;
                                            }
                                            // Brief spin to avoid busy-wait overhead
                                            if attempts % 10 == 0 {
                                                std::thread::sleep(std::time::Duration::from_millis(1));
                                            }
                                        }
                                        Err(_) => {
                                            return Ok(Value::Int(-1));
                                        }
                                    }
                                }
                            }
                            None => Ok(Value::Int(-1)),
                        }
                    }
                    _ => Err("phprs_server_accept() expects an int".into()),
                }
            }
            // ---- Threading (no-ops in interpreter) ----
            "phprs_thread_spawn" => Ok(Value::Int(0)),
            "phprs_thread_pool_init" => Ok(Value::Int(0)),
            "phprs_thread_pool_enqueue" => Ok(Value::Int(0)),
            "phprs_thread_pool_shutdown" => Ok(Value::Null),
            "phprs_mutex_new" => Ok(Value::Int(0)),
            "phprs_mutex_lock" => Ok(Value::Null),
            "phprs_mutex_unlock" => Ok(Value::Null),
            "phprs_client_ip" => Ok(Value::String_("127.0.0.1".to_string())),
            // ---- App State (no-op in interpreter — routes/port are local vars) ----
            "phprs_app_set_routes" => Ok(Value::Null),
            "phprs_app_get_routes" => Ok(Value::String_("".to_string())),
            "phprs_app_set_port" => Ok(Value::Null),
            "phprs_app_get_port" => Ok(Value::Int(0)),
            // ---- Production Infrastructure (no-op in interpreter) ----
            "phprs_config" | "phprs_config_max_body" | "phprs_config_timeout"
            | "phprs_config_max_connections" | "phprs_log" | "phprs_log_error_msg"
            | "phprs_log_init" | "phprs_server_init_signals" | "phprs_write_pidfile"
            // Redis (no-op in interpreter)
            | "phprs_redis_init" | "phprs_redis_close"
            // MySQL (no-op in interpreter)
            | "phprs_mysql_init" | "phprs_mysql_close"
            // WebSocket manager (no-op in interpreter)
            | "phprs_ws_manager_init" | "phprs_ws_unregister"
            | "phprs_ws_update_pong" | "phprs_ws_start_heartbeat"
            => Ok(Value::Null),
            "phprs_is_shutting_down" => Ok(Value::Int(0)),
            // Redis stubs returning strings
            "phprs_redis_cmd" | "phprs_redis_get" => Ok(Value::String_("(nil)".to_string())),
            "phprs_redis_set" | "phprs_redis_setex" => Ok(Value::String_("OK".to_string())),
            "phprs_redis_del" => Ok(Value::String_("0".to_string())),
            "phprs_redis_exists" | "phprs_redis_expire" | "phprs_redis_incr"
            | "phprs_redis_decr" | "phprs_redis_ttl" => Ok(Value::Int(0)),
            "phprs_redis_keys" | "phprs_redis_hgetall" | "phprs_redis_lrange"
            => Ok(Value::String_("[]".to_string())),
            "phprs_redis_hget" => Ok(Value::String_("(nil)".to_string())),
            "phprs_redis_hset" | "phprs_redis_lpush" | "phprs_redis_rpush"
            => Ok(Value::String_("0".to_string())),
            "phprs_redis_ping" => Ok(Value::String_("PONG".to_string())),
            "phprs_redis_select" => Ok(Value::String_("OK".to_string())),
            // MySQL stubs
            "phprs_mysql_escape" => {
                let s = if !args.is_empty() { match &args[0] { Value::String_(s) => s.clone(), _ => String::new() } } else { String::new() };
                Ok(Value::String_(s))
            }
            "phprs_mysql_query" | "phprs_mysql_exec" | "phprs_mysql_select"
            => Ok(Value::String_("[]".to_string())),
            "phprs_mysql_insert" | "phprs_mysql_update" | "phprs_mysql_delete"
            => Ok(Value::String_("{\"affected_rows\":0}".to_string())),
            // WebSocket manager stubs
            "phprs_ws_register" => Ok(Value::Int(0)),
            "phprs_ws_broadcast" | "phprs_ws_broadcast_all" | "phprs_ws_count"
            => Ok(Value::Int(0)),
            "phprs_ws_rooms" => Ok(Value::String_("[]".to_string())),
            // ---- String Validation ----
            "phprs_str_is_alnum" => {
                if args.len() != 1 { return Err("phprs_str_is_alnum() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(s) => {
                        let valid = !s.is_empty() && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-');
                        Ok(Value::Int(if valid { 1 } else { 0 }))
                    }
                    _ => Ok(Value::Int(0)),
                }
            }
            // ---- Rate Limiting ----
            "phprs_rate_limit_init" => {
                // no-op in interpreter
                Ok(Value::Null)
            }
            "phprs_rate_limit_check" => {
                if args.len() != 1 { return Err("phprs_rate_limit_check() expects 1 argument".into()); }
                // Simple check: always allow in interpreter mode
                Ok(Value::Int(1))
            }
            // ---- CORS ----
            "phprs_cors_set_config" => {
                if args.len() >= 1 {
                    if let Value::String_(s) = &args[0] {
                        self.cors_origin = s.clone();
                    }
                }
                if args.len() >= 2 {
                    if let Value::String_(s) = &args[1] {
                        self.cors_methods = s.clone();
                    }
                }
                if args.len() >= 3 {
                    if let Value::String_(s) = &args[2] {
                        self.cors_headers = s.clone();
                    }
                }
                Ok(Value::Null)
            }
            "phprs_cors_get_origin" => {
                Ok(Value::String_(self.cors_origin.clone()))
            }
            "phprs_cors_get_methods" => {
                Ok(Value::String_(self.cors_methods.clone()))
            }
            "phprs_cors_get_headers" => {
                Ok(Value::String_(self.cors_headers.clone()))
            }
            "phprs_cors_is_preflight" => {
                if args.len() != 1 { return Err("phprs_cors_is_preflight() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(raw) => {
                        Ok(Value::Int(if raw.starts_with("OPTIONS ") { 1 } else { 0 }))
                    }
                    _ => Err("phprs_cors_is_preflight() expects a string".into()),
                }
            }
            // ---- String functions ----
            "chr" => {
                if args.len() != 1 { return Err("chr() expects 1 argument".into()); }
                match &args[0] {
                    Value::Int(n) => match char::from_u32(*n as u32) {
                        Some(c) => Ok(Value::String_(c.to_string())),
                        None => Ok(Value::String_(String::new())),
                    },
                    _ => Err("chr() expects an int".into()),
                }
            }
            "ord" => {
                if args.len() != 1 { return Err("ord() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(s) => {
                        let code = s.chars().next().map(|c| c as i64).unwrap_or(0);
                        Ok(Value::Int(code))
                    }
                    _ => Err("ord() expects a string".into()),
                }
            }
            "addslashes" => {
                if args.len() != 1 { return Err("addslashes() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(s) => {
                        let mut result = String::with_capacity(s.len());
                        for c in s.chars() {
                            match c {
                                '\'' => result.push_str("\\'"),
                                '"' => result.push_str("\\\""),
                                '\\' => result.push_str("\\\\"),
                                '\0' => result.push_str("\\0"),
                                _ => result.push(c),
                            }
                        }
                        Ok(Value::String_(result))
                    }
                    _ => Err("addslashes() expects a string".into()),
                }
            }
            "stripslashes" => {
                if args.len() != 1 { return Err("stripslashes() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(s) => {
                        let mut result = String::with_capacity(s.len());
                        let mut chars = s.chars();
                        while let Some(c) = chars.next() {
                            if c == '\\' {
                                match chars.next() {
                                    Some('\'') => result.push('\''),
                                    Some('"') => result.push('"'),
                                    Some('\\') => result.push('\\'),
                                    Some('0') => result.push('\0'),
                                    Some(other) => { result.push('\\'); result.push(other); }
                                    None => result.push('\\'),
                                }
                            } else {
                                result.push(c);
                            }
                        }
                        Ok(Value::String_(result))
                    }
                    _ => Err("stripslashes() expects a string".into()),
                }
            }
            // ---- Filesystem functions ----
            "copy" => {
                if args.len() != 2 { return Err("copy() expects 2 arguments".into()); }
                match (&args[0], &args[1]) {
                    (Value::String_(src), Value::String_(dst)) => {
                        match std::fs::copy(src, dst) {
                            Ok(_) => Ok(Value::Bool(true)),
                            Err(_) => Ok(Value::Bool(false)),
                        }
                    }
                    _ => Err("copy() expects (string, string)".into()),
                }
            }
            "rename" => {
                if args.len() != 2 { return Err("rename() expects 2 arguments".into()); }
                match (&args[0], &args[1]) {
                    (Value::String_(old), Value::String_(new)) => {
                        match std::fs::rename(old, new) {
                            Ok(_) => Ok(Value::Bool(true)),
                            Err(_) => Ok(Value::Bool(false)),
                        }
                    }
                    _ => Err("rename() expects (string, string)".into()),
                }
            }
            "filesize" => {
                if args.len() != 1 { return Err("filesize() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(path) => {
                        match std::fs::metadata(path) {
                            Ok(meta) => Ok(Value::Int(meta.len() as i64)),
                            Err(_) => Ok(Value::Int(-1)),
                        }
                    }
                    _ => Err("filesize() expects a string".into()),
                }
            }
            "filemtime" => {
                if args.len() != 1 { return Err("filemtime() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(path) => {
                        match std::fs::metadata(path) {
                            Ok(meta) => {
                                match meta.modified() {
                                    Ok(time) => {
                                        match time.duration_since(std::time::UNIX_EPOCH) {
                                            Ok(d) => Ok(Value::Int(d.as_secs() as i64)),
                                            Err(_) => Ok(Value::Int(-1)),
                                        }
                                    }
                                    Err(_) => Ok(Value::Int(-1)),
                                }
                            }
                            Err(_) => Ok(Value::Int(-1)),
                        }
                    }
                    _ => Err("filemtime() expects a string".into()),
                }
            }
            "pathinfo" => {
                if args.len() != 1 { return Err("pathinfo() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(path) => {
                        let p = std::path::Path::new(path);
                        let dirname = p.parent().and_then(|d| d.to_str()).unwrap_or("");
                        let basename = p.file_name().and_then(|f| f.to_str()).unwrap_or("");
                        let filename = if let Some(stem) = p.file_stem() {
                            stem.to_str().unwrap_or("")
                        } else { "" };
                        let extension = p.extension().and_then(|e| e.to_str()).unwrap_or("");
                        let esc = |s: &str| -> String {
                            s.chars().map(|c| match c {
                                '"' => "\\\"".to_string(),
                                '\\' => "\\\\".to_string(),
                                '\n' => "\\n".to_string(),
                                '\r' => "\\r".to_string(),
                                '\t' => "\\t".to_string(),
                                c => c.to_string(),
                            }).collect()
                        };
                        let json = format!(
                            r#"{{"dirname":"{}","basename":"{}","extension":"{}","filename":"{}"}}"#,
                            esc(dirname), esc(basename), esc(extension), esc(filename)
                        );
                        Ok(Value::String_(json))
                    }
                    _ => Err("pathinfo() expects a string".into()),
                }
            }
            "move_uploaded_file" => {
                if args.len() != 2 { return Err("move_uploaded_file() expects 2 arguments".into()); }
                match (&args[0], &args[1]) {
                    (Value::String_(tmp), Value::String_(dest)) => {
                        // Verify source exists before moving (basic safety check)
                        if !std::path::Path::new(tmp).exists() {
                            return Ok(Value::Bool(false));
                        }
                        match std::fs::rename(tmp, dest) {
                            Ok(_) => Ok(Value::Bool(true)),
                            Err(_) => Ok(Value::Bool(false)),
                        }
                    }
                    _ => Err("move_uploaded_file() expects (string, string)".into()),
                }
            }
            // ---- Security functions ----
            "random_bytes" => {
                if args.len() != 1 { return Err("random_bytes() expects 1 argument".into()); }
                match &args[0] {
                    Value::Int(len) => {
                        let n = (*len).max(1).min(1024 * 1024) as usize;
                        let mut buf = vec![0u8; n];
                        match std::fs::File::open("/dev/urandom") {
                            Ok(mut f) => { let _ = f.read_exact(&mut buf); }
                            Err(_) => {
                                return Err("random_bytes(): unable to access secure random source".into());
                            }
                        }
                        let hex: String = buf.iter().map(|b| format!("{:02x}", b)).collect();
                        Ok(Value::String_(hex))
                    }
                    _ => Err("random_bytes() expects an int".into()),
                }
            }
            "random_int" => {
                if args.len() != 2 { return Err("random_int() expects 2 arguments".into()); }
                match (&args[0], &args[1]) {
                    (Value::Int(min), Value::Int(max)) => {
                        if min > max { return Err("random_int(): min must be <= max".into()); }
                        let range = (*max - *min) as u64;
                        if range == 0 { return Ok(Value::Int(*min)); }
                        let threshold = u64::MAX - (u64::MAX % (range + 1));
                        let val = loop {
                            let mut buf = [0u8; 8];
                            match std::fs::File::open("/dev/urandom") {
                                Ok(mut f) => { let _ = f.read_exact(&mut buf); }
                                Err(_) => {
                                    return Err("random_int(): unable to access secure random source".into());
                                }
                            }
                            let v = u64::from_le_bytes(buf);
                            if v < threshold { break v % (range + 1); }
                        };
                        Ok(Value::Int(*min + val as i64))
                    }
                    _ => Err("random_int() expects (int, int)".into()),
                }
            }
            "password_hash" => {
                if args.len() != 2 { return Err("password_hash() expects 2 arguments".into()); }
                match (&args[0], &args[1]) {
                    (Value::String_(password), Value::String_(algo)) => {
                        let mut salt_bytes = [0u8; 16];
                        match std::fs::File::open("/dev/urandom") {
                            Ok(mut f) => { let _ = f.read_exact(&mut salt_bytes); }
                            Err(_) => {
                                return Err("password_hash(): unable to access secure random source".into());
                            }
                        }
                        let salt_hex: String = salt_bytes.iter().map(|b| format!("{:02x}", b)).collect();
                        let algo_str = if algo == "sha1" { "sha1" } else { return Err(format!("password_hash(): unsupported algorithm '{}', only 'sha1' is supported", algo).into()); };
                        let mut hasher = Sha1::new();
                        hasher.update(format!("{}{}", salt_hex, password).as_bytes());
                        let mut hash = hasher.finalize();
                        for _ in 0..9999 {
                            let mut h = Sha1::new();
                            h.update(&hash[..]);
                            h.update(password.as_bytes());
                            hash = h.finalize();
                        }
                        let hash_hex = format!("{:x}", hash);
                        Ok(Value::String_(format!("{}${}${}", algo_str, salt_hex, hash_hex)))
                    }
                    _ => Err("password_hash() expects (string, string)".into()),
                }
            }
            "password_verify" => {
                if args.len() != 2 { return Err("password_verify() expects 2 arguments".into()); }
                match (&args[0], &args[1]) {
                    (Value::String_(password), Value::String_(stored_hash)) => {
                        let parts: Vec<&str> = stored_hash.split('$').collect();
                        if parts.len() != 3 { return Ok(Value::Bool(false)); }
                        let salt_hex = parts[1];
                        let mut hasher = Sha1::new();
                        hasher.update(format!("{}{}", salt_hex, password).as_bytes());
                        let mut hash = hasher.finalize();
                        for _ in 0..9999 {
                            let mut h = Sha1::new();
                            h.update(&hash[..]);
                            h.update(password.as_bytes());
                            hash = h.finalize();
                        }
                        let computed_hex = format!("{:x}", hash);
                        let expected = parts[2];
                        let result = if computed_hex.len() != expected.len() {
                            false
                        } else {
                            let diff = computed_hex.bytes().zip(expected.bytes())
                                .fold(0u8, |acc, (a, b)| acc | (a ^ b));
                            diff == 0
                        };
                        Ok(Value::Bool(result))
                    }
                    _ => Err("password_verify() expects (string, string)".into()),
                }
            }
            // ---- Array functions ----
            "array_chunk" => {
                if args.len() < 2 || args.len() > 3 { return Err("array_chunk() expects 2 or 3 arguments".into()); }
                let preserve_keys = if args.len() >= 3 { matches!(&args[2], Value::Bool(true)) } else { false };
                match (&args[0], &args[1]) {
                    (Value::Array(arr), Value::Int(size)) => {
                        let chunk_size = (*size).max(1) as usize;
                        let mut result = Vec::new();
                        let len = arr.len();
                        let mut i = 0;
                        while i < len {
                            let end = (i + chunk_size).min(len);
                            if preserve_keys {
                                // Create dict-like chunk with numeric keys preserved
                                let mut chunk = Vec::new();
                                for j in i..end {
                                    chunk.push(arr[j].clone());
                                }
                                result.push(Value::Array(chunk));
                            } else {
                                let chunk: Vec<Value> = arr[i..end].to_vec();
                                result.push(Value::Array(chunk));
                            }
                            i = end;
                        }
                        Ok(Value::Array(result))
                    }
                    _ => Err("array_chunk() expects (array, int)".into()),
                }
            }
            "array_count_values" => {
                if args.len() != 1 { return Err("array_count_values() expects 1 argument".into()); }
                match &args[0] {
                    Value::Array(arr) => {
                        let mut counts: HashMap<String, Value> = HashMap::new();
                        for val in arr {
                            let key = match val {
                                Value::String_(s) => s.clone(),
                                Value::Int(n) => n.to_string(),
                                _ => continue,
                            };
                            let count = counts.entry(key).or_insert(Value::Int(0));
                            if let Value::Int(n) = count { *n += 1; }
                        }
                        Ok(Value::Dict(counts))
                    }
                    _ => Err("array_count_values() expects an array".into()),
                }
            }
            "array_product" => {
                if args.len() != 1 { return Err("array_product() expects 1 argument".into()); }
                match &args[0] {
                    Value::Array(arr) => {
                        if arr.is_empty() { return Ok(Value::Int(1)); }
                        let mut has_float = false;
                        let mut product = 1.0f64;
                        for val in arr {
                            match val {
                                Value::Int(n) => product *= *n as f64,
                                Value::Float(f) => { product *= f; has_float = true; }
                                _ => {} // skip non-numeric
                            }
                        }
                        if has_float {
                            Ok(Value::Float(product))
                        } else {
                            Ok(Value::Int(product as i64))
                        }
                    }
                    _ => Err("array_product() expects an array".into()),
                }
            }
            "array_intersect" => {
                if args.len() != 2 { return Err("array_intersect() expects 2 arguments".into()); }
                match (&args[0], &args[1]) {
                    (Value::Array(arr1), Value::Array(arr2)) => {
                        let mut result = Vec::new();
                        let set2: std::collections::HashSet<String> = arr2.iter().map(value_to_key).collect();
                        for val in arr1 {
                            if set2.contains(&value_to_key(val)) {
                                result.push(val.clone());
                            }
                        }
                        Ok(Value::Array(result))
                    }
                    _ => Err("array_intersect() expects (array, array)".into()),
                }
            }
            // ---- Batch 2: Type Casting ----
            "intval" => {
                if args.is_empty() || args.len() > 2 { return Err("intval() expects 1-2 arguments".into()); }
                let base = if args.len() >= 2 {
                    match &args[1] { Value::Int(b) => *b as u32, _ => 10 }
                } else { 10 };
                match &args[0] {
                    Value::Int(n) => Ok(Value::Int(*n)),
                    Value::Float(n) => Ok(Value::Int(*n as i64)),
                    Value::Bool(b) => Ok(Value::Int(if *b { 1 } else { 0 })),
                    Value::Null => Ok(Value::Int(0)),
                    Value::String_(s) => {
                        let trimmed = s.trim();
                        if base == 10 {
                            Ok(Value::Int(trimmed.parse::<i64>().unwrap_or_else(|_| {
                                trimmed.parse::<f64>().map(|f| f as i64).unwrap_or(0)
                            })))
                        } else {
                            Ok(Value::Int(i64::from_str_radix(trimmed.trim_start_matches("0x").trim_start_matches("0X"), base).unwrap_or(0)))
                        }
                    }
                    _ => Ok(Value::Int(0)),
                }
            }
            "floatval" => {
                if args.len() != 1 { return Err("floatval() expects 1 argument".into()); }
                match &args[0] {
                    Value::Float(n) => Ok(Value::Float(*n)),
                    Value::Int(n) => Ok(Value::Float(*n as f64)),
                    Value::Bool(b) => Ok(Value::Float(if *b { 1.0 } else { 0.0 })),
                    Value::Null => Ok(Value::Float(0.0)),
                    Value::String_(s) => Ok(Value::Float(s.trim().parse::<f64>().unwrap_or(0.0))),
                    _ => Ok(Value::Float(0.0)),
                }
            }
            "strval" => {
                if args.len() != 1 { return Err("strval() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(s) => Ok(Value::String_(s.clone())),
                    Value::Int(n) => Ok(Value::String_(n.to_string())),
                    Value::Float(n) => Ok(Value::String_(n.to_string())),
                    Value::Bool(b) => Ok(Value::String_(if *b { "1".to_string() } else { "".to_string() })),
                    Value::Null => Ok(Value::String_(String::new())),
                    _ => Ok(Value::String_("Array".to_string())),
                }
            }
            "boolval" => {
                if args.len() != 1 { return Err("boolval() expects 1 argument".into()); }
                let result = match &args[0] {
                    Value::Bool(b) => *b,
                    Value::Int(n) => *n != 0,
                    Value::Float(n) => *n != 0.0,
                    Value::String_(s) => !s.is_empty() && s != "0",
                    Value::Null => false,
                    Value::Array(a) => !a.is_empty(),
                    Value::Dict(d) => !d.is_empty(),
                    _ => true,
                };
                Ok(Value::Bool(result))
            }
            // ---- Batch 2: String Functions ----
            "str_pad" => {
                if args.len() < 2 || args.len() > 4 { return Err("str_pad() expects 2-4 arguments".into()); }
                match (&args[0], &args[1]) {
                    (Value::String_(input), Value::Int(length)) => {
                        let pad_str = if args.len() >= 3 {
                            match &args[2] { Value::String_(s) => s.clone(), _ => " ".to_string() }
                        } else { " ".to_string() };
                        let pad_type = if args.len() >= 4 {
                            match &args[3] { Value::Int(t) => *t, _ => 0 }
                        } else { 0 };
                        let target_len = *length as usize;
                        let input_len = input.chars().count();
                        if input_len >= target_len || pad_str.is_empty() {
                            return Ok(Value::String_(input.clone()));
                        }
                        let pad_needed = target_len - input_len;
                        let pad_chars: Vec<char> = pad_str.chars().collect();
                        let make_pad = |n: usize| -> String {
                            pad_chars.iter().cycle().take(n).collect()
                        };
                        let result = match pad_type {
                            1 => format!("{}{}", make_pad(pad_needed), input), // STR_PAD_LEFT
                            2 => { // STR_PAD_BOTH
                                let left = pad_needed / 2;
                                let right = pad_needed - left;
                                format!("{}{}{}", make_pad(left), input, make_pad(right))
                            }
                            _ => format!("{}{}", input, make_pad(pad_needed)), // STR_PAD_RIGHT
                        };
                        Ok(Value::String_(result))
                    }
                    _ => Err("str_pad() expects (string, int, ...)".into()),
                }
            }
            "wordwrap" => {
                if args.is_empty() || args.len() > 4 { return Err("wordwrap() expects 1-4 arguments".into()); }
                match &args[0] {
                    Value::String_(s) => {
                        let width = if args.len() >= 2 { match &args[1] { Value::Int(n) => *n as usize, _ => 75 } } else { 75 };
                        let brk = if args.len() >= 3 { match &args[2] { Value::String_(b) => b.clone(), _ => "\n".to_string() } } else { "\n".to_string() };
                        let cut = if args.len() >= 4 { matches!(&args[3], Value::Bool(true)) } else { false };
                        if width == 0 && cut { return Err("wordwrap(): width cannot be 0 when cut is true".into()); }
                        let mut result = String::new();
                        let mut line_len = 0;
                        for word in s.split(' ') {
                            let wlen = word.chars().count();
                            if cut && wlen > width {
                                for ch in word.chars() {
                                    if line_len >= width {
                                        result.push_str(&brk);
                                        line_len = 0;
                                    }
                                    result.push(ch);
                                    line_len += 1;
                                }
                            } else if line_len + (if line_len > 0 { 1 } else { 0 }) + wlen > width && line_len > 0 {
                                result.push_str(&brk);
                                result.push_str(word);
                                line_len = wlen;
                            } else {
                                if line_len > 0 { result.push(' '); line_len += 1; }
                                result.push_str(word);
                                line_len += wlen;
                            }
                        }
                        Ok(Value::String_(result))
                    }
                    _ => Err("wordwrap() expects a string".into()),
                }
            }
            "str_word_count" => {
                if args.len() != 1 { return Err("str_word_count() expects 1 argument".into()); }
                match &args[0] {
                    Value::String_(s) => Ok(Value::Int(s.split_whitespace().count() as i64)),
                    _ => Err("str_word_count() expects a string".into()),
                }
            }
            "chunk_split" => {
                if args.is_empty() || args.len() > 3 { return Err("chunk_split() expects 1-3 arguments".into()); }
                match &args[0] {
                    Value::String_(body) => {
                        let chunklen = if args.len() >= 2 { match &args[1] { Value::Int(n) => (*n).max(1) as usize, _ => 76 } } else { 76 };
                        let end = if args.len() >= 3 { match &args[2] { Value::String_(s) => s.clone(), _ => "\r\n".to_string() } } else { "\r\n".to_string() };
                        let chars: Vec<char> = body.chars().collect();
                        let mut result = String::new();
                        for chunk in chars.chunks(chunklen) {
                            let s: String = chunk.iter().collect();
                            result.push_str(&s);
                            result.push_str(&end);
                        }
                        Ok(Value::String_(result))
                    }
                    _ => Err("chunk_split() expects a string".into()),
                }
            }
            // ---- Batch 2: Array Functions ----
            "array_splice" => {
                if args.len() < 2 || args.len() > 3 { return Err("array_splice() expects 2-3 arguments".into()); }
                match &args[0] {
                    Value::Array(arr) => {
                        let len = arr.len() as i64;
                        let offset = match &args[1] { Value::Int(n) => *n, _ => return Err("array_splice() offset must be int".into()) };
                        let splice_len = if args.len() >= 3 { match &args[2] { Value::Int(n) => *n, _ => len } } else { len };
                        let start = if offset < 0 { (len + offset).max(0) as usize } else { (offset as usize).min(arr.len()) };
                        let count = if splice_len < 0 { ((len - start as i64) + splice_len).max(0) as usize } else { (splice_len as usize).min(arr.len() - start) };
                        let removed: Vec<Value> = arr[start..start+count].to_vec();
                        Ok(Value::Array(removed))
                    }
                    _ => Err("array_splice() expects an array".into()),
                }
            }
            "array_pad" => {
                if args.len() != 3 { return Err("array_pad() expects 3 arguments".into()); }
                match (&args[0], &args[1]) {
                    (Value::Array(arr), Value::Int(size)) => {
                        let target = (*size).unsigned_abs() as usize;
                        if arr.len() >= target {
                            return Ok(Value::Array(arr.clone()));
                        }
                        let pad_count = target - arr.len();
                        let val = args[2].clone();
                        if *size < 0 {
                            let mut result: Vec<Value> = std::iter::repeat(val).take(pad_count).collect();
                            result.extend(arr.iter().cloned());
                            Ok(Value::Array(result))
                        } else {
                            let mut result = arr.clone();
                            result.extend(std::iter::repeat(val).take(pad_count));
                            Ok(Value::Array(result))
                        }
                    }
                    _ => Err("array_pad() expects (array, int, value)".into()),
                }
            }
            "array_key_first" => {
                if args.len() != 1 { return Err("array_key_first() expects 1 argument".into()); }
                match &args[0] {
                    Value::Array(arr) => {
                        if arr.is_empty() { Ok(Value::Null) } else { Ok(Value::Int(0)) }
                    }
                    Value::Dict(d) => {
                        if d.is_empty() { Ok(Value::Null) } else { Ok(Value::String_(d.keys().next().unwrap().clone())) }
                    }
                    _ => Err("array_key_first() expects an array".into()),
                }
            }
            "array_key_last" => {
                if args.len() != 1 { return Err("array_key_last() expects 1 argument".into()); }
                match &args[0] {
                    Value::Array(arr) => {
                        if arr.is_empty() { Ok(Value::Null) } else { Ok(Value::Int(arr.len() as i64 - 1)) }
                    }
                    Value::Dict(d) => {
                        if d.is_empty() { Ok(Value::Null) } else { Ok(Value::String_(d.keys().last().unwrap().clone())) }
                    }
                    _ => Err("array_key_last() expects an array".into()),
                }
            }
            "array_is_list" => {
                if args.len() != 1 { return Err("array_is_list() expects 1 argument".into()); }
                match &args[0] {
                    Value::Array(_) => Ok(Value::Bool(true)),
                    Value::Dict(_) => Ok(Value::Bool(false)),
                    _ => Err("array_is_list() expects an array".into()),
                }
            }
            // ---- Batch 2: Math/Date ----
            "fmod" => {
                if args.len() != 2 { return Err("fmod() expects 2 arguments".into()); }
                let x = match &args[0] { Value::Float(n) => *n, Value::Int(n) => *n as f64, _ => return Err("fmod() expects numbers".into()) };
                let y = match &args[1] { Value::Float(n) => *n, Value::Int(n) => *n as f64, _ => return Err("fmod() expects numbers".into()) };
                Ok(Value::Float(x % y))
            }
            "intdiv" => {
                if args.len() != 2 { return Err("intdiv() expects 2 arguments".into()); }
                match (&args[0], &args[1]) {
                    (Value::Int(a), Value::Int(b)) => {
                        if *b == 0 { return Err("intdiv(): division by zero".into()); }
                        Ok(Value::Int(a / b))
                    }
                    _ => Err("intdiv() expects (int, int)".into()),
                }
            }
            "checkdate" => {
                if args.len() != 3 { return Err("checkdate() expects 3 arguments".into()); }
                match (&args[0], &args[1], &args[2]) {
                    (Value::Int(month), Value::Int(day), Value::Int(year)) => {
                        let m = *month;
                        let d = *day;
                        let y = *year;
                        if y < 1 || y > 32767 || m < 1 || m > 12 || d < 1 {
                            return Ok(Value::Bool(false));
                        }
                        let days_in_month = match m {
                            1|3|5|7|8|10|12 => 31,
                            4|6|9|11 => 30,
                            2 => if (y % 4 == 0 && y % 100 != 0) || y % 400 == 0 { 29 } else { 28 },
                            _ => return Ok(Value::Bool(false)),
                        };
                        Ok(Value::Bool(d <= days_in_month))
                    }
                    _ => Err("checkdate() expects (int, int, int)".into()),
                }
            }
            "mktime" => {
                if args.len() != 6 { return Err("mktime() expects 6 arguments".into()); }
                match (&args[0], &args[1], &args[2], &args[3], &args[4], &args[5]) {
                    (Value::Int(hour), Value::Int(min), Value::Int(sec), Value::Int(month), Value::Int(day), Value::Int(year)) => {
                        let y = *year;
                        let m = *month;
                        let d = *day;
                        // Days from epoch (1970-01-01) using a simplified algorithm
                        let mut days: i64 = 0;
                        // Years
                        for yr in 1970..y {
                            days += if (yr % 4 == 0 && yr % 100 != 0) || yr % 400 == 0 { 366 } else { 365 };
                        }
                        if y < 1970 {
                            for yr in y..1970 {
                                days -= if (yr % 4 == 0 && yr % 100 != 0) || yr % 400 == 0 { 366 } else { 365 };
                            }
                        }
                        // Months
                        let is_leap = (y % 4 == 0 && y % 100 != 0) || y % 400 == 0;
                        let month_days = [31, if is_leap {29} else {28}, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
                        for mi in 0..(m-1) as usize {
                            if mi < 12 { days += month_days[mi] as i64; }
                        }
                        days += d - 1;
                        let timestamp = days * 86400 + hour * 3600 + min * 60 + sec;
                        Ok(Value::Int(timestamp))
                    }
                    _ => Err("mktime() expects 6 int arguments".into()),
                }
            }
            // ---- Batch 2: Misc ----
            "printf" => {
                if args.is_empty() { return Err("printf() expects at least 1 argument".into()); }
                match &args[0] {
                    Value::String_(fmt) => {
                        let mut result = fmt.clone();
                        let mut arg_idx = 1;
                        let mut i = 0;
                        while i < result.len() {
                            if result.as_bytes().get(i) == Some(&b'%') && i + 1 < result.len() {
                                let spec = result.as_bytes()[i + 1];
                                if spec == b'%' {
                                    result.replace_range(i..i+2, "%");
                                    i += 1;
                                    continue;
                                }
                                let replacement = if arg_idx < args.len() {
                                    match &args[arg_idx] {
                                        Value::String_(s) => s.clone(),
                                        Value::Int(n) => n.to_string(),
                                        Value::Float(n) => n.to_string(),
                                        Value::Bool(b) => (if *b { "true" } else { "false" }).to_string(),
                                        Value::Null => "null".to_string(),
                                        _ => format!("{}", args[arg_idx]),
                                    }
                                } else { String::new() };
                                result.replace_range(i..i+2, &replacement);
                                i += replacement.len();
                                arg_idx += 1;
                                continue;
                            }
                            i += 1;
                        }
                        print!("{}", result);
                        Ok(Value::Null)
                    }
                    _ => Err("printf() expects a format string".into()),
                }
            }
            "str_starts_with" => {
                if args.len() != 2 { return Err("str_starts_with() expects 2 arguments".into()); }
                match (&args[0], &args[1]) {
                    (Value::String_(haystack), Value::String_(needle)) => {
                        Ok(Value::Bool(haystack.starts_with(needle.as_str())))
                    }
                    _ => Err("str_starts_with() expects (string, string)".into()),
                }
            }
            "str_ends_with" => {
                if args.len() != 2 { return Err("str_ends_with() expects 2 arguments".into()); }
                match (&args[0], &args[1]) {
                    (Value::String_(haystack), Value::String_(needle)) => {
                        Ok(Value::Bool(haystack.ends_with(needle.as_str())))
                    }
                    _ => Err("str_ends_with() expects (string, string)".into()),
                }
            }
            _ => Err(format!("Unknown builtin function: {}", name)),
        }
    }

    pub fn interpret(&mut self, program: &Program) -> Result<(), String> {
        let mut env = Environment::new();
        for stmt in &program.stmts {
            match self.eval_in_env(stmt, &mut env) {
                Ok(_) => {}
                Err(e) if e.starts_with("__return__") => { /* ignore top-level returns */ }
                Err(e) if e.starts_with("__throw__") => return Err(format!("Uncaught exception: {}", deser_string(&e[9..]))),
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    pub fn eval_program(&mut self, program: &Program) -> Result<Value, String> {
        let mut env = Environment::new();
        let mut last = Value::Null;
        for stmt in &program.stmts {
            match self.eval_in_env(stmt, &mut env) {
                Ok(val) => last = val,
                Err(e) if e.starts_with("__return__") => return Ok(deser_string(&e[10..])),
                Err(e) if e.starts_with("__throw__") => return Err(format!("Uncaught exception: {}", deser_string(&e[9..]))),
                Err(e) if e == "__break__" => return Err("'break' used outside of a loop".to_string()),
                Err(e) if e == "__continue__" => return Err("'continue' used outside of a loop".to_string()),
                Err(e) => return Err(e),
            }
        }
        Ok(last)
    }

    fn eval_in_env(&mut self, stmt: &Stmt, env: &mut Environment) -> Result<Value, String> {
        match stmt {
            Stmt::Let { name, value, mutable, .. } => {
                let val = match value {
                    Some(expr) => self.eval_expr(expr, env)?,
                    None => Value::Null,
                };
                env.define(name.clone(), val);
                let _ = mutable; // MVP: ignore mutable for now
                Ok(Value::Null)
            }
            Stmt::Function { name, params, return_type, body } => {
                let fv = FunctionValue {
                    name: name.clone(),
                    params: params.clone(),
                    body: body.clone(),
                    closure_env: None,
                };
                let _ = return_type;
                env.define(name.clone(), Value::Function(fv));
                Ok(Value::Null)
            }
            Stmt::ExprStmt(expr) => {
                self.eval_expr(expr, env)
            }
            Stmt::Block(stmts) => {
                let mut block_env = Environment::with_enclosing(env.clone());
                let mut last = Value::Null;
                for stmt in stmts {
                    last = self.eval_in_env(stmt, &mut block_env)?;
                }
                Ok(last)
            }
            Stmt::If { condition, then_branch, else_branch } => {
                let cond = self.eval_expr(condition, env)?;
                if cond.is_truthy() {
                    self.eval_in_env(then_branch, &mut Environment::with_enclosing(env.clone()))
                } else if let Some(else_b) = else_branch {
                    self.eval_in_env(else_b, &mut Environment::with_enclosing(env.clone()))
                } else {
                    Ok(Value::Null)
                }
            }
            Stmt::For { init, condition, update, body } => {
                let mut for_env = Environment::with_enclosing(env.clone());
                self.eval_in_env(init, &mut for_env)?;
                loop {
                    if let Some(cond) = condition {
                        let result = self.eval_expr(cond, &mut for_env)?;
                        if !result.is_truthy() {
                            break;
                        }
                    }
                    let mut body_env = Environment::with_enclosing(for_env.clone());
                    match self.eval_in_env(body, &mut body_env) {
                        Ok(_) => {}
                        Err(e) if e == "__break__" => break,
                        Err(e) if e == "__continue__" => {
                            if let Some(upd) = update {
                                self.eval_expr(upd, &mut for_env)?;
                            }
                            continue;
                        }
                        Err(e) => return Err(e),
                    }
                    if let Some(upd) = update {
                        self.eval_expr(upd, &mut for_env)?;
                    }
                }
                Ok(Value::Null)
            }
            Stmt::While { condition, body } => {
                loop {
                    let cond = self.eval_expr(condition, env)?;
                    if !cond.is_truthy() {
                        break;
                    }
                    let mut body_env = Environment::with_enclosing(env.clone());
                    match self.eval_in_env(body, &mut body_env) {
                        Ok(_) => {}
                        Err(e) if e == "__break__" => break,
                        Err(e) if e == "__continue__" => continue,
                        Err(e) => return Err(e),
                    }
                }
                Ok(Value::Null)
            }
            Stmt::DoWhile { body, condition } => {
                loop {
                    let mut body_env = Environment::with_enclosing(env.clone());
                    match self.eval_in_env(body, &mut body_env) {
                        Ok(_) => {}
                        Err(e) if e == "__break__" => break,
                        Err(e) if e == "__continue__" => continue,
                        Err(e) => return Err(e),
                    }
                    let cond = self.eval_expr(condition, env)?;
                    if !cond.is_truthy() {
                        break;
                    }
                }
                Ok(Value::Null)
            }
            Stmt::Foreach { iterable, key_var, value_var, body } => {
                let iter = self.eval_expr(iterable, env)?;
                match iter {
                    Value::Array(items) => {
                        for (i, item) in items.iter().enumerate() {
                            let mut loop_env = Environment::with_enclosing(env.clone());
                            if let Some(key) = key_var {
                                loop_env.define(key.clone(), Value::Int(i as i64));
                            }
                            loop_env.define(value_var.clone(), item.clone());
                            match self.eval_in_env(body, &mut loop_env) {
                                Ok(_) => {}
                                Err(e) if e == "__break__" => break,
                                Err(e) if e == "__continue__" => continue,
                                Err(e) => return Err(e),
                            }
                        }
                    }
                    Value::Dict(map) => {
                        for (key, val) in map.iter() {
                            let mut loop_env = Environment::with_enclosing(env.clone());
                            if let Some(kv) = key_var {
                                loop_env.define(kv.clone(), Value::String_(key.clone()));
                            }
                            loop_env.define(value_var.clone(), val.clone());
                            match self.eval_in_env(body, &mut loop_env) {
                                Ok(_) => {}
                                Err(e) if e == "__break__" => break,
                                Err(e) if e == "__continue__" => continue,
                                Err(e) => return Err(e),
                            }
                        }
                    }
                    Value::String_(s) => {
                        for ch in s.chars() {
                            let mut loop_env = Environment::with_enclosing(env.clone());
                            if let Some(key) = key_var {
                                loop_env.define(key.clone(), Value::Int(0)); // index is 0 for string chars
                            }
                            loop_env.define(value_var.clone(), Value::String_(ch.to_string()));
                            match self.eval_in_env(body, &mut loop_env) {
                                Ok(_) => {}
                                Err(e) if e == "__break__" => break,
                                Err(e) if e == "__continue__" => continue,
                                Err(e) => return Err(e),
                            }
                        }
                    }
                    _ => return Err(format!("Cannot iterate over {}", iter.type_name())),
                }
                Ok(Value::Null)
            }
            Stmt::Return(expr) => {
                match expr {
                    Some(e) => {
                        let val = self.eval_expr(e, env)?;
                        Err(format!("__return__{}", serde_string(&val)))
                    }
                    None => Err("__return__null".to_string()),
                }
            }
            Stmt::Break => Err("__break__".to_string()),
            Stmt::Continue => Err("__continue__".to_string()),
            Stmt::Throw(expr) => {
                let val = self.eval_expr(expr, env)?;
                Err(format!("__throw__{}", serde_string(&val)))
            }
            Stmt::TryCatch { try_body, catch_var, catch_body } => {
                let mut try_env = Environment::with_enclosing(env.clone());
                match self.eval_in_env(try_body, &mut try_env) {
                    Ok(val) => Ok(val),
                    Err(e) => {
                        if e.starts_with("__throw__") {
                            let err_val = deser_string(&e[9..]);
                            let mut catch_env = Environment::with_enclosing(env.clone());
                            catch_env.define(catch_var.clone(), err_val);
                            self.eval_in_env(catch_body, &mut catch_env)
                        } else {
                            Err(e)
                        }
                    }
                }
            }
            Stmt::Echo(expr) => {
                let val = self.eval_expr(expr, env)?;
                print!("{}", val);
                Ok(Value::Null)
            }
            Stmt::Match { expr, arms } => {
                let val = self.eval_expr(expr, env)?;
                for arm in arms {
                    let mut arm_env = Environment::with_enclosing(env.clone());
                    if self.match_pattern(&arm.pattern, &val, &mut arm_env)? {
                        return self.eval_expr(&arm.body, &mut arm_env);
                    }
                }
                Err(format!("No match arm matched value: {}", val))
            }
            Stmt::StructDef { name, fields } => {
                // For MVP, struct definitions are stored but not fully used
                // They define named types that can be constructed as Dicts
                let _ = (name, fields);
                Ok(Value::Null)
            }
            Stmt::EnumDef { name, variants } => {
                let _ = (name, variants);
                Ok(Value::Null)
            }
            Stmt::ExternFunction { name, params, return_type } => {
                let fv = FunctionValue {
                    name: name.clone(),
                    params: params.clone(),
                    body: Box::new(Stmt::Block(vec![])),
                    closure_env: None,
                };
                let _ = return_type;
                env.define(name.clone(), Value::Function(fv));
                Ok(Value::Null)
            }
        }
    }

    #[allow(dead_code)]
    fn exec_stmt(&mut self, stmt: &Stmt, env: &mut Environment) -> Result<(), String> {
        self.eval_in_env(stmt, env)?;
        Ok(())
    }

    fn match_pattern(&mut self, pattern: &MatchPattern, value: &Value, env: &mut Environment) -> Result<bool, String> {
        match pattern {
            MatchPattern::Literal(lit) => {
                let pv = self.literal_to_value(lit);
                Ok(pv == *value)
            }
            MatchPattern::Variable(name) => {
                env.set_local(name, value.clone());
                Ok(true)
            }
            MatchPattern::Range { start, end, inclusive } => {
                let val_num = match value {
                    Value::Int(n) => *n as f64,
                    Value::Float(n) => *n,
                    _ => return Ok(false),
                };
                let start_num = match start {
                    Some(s) => {
                        let sv = self.eval_expr(s, env)?;
                        match sv {
                            Value::Int(n) => n as f64,
                            Value::Float(n) => n,
                            _ => return Ok(false),
                        }
                    }
                    None => f64::NEG_INFINITY,
                };
                let end_num = match end {
                    Some(e) => {
                        let ev = self.eval_expr(e, env)?;
                        match ev {
                            Value::Int(n) => n as f64,
                            Value::Float(n) => n,
                            _ => return Ok(false),
                        }
                    }
                    None => f64::INFINITY,
                };
                if *inclusive {
                    Ok(val_num >= start_num && val_num <= end_num)
                } else {
                    Ok(val_num >= start_num && val_num < end_num)
                }
            }
            MatchPattern::Wildcard => Ok(true),
        }
    }

    fn literal_to_value(&self, lit: &Literal) -> Value {
        match lit {
            Literal::Int(n) => Value::Int(*n),
            Literal::Float(n) => Value::Float(*n),
            Literal::String_(s) => Value::String_(s.clone()),
            Literal::Bool(b) => Value::Bool(*b),
            Literal::Null => Value::Null,
        }
    }

    fn eval_expr(&mut self, expr: &Expr, env: &mut Environment) -> Result<Value, String> {
        match expr {
            Expr::Literal(lit) => Ok(self.literal_to_value(lit)),
            Expr::Variable(name) => {
                // Check local env first (variables and user-defined functions take precedence)
                if let Ok(val) = env.get(name) {
                    return Ok(val);
                }
                // Fall back to builtin functions
                if self.is_builtin(name) {
                    return Ok(Value::Function(FunctionValue {
                        name: name.clone(),
                        params: vec![],
                        body: Box::new(Stmt::Block(vec![])),
                        closure_env: None,
                    }));
                }
                Err(format!("Undefined variable: ${}", name))
            }
            Expr::Binary { left, op, right } => {
                let l = self.eval_expr(left, env)?;
                let r = self.eval_expr(right, env)?;
                self.eval_binary(op, &l, &r)
            }
            Expr::Unary { op, right } => {
                let r = self.eval_expr(right, env)?;
                self.eval_unary(op, &r)
            }
            Expr::Call { callee, args } => {
                let func = self.eval_expr(callee, env)?;
                let mut arg_vals = Vec::new();
                for arg in args {
                    arg_vals.push(self.eval_expr(arg, env)?);
                }
                self.call_function(&func, arg_vals, env)
            }
            Expr::Index { target, index } => {
                let t = self.eval_expr(target, env)?;
                let i = self.eval_expr(index, env)?;
                self.eval_index(&t, &i)
            }
            Expr::FieldAccess { target, field } => {
                let t = self.eval_expr(target, env)?;
                match t {
                    Value::Dict(map) => {
                        map.get(field).cloned().ok_or_else(|| format!("Field not found: {}", field))
                    }
                    _ => Err(format!("Cannot access field '{}' on type {}", field, t.type_name())),
                }
            }
            Expr::Array(items) => {
                let mut arr = Vec::new();
                for item in items {
                    arr.push(self.eval_expr(item, env)?);
                }
                Ok(Value::Array(arr))
            }
            Expr::Dict(pairs) => {
                let mut map = std::collections::HashMap::new();
                for (key, val) in pairs {
                    let k = self.eval_expr(key, env)?;
                    let v = self.eval_expr(val, env)?;
                    map.insert(value_to_string_key(&k), v);
                }
                Ok(Value::Dict(map))
            }
            Expr::Assign { target, op, value } => {
                let val = match op {
                    Some(binop) => {
                        // Compound assignment: $x .= "suffix"
                        let current = self.eval_expr(target, env)?;
                        let rhs = self.eval_expr(value, env)?;
                        self.eval_binary(binop, &current, &rhs)?
                    }
                    None => self.eval_expr(value, env)?,
                };
                match target.as_ref() {
                    Expr::Variable(name) => {
                        env.assign(name, val.clone())?;
                    }
                    Expr::Index { target: t, index: i } => {
                        let t_val = self.eval_expr(t, env)?;
                        let i_val = self.eval_expr(i, env)?;
                        // For MVP: only handle array/dict assignment through env reassign
                        match t_val {
                            Value::Array(mut arr) => {
                                if let Value::Int(idx) = i_val {
                                    if idx >= 0 && (idx as usize) < arr.len() {
                                        arr[idx as usize] = val.clone();
                                    } else if idx >= 0 {
                                        arr.resize(idx as usize + 1, Value::Null);
                                        arr[idx as usize] = val.clone();
                                    }
                                }
                                // Re-assign to the env variable
                                if let Expr::Variable(vname) = t.as_ref() {
                                    env.assign(vname, Value::Array(arr))?;
                                }
                            }
                            Value::Dict(mut map) => {
                                let key = value_to_string_key(&i_val);
                                map.insert(key, val.clone());
                                if let Expr::Variable(vname) = t.as_ref() {
                                    env.assign(vname, Value::Dict(map))?;
                                }
                            }
                            _ => return Err(format!("Cannot index-assign to type {}", t_val.type_name())),
                        }
                    }
                    _ => return Err("Invalid assignment target".to_string()),
                }
                Ok(val)
            }
            Expr::IncDec { target, is_inc, is_prefix } => {
                let old_val = self.eval_expr(target, env)?;
                let one = Value::Int(1);
                let new_val = if *is_inc {
                    self.eval_binary(&BinaryOp::Add, &old_val, &one)?
                } else {
                    self.eval_binary(&BinaryOp::Sub, &old_val, &one)?
                };
                match target.as_ref() {
                    Expr::Variable(name) => {
                        env.assign(name, new_val.clone())?;
                    }
                    Expr::Index { target: t, index: i } => {
                        let t_val = self.eval_expr(t, env)?;
                        let i_val = self.eval_expr(i, env)?;
                        match t_val {
                            Value::Array(mut arr) => {
                                if let Value::Int(idx) = i_val {
                                    if idx >= 0 && (idx as usize) < arr.len() {
                                        arr[idx as usize] = new_val.clone();
                                    }
                                }
                                if let Expr::Variable(vname) = t.as_ref() {
                                    env.assign(vname, Value::Array(arr))?;
                                }
                            }
                            Value::Dict(mut map) => {
                                let key = value_to_string_key(&i_val);
                                map.insert(key, new_val.clone());
                                if let Expr::Variable(vname) = t.as_ref() {
                                    env.assign(vname, Value::Dict(map))?;
                                }
                            }
                            _ => return Err(format!("Cannot inc/dec on type {}", t_val.type_name())),
                        }
                    }
                    _ => return Err("Invalid inc/dec target".to_string()),
                }
                Ok(if *is_prefix { new_val } else { old_val })
            }
            Expr::Range { start, end, inclusive } => {
                let s = match start {
                    Some(e) => Some(self.eval_expr(e, env)?),
                    None => None,
                };
                let e = match end {
                    Some(e) => Some(self.eval_expr(e, env)?),
                    None => None,
                };
                // Ranges are currently only used in match patterns
                // Return as array for now if used in expression context
                let mut arr = Vec::new();
                if let (Some(Value::Int(si)), Some(Value::Int(ei))) = (&s, &e) {
                    let end_limit = if *inclusive { *ei + 1 } else { *ei };
                    for n in *si..end_limit {
                        arr.push(Value::Int(n));
                    }
                }
                Ok(Value::Array(arr))
            }
            Expr::MatchExpr { expr, arms } => {
                let val = self.eval_expr(expr, env)?;
                for arm in arms {
                    let mut arm_env = Environment::with_enclosing(env.clone());
                    if self.match_pattern(&arm.pattern, &val, &mut arm_env)? {
                        return self.eval_expr(&arm.body, &mut arm_env);
                    }
                }
                Err(format!("No match arm matched value: {}", val))
            }
            Expr::Closure { params, return_type, body } => {
                let _ = return_type;
                Ok(Value::Function(FunctionValue {
                    name: "<closure>".into(),
                    params: params.clone(),
                    body: body.clone(),
                    closure_env: Some(env.clone()),
                }))
            }
        }
    }

    fn eval_binary(&self, op: &BinaryOp, left: &Value, right: &Value) -> Result<Value, String> {
        match op {
            BinaryOp::Add => match (left, right) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 + b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a + *b as f64)),
                (Value::String_(a), Value::String_(b)) => Ok(Value::String_(format!("{}{}", a, b))),
                _ => Err(format!("Cannot add {} and {}", left.type_name(), right.type_name())),
            },
            BinaryOp::Sub => match (left, right) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 - b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a - *b as f64)),
                _ => Err(format!("Cannot subtract {} and {}", left.type_name(), right.type_name())),
            },
            BinaryOp::Mul => match (left, right) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
                (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
                (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 * b)),
                (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a * *b as f64)),
                _ => Err(format!("Cannot multiply {} and {}", left.type_name(), right.type_name())),
            },
            BinaryOp::Div => match (left, right) {
                (Value::Int(a), Value::Int(b)) => {
                    if *b == 0 { return Err("Division by zero".into()); }
                    Ok(Value::Int(a / b))
                }
                (Value::Float(a), Value::Float(b)) => {
                    if *b == 0.0 { return Err("Division by zero".into()); }
                    Ok(Value::Float(a / b))
                }
                (Value::Int(a), Value::Float(b)) => {
                    if *b == 0.0 { return Err("Division by zero".into()); }
                    Ok(Value::Float(*a as f64 / b))
                }
                (Value::Float(a), Value::Int(b)) => {
                    if *b == 0 { return Err("Division by zero".into()); }
                    Ok(Value::Float(a / *b as f64))
                }
                _ => Err(format!("Cannot divide {} and {}", left.type_name(), right.type_name())),
            },
            BinaryOp::Mod => match (left, right) {
                (Value::Int(a), Value::Int(b)) => {
                    if *b == 0 { return Err("Modulo by zero".into()); }
                    Ok(Value::Int(a % b))
                }
                _ => Err(format!("Cannot modulo {} and {}", left.type_name(), right.type_name())),
            },
            BinaryOp::Concat => {
                let l = value_to_string(left);
                let r = value_to_string(right);
                Ok(Value::String_(format!("{}{}", l, r)))
            }
            BinaryOp::Eq => Ok(Value::Bool(left == right)),
            BinaryOp::Neq => Ok(Value::Bool(left != right)),
            BinaryOp::StrictEq => Ok(Value::Bool(left.strict_eq(right))),
            BinaryOp::StrictNeq => Ok(Value::Bool(!left.strict_eq(right))),
            BinaryOp::Lt => self.compare(left, right, |a, b| a < b),
            BinaryOp::Gt => self.compare(left, right, |a, b| a > b),
            BinaryOp::Le => self.compare(left, right, |a, b| a <= b),
            BinaryOp::Ge => self.compare(left, right, |a, b| a >= b),
            BinaryOp::And => {
                Ok(Value::Bool(left.is_truthy() && right.is_truthy()))
            }
            BinaryOp::Or => {
                Ok(Value::Bool(left.is_truthy() || right.is_truthy()))
            }
        }
    }

    fn compare<F>(&self, left: &Value, right: &Value, cmp: F) -> Result<Value, String>
    where F: Fn(f64, f64) -> bool
    {
        match (left, right) {
            (Value::Null, _) | (_, Value::Null) => Ok(Value::Bool(false)),
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(cmp(*a as f64, *b as f64))),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(cmp(*a, *b))),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Bool(cmp(*a as f64, *b))),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(cmp(*a, *b as f64))),
            (Value::String_(a), Value::String_(b)) => Ok(Value::Bool(cmp(
                a.len() as f64, b.len() as f64,
            ))),
            _ => Err(format!("Cannot compare {} and {}", left.type_name(), right.type_name())),
        }
    }

    fn eval_unary(&self, op: &UnaryOp, right: &Value) -> Result<Value, String> {
        match op {
            UnaryOp::Neg => match right {
                Value::Int(n) => Ok(Value::Int(-n)),
                Value::Float(n) => Ok(Value::Float(-n)),
                _ => Err(format!("Cannot negate {}", right.type_name())),
            },
            UnaryOp::Not => Ok(Value::Bool(!right.is_truthy())),
        }
    }

    fn eval_index(&self, target: &Value, index: &Value) -> Result<Value, String> {
        match target {
            Value::Array(arr) => {
                match index {
                    Value::Int(n) => {
                        if *n >= 0 && (*n as usize) < arr.len() {
                            Ok(arr[*n as usize].clone())
                        } else {
                            Err(format!("Index out of bounds: {} (len: {})", n, arr.len()))
                        }
                    }
                    _ => Err("Array index must be integer".into()),
                }
            }
            Value::Dict(map) => {
                let key = value_to_string_key(index);
                map.get(&key).cloned().ok_or_else(|| format!("Key not found: {}", key))
            }
            Value::String_(s) => {
                match index {
                    Value::Int(n) => {
                        let chars: Vec<char> = s.chars().collect();
                        if *n >= 0 && (*n as usize) < chars.len() {
                            Ok(Value::String_(chars[*n as usize].to_string()))
                        } else {
                            Err(format!("String index out of bounds: {} (len: {})", n, chars.len()))
                        }
                    }
                    _ => Err("String index must be integer".into()),
                }
            }
            _ => Err(format!("Cannot index type {}", target.type_name())),
        }
    }

    /// Parse a URL into (protocol, host, port, path)
    fn parse_url(url: &str) -> (String, String, i64, String) {
        let rest = url.trim();
        let (proto, rest) = if rest.starts_with("https://") {
            ("https".to_string(), &rest["https://".len()..])
        } else if rest.starts_with("http://") {
            ("http".to_string(), &rest["http://".len()..])
        } else {
            ("http".to_string(), rest)
        };
        let (host_port, path) = match rest.find('/') {
            Some(idx) => (&rest[..idx], rest[idx..].to_string()),
            None => (rest, "/".to_string()),
        };
        let (host, port) = match host_port.find(':') {
            Some(idx) => {
                let h = host_port[..idx].to_string();
                let p: i64 = host_port[idx+1..].parse().unwrap_or(0);
                (h, p)
            }
            None => {
                let default_port = if proto == "https" { 443 } else { 80 };
                (host_port.to_string(), default_port)
            }
        };
        (proto, host, port, path)
    }

    /// Execute a synchronous HTTP request. Returns dict {status, headers, body}.
    fn curl_exec(&mut self, url: &str, options: &Value) -> Result<Value, String> {
        let (proto, host, port, path) = Self::parse_url(url);

        // Extract options
        let method = Self::get_dict_str(options, "method").unwrap_or_else(|| "GET".to_string());
        let body = Self::get_dict_str(options, "body").unwrap_or_default();
        let timeout = Self::get_dict_int(options, "timeout").unwrap_or(30);
        let extra_headers = Self::get_dict_str(options, "headers").unwrap_or_default();

        // Build headers
        let headers = if body.is_empty() {
            format!("User-Agent: phprs-curl\r\nAccept: */*\r\n{}", extra_headers)
        } else {
            format!("Content-Length: {}\r\nUser-Agent: phprs-curl\r\nAccept: */*\r\n{}", body.len(), extra_headers)
        };

        let fd: i64;
        if proto == "https" {
            let addr = format!("{}:{}", host, port);
            let tcp = match TcpStream::connect(&addr) {
                Ok(s) => s,
                Err(e) => return Ok(Self::make_error_response(&format!("TCP connect failed: {}", e))),
            };
            let _ = tcp.set_read_timeout(Some(std::time::Duration::from_secs(timeout as u64)));
            let connector = match native_tls::TlsConnector::builder()
                .danger_accept_invalid_certs(true)
                .danger_accept_invalid_hostnames(true)
                .build()
            {
                Ok(c) => c,
                Err(e) => return Ok(Self::make_error_response(&format!("TLS init failed: {}", e))),
            };
            match connector.connect(&host, tcp) {
                Ok(tls_stream) => {
                    self.socket_counter += 1;
                    fd = self.socket_counter;
                    self.sockets.insert(fd, SocketWrapper::Tls(Box::new(tls_stream)));
                }
                Err(e) => return Ok(Self::make_error_response(&format!("TLS connect failed: {}", e))),
            }
        } else {
            let addr = format!("{}:{}", host, port);
            match TcpStream::connect(&addr) {
                Ok(stream) => {
                    let _ = stream.set_read_timeout(Some(std::time::Duration::from_secs(timeout as u64)));
                    self.socket_counter += 1;
                    fd = self.socket_counter;
                    self.sockets.insert(fd, SocketWrapper::Tcp(stream));
                }
                Err(e) => return Ok(Self::make_error_response(&format!("TCP connect failed: {}", e))),
            }
        }

        let req = Self::build_http_request(&method, &host, &path, &headers, &body);
        {
            let stream = self.sockets.get_mut(&fd).unwrap();
            let _ = stream.write_all(req.as_bytes());
        }

        let raw_response = self.read_http_response(fd);
        self.sockets.remove(&fd);

        Ok(Self::parse_http_response(&raw_response))
    }

    /// Execute an async HTTP request. Returns an int handle.
    fn curl_async_exec(&mut self, url: &str, options: &Value) -> Result<Value, String> {
        let url = url.to_string();
        // Extract all options into plain types before spawning thread (Value is not Send due to Environment in FunctionValue)
        let method = Self::get_dict_str(options, "method").unwrap_or_else(|| "GET".to_string());
        let body = Self::get_dict_str(options, "body").unwrap_or_default();
        let timeout = Self::get_dict_int(options, "timeout").unwrap_or(30);
        let extra_headers = Self::get_dict_str(options, "headers").unwrap_or_default();

        self.async_counter += 1;
        let handle = self.async_counter;
        let state = Arc::new(Mutex::new(AsyncCurlState { done: false, result: None }));
        self.async_handles.insert(handle, state.clone());

        let state_clone = state;
        thread::spawn(move || {
            let (proto, host, port, path) = Self::parse_url_standalone(&url);

            let headers = if body.is_empty() {
                format!("User-Agent: phprs-curl\r\nAccept: */*\r\n{}", extra_headers)
            } else {
                format!("Content-Length: {}\r\nUser-Agent: phprs-curl\r\nAccept: */*\r\n{}", body.len(), extra_headers)
            };

            let result = if proto == "https" {
                Self::do_https_request(&host, port, &path, &method, &headers, &body, timeout)
            } else {
                Self::do_http_request(&host, port, &path, &method, &headers, &body, timeout)
            };

            let mut s = state_clone.lock().unwrap();
            s.done = true;
            s.result = Some(result);
        });

        Ok(Value::Int(handle))
    }

    /// Wait for an async curl request to complete. Returns the response dict.
    fn curl_wait_exec(&mut self, handle: i64) -> Result<Value, String> {
        let state = match self.async_handles.get(&handle) {
            Some(s) => s.clone(),
            None => return Ok(Self::make_error_response("Invalid handle")),
        };

        // Busy-wait with small sleeps
        loop {
            {
                let s = state.lock().unwrap();
                if s.done {
                    let resp = s.result.clone().unwrap_or(AsyncCurlResponse { status: 0, headers: String::new(), body: String::new(), error: "No result".to_string() });
                    self.async_handles.remove(&handle);
                    return Ok(Self::async_response_to_value(&resp));
                }
            }
            thread::sleep(std::time::Duration::from_millis(10));
        }
    }

    fn parse_response_to_async(raw: &str) -> AsyncCurlResponse {
        let status = if let Some(first_space) = raw.find(' ') {
            let after_space = &raw[first_space + 1..];
            after_space[..3].trim().parse::<i64>().unwrap_or(0)
        } else {
            0
        };
        let (headers, body) = if let Some(pos) = raw.find("\r\n\r\n") {
            let header_end = raw[..pos].find("\r\n").unwrap_or(0);
            (raw[header_end + 2..pos].to_string(), raw[pos + 4..].to_string())
        } else {
            (String::new(), String::new())
        };
        AsyncCurlResponse { status, headers, body, error: String::new() }
    }

    fn async_response_to_value(resp: &AsyncCurlResponse) -> Value {
        let mut d = HashMap::new();
        d.insert("status".to_string(), Value::Int(resp.status));
        d.insert("headers".to_string(), Value::String_(resp.headers.clone()));
        d.insert("body".to_string(), Value::String_(resp.body.clone()));
        if !resp.error.is_empty() {
            d.insert("error".to_string(), Value::String_(resp.error.clone()));
        }
        Value::Dict(d)
    }

    // Standalone implementations for async (no &self needed)

    fn read_http_response(&mut self, fd: i64) -> String {
        match self.sockets.get_mut(&fd) {
            Some(stream) => Self::read_http_body_full(stream),
            None => String::new(),
        }
    }

    fn parse_url_standalone(url: &str) -> (String, String, i64, String) {
        Self::parse_url(url)
    }

    /// Read full HTTP response from a generic reader (handles Content-Length and chunked)
    fn read_http_body_full(reader: &mut dyn Read) -> String {
        let mut buf = Vec::new();
        let mut chunk = [0u8; 16384];

        // Phase 1: Read until full headers received
        loop {
            match reader.read(&mut chunk) {
                Ok(0) => break,
                Ok(n) => {
                    buf.extend_from_slice(&chunk[..n]);
                    if buf.windows(4).any(|w| w == b"\r\n\r\n") {
                        break;
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                Err(_) => break,
            }
        }

        // Phase 2: Read body according to Content-Length or chunked encoding
        if let Some(header_end) = buf.windows(4).position(|w| w == b"\r\n\r\n") {
            let headers = &buf[..header_end + 4];
            let headers_str = String::from_utf8_lossy(headers);
            let body_start = header_end + 4;

            let content_length = headers_str.lines()
                .find(|l| l.to_lowercase().starts_with("content-length:"))
                .and_then(|l| l.split(':').nth(1))
                .and_then(|v| v.trim().parse::<usize>().ok());

            let is_chunked = headers_str.lines()
                .any(|l| l.to_lowercase().contains("transfer-encoding:") && l.contains("chunked"));

            if let Some(cl) = content_length {
                let needed = body_start + cl;
                while buf.len() < needed {
                    match reader.read(&mut chunk) {
                        Ok(0) => break,
                        Ok(n) => buf.extend_from_slice(&chunk[..n]),
                        Err(_) => break,
                    }
                }
            } else if is_chunked {
                let mut all_body = buf[body_start..].to_vec();
                while !all_body.windows(5).any(|w| w == b"0\r\n\r\n") && !all_body.ends_with(b"0\r\n\r\n") {
                    match reader.read(&mut chunk) {
                        Ok(0) => break,
                        Ok(n) => {
                            buf.extend_from_slice(&chunk[..n]);
                            all_body = buf[body_start..].to_vec();
                        }
                        Err(_) => break,
                    }
                }
            }
        }

        String::from_utf8_lossy(&buf).to_string()
    }

    fn do_http_request(host: &str, port: i64, path: &str, method: &str, headers: &str, body: &str, timeout: i64) -> AsyncCurlResponse {
        let addr = format!("{}:{}", host, port);
        let mut stream = match TcpStream::connect(&addr) {
            Ok(s) => s,
            Err(e) => return AsyncCurlResponse { status: 0, headers: String::new(), body: String::new(), error: format!("TCP connect failed: {}", e) },
        };
        let _ = stream.set_read_timeout(Some(std::time::Duration::from_secs(timeout as u64)));

        let req = Self::build_http_request(method, host, path, headers, body);
        let _ = stream.write_all(req.as_bytes());

        let raw = Self::read_http_body_full(&mut stream);
        Self::parse_response_to_async(&raw)
    }

    fn do_https_request(host: &str, port: i64, path: &str, method: &str, headers: &str, body: &str, timeout: i64) -> AsyncCurlResponse {
        let addr = format!("{}:{}", host, port);
        let tcp = match TcpStream::connect(&addr) {
            Ok(s) => s,
            Err(e) => return AsyncCurlResponse { status: 0, headers: String::new(), body: String::new(), error: format!("TCP connect failed: {}", e) },
        };
        let _ = tcp.set_read_timeout(Some(std::time::Duration::from_secs(timeout as u64)));

        let connector = match native_tls::TlsConnector::builder()
            .danger_accept_invalid_certs(true)
            .danger_accept_invalid_hostnames(true)
            .build()
        {
            Ok(c) => c,
            Err(e) => return AsyncCurlResponse { status: 0, headers: String::new(), body: String::new(), error: format!("TLS init failed: {}", e) },
        };
        let mut tls = match connector.connect(host, tcp) {
            Ok(s) => s,
            Err(e) => return AsyncCurlResponse { status: 0, headers: String::new(), body: String::new(), error: format!("TLS connect failed: {}", e) },
        };

        let req = Self::build_http_request(method, host, path, headers, body);
        let _ = tls.write_all(req.as_bytes());

        let raw = Self::read_http_body_full(&mut tls);
        Self::parse_response_to_async(&raw)
    }

    fn build_http_request(method: &str, host: &str, path: &str, headers: &str, body: &str) -> String {
        let mut req = format!("{} {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n{}", method, path, host, headers);
        if !headers.contains("Content-Length") && !body.is_empty() {
            req.push_str(&format!("\r\nContent-Length: {}", body.len()));
        }
        req.push_str("\r\n\r\n");
        req.push_str(body);
        req
    }

    fn make_error_response(msg: &str) -> Value {
        let mut d = HashMap::new();
        d.insert("status".to_string(), Value::Int(0));
        d.insert("headers".to_string(), Value::String_(String::new()));
        d.insert("body".to_string(), Value::String_(String::new()));
        d.insert("error".to_string(), Value::String_(msg.to_string()));
        Value::Dict(d)
    }

    fn parse_http_response(raw: &str) -> Value {
        // Extract status code
        let status = if let Some(first_space) = raw.find(' ') {
            let after_space = &raw[first_space + 1..];
            after_space[..3].trim().parse::<i64>().unwrap_or(0)
        } else {
            0
        };

        // Extract headers (between first line and \r\n\r\n)
        let (headers, body) = if let Some(pos) = raw.find("\r\n\r\n") {
            let header_end = raw[..pos].find("\r\n").unwrap_or(0);
            let header_str = raw[header_end + 2..pos].to_string();
            let body_str = raw[pos + 4..].to_string();
            (header_str, body_str)
        } else {
            (String::new(), String::new())
        };

        let mut d = HashMap::new();
        d.insert("status".to_string(), Value::Int(status));
        d.insert("headers".to_string(), Value::String_(headers));
        d.insert("body".to_string(), Value::String_(body));
        Value::Dict(d)
    }

    fn get_dict_str(value: &Value, key: &str) -> Option<String> {
        match value {
            Value::Dict(d) => d.get(key).and_then(|v| match v {
                Value::String_(s) => Some(s.clone()),
                _ => None,
            }),
            Value::String_(json) => {
                // Simple JSON string value extraction: "key":"value" or "key": "value"
                let search = format!("\"{}\"", key);
                if let Some(pos) = json.find(&search) {
                    let after = &json[pos + search.len()..];
                    let colon = after.find(':')?;
                    let val_part = after[colon + 1..].trim();
                    if val_part.starts_with('"') {
                        let end = val_part[1..].find('"')?;
                        Some(val_part[1..end + 1].to_string())
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn get_dict_int(value: &Value, key: &str) -> Option<i64> {
        match value {
            Value::Dict(d) => d.get(key).and_then(|v| match v {
                Value::Int(n) => Some(*n),
                _ => None,
            }),
            Value::String_(json) => {
                let search = format!("\"{}\"", key);
                if let Some(pos) = json.find(&search) {
                    let after = &json[pos + search.len()..];
                    let colon = after.find(':')?;
                    let val_part = after[colon + 1..].trim();
                    val_part.split(',').next()?.split('}').next()?.trim().parse::<i64>().ok()
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn is_builtin(&self, name: &str) -> bool {
        matches!(name,
            "strlen" | "count" | "array_push" | "str_contains" | "trim"
            | "is_null" | "is_int" | "is_string" | "is_bool" | "is_float"
            | "is_array" | "gettype" | "isset" | "empty" | "unset" | "var_dump" | "print_r"
            | "substr" | "strpos" | "stripos" | "explode" | "implode" | "join"
            | "str_repeat" | "strtolower" | "strtoupper" | "htmlspecialchars" | "str_replace" | "ltrim" | "rtrim" | "strrpos" | "ucfirst" | "sprintf" | "number_format"
            | "strip_tags" | "nl2br"
            | "abs" | "ceil" | "floor" | "round" | "max" | "min"
            | "rand" | "mt_rand" | "pow" | "sqrt"
            | "array_pop" | "array_shift" | "array_unshift"
            | "array_keys" | "array_values" | "array_merge" | "array_flip"
            | "in_array" | "array_search" | "array_key_exists"
            | "array_slice" | "array_sum" | "array_unique" | "array_reverse"
            | "array_filter" | "array_map" | "array_reduce" | "range"
            | "sort" | "rsort" | "array_diff" | "array_combine" | "array_column" | "array_fill" | "array_rand"
            | "time" | "date" | "strtotime" | "microtime"
            | "json_encode" | "json_decode" | "urlencode" | "urldecode" | "parse_url" | "http_build_query" | "base64_encode" | "base64_decode" | "md5" | "sha1" | "uniqid" | "sleep" | "usleep" | "realpath" | "is_file" | "getcwd"
            | "file_get_contents" | "file_put_contents" | "file_exists"
            | "is_dir" | "mkdir" | "unlink" | "basename" | "dirname" | "scandir"
            // Socket Primitives
            | "phprs_server_new" | "phprs_server_accept" | "phprs_client_ip"
            | "phprs_socket_read" | "phprs_socket_write" | "phprs_socket_close"
            // File I/O
            | "phprs_file_read" | "phprs_file_write" | "phprs_file_exists"
            // HTTP Parsing
            | "phprs_http_method" | "phprs_http_path" | "phprs_http_header" | "phprs_http_body"
            | "phprs_url_decode" | "phprs_http_response" | "phprs_request_parse"
            // JSON Helpers
            | "phprs_json_get_string" | "phprs_json_get_int"
            // String Helpers
            | "phprs_str_replace" | "phprs_str_contains" | "phprs_str_split"
            | "phprs_str_starts_with" | "phprs_str_ends_with"
            | "phprs_str_upper" | "phprs_str_lower"
            // WebSocket
            | "phprs_is_websocket_upgrade" | "phprs_ws_handshake_response"
            | "phprs_ws_read_frame" | "phprs_ws_write_frame"
            | "phprs_ws_send_pong" | "phprs_ws_close"
            // HTTP Client
            | "phprs_dns_resolve" | "phprs_tcp_connect" | "phprs_tls_connect" | "phprs_socket_read_all"
            | "phprs_http_build_request" | "phprs_http_response_status" | "phprs_http_response_body"
            // Threading (no-op in interpreter)
            | "phprs_thread_spawn" | "phprs_mutex_new" | "phprs_mutex_lock" | "phprs_mutex_unlock"
            | "phprs_thread_pool_init" | "phprs_thread_pool_enqueue" | "phprs_thread_pool_shutdown"
            // App state (no-op in interpreter)
            | "phprs_app_set_routes" | "phprs_app_get_routes" | "phprs_app_set_port" | "phprs_app_get_port"
            // String validation
            | "phprs_str_is_alnum"
            // Rate limiting + CORS
            | "phprs_rate_limit_init" | "phprs_rate_limit_check"
            | "phprs_cors_set_config" | "phprs_cors_get_origin" | "phprs_cors_get_methods" | "phprs_cors_get_headers"
            | "phprs_cors_is_preflight"
            // curl HTTP client
            | "curl" | "curl_async" | "curl_wait" | "curl_is_done"
            // String functions
            | "chr" | "ord" | "addslashes" | "stripslashes"
            // Filesystem functions
            | "copy" | "rename" | "filesize" | "filemtime" | "pathinfo" | "move_uploaded_file"
            // Security functions
            | "password_hash" | "password_verify" | "random_bytes" | "random_int"
            // Array functions
            | "array_chunk" | "array_count_values" | "array_product" | "array_intersect"
            // Batch 2: Type casting
            | "intval" | "floatval" | "strval" | "boolval"
            // Batch 2: String functions
            | "str_pad" | "wordwrap" | "str_word_count" | "chunk_split"
            // Batch 2: Array functions
            | "array_splice" | "array_pad" | "array_key_first" | "array_key_last" | "array_is_list"
            // Batch 2: Math/Date
            | "fmod" | "intdiv" | "checkdate" | "mktime"
            // Batch 2: Misc
            | "printf" | "str_starts_with" | "str_ends_with"
            | "phprs_config" | "phprs_config_max_body" | "phprs_config_timeout"
            | "phprs_config_max_connections" | "phprs_is_shutting_down"
            | "phprs_log" | "phprs_log_error_msg" | "phprs_log_init"
            | "phprs_server_init_signals" | "phprs_write_pidfile"
            // Redis
            | "phprs_redis_init" | "phprs_redis_close" | "phprs_redis_cmd"
            | "phprs_redis_get" | "phprs_redis_set" | "phprs_redis_setex"
            | "phprs_redis_del" | "phprs_redis_exists" | "phprs_redis_keys"
            | "phprs_redis_expire" | "phprs_redis_incr" | "phprs_redis_decr"
            | "phprs_redis_hget" | "phprs_redis_hset" | "phprs_redis_hgetall"
            | "phprs_redis_lpush" | "phprs_redis_rpush" | "phprs_redis_lrange"
            | "phprs_redis_ping" | "phprs_redis_ttl" | "phprs_redis_select"
            // MySQL
            | "phprs_mysql_init" | "phprs_mysql_close" | "phprs_mysql_escape"
            | "phprs_mysql_query" | "phprs_mysql_exec" | "phprs_mysql_select"
            | "phprs_mysql_insert" | "phprs_mysql_update" | "phprs_mysql_delete"
            // WebSocket manager
            | "phprs_ws_manager_init" | "phprs_ws_register" | "phprs_ws_unregister"
            | "phprs_ws_update_pong" | "phprs_ws_broadcast" | "phprs_ws_broadcast_all"
            | "phprs_ws_count" | "phprs_ws_rooms" | "phprs_ws_start_heartbeat"
        )
    }

    fn call_function(&mut self, func: &Value, args: Vec<Value>, env: &Environment) -> Result<Value, String> {
        match func {
            Value::Function(fv) => {
                // Check if it's a builtin
                if self.is_builtin(&fv.name) {
                    return self.call_builtin(&fv.name, args);
                }

                let mut fn_env = match &fv.closure_env {
                    Some(ce) => Environment::with_enclosing(ce.clone()),
                    None => Environment::with_enclosing(env.clone()),
                };

                for (i, param) in fv.params.iter().enumerate() {
                    let val = if i < args.len() {
                        args[i].clone()
                    } else {
                        Value::Null
                    };
                    fn_env.define(param.name.clone(), val);
                }

                match self.eval_in_env(&fv.body, &mut fn_env) {
                    Ok(val) => Ok(val),
                    Err(e) => {
                        if e.starts_with("__return__") {
                            let val_str = &e["__return__".len()..];
                            Ok(deser_string(val_str))
                        } else {
                            Err(e)
                        }
                    }
                }
            }
            Value::NativeFunction(nf) => (nf.func)(args),
            _ => Err(format!("Cannot call value of type {}", func.type_name())),
        }
    }
}

fn value_to_string(v: &Value) -> String {
    match v {
        Value::Int(n) => n.to_string(),
        Value::Float(n) => n.to_string(),
        Value::String_(s) => s.clone(),
        Value::Bool(b) => b.to_string(),
        Value::Null => "".to_string(),
        _ => format!("{}", v),
    }
}

fn value_to_string_key(v: &Value) -> String {
    match v {
        Value::String_(s) => s.clone(),
        Value::Int(n) => n.to_string(),
        _ => format!("{}", v),
    }
}

fn serde_string(v: &Value) -> String {
    match v {
        Value::Int(n) => format!("i:{}", n),
        Value::Float(n) => format!("f:{}", n),
        Value::String_(s) => format!("s:{}", s),
        Value::Bool(b) => format!("b:{}", b),
        Value::Null => "null".to_string(),
        _ => format!("v:{}", v),
    }
}

fn deser_string(s: &str) -> Value {
    if s == "null" {
        return Value::Null;
    }
    if let Some(rest) = s.strip_prefix("i:") {
        return Value::Int(rest.parse().unwrap_or(0));
    }
    if let Some(rest) = s.strip_prefix("f:") {
        return Value::Float(rest.parse().unwrap_or(0.0));
    }
    if let Some(rest) = s.strip_prefix("s:") {
        return Value::String_(rest.to_string());
    }
    if let Some(rest) = s.strip_prefix("b:") {
        return Value::Bool(rest == "true");
    }
    Value::Null
}

fn url_decode(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.bytes();
    while let Some(b) = chars.next() {
        match b {
            b'+' => result.push(' '),
            b'%' => {
                let hi = chars.next().map(hex_nibble).unwrap_or(0);
                let lo = chars.next().map(hex_nibble).unwrap_or(0);
                result.push((hi << 4 | lo) as char);
            }
            _ => result.push(b as char),
        }
    }
    result
}

fn hex_nibble(b: u8) -> u8 {
    match b {
        b'0'..=b'9' => b - b'0',
        b'a'..=b'f' => b - b'a' + 10,
        b'A'..=b'F' => b - b'A' + 10,
        _ => 0,
    }
}

fn simple_json_get(json: &str, key: &str) -> String {
    let search = format!("\"{}\"", key);
    if let Some(key_pos) = json.find(&search) {
        let after_key = &json[key_pos + search.len()..];
        if let Some(colon_pos) = after_key.find(':') {
            let after_colon = after_key[colon_pos + 1..].trim();
            if after_colon.starts_with('"') {
                let end = after_colon[1..].find('"').unwrap_or(after_colon.len() - 1);
                return after_colon[1..end + 1].to_string();
            } else {
                let end = after_colon.find(|c: char| c == ',' || c == '}' || c == '\n').unwrap_or(after_colon.len());
                return after_colon[..end].trim().to_string();
            }
        }
    }
    String::new()
}

// ---- Hash Helpers ----

fn md5_hash(input: &str) -> String {
    // Compact MD5 implementation
    let s = [0x67452301u32, 0xEFCDAB89, 0x98BADCFE, 0x10325476];
    let k: [u32; 64] = [
        0xD76AA478, 0xE8C7B756, 0x242070DB, 0xC1BDCEEE, 0xF57C0FAF, 0x4787C62A, 0xA8304613, 0xFD469501,
        0x698098D8, 0x8B44F7AF, 0xFFFF5BB1, 0x895CD7BE, 0x6B901122, 0xFD987193, 0xA679438E, 0x49B40821,
        0xF61E2562, 0xC040B340, 0x265E5A51, 0xE9B6C7AA, 0xD62F105D, 0x02441453, 0xD8A1E681, 0xE7D3FBC8,
        0x21E1CDE6, 0xC33707D6, 0xF4D50D87, 0x455A14ED, 0xA9E3E905, 0xFCEFA3F8, 0x676F02D9, 0x8D2A4C8A,
        0xFFFA3942, 0x8771F681, 0x6D9D6122, 0xFDE5380C, 0xA4BEEA44, 0x4BDECFA9, 0xF6BB4B60, 0xBEBFBC70,
        0x289B7EC6, 0xEAA127FA, 0xD4EF3085, 0x04881D05, 0xD9D4D039, 0xE6DB99E5, 0x1FA27CF8, 0xC4AC5665,
        0xF4292244, 0x432AFF97, 0xAB9423A7, 0xFC93A039, 0x655B59C3, 0x8F0CCC92, 0xFFEFF47D, 0x85845DD1,
        0x6FA87E4F, 0xFE2CE6E0, 0xA3014314, 0x4E0811A1, 0xF7537E82, 0xBD3AF235, 0x2AD7D2BB, 0xEB86D391,
    ];
    let bytes = input.as_bytes();
    let mut msg = bytes.to_vec();
    let orig_len_bits = (msg.len() as u64) * 8;
    msg.push(0x80);
    while (msg.len() % 64) != 56 { msg.push(0); }
    msg.extend_from_slice(&orig_len_bits.to_le_bytes());
    let (mut a, mut b, mut c, mut d) = (s[0], s[1], s[2], s[3]);
    for chunk in msg.chunks(64) {
        let mut m = [0u32; 16];
        for (i, w) in chunk.chunks(4).enumerate() {
            m[i] = u32::from_le_bytes([w[0], w[1], w[2], w[3]]);
        }
        let (mut aa, mut bb, mut cc, mut dd) = (a, b, c, d);
        for i in 0..64 {
            let (f, g) = if i < 16 {
                ((bb & cc) | (!bb & dd), i)
            } else if i < 32 {
                ((bb & dd) | (cc & !dd), (5 * i + 1) % 16)
            } else if i < 48 {
                (bb ^ cc ^ dd, (3 * i + 5) % 16)
            } else {
                (cc ^ (bb | !dd), (7 * i) % 16)
            };
            let temp = dd;
            dd = cc;
            cc = bb;
            bb = bb.wrapping_add((aa.wrapping_add(f).wrapping_add(k[i]).wrapping_add(m[g])).rotate_left(
                [7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22,
                 5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20,
                 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23,
                 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21][i]
            ));
            aa = temp;
        }
        a = a.wrapping_add(aa); b = b.wrapping_add(bb); c = c.wrapping_add(cc); d = d.wrapping_add(dd);
    }
    let ab = a.to_le_bytes(); let bb = b.to_le_bytes();
    let cb = c.to_le_bytes(); let db = d.to_le_bytes();
    format!("{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        ab[0], ab[1], ab[2], ab[3], bb[0], bb[1], bb[2], bb[3],
        cb[0], cb[1], cb[2], cb[3], db[0], db[1], db[2], db[3])
}

fn sha1_hash(input: &str) -> String {
    let mut h: [u32; 5] = [0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0];
    let bytes = input.as_bytes();
    let mut msg = bytes.to_vec();
    let orig_len_bits = (msg.len() as u64) * 8;
    msg.push(0x80);
    while (msg.len() % 64) != 56 { msg.push(0); }
    msg.extend_from_slice(&orig_len_bits.to_be_bytes());
    for chunk in msg.chunks(64) {
        let mut w = [0u32; 80];
        for (i, word) in chunk.chunks(4).enumerate() {
            w[i] = u32::from_be_bytes([word[0], word[1], word[2], word[3]]);
        }
        for i in 16..80 {
            w[i] = (w[i-3] ^ w[i-8] ^ w[i-14] ^ w[i-16]).rotate_left(1);
        }
        let (mut a, mut b, mut c, mut d, mut e) = (h[0], h[1], h[2], h[3], h[4]);
        for i in 0..80 {
            let (f, k) = if i < 20 {
                ((b & c) | (!b & d), 0x5A827999)
            } else if i < 40 {
                (b ^ c ^ d, 0x6ED9EBA1)
            } else if i < 60 {
                ((b & c) | (b & d) | (c & d), 0x8F1BBCDC)
            } else {
                (b ^ c ^ d, 0xCA62C1D6)
            };
            let temp = a.rotate_left(5).wrapping_add(f).wrapping_add(e).wrapping_add(k).wrapping_add(w[i]);
            e = d; d = c; c = b.rotate_left(30); b = a; a = temp;
        }
        h[0] = h[0].wrapping_add(a); h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c); h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
    }
    format!("{:08x}{:08x}{:08x}{:08x}{:08x}", h[0], h[1], h[2], h[3], h[4])
}

// ---- Debug Helpers ----

fn value_var_dump(v: &Value, indent: usize) -> String {
    let prefix = "  ".repeat(indent);
    match v {
        Value::Null => format!("{}NULL\n", prefix),
        Value::Bool(b) => format!("{}bool({})\n", prefix, if *b { "true" } else { "false" }),
        Value::Int(n) => format!("{}int({})\n", prefix, n),
        Value::Float(n) => format!("{}float({})\n", prefix, n),
        Value::String_(s) => format!("{}string({}): \"{}\"\n", prefix, s.len(), s),
        Value::Array(items) => {
            let mut out = format!("{}array({}) {{\n", prefix, items.len());
            for (i, item) in items.iter().enumerate() {
                out.push_str(&format!("{}  [{}] => ", prefix, i));
                let inner = value_var_dump(item, indent + 1);
                out.push_str(inner.trim_start());
            }
            out.push_str(&format!("{}}}\n", prefix));
            out
        }
        Value::Dict(map) => {
            let mut out = format!("{}dict({}) {{\n", prefix, map.len());
            for (k, v) in map.iter() {
                out.push_str(&format!("{}  [\"{}\"] => ", prefix, k));
                let inner = value_var_dump(v, indent + 1);
                out.push_str(inner.trim_start());
            }
            out.push_str(&format!("{}}}\n", prefix));
            out
        }
        Value::Function(fv) => format!("{}function({})\n", prefix, fv.name),
        Value::NativeFunction(nf) => format!("{}native_function({})\n", prefix, nf.name),
    }
}

fn value_print_r(v: &Value, indent: usize) -> String {
    let prefix = "  ".repeat(indent);
    match v {
        Value::Null => "(null)".to_string(),
        Value::Bool(b) => (if *b { "true" } else { "false" }).to_string(),
        Value::Int(n) => n.to_string(),
        Value::Float(n) => n.to_string(),
        Value::String_(s) => s.clone(),
        Value::Array(items) => {
            if items.is_empty() {
                return "Array()\n".to_string();
            }
            let mut out = format!("Array\n{}(\n", prefix);
            for (i, item) in items.iter().enumerate() {
                out.push_str(&format!("{}  [{}] => ", prefix, i));
                out.push_str(&value_print_r(item, indent + 1));
                out.push('\n');
            }
            out.push_str(&format!("{})\n", prefix));
            out
        }
        Value::Dict(map) => {
            if map.is_empty() {
                return "Array()\n".to_string();
            }
            let mut out = format!("Array\n{}(\n", prefix);
            for (k, v) in map.iter() {
                out.push_str(&format!("{}  [{}] => ", prefix, k));
                out.push_str(&value_print_r(v, indent + 1));
                out.push('\n');
            }
            out.push_str(&format!("{})\n", prefix));
            out
        }
        Value::Function(fv) => format!("<function {}>", fv.name),
        Value::NativeFunction(nf) => format!("<native function {}>", nf.name),
    }
}

// ---- JSON Helpers ----

fn value_to_json(v: &Value) -> String {
    match v {
        Value::Null => "null".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Int(n) => n.to_string(),
        Value::Float(n) => {
            if n.is_nan() || n.is_infinite() { "null".to_string() }
            else { format!("{}", n) }
        }
        Value::String_(s) => {
            let escaped = s.replace('\\', "\\\\").replace('"', "\\\"")
                .replace('\n', "\\n").replace('\r', "\\r").replace('\t', "\\t");
            format!("\"{}\"", escaped)
        }
        Value::Array(items) => {
            let parts: Vec<String> = items.iter().map(value_to_json).collect();
            format!("[{}]", parts.join(","))
        }
        Value::Dict(map) => {
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            let mut parts: Vec<String> = Vec::new();
            for k in keys {
                let v = &map[k];
                parts.push(format!("\"{}\":{}", k, value_to_json(v)));
            }
            format!("{{{}}}", parts.join(","))
        }
        _ => "\"\"".to_string(),
    }
}

fn json_parse(s: &str) -> Result<Value, String> {
    let s = s.trim();
    if s.is_empty() { return Ok(Value::Null); }
    match s.chars().next().unwrap() {
        'n' => Ok(Value::Null),
        't' => Ok(Value::Bool(true)),
        'f' => Ok(Value::Bool(false)),
        '"' => {
            let mut result = String::new();
            let mut chars = s[1..].chars();
            loop {
                match chars.next() {
                    Some('\\') => match chars.next() {
                        Some('"') => result.push('"'),
                        Some('\\') => result.push('\\'),
                        Some('/') => result.push('/'),
                        Some('n') => result.push('\n'),
                        Some('r') => result.push('\r'),
                        Some('t') => result.push('\t'),
                        Some(c) => { result.push('\\'); result.push(c); }
                        None => break,
                    },
                    Some('"') => break,
                    Some(c) => result.push(c),
                    None => break,
                }
            }
            Ok(Value::String_(result))
        }
        '[' => {
            let inner = &s[1..s.len().saturating_sub(1)].trim();
            if inner.is_empty() { return Ok(Value::Array(vec![])); }
            let mut items = Vec::new();
            let mut depth = 0;
            let mut start = 0;
            let mut in_string = false;
            let mut escaped = false;
            for (i, c) in inner.char_indices() {
                if escaped { escaped = false; continue; }
                if c == '\\' && in_string { escaped = true; continue; }
                if c == '"' { in_string = !in_string; continue; }
                if in_string { continue; }
                if c == '[' || c == '{' { depth += 1; }
                if c == ']' || c == '}' { depth -= 1; }
                if c == ',' && depth == 0 {
                    items.push(json_parse(inner[start..i].trim())?);
                    start = i + 1;
                }
            }
            if start < inner.len() {
                items.push(json_parse(inner[start..].trim())?);
            }
            Ok(Value::Array(items))
        }
        '{' => {
            let inner = &s[1..s.len().saturating_sub(1)].trim();
            if inner.is_empty() { return Ok(Value::Dict(HashMap::new())); }
            let mut map = HashMap::new();
            let mut depth = 0;
            let mut in_string = false;
            let mut escaped = false;
            let mut element_start = 0;
            for (i, c) in inner.char_indices() {
                if escaped { escaped = false; continue; }
                if c == '\\' && in_string { escaped = true; continue; }
                if c == '"' { in_string = !in_string; continue; }
                if in_string { continue; }
                if c == '[' || c == '{' { depth += 1; }
                if c == ']' || c == '}' { depth -= 1; }
                if c == ',' && depth == 0 {
                    let elem = inner[element_start..i].trim();
                    if let Some(colon_pos) = elem.find(':') {
                        let key_str = elem[..colon_pos].trim();
                        let val_str = elem[colon_pos+1..].trim();
                        if key_str.starts_with('"') && key_str.ends_with('"') {
                            let k = match json_parse(key_str)? {
                                Value::String_(s) => s,
                                v => format!("{}", v),
                            };
                            let v = json_parse(val_str)?;
                            map.insert(k, v);
                        }
                    }
                    element_start = i + 1;
                }
            }
            // last element
            if element_start < inner.len() {
                let elem = inner[element_start..].trim();
                if let Some(colon_pos) = elem.find(':') {
                    let key_str = elem[..colon_pos].trim();
                    let val_str = elem[colon_pos+1..].trim();
                    if key_str.starts_with('"') && key_str.ends_with('"') {
                        let k = match json_parse(key_str)? {
                                Value::String_(s) => s,
                                v => format!("{}", v),
                            };
                        let v = json_parse(val_str)?;
                        map.insert(k, v);
                    }
                }
            }
            Ok(Value::Dict(map))
        }
        _ => {
            // Number
            if let Ok(n) = s.parse::<i64>() {
                Ok(Value::Int(n))
            } else if let Ok(n) = s.parse::<f64>() {
                Ok(Value::Float(n))
            } else {
                Ok(Value::Null)
            }
        }
    }
}

fn base64_encode(input: &[u8]) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut output = String::with_capacity(((input.len() + 2) / 3) * 4);
    for chunk in input.chunks(3) {
        let b0 = chunk[0];
        let b1 = if chunk.len() > 1 { chunk[1] } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] } else { 0 };
        output.push(ALPHABET[(b0 >> 2) as usize] as char);
        output.push(ALPHABET[((b0 << 4) | (b1 >> 4)) as usize & 0x3F] as char);
        output.push(if chunk.len() > 1 { ALPHABET[((b1 << 2) | (b2 >> 6)) as usize & 0x3F] as char } else { '=' });
        output.push(if chunk.len() > 2 { ALPHABET[(b2 & 0x3F) as usize] as char } else { '=' });
    }
    output
}
