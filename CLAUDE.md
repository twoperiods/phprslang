# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Test

```bash
cargo build              # Debug build
cargo build --release    # Release build (produces phprs binary)
cargo test               # Run all Rust unit tests
cargo run -- run <file.phprs>     # Interpret a PHPRS file
cargo run -- build <file.phprs>   # Compile to native binary
cargo run -- emit-c <file.phprs>  # Dump generated C code
```

## Architecture

`phprs` is a compiler for a PHP-like language that transpiles to C, then compiles to native binaries. It also includes a tree-walking interpreter for dev mode.

### Compilation Pipeline

```
Source (.phprs)
  → Preprocessor (include/require resolution, tag stripping)
  → Lexer (token stream with <?phprs ... ?> tag awareness)
  → Parser (recursive descent, produces AST)
  → Type Checker (TypeEnv-based, reports errors)
  → MIR Builder (AST → SSA-like Mid-level IR with virtual registers and basic blocks)
  → C Codegen (MIR → C source with embedded runtime)
  → System C compiler (MSVC/GCC/Clang → native binary)
```

### Module Map

| Module | Purpose |
|---|---|
| `src/main.rs` | CLI: `phprs run|build|emit-c|create_project|help` |
| `src/lib.rs` | Library root, pipeline orchestration, compile-via-C logic |
| `src/preprocessor.rs` | Text-level `include`/`require`/`include_once` resolution |
| `src/lexer/` | Tokenizer (`token.rs` types, `lexer.rs` scanner) |
| `src/parser/` | Recursive-descent parser (`ast.rs` node types, `parser.rs`) |
| `src/interpreter/` | Tree-walking interpreter for `phprs run` mode |
| `src/typeck/` | Static type checker (`ty.rs` types, `check.rs` checker with TypeEnv) |
| `src/mir/` | MIR IR definitions (`ir.rs`) and AST→MIR lowering (`build.rs`) |
| `src/codegen/` | C transpiler (`ast_to_c.rs` direct path, `c_backend.rs` MIR path) and embedded C runtime (`phprs_runtime.c`) |
| `src/scaffold.rs` | `phprs create_project` MVC project generator |

### Key Design Points

- **Two codegen paths**: `ast_to_c.rs` provides direct AST→C transpilation (`transpile_program`). `c_backend.rs` goes through the MIR first (`compile_to_c`). The `phprs build` command uses the MIR path.
- **MIR is SSA-like**: Basic blocks with virtual registers (`%0`, `%1`, ...), phi nodes, and terminators (return, jump, branch, switch).
- **Interpreter mode** (`phprs run`) skips type checking and MIR — it evaluates the AST directly via `Interpreter::interpret()`.
- **Include resolution** happens textually before lexing. It handles tag stripping when files are included inside an active `<?phprs` block.
- **C compilation**: On Windows, prefers MSVC (via vcvars64.bat). Falls back to GCC/Clang on PATH.
- **File extension**: `.phprs` sources must be wrapped in `<?phprs ... ?>` tags.
- **$variables**: Variables are prefixed with `$` (like PHP), parsed as `TokenKind::Var`.
