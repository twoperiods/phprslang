use std::collections::HashMap;
use crate::parser::*;
use crate::typeck::ty::Ty;

#[derive(Debug, Clone)]
pub struct TypeEnv {
    vars: HashMap<String, Ty>,
    funcs: HashMap<String, (Vec<Ty>, Ty)>,
}

impl TypeEnv {
    pub fn new() -> Self {
        let mut env = Self {
            vars: HashMap::new(),
            funcs: HashMap::new(),
        };
        // Register builtins
        env.funcs.insert("strlen".into(), (vec![Ty::String], Ty::Int));
        env.funcs.insert("count".into(), (vec![Ty::Array(Box::new(Ty::Unknown))], Ty::Int));
        env.funcs.insert("trim".into(), (vec![Ty::String], Ty::String));
        env.funcs.insert("str_contains".into(), (vec![Ty::String, Ty::String], Ty::Bool));
        env.funcs.insert("chr".into(), (vec![Ty::Int], Ty::String));
        env.funcs.insert("ord".into(), (vec![Ty::String], Ty::Int));
        env.funcs.insert("addslashes".into(), (vec![Ty::String], Ty::String));
        env.funcs.insert("stripslashes".into(), (vec![Ty::String], Ty::String));
        env.funcs.insert("copy".into(), (vec![Ty::String, Ty::String], Ty::Bool));
        env.funcs.insert("rename".into(), (vec![Ty::String, Ty::String], Ty::Bool));
        env.funcs.insert("filesize".into(), (vec![Ty::String], Ty::Int));
        env.funcs.insert("filemtime".into(), (vec![Ty::String], Ty::Int));
        env.funcs.insert("pathinfo".into(), (vec![Ty::String], Ty::String));
        env.funcs.insert("move_uploaded_file".into(), (vec![Ty::String, Ty::String], Ty::Bool));
        env.funcs.insert("password_hash".into(), (vec![Ty::String, Ty::String], Ty::String));
        env.funcs.insert("password_verify".into(), (vec![Ty::String, Ty::String], Ty::Bool));
        env.funcs.insert("random_bytes".into(), (vec![Ty::Int], Ty::String));
        env.funcs.insert("random_int".into(), (vec![Ty::Int, Ty::Int], Ty::Int));
        env.funcs.insert("array_chunk".into(), (vec![Ty::Array(Box::new(Ty::Unknown)), Ty::Int, Ty::Bool], Ty::Array(Box::new(Ty::Unknown))));
        env.funcs.insert("array_count_values".into(), (vec![Ty::Array(Box::new(Ty::Unknown))], Ty::Dict(Box::new(Ty::String), Box::new(Ty::Int))));
        env.funcs.insert("array_product".into(), (vec![Ty::Array(Box::new(Ty::Unknown))], Ty::Float));
        env.funcs.insert("array_intersect".into(), (vec![Ty::Array(Box::new(Ty::Unknown)), Ty::Array(Box::new(Ty::Unknown))], Ty::Array(Box::new(Ty::Unknown))));
        // Batch 2: Type casting
        env.funcs.insert("intval".into(), (vec![Ty::Unknown], Ty::Int));
        env.funcs.insert("floatval".into(), (vec![Ty::Unknown], Ty::Float));
        env.funcs.insert("strval".into(), (vec![Ty::Unknown], Ty::String));
        env.funcs.insert("boolval".into(), (vec![Ty::Unknown], Ty::Bool));
        // Batch 2: String functions
        env.funcs.insert("str_pad".into(), (vec![Ty::String, Ty::Int], Ty::String));
        env.funcs.insert("wordwrap".into(), (vec![Ty::String], Ty::String));
        env.funcs.insert("str_word_count".into(), (vec![Ty::String], Ty::Int));
        env.funcs.insert("chunk_split".into(), (vec![Ty::String], Ty::String));
        // Batch 2: Array functions
        env.funcs.insert("array_splice".into(), (vec![Ty::Array(Box::new(Ty::Unknown)), Ty::Int, Ty::Int], Ty::Array(Box::new(Ty::Unknown))));
        env.funcs.insert("array_pad".into(), (vec![Ty::Array(Box::new(Ty::Unknown)), Ty::Int, Ty::Unknown], Ty::Array(Box::new(Ty::Unknown))));
        env.funcs.insert("array_key_first".into(), (vec![Ty::Array(Box::new(Ty::Unknown))], Ty::Unknown));
        env.funcs.insert("array_key_last".into(), (vec![Ty::Array(Box::new(Ty::Unknown))], Ty::Unknown));
        env.funcs.insert("array_is_list".into(), (vec![Ty::Array(Box::new(Ty::Unknown))], Ty::Bool));
        // Batch 2: Math/Date
        env.funcs.insert("fmod".into(), (vec![Ty::Float, Ty::Float], Ty::Float));
        env.funcs.insert("intdiv".into(), (vec![Ty::Int, Ty::Int], Ty::Int));
        env.funcs.insert("checkdate".into(), (vec![Ty::Int, Ty::Int, Ty::Int], Ty::Bool));
        env.funcs.insert("mktime".into(), (vec![Ty::Int, Ty::Int, Ty::Int, Ty::Int, Ty::Int, Ty::Int], Ty::Int));
        // Batch 2: Misc
        env.funcs.insert("printf".into(), (vec![Ty::String], Ty::Void));
        env.funcs.insert("str_starts_with".into(), (vec![Ty::String, Ty::String], Ty::Bool));
        env.funcs.insert("str_ends_with".into(), (vec![Ty::String, Ty::String], Ty::Bool));
        // Thread pool
        env.funcs.insert("phprs_thread_pool_init".into(), (vec![Ty::Int], Ty::Int));
        env.funcs.insert("phprs_thread_pool_enqueue".into(), (vec![Ty::String, Ty::Int, Ty::String], Ty::Int));
        env.funcs.insert("phprs_thread_pool_shutdown".into(), (vec![], Ty::Void));
        // App state
        env.funcs.insert("phprs_app_set_routes".into(), (vec![Ty::String], Ty::Void));
        env.funcs.insert("phprs_app_get_routes".into(), (vec![], Ty::String));
        env.funcs.insert("phprs_app_set_port".into(), (vec![Ty::Int], Ty::Void));
        env.funcs.insert("phprs_app_get_port".into(), (vec![], Ty::Int));
        // String validation
        env.funcs.insert("phprs_str_is_alnum".into(), (vec![Ty::String], Ty::Int));
        // Production infrastructure
        env.funcs.insert("phprs_config".into(), (vec![Ty::String], Ty::Void));
        env.funcs.insert("phprs_config_max_body".into(), (vec![Ty::Int], Ty::Void));
        env.funcs.insert("phprs_config_timeout".into(), (vec![Ty::Int, Ty::Int], Ty::Void));
        env.funcs.insert("phprs_config_max_connections".into(), (vec![Ty::Int], Ty::Void));
        env.funcs.insert("phprs_is_shutting_down".into(), (vec![], Ty::Int));
        env.funcs.insert("phprs_log".into(), (vec![Ty::String], Ty::Void));
        env.funcs.insert("phprs_log_error_msg".into(), (vec![Ty::String], Ty::Void));
        env.funcs.insert("phprs_log_init".into(), (vec![Ty::String], Ty::Void));
        env.funcs.insert("phprs_server_init_signals".into(), (vec![], Ty::Void));
        env.funcs.insert("phprs_write_pidfile".into(), (vec![Ty::String], Ty::Void));
        // Redis client
        env.funcs.insert("phprs_redis_init".into(), (vec![Ty::String, Ty::Int, Ty::String], Ty::Void));
        env.funcs.insert("phprs_redis_close".into(), (vec![], Ty::Void));
        env.funcs.insert("phprs_redis_cmd".into(), (vec![Ty::String], Ty::String));
        env.funcs.insert("phprs_redis_get".into(), (vec![Ty::String], Ty::String));
        env.funcs.insert("phprs_redis_set".into(), (vec![Ty::String, Ty::String], Ty::String));
        env.funcs.insert("phprs_redis_setex".into(), (vec![Ty::String, Ty::Int, Ty::String], Ty::String));
        env.funcs.insert("phprs_redis_del".into(), (vec![Ty::String], Ty::String));
        env.funcs.insert("phprs_redis_exists".into(), (vec![Ty::String], Ty::Int));
        env.funcs.insert("phprs_redis_keys".into(), (vec![Ty::String], Ty::String));
        env.funcs.insert("phprs_redis_expire".into(), (vec![Ty::String, Ty::Int], Ty::Int));
        env.funcs.insert("phprs_redis_incr".into(), (vec![Ty::String], Ty::Int));
        env.funcs.insert("phprs_redis_decr".into(), (vec![Ty::String], Ty::Int));
        env.funcs.insert("phprs_redis_hget".into(), (vec![Ty::String, Ty::String], Ty::String));
        env.funcs.insert("phprs_redis_hset".into(), (vec![Ty::String, Ty::String, Ty::String], Ty::String));
        env.funcs.insert("phprs_redis_hgetall".into(), (vec![Ty::String], Ty::String));
        env.funcs.insert("phprs_redis_lpush".into(), (vec![Ty::String, Ty::String], Ty::String));
        env.funcs.insert("phprs_redis_rpush".into(), (vec![Ty::String, Ty::String], Ty::String));
        env.funcs.insert("phprs_redis_lrange".into(), (vec![Ty::String, Ty::Int, Ty::Int], Ty::String));
        env.funcs.insert("phprs_redis_ping".into(), (vec![], Ty::String));
        env.funcs.insert("phprs_redis_ttl".into(), (vec![Ty::String], Ty::Int));
        env.funcs.insert("phprs_redis_select".into(), (vec![Ty::Int], Ty::String));
        // MySQL client
        env.funcs.insert("phprs_mysql_init".into(), (vec![Ty::String, Ty::Int, Ty::String, Ty::String, Ty::String], Ty::Void));
        env.funcs.insert("phprs_mysql_close".into(), (vec![], Ty::Void));
        env.funcs.insert("phprs_mysql_escape".into(), (vec![Ty::String], Ty::String));
        env.funcs.insert("phprs_mysql_query".into(), (vec![Ty::String], Ty::String));
        env.funcs.insert("phprs_mysql_exec".into(), (vec![Ty::String], Ty::String));
        env.funcs.insert("phprs_mysql_select".into(), (vec![Ty::String, Ty::String], Ty::String));
        env.funcs.insert("phprs_mysql_insert".into(), (vec![Ty::String, Ty::String], Ty::String));
        env.funcs.insert("phprs_mysql_update".into(), (vec![Ty::String, Ty::String, Ty::String], Ty::String));
        env.funcs.insert("phprs_mysql_delete".into(), (vec![Ty::String, Ty::String], Ty::String));
        // WebSocket connection manager
        env.funcs.insert("phprs_ws_manager_init".into(), (vec![Ty::Int], Ty::Void));
        env.funcs.insert("phprs_ws_register".into(), (vec![Ty::Int, Ty::String], Ty::Int));
        env.funcs.insert("phprs_ws_unregister".into(), (vec![Ty::Int], Ty::Void));
        env.funcs.insert("phprs_ws_update_pong".into(), (vec![Ty::Int], Ty::Void));
        env.funcs.insert("phprs_ws_broadcast".into(), (vec![Ty::String, Ty::String, Ty::Int], Ty::Int));
        env.funcs.insert("phprs_ws_broadcast_all".into(), (vec![Ty::String, Ty::Int], Ty::Int));
        env.funcs.insert("phprs_ws_count".into(), (vec![Ty::String], Ty::Int));
        env.funcs.insert("phprs_ws_rooms".into(), (vec![], Ty::String));
        env.funcs.insert("phprs_ws_start_heartbeat".into(), (vec![Ty::Int], Ty::Void));
        env
    }

    pub fn define(&mut self, name: &str, ty: Ty) {
        self.vars.insert(name.to_string(), ty);
    }

    pub fn get(&self, name: &str) -> Option<&Ty> {
        self.vars.get(name)
    }

    pub fn get_func(&self, name: &str) -> Option<&(Vec<Ty>, Ty)> {
        self.funcs.get(name)
    }
}

pub struct TypeChecker {
    env: TypeEnv,
    errors: Vec<String>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            env: TypeEnv::new(),
            errors: Vec::new(),
        }
    }

    pub fn check(&mut self, program: &Program) -> Result<TypeEnv, Vec<String>> {
        // First pass: collect all function signatures
        for stmt in &program.stmts {
            match stmt {
                Stmt::Function { name, params, return_type, .. }
                | Stmt::ExternFunction { name, params, return_type } => {
                    let param_tys: Vec<Ty> = params.iter().map(|p| {
                        p.ty.as_ref().map(|t| Ty::from_ast_type(t)).unwrap_or(Ty::Unknown)
                    }).collect();
                    let ret_ty = return_type.as_ref().map(|t| Ty::from_ast_type(t)).unwrap_or(Ty::Void);
                    self.env.funcs.insert(name.clone(), (param_tys, ret_ty));
                }
                _ => {}
            }
        }

        // Second pass: check all statements
        for stmt in &program.stmts {
            self.check_stmt(stmt);
        }

        if self.errors.is_empty() {
            Ok(self.env.clone())
        } else {
            Err(self.errors.clone())
        }
    }

    fn error(&mut self, msg: String) {
        self.errors.push(msg);
    }

    pub fn check_and_get_env(&mut self, program: &Program) -> Result<TypeEnv, Vec<String>> {
        self.check(program)
    }

    fn check_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Let { name, ty, value, .. } => {
                let inferred = match value {
                    Some(expr) => self.infer_expr(expr),
                    None => Ty::Null,
                };
                let declared = ty.as_ref().map(|t| Ty::from_ast_type(t));
                let final_ty = match (&declared, &inferred) {
                    (Some(d), Ty::Unknown) => d.clone(),
                    (Some(d), i) => {
                        if d != i && *d != Ty::Unknown && *i != Ty::Null {
                            // Allow int->float promotion
                            if !(*d == Ty::Float && *i == Ty::Int) {
                                self.error(format!("Type mismatch: declared {}, inferred {}", d, i));
                            }
                        }
                        d.clone()
                    }
                    (None, i) => i.clone(),
                };
                self.env.define(name, final_ty);
            }
            Stmt::Function { name, params, return_type, body } => {
                let param_tys: Vec<Ty> = params.iter().map(|p| {
                    p.ty.as_ref().map(|t| Ty::from_ast_type(t)).unwrap_or(Ty::Unknown)
                }).collect();
                let ret_ty = return_type.as_ref().map(|t| Ty::from_ast_type(t)).unwrap_or(Ty::Void);

                // Add params to env for body checking
                for (param, param_ty) in params.iter().zip(param_tys.iter()) {
                    self.env.define(&param.name, param_ty.clone());
                }
                self.check_stmt(body);
                // We don't validate return type strictly in MVP
                let _ = (name, ret_ty);
            }
            Stmt::If { condition, then_branch, else_branch } => {
                self.infer_expr(condition);
                self.check_stmt(then_branch);
                if let Some(b) = else_branch {
                    self.check_stmt(b);
                }
            }
            Stmt::For { init, condition, update, body } => {
                self.check_stmt(init);
                if let Some(c) = condition { self.infer_expr(c); }
                if let Some(u) = update { self.infer_expr(u); }
                self.check_stmt(body);
            }
            Stmt::While { condition, body } => {
                self.infer_expr(condition);
                self.check_stmt(body);
            }
            Stmt::DoWhile { body, condition } => {
                self.check_stmt(body);
                self.infer_expr(condition);
            }
            Stmt::Foreach { iterable, value_var, key_var, body } => {
                let iter_ty = self.infer_expr(iterable);
                match iter_ty {
                    Ty::Array(elem_ty) => {
                        if let Some(k) = key_var { self.env.define(k, Ty::Int); }
                        self.env.define(value_var, (*elem_ty).clone());
                    }
                    Ty::Dict(k_ty, v_ty) => {
                        if let Some(k) = key_var { self.env.define(k, (*k_ty).clone()); }
                        self.env.define(value_var, (*v_ty).clone());
                    }
                    Ty::String => {
                        if let Some(k) = key_var { self.env.define(k, Ty::Int); }
                        self.env.define(value_var, Ty::String);
                    }
                    Ty::Any | Ty::Unknown => {
                        if let Some(k) = key_var { self.env.define(k, Ty::Any); }
                        self.env.define(value_var, Ty::Any);
                    }
                    _ => {}
                }
                self.check_stmt(body);
            }
            Stmt::Return(expr) => {
                if let Some(e) = expr { self.infer_expr(e); }
            }
            Stmt::Break | Stmt::Continue => {}
            Stmt::Throw(expr) => { self.infer_expr(expr); }
            Stmt::TryCatch { try_body, catch_var, catch_body } => {
                self.check_stmt(try_body);
                self.env.define(catch_var, Ty::String);
                self.check_stmt(catch_body);
            }
            Stmt::Echo(expr) => { self.infer_expr(expr); }
            Stmt::ExprStmt(expr) => { self.infer_expr(expr); }
            Stmt::Block(stmts) => {
                for s in stmts { self.check_stmt(s); }
            }
            Stmt::Match { expr, arms } => {
                let _match_ty = self.infer_expr(expr);
                for arm in arms {
                    let _arm_ty = self.infer_expr(&arm.body);
                }
            }
            _ => {} // StructDef, EnumDef ignored for now
        }
    }

    fn infer_expr(&mut self, expr: &Expr) -> Ty {
        match expr {
            Expr::Literal(lit) => match lit {
                Literal::Int(_) => Ty::Int,
                Literal::Float(_) => Ty::Float,
                Literal::String_(_) => Ty::String,
                Literal::Bool(_) => Ty::Bool,
                Literal::Null => Ty::Null,
            },
            Expr::Variable(name) => {
                if let Some(ty) = self.env.get(name) {
                    ty.clone()
                } else if self.env.get_func(name).is_some() {
                    // Function reference - return function type
                    Ty::Unknown
                } else {
                    self.error(format!("Undefined variable: ${}", name));
                    Ty::Unknown
                }
            }
            Expr::Binary { left, op, right } => {
                let lt = self.infer_expr(left);
                let rt = self.infer_expr(right);
                match op {
                    BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => {
                        if lt.is_numeric() && rt.is_numeric() {
                            match (&lt, &rt) {
                                (Ty::Float, _) | (_, Ty::Float) => Ty::Float,
                                _ => Ty::Int,
                            }
                        } else if matches!(op, BinaryOp::Add) && (lt == Ty::String || rt == Ty::String) {
                            Ty::String
                        } else {
                            self.error(format!("Cannot apply {:?} to {} and {}", op, lt, rt));
                            Ty::Unknown
                        }
                    }
                    BinaryOp::Concat => Ty::String,
                    BinaryOp::Eq | BinaryOp::Neq | BinaryOp::StrictEq | BinaryOp::StrictNeq | BinaryOp::Lt | BinaryOp::Gt | BinaryOp::Le | BinaryOp::Ge => {
                        Ty::Bool
                    }
                    BinaryOp::And | BinaryOp::Or => Ty::Bool,
                }
            }
            Expr::Unary { op, right } => {
                let rt = self.infer_expr(right);
                match op {
                    UnaryOp::Neg => if rt.is_numeric() { rt } else { Ty::Unknown },
                    UnaryOp::Not => Ty::Bool,
                }
            }
            Expr::Call { callee, args } => {
                let _ = args.iter().map(|a| self.infer_expr(a)).collect::<Vec<_>>();
                // Look up function return type
                if let Expr::Variable(name) = callee.as_ref() {
                    if let Some((_, ret_ty)) = self.env.get_func(name) {
                        return ret_ty.clone();
                    }
                }
                Ty::Unknown
            }
            Expr::Index { target, index } => {
                let tt = self.infer_expr(target);
                let _ = self.infer_expr(index);
                match tt {
                    Ty::Array(elem) => (*elem).clone(),
                    Ty::Dict(_, v) => (*v).clone(),
                    Ty::String => Ty::String,
                    Ty::Any => Ty::Any,
                    Ty::Unknown => Ty::Unknown,
                    _ => {
                        self.error(format!("Cannot index type {}", tt));
                        Ty::Unknown
                    }
                }
            }
            Expr::FieldAccess { target, field } => {
                let _ = self.infer_expr(target);
                let _ = field;
                Ty::Unknown
            }
            Expr::Array(items) => {
                if items.is_empty() {
                    Ty::Array(Box::new(Ty::Unknown))
                } else {
                    let elem = self.infer_expr(&items[0]);
                    Ty::Array(Box::new(elem))
                }
            }
            Expr::Dict(pairs) => {
                if pairs.is_empty() {
                    Ty::Dict(Box::new(Ty::Unknown), Box::new(Ty::Unknown))
                } else {
                    let k = self.infer_expr(&pairs[0].0);
                    let v = self.infer_expr(&pairs[0].1);
                    Ty::Dict(Box::new(k), Box::new(v))
                }
            }
            Expr::Assign { target, value, .. } => {
                let val_ty = self.infer_expr(value);
                // Try to update the target variable's type
                if let Expr::Variable(name) = target.as_ref() {
                    if let Some(existing) = self.env.get(name) {
                        // Allow reassignment with compatible type
                        if *existing != val_ty && *existing != Ty::Unknown {
                            // Just use existing type
                        }
                    }
                }
                val_ty
            }
            Expr::Range { .. } => Ty::Array(Box::new(Ty::Int)),
            Expr::Closure { .. } => Ty::Unknown,
            Expr::MatchExpr { arms, .. } => {
                if arms.is_empty() {
                    Ty::Unknown
                } else {
                    self.infer_expr(&arms[0].body)
                }
            }
            Expr::IncDec { target, is_inc: _, is_prefix: _ } => {
                self.infer_expr(target)
            }
        }
    }
}
