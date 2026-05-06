# PHPRS

**类 PHP 语法的原生编译器 — 将 PHP 风格代码编译为独立可执行文件。**

用 PHP 风格写代码，编译为独立的 `.exe` 二进制文件——无需解释器、无需虚拟机、零运行时开销。PHPRS 融合了 PHP 的简洁语法与编译型语言的极致性能。

## 快速开始

```bash
# 从源码安装（需要 Rust + C 编译器）
git clone https://github.com/twoperiods/phprslang.git
cd phprslang
cargo build --release

# 运行脚本（解释模式）
./target/release/phprs run examples/websocket/echo.phprs

# 编译为原生二进制
./target/release/phprs build examples/blog/app.phprs -o blog.exe
./blog.exe

# 创建 MVC 项目脚手架
./target/release/phprs create_project my_app
cd my_app
../target/release/phprs run app.phprs    # 开发模式
```

## 命令行

```
phprs run   <file.phprs>          解释运行（即时反馈，适合开发）
phprs build <file.phprs> [-o exe]  编译为原生二进制
phprs emit-c <file.phprs>          输出生成的 C 代码
phprs create_project <name>        创建 MVC 项目脚手架
phprs help                          显示帮助
```

## 语言概览

PHPRS 语法是 PHP 的子集，增加静态类型标注。源文件使用 `.phprs` 扩展名，代码须包裹在 `<?phprs ... ?>` 标签中。

```php
<?phprs
// 变量
let $name = "World";
let $count = 42;

// 带类型标注的函数
function greet(string $name): string {
    return "Hello, " . $name . "!";
}

// 条件判断
if ($count > 0) {
    echo greet($name) . "\n";
}

// 数组与字典
let $items = ["apple", "banana", "cherry"];
let $config = ["host" => "127.0.0.1", "port" => "8080"];

// JSON
echo json_encode(["status" => "ok", "data" => $items]);
?>
```

### 内置函数

120+ 内置函数，覆盖：

| 分类 | 函数 |
|---|---|
| **字符串** | `strlen`, `substr`, `strpos`, `stripos`, `strrpos`, `explode`, `implode`, `sprintf`, `trim`, `ltrim`, `rtrim`, `str_replace`, `strtolower`, `strtoupper`, `ucfirst`, `htmlspecialchars`, `nl2br`, `strip_tags`, `str_repeat`, `number_format`, `chr`, `ord`, `addslashes`, `stripslashes`, `phprs_str_contains`, `phprs_str_starts_with`, `phprs_str_ends_with`, `phprs_str_split` |
| **数组** | `array_push`, `array_pop`, `array_shift`, `array_unshift`, `array_keys`, `array_values`, `array_merge`, `array_flip`, `array_slice`, `array_sum`, `array_unique`, `array_reverse`, `array_filter`, `array_map`, `array_reduce`, `array_diff`, `array_combine`, `array_column`, `array_fill`, `array_rand`, `array_chunk`, `array_count_values`, `array_product`, `array_intersect`, `in_array`, `array_search`, `array_key_exists`, `range`, `sort`, `rsort` |
| **文件 I/O** | `file_get_contents`, `file_put_contents`, `file_exists`, `is_dir`, `is_file`, `mkdir`, `unlink`, `basename`, `dirname`, `scandir`, `copy`, `rename`, `filesize`, `filemtime`, `pathinfo`, `move_uploaded_file`, `realpath`, `getcwd` |
| **JSON** | `json_encode`, `json_decode`, `phprs_json_get_string`, `phprs_json_get_int` |
| **URL/编码** | `urlencode`, `urldecode`, `parse_url`, `http_build_query`, `base64_encode`, `base64_decode` |
| **HTTP** | `phprs_http_response`, `phprs_http_method`, `phprs_http_path`, `phprs_http_header`, `phprs_http_body`, `phprs_url_decode`, `phprs_request_parse`, `curl`, `curl_async`, `curl_wait`, `curl_is_done` |
| **HTTP 客户端** | `phprs_dns_resolve`, `phprs_tcp_connect`, `phprs_tls_connect`, `phprs_socket_read_all`, `phprs_http_build_request`, `phprs_http_response_status`, `phprs_http_response_body` |
| **网络** | `phprs_server_new`, `phprs_server_accept`, `phprs_socket_read`, `phprs_socket_write`, `phprs_socket_close` |
| **WebSocket** | `phprs_is_websocket_upgrade`, `phprs_ws_handshake_response`, `phprs_ws_read_frame`, `phprs_ws_write_frame`, `phprs_ws_send_pong`, `phprs_ws_close` |
| **类型** | `is_null`, `is_int`, `is_float`, `is_string`, `is_bool`, `is_array`, `empty`, `isset`, `unset`, `gettype` |
| **哈希/安全** | `md5`, `sha1`, `uniqid`, `password_hash`, `password_verify`, `random_bytes`, `random_int` |
| **数学** | `abs`, `ceil`, `floor`, `round`, `max`, `min`, `rand`, `mt_rand`, `pow`, `sqrt` |
| **日期** | `time`, `date`, `strtotime`, `microtime` |
| **多线程** | `phprs_thread_spawn`, `phprs_thread_pool_init`, `phprs_thread_pool_enqueue`, `phprs_thread_pool_shutdown`, `phprs_mutex_new`, `phprs_mutex_lock`, `phprs_mutex_unlock` |
| **中间件** | `phprs_rate_limit_init`, `phprs_rate_limit_check`, `phprs_cors_set_config`, `phprs_cors_get_origin`, `phprs_cors_get_methods`, `phprs_cors_get_headers`, `phprs_cors_is_preflight` |
| **系统** | `sleep`, `usleep` |

## 架构

### 编译管线

```
源码 (.phprs)
  → 预处理器     (include/require 解析、标签剥离)
  → 词法分析器   (Token 流)
  → 语法分析器   (递归下降 → AST)
  → 类型检查器   (静态分析、TypeEnv)
  → MIR 构建器   (AST → 类 SSA 的中间表示)
  → C 代码生成   (MIR → C 源码 + 嵌入式运行时)
  → 系统 C 编译器 (MSVC/GCC/Clang → 原生二进制)
```

### 双执行模式

| 模式 | 命令 | 原理 |
|---|---|---|
| **解释模式** | `phprs run` | 树遍历求值器直接执行 AST，即时反馈 |
| **编译模式** | `phprs build` | 完整管线：AST → MIR → C → 原生二进制 |

### 模块地图

| 模块 | 职责 |
|---|---|
| `src/main.rs` | CLI 入口 |
| `src/lib.rs` | 库根，管线编排 |
| `src/preprocessor.rs` | 文本级 include/require 解析 |
| `src/lexer/` | 词法分析器 |
| `src/parser/` | 递归下降语法分析器、AST 定义 |
| `src/interpreter/` | 树遍历解释器（开发模式） |
| `src/typeck/` | 静态类型检查 |
| `src/mir/` | MIR 定义及 AST→MIR 转换 |
| `src/codegen/` | C 转译器 + 嵌入式 C 运行时（3757 行） |
| `src/scaffold.rs` | MVC 项目脚手架生成器（1359 行） |

### C 运行时 (`phprs_runtime.c`)

自包含的 C 库，编译进每个二进制文件。提供 HTTP 服务器、WebSocket、JSON、TLS/HTTPS、文件 I/O、字符串工具、哈希函数以及自定义内存分配器。零外部依赖。

## MVC 框架

`phprs create_project` 生成可直接用于生产的 MVC 项目：

```
my_app/
├── app.phprs                    入口文件 + 服务器主循环
├── system/                      运行时 & 核心库
│   ├── runtime.phprs            外部函数声明（100+ 函数）
│   ├── request.phprs            请求解析、Session、CSRF
│   ├── response.phprs           HTTP 响应构建器
│   ├── view.phprs               模板引擎
│   ├── websocket.phprs          WebSocket 助手
│   ├── http_client.phprs        HTTP 客户端
│   └── curl.phprs               cURL 封装
├── config/                      配置文件
│   ├── router_simple.phprs      简单路由解析器
│   ├── router.phprs             基础路径路由
│   ├── router_advanced.phprs    高级路由（类型化参数）
│   ├── database.phprs           数据库配置（Webman 风格）
│   └── redis.phprs              Redis 配置（Webman 风格）
├── middleware/                   请求中间件
│   ├── rate_limit.phprs         速率限制（基于 IP）
│   └── cors.phprs               CORS 头部注入
├── controllers/                 MVC 控制器
│   ├── home_controller.phprs    默认路由（/, /about）
│   ├── db_controller.phprs      数据库 CRUD 示例
│   ├── redis_controller.phprs   Redis 键值操作示例
│   └── ws_controller.phprs      WebSocket 聊天/回声示例
├── views/
│   └── layout.phprs             HTML 布局 + 模板助手
└── data/                        文件存储
```

### 内置功能

- **速率限制** — 每 IP 每分钟 100 请求（可配置）
- **CORS** — 通配符来源，可配置方法与头部
- **JSON 文件数据库** — 完整 CRUD，自动生成 ID
- **键值存储** — Redis 风格的 get/set/del/keys
- **WebSocket** — 同一端口上的聊天和回声端点
- **类型化路由** — `/api/hello?name={any}&age={int}`

### API 接口示例

```
GET  /api/hello?name=张三&age=25   → JSON { message, name, age }
POST /api/user                       → JSON { name, email }（支持 JSON/表单）
POST /api/upload                     → JSON 上传回执
GET  /api/db/list                    → 记录列表
POST /api/db/create                  → 创建记录
POST /api/db/update                  → 更新记录
POST /api/db/delete?id=xxx           → 删除记录
POST /api/redis/set                  → 设置键值
GET  /api/redis/get?key=xxx          → 获取值
GET  /api/redis/keys                 → 列出所有键
GET  /api/ws/info                   → WebSocket 信息页面
WS   ws://localhost:8080/ws/chat     → 聊天端点
WS   ws://localhost:8080/ws/echo     → 回声端点
```

## 示例项目

| 目录 | 说明 |
|---|---|
| `examples/blog/` | 带路由和模板的博客 |
| `examples/binotes/` | 带 CRUD 的笔记应用 |
| `examples/websocket/` | WebSocket 回声服务器 |
| `examples/http_client/` | HTTP 客户端示例（GET/POST） |
| `examples/threaded/` | 多线程博客服务器 |

## 环境要求

- **Rust** 1.70+（编译编译器本身）
- **C 编译器**（Windows: MSVC，Linux/macOS: GCC/Clang）
- Windows: Visual Studio Build Tools
- Linux: `build-essential`
- macOS: Xcode Command Line Tools

## License

MIT
