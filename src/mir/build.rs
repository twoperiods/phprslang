use crate::parser::*;
use crate::typeck::{Ty, TypeEnv};
use crate::mir::ir::*;

struct LoopState {
    break_block: Vec<BlockId>,
    continue_block: Vec<BlockId>,
}

pub struct MirBuilder {
    env: TypeEnv,
}

impl MirBuilder {
    pub fn new(env: TypeEnv) -> Self {
        Self { env }
    }

    pub fn build(self, program: &Program) -> MirProgram {
        let mut prog = MirProgram::new();
        let mut loops = LoopState { break_block: vec![], continue_block: vec![] };

        // Separate function defs from top-level statements
        let (funcs, stmts): (Vec<&Stmt>, Vec<&Stmt>) = program.stmts.iter().partition(|s| {
            matches!(s, Stmt::Function { .. } | Stmt::ExternFunction { .. } | Stmt::StructDef { .. } | Stmt::EnumDef { .. })
        });

        // Build each function
        for stmt in &funcs {
            if let Stmt::Function { name, params, return_type, body } = stmt {
                let param_tys: Vec<(String, Ty)> = params.iter().map(|p| {
                    let ty = p.ty.as_ref().map(|t| Ty::from_ast_type(t)).unwrap_or(Ty::Unknown);
                    (p.name.clone(), ty)
                }).collect();
                let ret_ty = return_type.as_ref().map(|t| Ty::from_ast_type(t)).unwrap_or(Ty::Void);

                let mut func = MirFunction::new(name.clone(), param_tys, ret_ty);
                let entry = func.new_block();
                let ret_block = func.new_block();
                func.set_term(ret_block, Terminator::Return(None));

                let val = build_stmt(body, entry, ret_block, &mut func, &self.env, &mut loops);
                if let Some(v) = val {
                    if matches!(func.block(ret_block).term, Terminator::Return(None)) {
                        func.set_term(ret_block, Terminator::Return(Some(v)));
                    }
                }
                prog.functions.push(func);
            }
        }

        // Build __main from non-function top-level statements
        if !stmts.is_empty() {
            let mut main_func = MirFunction::new("__main".into(), vec![], Ty::Int);
            let mut cur_block = main_func.new_block();
            let ret_block = main_func.new_block();
            main_func.set_term(ret_block, Terminator::Return(Some(Operand::Const(MirConst::Int(0)))));

            for stmt in stmts {
                build_stmt(stmt, cur_block, ret_block, &mut main_func, &self.env, &mut loops);
                // If the current block was terminated, create a new one for the next stmt
                if matches!(main_func.block(cur_block).term, Terminator::Return(_) | Terminator::Jump(_) | Terminator::Branch { .. }) {
                    let next = main_func.new_block();
                    main_func.set_term(cur_block, Terminator::Jump(next));
                    cur_block = next;
                }
            }
            // Jump to return if not already terminated
            if !matches!(main_func.block(cur_block).term, Terminator::Return(_)) {
                main_func.set_term(cur_block, Terminator::Jump(ret_block));
            }
            prog.functions.push(main_func);
        }

        prog
    }
}

fn build_stmt(
    stmt: &Stmt,
    block: BlockId,
    ret_block: BlockId,
    func: &mut MirFunction,
    env: &TypeEnv,
    loops: &mut LoopState,
) -> Option<Operand> {
    match stmt {
        Stmt::Let { value, .. } => {
            if let Some(val_expr) = value {
                let val = build_expr(val_expr, block, func, env);
                let ptr = func.new_reg();
                func.push_stmt(block, MirStmt::Alloca { dst: ptr, ty: Ty::Unknown });
                func.push_stmt(block, MirStmt::Store { ptr, value: val });
                Some(Operand::Reg(ptr))
            } else {
                None
            }
        }
        Stmt::Echo(expr) => {
            let val = build_expr(expr, block, func, env);
            func.push_stmt(block, MirStmt::Print { value: val });
            None
        }
        Stmt::ExprStmt(expr) => {
            let val = build_expr(expr, block, func, env);
            Some(val)
        }
        Stmt::Return(Some(expr)) => {
            let val = build_expr(expr, block, func, env);
            func.set_term(block, Terminator::Return(Some(val)));
            None
        }
        Stmt::Return(None) => {
            func.set_term(block, Terminator::Return(None));
            None
        }
        Stmt::Block(stmts) => {
            let cur_block = block;
            let mut last_val = None;
            for s in stmts {
                let val = build_stmt(s, cur_block, ret_block, func, env, loops);
                if val.is_some() {
                    last_val = val;
                }
                if matches!(func.block(cur_block).term, Terminator::Return(_) | Terminator::Jump(_)) {
                    break;
                }
            }
            last_val
        }
        Stmt::If { condition, then_branch, else_branch } => {
            let cond = build_expr(condition, block, func, env);
            let then_block = func.new_block();
            let else_block = if else_branch.is_some() { func.new_block() } else { ret_block };
            let merge_block = func.new_block();

            func.set_term(block, Terminator::Branch {
                cond,
                then_block,
                else_block: if else_branch.is_some() { else_block } else { merge_block },
            });

            build_stmt(then_branch, then_block, ret_block, func, env, loops);
            if !matches!(func.block(then_block).term, Terminator::Return(_)) {
                func.set_term(then_block, Terminator::Jump(merge_block));
            }

            if let Some(eb) = else_branch {
                build_stmt(eb, else_block, ret_block, func, env, loops);
                if !matches!(func.block(else_block).term, Terminator::Return(_)) {
                    func.set_term(else_block, Terminator::Jump(merge_block));
                }
            }

            func.block_mut(merge_block);
            None
        }
        Stmt::For { init, condition, update, body } => {
            build_stmt(init, block, ret_block, func, env, loops);

            let cond_block = func.new_block();
            let body_block = func.new_block();
            let update_block = func.new_block();
            let exit_block = func.new_block();

            // Jump from the init block to condition
            if !matches!(func.block(block).term, Terminator::Return(_)) {
                func.set_term(block, Terminator::Jump(cond_block));
            }

            loops.break_block.push(exit_block);
            loops.continue_block.push(update_block);

            if let Some(cond_expr) = condition {
                let cond_val = build_expr(cond_expr, cond_block, func, env);
                func.set_term(cond_block, Terminator::Branch {
                    cond: cond_val,
                    then_block: body_block,
                    else_block: exit_block,
                });
            } else {
                func.set_term(cond_block, Terminator::Jump(body_block));
            }

            build_stmt(body, body_block, ret_block, func, env, loops);
            if !matches!(func.block(body_block).term, Terminator::Return(_)) {
                func.set_term(body_block, Terminator::Jump(update_block));
            }

            if let Some(upd) = update {
                build_expr(upd, update_block, func, env);
            }
            func.set_term(update_block, Terminator::Jump(cond_block));

            loops.break_block.pop();
            loops.continue_block.pop();

            func.block_mut(exit_block);
            None
        }
        Stmt::While { condition, body } => {
            let cond_block = func.new_block();
            let body_block = func.new_block();
            let exit_block = func.new_block();

            func.set_term(block, Terminator::Jump(cond_block));

            loops.break_block.push(exit_block);
            loops.continue_block.push(cond_block);

            let cond_val = build_expr(condition, cond_block, func, env);
            func.set_term(cond_block, Terminator::Branch {
                cond: cond_val,
                then_block: body_block,
                else_block: exit_block,
            });

            build_stmt(body, body_block, ret_block, func, env, loops);
            if !matches!(func.block(body_block).term, Terminator::Return(_)) {
                func.set_term(body_block, Terminator::Jump(cond_block));
            }

            loops.break_block.pop();
            loops.continue_block.pop();

            func.block_mut(exit_block);
            None
        }
        Stmt::DoWhile { body, condition } => {
            let body_block = func.new_block();
            let cond_block = func.new_block();
            let exit_block = func.new_block();

            // Jump from current block to body (always executes at least once)
            func.set_term(block, Terminator::Jump(body_block));

            loops.break_block.push(exit_block);
            loops.continue_block.push(cond_block);

            build_stmt(body, body_block, ret_block, func, env, loops);
            if !matches!(func.block(body_block).term, Terminator::Return(_)) {
                func.set_term(body_block, Terminator::Jump(cond_block));
            }

            let cond_val = build_expr(condition, cond_block, func, env);
            func.set_term(cond_block, Terminator::Branch {
                cond: cond_val,
                then_block: body_block,
                else_block: exit_block,
            });

            loops.break_block.pop();
            loops.continue_block.pop();

            func.block_mut(exit_block);
            None
        }
        Stmt::Foreach { iterable, .. } => {
            build_expr(iterable, block, func, env);
            let body_block = func.new_block();
            let exit_block = func.new_block();
            func.set_term(block, Terminator::Jump(body_block));
            func.set_term(body_block, Terminator::Jump(exit_block));
            func.block_mut(exit_block);
            None
        }
        Stmt::Match { arms, .. } => {
            let exit_block = func.new_block();
            let arm_blocks: Vec<BlockId> = arms.iter().map(|_| func.new_block()).collect();

            if !arms.is_empty() {
                func.set_term(block, Terminator::Jump(arm_blocks[0]));
                for (i, arm) in arms.iter().enumerate() {
                    build_expr(&arm.body, arm_blocks[i], func, env);
                    if !matches!(func.block(arm_blocks[i]).term, Terminator::Return(_)) {
                        func.set_term(arm_blocks[i], Terminator::Jump(exit_block));
                    }
                }
            } else {
                func.set_term(block, Terminator::Jump(exit_block));
            }

            func.block_mut(exit_block);
            None
        }
        Stmt::Break => {
            if let Some(target) = loops.break_block.last() {
                func.set_term(block, Terminator::Jump(*target));
            }
            None
        }
        Stmt::Continue => {
            if let Some(target) = loops.continue_block.last() {
                func.set_term(block, Terminator::Jump(*target));
            }
            None
        }
        Stmt::Throw(expr) => {
            let val = build_expr(expr, block, func, env);
            func.set_term(block, Terminator::Throw(val));
            None
        }
        Stmt::TryCatch { try_body, catch_var, catch_body } => {
            let try_block = func.new_block();
            let catch_block = func.new_block();
            let merge_block = func.new_block();

            // Jump from current block to try block
            func.set_term(block, Terminator::Jump(try_block));

            // Build try body — throw will jump to catch_block via the Throw terminator
            let _ = build_stmt(try_body, try_block, ret_block, func, env, loops);

            // If try body completes without throw, jump to merge
            if !matches!(func.block(try_block).term, Terminator::Return(_) | Terminator::Jump(_) | Terminator::Branch { .. } | Terminator::Throw(_)) {
                func.set_term(try_block, Terminator::Jump(merge_block));
            }

            // Build catch body
            build_stmt(catch_body, catch_block, ret_block, func, env, loops);
            if !matches!(func.block(catch_block).term, Terminator::Return(_)) {
                func.set_term(catch_block, Terminator::Jump(merge_block));
            }

            // For MIR, we note catch_block as the handler for throws
            // The C backend will wire up the throw-to-catch edges
            let _ = catch_var;
            func.block_mut(merge_block);
            None
        }
        _ => None,
    }
}

fn build_expr(
    expr: &Expr,
    block: BlockId,
    func: &mut MirFunction,
    env: &TypeEnv,
) -> Operand {
    match expr {
        Expr::Literal(lit) => {
            let dst = func.new_reg();
            let const_val = match lit {
                Literal::Int(n) => MirConst::Int(*n),
                Literal::Float(n) => MirConst::Float(*n),
                Literal::String_(s) => MirConst::String(s.clone()),
                Literal::Bool(b) => MirConst::Bool(*b),
                Literal::Null => MirConst::Null,
            };
            func.push_stmt(block, MirStmt::Const { dst, value: const_val });
            Operand::Reg(dst)
        }
        Expr::Variable(name) => {
            let dst = func.new_reg();
            func.push_stmt(block, MirStmt::Const { dst, value: MirConst::String(name.clone()) });
            Operand::Reg(dst)
        }
        Expr::Binary { left, op, right } => {
            let l = build_expr(left, block, func, env);
            let r = build_expr(right, block, func, env);
            let dst = func.new_reg();
            let mir_op = match op {
                BinaryOp::Add => MirBinOp::Add,
                BinaryOp::Sub => MirBinOp::Sub,
                BinaryOp::Mul => MirBinOp::Mul,
                BinaryOp::Div => MirBinOp::Div,
                BinaryOp::Mod => MirBinOp::Mod,
                BinaryOp::Eq => MirBinOp::Eq,
                BinaryOp::Neq => MirBinOp::Neq,
                BinaryOp::StrictEq => MirBinOp::StrictEq,
                BinaryOp::StrictNeq => MirBinOp::StrictNeq,
                BinaryOp::Lt => MirBinOp::Lt,
                BinaryOp::Gt => MirBinOp::Gt,
                BinaryOp::Le => MirBinOp::Le,
                BinaryOp::Ge => MirBinOp::Ge,
                BinaryOp::And => MirBinOp::And,
                BinaryOp::Or => MirBinOp::Or,
                BinaryOp::Concat => MirBinOp::Concat,
            };
            func.push_stmt(block, MirStmt::Binary { dst, op: mir_op, left: l, right: r });
            Operand::Reg(dst)
        }
        Expr::Unary { op, right } => {
            let r = build_expr(right, block, func, env);
            let dst = func.new_reg();
            let mir_op = match op {
                UnaryOp::Neg => MirUnOp::Neg,
                UnaryOp::Not => MirUnOp::Not,
            };
            func.push_stmt(block, MirStmt::Unary { dst, op: mir_op, operand: r });
            Operand::Reg(dst)
        }
        Expr::Call { callee, args } => {
            let arg_vals: Vec<Operand> = args.iter()
                .map(|a| build_expr(a, block, func, env))
                .collect();
            let func_name = match callee.as_ref() {
                Expr::Variable(name) => name.clone(),
                _ => "unknown".into(),
            };
            let dst = func.new_reg();
            func.push_stmt(block, MirStmt::Call { dst: Some(dst), func: func_name, args: arg_vals });
            Operand::Reg(dst)
        }
        Expr::Array(items) => {
            let _args: Vec<Operand> = items.iter()
                .map(|i| build_expr(i, block, func, env))
                .collect();
            let dst = func.new_reg();
            func.push_stmt(block, MirStmt::Const { dst, value: MirConst::Null });
            Operand::Reg(dst)
        }
        Expr::Dict(_pairs) => {
            let dst = func.new_reg();
            func.push_stmt(block, MirStmt::Const { dst, value: MirConst::Null });
            Operand::Reg(dst)
        }
        Expr::Assign { value, .. } => {
            build_expr(value, block, func, env)
        }
        Expr::MatchExpr { arms, .. } => {
            if let Some(arm) = arms.first() {
                build_expr(&arm.body, block, func, env)
            } else {
                let dst = func.new_reg();
                func.push_stmt(block, MirStmt::Const { dst, value: MirConst::Null });
                Operand::Reg(dst)
            }
        }
        Expr::IncDec { .. } | Expr::Index { .. } | Expr::FieldAccess { .. } | Expr::Range { .. } | Expr::Closure { .. } => {
            let dst = func.new_reg();
            func.push_stmt(block, MirStmt::Const { dst, value: MirConst::Null });
            Operand::Reg(dst)
        }
    }
}
