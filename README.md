# PHPRS

**A PHP-like language that compiles to native binaries via C.**

Write PHP-style code, compile it to a standalone `.exe` — no interpreter, no VM, no runtime overhead. PHPRS combines the simplicity of PHP syntax with the performance of compiled languages.

## Quick Start

```bash
# Install from source (requires Rust + C compiler)
git clone https://github.com/twoperiods/phprslang.git
cd phprslang
cargo build --release

# Run a script (interpreted mode)
./target/release/phprs run examples/websocket/echo.phprs

# Compile to native binary
./target/release/phprs build examples/blog/app.phprs -o blog.exe
./blog.exe

# Scaffold an MVC project
./target/release/phprs create_project my_app
cd my_app
../target/release/phprs run app.phprs    # Dev mode
```

## Command-Line Interface

```
phprs run   <file.phprs>          Run (interpreted, instant feedback)
phprs build <file.phprs> [-o exe] Compile to native binary
phprs emit-c <file.phprs>         Dump generated C source
phprs create_project <name>       Scaffold MVC project
phprs help                         Show help
```

## Language Overview

PHPRS syntax is a subset of PHP with static typing. Source files use the `.phprs` extension and must wrap code in `<?phprs ... ?>` tags.

```php
<?phprs
// Variables
let $name = "World";
let $count = 42;

// Functions with type annotations
function greet(string $name): string {
    return "Hello, " . $name . "!";
}

// Conditions
if ($count > 0) {
    echo greet($name) . "\n";
}

// Arrays & dicts
let $items = ["apple", "banana", "cherry"];
let $config = ["host" => "127.0.0.1", "port" => "8080"];

// JSON
echo json_encode(["status" => "ok", "data" => $items]);
?>
```

### Built-in Functions

Over 100 built-in functions covering:

| Category | Functions |
|---|---|
| **String** | `strlen`, `substr`, `strpos`, `explode`, `implode`, `sprintf`, `trim`, `str_replace`, `strtolower`, `strtoupper`, `htmlspecialchars`, `nl2br`, `strip_tags` |
| **File I/O** | `file_get_contents`, `file_put_contents`, `file_exists`, `is_dir`, `mkdir`, `unlink`, `basename`, `dirname`, `scandir`, `realpath` |
| **JSON** | `json_encode`, `json_decode`, `phprs_json_get_string`, `phprs_json_get_int` |
| **HTTP** | `phprs_http_response`, `phprs_http_method`, `phprs_http_path`, `phprs_http_header`, `phprs_http_body`, `curl`, `curl_async` |
| **Net** | `phprs_server_new`, `phprs_server_accept`, `phprs_socket_read`, `phprs_socket_write`, `phprs_socket_close` |
| **WebSocket** | `phprs_is_websocket_upgrade`, `phprs_ws_handshake_response`, `phprs_ws_read_frame`, `phprs_ws_write_frame`, `phprs_ws_close` |
| **Type** | `is_null`, `is_int`, `is_float`, `is_string`, `is_bool`, `is_array`, `empty`, `isset`, `gettype` |
| **Hash** | `md5`, `sha1`, `uniqid` |
| **Math** | `abs`, `ceil`, `floor`, `round`, `max`, `min`, `rand`, `mt_rand`, `pow`, `sqrt` |
| **Date** | `time`, `date`, `strtotime`, `microtime` |

## Architecture

### Compilation Pipeline

```
Source (.phprs)
  → Preprocessor    (include/require resolution, tag stripping)
  → Lexer           (token stream)
  → Parser          (recursive descent → AST)
  → Type Checker    (static analysis, TypeEnv)
  → MIR Builder     (AST → SSA-like Mid-level IR)
  → C Codegen       (MIR → C source + embedded runtime)
  → System CC       (MSVC/GCC/Clang → native binary)
```

### Two Execution Modes

| Mode | Command | How it works |
|---|---|---|
| **Interpreter** | `phprs run` | Tree-walking evaluator over AST — instant, no compile step |
| **Compiled** | `phprs build` | Full pipeline: AST → MIR → C → native binary |

### Module Map

| Module | Purpose |
|---|---|
| `src/main.rs` | CLI entry point |
| `src/lib.rs` | Library root, pipeline orchestration |
| `src/preprocessor.rs` | Text-level include/require resolution |
| `src/lexer/` | Tokenizer |
| `src/parser/` | Recursive-descent parser, AST definitions |
| `src/interpreter/` | Tree-walking interpreter for dev mode |
| `src/typeck/` | Static type checker |
| `src/mir/` | MIR definitions and AST→MIR lowering |
| `src/codegen/` | C transpiler + embedded C runtime (3757 lines) |
| `src/scaffold.rs` | MVC project generator (1359 lines) |

### C Runtime (`phprs_runtime.c`)

A self-contained C library compiled into every binary. Provides HTTP server, WebSocket, JSON, TLS/HTTPS, file I/O, string utilities, hash functions, and a custom memory allocator. No external dependencies.

## MVC Framework

`phprs create_project` generates a production-ready MVC project:

```
my_app/
├── app.phprs                    Entry point + server loop
├── system/                      Runtime & core libraries
│   ├── runtime.phprs            Extern declarations (100+ functions)
│   ├── request.phprs            Request parsing, session, CSRF
│   ├── response.phprs           HTTP response builders
│   ├── view.phprs               Template engine
│   ├── websocket.phprs          WebSocket helpers
│   ├── http_client.phprs        HTTP client
│   └── curl.phprs               cURL wrapper
├── config/                      Configuration files
│   ├── router_simple.phprs      Simple route parser
│   ├── router.phprs             Basic path router
│   ├── router_advanced.phprs    Advanced router (typed params)
│   ├── database.phprs           Database config (Webman-style)
│   └── redis.phprs              Redis config (Webman-style)
├── middleware/                   Request middleware
│   ├── rate_limit.phprs         Rate limiter (IP-based)
│   └── cors.phprs               CORS header injection
├── controllers/                 MVC controllers
│   ├── home_controller.phprs    Default routes (/, /about)
│   ├── db_controller.phprs      Database CRUD examples
│   ├── redis_controller.phprs   Redis key-value examples
│   └── ws_controller.phprs      WebSocket chat/echo examples
├── views/
│   └── layout.phprs             HTML layout + template helpers
└── data/                        File-based storage
```

### Built-in Features

- **Rate limiting** — 100 req/min per IP (configurable)
- **CORS** — Wildcard origin, configurable methods/headers
- **JSON file database** — CRUD with auto-increment IDs
- **Key-value store** — Redis-style get/set/del/keys
- **WebSocket** — Chat and echo endpoints on same port
- **Type-safe routing** — `/api/hello?name={any}&age={int}`

### Example API Endpoints

```
GET  /api/hello?name=Alice&age=25    → JSON { message, name, age }
POST /api/user                        → JSON { name, email } (JSON or form)
POST /api/upload                      → JSON upload receipt
GET  /api/db/list                     → List records
POST /api/db/create                   → Create record
POST /api/db/update                   → Update record
POST /api/db/delete?id=xxx            → Delete record
POST /api/redis/set                   → Set key-value
GET  /api/redis/get?key=xxx           → Get value
GET  /api/redis/keys                  → List all keys
GET  /api/ws/info                    → WebSocket info page
WS   ws://localhost:8080/ws/chat      → Chat endpoint
WS   ws://localhost:8080/ws/echo      → Echo endpoint
```

## Examples

| Directory | Description |
|---|---|
| `examples/blog/` | Blog with routing and templates |
| `examples/binotes/` | Note-taking app with CRUD |
| `examples/websocket/` | WebSocket echo server |
| `examples/http_client/` | HTTP client demo (GET/POST) |
| `examples/threaded/` | Multi-threaded blog server |

## Requirements

- **Rust** 1.70+ (build the compiler)
- **C compiler** (MSVC on Windows, GCC/Clang on Linux/macOS)
- Windows: Visual Studio Build Tools (MSVC)
- Linux: `build-essential`
- macOS: Xcode Command Line Tools

## License

MIT
