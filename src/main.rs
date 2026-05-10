use std::env;
use std::fs;
use std::process;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    match args[1].as_str() {
        "run" => cmd_run(&args),
        "build" => cmd_build(&args),
        "c" | "emit-c" => cmd_emit_c(&args),
        "new" | "create_project" | "create-project" => cmd_create_project(&args),
        "help" | "--help" | "-h" => print_help(),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_usage();
            process::exit(1);
        }
    }
}

fn cmd_run(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: phprs run <file.phprs> [-- arg1 arg2 ...]");
        process::exit(1);
    }
    let path = &args[2];
    let source = read_file(path);
    let source_path = Path::new(path);
    let script_args = build_script_args(path, args, 3);
    match phprs::run_with_args(&source, Some(source_path), &script_args) {
        Ok(_) => println!(),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

fn cmd_build(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: phprs build <file.phprs> [-o output]");
        process::exit(1);
    }

    let path = &args[2];
    let source = read_file(path);

    // Determine output path
    let output = if args.len() >= 5 && args[3] == "-o" {
        args[4].clone()
    } else {
        let p = Path::new(path);
        let stem = p.file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        if cfg!(windows) { format!("{}.exe", stem) } else { stem }
    };

    println!("Compiling {} -> {}", path, output);
    let source_path = Path::new(path);
    match phprs::build_with_path(&source, &output, Some(source_path)) {
        Ok(()) => {
            println!("Successfully compiled to '{}'", output);
            println!("Run: ./{}", output);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

fn cmd_emit_c(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: phprs emit-c <file.phprs>");
        process::exit(1);
    }
    let path = &args[2];
    let source = read_file(path);
    let file_path = Path::new(path);
    let dir = file_path.parent().unwrap_or(Path::new("."));
    let source = match phprs::preprocessor::preprocess(&source, dir) {
        Ok(s) => s,
        Err(e) => { eprintln!("Preprocessor error: {}", e); process::exit(1); }
    };

    let mut lexer = phprs::Lexer::new(&source);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => { eprintln!("Lexer error: {}", e); process::exit(1); }
    };
    let mut parser = phprs::Parser::new(tokens);
    let program = match parser.parse_program() {
        Ok(p) => p,
        Err(e) => { eprintln!("Parse error: {}", e); process::exit(1); }
    };
    let c_code = phprs::codegen::transpile_program(&program);
    println!("{}", c_code);
}

fn read_file(path: &str) -> String {
    match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", path, e);
            process::exit(1);
        }
    }
}

fn cmd_create_project(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: phprs create_project <project_name>");
        eprintln!("  Aliases: new, create-project");
        eprintln!();
        eprintln!("  Creates a new PHPRS MVC project with:");
        eprintln!("    - Ready-to-compile app.phprs entry point");
        eprintln!("    - Minimal C-safe runtime declarations");
        eprintln!("    - Default controller & view helpers");
        eprintln!("    - Standard directory structure");
        process::exit(1);
    }
    let name = &args[2];
    match phprs::scaffold::create_project(name) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

fn build_script_args(script_path: &str, args: &[String], start: usize) -> Vec<String> {
    let mut script_args = vec![script_path.to_string()];
    let mut i = start;
    while i < args.len() {
        if args[i] == "--" {
            i += 1;
            break;
        }
        if args[i] == "-o" {
            i += 2;
            continue;
        }
        break;
    }
    while i < args.len() {
        script_args.push(args[i].clone());
        i += 1;
    }
    script_args
}

fn print_usage() {
    eprintln!("PHPRS Compiler v0.2.0");
    eprintln!("Usage:");
    eprintln!("  phprs run   <file.phprs>         Run a PHPRS script (interpreter)");
    eprintln!("  phprs build <file.phprs> [-o exe] Compile to native binary");
    eprintln!("  phprs emit-c <file.phprs>        Emit generated C code");
    eprintln!("  phprs create_project <name>      Scaffold a new MVC project");
    eprintln!("  phprs help                        Show this help");
}

fn print_help() {
    println!("PHPRS Compiler v0.2.0");
    println!();
    println!("A new language combining PHP's simple syntax with Rust's performance.");
    println!("File extension: .phprs");
    println!("Code must be enclosed in: <?phprs ... ?>");
    println!();
    println!("Commands:");
    println!("  phprs run   <file.phprs>         Interpret and run a PHPRS script");
    println!("  phprs build <file.phprs> [-o exe] Compile to native binary (via C)");
    println!("  phprs emit-c <file.phprs>        Output generated C source code");
    println!("  phprs create_project <name>      Scaffold a new MVC project");
    println!("  phprs help                        Show this help");
    println!();
    println!("Examples:");
    println!("  phprs run hello.phprs");
    println!("  phprs build hello.phprs -o hello.exe");
    println!("  phprs emit-c hello.phprs");
    println!("  phprs create_project my_app");
}
