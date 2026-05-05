use std::fmt;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    String_(String),
    Bool(bool),
    Null,
    Array(Vec<Value>),
    Dict(HashMap<String, Value>),
    Function(FunctionValue),
    NativeFunction(NativeFunctionValue),
}

#[derive(Debug, Clone)]
pub struct FunctionValue {
    pub name: String,
    pub params: Vec<crate::parser::FnParam>,
    pub body: Box<crate::parser::Stmt>,
    pub closure_env: Option<Environment>,
}

#[derive(Debug, Clone)]
pub struct NativeFunctionValue {
    pub name: String,
    pub arity: usize,
    pub func: fn(Vec<Value>) -> Result<Value, String>,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(n) => write!(f, "{}", n),
            Value::Float(n) => write!(f, "{}", n),
            Value::String_(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Null => write!(f, "null"),
            Value::Array(items) => {
                write!(f, "[")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            Value::Dict(map) => {
                write!(f, "[")?;
                for (i, (k, v)) in map.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{} => {}", k, v)?;
                }
                write!(f, "]")
            }
            Value::Function(fv) => write!(f, "<function {}>", fv.name),
            Value::NativeFunction(nf) => write!(f, "<native function {}>", nf.name),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::String_(a), Value::String_(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Null, Value::Null) => true,
            _ => false,
        }
    }
}

impl Value {
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Int(_) => "int",
            Value::Float(_) => "float",
            Value::String_(_) => "string",
            Value::Bool(_) => "bool",
            Value::Null => "null",
            Value::Array(_) => "array",
            Value::Dict(_) => "dict",
            Value::Function(_) => "function",
            Value::NativeFunction(_) => "function",
        }
    }

    /// Strict equality (===): same type AND same value.
    pub fn strict_eq(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::String_(a), Value::String_(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Null, Value::Null) => true,
            _ => false, // Different types → not equal
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Null => false,
            Value::Bool(b) => *b,
            Value::Int(n) => *n != 0,
            Value::Float(n) => *n != 0.0,
            Value::String_(s) => !s.is_empty(),
            Value::Array(a) => !a.is_empty(),
            Value::Dict(d) => !d.is_empty(),
            _ => true,
        }
    }
}

// Environment with scoping.
// Uses Rc<RefCell<>> for the values map so that clones share the same backing store.
// This ensures assignments propagate correctly when environments are cloned for
// control flow (if/while/for/foreach) and nested blocks.
#[derive(Debug, Clone)]
pub struct Environment {
    values: Rc<RefCell<HashMap<String, Value>>>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: Rc::new(RefCell::new(HashMap::new())),
            enclosing: None,
        }
    }

    pub fn with_enclosing(enclosing: Environment) -> Self {
        Self {
            values: Rc::new(RefCell::new(HashMap::new())),
            enclosing: Some(Box::new(enclosing)),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.borrow_mut().insert(name, value);
    }

    pub fn get(&self, name: &str) -> Result<Value, String> {
        if let Some(value) = self.values.borrow().get(name) {
            return Ok(value.clone());
        }
        if let Some(enclosing) = &self.enclosing {
            return enclosing.get(name);
        }
        Err(format!("Undefined variable: ${}", name))
    }

    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), String> {
        if self.values.borrow().contains_key(name) {
            self.values.borrow_mut().insert(name.to_string(), value);
            return Ok(());
        }
        if let Some(enclosing) = &mut self.enclosing {
            return enclosing.assign(name, value);
        }
        Err(format!("Undefined variable: ${}", name))
    }

    pub fn set_local(&mut self, name: &str, value: Value) {
        self.values.borrow_mut().insert(name.to_string(), value);
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        for (i, (k, v)) in self.values.borrow().iter().enumerate() {
            if i > 0 { write!(f, ", ")?; }
            write!(f, "${} = {}", k, v)?;
        }
        write!(f, "}}")
    }
}
