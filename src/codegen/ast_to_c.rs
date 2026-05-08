/// Direct AST-to-C transpiler for PHPRS.
/// This bypasses the MIR and generates C code directly from the AST,
/// producing correct, compilable C for all Phase 1 language features.
use crate::parser::*;
use std::collections::HashMap;

pub struct CTranspiler {
    output: String,
    indent: usize,
    /// Tracks variable names that are in scope
    vars: HashMap<String, String>, // PHP var name -> C var name
    var_counter: usize,
    /// Tracks the C type of each variable in scope
    var_types: HashMap<String, String>, // PHP var name -> C type ("int64_t", "const char*", etc.)
    /// Tracks the length variable name for array variables
    var_array_lens: HashMap<String, String>, // PHP var name -> C length var name
    /// Tracks dict key/value arrays for foreach iteration
    var_dict_keys: HashMap<String, String>, // PHP var name -> C keys array name
    var_dict_vals: HashMap<String, String>, // PHP var name -> C values array name
    /// Functions that have been declared
    funcs: Vec<String>,
    /// Function return types: name -> C type
    func_return_types: HashMap<String, String>,
    /// String constants
    strings: Vec<String>,
}

impl CTranspiler {
    pub fn new() -> Self {
        Self {
            output: String::new(),
            indent: 0,
            vars: HashMap::new(),
            var_counter: 0,
            var_types: HashMap::new(),
            var_array_lens: HashMap::new(),
            var_dict_keys: HashMap::new(),
            var_dict_vals: HashMap::new(),
            funcs: Vec::new(),
            func_return_types: HashMap::new(),
            strings: Vec::new(),
        }
    }

    pub fn transpile(&mut self, program: &Program) -> String {
        // First collect all function signatures (regular + extern)
        for stmt in &program.stmts {
            match stmt {
                Stmt::Function { name, params, return_type, .. } => {
                    let ret = return_type.as_ref()
                        .map(|t| ast_ty_to_c(t))
                        .unwrap_or("void");
                    let p_str: Vec<String> = params.iter().map(|p| {
                        let ty = p.ty.as_ref().map(|t| ast_ty_to_c(t)).unwrap_or("int64_t");
                        format!("{} {}", ty, php_var_to_c(&p.name))
                    }).collect();
                    self.emitln(&format!("{} {}({});", ret, name, p_str.join(", ")));
                    self.funcs.push(name.clone());
                    self.func_return_types.insert(name.clone(), ret.to_string());
                }
                Stmt::ExternFunction { name, params, return_type } => {
                    // Extern functions are provided by the runtime — no forward declaration needed
                    let ret = return_type.as_ref()
                        .map(|t| ast_ty_to_c(t))
                        .unwrap_or("void");
                    self.funcs.push(name.clone());
                    self.func_return_types.insert(name.clone(), ret.to_string());
                    let _ = params; // suppress unused warning
                }
                _ => {}
            }
        }

        // Register builtin function return types (available without includes)
        let builtins: Vec<(&str, &str)> = vec![
            ("strlen", "int64_t"),
            ("count", "int64_t"),
            ("trim", "const char*"),
            ("str_contains", "bool"),
            ("substr", "const char*"),
            ("strpos", "int64_t"),
            ("stripos", "int64_t"),
            ("explode", "const char*"),
            ("implode", "const char*"),
            ("str_repeat", "const char*"),
            ("strtolower", "const char*"),
            ("strtoupper", "const char*"),
            ("htmlspecialchars", "const char*"),
            ("strip_tags", "const char*"),
            ("nl2br", "const char*"),
            ("str_replace", "const char*"),
            ("ltrim", "const char*"),
            ("rtrim", "const char*"),
            ("strrpos", "int64_t"),
            ("ucfirst", "const char*"),
            ("sprintf", "const char*"),
            ("number_format", "const char*"),
            ("urlencode", "const char*"),
            ("urldecode", "const char*"),
            ("parse_url", "const char*"),
            ("http_build_query", "const char*"),
            ("base64_encode", "const char*"),
            ("base64_decode", "const char*"),
            ("is_null", "bool"),
            ("is_int", "bool"),
            ("is_string", "bool"),
            ("is_bool", "bool"),
            ("is_float", "bool"),
            ("is_array", "bool"),
            ("gettype", "const char*"),
            ("isset", "bool"),
            ("empty", "bool"),
            ("unset", "void"),
            ("var_dump", "void"),
            ("print_r", "void"),
            ("abs", "int64_t"),
            ("ceil", "int64_t"),
            ("floor", "int64_t"),
            ("round", "double"),
            ("max", "int64_t"),
            ("min", "int64_t"),
            ("rand", "int64_t"),
            ("mt_rand", "int64_t"),
            ("pow", "double"),
            ("sqrt", "double"),
            ("time", "int64_t"),
            ("date", "const char*"),
            ("strtotime", "int64_t"),
            ("microtime", "const char*"),
            ("json_encode", "const char*"),
            ("json_decode", "const char*"),
            ("file_get_contents", "const char*"),
            ("file_put_contents", "int64_t"),
            ("file_exists", "bool"),
            ("is_dir", "bool"),
            ("mkdir", "bool"),
            ("unlink", "bool"),
            ("basename", "const char*"),
            ("dirname", "const char*"),
            ("scandir", "const char*"),
            ("array_push", "int64_t"),
            ("array_pop", "int64_t"),
            ("array_shift", "int64_t"),
            ("array_unshift", "int64_t"),
            ("array_keys", "const char*"),
            ("array_values", "const char*"),
            ("array_merge", "const char*"),
            ("array_flip", "const char*"),
            ("in_array", "bool"),
            ("array_search", "int64_t"),
            ("array_key_exists", "bool"),
            ("array_slice", "const char*"),
            ("array_sum", "double"),
            ("array_unique", "const char*"),
            ("array_reverse", "const char*"),
            ("array_filter", "const char*"),
            ("array_map", "const char*"),
            ("array_reduce", "int64_t"),
            ("range", "const char*"),
            ("sort", "const char*"),
            ("rsort", "const char*"),
            ("array_diff", "int64_t"),
            ("array_combine", "int64_t"),
            ("array_column", "int64_t"),
            ("array_fill", "int64_t"),
            ("array_rand", "int64_t"),
            ("md5", "const char*"),
            ("sha1", "const char*"),
            ("uniqid", "const char*"),
            ("sleep", "void"),
            ("usleep", "void"),
            ("realpath", "const char*"),
            ("is_file", "int64_t"),
            ("getcwd", "const char*"),
            ("chr", "const char*"),
            ("ord", "int64_t"),
            ("addslashes", "const char*"),
            ("stripslashes", "const char*"),
            ("copy", "bool"),
            ("rename", "bool"),
            ("filesize", "int64_t"),
            ("filemtime", "int64_t"),
            ("pathinfo", "const char*"),
            ("move_uploaded_file", "bool"),
            ("password_hash", "const char*"),
            ("password_verify", "bool"),
            ("random_bytes", "const char*"),
            ("random_int", "int64_t"),
            ("array_chunk", "const char*"),
            ("array_count_values", "const char*"),
            ("array_product", "double"),
            ("array_intersect", "const char*"),
            // Batch 2
            ("intval", "int64_t"),
            ("floatval", "double"),
            ("strval", "const char*"),
            ("boolval", "bool"),
            ("str_pad", "const char*"),
            ("wordwrap", "const char*"),
            ("str_word_count", "int64_t"),
            ("chunk_split", "const char*"),
            ("array_splice", "const char*"),
            ("array_pad", "const char*"),
            ("array_key_first", "int64_t"),
            ("array_key_last", "int64_t"),
            ("array_is_list", "bool"),
            ("fmod", "double"),
            ("intdiv", "int64_t"),
            ("checkdate", "bool"),
            ("mktime", "int64_t"),
            ("printf", "void"),
            ("str_starts_with", "bool"),
            ("str_ends_with", "bool"),
            ("phprs_client_ip", "const char*"),
            // Thread pool + app state
            ("phprs_thread_pool_init", "int64_t"),
            ("phprs_thread_pool_enqueue", "int64_t"),
            ("phprs_thread_pool_shutdown", "void"),
            ("phprs_app_set_routes", "void"),
            ("phprs_app_get_routes", "const char*"),
            ("phprs_app_set_port", "void"),
            ("phprs_app_get_port", "int64_t"),
            ("phprs_str_is_alnum", "int64_t"),
            // WebSocket
            ("phprs_is_websocket_upgrade", "int64_t"),
            ("phprs_ws_handshake_response", "const char*"),
            ("phprs_ws_read_frame", "const char*"),
            ("phprs_ws_write_frame", "int64_t"),
            ("phprs_ws_close", "void"),
            // Networking / HTTP
            ("phprs_server_new", "int64_t"),
            ("phprs_server_accept", "int64_t"),
            ("phprs_socket_read", "const char*"),
            ("phprs_socket_write", "int64_t"),
            ("phprs_socket_close", "void"),
            ("phprs_http_method", "const char*"),
            ("phprs_http_path", "const char*"),
            ("phprs_http_header", "const char*"),
            ("phprs_http_body", "const char*"),
            ("phprs_http_response", "const char*"),
            ("phprs_url_decode", "const char*"),
            ("phprs_request_parse", "const char*"),
            ("phprs_file_read", "const char*"),
            ("phprs_file_write", "int64_t"),
            ("phprs_file_exists", "int64_t"),
            // String helpers
            ("phprs_str_replace", "const char*"),
            ("phprs_str_split", "const char*"),
            ("phprs_str_contains", "int64_t"),
            ("phprs_str_starts_with", "int64_t"),
            ("phprs_str_ends_with", "int64_t"),
            ("phprs_str_upper", "const char*"),
            ("phprs_str_lower", "const char*"),
            // JSON
            ("phprs_json_get_string", "const char*"),
            ("phprs_json_get_int", "int64_t"),
            // Curl
            ("curl", "const char*"),
            ("curl_async", "int64_t"),
            ("curl_wait", "const char*"),
            ("curl_is_done", "int64_t"),
            // Threading
            ("phprs_thread_spawn", "int64_t"),
            // Rate limiting / CORS
            ("phprs_rate_limit_init", "void"),
            ("phprs_rate_limit_check", "int64_t"),
            ("phprs_cors_set_config", "void"),
            ("phprs_cors_get_origin", "const char*"),
            ("phprs_cors_get_methods", "const char*"),
            ("phprs_cors_get_headers", "const char*"),
            ("phprs_cors_is_preflight", "int64_t"),
            // Production infrastructure
            ("phprs_config", "void"),
            ("phprs_config_max_body", "void"),
            ("phprs_config_timeout", "void"),
            ("phprs_config_max_connections", "void"),
            ("phprs_is_shutting_down", "int64_t"),
            ("phprs_log", "void"),
            ("phprs_log_error_msg", "void"),
            ("phprs_log_init", "void"),
            ("phprs_server_init_signals", "void"),
            ("phprs_write_pidfile", "void"),
            // Redis client
            ("phprs_redis_init", "void"),
            ("phprs_redis_close", "void"),
            ("phprs_redis_cmd", "const char*"),
            ("phprs_redis_get", "const char*"),
            ("phprs_redis_set", "const char*"),
            ("phprs_redis_setex", "const char*"),
            ("phprs_redis_del", "const char*"),
            ("phprs_redis_exists", "int64_t"),
            ("phprs_redis_keys", "const char*"),
            ("phprs_redis_expire", "int64_t"),
            ("phprs_redis_incr", "int64_t"),
            ("phprs_redis_decr", "int64_t"),
            ("phprs_redis_hget", "const char*"),
            ("phprs_redis_hset", "const char*"),
            ("phprs_redis_hgetall", "const char*"),
            ("phprs_redis_lpush", "const char*"),
            ("phprs_redis_rpush", "const char*"),
            ("phprs_redis_lrange", "const char*"),
            ("phprs_redis_ping", "const char*"),
            ("phprs_redis_ttl", "int64_t"),
            ("phprs_redis_select", "const char*"),
            // MySQL client
            ("phprs_mysql_init", "void"),
            ("phprs_mysql_close", "void"),
            ("phprs_mysql_escape", "const char*"),
            ("phprs_mysql_query", "const char*"),
            ("phprs_mysql_exec", "const char*"),
            ("phprs_mysql_select", "const char*"),
            ("phprs_mysql_insert", "const char*"),
            ("phprs_mysql_update", "const char*"),
            ("phprs_mysql_delete", "const char*"),
            // WebSocket connection manager
            ("phprs_ws_manager_init", "void"),
            ("phprs_ws_register", "int64_t"),
            ("phprs_ws_unregister", "void"),
            ("phprs_ws_update_pong", "void"),
            ("phprs_ws_broadcast", "int64_t"),
            ("phprs_ws_broadcast_all", "int64_t"),
            ("phprs_ws_count", "int64_t"),
            ("phprs_ws_rooms", "const char*"),
            ("phprs_ws_start_heartbeat", "void"),
        ];
        for (name, ret) in builtins {
            if !self.func_return_types.contains_key(name) {
                self.func_return_types.insert(name.to_string(), ret.to_string());
            }
        }

        // Collect handler functions (signature: string -> string) for dispatch table
        let handler_funcs: Vec<String> = program.stmts.iter()
            .filter_map(|s| {
                if let Stmt::Function { name, params, return_type, .. } = s {
                    let has_string_param = params.len() == 1
                        && params[0].ty.as_ref().map(|t| matches!(t, TypeAnnotation::String_)).unwrap_or(false);
                    let returns_string = return_type.as_ref()
                        .map(|t| matches!(t, TypeAnnotation::String_))
                        .unwrap_or(false);
                    if has_string_param && returns_string {
                        return Some(name.clone());
                    }
                }
                None
            })
            .collect();

        // Collect top-level statements into __main
        let top_stmts: Vec<&Stmt> = program.stmts.iter()
            .filter(|s| !matches!(s, Stmt::Function { .. } | Stmt::ExternFunction { .. } | Stmt::StructDef { .. } | Stmt::EnumDef { .. }))
            .collect();

        if !top_stmts.is_empty() || !handler_funcs.is_empty() {
            self.emitln("");
            self.emitln("int main(int argc, char** argv) {");
            self.indent += 1;
            // Register handler functions for thread dispatch
            for name in &handler_funcs {
                self.emitln(&format!("phprs_register_handler(\"{}\", (phprs_handler_fn){});", name, name));
            }
            for stmt in &top_stmts {
                self.transpile_stmt(stmt);
            }
            self.emitln("return 0;");
            self.indent -= 1;
            self.emitln("}");
            self.emitln("");
        }

        // Transpile functions
        for stmt in &program.stmts {
            if let Stmt::Function { name, params, return_type, body } = stmt {
                self.vars.clear();
                self.transpile_function(name, params, return_type.as_ref(), body);
            }
        }

        std::mem::take(&mut self.output)
    }

    fn transpile_function(&mut self, name: &str, params: &[FnParam], ret: Option<&TypeAnnotation>, body: &Stmt) {
        let ret_c = ret.map(|t| ast_ty_to_c(t)).unwrap_or("void");
        let p_str: Vec<String> = params.iter().map(|p| {
            let ty = p.ty.as_ref().map(|t| ast_ty_to_c(t)).unwrap_or("int64_t");
            let c_name = format!("_p_{}", p.name);
            self.vars.insert(p.name.clone(), c_name.clone());
            self.var_types.insert(p.name.clone(), ty.to_string());
            format!("{} {}", ty, c_name)
        }).collect();

        self.emitln(&format!("{} {}({}) {{", ret_c, name, p_str.join(", ")));
        self.indent += 1;
        self.transpile_stmt(body);
        if ret_c == "void" || name == "__main" {
            self.emitln("return;");
        }
        self.indent -= 1;
        self.emitln("}");
        self.emitln("");
    }

    /// Infer the C type of an expression based on its structure.
    fn infer_expr_type(&self, expr: &Expr) -> String {
        match expr {
            Expr::Literal(lit) => match lit {
                Literal::Int(_) => "int64_t".to_string(),
                Literal::Float(_) => "double".to_string(),
                Literal::String_(_) => "const char*".to_string(),
                Literal::Bool(_) => "bool".to_string(),
                Literal::Null => "int64_t".to_string(),
            },
            Expr::Variable(name) => {
                self.var_types.get(name)
                    .cloned()
                    .unwrap_or_else(|| "int64_t".to_string())
            }
            Expr::Binary { op: BinaryOp::Concat, .. } => "const char*".to_string(),
            Expr::Binary { left, op: _, right } => {
                let lt = self.infer_expr_type(left);
                let rt = self.infer_expr_type(right);
                if lt == "double" || rt == "double" {
                    "double".to_string()
                } else if lt == "const char*" || rt == "const char*" {
                    "const char*".to_string()
                } else {
                    "int64_t".to_string()
                }
            }
            Expr::Call { callee, .. } => {
                if let Expr::Variable(name) = callee.as_ref() {
                    self.func_return_types.get(name)
                        .cloned()
                        .unwrap_or_else(|| "int64_t".to_string())
                } else {
                    "int64_t".to_string()
                }
            }
            Expr::MatchExpr { arms, .. } => {
                if let Some(arm) = arms.first() {
                    self.infer_expr_type(&arm.body)
                } else {
                    "int64_t".to_string()
                }
            }
            Expr::Assign { value, .. } => self.infer_expr_type(value),
            Expr::IncDec { target, .. } => self.infer_expr_type(target),
            Expr::Unary { op: _, right } => self.infer_expr_type(right),
            Expr::Index { .. } => {
                // Arrays are always int64_t in the simple C backend
                "int64_t".to_string()
            }
            _ => "int64_t".to_string(),
        }
    }

    fn transpile_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Let { name, value, .. } => {
                let c_name = format!("v_{}", self.var_counter);
                self.var_counter += 1;
                self.vars.insert(name.clone(), c_name.clone());
                // Special handling for array literals
                if let Some(Expr::Array(elements)) = value {
                    let elem_type = if let Some(first) = elements.first() {
                        self.infer_expr_type(first)
                    } else {
                        "int64_t".to_string()
                    };
                    let len_var = format!("{}_len", c_name);
                    self.var_array_lens.insert(name.clone(), len_var.clone());
                    self.var_types.insert(name.clone(), format!("{}*", elem_type));
                    self.emit(&format!("{} {}[] = {{", elem_type, c_name));
                    for (i, elem) in elements.iter().enumerate() {
                        if i > 0 { self.write(", "); }
                        self.transpile_expr(elem);
                    }
                    self.emitln("};");
                    self.emitln(&format!("int {} = {};", len_var, elements.len()));
                } else if let Some(Expr::Dict(pairs)) = value {
                    // Dict: create parallel key/value arrays for foreach iteration
                    let val_type = if let Some((_, first_val)) = pairs.first() {
                        self.infer_expr_type(first_val)
                    } else {
                        "int64_t".to_string()
                    };
                    let keys_c_name = format!("{}_keys", c_name);
                    let vals_c_name = format!("{}_vals", c_name);
                    let len_var = format!("{}_len", c_name);
                    self.var_dict_keys.insert(name.clone(), keys_c_name.clone());
                    self.var_dict_vals.insert(name.clone(), vals_c_name.clone());
                    self.var_array_lens.insert(name.clone(), len_var.clone());
                    self.var_types.insert(name.clone(), format!("{}*", val_type));
                    // Emit keys array
                    self.emit(&format!("const char* {}[] = {{", keys_c_name));
                    for (i, (key, _)) in pairs.iter().enumerate() {
                        if i > 0 { self.write(", "); }
                        self.transpile_expr(key);
                    }
                    self.emitln("};");
                    // Emit values array
                    self.emit(&format!("{} {}[] = {{", val_type, vals_c_name));
                    for (i, (_, val)) in pairs.iter().enumerate() {
                        if i > 0 { self.write(", "); }
                        self.transpile_expr(val);
                    }
                    self.emitln("};");
                    self.emitln(&format!("int {} = {};", len_var, pairs.len()));
                } else {
                    let c_type = if let Some(expr) = value {
                        self.infer_expr_type(expr)
                    } else {
                        "int64_t".to_string()
                    };
                    self.var_types.insert(name.clone(), c_type.clone());
                    self.emit(&format!("{} {} = ", c_type, c_name));
                    if let Some(expr) = value {
                        self.transpile_expr(expr);
                    } else {
                        self.write("0");
                    }
                    self.emitln(";");
                }
            }
            Stmt::Echo(expr) => {
                self.emit("printf(\"%s\", ");
                self.transpile_expr_to_string(expr);
                self.emitln(");");
            }
            Stmt::ExprStmt(expr) => {
                self.transpile_expr(expr);
                self.emitln(";");
            }
            Stmt::Break => {
                self.emitln("break;");
            }
            Stmt::Continue => {
                self.emitln("continue;");
            }
            Stmt::Throw(expr) => {
                self.emit("__throw(");
                self.transpile_expr_to_string(expr);
                self.emitln(");");
            }
            Stmt::TryCatch { try_body, catch_var, catch_body } => {
                // Use setjmp/longjmp for C try/catch
                self.emitln("{");
                self.indent += 1;
                self.emitln("jmp_buf __catch_buf;");
                self.emitln("bool __caught = false;");
                self.emitln("if (setjmp(__catch_buf) == 0) {");
                self.indent += 1;
                self.emitln("__push_catch(&__catch_buf);");
                self.transpile_stmt(try_body);
                self.emitln("__pop_catch();");
                self.indent -= 1;
                self.emitln("} else {");
                self.indent += 1;
                self.emitln("__pop_catch();");
                self.emit("const char* ");
                self.emit(&catch_var);
                self.emitln(" = __catch_error;");
                self.vars.insert(catch_var.clone(), catch_var.clone());
                self.var_types.insert(catch_var.clone(), "const char*".to_string());
                self.transpile_stmt(catch_body);
                self.indent -= 1;
                self.emitln("}");
                self.indent -= 1;
                self.emitln("}");
            }
            Stmt::Return(Some(expr)) => {
                self.emit("return ");
                self.transpile_expr(expr);
                self.emitln(";");
            }
            Stmt::Return(None) => {
                self.emitln("return;");
            }
            Stmt::Block(stmts) => {
                for s in stmts {
                    self.transpile_stmt(s);
                }
            }
            Stmt::If { condition, then_branch, else_branch } => {
                self.emit("if (");
                self.transpile_expr(condition);
                self.emitln(") {");
                self.indent += 1;
                self.transpile_stmt(then_branch);
                self.indent -= 1;
                if let Some(else_b) = else_branch {
                    self.emitln("} else {");
                    self.indent += 1;
                    self.transpile_stmt(else_b);
                    self.indent -= 1;
                }
                self.emitln("}");
            }
            Stmt::For { init, condition, update, body } => {
                self.emit("for (");
                self.transpile_for_init(init);
                self.write("; ");
                if let Some(cond) = condition {
                    self.transpile_expr(cond);
                } else {
                    self.write("1");
                }
                self.write("; ");
                if let Some(upd) = update {
                    self.transpile_expr(upd);
                }
                self.emitln(") {");
                self.indent += 1;
                self.transpile_stmt(body);
                self.indent -= 1;
                self.emitln("}");
            }
            Stmt::While { condition, body } => {
                self.emit("while (");
                self.transpile_expr(condition);
                self.emitln(") {");
                self.indent += 1;
                self.transpile_stmt(body);
                self.indent -= 1;
                self.emitln("}");
            }
            Stmt::DoWhile { body, condition } => {
                self.emitln("do {");
                self.indent += 1;
                self.transpile_stmt(body);
                self.indent -= 1;
                self.emit("} while (");
                self.transpile_expr(condition);
                self.emitln(");");
            }
            Stmt::Foreach { iterable, key_var, value_var, body } => {
                if let Expr::Variable(arr_name) = iterable {
                    let arr_c_name = self.vars.get(arr_name.as_str()).cloned();
                    let len_var = self.var_array_lens.get(arr_name.as_str()).cloned();
                    let dict_keys = self.var_dict_keys.get(arr_name.as_str()).cloned();
                    let dict_vals = self.var_dict_vals.get(arr_name.as_str()).cloned();

                    // Dict iteration with parallel key/value arrays
                    if let (Some(keys_c), Some(vals_c), Some(ref len)) = (dict_keys, dict_vals, &len_var) {
                        let idx_var = format!("_fi_{}", self.var_counter);
                        self.var_counter += 1;
                        let val_c_name = format!("v_{}", self.var_counter);
                        self.var_counter += 1;
                        let val_type = self.var_types.get(arr_name.as_str())
                            .cloned().unwrap_or_else(|| "int64_t*".to_string());
                        let elem_type = val_type.strip_suffix('*').unwrap_or("int64_t");
                        self.vars.insert(value_var.clone(), val_c_name.clone());
                        self.var_types.insert(value_var.clone(), elem_type.to_string());
                        if let Some(kv) = &key_var {
                            let key_c_name = format!("v_{}", self.var_counter);
                            self.var_counter += 1;
                            self.vars.insert(kv.clone(), key_c_name.clone());
                            self.var_types.insert(kv.clone(), "const char*".to_string());
                        }
                        self.emitln(&format!("for (int {} = 0; {} < {}; {}++) {{", idx_var, idx_var, len, idx_var));
                        self.indent += 1;
                        if let Some(kv) = &key_var {
                            let key_c = self.vars.get(kv.as_str()).unwrap();
                            self.emitln(&format!("const char* {} = {}[{}];", key_c, keys_c, idx_var));
                        }
                        self.emitln(&format!("{} {} = {}[{}];", elem_type, val_c_name, vals_c, idx_var));
                        self.transpile_stmt(body);
                        self.indent -= 1;
                        self.emitln("}");
                        return;
                    }

                    // Array iteration
                    if let (Some(ref arr_c), Some(ref len)) = (&arr_c_name, &len_var) {
                        let idx_var = format!("_fi_{}", self.var_counter);
                        self.var_counter += 1;
                        let val_c_name = format!("v_{}", self.var_counter);
                        self.var_counter += 1;
                        let arr_type = self.var_types.get(arr_name.as_str())
                            .cloned().unwrap_or_else(|| "int64_t*".to_string());
                        let elem_type = arr_type.strip_suffix('*').unwrap_or("int64_t");
                        self.vars.insert(value_var.clone(), val_c_name.clone());
                        self.var_types.insert(value_var.clone(), elem_type.to_string());
                        if let Some(kv) = &key_var {
                            self.vars.insert(kv.clone(), idx_var.clone());
                            self.var_types.insert(kv.clone(), "int".to_string());
                        }
                        self.emitln(&format!("for (int {} = 0; {} < {}; {}++) {{", idx_var, idx_var, len, idx_var));
                        self.indent += 1;
                        self.emitln(&format!("{} {} = {}[{}];", elem_type, val_c_name, arr_c, idx_var));
                        self.transpile_stmt(body);
                        self.indent -= 1;
                        self.emitln("}");
                        return;
                    }
                }
                // Fallback: emit comment and body
                self.emit("/* foreach: unsupported iterable */ ");
                self.transpile_stmt(body);
                let _ = (key_var, value_var);
            }
            Stmt::Match { expr, arms } => {
                // Emit as if/else chain
                self.emit("// match ");
                self.transpile_expr(expr);
                self.emitln("");

                if let Some(first_arm) = arms.first() {
                    // For Variable/Wildcard patterns, we handle specially
                    self.emit("if (1) ");
                    self.emitln("{ // matched arm");
                    self.indent += 1;
                    self.write_indent();
                    self.transpile_expr(&first_arm.body);
                    self.emitln(";");
                    self.indent -= 1;
                    self.emitln("}");
                }
            }
            _ => {
                self.emitln("/* unsupported statement */");
            }
        }
    }

    fn transpile_for_init(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Let { name, value, .. } => {
                let c_name = format!("v_{}", self.var_counter);
                self.var_counter += 1;
                self.vars.insert(name.clone(), c_name.clone());
                let c_type = if let Some(expr) = value {
                    self.infer_expr_type(expr)
                } else {
                    "int64_t".to_string()
                };
                self.var_types.insert(name.clone(), c_type.clone());
                self.write(&format!("{} {} = ", c_type, c_name));
                if let Some(expr) = value {
                    self.transpile_expr(expr);
                } else {
                    self.write("0");
                }
            }
            _ => self.transpile_expr_stmt(stmt),
        }
    }

    fn transpile_expr_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::ExprStmt(expr) => { self.transpile_expr(expr); }
            _ => {}
        }
    }

    fn transpile_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Literal(lit) => match lit {
                Literal::Int(n) => self.write(&format!("{}LL", n)),
                Literal::Float(n) => self.write(&format!("{}", n)),
                Literal::String_(s) => {
                    let idx = self.add_string(s);
                    self.write(&format!("__str_{}", idx));
                }
                Literal::Bool(b) => self.write(if *b { "1" } else { "0" }),
                Literal::Null => self.write("0"),
            },
            Expr::Variable(name) => {
                let c_name = self.vars.get(name).cloned();
                if let Some(ref c) = c_name {
                    self.write(c);
                } else if self.funcs.contains(name) {
                    self.write(name);
                } else {
                    self.write(&format!("/*undef:{}*/0", name));
                }
            }
            Expr::Binary { left, op, right } => {
                match op {
                    BinaryOp::Concat => {
                        self.transpile_expr_to_string(expr);
                        return;
                    }
                    _ => {
                        let lt = self.infer_expr_type(left);
                        let rt = self.infer_expr_type(right);
                        let is_str_cmp = (lt == "const char*" || rt == "const char*")
                            && matches!(op, BinaryOp::Eq | BinaryOp::Neq | BinaryOp::StrictEq | BinaryOp::StrictNeq);
                        if is_str_cmp {
                            self.write("(strcmp(");
                            self.transpile_expr(left);
                            self.write(", ");
                            self.transpile_expr(right);
                            if matches!(op, BinaryOp::Eq | BinaryOp::StrictEq) {
                                self.write(") == 0)");
                            } else {
                                self.write(") != 0)");
                            }
                        } else {
                            self.write("(");
                            self.transpile_expr(left);
                            self.write(&format!(" {} ", bin_op_c_str(op)));
                            self.transpile_expr(right);
                            self.write(")");
                        }
                    }
                }
            }
            Expr::Unary { op, right } => {
                let c_op = match op {
                    UnaryOp::Neg => "-",
                    UnaryOp::Not => "!",
                };
                self.write(c_op);
                self.write("(");
                self.transpile_expr(right);
                self.write(")");
            }
            Expr::Call { callee, args } => {
                // Handle built-in count($array) -> emit array length variable
                if let Expr::Variable(name) = callee.as_ref() {
                    if name == "count" && args.len() == 1 {
                        if let Expr::Variable(arr_name) = &args[0] {
                            let len_var = self.var_array_lens.get(arr_name.as_str()).cloned();
                            if let Some(lv) = len_var {
                                self.write(&lv);
                                return;
                            }
                        }
                    }
                    // var_dump($x) -> var_dump("C_type", value_as_string)
                    if name == "var_dump" && args.len() == 1 {
                        let c_type = self.infer_expr_type(&args[0]);
                        let is_array_var = if let Expr::Variable(vname) = &args[0] {
                            self.var_array_lens.contains_key(vname.as_str())
                        } else {
                            false
                        };
                        let type_name = match c_type.as_str() {
                            "int64_t" => {
                                match &args[0] {
                                    Expr::Literal(Literal::Null) => "NULL",
                                    _ if is_array_var => "array",
                                    _ => "int",
                                }
                            }
                            "double" => "float",
                            "const char*" => "string",
                            "bool" => "bool",
                            _ if is_array_var || (c_type.ends_with('*') && c_type != "const char*") => "array",
                            _ => "unknown",
                        };
                        self.write("var_dump(\"");
                        self.write(type_name);
                        self.write("\", ");
                        if is_array_var {
                            self.write("\"[...]\"");
                        } else {
                            self.transpile_expr_to_string(&args[0]);
                        }
                        self.write(")");
                        return;
                    }
                    // print_r($x) -> print_r(value_as_string)
                    if name == "print_r" && args.len() == 1 {
                        let is_array_var = if let Expr::Variable(vname) = &args[0] {
                            self.var_array_lens.contains_key(vname.as_str())
                        } else {
                            false
                        };
                        if is_array_var {
                            self.write("print_r(\"[Array]\")");
                        } else {
                            self.write("print_r(");
                            self.transpile_expr_to_string(&args[0]);
                            self.write(")");
                        }
                        return;
                    }
                    // sprintf(fmt, ...) -> phprs_sprintf(fmt, a1, a2, a3, a4) with padding
                    if name == "sprintf" {
                        self.write("phprs_sprintf(");
                        for (i, arg) in args.iter().enumerate() {
                            if i > 0 { self.write(", "); }
                            self.transpile_expr_to_string(arg);
                        }
                        // Pad to 5 arguments (fmt + 4 string args)
                        for i in args.len()..5 {
                            if i > 0 { self.write(", "); }
                            self.write("\"\"");
                        }
                        self.write(")");
                        return;
                    }
                    // empty($var) — check emptiness, skip __itos on result (returns bool)
                    if name == "empty" && args.len() == 1 {
                        match &args[0] {
                            Expr::Literal(Literal::Null) => { self.write("1"); return; }
                            Expr::Literal(Literal::Bool(b)) => { self.write(if *b { "0" } else { "1" }); return; }
                            Expr::Variable(name) => {
                                let var_ty = self.var_types.get(name).map(|s| s.as_str()).unwrap_or("");
                                if var_ty == "const char*" || var_ty == "string" {
                                    self.write("empty_(");
                                    self.write(&self.vars.get(name).cloned().unwrap_or_default());
                                    self.write(")");
                                } else if var_ty == "bool" {
                                    let c_var = self.vars.get(name).cloned().unwrap_or_default();
                                    self.write(&format!("(!{})", c_var));
                                } else {
                                    let c_var = self.vars.get(name).cloned().unwrap_or_default();
                                    self.write(&format!("({0} == 0)", c_var));
                                }
                                return;
                            }
                            _ => {
                                self.write("empty_(");
                                self.transpile_expr_to_string(&args[0]);
                                self.write(")");
                                return;
                            }
                        }
                    }
                    // json_encode with dict/array literal
                    if name == "json_encode" && args.len() == 1 {
                        match &args[0] {
                            Expr::Dict(pairs) => {
                                self.emit_json_encode_dict(pairs);
                                return;
                            }
                            Expr::Array(elements) => {
                                self.emit_json_encode_array(elements);
                                return;
                            }
                            _ => {}
                        }
                    }
                }
                if let Expr::Variable(name) = callee.as_ref() {
                    // Name remapping for C compatibility
                    match name.as_str() {
                        "sprintf" => self.write("phprs_sprintf"),
                        "sleep" => self.write("sleep_"),
                        "usleep" => self.write("usleep_"),
                        "realpath" => self.write("realpath_"),
                        "getcwd" => self.write("getcwd_"),
                        "time" => self.write("time_"),
                        "abs" => self.write("abs_"),
                        "ceil" => self.write("ceil_"),
                        "floor" => self.write("floor_"),
                        "round" => self.write("round_"),
                        "rand" => self.write("rand_"),
                        "mt_rand" => self.write("mt_rand_"),
                        "pow" => self.write("pow_"),
                        "sqrt" => self.write("sqrt_"),
                        "max" => self.write("max_i"),
                        "min" => self.write("min_i"),
                        "empty" => self.write("empty_"),
                        "unset" => self.write("unset_"),
                        "mkdir" => self.write("mkdir_"),
                        "unlink" => self.write("unlink_"),
                        "basename" => self.write("basename_"),
                        "dirname" => self.write("dirname_"),
                        "scandir" => self.write("scandir_"),
                        "rename" => self.write("rename_"),
                        "fmod" => self.write("fmod_"),
                        "mktime" => self.write("mktime_"),
                        _ => self.write(name),
                    }
                } else {
                    self.write("/*unknown_func*/");
                }
                self.write("(");
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 { self.write(", "); }
                    self.transpile_expr(arg);
                }
                self.write(")");
            }
            Expr::Assign { target, op, value } => {
                if let Some(bin_op) = op {
                    if *bin_op == BinaryOp::Concat {
                        // . = -> target = __concat(target, value)
                        self.transpile_expr(target);
                        self.write(" = __concat(");
                        self.transpile_expr(target);
                        self.write(", ");
                        self.transpile_expr(value);
                        self.write(")");
                    } else {
                        self.transpile_expr(target);
                        let op_str = match bin_op {
                            BinaryOp::Add => "+",
                            BinaryOp::Sub => "-",
                            BinaryOp::Mul => "*",
                            BinaryOp::Div => "/",
                            BinaryOp::Mod => "%",
                            _ => "?",
                        };
                        self.write(&format!(" {}=", op_str));
                        self.write(" ");
                        self.transpile_expr(value);
                    }
                } else {
                    self.transpile_expr(target);
                    self.write(" = ");
                    self.transpile_expr(value);
                }
            }
            Expr::IncDec { target, is_inc, is_prefix } => {
                if *is_prefix {
                    self.write(if *is_inc { "++" } else { "--" });
                    self.transpile_expr(target);
                } else {
                    self.transpile_expr(target);
                    self.write(if *is_inc { "++" } else { "--" });
                }
            }
            Expr::Index { target, index } => {
                self.transpile_expr(target);
                self.write("[");
                self.transpile_expr(index);
                self.write("]");
            }
            Expr::MatchExpr { expr, arms } => {
                // Emit as ternary chain: cond1 ? val1 : cond2 ? val2 : ... : 0
                for (i, arm) in arms.iter().enumerate() {
                    match &arm.pattern {
                        MatchPattern::Range { start, end, inclusive } => {
                            self.write("(");
                            if let Some(s) = start {
                                self.transpile_expr(expr);
                                self.write(" >= ");
                                self.transpile_expr(s);
                            }
                            if start.is_some() && end.is_some() {
                                self.write(" && ");
                            }
                            if let Some(e) = end {
                                self.transpile_expr(expr);
                                if *inclusive {
                                    self.write(" <= ");
                                } else {
                                    self.write(" < ");
                                }
                                self.transpile_expr(e);
                            }
                            self.write(")");
                        }
                        MatchPattern::Wildcard => {
                            self.write("1");
                        }
                        MatchPattern::Literal(lit) => {
                            if let Literal::String_(_) = lit {
                                self.write("strcmp(");
                                self.transpile_expr(expr);
                                self.write(", ");
                                self.transpile_expr(&Expr::Literal(lit.clone()));
                                self.write(") == 0");
                            } else {
                                self.transpile_expr(expr);
                                self.write(" == ");
                                self.transpile_expr(&Expr::Literal(lit.clone()));
                            }
                        }
                        MatchPattern::Variable(_) => {
                            self.write("1");
                        }
                    }
                    if i < arms.len() - 1 {
                        self.write(" ? ");
                        self.transpile_expr(&arm.body);
                        self.write(" : ");
                    } else {
                        self.write(" ? ");
                        self.transpile_expr(&arm.body);
                        self.write(" : 0");
                    }
                }
            }
            _ => { self.write("0 /* TODO */"); }
        }
    }

    /// Transpile an expression to a C string expression (const char*)
    fn transpile_expr_to_string(&mut self, expr: &Expr) {
        match expr {
            Expr::Literal(Literal::String_(s)) => {
                let idx = self.add_string(s);
                self.write(&format!("__str_{}", idx));
            }
            Expr::Literal(Literal::Int(n)) => {
                let idx = self.add_string(&n.to_string());
                self.write(&format!("__str_{}", idx));
            }
            Expr::Literal(Literal::Float(n)) => {
                let idx = self.add_string(&n.to_string());
                self.write(&format!("__str_{}", idx));
            }
            Expr::Literal(Literal::Bool(b)) => {
                let idx = self.add_string(if *b { "true" } else { "false" });
                self.write(&format!("__str_{}", idx));
            }
            Expr::Literal(Literal::Null) => {
                let idx = self.add_string("null");
                self.write(&format!("__str_{}", idx));
            }
            Expr::Variable(name) => {
                let c_name = self.vars.get(name).cloned();
                if let Some(ref c) = c_name {
                    let var_ty = self.var_types.get(name).map(|s| s.as_str()).unwrap_or("");
                    if var_ty == "const char*" {
                        self.write(c);
                    } else {
                        self.write(&format!("__itos({})", c));
                    }
                } else {
                    self.write("\"\"");
                }
            }
            Expr::Binary { op: BinaryOp::Concat, left, right } => {
                self.write("__concat(");
                self.transpile_expr_to_string(left);
                self.write(", ");
                self.transpile_expr_to_string(right);
                self.write(")");
            }
            Expr::Call { callee, args } => {
                if let Expr::Variable(name) = callee.as_ref() {
                    // empty($var) — check emptiness, wrap bool result in __itos for string context
                    if name == "empty" && args.len() == 1 {
                        match &args[0] {
                            Expr::Literal(Literal::Null) => { self.write("__itos(1)"); return; }
                            Expr::Literal(Literal::Bool(b)) => { self.write(if *b { "__itos(0)" } else { "__itos(1)" }); return; }
                            Expr::Variable(name) => {
                                let var_ty = self.var_types.get(name).map(|s| s.as_str()).unwrap_or("");
                                if var_ty == "const char*" || var_ty == "string" {
                                    self.write("__itos(empty_(");
                                    self.write(&self.vars.get(name).cloned().unwrap_or_default());
                                    self.write("))");
                                } else if var_ty == "bool" {
                                    let c_var = self.vars.get(name).cloned().unwrap_or_default();
                                    self.write(&format!("__itos((!{}))", c_var));
                                } else {
                                    let c_var = self.vars.get(name).cloned().unwrap_or_default();
                                    self.write(&format!("__itos(({0} == 0))", c_var));
                                }
                                return;
                            }
                            _ => {
                                self.write("__itos(empty_(");
                                self.transpile_expr_to_string(&args[0]);
                                self.write("))");
                                return;
                            }
                        }
                    }
                    // json_encode with dict/array literal — generate inline JSON
                    if name == "json_encode" && args.len() == 1 {
                        match &args[0] {
                            Expr::Dict(pairs) => {
                                self.emit_json_encode_dict(pairs);
                                return;
                            }
                            Expr::Array(elements) => {
                                self.emit_json_encode_array(elements);
                                return;
                            }
                            _ => {}
                        }
                    }
                    let returns_string = self.func_return_types.get(name)
                        .map(|t| t == "const char*")
                        .unwrap_or(false);
                    let is_strlen_or_count = name == "strlen" || name == "count";
                    let needs_itos = is_strlen_or_count || !returns_string;
                    if needs_itos {
                        self.write("__itos(");
                    }
                    let is_sprintf = name == "sprintf";
                    match name.as_str() {
                        "sprintf" => self.write("phprs_sprintf"),
                        "sleep" => self.write("sleep_"),
                        "usleep" => self.write("usleep_"),
                        "realpath" => self.write("realpath_"),
                        "getcwd" => self.write("getcwd_"),
                        "time" => self.write("time_"),
                        "abs" => self.write("abs_"),
                        "ceil" => self.write("ceil_"),
                        "floor" => self.write("floor_"),
                        "round" => self.write("round_"),
                        "rand" => self.write("rand_"),
                        "mt_rand" => self.write("mt_rand_"),
                        "pow" => self.write("pow_"),
                        "sqrt" => self.write("sqrt_"),
                        "max" => self.write("max_i"),
                        "min" => self.write("min_i"),
                        "empty" => self.write("empty_"),
                        "unset" => self.write("unset_"),
                        "mkdir" => self.write("mkdir_"),
                        "unlink" => self.write("unlink_"),
                        "basename" => self.write("basename_"),
                        "dirname" => self.write("dirname_"),
                        "scandir" => self.write("scandir_"),
                        "rename" => self.write("rename_"),
                        "fmod" => self.write("fmod_"),
                        "mktime" => self.write("mktime_"),
                        _ => self.write(name),
                    }
                    self.write("(");
                    for (i, arg) in args.iter().enumerate() {
                        if i > 0 { self.write(", "); }
                        self.transpile_expr(arg);
                    }
                    // Pad sprintf to 5 arguments
                    if is_sprintf {
                        for i in args.len()..5 {
                            if i > 0 || !args.is_empty() { self.write(", "); }
                            self.write("\"\"");
                        }
                    }
                    self.write(")");
                    if needs_itos {
                        self.write(")");
                    }
                } else {
                    self.write("\"\"");
                }
            }
            Expr::MatchExpr { arms, .. } => {
                if let Some(arm) = arms.first() {
                    self.transpile_expr_to_string(&arm.body);
                } else {
                    self.write("\"\"");
                }
            }
            Expr::IncDec { target, is_inc, is_prefix } => {
                self.write("__itos(");
                if *is_prefix {
                    self.write(if *is_inc { "++" } else { "--" });
                    self.transpile_expr(target);
                } else {
                    self.transpile_expr(target);
                    self.write(if *is_inc { "++" } else { "--" });
                }
                self.write(")");
            }
            _ => {
                self.write("\"\" /* non-string expr */");
            }
        }
    }

    fn add_string(&mut self, s: &str) -> usize {
        if let Some(pos) = self.strings.iter().position(|x| x == s) {
            return pos;
        }
        self.strings.push(s.to_string());
        self.strings.len() - 1
    }

    fn emit(&mut self, s: &str) {
        if self.output.ends_with('\n') {
            self.write_indent();
        }
        self.write(s);
    }

    fn emitln(&mut self, s: &str) {
        if s.is_empty() {
            self.output.push('\n');
            return;
        }
        if !self.output.ends_with('\n') {
            // Check if we need to indent before this line
        }
        self.write_indent();
        self.write(s);
        self.output.push('\n');
    }

    fn write_indent(&mut self) {
        for _ in 0..self.indent {
            self.output.push_str("    ");
        }
    }

    fn write(&mut self, s: &str) {
        self.output.push_str(s);
    }

    fn emit_json_encode_dict(&mut self, pairs: &[(Expr, Expr)]) {
        if pairs.is_empty() {
            let idx = self.add_string("{}");
            self.write(&format!("__str_{}", idx));
            return;
        }

        let n = pairs.len();
        let total_concats = 2 * n + 1;

        for _ in 0..total_concats {
            self.write("__concat(");
        }

        self.write("\"{\"");

        for i in 0..n {
            let (ref key_expr, ref val_expr) = &pairs[i];

            let key_str = match key_expr {
                Expr::Literal(Literal::String_(s)) => s.clone(),
                Expr::Literal(Literal::Int(n)) => n.to_string(),
                _ => "".to_string(),
            };
            let c_key = escape_c_str(&key_str);

            let key_content = if i == 0 {
                format!("\\\"{}\\\":", c_key)
            } else {
                format!(",\\\"{}\\\":", c_key)
            };
            self.write(&format!(", \"{}\"", key_content));
            self.write(")");

            self.write(", json_encode(");
            self.transpile_expr_to_string(val_expr);
            self.write("))");
        }

        self.write(", \"}\")");
    }

    fn emit_json_encode_array(&mut self, elements: &[Expr]) {
        if elements.is_empty() {
            let idx = self.add_string("[]");
            self.write(&format!("__str_{}", idx));
            return;
        }

        let n = elements.len();
        let total_concats = 2 * n;

        for _ in 0..total_concats {
            self.write("__concat(");
        }

        self.write("\"[\"");

        for i in 0..n {
            self.write(", json_encode(");
            self.transpile_expr_to_string(&elements[i]);
            self.write("))");

            if i < n - 1 {
                self.write(", \",\")");
            }
        }

        self.write(", \"]\")");
    }
}

fn bin_op_c_str(op: &BinaryOp) -> &'static str {
    match op {
        BinaryOp::Add => "+",
        BinaryOp::Sub => "-",
        BinaryOp::Mul => "*",
        BinaryOp::Div => "/",
        BinaryOp::Mod => "%",
        BinaryOp::Eq => "==",
        BinaryOp::Neq => "!=",
        BinaryOp::StrictEq => "==",
        BinaryOp::StrictNeq => "!=",
        BinaryOp::Lt => "<",
        BinaryOp::Gt => ">",
        BinaryOp::Le => "<=",
        BinaryOp::Ge => ">=",
        BinaryOp::And => "&&",
        BinaryOp::Or => "||",
        BinaryOp::Concat => "+", // not used for binary op
    }
}

fn ast_ty_to_c(ty: &TypeAnnotation) -> &'static str {
    match ty {
        TypeAnnotation::Int => "int64_t",
        TypeAnnotation::Float => "double",
        TypeAnnotation::String_ => "const char*",
        TypeAnnotation::Bool => "bool",
        TypeAnnotation::Void => "void",
        TypeAnnotation::Array(inner) => {
            match inner.as_ref() {
                TypeAnnotation::String_ => "const char**",
                TypeAnnotation::Int => "int64_t*",
                TypeAnnotation::Float => "double*",
                _ => "int64_t*",
            }
        }
        _ => "int64_t",
    }
}

fn php_var_to_c(name: &str) -> String {
    format!("_p_{}", name)
}

/// Generate complete compilable C code including runtime helpers
pub fn transpile_program(program: &Program) -> String {
    let mut t = CTranspiler::new();
    let body = t.transpile(program);

    let mut out = String::new();
    out.push_str("// Generated by PHPRS Compiler v0.2.0\n");
    out.push_str("#include <stdio.h>\n");
    out.push_str("#include <stdlib.h>\n");
    out.push_str("#include <string.h>\n");
    out.push_str("#include <stdint.h>\n");
    out.push_str("#include <stdbool.h>\n\n");

    // String constants pool
    for (i, s) in t.strings.iter().enumerate() {
        let escaped = escape_c_str(s);
        out.push_str(&format!("static const char* __str_{} = \"{}\";\n", i, escaped));
    }
    out.push('\n');

    // Runtime: string concatenation
    out.push_str("static char* __concat(const char* a, const char* b) {\n");
    out.push_str("    if (!a) a = \"\"; if (!b) b = \"\";\n");
    out.push_str("    size_t la = strlen(a), lb = strlen(b);\n");
    out.push_str("    char* r = malloc(la + lb + 1);\n");
    out.push_str("    memcpy(r, a, la); memcpy(r + la, b, lb);\n");
    out.push_str("    r[la + lb] = 0;\n");
    out.push_str("    return r;\n");
    out.push_str("}\n\n");

    // Runtime: int to string
    out.push_str("static char* __itos(int64_t n) {\n");
    out.push_str("    char* r = malloc(32);\n");
    out.push_str("    snprintf(r, 32, \"%lld\", (long long)n);\n");
    out.push_str("    return r;\n");
    out.push_str("}\n\n");

    // PHP Runtime Library (sockets, file I/O, HTTP, JSON, string helpers)
    out.push_str(include_str!("phprs_runtime.c"));
    out.push('\n');

    out.push_str(&body);

    out
}

fn escape_c_str(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        match c {
            '"' => result.push_str("\\\""),
            '\\' => result.push_str("\\\\"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            _ if c.is_ascii_graphic() || c == ' ' => result.push(c),
            _ => result.push_str(&format!("\\u{:04x}", c as u32)),
        }
    }
    result
}
