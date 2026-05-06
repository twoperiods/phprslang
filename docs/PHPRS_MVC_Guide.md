# PHPRS MVC Framework Guide

PHPRS 是一种将 PHP 简洁语法与 Rust 级性能相结合的编程语言。使用 PHPRS，你可以用熟悉的 PHP 风格语法编写 Web 应用，并将其编译为单一原生二进制文件。

## 目录

1. [快速开始](#快速开始)
2. [PHPRS 语言基础](#phprs-语言基础)
3. [MVC 框架架构](#mvc-框架架构)
4. [创建你的第一个应用](#创建你的第一个应用)
5. [路由与请求处理：GET/POST 参数完整示例](#路由与请求处理getpost-参数完整示例)
6. [高级路由：带类型约束的查询参数匹配](#高级路由带类型约束的查询参数匹配)
7. [简化路由：一行定义所有路由](#简化路由一行定义所有路由)
8. [phprs_request_parse(): 统一请求数据获取](#phprs_request_parse-统一请求数据获取)
9. [Blog 示例应用](#blog-示例应用)
10. [WebSocket 支持](#websocket-支持)
11. [安全特性](#安全特性)
12. [API 参考](#api-参考)
12. [部署](#部署)
13. [故障排除](#故障排除)

---

## 快速开始

### 构建编译器

```bash
cd phprs
cargo build --release
```

### 第一个程序

创建文件 `hello.phprs`：

```php
<?phprs
echo "Hello, PHPRS!\n";
?>
```

编译并运行：

```bash
phprs build hello.phprs -o hello.exe
./hello.exe
# 输出: Hello, PHPRS!
```

---

## PHPRS 语言基础

### 语法概览

PHPRS 语法基于 PHP，但添加了类型标注和现代化的特性。

#### 变量

变量以 `$` 为前缀，使用 `let` 声明：

```php
let $name = "World";
let $age = 25;
let $price = 9.99;
let $active = true;
```

**可变变量** 使用 `let mut` 声明：

```php
let mut $counter = 0;
$counter = $counter + 1;
```

#### 类型

PHPRS 支持静态类型标注：

| 类型 | 说明 | C 类型映射 |
|------|------|-----------|
| `int` | 64 位整数 | `int64_t` |
| `float` | 双精度浮点数 | `double` |
| `string` | C 字符串 | `const char*` |
| `bool` | 布尔值 | `bool` |
| `void` | 无返回值 | `void` |

类型标注语法：

```php
function add(int $a, int $b): int {
    return $a + $b;
}
```

数组类型的参数需要标注：

```php
function handler([string] $items): void {
    let $first = $items[0];
}
```

#### 函数

```php
// 标准函数
function greet(string $name): string {
    return "Hello, " . $name;
}

// 外部函数声明（C FFI）
extern function phprs_server_new(int $port): int;
```

函数调用：

```php
let $result = greet("PHPRS");
echo $result . "\n";
```

#### 控制流

**If/Else**：

```php
if ($score >= 90) {
    echo "优秀\n";
} else if ($score >= 60) {
    echo "及格\n";
} else {
    echo "不及格\n";
}
```

**For 循环**：

```php
for (let mut $i = 0; $i < 10; $i = $i + 1) {
    echo $i . "\n";
}
```

**Foreach 遍历数组**：

```php
let $items = ["apple", "banana", "cherry"];
foreach ($items as $item) {
    echo $item . "\n";
}
```

使用 `=>` 同时获取键和值：

```php
let $items = ["apple", "banana", "cherry"];
foreach ($items as $index => $item) {
    echo $index . ": " . $item . "\n";
}
// 输出:
// 0: apple
// 1: banana
// 2: cherry
```

> **注意：** 键值对数组字面量（`["key" => "value"]` / Dict）目前仅解释器支持，C 编译后端暂未实现。如需在编译模式下使用键值映射，可使用 JSON 字符串 + `phprs_json_get_string()` 代替。```

**Match 表达式**：

```php
let $grade = match ($score) {
    90..=100 => "优秀",
    60..=89 => "及格",
    0..=59 => "不及格",
    _ => "无效",
};
```

#### 字符串操作

PHPRS 使用 `.` 进行字符串连接：

```php
let $greeting = "Hello, " . $name . "!";
```

复合赋值：

```php
let mut $html = "<h1>Title</h1>";
$html .= "<p>Content</p>";
```

#### include 预处理

在编译时通过文本级别的 include 指令复用代码：

```php
<?phprs
include "framework/runtime.phprs";
include "framework/response.phprs";

// 现在可以使用包含文件中定义的函数
let $resp = response_html(200, "<h1>Hello</h1>");
?>
```

---

## MVC 框架架构

### 架构图

```
┌─────────────────────────────────────────────────┐
│  用户应用 (app.phprs)                            │
│  ┌──────────┐ ┌──────────┐ ┌──────────────────┐ │
│  │ 路由定义  │ │ 控制器    │ │ 视图模板          │ │
│  └──────────┘ └──────────┘ └──────────────────┘ │
├─────────────────────────────────────────────────┤
│  MVC 框架层 (.phprs)                             │
│  ┌──────────┐ ┌──────────┐ ┌──────────────────┐ │
│  │ Router   │ │ Request  │ │ Response / View  │ │
│  └──────────┘ └──────────┘ └──────────────────┘ │
├─────────────────────────────────────────────────┤
│  FFI 层 (extern function 声明)                   │
│  ┌──────────────────────────────────────────────┐ │
│  │  runtime.phprs — 23 个 C 运行时函数声明      │ │
│  └──────────────────────────────────────────────┘ │
├─────────────────────────────────────────────────┤
│  C 运行时 (phprs_runtime.c ~400 行)              │
│  ┌──────┐ ┌──────┐ ┌──────┐ ┌────────────────┐ │
│  │ TCP  │ │ HTTP │ │ JSON │ │ String Helpers │ │
│  │ 套接字│ │ 解析  │ │ 辅助 │ │ 字符串工具     │ │
│  └──────┘ └──────┘ └──────┘ └────────────────┘ │
└─────────────────────────────────────────────────┘
```

### 框架文件说明

| 文件 | 路径 | 说明 |
|------|------|------|
| `runtime.phprs` | `framework/` | FFI 声明 — 36 个 C 运行时函数的外部声明 |
| `router.phprs` | `framework/` | URL 路由器 — 路径段模式匹配与参数提取 |
| `router_advanced.phprs` | `framework/` | 高级路由器 — 查询参数类型约束 `?key={int}` |
| `router_simple.phprs` | `framework/` | 简化路由器 — 一行字符串定义所有路由 |
| `request.phprs` | `framework/` | 请求解析 — 提取 method、path、body + `request_param()` |
| `response.phprs` | `framework/` | 响应构建器 — HTML、JSON、404/500 |
| `view.phprs` | `framework/` | 模板引擎 — `{{placeholder}}` 变量替换 |
| `websocket.phprs` | `framework/` | WebSocket 辅助 — accept、read、send、close |
| `http_client.phprs` | `framework/` | HTTP 客户端 — GET、POST、URL 解析 |
| `app.phprs` | `framework/` | 应用启动器 — 单线程事件循环 + 路由分发 |
| `app_threaded.phprs` | `framework/` | 应用启动器 — 多线程 accept + 工作线程池 |

### 请求生命周期

```
客户端请求
  │
  ▼
TCP Socket (phprs_server_accept)
  │
  ▼
读取 HTTP 数据 (phprs_socket_read)
  │
  ▼
解析 HTTP (phprs_http_method, phprs_http_path)
  │
  ▼
URL 路由匹配 (match 表达式)
  │
  ├─ "/" ──────► home_page()
  ├─ "/about" ─► about_page()
  ├─ "/api/*" ─► api_handler()
  └─ _ ────────► response_404()
  │
  ▼
控制器处理 (controller 函数)
  │
  ▼
构建 HTTP 响应 (phprs_http_response)
  │
  ▼
发送响应 (phprs_socket_write)
  │
  ▼
关闭连接 (phprs_socket_close)
```

---

## 创建你的第一个应用

以下是一个完整的 MVC 应用，编译为单一原生二进制文件。

### 步骤 1：创建应用文件 `myapp.phprs`

```php
<?phprs
// 引入框架运行时（FFI 声明）
include "framework/runtime.phprs";

// ---- 响应辅助函数 ----
function response_html(int $status, string $html): string {
    return phprs_http_response($status, "text/html; charset=utf-8", $html);
}

function response_json(int $status, string $json): string {
    return phprs_http_response($status, "application/json", $json);
}

function response_404(): string {
    let $body = "<h1>404 Not Found</h1>";
    return phprs_http_response(404, "text/html; charset=utf-8", $body);
}

// ---- 模板渲染 ----
function render(string $tmpl, string $k1, string $v1): string {
    let $ph = "{{" . $k1 . "}}";
    return phprs_str_replace($tmpl, $ph, $v1);
}

function render2(string $tmpl, string $k1, string $v1, string $k2, string $v2): string {
    let $tmp = render($tmpl, $k1, $v1);
    return render($tmp, $k2, $v2);
}

// ---- 控制器 ----
function home_page(): string {
    let $title = "我的应用";
    let $message = "Hello from PHPRS MVC!";

    let $html = "<!DOCTYPE html>
<html>
<head><title>{{title}}</title></head>
<body>
    <h1>{{title}}</h1>
    <p>{{message}}</p>
</body>
</html>";

    return response_html(200, render2($html, "title", $title, "message", $message));
}

function api_hello(): string {
    return response_json(200, "{\"message\": \"Hello, World!\"}");
}

// ---- 路由分发 ----
function app_dispatch(string $path): string {
    return match ($path) {
        "/" => home_page(),
        "/api/hello" => api_hello(),
        _ => response_404(),
    };
}

// ---- 主服务器循环 ----
let $server = phprs_server_new(8080);

if ($server < 0) {
    echo "无法启动服务器\n";
    return;
}

echo "=== 服务器已启动 ===\n";
echo "访问: http://localhost:8080\n";

for (let mut $running = 1; $running == 1; ) {
    let $client = phprs_server_accept($server);

    if ($client >= 0) {
        let $raw = phprs_socket_read($client, 65536);
        let $path = phprs_http_path($raw);

        let $response = app_dispatch($path);
        phprs_socket_write($client, $response);
        phprs_socket_close($client);
    }
}

phprs_socket_close($server);
?>
```

### 步骤 2：编译

```bash
phprs build myapp.phprs -o myapp.exe
```

### 步骤 3：运行

```bash
./myapp.exe
```

### 步骤 4：测试

```bash
curl http://localhost:8080/          # 返回 HTML 页面
curl http://localhost:8080/api/hello # 返回 JSON
curl http://localhost:8080/other     # 返回 404
```

---

## 路由与请求处理：GET/POST 参数完整示例

以下展示 MVC 框架中所有常见的请求处理模式，均可在编译模式（`phprs build`）下直接使用。

### 准备工作：引入框架文件

```php
<?phprs
include "framework/runtime.phprs";
include "framework/router.phprs";
include "framework/request.phprs";
include "framework/response.phprs";
```

### 1. GET 请求 — URL 查询参数

URL 查询参数（`?key=value`）包含在 `phprs_http_path()` 返回的 path 中，需要自行解析。

#### 查询参数解析函数

```php
// 从 path 中提取查询字符串，返回 key1=val1&key2=val2 格式
function query_parse(string $path): string {
    if (phprs_str_contains($path, "?") == 0) {
        return "";
    }
    // 提取 ? 之后的部分
    let $parts = phprs_str_split($path, "?", 1);
    return $parts;
}

// 从查询参数字符串中获取指定 key 的值
function query_get(string $query_string, string $key): string {
    if ($query_string == "") {
        return "";
    }
    let $search = $key . "=";
    let mut $i = 0;
    for (let mut $i = 0; $i < 100; $i = $i + 1) {
        let $segment = phprs_str_split($query_string, "&", $i);
        if ($segment == "") {
            $i = 200;
        } else {
            if (phprs_str_starts_with($segment, $search)) {
                let $val = phprs_str_replace($segment, $search, "");
                return phprs_url_decode($val);
            }
        }
    }
    return "";
}
```

#### 控制器示例：处理 `/search?q=keyword&page=1`

```php
function search_page(string $raw, string $path): string {
    let $query = query_parse($path);
    let $keyword = query_get($query, "q");
    let $page = query_get($query, "page");

    if ($keyword == "") {
        return response_json(400, "{\"error\": \"missing 'q' parameter\"}");
    }
    if ($page == "") {
        $page = "1"; // 默认值
    }

    let $json = "{\"keyword\":\"" . $keyword . "\",\"page\":" . $page . "}";
    return response_json(200, $json);
}
```

#### 路由分发

```php
function app_dispatch(string $handler, string $raw, string $path): string {
    // 去掉查询字符串，只保留路径部分用于路由匹配
    let $route_path = $path;
    if (phprs_str_contains($path, "?") == 1) {
        $route_path = phprs_str_split($path, "?", 0);
    }

    if ($handler == "search") {
        return search_page($raw, $path);
    }
    return response_404();
}
```

测试：
```bash
curl "http://localhost:8080/search?q=hello&page=2"
# → {"keyword":"hello","page":2}

curl "http://localhost:8080/search?q=你好"
# → {"keyword":"你好","page":1}
```

### 2. POST 请求 — application/x-www-form-urlencoded

表单提交的数据存放在 HTTP body 中，格式为 `key1=val1&key2=val2`。

#### 控制器示例：处理登录表单

```php
function login_handler(string $raw, string $path): string {
    let $method = phprs_http_method($raw);
    let $body = phprs_http_body($raw);

    // 仅接受 POST
    if ($method != "POST") {
        return response_json(405, "{\"error\": \"Method Not Allowed\"}");
    }

    // 解析表单数据 (application/x-www-form-urlencoded)
    let $username = request_param($body, "username");
    let $password = request_param($body, "password");

    if ($username == "" || $password == "") {
        return response_json(400, "{\"error\": \"username and password required\"}");
    }

    // 业务逻辑：验证用户名密码
    if ($username == "admin" && $password == "123456") {
        return response_json(200, "{\"message\": \"Login success\",\"user\":\"" . $username . "\"}");
    }

    return response_json(401, "{\"error\": \"Invalid credentials\"}");
}
```

测试：
```bash
# 表单格式 POST
curl -X POST http://localhost:8080/login \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "username=admin&password=123456"
# → {"message":"Login success","user":"admin"}

curl -X POST http://localhost:8080/login \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "username=admin&password=wrong"
# → {"error":"Invalid credentials"}
```

### 3. POST 请求 — Raw Body (application/json)

直接读取原始 body 内容，适用于 JSON、XML、纯文本等格式。

#### 控制器示例：处理 JSON 原始请求体

```php
function api_create_user(string $raw, string $path): string {
    let $method = phprs_http_method($raw);
    let $body = phprs_http_body($raw);

    if ($method != "POST") {
        return response_json(405, "{\"error\": \"Method Not Allowed\"}");
    }

    if ($body == "") {
        return response_json(400, "{\"error\": \"Empty body\"}");
    }

    // 从 JSON body 中提取字段
    let $name = phprs_json_get_string($body, "name");
    let $email = phprs_json_get_string($body, "email");
    let $age = phprs_json_get_int($body, "age");

    if ($name == "" || $email == "") {
        return response_json(400, "{\"error\": \"name and email are required\"}");
    }

    // 构造响应 JSON
    let $result = "{\"status\":\"created\",\"name\":\"" . $name . "\",\"email\":\"" . $email . "\",\"age\":" . $age . "}";
    return response_json(201, $result);
}
```

测试：
```bash
# JSON raw body POST
curl -X POST http://localhost:8080/api/users \
  -H "Content-Type: application/json" \
  -d '{"name":"张三","email":"zhangsan@example.com","age":28}'
# → {"status":"created","name":"张三","email":"zhangsan@example.com","age":28}

# 空 body 的错误处理
curl -X POST http://localhost:8080/api/users \
  -H "Content-Type: application/json" \
  -d ""
# → {"error":"Empty body"}
```

### 4. GET 请求 — 路径参数（RESTful 风格）

使用 `router.phprs` 的 `{param}` 模式匹配路径中的动态部分。

#### 控制器示例：处理 `/users/{id}` 和 `/users/{id}/posts/{post_id}`

```php
function user_detail(string $raw, string $path): string {
    let $pattern = "/users/{id}";
    let $params = router_extract_params($path, $pattern);
    let $user_id = request_param($params, "id");

    if ($user_id == "") {
        return response_json(400, "{\"error\": \"missing user id\"}");
    }

    // 模拟从数据库获取用户
    let $json = "{\"id\":" . $user_id . ",\"name\":\"User " . $user_id . "\",\"email\":\"user" . $user_id . "@example.com\"}";
    return response_json(200, $json);
}

function user_post_detail(string $raw, string $path): string {
    let $pattern = "/users/{id}/posts/{post_id}";
    let $params = router_extract_params($path, $pattern);
    let $user_id = request_param($params, "id");
    let $post_id = request_param($params, "post_id");

    let $json = "{\"user_id\":" . $user_id . ",\"post_id\":" . $post_id . ",\"title\":\"Post " . $post_id . " by User " . $user_id . "\"}";
    return response_json(200, $json);
}
```

#### 路由分发（含路径参数匹配）

```php
function app_dispatch(string $handler, string $raw, string $path): string {
    if ($handler == "login") {
        return login_handler($raw, $path);
    }
    if ($handler == "create_user") {
        return api_create_user($raw, $path);
    }
    if ($handler == "search") {
        return search_page($raw, $path);
    }
    if ($handler == "user_detail") {
        return user_detail($raw, $path);
    }
    if ($handler == "user_post") {
        return user_post_detail($raw, $path);
    }
    return response_404();
}
```

测试：
```bash
curl http://localhost:8080/users/42
# → {"id":42,"name":"User 42","email":"user42@example.com"}

curl http://localhost:8080/users/42/posts/7
# → {"user_id":42,"post_id":7,"title":"Post 7 by User 42"}
```

### 5. 完整应用示例：整合所有模式

以下是一个完整的 `api_server.phprs`，整合了上述所有请求处理模式：

```php
<?phprs
include "framework/runtime.phprs";
include "framework/router.phprs";
include "framework/request.phprs";
include "framework/response.phprs";

// ======== 查询参数解析 ========
function query_parse(string $path): string {
    if (phprs_str_contains($path, "?") == 0) {
        return "";
    }
    return phprs_str_split($path, "?", 1);
}

function query_get(string $query_string, string $key): string {
    if ($query_string == "") { return ""; }
    let $search = $key . "=";
    let mut $i = 0;
    for (let mut $i = 0; $i < 100; $i = $i + 1) {
        let $segment = phprs_str_split($query_string, "&", $i);
        if ($segment == "") { $i = 200; }
        else {
            if (phprs_str_starts_with($segment, $search)) {
                return phprs_url_decode(phprs_str_replace($segment, $search, ""));
            }
        }
    }
    return "";
}

// ======== 控制器 ========

// GET /search?q=keyword&page=1
function search_handler(string $raw, string $path): string {
    let $query = query_parse($path);
    let $q = query_get($query, "q");
    let $page = query_get($query, "page");
    if ($page == "") { $page = "1"; }
    if ($q == "") {
        return response_json(400, "{\"error\":\"missing 'q' parameter\"}");
    }
    return response_json(200, "{\"q\":\"" . $q . "\",\"page\":" . $page . "}");
}

// POST /login  (form-urlencoded)
function login_handler(string $raw, string $path): string {
    if (phprs_http_method($raw) != "POST") {
        return response_json(405, "{\"error\":\"Method Not Allowed\"}");
    }
    let $body = phprs_http_body($raw);
    let $username = request_param($body, "username");
    let $password = request_param($body, "password");
    if ($username == "" || $password == "") {
        return response_json(400, "{\"error\":\"username and password required\"}");
    }
    if ($username == "admin" && $password == "123456") {
        return response_json(200, "{\"message\":\"Login success\"}");
    }
    return response_json(401, "{\"error\":\"Invalid credentials\"}");
}

// POST /api/users  (JSON raw body)
function create_user_handler(string $raw, string $path): string {
    if (phprs_http_method($raw) != "POST") {
        return response_json(405, "{\"error\":\"Method Not Allowed\"}");
    }
    let $body = phprs_http_body($raw);
    if ($body == "") {
        return response_json(400, "{\"error\":\"Empty body\"}");
    }
    let $name = phprs_json_get_string($body, "name");
    let $email = phprs_json_get_string($body, "email");
    if ($name == "" || $email == "") {
        return response_json(400, "{\"error\":\"name and email are required\"}");
    }
    return response_json(201, "{\"status\":\"created\",\"name\":\"" . $name . "\"}");
}

// GET /users/{id}
function user_detail_handler(string $raw, string $path): string {
    let $params = router_extract_params($path, "/users/{id}");
    let $user_id = request_param($params, "id");
    if ($user_id == "") {
        return response_json(400, "{\"error\":\"missing user id\"}");
    }
    return response_json(200, "{\"id\":" . $user_id . ",\"name\":\"User " . $user_id . "\"}");
}

// ======== 路由分发 ========
function app_dispatch(string $handler, string $raw, string $path): string {
    if ($handler == "search")    { return search_handler($raw, $path); }
    if ($handler == "login")     { return login_handler($raw, $path); }
    if ($handler == "create_user") { return create_user_handler($raw, $path); }
    if ($handler == "user_detail") { return user_detail_handler($raw, $path); }
    return response_404();
}

// ======== 主服务器 ========
function app_main(): void {
    // 路由表：三个平行数组
    let $methods  = ["GET",  "POST", "POST",       "GET"];
    let $patterns = ["/search", "/login", "/api/users", "/users/{id}"];
    let $handlers = ["search", "login", "create_user", "user_detail"];

    app_run(8080, $methods, $patterns, $handlers);
}

app_main();
?>
```

编译并运行：

```bash
phprs build api_server.phprs -o api_server.exe
./api_server.exe
```

全部测试命令：

```bash
# 1. GET 查询参数
curl "http://localhost:8080/search?q=hello&page=2"

# 2. GET 路径参数 (RESTful)
curl http://localhost:8080/users/42

# 3. POST 表单数据 (urlencoded)
curl -X POST http://localhost:8080/login \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "username=admin&password=123456"

# 4. POST 原始 JSON body
curl -X POST http://localhost:8080/api/users \
  -H "Content-Type: application/json" \
  -d '{"name":"张三","email":"zhangsan@example.com"}'

# 5. 未匹配路由 → 404
curl http://localhost:8080/notfound
```

### 请求处理模式速查表

| 场景 | 数据来源 | 解析方式 | 示例 |
|------|----------|----------|------|
| URL 查询参数 `?a=1&b=2` | `phprs_http_path()` | 手动解析 `?` 后的字符串，用 `&` 分割 | `/search?q=hello&page=1` |
| 路径参数 `/{id}` | `phprs_http_path()` | `router_extract_params()` + `request_param()` | `/users/42` |
| POST 表单 `urlencoded` | `phprs_http_body()` | `request_param(body, key)` | `username=admin&password=123` |
| POST 原始 JSON body | `phprs_http_body()` | `phprs_json_get_string/int(body, key)` | `{"name":"张三"}` |
| HTTP 方法判断 | `phprs_http_method()` | 直接比较字符串 `"GET"` `"POST"` | — |

---

## 高级路由：带类型约束的查询参数匹配

`framework/router_advanced.phprs` 在基础 router 之上增加了**查询参数的类型约束**，使得路由定义可以直接写成 `/api/user?user_id={int}`，并自动完成类型校验和参数提取。

### 引入

```php
<?phprs
include "framework/runtime.phprs";
include "framework/router.phprs";           // 基础: router_path_matches, router_extract_params
include "framework/router_advanced.phprs";   // 高级: router_match_advanced, router_extract_params_advanced
include "framework/request.phprs";           // request_param
include "framework/response.phprs";
```

### 核心 API

| 函数 | 签名 | 说明 |
|------|------|------|
| `router_match_advanced` | `(string $url, string $pattern): int` | 完整匹配 URL（含 query string）与模式，含类型校验 |
| `router_extract_params_advanced` | `(string $url, string $pattern): string` | 提取所有参数（路径 + 查询），返回 `key1=val1&key2=val2` |
| `router_find_advanced` | `(string $method, string $url, [string] $methods, [string] $patterns): int` | 在平行数组中查找匹配的路由索引 |
| `router_query_param_get` | `(string $query_string, string $key): string` | 从 query 字符串中获取单个参数值（含 URL decode） |
| `router_is_int` | `(string $s): int` | 类型校验：是否为合法整数 |
| `router_is_float` | `(string $s): int` | 类型校验：是否为合法浮点数 |

### 支持的类型约束

| 占位符 | 含义 | `"42"` | `"-7"` | `"3.14"` | `"abc"` | `""` |
|--------|------|--------|--------|---------|---------|------|
| `{int}` | 整数（仅数字 + 可选前导 `-`） | 通过 | 通过 | 拒绝 | 拒绝 | 拒绝 |
| `{float}` | 浮点数（数字 + 可选 `.` 和 `-`） | 通过 | 通过 | 通过 | 拒绝 | 拒绝 |
| `{string}` | 任意非空字符串 | 通过 | 通过 | 通过 | 通过 | 拒绝 |
| `{any}` | 任意值（含空字符串） | 通过 | 通过 | 通过 | 通过 | 通过 |

### 示例 1：单个查询参数 + 类型约束

场景：`GET /api/user?user_id=1` 返回用户信息，`user_id` 必须是整数。

```php
// 控制器
function api_user_by_id(string $raw, string $url): string {
    let $params = router_extract_params_advanced($url, "/api/user?user_id={int}");
    let $user_id = request_param($params, "user_id");

    // user_id 已确保是合法整数，可以直接使用
    let $json = "{\"id\":" . $user_id . ",\"name\":\"User " . $user_id . "\"}";
    return response_json(200, $json);
}

// 路由表（模式中包含查询参数类型）
let $methods  = ["GET"];
let $patterns = ["/api/user?user_id={int}"];
let $handlers = ["api_user_by_id"];

// 完整 URL 传入路由匹配
function app_dispatch(string $handler, string $raw, string $url): string {
    if ($handler == "api_user_by_id") { return api_user_by_id($raw, $url); }
    return response_404();
}
```

测试：
```bash
curl "http://localhost:8080/api/user?user_id=1"
# → {"id":1,"name":"User 1"}                    (int 合法 → 匹配)

curl "http://localhost:8080/api/user?user_id=42"
# → {"id":42,"name":"User 42"}                   (int 合法 → 匹配)

curl "http://localhost:8080/api/user?user_id=abc"
# → 404                                          (类型不匹配 → 拒绝)

curl "http://localhost:8080/api/user"
# → 404                                          (缺少参数 → 拒绝)
```

### 示例 2：路径参数 + 查询参数混用

场景：`GET /users/{id}/posts?page={int}&sort={string}` — RESTful 路径参数 + 类型化查询参数。

```php
// Pattern: "/users/{id}/posts?page={int}&sort={string}"
function user_posts_handler(string $raw, string $url): string {
    let $pattern = "/users/{id}/posts?page={int}&sort={string}";
    let $params = router_extract_params_advanced($url, $pattern);

    let $user_id = request_param($params, "id");
    let $page    = request_param($params, "page");
    let $sort    = request_param($params, "sort");

    if ($page == "") { $page = "1"; }
    if ($sort == "") { $sort = "desc"; }

    let $json = "{\"user_id\":" . $user_id . ",\"page\":" . $page . ",\"sort\":\"" . $sort . "\"}";
    return response_json(200, $json);
}
```

测试：
```bash
curl "http://localhost:8080/users/10/posts?page=2&sort=asc"
# → {"user_id":10,"page":2,"sort":"asc"}

curl "http://localhost:8080/users/10/posts?page=abc&sort=asc"
# → 404  (page 不是合法 int，类型校验失败)

curl "http://localhost:8080/users/10/posts"
# → 404  (缺少必需的查询参数)
```

### 示例 3：多路由并行匹配（使用 router_find_advanced）

场景：一个 API 服务同时处理多个带查询参数约束的路由。

```php
function app_dispatch(string $handler, string $raw, string $url): string {
    if ($handler == "user_by_id")   { return api_user_by_id($raw, $url); }
    if ($handler == "user_posts")   { return user_posts_handler($raw, $url); }
    if ($handler == "search")       { return search_handler($raw, $url); }
    if ($handler == "product_info") { return product_info_handler($raw, $url); }
    return response_404();
}

function app_main(): void {
    // 路由表 — patterns 中直接写完整匹配规则
    let $methods = [
        "GET",
        "GET",
        "GET",
        "GET"
    ];
    let $patterns = [
        "/api/user?user_id={int}",                  // 单个 int 参数
        "/users/{id}/posts?page={int}&sort={string}", // 路径 + 多查询参数
        "/search?q={string}&page={int}",             // 纯查询参数
        "/product?product_id={int}&lang={string}"    // 另一个业务接口
    ];
    let $handlers = [
        "user_by_id",
        "user_posts",
        "search",
        "product_info"
    ];

    // 主循环
    let $server = phprs_server_new(8080);
    if ($server < 0) { echo "Server start failed\n"; return; }
    echo "=== API Server on :8080 ===\n";

    for (let mut $running = 1; $running == 1; ) {
        let $client = phprs_server_accept($server);
        if ($client >= 0) {
            let $raw  = phprs_socket_read($client, 65536);
            let $method = phprs_http_method($raw);
            let $url    = phprs_url_decode(phprs_http_path($raw));

            // 使用高级路由查找
            let $route_idx = router_find_advanced($method, $url, $methods, $patterns);

            let $response = "";
            if ($route_idx >= 0) {
                $response = app_dispatch($handlers[$route_idx], $raw, $url);
            } else {
                $response = response_404();
            }

            phprs_socket_write($client, $response);
            phprs_socket_close($client);
        }
    }
    phprs_socket_close($server);
}

app_main();
```

全部测试：
```bash
# 匹配 /api/user?user_id={int}
curl "http://localhost:8080/api/user?user_id=1"
# → {"id":1,"name":"User 1"}

# 匹配 /users/{id}/posts?page={int}&sort={string}
curl "http://localhost:8080/users/10/posts?page=2&sort=asc"
# → {"user_id":10,"page":2,"sort":"asc"}

# 匹配 /search?q={string}&page={int}
curl "http://localhost:8080/search?q=hello&page=1"
# → (search results)

# 匹配 /product?product_id={int}&lang={string}
curl "http://localhost:8080/product?product_id=99&lang=zh"
# → (product info)

# 类型不匹配 → 404
curl "http://localhost:8080/api/user?user_id=abc"
# → 404

# 缺少参数 → 404
curl "http://localhost:8080/search?q=hello"
# → 404
```

### 比较：基础路由 vs 高级路由

| 特性 | 基础 router (`router.phprs`) | 高级 router (`router_advanced.phprs`) |
|------|-------------------------------|------------------------------------------|
| 路径段参数 `{id}` | 支持 | 支持（复用基础实现） |
| 查询参数匹配 | 不支持，需手动解析 | 支持，模式中直接写 `?key={type}` |
| 类型校验 | 无 | `{int}`, `{float}`, `{string}`, `{any}` |
| 路由匹配函数 | `router_find(method, path, ...)` | `router_find_advanced(method, full_url, ...)` |
| 参数提取函数 | `router_extract_params(path, pattern)` | `router_extract_params_advanced(full_url, pattern)` |
| 适用场景 | 简单 RESTful 路径 | 带查询参数的复杂 API |

---

## 简化路由：一行定义所有路由

`framework/router_simple.phprs` 提供了最简洁的路由定义方式 —— **一个字符串定义所有路由**，自动完成方法匹配、路径匹配、参数提取和类型校验。

### 引入

```php
include "framework/runtime.phprs";
include "framework/router_simple.phprs";
include "framework/request.phprs";
```

### 路由定义格式

```
"METHOD /path?query={type} => handler_name"
```

多条路由用 `|` 分隔：

```php
let $RT = "GET / => home|GET /users/{id} => user_detail|POST /login => login|GET /search?q={string}&page={int} => search";
```

### 核心 API

| 函数 | 说明 | 示例 |
|------|------|------|
| `route_match(method, url, routes)` | 匹配路由，返回 `"handler&key=val"` 或 `"404"` | `route_match("GET", "/users/42", $RT)` → `"user_detail&id=42"` |
| `route_handler(result)` | 从匹配结果中提取 handler 名称 | `route_handler("user_detail&id=42")` → `"user_detail"` |
| `route_params(result)` | 从匹配结果中提取参数字符串 | `route_params("user_detail&id=42")` → `"id=42"` |
| `route_param(params, key)` | 从参数字符串中获取单个值 | `route_param("id=42", "id")` → `"42"` |

### 完整示例

```php
<?phprs
include "framework/runtime.phprs";
include "framework/router_simple.phprs";
include "framework/request.phprs";
include "framework/response.phprs";

// ---- 一行路由定义 ----
let $RT = "GET / => home|GET /users/{id} => user_detail|POST /login => login|GET /search?q={string}&page={int} => search";

function home(string $params): string {
    return response_html(200, "<h1>Welcome</h1>");
}

function user_detail(string $params): string {
    let $id = route_param($params, "id");
    return response_json(200, "{\"id\":" . $id . "}");
}

function login(string $params): string {
    let $username = route_param($params, "username");
    return response_json(200, "{\"user\":\"" . $username . "\"}");
}

function search(string $params): string {
    let $q = route_param($params, "q");
    let $page = route_param($params, "page");
    return response_json(200, "{\"q\":\"" . $q . "\",\"page\":" . $page . "}");
}

// ---- 通用分发 ----
function app_dispatch(string $method, string $path, string $raw, string $routes): string {
    let $m = route_match($method, $path, $routes);
    let $h = route_handler($m);
    if ($h == "404") { return response_404(); }

    let $p = route_params($m);
    if ($h == "home")        { return home($p); }
    if ($h == "user_detail") { return user_detail($p); }
    if ($h == "login")       {
        let $req = phprs_request_parse($raw);
        let $all_params = route_params($m);
        if ($all_params == "") { $all_params = $req; }
        return login($all_params);
    }
    if ($h == "search")      { return search($p); }
    return response_404();
}

// ---- 主循环 ----
function app_main(): void {
    let $routes = "GET / => home|GET /users/{id} => user_detail|POST /login => login|GET /search?q={string}&page={int} => search";
    let $server = phprs_server_new(8080);
    for (let mut $running = 1; $running == 1; ) {
        let $client = phprs_server_accept($server);
        if ($client >= 0) {
            let $raw = phprs_socket_read($client, 65536);
            let $method = phprs_http_method($raw);
            let $path = phprs_http_path($raw);
            phprs_socket_write($client, app_dispatch($method, $path, $raw, $routes));
            phprs_socket_close($client);
        }
    }
    phprs_socket_close($server);
}
app_main();
?>
```

### 路由模式速查

| 模式写法 | 匹配的 URL | 说明 |
|----------|-----------|------|
| `GET /` | `GET /` | 精确路径匹配 |
| `GET /users/{id}` | `GET /users/42` | 路径段通配，提取 `id=42` |
| `GET /search?q={string}` | `GET /search?q=hello` | 必需查询参数，类型校验 |
| `GET /users/{id}/posts?page={int}` | `GET /users/42/posts?page=1` | 路径 + 查询参数混用 |
| `POST /login` | `POST /login` | POST 方法路由 |

### 对比：传统写法 vs 简化路由

**传统写法（三数组 + 手动解析）：**
```php
let $methods = ["GET", "GET", "POST", "GET"];
let $patterns = ["/", "/users/{id}", "/login", "/search?q={string}"];
let $handlers = ["home", "user_detail", "login", "search"];
// ... router_find + router_extract_params + 手动解析 query ...
```

**简化路由（一行字符串）：**
```php
let $RT = "GET / => home|GET /users/{id} => user_detail|POST /login => login|GET /search?q={string} => search";
// route_match() 自动完成所有匹配和参数提取
```

---

## phprs_request_parse(): 统一请求数据获取

`phprs_request_parse($raw)` 是一个 **C 运行时函数**，将原始 HTTP 请求解析为统一的 `key=value` 字符串，包含方法、路径、查询参数、Body、Headers。

### 返回格式

```
method=GET&path=/api/user&user_id=42&sort=desc&body=...&content_type=...&host=...
```

### 字段说明

| 字段 | 始终存在 | 说明 |
|------|----------|------|
| `method` | 是 | HTTP 方法：GET / POST / PUT / DELETE |
| `path` | 是 | URL 路径（不含 query string） |
| `body` | 是 | 原始 HTTP body（GET 请求为空字符串） |
| `content_type` | 是 | Content-Type 头（无则为空） |
| `host` | 是 | Host 头（无则为空） |
| `key=val` (查询参数) | 有参数时 | 所有 URL query string 参数，扁平合并 |
| `key=val` (POST 参数) | form-urlencoded 时 | 表单参数自动解析并扁平合并 |

### 示例

**GET 请求：**
```
Input:  GET /search?q=hello&page=1 HTTP/1.1\r\nHost: localhost\r\n\r\n
Output: method=GET&path=/search&q=hello&page=1&body=&content_type=&host=localhost
```

**POST 表单：**
```
Input:  POST /login HTTP/1.1\r\nHost: localhost\r\nContent-Type: application/x-www-form-urlencoded\r\n\r\nusername=admin&password=123
Output: method=POST&path=/login&body=username=admin&password=123&content_type=application/x-www-form-urlencoded&host=localhost&username=admin&password=123
```

**POST JSON：**
```
Input:  POST /api/users HTTP/1.1\r\nHost: localhost\r\nContent-Type: application/json\r\n\r\n{"name":"test"}
Output: method=POST&path=/api/users&body={"name":"test"}&content_type=application/json&host=localhost
```

### 使用方式

配合 `request_param()` 获取任意字段：

```php
let $req = phprs_request_parse($raw);
let $method = request_param($req, "method");     // "GET"
let $path = request_param($req, "path");         // "/search"
let $user_id = request_param($req, "user_id");   // "42" (from query string or form body)
let $body = request_param($req, "body");         // raw body
let $ct = request_param($req, "content_type");   // "application/json"
let $host = request_param($req, "host");         // "localhost:8002"
```

> **注意：** `phprs_request_parse` 会将 GET 查询参数和 POST 表单参数**合并到同一扁平空间**。如果 GET 和 POST 中有同名参数，POST 的值会覆盖 GET 值。

---

## Blog 示例应用

`examples/blog/app.phprs` 是一个完整的功能演示应用，展示了 MVC 框架的所有核心功能。

### 路由表

| 路径 | HTTP 方法 | 控制器 | 返回类型 | 说明 |
|------|-----------|--------|----------|------|
| `/` | GET | `home_page()` | HTML | 欢迎页面，带 CSS 样式 |
| `/about` | GET | `about_page()` | HTML | 框架架构介绍 |
| `/api/info` | GET | `api_info()` | JSON | 框架元数据 |
| `/api/echo` | GET | `api_echo()` | JSON | Echo 端点 |
| `*` | GET | `response_404()` | HTML | 404 未找到 |

### 构建并运行

```bash
phprs build examples/blog/app.phprs -o blog.exe
./blog.exe
```

访问 `http://localhost:8080` 查看效果。

### 模板渲染系统

Blog 应用使用简单的 `{{placeholder}}` 替换模板引擎：

```php
let $html = "<h1>{{title}}</h1><p>{{message}}</p>";
let $out = render2($html, "title", "Welcome", "message", "Hello World");
// 输出: <h1>Welcome</h1><p>Hello World</p>
```

`render()` 函数替换单个占位符，`render2()` 替换两个。通过链式调用可以支持任意数量的占位符。

---

## WebSocket 支持

PHPRS MVC 框架支持 WebSocket 协议 (RFC 6455)，可以创建持久化的双向连接，适用于实时聊天、消息推送、协作编辑等场景。

### 架构

```
HTTP Upgrade 请求
  │
  ├─ phprs_is_websocket_upgrade(raw) → 检测是否为 WebSocket 升级
  │
  ├─ phprs_ws_handshake_response(raw) → 生成 101 响应 (SHA-1 + Base64)
  │     └─ Sec-WebSocket-Key + Magic GUID → SHA1 → Base64 → Accept
  │
  └─ 消息循环 (保持连接不关闭)
        ├─ phprs_ws_read_frame(fd) → 读取帧，返回 "opcode:payload"
        ├─ phprs_ws_write_frame(fd, payload, opcode) → 发送帧
        ├─ phprs_ws_send_pong(fd, payload) → 响应 Ping
        └─ phprs_ws_close(fd) → 发送关闭帧 + 关闭连接
```

### WebSocket Echo 服务器

#### 服务端代码

`examples/websocket/echo.phprs` — 完整的 WebSocket 回显服务器：

```php
<?phprs
include "../../framework/runtime.phprs";

let $server = phprs_server_new(8080);

for (let mut $running = 1; $running == 1; ) {
    let $client = phprs_server_accept($server);

    if ($client >= 0) {
        let $raw = phprs_socket_read($client, 65536);

        // 检测 WebSocket 升级请求
        if (phprs_is_websocket_upgrade($raw) == 1) {
            // 握手
            let $handshake = phprs_ws_handshake_response($raw);
            phprs_socket_write($client, $handshake);

            // 消息循环 (保持连接 - 不关闭 socket)
            let mut $ws_alive = 1;
            for (let mut $ws_alive = 1; $ws_alive == 1; ) {
                let $frame_raw = phprs_ws_read_frame($client, 0);
                let $opcode_str = phprs_str_split($frame_raw, ":", 0);
                let $payload = phprs_str_split($frame_raw, ":", 1);

                if ($opcode_str == "1") {
                    // Text 帧 — 回显
                    phprs_ws_write_frame($client, $payload, 1);
                } else {
                    if ($opcode_str == "8") {
                        // Close 帧 — 退出
                        $ws_alive = 0;
                    } else {
                        if ($opcode_str == "9") {
                            // Ping — 回复 Pong
                            phprs_ws_send_pong($client, $payload);
                        } else {
                            $ws_alive = 0;
                        }
                    }
                }
            }
            phprs_ws_close($client);
        } else {
            // HTTP 回退
            let $html = "<h1>WebSocket Echo Server</h1>...";
            let $response = phprs_http_response(200, "text/html; charset=utf-8", $html);
            phprs_socket_write($client, $response);
            phprs_socket_close($client);
        }
    }
}
phprs_socket_close($server);
?>
```

#### 构建和测试

```bash
# 构建
phprs build examples/websocket/echo.phprs -o echo_ws.exe

# 运行
./echo_ws.exe

# 使用 wscat 测试
npx wscat -c ws://localhost:8080
> Hello
< Hello

# 在浏览器中打开 HTTP 测试页面
# http://localhost:8080
```

### 关键设计模式

#### 嵌套循环模式

WebSocket 连接使用嵌套循环，避免 PHPRS 缺少 `break`/`continue` 的限制：

```php
// 外层循环 — 接受连接
for (let mut $running = 1; $running == 1; ) {
    let $client = phprs_server_accept($server);

    if ($client >= 0) {
        let $raw = phprs_socket_read($client, 65536);

        if (phprs_is_websocket_upgrade($raw) == 1) {
            // WebSocket 握手
            phprs_socket_write($client, phprs_ws_handshake_response($raw));

            // 内层循环 — 处理消息 (保持连接)
            let mut $ws_alive = 1;
            for (let mut $ws_alive = 1; $ws_alive == 1; ) {
                // 读取帧、处理、如需要设置 $ws_alive = 0 退出
            }

            phprs_ws_close($client);
        } else {
            // HTTP 请求 — 直接响应并关闭
            phprs_socket_write($client, response);
            phprs_socket_close($client);
        }
    }
}
```

#### 帧格式

`phprs_ws_read_frame()` 返回 `"opcode:payload"` 格式的字符串：

| 返回值 | 含义 |
|--------|------|
| `"1:hello"` | Text 帧，payload 为 "hello" |
| `"8:"` | Close 帧 |
| `"9:ping-data"` | Ping 帧 |
| `"10:pong-data"` | Pong 帧 |
| `"-1:"` | 连接错误或断开 |

使用 `phprs_str_split()` 提取各部分：

```php
let $opcode_str = phprs_str_split($frame_raw, ":", 0);  // "1"
let $payload = phprs_str_split($frame_raw, ":", 1);      // "hello"
```

### 框架辅助函数

`framework/websocket.phprs` 提供了便捷的封装函数：

| 函数 | 说明 |
|------|------|
| `ws_accept(raw, fd)` → int | 检测升级 + 发送握手响应 |
| `ws_read(fd)` → string | 读取下一帧 |
| `ws_send_text(fd, msg)` → int | 发送文本帧 |
| `ws_disconnect(fd)` → void | 发送关闭帧 + 关闭连接 |
| `ws_frame_opcode(frame)` → int | 提取 opcode |
| `ws_frame_payload(frame)` → string | 提取 payload |

使用示例：

```php
include "framework/runtime.phprs";
include "framework/websocket.phprs";

// 简化版消息循环
if (ws_accept($raw, $client) == 1) {
    let mut $alive = 1;
    for (let mut $alive = 1; $alive == 1; ) {
        let $frame = ws_read($client);
        let $op = ws_frame_opcode($frame);
        let $msg = ws_frame_payload($frame);

        if ($op == 1) {
            ws_send_text($client, $msg);  // Echo
        } else {
            $alive = 0;
        }
    }
    ws_disconnect($client);
}
```

### 限制

- **WebSocket 线程隔离**：每个 WebSocket 连接在独立线程中运行，不会阻塞 HTTP 服务
- **阻塞 I/O**：读写均为阻塞模式，与现有服务器模型一致
- **无分片支持**：每个帧独立处理，不支持跨帧重组大消息
- **1 MB 帧大小限制**：防止内存耗尽
- **回声模式**：chat 端点为演示用途，仅回显给发送者（真正的广播需跟踪所有连接 FD）

---

## 安全特性

PHPRS MVC 框架内置多项安全机制，保护应用免受常见 Web 攻击。

### CSRF 防护

所有 POST/PUT/PATCH/DELETE 请求自动验证 CSRF token。

#### 生成 Token

```php
<?phprs
include "system/request.phprs";

// 在 HTML 表单中嵌入 CSRF 隐藏字段：
function show_form(string $raw): string {
    let $html = "<form method=\"POST\" action=\"/api/db/create\">"
        . csrf_field($raw)
        . "<input name=\"name\" placeholder=\"Name\">"
        . "<button type=\"submit\">Submit</button>"
        . "</form>";
    return render_page("New Record", $html);
}
?>
```

#### 验证流程

框架在 `app_dispatch` 中自动验证所有写操作请求：

```php
// 自动执行，无需手动调用：
if ($method == "POST" || $method == "PUT" || $method == "PATCH" || $method == "DELETE") {
    let $csrf = request_param($body, "_csrf_token");
    if (csrf_verify($raw, $csrf) == 0) {
        return api_error(403, "CSRF token invalid");
    }
}
```

#### API 客户端使用

JSON API 请求中包含 `_csrf_token` 字段：

```bash
# 先获取 token（通过 session cookie）
curl -c cookies.txt http://localhost:8080/

# 提交带 CSRF token 的 POST 请求
curl -b cookies.txt -X POST http://localhost:8080/api/db/create \
  -H "Content-Type: application/json" \
  -d '{"name":"Alice","email":"a@b.com","title":"Test","_csrf_token":"YOUR_TOKEN"}'
```

### Session 管理

基于文件的 Session 存储，使用加密安全的 Session ID。

```php
<?phprs
include "system/request.phprs";

// 启动/恢复 session
let $sid = session_start($raw);

// 读取 session 数据
let $username = session_get($sid, "username");

// 写入 session 数据
session_set($sid, "username", "Alice");
session_set($sid, "role", "admin");

// 设置 session cookie（在响应头中）
let $cookie_hdr = session_cookie_header($sid);
// 返回：Set-Cookie: PHPRS_SESSID=sess_xxx; Path=/; HttpOnly; SameSite=Strict

// 销毁 session（登出）
session_destroy($sid);
?>
```

**安全特性**：
- Session ID 使用 `md5(random_bytes(32))` 生成（加密安全）
- Cookie 设置 `HttpOnly`（防 XSS 窃取）和 `SameSite=Strict`（防 CSRF）
- Session ID 验证：仅允许 `[a-zA-Z0-9_-]`，最长 64 字符（防路径遍历）

### CORS 跨域配置

默认限制为 `http://localhost:8080`，生产环境应修改为实际域名。

```php
<?phprs
// 在 app_main() 中配置 CORS：
cors_config("https://your-domain.com", "GET,POST,PUT,DELETE,PATCH,OPTIONS", "Content-Type,Authorization");

// 框架自动处理：
// 1. OPTIONS 预检请求返回正确的 CORS 头
// 2. 所有响应注入 Access-Control-Allow-* 头
?>
```

#### 修改 CORS 配置

编辑 `app.phprs` 中的 `app_main()` 函数：

```php
// 允许多个来源（生产环境）：
cors_config("https://app.example.com", "GET,POST,PUT,DELETE", "Content-Type,Authorization,X-Custom-Header");

// 开发环境允许所有来源：
cors_config("*", "GET,POST,PUT,DELETE,PATCH,OPTIONS", "Content-Type,Authorization");
```

### XSS 防护

#### 模板引擎自动转义

`view_render_var()` 默认对所有变量进行 HTML 转义：

```php
<?phprs
include "system/view.phprs";

let $template = "<h1>Hello, {{name}}</h1>";
let $user_input = "<script>alert('xss')</script>";

// 安全：自动转义
let $safe = view_render_var($template, "name", $user_input);
// 输出: <h1>Hello, &lt;script&gt;alert(&#039;xss&#039;)&lt;/script&gt;</h1>

// 如确实需要插入 HTML（仅用于受信内容）：
let $raw_html = view_render_var_raw($template, "name", "<strong>Bold</strong>");
?>
```

#### render_page 标题转义

`render_page()` 自动对 `$title` 参数进行 HTML 转义：

```php
// 安全：即使标题包含用户输入也不会产生 XSS
let $response = render_page($user_title, $html_body);
```

#### 手动转义

```php
let $safe = view_escape($user_input);
// 转义字符：& < > " '
```

### 速率限制

基于客户端 IP 的请求频率限制：

```php
<?phprs
// 配置：100 请求/分钟（每 IP）
rate_limit_config(100, 60);

// 自动应用于所有请求 —— 超限返回 429：
// HTTP/1.1 429 Too Many Requests
// {"code":429,"msg":"Too Many Requests"...}
?>
```

**自定义配置**：

```php
// 严格限制：10 请求/分钟
rate_limit_config(10, 60);

// 宽松限制：1000 请求/分钟
rate_limit_config(1000, 60);
```

### 客户端 IP 获取

框架从 TCP 连接自动提取客户端真实 IP（非硬编码）：

```php
<?phprs
// 在请求处理中获取客户端 IP：
let $server = request_server($raw, $port, $client_fd);
let $ip = server_param($server, "REMOTE_ADDR");
echo "Client IP: " . $ip . "\n";  // 如 "192.168.1.100"

// 或直接使用底层函数：
let $ip = phprs_client_ip($client_fd);
?>
```

### URL 解码

`request_param()` 自动对提取的参数值进行 URL 解码：

```php
// 请求: GET /search?q=hello%20world
let $query = request_param($params, "q");
echo $query;  // "hello world" （自动解码）
```

### 数据库操作错误处理

所有写操作（create/update/delete）检查文件写入结果，失败返回 500：

```php
// 如果磁盘满或权限不足，返回：
// HTTP/1.1 500 Internal Server Error
// {"code":500,"msg":"Failed to save record","data":[]}
```

### 安全最佳实践

1. **CSRF**：HTML 表单使用 `csrf_field($raw)` 嵌入 token
2. **XSS**：始终使用 `view_render_var()`（自动转义），仅对受信 HTML 使用 `view_render_var_raw()`
3. **CORS**：生产环境将 `*` 替换为实际域名
4. **Session**：不要在 URL 中传递 session ID
5. **输入验证**：对路由参数使用类型约束（`{int}` 自动拒绝溢出值）
6. **密码**：使用 `password_hash()` / `password_verify()`（SHA1 HMAC + 安全随机）
7. **随机数**：使用 `random_bytes()` / `random_int()`（加密安全）

---

## API 参考

> 完整的函数文档（含参数说明、返回值、示例代码）请参阅 **[API_Reference.md](API_Reference.md)**。
>
> 以下是所有可用函数的快速概览。

### 需引入 runtime.phprs 的函数（38 个）

`include "framework/runtime.phprs"` 后可用：

**TCP 套接字**

| 函数 | 签名 | 说明 |
|------|------|------|
| `phprs_server_new` | `(int $port) -> int` | 创建 TCP 服务器套接字。返回文件描述符（负数表示失败） |
| `phprs_server_accept` | `(int $fd) -> int` | 接受客户端连接。阻塞直到有连接到达 |
| `phprs_socket_read` | `(int $fd, int $max_size) -> string` | 从套接字读取最多 $max 字节 |
| `phprs_socket_write` | `(int $fd, string $data) -> int` | 向套接字写入数据。返回写入的字节数 |
| `phprs_socket_close` | `(int $fd) -> void` | 关闭套接字 |

**网络连接：** `phprs_tcp_connect`, `phprs_tls_connect`, `phprs_dns_resolve`, `phprs_socket_read_all`

**HTTP 解析（服务端）：** `phprs_http_method`, `phprs_http_path`, `phprs_http_header`, `phprs_http_body`, `phprs_url_decode`, `phprs_http_response`, `phprs_request_parse`

**HTTP 客户端（底层）：** `phprs_http_build_request`, `phprs_http_response_status`, `phprs_http_response_body`

**文件 I/O：** `phprs_file_read`, `phprs_file_write`, `phprs_file_exists`

**JSON：** `phprs_json_get_string`, `phprs_json_get_int`

**字符串：** `phprs_str_replace`, `phprs_str_contains`, `phprs_str_split`, `phprs_str_starts_with`, `phprs_str_ends_with`, `phprs_str_upper`, `phprs_str_lower`

**WebSocket：** `phprs_is_websocket_upgrade`, `phprs_ws_handshake_response`, `phprs_ws_read_frame`, `phprs_ws_write_frame`, `phprs_ws_send_pong`, `phprs_ws_close`

**线程：** `phprs_thread_spawn`, `phprs_mutex_new`, `phprs_mutex_lock`, `phprs_mutex_unlock`

### PHPRS 内置函数（无需 include）

| 函数 | 签名 | 说明 |
|------|------|------|
| `echo` | 语句 | 打印到标准输出（可接受 string / int / bool） |
| `strlen` | `(string): int` | 字符串字符数 |
| `count` | `(array\|dict): int` | 数组/字典元素个数 |
| `trim` | `(string): string` | 去除首尾空白字符 |
| `str_contains` | `(string, string): bool` | 检查子串存在（返回 true/false） |

### HTTP 客户端高级封装

`include "framework/http_client.phprs"` 后可用（自动引入 runtime.phprs）：

| 函数 | 签名 | 说明 |
|------|------|------|
| `http_get` | `(string $url): string` | HTTP GET — 自动处理 HTTP/HTTPS/DNS/端口 |
| `http_post` | `(string $url, string $body, string $content_type): string` | HTTP POST — 发送 JSON 等格式数据 |
| `http_status` | `(string $response): int` | 提取状态码 |
| `http_body` | `(string $response): string` | 提取响应体 |
| `http_parse_url` | `(string $url): string` | 解析 URL 为 `proto=...&host=...&path=...` |
| `http_parsed_proto` | `(string $parsed): string` | 提取协议（http / https） |
| `http_parsed_host` | `(string $parsed): string` | 提取主机名 |
| `http_parsed_path` | `(string $parsed): string` | 提取路径 |

### Framework WebSocket 辅助函数

通过 `include "framework/websocket.phprs"` 可用：

| 函数 | 签名 | 说明 |
|------|------|------|
| `ws_accept` | `(string $raw, int $client_fd): int` | 检测升级 + 发送握手响应 |
| `ws_read` | `(int $fd): string` | 读取下一帧（超时 5s） |
| `ws_send_text` | `(int $fd, string $message): int` | 发送文本帧 |
| `ws_disconnect` | `(int $fd): void` | 发送关闭帧 + 关闭连接 |
| `ws_frame_opcode` | `(string $frame): int` | 提取 opcode |
| `ws_frame_payload` | `(string $frame): string` | 提取 payload |

**WebSocket Opcode 常量：** Text=1, Close=8, Ping=9, Pong=10

---

## HTTP 客户端指南

PHPRS MVC 框架支持从控制器中发起对第三方 API 的出站 HTTP 请求。

### 基本用法

```php
include "framework/runtime.phprs";
include "framework/http_client.phprs";

// GET 请求
let $response = http_get("http://api.example.com/data");
let $status = http_status($response);
let $body = http_body($response);

if ($status == 200) {
    echo "成功: " . $body . "\n";
}

// POST 请求（JSON）
let $json_body = "{\"name\":\"test\"}";
let $resp = http_post("http://api.example.com/create", $json_body, "application/json");
```

### 底层 API

如需更精细的控制，可以直接使用底层函数：

```php
// DNS 解析
let $ip = phprs_dns_resolve("example.com");  // → "93.184.216.34"

// TCP 连接
let $fd = phprs_tcp_connect("example.com", 80);
if ($fd >= 0) {
    // 构建请求
    let $req = phprs_http_build_request("GET", "example.com", "/", "", "");
    phprs_socket_write($fd, $req);
    
    // 读取完整响应
    let $resp = phprs_socket_read_all($fd);
    let $status = phprs_http_response_status($resp);
    let $body = phprs_http_response_body($resp);
    
    phprs_socket_close($fd);
}
```

### URL 解析

```php
let $parsed = http_parse_url("http://example.com/api/data?id=1");
// 返回: "host=example.com&path=/api/data?id=1"

let $host = http_parsed_host($parsed);  // "example.com"
let $path = http_parsed_path($parsed);  // "/api/data?id=1"
```

> **注意：** HTTPS 暂不支持。如需 HTTPS，可使用本地代理（nginx、mitmproxy 等）。

---

## 高并发指南

PHPRS MVC 框架支持线程池模型的高并发处理。

### 线程模型

每个连接在独立线程中处理：主线程负责 `accept()` + `read()`，然后将请求派发给工作线程。工作线程完成业务逻辑后写回响应并关闭连接。

```
主线程 (Main Thread)
  │
  ├─ accept() 客户端连接
  ├─ read() 读取请求
  ├─ phprs_thread_spawn("handler", fd, raw)
  │     │
  │     └─ 工作线程 (Worker Thread)
  │           ├─ 调用 handler(raw) → 执行业务逻辑
  │           ├─ write() 写回响应
  │           └─ close() 关闭连接
  │
  └─ 立即返回 accept() 等待下一连接
```

### 基本用法

**步骤 1：** 定义处理函数，签名为 `(string $raw): string`：

```php
function handle_request(string $raw): string {
    let $path = phprs_http_path($raw);
    return match ($path) {
        "/" => home_page(),
        "/api" => api_handler(),
        _ => response_404(),
    };
}
```

**步骤 2：** 使用线程化的 accept 循环：

```php
let $server = phprs_server_new(8080);

for (let mut $running = 1; $running == 1; ) {
    let $client = phprs_server_accept($server);
    if ($client >= 0) {
        let $raw = phprs_socket_read($client, 65536);
        phprs_thread_spawn("handle_request", $client, $raw);
    }
}
```

> **重要：** 处理函数必须具有 `(string $raw): string` 签名（接收原始 HTTP 请求，返回完整 HTTP 响应）。编译器会自动将此类函数注册到线程调度表中。

### 互斥锁

在多个线程间保护共享数据：

```php
let $counter_lock = phprs_mutex_new();

function handle_request(string $raw): string {
    phprs_mutex_lock($counter_lock);
    // ... 修改共享数据 ...
    phprs_mutex_unlock($counter_lock);
    return response;
}
```

### 框架辅助

使用 `include "framework/app_threaded.phprs"` 快速搭建线程化服务器：

```php
include "framework/runtime.phprs";
include "framework/app_threaded.phprs";

function handle_request(string $raw): string {
    // 业务逻辑
}

// 自动使用线程池模型
app_run_threaded(8080, "handle_request");
```

### 示例应用

完整示例见 `examples/threaded/blog_threaded.phprs`：

```bash
phprs build examples/threaded/blog_threaded.phprs -o blog_threaded.exe
./blog_threaded.exe
```

并发测试：
```bash
for i in {1..5}; do curl http://localhost:8080/ &; done
# 所有请求应同时返回 200
```

---

## 部署

### 编译为单一二进制

```bash
phprs build myapp.phprs -o myapp.exe
```

输出的 `myapp.exe` 包含了：
- 你的应用代码
- MVC 框架代码
- C 运行时（HTTP/TCP/JSON/字符串）
- 所有依赖

无需 PHP 运行时、Web 服务器或外部依赖。将其复制到任何同架构的 Windows/Linux 机器即可运行。

### Windows 部署

```powershell
# 使用 MSVC 编译
phprs build app.phprs -o app.exe

# 直接运行
.\app.exe

# 或安装为 Windows 服务
sc create "PHPRS App" binPath="C:\app\app.exe"
```

### Linux 部署

```bash
# 打包为 systemd 服务
cat > /etc/systemd/system/phprs-app.service << 'EOF'
[Unit]
Description=PHPRS MVC Application
After=network.target

[Service]
ExecStart=/opt/phprs-app/app
Restart=always
User=www-data

[Install]
WantedBy=multi-user.target
EOF

systemctl enable phprs-app
systemctl start phprs-app
```

### 性能考量

- **线程池模型**：每连接一线程，可同时处理数百个并发连接
- **阻塞 I/O + 线程**：工作线程中可使用阻塞 HTTP 客户端调用第三方 API，不影响其他连接的响应
- **无 GC 暂停**：零成本抽象，无垃圾回收
- **零依赖部署**：单一二进制文件，启动时间 < 100ms
- **内存占用**：基线 ~5-10MB（取决于静态资源和线程数）

---

## 故障排除

### 编译错误

**`Failed to start server on port 8080`**
→ 端口已被占用。更改端口号或终止占用端口的进程。

**`LINK: fatal error LNK1104: cannot open file`**
→ 目标 .exe 文件正在运行。先停止运行中的进程，再编译。

**`error: No working C compiler found`**
→ 安装以下任一 C 编译器：
- Windows: Visual Studio (MSVC) 或 `choco install mingw`
- Linux: `apt install gcc`

### 运行时错误

**curl 返回空响应**
→ 确认服务器已启动，检查控制台输出。

**端口被占用**
```bash
# 查找占用端口的进程
netstat -ano | findstr :8080
# 终止进程
taskkill /F /PID <PID>
```

### 语言限制

当前 MVP 版本的限制：

| 限制 | 解决方案 |
|------|----------|
| 数组作为函数参数时会丢失长度信息 | 在调用处手动传递长度，或使用内联代码 |
| 不支持 `break`/`continue` | 使用 if/else 或强制循环退出（设置循环变量） |
| 不支持嵌套 JSON/对象 | 使用扁平 JSON 或字符串解析 |
| 单线程 accept 循环 | 串行处理连接，未来可在线程池中扩展 |
| 不支持函数指针/闭包 | 使用 `match` 表达式显式分发 |

---

## 编译器命令

```bash
# 编译为原生二进制
phprs build <source.phprs> -o <output.exe>

# 仅生成 C 代码（不编译）
phprs emit-c <source.phprs>

# 运行解释器（用于开发/调试）
phprs run <source.phprs>
```

---

## 项目结构

```
php/
├── src/
│   ├── lexer/           # 词法分析器
│   ├── parser/          # 语法分析器 + AST
│   ├── typechecker/     # 类型检查器
│   ├── mir/             # 中级表示
│   ├── codegen/
│   │   ├── ast_to_c.rs  # AST → C 转译器（含线程调度表生成）
│   │   └── phprs_runtime.c  # C 运行时（嵌入，~1100 行）
│   ├── preprocessor.rs  # include 预处理器
│   └── lib.rs           # 库入口
├── framework/
│   ├── runtime.phprs      # FFI 声明（36 个系统函数）
│   ├── router.phprs       # URL 路由器（路径段）
│   ├── router_advanced.phprs # 高级路由器（查询参数类型约束）
│   ├── router_simple.phprs   # 简化路由器（一行字符串定义所有路由）
│   ├── request.phprs      # 请求解析 + request_param
│   ├── response.phprs     # 响应构建器
│   ├── view.phprs         # 模板引擎
│   ├── websocket.phprs    # WebSocket 辅助函数
│   ├── http_client.phprs  # HTTP 客户端辅助函数（新增）
│   ├── app.phprs          # 应用启动器（单线程）
│   └── app_threaded.phprs # 应用启动器（多线程，新增）
├── examples/
│   ├── blog/
│   │   └── app.phprs      # Blog 示例应用
│   ├── websocket/
│   │   └── echo.phprs     # WebSocket 回显服务器
│   ├── http_client/
│   │   └── demo.phprs     # HTTP 客户端演示（新增）
│   └── threaded/
│       └── blog_threaded.phprs  # 多线程 Blog 示例（新增）
├── docs/
│   └── PHPRS_MVC_Guide.md # 本文档
├── Cargo.toml
└── README.md
```

---

*PHPRS MVC Framework v0.4.0 — 将 PHP 的简洁与 Rust 的性能合二为一，现已支持 HTTP 客户端、高并发与 WebSocket*
