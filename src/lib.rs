pub mod lexer;
pub mod parser;
pub mod interpreter;
pub mod typeck;
pub mod mir;
pub mod codegen;
pub mod preprocessor;
pub mod scaffold;

pub use lexer::Lexer;
pub use parser::Parser;
pub use interpreter::Interpreter;

use std::path::Path;

/// Compile and run a PHPRS source string via the interpreter.
pub fn run(source: &str) -> Result<String, String> {
    run_with_path(source, None)
}

/// Compile and run with include resolution relative to source_path.
pub fn run_with_path(source: &str, source_path: Option<&Path>) -> Result<String, String> {
    let processed = preprocess_source(source, source_path)?;
    let mut lexer = Lexer::new(&processed);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program()?;
    let mut interpreter = Interpreter::new();
    interpreter.interpret(&program)?;
    Ok("OK".to_string())
}

/// Compile a PHPRS source string to a native binary via C code generation.
pub fn build(source: &str, output: &str) -> Result<(), String> {
    build_with_path(source, output, None)
}

/// Compile with include resolution relative to source_path.
pub fn build_with_path(source: &str, output: &str, source_path: Option<&Path>) -> Result<(), String> {
    // Phase 0: Preprocess includes
    let processed = preprocess_source(source, source_path)?;

    // Phase 1: Lex + Parse
    let mut lexer = Lexer::new(&processed);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program()?;

    // Phase 2: Type check
    let mut typeck = typeck::TypeChecker::new();
    typeck.check_and_get_env(&program)
        .map_err(|errors| errors.join("\n"))?;

    // Phase 3: AST-to-C transpilation + compilation
    let c_code = codegen::transpile_program(&program);

    let temp_dir = std::env::temp_dir().join("phprs_build");
    let _ = std::fs::create_dir_all(&temp_dir);
    let c_path = temp_dir.join("output.c");
    std::fs::write(&c_path, &c_code)
        .map_err(|e| format!("Failed to write C source: {}", e))?;

    // Compile with system C compiler
    compile_c(&c_path, output)?;

    Ok(())
}

fn compile_c(c_path: &std::path::Path, output: &str) -> Result<(), String> {
    // Try MSVC first on Windows (with proper environment)
    if cfg!(windows) {
        let vs_path = find_vs()?;
        if let Some(vs_path) = vs_path {
            // Write a temp batch file to work around cmd quoting issues
            let batch_path = std::env::temp_dir().join("phprs_build").join("build.bat");
            let vcvars = format!("{}\\VC\\Auxiliary\\Build\\vcvars64.bat", vs_path);
            let batch_content = format!(
                "@echo off\r\ncall \"{}\" >nul 2>&1\r\ncl /nologo \"{}\" /Fe:\"{}\"\r\n",
                vcvars,
                c_path.display(),
                output,
            );
            if let Ok(_) = std::fs::write(&batch_path, &batch_content) {
                let status = std::process::Command::new("cmd")
                    .args(&["/c", &batch_path.to_string_lossy()])
                    .status()
                    .map_err(|_| ());
                if let Ok(s) = status {
                    if s.success() {
                        return Ok(());
                    }
                }
            }
        }
    }

    // Fall back to GCC/Clang
    try_gcc(c_path, output)
}

fn try_gcc(c_path: &std::path::Path, output: &str) -> Result<(), String> {
    let compilers = vec!["gcc", "clang", "cc"];
    for compiler in &compilers {
        if let Ok(version_out) = std::process::Command::new(compiler).arg("--version").output() {
            if version_out.status.success() {
                let mut cmd = std::process::Command::new(compiler);
                cmd.arg(c_path)
                    .arg("-o")
                    .arg(output)
                    .arg("-std=c11");
                if cfg!(windows) {
                    // MinGW on Windows needs secur32 for Schannel SSPI
                    cmd.arg("-lsecur32").arg("-lcrypt32");
                } else {
                    // POSIX needs OpenSSL
                    cmd.arg("-lssl").arg("-lcrypto");
                }
                let status = cmd.status()
                    .map_err(|e| format!("Failed to run {}: {}", compiler, e))?;
                if status.success() {
                    return Ok(());
                }
            }
        }
    }
    let c_path_str = c_path.display();
    Err(format!(
        "No working C compiler found.\n\n\
         To compile PHPRS to native binaries, install one of:\n  \
         - MinGW: choco install mingw    (recommended for Git Bash)\n  \
         - MSVC: Run from 'Developer Command Prompt for VS'\n  \
         - clang: choco install llvm\n\n\
         Generated C source is at: {}\n\
         You can compile it manually: gcc {} -o program.exe",
        c_path_str, c_path_str
    ))
}

fn find_vs() -> Result<Option<String>, String> {
    let vs_editions = ["Community", "Enterprise", "Professional"];
    for edition in &vs_editions {
        let base = format!(r"C:\Program Files\Microsoft Visual Studio\2022\{}", edition);
        if std::path::Path::new(&base).exists() {
            return Ok(Some(base));
        }
    }
    Ok(None)
}

fn preprocess_source(source: &str, source_path: Option<&Path>) -> Result<String, String> {
    if let Some(path) = source_path {
        let dir = path.parent().unwrap_or(Path::new("."));
        preprocessor::preprocess(source, dir)
    } else {
        Ok(source.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world() {
        let src = "<?phprs echo \"Hello World\"; ?>";
        run(src).unwrap();
    }

    #[test]
    fn test_variables() {
        let src = r#"<?phprs
            let $x = 42;
            let $y = $x + 8;
            echo $y;
        ?>"#;
        run(src).unwrap();
    }

    #[test]
    fn test_if_statement() {
        let src = r#"<?phprs
            let $x = 10;
            if ($x > 5) {
                echo "yes";
            } else {
                echo "no";
            }
        ?>"#;
        run(src).unwrap();
    }

    #[test]
    fn test_loop() {
        let src = r#"<?phprs
            for (let mut $i = 0; $i < 5; $i = $i + 1) {
                echo $i;
            }
        ?>"#;
        run(src).unwrap();
    }

    #[test]
    fn test_function() {
        let src = r#"<?phprs
            function add(int $a, int $b): int {
                return $a + $b;
            }
            let $result = add(3, 4);
            echo $result;
        ?>"#;
        run(src).unwrap();
    }

    #[test]
    fn test_foreach() {
        let src = r#"<?phprs
            let $items = [1, 2, 3];
            foreach ($items as $item) {
                echo $item;
            }
        ?>"#;
        run(src).unwrap();
    }

    #[test]
    fn test_match() {
        let src = r#"<?phprs
            let $score = 85;
            let $grade = match ($score) {
                0..=59 => "F",
                60..=69 => "D",
                70..=79 => "C",
                80..=89 => "B",
                90..=100 => "A",
                _ => "Invalid",
            };
            echo $grade;
        ?>"#;
        run(src).unwrap();
    }

    #[test]
    fn test_type_check_basic() {
        let src = r#"<?phprs
            let $x: int = 42;
            let $y = $x + 8;
            echo $y;
        ?>"#;
        // Just verify it type-checks without error
        let mut lexer = Lexer::new(src);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program().unwrap();
        let mut typeck = typeck::TypeChecker::new();
        typeck.check_and_get_env(&program).unwrap();
    }

    #[test]
    fn test_mir_build() {
        let src = r#"<?phprs
            let $x = 42;
            echo $x;
        ?>"#;
        let mut lexer = Lexer::new(src);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program().unwrap();
        let mut typeck = typeck::TypeChecker::new();
        let env = typeck.check_and_get_env(&program).unwrap();
        let builder = mir::MirBuilder::new(env);
        let mir_prog = builder.build(&program);
        assert!(!mir_prog.functions.is_empty());
    }

    #[test]
    fn test_c_codegen() {
        let src = r#"<?phprs
            let $x = 42;
            echo $x;
        ?>"#;
        let mut lexer = Lexer::new(src);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program().unwrap();
        let mut typeck = typeck::TypeChecker::new();
        let env = typeck.check_and_get_env(&program).unwrap();
        let builder = mir::MirBuilder::new(env);
        let mir_prog = builder.build(&program);
        let c_code = codegen::compile_to_c(&mir_prog);
        assert!(c_code.contains("__main"));
        assert!(c_code.contains("#include"));
    }
}
