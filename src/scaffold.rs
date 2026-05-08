/// Scaffolding for `phprs create_project <name>` — generates a self-contained MVC project.
use std::fs;
use std::path::Path;

// Framework library files embedded at compile time — each project gets its own copy.
const ROUTER_SIMPLE: &str = include_str!("../framework/router_simple.phprs");
const ROUTER: &str = include_str!("../framework/router.phprs");
const ROUTER_ADVANCED: &str = include_str!("../framework/router_advanced.phprs");
const REQUEST: &str = include_str!("../framework/request.phprs");
const RESPONSE: &str = include_str!("../framework/response.phprs");
const VIEW: &str = include_str!("../framework/view.phprs");
const WEBSOCKET: &str = include_str!("../framework/websocket.phprs");
const HTTP_CLIENT: &str = include_str!("../framework/http_client.phprs");
const CURL: &str = include_str!("../framework/curl.phprs");
const RATE_LIMIT_MW: &str = include_str!("../framework/middleware/rate_limit.phprs");
const CORS_MW: &str = include_str!("../framework/middleware/cors.phprs");

pub fn create_project(name: &str) -> Result<(), String> {
    let root = Path::new(name);

    if root.exists() {
        return Err(format!("Directory '{}' already exists. Choose a different project name.", name));
    }

    // Create directory structure
    fs::create_dir_all(root.join("system"))
        .map_err(|e| format!("Cannot create system/: {}", e))?;
    fs::create_dir_all(root.join("config"))
        .map_err(|e| format!("Cannot create config/: {}", e))?;
    fs::create_dir_all(root.join("middleware"))
        .map_err(|e| format!("Cannot create middleware/: {}", e))?;
    fs::create_dir_all(root.join("controllers"))
        .map_err(|e| format!("Cannot create controllers/: {}", e))?;
    fs::create_dir_all(root.join("views"))
        .map_err(|e| format!("Cannot create views/: {}", e))?;
    fs::create_dir_all(root.join("data"))
        .map_err(|e| format!("Cannot create data/: {}", e))?;

    // Entry point
    write_file(&root.join("app.phprs"), &app_template(name))?;

    // System: runtime + core libraries
    write_file(&root.join("system").join("runtime.phprs"), RUNTIME_TEMPLATE)?;
    write_file(&root.join("system").join("request.phprs"), REQUEST)?;
    write_file(&root.join("system").join("response.phprs"), RESPONSE)?;
    write_file(&root.join("system").join("view.phprs"), VIEW)?;
    write_file(&root.join("system").join("websocket.phprs"), WEBSOCKET)?;
    write_file(&root.join("system").join("http_client.phprs"), HTTP_CLIENT)?;
    write_file(&root.join("system").join("curl.phprs"), CURL)?;

    // Config: routing + database + redis
    write_file(&root.join("config").join("router_simple.phprs"), ROUTER_SIMPLE)?;
    write_file(&root.join("config").join("router.phprs"), ROUTER)?;
    write_file(&root.join("config").join("router_advanced.phprs"), ROUTER_ADVANCED)?;
    write_file(&root.join("config").join("database.phprs"), DATABASE_CONFIG)?;
    write_file(&root.join("config").join("redis.phprs"), REDIS_CONFIG)?;

    // Middleware
    write_file(&root.join("middleware").join("rate_limit.phprs"), RATE_LIMIT_MW)?;
    write_file(&root.join("middleware").join("cors.phprs"), CORS_MW)?;

    // MVC
    write_file(&root.join("controllers").join("home_controller.phprs"), HOME_CONTROLLER)?;
    write_file(&root.join("controllers").join("db_controller.phprs"), DB_CONTROLLER)?;
    write_file(&root.join("controllers").join("redis_controller.phprs"), REDIS_CONTROLLER)?;
    write_file(&root.join("controllers").join("ws_controller.phprs"), WS_CONTROLLER)?;
    write_file(&root.join("views").join("layout.phprs"), LAYOUT_VIEW)?;

    println!();
    println!("  Project '{}' created successfully!", name);
    println!();
    println!("  {}/", name);
    println!("  ├── app.phprs                       Entry point (router + middleware)");
    println!("  ├── system/                         Runtime & core libraries");
    println!("  │   ├── runtime.phprs               Extern declarations");
    println!("  │   ├── request.phprs               Request / Session / CSRF");
    println!("  │   ├── response.phprs              HTTP response builders");
    println!("  │   ├── view.phprs                  Template engine");
    println!("  │   ├── websocket.phprs             WebSocket helpers");
    println!("  │   ├── http_client.phprs           HTTP client (GET/POST)");
    println!("  │   └── curl.phprs                  curl HTTP client wrapper");
    println!("  ├── config/                         Routing + database + redis");
    println!("  │   ├── router_simple.phprs         Simple route parser");
    println!("  │   ├── router.phprs                Basic path router");
    println!("  │   ├── router_advanced.phprs       Advanced router (query params)");
    println!("  │   ├── database.phprs              Database configuration (multi-connection)");
    println!("  │   └── redis.phprs                 Redis configuration (+ connection pool)");
    println!("  ├── middleware/                     Request middleware");
    println!("  │   ├── rate_limit.phprs            Rate limit middleware");
    println!("  │   └── cors.phprs                  CORS middleware");
    println!("  ├── controllers/                    MVC controllers");
    println!("  │   ├── home_controller.phprs       Default controller");
    println!("  │   ├── db_controller.phprs         Database CRUD examples");
    println!("  │   ├── redis_controller.phprs      Redis key-value examples");
    println!("  │   └── ws_controller.phprs         WebSocket chat examples");
    println!("  ├── views/                          MVC views");
    println!("  │   └── layout.phprs                Layout & template helpers");
    println!("  └── data/                           File-based storage");
    println!();
    println!("  Getting started:");
    println!("    cd {}", name);
    println!("    phprs run app.phprs               # Development (interpreted)");
    println!("    phprs build app.phprs -o {}.exe   # Compile to native binary", name);
    println!("    ./{}.exe                           # Run the binary", name);
    println!();

    Ok(())
}

fn write_file(path: &Path, content: &str) -> Result<(), String> {
    fs::write(path, content)
        .map_err(|e| format!("Cannot write {}: {}", path.display(), e))
}

fn app_template(name: &str) -> String {
    format!(r#"<?phprs
// {} — PHPRS MVC Application
// Build:  phprs build app.phprs -o {0}.exe
// Run:    ./{0}.exe
// Dev:    phprs run app.phprs

include "system/runtime.phprs";
include "system/request.phprs";
include "system/view.phprs";
include "config/router_simple.phprs";
include "views/layout.phprs";
include "controllers/home_controller.phprs";
include "controllers/db_controller.phprs";
include "controllers/redis_controller.phprs";
include "controllers/ws_controller.phprs";
include "middleware/rate_limit.phprs";
include "middleware/cors.phprs";

// ------- Route Dispatch -------

function app_dispatch(string $method, string $path, string $raw, string $routes, int $port, int $client_fd): string {{
    let $m = route_match($method, $path, $routes);
    let $h = route_handler($m);
    let $p = route_params($m);
    let $server = request_server($raw, $port, $client_fd);

    // CSRF protection: verify token on all POST/PUT/PATCH/DELETE requests
    if ($method == "POST" || $method == "PUT" || $method == "PATCH" || $method == "DELETE") {{
        let $body = phprs_http_body($raw);
        let $csrf = request_param($body, "_csrf_token");
        if ($csrf == "") {{
            let $ct = phprs_http_header($raw, "Content-Type");
            if (phprs_str_contains($ct, "json") == 1) {{
                $csrf = phprs_json_get_string($body, "_csrf_token");
            }}
        }}
        if (csrf_verify($raw, $csrf) == 0) {{
            return api_error(403, "CSRF token invalid");
        }}
    }}

    if ($h == "home_index")    {{ return home_index(); }}
    if ($h == "home_about")    {{ return home_about(); }}

    // API: GET with query params
    if ($h == "api_hello")     {{ return api_hello($p); }}
    // API: POST body
    if ($h == "api_user_create") {{ return api_user_create($raw); }}
    // API: File upload
    if ($h == "api_upload")    {{ return api_upload($raw); }}

    // Database CRUD
    if ($h == "db_list")       {{ return db_list(); }}
    if ($h == "db_read")       {{ return db_read($p); }}
    if ($h == "db_create")     {{ return db_create($raw); }}
    if ($h == "db_update")     {{ return db_update($raw); }}
    if ($h == "db_delete")     {{ return db_delete($p); }}
    if ($h == "db_raw_query")  {{ return db_raw_query($p); }}

    // Redis KV
    if ($h == "redis_set")     {{ return redis_set($raw); }}
    if ($h == "redis_get")     {{ return redis_get($p); }}
    if ($h == "redis_del")     {{ return redis_del($p); }}
    if ($h == "redis_keys")    {{ return redis_keys(); }}
    if ($h == "redis_incr")    {{ return redis_incr($p); }}
    if ($h == "redis_hset")    {{ return redis_hset($raw); }}
    if ($h == "redis_hgetall") {{ return redis_hgetall($p); }}
    if ($h == "redis_lpush")   {{ return redis_lpush($raw); }}
    if ($h == "redis_lrange")  {{ return redis_lrange($p); }}
    if ($h == "redis_cmd")     {{ return redis_cmd($p); }}

    // WebSocket
    if ($h == "api_ws_info")   {{ return ws_info_page(); }}

    return response_404();
}}

function app_routes(): string {{
    let $R = "";
    $R = route_add($R, "GET", "/", "home_index");
    $R = route_add($R, "GET", "/about", "home_about");
    // GET with custom params: /api/hello?name=xxx&age=123
    $R = route_add($R, "GET", "/api/hello?name={{any}}&age={{int}}", "api_hello");
    // POST with body
    $R = route_add($R, "POST", "/api/user", "api_user_create");
    // File upload
    $R = route_add($R, "POST", "/api/upload", "api_upload");

    // Database CRUD
    $R = route_add($R, "GET", "/api/db/list", "db_list");
    $R = route_add($R, "GET", "/api/db/read?id={{any}}", "db_read");
    $R = route_add($R, "POST", "/api/db/create", "db_create");
    $R = route_add($R, "POST", "/api/db/update", "db_update");
    $R = route_add($R, "POST", "/api/db/delete?id={{any}}", "db_delete");
    $R = route_add($R, "POST", "/api/db/query?sql={{any}}", "db_raw_query");

    // Redis KV
    $R = route_add($R, "POST", "/api/redis/set", "redis_set");
    $R = route_add($R, "GET", "/api/redis/get?key={{any}}", "redis_get");
    $R = route_add($R, "POST", "/api/redis/del?key={{any}}", "redis_del");
    $R = route_add($R, "GET", "/api/redis/keys", "redis_keys");
    $R = route_add($R, "POST", "/api/redis/incr?key={{any}}", "redis_incr");
    $R = route_add($R, "POST", "/api/redis/hset", "redis_hset");
    $R = route_add($R, "GET", "/api/redis/hgetall?key={{any}}", "redis_hgetall");
    $R = route_add($R, "POST", "/api/redis/lpush", "redis_lpush");
    $R = route_add($R, "GET", "/api/redis/lrange?key={{any}}", "redis_lrange");
    $R = route_add($R, "GET", "/api/redis/cmd?cmd={{any}}", "redis_cmd");

    // WebSocket
    $R = route_add($R, "GET", "/api/ws/info", "api_ws_info");

    return $R;
}}

// ------- Thread Pool Request Handler -------
// Signature: string → string — auto-registered by codegen as a thread pool handler.
// The thread pool worker calls this with the raw HTTP request, writes the response,
// and closes the socket automatically.

function handle_request(string $raw): string {{
    let $routes = phprs_app_get_routes();
    let $port = phprs_app_get_port();
    let $client = 0;  // fd is managed by the thread pool worker

    let $server = request_server($raw, $port, $client);
    let $ip = server_param($server, "REMOTE_ADDR");

    // --- Rate Limit ---
    if (rate_limit_allow($ip) == 0) {{
        let $cors = cors_headers();
        let $resp = rate_limit_429_response();
        if ($cors != "") {{
            $resp = phprs_str_replace($resp, "\r\n\r\n", "\r\n" . $cors . "\r\n");
        }}
        return $resp;
    }}

    // --- CORS Preflight ---
    if (cors_is_preflight($raw) == 1) {{
        return cors_preflight_response();
    }}

    let $method = phprs_http_method($raw);
    let $path = phprs_http_path($raw);

    let $response = app_dispatch($method, $path, $raw, $routes, $port, $client);

    // --- Inject CORS headers ---
    let $cors_hdrs = cors_headers();
    if ($cors_hdrs != "") {{
        $response = phprs_str_replace($response, "\r\n\r\n", "\r\n" . $cors_hdrs . "\r\n");
    }}

    return $response;
}}

// ------- Main Server Loop -------

function app_main(): void {{
    let $port = 8080;

    // ---- Production Config ----
    phprs_config_max_body(10485760);    // 10 MB max request body
    phprs_config_timeout(30, 60);       // read=30s, write=60s
    phprs_config_max_connections(10000); // max 10000 concurrent connections
    phprs_log_init("-");                 // access log to stdout

    // ---- Redis / MySQL / WebSocket ----
    phprs_redis_init("127.0.0.1", 6379, "redis_123");
    phprs_mysql_init("127.0.0.1", 3306, "root", "boboshanghai", "test");
    phprs_ws_manager_init(30);           // 30s heartbeat interval

    // ---- Middleware Config ----
    rate_limit_config(100, 60);   // 100 req/min per IP
    cors_config("http://localhost:8080", "GET,POST,PUT,DELETE,PATCH,OPTIONS", "Content-Type,Authorization");

    let $sock = phprs_server_new($port);

    if ($sock < 0) {{
        echo "ERROR: Failed to start server on port " . $port . "\n";
        echo "Make sure the port is not already in use.\n";
        return;
    }}

    let $RT = app_routes();

    // Store routes and port in C globals for thread pool handler access
    phprs_app_set_routes($RT);
    phprs_app_set_port($port);

    // Initialize thread pool with 8 worker threads
    phprs_thread_pool_init(8);

    // Start WebSocket heartbeat background thread
    phprs_ws_start_heartbeat(30);

    echo "============================================\n";
    echo "  {0} - PHPRS MVC Application\n";
    echo "  Server: http://0.0.0.0:" . $port . "\n";
    echo "  Workers: 8 threads\n";
    echo "  Max body: 10 MB\n";
    echo "  Timeouts: read=30s write=60s\n";
    echo "  Redis:  127.0.0.1:6379\n";
    echo "  MySQL:  127.0.0.1:3306/test\n";
    echo "  WS heartbeat: 30s\n";
    echo "  Endpoints: /health /metrics\n";
    echo "  Ctrl+C for graceful shutdown.\n";
    echo "============================================\n";

    for (let mut $running = 1; $running == 1; ) {{
        // Graceful shutdown check
        if (phprs_is_shutting_down() == 1) {{
            break;
        }}

        let $client = phprs_server_accept($sock);

        if ($client >= 0) {{
            let $raw = phprs_socket_read($client, 65536);
            if ($raw != "") {{
                let $method = phprs_http_method($raw);
                let $path = phprs_http_path($raw);

                // --- WebSocket Upgrade (handled on main thread) ---
                if (phprs_is_websocket_upgrade($raw) == 1) {{
                    ws_accept($raw, $client);
                    if (phprs_str_starts_with($path, "/ws/echo") == 1) {{
                        phprs_thread_spawn("ws_handle_echo", $client, "");
                    }} else {{
                        phprs_thread_spawn("ws_handle_chat", $client, "");
                    }}
                    continue;
                }}

                // Dispatch to thread pool — worker calls handle_request,
                // writes response to socket, and closes it
                phprs_thread_pool_enqueue("handle_request", $client, $raw);
                continue;
            }}
            phprs_socket_close($client);
        }}
    }}

    echo "\nGraceful shutdown: draining thread pool...\n";
    phprs_thread_pool_shutdown();
    phprs_redis_close();
    phprs_mysql_close();
    phprs_socket_close($sock);
    echo "Server stopped.\n";
}}

app_main();
?>"#, name)
}

// ---- Minimal C-Safe Runtime ----
// Only extern declarations that do NOT conflict with the C standard library.
const RUNTIME_TEMPLATE: &str = r##"<?phprs
// PHPRS Runtime — minimal extern declarations safe for compiled mode.
// All functions use phprs_ prefix to avoid C standard library name conflicts.

// ---- Socket Primitives ----
extern function phprs_server_new(int $port): int;
extern function phprs_server_accept(int $fd): int;
extern function phprs_client_ip(int $fd): string;
extern function phprs_socket_read(int $fd, int $max_size): string;
extern function phprs_socket_write(int $fd, string $data): int;
extern function phprs_socket_close(int $fd): void;

// ---- HTTP Parsing (Server-side) ----
extern function phprs_http_method(string $raw): string;
extern function phprs_http_path(string $raw): string;
extern function phprs_http_header(string $raw, string $name): string;
extern function phprs_http_body(string $raw): string;
extern function phprs_url_decode(string $encoded): string;
extern function phprs_request_parse(string $raw): string;
extern function phprs_http_response(int $status_code, string $content_type, string $body): string;

// ---- File I/O ----
extern function phprs_file_read(string $path): string;
extern function phprs_file_write(string $path, string $content): int;
extern function phprs_file_exists(string $path): int;
extern function file_get_contents(string $path): string;
extern function file_put_contents(string $path, string $content): int;
extern function file_exists(string $path): bool;
extern function is_dir(string $path): bool;
extern function mkdir(string $path): bool;
extern function unlink(string $path): bool;
extern function basename(string $path): string;
extern function dirname(string $path): string;
extern function scandir(string $path): array;
extern function is_file(string $path): int;
extern function getcwd(): string;
extern function realpath(string $path): string;

// ---- String Helpers ----
extern function phprs_str_replace(string $s, string $from, string $to): string;
extern function phprs_str_split(string $s, string $delim, int $index): string;
extern function phprs_str_contains(string $haystack, string $needle): int;
extern function phprs_str_starts_with(string $s, string $prefix): int;
extern function phprs_str_ends_with(string $s, string $suffix): int;
extern function phprs_str_upper(string $s): string;
extern function phprs_str_lower(string $s): string;
extern function str_replace(string $search, string $replace, string $subject): string;
extern function ltrim(string $s): string;
extern function rtrim(string $s): string;
extern function ucfirst(string $s): string;
extern function strlen(string $s): int;
extern function substr(string $s, int $start, int $length): string;
extern function strpos(string $haystack, string $needle): int;
extern function stripos(string $haystack, string $needle): int;
extern function strrpos(string $haystack, string $needle): int;
extern function explode(string $delimiter, string $s): string;
extern function implode(string $glue, string $list): string;
extern function str_repeat(string $s, int $count): string;
extern function strtolower(string $s): string;
extern function strtoupper(string $s): string;
extern function htmlspecialchars(string $s): string;
extern function strip_tags(string $s): string;
extern function nl2br(string $s): string;
extern function sprintf(string $fmt, string $a1, string $a2, string $a3, string $a4): string;
extern function number_format(any $num, int $decimals): string;

// ---- URL & Encoding ----
extern function urlencode(string $s): string;
extern function urldecode(string $s): string;
extern function parse_url(string $url): string;
extern function http_build_query(any $data): string;
extern function base64_encode(string $s): string;
extern function base64_decode(string $s): string;

// ---- JSON Helpers ----
extern function phprs_json_get_string(string $json, string $key): string;
extern function phprs_json_get_int(string $json, string $key): int;
extern function json_encode(any $value): string;
extern function json_decode(string $json): any;

// ---- Type Checking (accept any type, return concrete) ----
extern function is_null(any $var): bool;
extern function is_int(any $var): bool;
extern function is_float(any $var): bool;
extern function is_string(any $var): bool;
extern function is_bool(any $var): bool;
extern function is_array(any $var): bool;
extern function gettype(any $var): string;
extern function isset(any $var): bool;
extern function empty(any $var): bool;
extern function unset(any $var): void;

// ---- Math Functions ----
extern function abs(any $n): int;
extern function ceil(float $n): int;
extern function floor(float $n): int;
extern function round(float $n, int $precision): float;
extern function max(any $a, any $b): any;
extern function min(any $a, any $b): any;
extern function rand(int $min, int $max): int;
extern function mt_rand(int $min, int $max): int;
extern function pow(any $base, any $exp): float;
extern function sqrt(float $n): float;

// ---- HTTP Client ----
extern function curl(string $url, string $options): string;
extern function curl_async(string $url, string $options): int;
extern function curl_wait(int $handle): string;
extern function curl_is_done(int $handle): int;

// ---- Date/Time ----
extern function time(): int;
extern function date(string $format, int $timestamp): string;
extern function strtotime(string $datetime): int;
extern function microtime(): string;

// ---- Misc ----
extern function sleep(int $seconds): void;
extern function usleep(int $microseconds): void;

// ---- Hash & Utility ----
extern function md5(string $s): string;
extern function sha1(string $s): string;
extern function uniqid(string $prefix): string;
extern function random_bytes(int $length): string;
extern function random_int(int $min, int $max): int;

// ---- Threading ----
extern function phprs_thread_spawn(string $func_name, int $arg, string $raw): int;
extern function phprs_thread_pool_init(int $num_threads): int;
extern function phprs_thread_pool_enqueue(string $func_name, int $fd, string $data): int;
extern function phprs_thread_pool_shutdown(): void;

// ---- App State (thread-safe globals) ----
extern function phprs_app_set_routes(string $routes): void;
extern function phprs_app_get_routes(): string;
extern function phprs_app_set_port(int $port): void;
extern function phprs_app_get_port(): int;

// ---- String Validation ----
extern function phprs_str_is_alnum(string $s): int;

// ---- Middleware ----
extern function phprs_rate_limit_init(int $max_req, int $window_sec): void;
extern function phprs_rate_limit_check(string $ip): int;
extern function phprs_cors_set_config(string $origin, string $methods, string $headers): void;
extern function phprs_cors_get_origin(): string;
extern function phprs_cors_get_methods(): string;
extern function phprs_cors_get_headers(): string;
extern function phprs_cors_is_preflight(string $raw): int;

// ---- Production Infrastructure ----
extern function phprs_config(string $json): void;
extern function phprs_config_max_body(int $bytes): void;
extern function phprs_config_timeout(int $read_sec, int $write_sec): void;
extern function phprs_config_max_connections(int $max): void;
extern function phprs_is_shutting_down(): int;
extern function phprs_log(string $msg): void;
extern function phprs_log_error_msg(string $msg): void;
extern function phprs_log_init(string $path): void;
extern function phprs_server_init_signals(): void;
extern function phprs_write_pidfile(string $path): void;

// ---- Redis Client ----
extern function phprs_redis_init(string $host, int $port, string $password): void;
extern function phprs_redis_close(): void;
extern function phprs_redis_cmd(string $command): string;
extern function phprs_redis_get(string $key): string;
extern function phprs_redis_set(string $key, string $value): string;
extern function phprs_redis_setex(string $key, int $seconds, string $value): string;
extern function phprs_redis_del(string $key): string;
extern function phprs_redis_exists(string $key): int;
extern function phprs_redis_keys(string $pattern): string;
extern function phprs_redis_expire(string $key, int $seconds): int;
extern function phprs_redis_incr(string $key): int;
extern function phprs_redis_decr(string $key): int;
extern function phprs_redis_hget(string $key, string $field): string;
extern function phprs_redis_hset(string $key, string $field, string $value): string;
extern function phprs_redis_hgetall(string $key): string;
extern function phprs_redis_lpush(string $key, string $value): string;
extern function phprs_redis_rpush(string $key, string $value): string;
extern function phprs_redis_lrange(string $key, int $start, int $stop): string;
extern function phprs_redis_ping(): string;
extern function phprs_redis_ttl(string $key): int;
extern function phprs_redis_select(int $db): string;

// ---- MySQL Client ----
extern function phprs_mysql_init(string $host, int $port, string $user, string $pass, string $dbname): void;
extern function phprs_mysql_close(): void;
extern function phprs_mysql_escape(string $s): string;
extern function phprs_mysql_query(string $sql): string;
extern function phprs_mysql_exec(string $sql): string;
extern function phprs_mysql_select(string $table, string $where_clause): string;
extern function phprs_mysql_insert(string $table, string $json_data): string;
extern function phprs_mysql_update(string $table, string $set_clause, string $where_clause): string;
extern function phprs_mysql_delete(string $table, string $where_clause): string;

// ---- WebSocket Connection Manager ----
extern function phprs_ws_manager_init(int $heartbeat_sec): void;
extern function phprs_ws_register(int $fd, string $room): int;
extern function phprs_ws_unregister(int $fd): void;
extern function phprs_ws_update_pong(int $fd): void;
extern function phprs_ws_broadcast(string $room, string $message, int $exclude_fd): int;
extern function phprs_ws_broadcast_all(string $message, int $exclude_fd): int;
extern function phprs_ws_count(string $room): int;
extern function phprs_ws_rooms(): string;
extern function phprs_ws_start_heartbeat(int $interval_sec): void;
?>"##;

// ---- Default Home Controller ----
const HOME_CONTROLLER: &str = r##"<?phprs
// Home Controller — handles the default routes ( / , /about , /api/hello ).

function home_index(): string {
    let $title = "Welcome to PHPRS MVC";
    let $body = "<div class=\"card\">
        <h2>It works!</h2>
        <p>Your PHPRS MVC application is running.</p>
        <p>This is a compiled native binary web server — no interpreter, no garbage collector, no runtime overhead.</p>
        <ul>
            <li><a href=\"/\">Home</a> — This page</li>
            <li><a href=\"/about\">About</a> — Framework info</li>
            <li><a href=\"/api/hello?name=Alice&age=25\">GET /api/hello?name=Alice&age=25</a> — JSON response</li>
            <li>POST /api/user — form body → JSON</li>
            <li>POST /api/upload — file upload → JSON</li>
            <li><a href=\"/api/db/list\">GET /api/db/list</a> — Database CRUD</li>
            <li><a href=\"/api/redis/keys\">GET /api/redis/keys</a> — Redis key-value store</li>
            <li><a href=\"/api/ws/info\">GET /api/ws/info</a> — WebSocket examples</li>
        </ul>
    </div>
    <div class=\"card\">
        <h3>Project Structure</h3>
        <pre>app.phprs
system/
  runtime.phprs, request.phprs, response.phprs,
  view.phprs, websocket.phprs, http_client.phprs, curl.phprs
config/
  router_simple.phprs, router.phprs, router_advanced.phprs
  database.phprs, redis.phprs
middleware/
  rate_limit.phprs, cors.phprs
controllers/
  home_controller.phprs, db_controller.phprs
  redis_controller.phprs, ws_controller.phprs
views/
  layout.phprs
data/</pre>
    </div>";

    return render_page($title, $body);
}

function home_about(): string {
    let $body = "<div class=\"card\">
        <h2>About PHPRS MVC</h2>
        <p><strong>PHPRS</strong> is a programming language that combines PHP's simple syntax
           with the performance of a compiled language.</p>
        <h3>Architecture</h3>
        <ul>
            <li><strong>Lexer → Parser → AST → Type Checker → C Transpiler → Native Binary</strong></li>
            <li>Zero garbage collection — value types on the stack</li>
            <li>Embedded HTTP server in the C runtime</li>
            <li>Single-file binary deployment</li>
        </ul>
        <h3>API Examples</h3>
        <ul>
            <li><strong>GET /api/hello?name=Alice&age=25</strong> — GET with params, returns JSON</li>
            <li><strong>POST /api/user</strong> — POST form-urlencoded, returns JSON</li>
            <li><strong>POST /api/upload</strong> — File upload, returns JSON</li>
        </ul>
        <h3>Database & Redis</h3>
        <ul>
            <li><strong>GET /api/db/list</strong> — List all records (JSON file store)</li>
            <li><strong>GET /api/db/read?id=xxx</strong> — Read a single record</li>
            <li><strong>POST /api/db/create</strong> — Create a record</li>
            <li><strong>POST /api/db/update</strong> — Update a record</li>
            <li><strong>POST /api/db/delete?id=xxx</strong> — Delete a record</li>
            <li><strong>POST /api/redis/set</strong> — Set key-value pair</li>
            <li><strong>GET /api/redis/get?key=xxx</strong> — Get value by key</li>
            <li><strong>GET /api/redis/keys</strong> — List all keys</li>
        </ul>
        <h3>WebSocket</h3>
        <ul>
            <li><strong>ws://localhost:8080/ws/chat</strong> — Chat endpoint</li>
            <li><strong>ws://localhost:8080/ws/echo</strong> — Echo endpoint</li>
            <li><strong><a href=\"/api/ws/info\">/api/ws/info</a></strong> — WebSocket info page</li>
        </ul>
        <h3>Commands</h3>
        <pre>phprs run app.phprs        # Development mode (interpreted)
phprs build app.phprs      # Compile to native binary
./app.exe                  # Run the binary</pre>
    </div>";

    return render_page("About PHPRS", $body);
}

// GET /api/hello?name=Alice&age=25 — query params via route, returns JSON
function api_hello(string $params): string {
    let $name = route_param($params, "name");
    let $age = route_param($params, "age");
    if ($name == "") { $name = "World"; }
    if ($age == "") { $age = "0"; }
    let $data = json_encode(["message"=>"Hello, " . $name . "!", "name"=>$name, "age"=>$age]);
    return api_response(200, "OK", $data);
}

// POST /api/user — accepts both JSON and form-urlencoded body, returns JSON
function api_user_create(string $raw): string {
    let $body = phprs_http_body($raw);
    let $content_type = phprs_http_header($raw, "Content-Type");

    let mut $name = "";
    let mut $email = "";

    // Detect JSON body
    if (phprs_str_contains($content_type, "json") == 1) {
        // JSON: {"name":"Alice","email":"alice@example.com"}
        $name = phprs_json_get_string($body, "name");
        $email = phprs_json_get_string($body, "email");
    } else {
        // form-urlencoded: name=Alice&email=alice@example.com
        $name = request_param($body, "name");
        $email = request_param($body, "email");
    }

    if ($name == "") { $name = "unknown"; }
    if ($email == "") { $email = "unknown"; }

    let $data = json_encode(["name"=>$name, "email"=>$email, "content_type"=>$content_type, "message"=>"Created"]);
    return api_response(201, "Created", $data);
}

// POST /api/upload — file upload, reads raw body + Content-Type header
function api_upload(string $raw): string {
    let $body = phprs_http_body($raw);
    let $content_type = phprs_http_header($raw, "Content-Type");
    if ($body == "") {
        return api_response(400, "No data", "[]");
    }
    let $data = json_encode([
        "content_type"=>$content_type,
        "body_size"=>strlen($body),
        "message"=>"Upload received"
    ]);
    return api_response(200, "OK", $data);
}
?>"##;

// ---- Database Controller ----
const DB_CONTROLLER: &str = r##"<?phprs
// Database Controller — CRUD operations using real MySQL driver.
// Uses phprs_mysql_* functions for direct MySQL queries.

include "../config/database.phprs";

// ---- Public API Handlers ----

// GET /api/db/list — list all records
function db_list(): string {
    let $rows = phprs_mysql_select("items", "");
    let $data = json_encode([
        "driver"=>"mysql",
        "rows"=>$rows
    ]);
    return api_response(200, "OK", $data);
}

// GET /api/db/read?id=xxx — read a single record by ID
function db_read(string $params): string {
    let $id = route_param($params, "id");
    if ($id == "") {
        return api_error(400, "Missing id parameter");
    }
    let $escaped_id = phprs_mysql_escape($id);
    let $rows = phprs_mysql_select("items", "id='" . $escaped_id . "'");
    if ($rows == "[]" || $rows == "") {
        return api_error(404, "Record not found: " . $id);
    }
    let $data = json_encode(["record"=>$rows]);
    return api_response(200, "OK", $data);
}

// POST /api/db/create — create a new record
// Body (JSON or form): name, email, title
function db_create(string $raw): string {
    let $body = phprs_http_body($raw);
    let $content_type = phprs_http_header($raw, "Content-Type");

    let mut $name = "";
    let mut $email = "";
    let mut $title = "";

    if (phprs_str_contains($content_type, "json") == 1) {
        $name = phprs_json_get_string($body, "name");
        $email = phprs_json_get_string($body, "email");
        $title = phprs_json_get_string($body, "title");
    } else {
        $name = request_param($body, "name");
        $email = request_param($body, "email");
        $title = request_param($body, "title");
    }

    if ($name == "") { $name = "Anonymous"; }
    if ($title == "") { $title = "Untitled"; }

    let $json_data = json_encode([
        "name"=>$name,
        "email"=>$email,
        "title"=>$title
    ]);
    let $result = phprs_mysql_insert("items", $json_data);

    let $data = json_encode(["message"=>"Created", "result"=>$result]);
    return api_response(201, "Created", $data);
}

// POST /api/db/update — update a record by ID
// Body (JSON or form): id, name, email, title
function db_update(string $raw): string {
    let $body = phprs_http_body($raw);
    let $content_type = phprs_http_header($raw, "Content-Type");

    let mut $id = "";
    let mut $name = "";
    let mut $email = "";
    let mut $title = "";

    if (phprs_str_contains($content_type, "json") == 1) {
        $id = phprs_json_get_string($body, "id");
        $name = phprs_json_get_string($body, "name");
        $email = phprs_json_get_string($body, "email");
        $title = phprs_json_get_string($body, "title");
    } else {
        $id = request_param($body, "id");
        $name = request_param($body, "name");
        $email = request_param($body, "email");
        $title = request_param($body, "title");
    }

    if ($id == "") {
        return api_error(400, "Missing id");
    }

    let mut $set_parts = "";
    if ($name != "") {
        let $esc_name = phprs_mysql_escape($name);
        $set_parts = $set_parts . "name='" . $esc_name . "'";
    }
    if ($email != "") {
        if ($set_parts != "") { $set_parts = $set_parts . ","; }
        let $esc_email = phprs_mysql_escape($email);
        $set_parts = $set_parts . "email='" . $esc_email . "'";
    }
    if ($title != "") {
        if ($set_parts != "") { $set_parts = $set_parts . ","; }
        let $esc_title = phprs_mysql_escape($title);
        $set_parts = $set_parts . "title='" . $esc_title . "'";
    }

    if ($set_parts == "") {
        return api_error(400, "No fields to update");
    }

    let $esc_id = phprs_mysql_escape($id);
    let $result = phprs_mysql_update("items", $set_parts, "id='" . $esc_id . "'");

    let $data = json_encode(["message"=>"Updated", "result"=>$result]);
    return api_response(200, "OK", $data);
}

// POST /api/db/delete?id=xxx — delete a record by ID
function db_delete(string $params): string {
    let $id = route_param($params, "id");
    if ($id == "") {
        return api_error(400, "Missing id parameter");
    }
    let $esc_id = phprs_mysql_escape($id);
    let $result = phprs_mysql_delete("items", "id='" . $esc_id . "'");

    let $data = json_encode(["message"=>"Deleted", "id"=>$id, "result"=>$result]);
    return api_response(200, "OK", $data);
}

// GET /api/db/query?sql=SELECT... — raw SQL query (for admin use only)
function db_raw_query(string $params): string {
    let $sql = route_param($params, "sql");
    if ($sql == "") {
        return api_error(400, "Missing sql parameter");
    }
    let $result = phprs_mysql_query($sql);
    let $data = json_encode(["query"=>$sql, "result"=>$result]);
    return api_response(200, "OK", $data);
}
?>"##;

// ---- Redis Controller ----
const REDIS_CONTROLLER: &str = r##"<?phprs
// Redis Controller — key-value operations using real Redis driver.
// Uses phprs_redis_* functions for direct Redis RESP protocol communication.

include "../config/redis.phprs";

// ---- Public API Handlers ----

// POST /api/redis/set — set a key-value pair
// Body (JSON or form): key, value, ttl (optional seconds)
function redis_set(string $raw): string {
    let $body = phprs_http_body($raw);
    let $content_type = phprs_http_header($raw, "Content-Type");

    let mut $key = "";
    let mut $value = "";
    let mut $ttl = 0;

    if (phprs_str_contains($content_type, "json") == 1) {
        $key = phprs_json_get_string($body, "key");
        $value = phprs_json_get_string($body, "value");
        $ttl = phprs_json_get_int($body, "ttl");
    } else {
        $key = request_param($body, "key");
        $value = request_param($body, "value");
    }

    if ($key == "") {
        return api_error(400, "Missing key");
    }

    let mut $result = "";
    if ($ttl > 0) {
        $result = phprs_redis_setex($key, $ttl, $value);
    } else {
        $result = phprs_redis_set($key, $value);
    }

    let $data = json_encode([
        "message"=>$result,
        "key"=>$key,
        "value"=>$value,
        "ttl"=>$ttl
    ]);
    return api_response(200, "OK", $data);
}

// GET /api/redis/get?key=xxx — get value by key
function redis_get(string $params): string {
    let $key = route_param($params, "key");
    if ($key == "") {
        return api_error(400, "Missing key parameter");
    }
    let $value = phprs_redis_get($key);
    if ($value == "(nil)") {
        return api_error(404, "Key not found: " . $key);
    }
    let $ttl = phprs_redis_ttl($key);
    let $data = json_encode(["key"=>$key, "value"=>$value, "ttl"=>$ttl]);
    return api_response(200, "OK", $data);
}

// POST /api/redis/del?key=xxx — delete a key
function redis_del(string $params): string {
    let $key = route_param($params, "key");
    if ($key == "") {
        return api_error(400, "Missing key parameter");
    }
    let $exists = phprs_redis_exists($key);
    if ($exists == 0) {
        return api_error(404, "Key not found: " . $key);
    }
    let $result = phprs_redis_del($key);
    let $data = json_encode(["message"=>"Deleted", "key"=>$key, "deleted"=>$result]);
    return api_response(200, "OK", $data);
}

// GET /api/redis/keys — list all keys matching pattern
function redis_keys(): string {
    let $keys_json = phprs_redis_keys("*");
    let $ping = phprs_redis_ping();
    let $data = json_encode([
        "ping"=>$ping,
        "keys"=>$keys_json
    ]);
    return api_response(200, "OK", $data);
}

// POST /api/redis/incr?key=xxx — increment a numeric key
function redis_incr(string $params): string {
    let $key = route_param($params, "key");
    if ($key == "") {
        return api_error(400, "Missing key parameter");
    }
    let $val = phprs_redis_incr($key);
    let $data = json_encode(["key"=>$key, "value"=>$val]);
    return api_response(200, "OK", $data);
}

// POST /api/redis/hset — set a hash field
// Body (JSON): key, field, value
function redis_hset(string $raw): string {
    let $body = phprs_http_body($raw);
    let $key = phprs_json_get_string($body, "key");
    let $field = phprs_json_get_string($body, "field");
    let $value = phprs_json_get_string($body, "value");
    if ($key == "" || $field == "") {
        return api_error(400, "Missing key or field");
    }
    let $result = phprs_redis_hset($key, $field, $value);
    let $data = json_encode(["key"=>$key, "field"=>$field, "value"=>$value, "result"=>$result]);
    return api_response(200, "OK", $data);
}

// GET /api/redis/hgetall?key=xxx — get all hash fields
function redis_hgetall(string $params): string {
    let $key = route_param($params, "key");
    if ($key == "") {
        return api_error(400, "Missing key parameter");
    }
    let $result = phprs_redis_hgetall($key);
    let $data = json_encode(["key"=>$key, "fields"=>$result]);
    return api_response(200, "OK", $data);
}

// POST /api/redis/lpush — push to list
// Body (JSON): key, value
function redis_lpush(string $raw): string {
    let $body = phprs_http_body($raw);
    let $key = phprs_json_get_string($body, "key");
    let $value = phprs_json_get_string($body, "value");
    if ($key == "") {
        return api_error(400, "Missing key");
    }
    let $result = phprs_redis_lpush($key, $value);
    let $data = json_encode(["key"=>$key, "value"=>$value, "length"=>$result]);
    return api_response(200, "OK", $data);
}

// GET /api/redis/lrange?key=xxx&start=0&stop=-1 — get list range
function redis_lrange(string $params): string {
    let $key = route_param($params, "key");
    if ($key == "") {
        return api_error(400, "Missing key parameter");
    }
    let $items = phprs_redis_lrange($key, 0, -1);
    let $data = json_encode(["key"=>$key, "items"=>$items]);
    return api_response(200, "OK", $data);
}

// GET /api/redis/cmd?cmd=SET+mykey+myvalue — raw command (space-separated)
function redis_cmd(string $params): string {
    let $cmd = route_param($params, "cmd");
    if ($cmd == "") {
        return api_error(400, "Missing cmd parameter");
    }
    let $result = phprs_redis_cmd($cmd);
    let $data = json_encode(["command"=>$cmd, "result"=>$result]);
    return api_response(200, "OK", $data);
}
?>"##;

// ---- WebSocket Controller ----
const WS_CONTROLLER: &str = r##"<?phprs
// WebSocket Controller — WebSocket chat & echo server examples.
// Requires: include "../system/websocket.phprs"
//
// The PHPRS MVC server on port 8080 also handles WebSocket upgrades on
// the same port. Clients connect to ws://localhost:8080/ws/chat and
// negotiate a WebSocket upgrade.

include "../system/websocket.phprs";

// GET /api/ws/info — WebSocket endpoint information page
function ws_info_page(): string {
    let $body = "<div class=\"card\">
        <h2>WebSocket Examples</h2>
        <p>PHPRS supports WebSocket upgrades on the same HTTP server port (8080).</p>

        <h3>Available Endpoints</h3>
        <table style=\"width:100%;border-collapse:collapse;\">
        <tr style=\"background:#f0f0f0;\">
            <th style=\"padding:8px;text-align:left;border:1px solid #ddd;\">Path</th>
            <th style=\"padding:8px;text-align:left;border:1px solid #ddd;\">Description</th>
        </tr>
        <tr>
            <td style=\"padding:8px;border:1px solid #ddd;\"><code>ws://localhost:8080/ws/chat</code></td>
            <td style=\"padding:8px;border:1px solid #ddd;\">Chat room — broadcast messages to all connected clients</td>
        </tr>
        <tr>
            <td style=\"padding:8px;border:1px solid #ddd;\"><code>ws://localhost:8080/ws/echo</code></td>
            <td style=\"padding:8px;border:1px solid #ddd;\">Echo server — sends back whatever you send</td>
        </tr>
        </table>

        <h3>Testing with a Browser Console</h3>
        <pre>// Connect to chat
let ws = new WebSocket('ws://localhost:8080/ws/chat');
ws.onmessage = (e) => console.log('Chat:', e.data);
ws.onopen = () => ws.send('Hello everyone!');

// Connect to echo
let ws2 = new WebSocket('ws://localhost:8080/ws/echo');
ws2.onmessage = (e) => console.log('Echo:', e.data);
ws2.onopen = () => ws2.send('ping');</pre>

        <h3>Testing with CLI (websocat)</h3>
        <pre>websocat ws://localhost:8080/ws/chat
websocat ws://localhost:8080/ws/echo</pre>

        <h3>WebSocket Frame Format</h3>
        <p>Frames are read as <code>opcode:payload</code> strings:</p>
        <ul>
            <li><strong>Opcode 1</strong> — Text frame</li>
            <li><strong>Opcode 8</strong> — Close frame</li>
            <li><strong>Opcode 9</strong> — Ping (server auto-responds with pong)</li>
            <li><strong>Opcode 10</strong> — Pong</li>
        </ul>
    </div>";

    return render_page("WebSocket Examples", $body);
}

// ---- WebSocket Handlers ----
// These are called from app.phprs when a WebSocket upgrade is detected.
// The main loop in app_main() checks for ws:// paths and delegates here.

// Handle a WebSocket chat connection.
// Reads text frames and broadcasts to all other connected clients.
// client_fd: the accepted socket file descriptor
// path: the requested WebSocket path (e.g., "/ws/chat")
// Returns 1 if this was a WebSocket connection (handled), 0 otherwise.
function ws_handle_chat(int $client_fd, string $path): int {
    // Register this connection in the manager
    let $slot = phprs_ws_register($client_fd, "chat");

    // Broadcast loop — read frames until disconnect
    for (let mut $running = 1; $running == 1; ) {
        let $frame = ws_read($client_fd);
        if ($frame == "" || phprs_str_starts_with($frame, "-1:") == 1) {
            break;
        }

        let $opcode = ws_frame_opcode($frame);

        // Handle close
        if ($opcode == 8) {
            ws_disconnect($client_fd);
            break;
        }

        // Handle pong — update heartbeat timestamp
        if ($opcode == 10) {
            phprs_ws_update_pong($client_fd);
        }

        // Handle text message — broadcast to all in "chat" room
        if ($opcode == 1) {
            let $payload = ws_frame_payload($frame);
            let $msg = "[chat] " . $path . " says: " . $payload;
            phprs_ws_broadcast("chat", $msg, $client_fd);
            // Also echo back to sender
            ws_send_text($client_fd, $msg);
        }
    }

    // Unregister on disconnect
    phprs_ws_unregister($client_fd);
    return 1;
}

// Handle a WebSocket echo connection.
// Simply echoes back every text frame received.
function ws_handle_echo(int $client_fd, string $path): int {
    // Register this connection in the manager
    let $slot = phprs_ws_register($client_fd, "echo");

    for (let mut $running = 1; $running == 1; ) {
        let $frame = ws_read($client_fd);
        if ($frame == "" || phprs_str_starts_with($frame, "-1:") == 1) {
            break;
        }

        let $opcode = ws_frame_opcode($frame);

        if ($opcode == 8) {
            ws_disconnect($client_fd);
            break;
        }

        // Handle pong — update heartbeat timestamp
        if ($opcode == 10) {
            phprs_ws_update_pong($client_fd);
        }

        if ($opcode == 1) {
            let $payload = ws_frame_payload($frame);
            let $reply = "[echo] " . $payload;
            ws_send_text($client_fd, $reply);
        }
    }

    // Unregister on disconnect
    phprs_ws_unregister($client_fd);
    return 1;
}
?>"##;

// ---- Layout / View Helpers ----
const LAYOUT_VIEW: &str = r##"<?phprs
// View helpers — layout rendering, API responses.

// Standard API JSON response: {"code":200,"msg":"ok","data":...,"ip":"...","ts":...}
function api_response(int $code, string $msg, string $data): string {
    let $ts = time();
    let $json = "{\"code\":" . $code . ",\"msg\":\"" . $msg . "\",\"data\":" . $data . ",\"ip\":\"127.0.0.1\",\"ts\":" . $ts . "}";
    return phprs_http_response($code, "application/json; charset=utf-8", $json);
}

// Error API response shortcut
function api_error(int $code, string $msg): string {
    return api_response($code, $msg, "[]");
}

function render_page(string $title, string $body): string {
    let $html = "<!DOCTYPE html>
<html>
<head>
    <meta charset=\"utf-8\">
    <title>{{title}}</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif; background: #f5f5f5; color: #333; line-height: 1.6; }
        .container { max-width: 800px; margin: 0 auto; padding: 1.5rem; }
        header { background: #4F46E5; color: #fff; padding: 1rem 1.5rem; }
        header h1 { font-size: 1.5rem; }
        header a { color: #fff; text-decoration: none; }
        .card { background: #fff; border-radius: 8px; padding: 1.5rem; margin: 1rem 0; box-shadow: 0 1px 3px rgba(0,0,0,0.1); }
        h2 { margin-bottom: 0.5rem; }
        h3 { margin: 1rem 0 0.5rem; }
        ul { padding-left: 1.5rem; }
        li { margin: 0.25rem 0; }
        a { color: #4F46E5; }
        pre { background: #f0f0f0; padding: 0.75rem; border-radius: 4px; font-size: 0.9em; overflow-x: auto; }
        footer { text-align: center; color: #999; font-size: 0.85rem; margin-top: 2rem; padding: 1rem; }
    </style>
</head>
<body>
<header>
    <div class=\"container\">
        <h1><a href=\"/\">PHPRS MVC</a></h1>
    </div>
</header>
<div class=\"container\">
{{body}}
</div>
<footer>
    <p>Powered by PHPRS — compiled to native code</p>
</footer>
</body>
</html>";

    let $result = phprs_str_replace($html, "{{title}}", view_escape($title));
    $result = phprs_str_replace($result, "{{body}}", $body);
    return phprs_http_response(200, "text/html; charset=utf-8", $result);
}

function response_404(): string {
    let $html = "<!DOCTYPE html>
<html>
<head><meta charset=\"utf-8\"><title>404 — Not Found</title>
<style>body{{font-family:sans-serif;max-width:600px;margin:3rem auto;text-align:center;}}</style>
</head>
<body><h1>404</h1><p>Page not found.</p><p><a href=\"/\">Home</a></p></body></html>";
    return phprs_http_response(404, "text/html; charset=utf-8", $html);
}

function response_500(string $msg): string {
    let $html = "<!DOCTYPE html>
<html>
<head><meta charset=\"utf-8\"><title>500 — Error</title>
<style>body{{font-family:sans-serif;max-width:600px;margin:3rem auto;text-align:center;}}</style>
</head>
<body><h1>500</h1><p>Something went wrong.</p><p><a href=\"/\">Home</a></p></body></html>";
    return phprs_http_response(500, "text/html; charset=utf-8", $html);
}
?>"##;

// ---- Database Configuration (Webman-style) ----
const DATABASE_CONFIG: &str = r##"<?phprs
// Database Configuration — Webman-style multi-connection config.
// Usage:
//   include "../config/database.phprs";
//   let $mysql  = db_mysql();       // MySQL connection config (JSON)
//   let $pgsql  = db_pgsql();       // PostgreSQL connection config (JSON)
//   let $mysql_pool = db_mysql_pool(); // Connection pool config (JSON)
//
// Reference: webman/config/database.php (illuminate/database)

// Default connection name
function db_default_connection(): string {
    return "mysql";
}

// MySQL connection
function db_mysql(): string {
    return json_encode([
        "driver"=>"mysql",
        "host"=>"127.0.0.1",
        "port"=>"3306",
        "database"=>"test",
        "username"=>"root",
        "password"=>"",
        "charset"=>"utf8mb4",
        "collation"=>"utf8mb4_unicode_ci",
        "prefix"=>"",
        "strict"=>"true",
        "engine"=>""
    ]);
}

// PostgreSQL connection example
function db_pgsql(): string {
    return json_encode([
        "driver"=>"pgsql",
        "host"=>"127.0.0.1",
        "port"=>"5432",
        "database"=>"test",
        "username"=>"postgres",
        "password"=>"",
        "charset"=>"utf8",
        "prefix"=>"",
        "schema"=>"public"
    ]);
}

// SQLite connection example
function db_sqlite(): string {
    return json_encode([
        "driver"=>"sqlite",
        "database"=>"data/database.sqlite",
        "prefix"=>""
    ]);
}

// Connection pool (Webman's pool config)
function db_mysql_pool(): string {
    return json_encode([
        "max_connections"=>"5",
        "min_connections"=>"1",
        "wait_timeout"=>"3",
        "idle_timeout"=>"60",
        "heartbeat_interval"=>"50"
    ]);
}
?>"##;

// ---- Redis Configuration (Webman-style) ----
const REDIS_CONFIG: &str = r##"<?phprs
// Redis Configuration — Webman-style config with connection pool support.
// Usage:
//   include "config/redis.phprs";
//   let $redis = redis_default();       // Default connection config (JSON)
//   let $cache = redis_cache();         // Cache connection config (JSON)
//   let $pool  = redis_pool_default();  // Connection pool config (JSON)
//
// Reference: webman/config/redis.php (illuminate/redis)

// Default Redis connection
function redis_default(): string {
    return json_encode([
        "host"=>"127.0.0.1",
        "password"=>"",
        "port"=>"6379",
        "database"=>"0"
    ]);
}

// Cache-specific Redis connection (separate DB, with key prefix)
function redis_cache(): string {
    return json_encode([
        "host"=>"127.0.0.1",
        "password"=>"",
        "port"=>"6379",
        "database"=>"1",
        "prefix"=>"cache_"
    ]);
}

// Session-specific Redis connection
function redis_session(): string {
    return json_encode([
        "host"=>"127.0.0.1",
        "password"=>"",
        "port"=>"6379",
        "database"=>"2",
        "prefix"=>"sess_"
    ]);
}

// Queue-specific Redis connection
function redis_queue(): string {
    return json_encode([
        "host"=>"127.0.0.1",
        "password"=>"",
        "port"=>"6379",
        "database"=>"3",
        "prefix"=>"queue_"
    ]);
}

// Default connection pool config
function redis_pool_default(): string {
    return json_encode([
        "max_connections"=>"10",
        "min_connections"=>"1",
        "wait_timeout"=>"3",
        "idle_timeout"=>"50",
        "heartbeat_interval"=>"50"
    ]);
}
?>"##;
