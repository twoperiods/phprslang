use crate::mir::ir::*;
use crate::typeck::Ty;
use std::fs;
use std::process::Command;

pub fn compile_to_c(program: &MirProgram) -> String {
    let mut out = String::new();

    // Header
    out.push_str("#include <stdio.h>\n");
    out.push_str("#include <stdlib.h>\n");
    out.push_str("#include <string.h>\n");
    out.push_str("#include <stdint.h>\n");
    out.push_str("#include <stdbool.h>\n\n");

    // Forward declarations
    for func in &program.functions {
        let ret_c = ty_to_c(&func.ret_ty);
        let params: Vec<String> = func.params.iter()
            .map(|(_, t)| param_to_c(t).to_string())
            .collect();
        out.push_str(&format!("{} {}({});\n", ret_c, func.name, params.join(", ")));
    }
    out.push('\n');

    // Generate each function
    for func in &program.functions {
        out.push_str(&generate_function(func));
        out.push('\n');
    }

    // Generate main
    if program.functions.iter().any(|f| f.name == "__main") {
        out.push_str("int main(int argc, char** argv) {\n");
        out.push_str("    return __main();\n");
        out.push_str("}\n");
    }

    out
}

fn generate_function(func: &MirFunction) -> String {
    let mut out = String::new();
    let ret_c = ty_to_c(&func.ret_ty);

    let params_str: Vec<String> = func.params.iter()
        .map(|(n, t)| format!("{} {}", param_to_c(t), n))
        .collect();

    out.push_str(&format!("{} {}({}) {{\n", ret_c, func.name, params_str.join(", ")));

    // Declare registers
    for r in 0..func.next_reg {
        out.push_str(&format!("    int64_t r{} = 0;\n", r));
    }
    out.push_str("    int64_t tmp_i64;\n");
    out.push_str("    double tmp_f64;\n");
    out.push_str("    bool tmp_bool;\n");
    out.push_str("    const char* tmp_str;\n\n");

    // Block labels
    let block_labels: Vec<String> = func.blocks.iter()
        .map(|b| format!("block_{}", b.id))
        .collect();

    // Entry goto
    if !func.blocks.is_empty() {
        out.push_str(&format!("    goto block_{};\n\n", func.blocks[0].id));
    }

    // Generate each block
    for block in &func.blocks {
        out.push_str(&format!("{}:\n", block_labels[block.id]));

        for stmt in &block.stmts {
            out.push_str(&generate_stmt(stmt));
        }

        out.push_str(&generate_terminator(&block.term, &block_labels));
        out.push('\n');
    }

    out.push_str("}\n");
    out
}

fn generate_stmt(stmt: &MirStmt) -> String {
    match stmt {
        MirStmt::Alloca { dst, .. } => format!("    // alloca r{}\n", dst),
        MirStmt::Store { .. } => "    // store\n".to_string(),
        MirStmt::Load { dst, .. } => format!("    // load r{}\n", dst),
        MirStmt::Const { dst, value } => match value {
            MirConst::Int(n) => format!("    r{} = {}LL;\n", dst, n),
            MirConst::Float(n) => format!("    tmp_f64 = {};\n", n),
            MirConst::String(s) => {
                let escaped = escape_c_str(s);
                format!("    tmp_str = \"{}\";\n", escaped)
            }
            MirConst::Bool(b) => format!("    tmp_bool = {};\n", b),
            MirConst::Null => format!("    r{} = 0;\n", dst),
        },
        MirStmt::Binary { dst, op, left, right } => {
            let l = operand_c_expr(left);
            let r = operand_c_expr(right);
            match op {
                MirBinOp::Concat => {
                    format!(
                        "    {{ char* _buf = malloc(strlen({}) + strlen({}) + 1); strcpy(_buf, {}); strcat(_buf, {}); tmp_str = _buf; }}\n",
                        l, r, l, r
                    )
                }
                _ => {
                    let op_str = bin_op_c(op);
                    format!("    r{} = {} {} {};\n", dst, l, op_str, r)
                }
            }
        }
        MirStmt::Unary { dst, op, operand } => {
            let op_str = match op {
                MirUnOp::Neg => "-",
                MirUnOp::Not => "!",
            };
            format!("    r{} = {}({});\n", dst, op_str, operand_c_expr(operand))
        }
        MirStmt::Call { dst, func, args } => {
            let args_str: Vec<String> = args.iter().map(|a| operand_c_expr(a)).collect();
            let call = format!("{}({})", func, args_str.join(", "));
            if let Some(d) = dst {
                format!("    r{} = {};\n", d, call)
            } else {
                format!("    {};\n", call)
            }
        }
        MirStmt::Print { value } => {
            // Determine type from context - for strings, use %s; for ints, use %lld
            format!("    printf(\"%s\", {});\n", operand_c_expr(value))
        }
        MirStmt::Phi { .. } => "    // phi\n".to_string(),
    }
}

fn generate_terminator(term: &Terminator, labels: &[String]) -> String {
    match term {
        Terminator::Return(Some(op)) => format!("    return {};\n", operand_c_expr(op)),
        Terminator::Return(None) => "    return 0;\n".to_string(),
        Terminator::Jump(target) => format!("    goto {};\n", labels[*target]),
        Terminator::Branch { cond, then_block, else_block } => {
            format!("    if ({}) goto {}; else goto {};\n",
                operand_c_expr(cond), labels[*then_block], labels[*else_block])
        }
        Terminator::Switch { .. } => "    // switch\n".to_string(),
        Terminator::Throw(op) => format!("    __throw({});\n", operand_c_expr(op)),
        Terminator::Unreachable => "    return 0; /* unreachable */\n".to_string(),
    }
}

fn operand_c_expr(op: &Operand) -> String {
    match op {
        Operand::Reg(r) => format!("r{}", r),
        Operand::Const(c) => match c {
            MirConst::Int(n) => format!("{}LL", n),
            MirConst::Float(n) => format!("{}", n),
            MirConst::String(s) => format!("\"{}\"", escape_c_str(s)),
            MirConst::Bool(b) => format!("{}", b),
            MirConst::Null => "0".to_string(),
        },
    }
}

fn bin_op_c(op: &MirBinOp) -> &'static str {
    match op {
        MirBinOp::Add => "+",
        MirBinOp::Sub => "-",
        MirBinOp::Mul => "*",
        MirBinOp::Div => "/",
        MirBinOp::Mod => "%",
        MirBinOp::Eq => "==",
        MirBinOp::Neq => "!=",
        MirBinOp::StrictEq => "==",
        MirBinOp::StrictNeq => "!=",
        MirBinOp::Lt => "<",
        MirBinOp::Gt => ">",
        MirBinOp::Le => "<=",
        MirBinOp::Ge => ">=",
        MirBinOp::And => "&&",
        MirBinOp::Or => "||",
        MirBinOp::Concat => "+",
    }
}

fn ty_to_c(ty: &Ty) -> &'static str {
    match ty {
        Ty::Int => "int64_t",
        Ty::Float => "double",
        Ty::String => "const char*",
        Ty::Bool => "bool",
        Ty::Void => "void",
        _ => "int64_t",
    }
}

fn param_to_c(ty: &Ty) -> &'static str {
    match ty {
        Ty::Int => "int64_t",
        Ty::Float => "double",
        Ty::String => "const char*",
        Ty::Bool => "bool",
        _ => "int64_t",
    }
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

/// Compile MIR program to a native executable
pub fn compile_to_binary(program: &MirProgram, output_path: &str) -> Result<(), String> {
    let c_code = compile_to_c(program);

    let temp_dir = std::env::temp_dir().join("phprs_build");
    let _ = fs::create_dir_all(&temp_dir);

    let c_path = temp_dir.join("output.c");
    fs::write(&c_path, &c_code).map_err(|e| format!("Failed to write C source: {}", e))?;

    // Try common Windows C compilers
    let compilers = if cfg!(windows) {
        vec!["gcc", "clang", "cc", "cl"]
    } else {
        vec!["gcc", "clang", "cc"]
    };

    let compiler = compilers.iter().find(|c| {
        Command::new(c).arg("--version").output().is_ok()
            || Command::new(c).arg("/?").output().is_ok()
    }).copied().unwrap_or("gcc");

    let status = Command::new(compiler)
        .arg(&c_path)
        .arg("-o")
        .arg(output_path)
        .arg("-std=c11")
        .status()
        .map_err(|e| format!(
            "Failed to run C compiler '{}'. Install MinGW: choco install mingw\n  Error: {}",
            compiler, e
        ))?;

    if !status.success() {
        return Err(format!("C compiler '{}' failed. Check the generated C code at: {}",
            compiler, c_path.display()));
    }

    Ok(())
}
