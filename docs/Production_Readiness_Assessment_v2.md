# PHPRS 生产环境就绪度评估报告 (v2)

> 评估日期: 2026-05-08  
> 评估版本: master@878f056 (生产加固后)  
> 评估范围: PHPRS 编译器 + MVC 框架 + C 运行时  
> 目标场景: **API 服务 / WebSocket 长连接 / MySQL + Redis 交互**

---

## 总体评估

**综合评分: 6.5/10**（上次评估 4/10，提升 +2.5 分）

| 结论 | 说明 |
|------|------|
| ✅ **API 服务** | 基本可用于内部/低流量生产环境，高流量场景需补充 keep-alive 和连接池 |
| ⚠️ **WebSocket** | 有完整的 RFC 6455 实现，可用于简单场景；缺乏心跳管理和连接状态追踪 |
| ❌ **MySQL 交互** | **无真实驱动**，当前为文件模拟，不可用于生产 |
| ❌ **Redis 交互** | **无真实驱动**，当前为 JSON 文件模拟，不可用于生产 |

**结论: API 和 WebSocket 能力已初步达到生产标准；MySQL/Redis 是硬性阻断项，必须实现真实驱动后才能部署到目标生产环境。**

---

## 1. 评估维度总览

| 维度 | v1 评分 | v2 评分 | 变化 | 状态 |
|------|---------|---------|------|------|
| 性能 | 8/10 | 7/10 | -1 | ✅ 达标 (QPS 下降至 ~13K，因增加安全/日志开销) |
| 稳定性 | 5/10 | 6/10 | +1 | ⚠️ 改善 (优雅关闭、信号处理) |
| 内存安全 | 3/10 | 5/10 | +2 | ⚠️ 改善 (Arena 分配器、内存上限、有界操作) |
| 安全性 | 4/10 | 7/10 | +3 | ✅ 达标 (安全头、Body 限制、超时保护、SIGPIPE) |
| 可观测性 | 2/10 | 7/10 | +5 | ✅ 达标 (Access log、Error log、Health、Prometheus) |
| 错误处理 | 3/10 | 4/10 | +1 | ⚠️ 微改善 |
| 测试覆盖 | 3/10 | 3/10 | — | ❌ 未变 |
| 运维能力 | 2/10 | 7/10 | +5 | ✅ 达标 (优雅关闭、PID文件、配置管理、启动横幅) |
| 数据持久化 | 2/10 | 2/10 | — | ❌ 未变 (硬性阻断) |
| 生态成熟度 | 2/10 | 2/10 | — | ❌ 未变 |

---

## 2. 按目标场景逐项评估

### 2.1 API 服务 — 评分 7/10 ⚠️ 基本可用

#### 已具备能力

| 能力 | 实现情况 | 说明 |
|------|----------|------|
| HTTP 解析 | ✅ 完整 | GET/POST/PUT/DELETE，query params, headers, body 解析 |
| 路由分发 | ✅ 完整 | MVC 路由表，支持 path 参数和通配符 |
| JSON 处理 | ✅ 完整 | `json_encode`/`json_decode`，`phprs_json_get_string/int` |
| CORS 支持 | ✅ 完整 | 自动 `Access-Control-*` 头注入，OPTIONS 预检处理 |
| 安全头 | ✅ 新增 | X-Content-Type-Options, X-Frame-Options, X-XSS-Protection, Referrer-Policy |
| 请求限制 | ✅ 新增 | Body 大小限制 (10MB)，连接数限制 (10000)，读写超时 (30s/60s) |
| 并发模型 | ✅ 完整 | 8 线程池 + 队列（支持 4096 并发请求缓冲） |
| 日志 | ✅ 新增 | Access log（时间戳、IP、方法、路径、状态码、耗时、Request-ID） |
| 监控 | ✅ 新增 | `/health` (JSON)，`/metrics` (Prometheus 格式) |
| 优雅关闭 | ✅ 新增 | SIGTERM/SIGINT 处理，队列排空后退出 |
| HTTP 客户端 | ✅ 完整 | `phprs_curl()` — 同步+异步 HTTP 请求，支持 TLS |
| 密码学 | ✅ 基础 | SHA1, SHA256, Base64, HMAC (通过 OpenSSL) |

#### 缺失能力

| 缺失项 | 影响 | 紧迫度 |
|--------|------|--------|
| HTTP Keep-Alive | 每请求新建 TCP 连接，高并发下效率低 ~40% | 🔴 高 |
| HTTP/2 | 不支持多路复用，浏览器不能充分利用连接 | 🟡 中 |
| Rate Limiting | 无每 IP 速率限制，易被暴力攻击 | 🟡 中 |
| Request Validation | 无内建 schema 校验，需手动实现 | 🟡 中 |
| Session/JWT | 无内建会话管理或 JWT 签名验证 | 🟡 中 |
| gzip 压缩 | 不压缩响应体，带宽利用低 | 🟡 中 |
| 异步 I/O | 无 epoll/kqueue，blocking I/O | 🟠 中低 |

#### API 场景生产建议

```
✅ 适合: 内部微服务、管理后台 API、中小流量 API (<5K QPS)
⚠️ 慎用: 公网暴露的 API（需前置 Nginx/Caddy 做反代 + TLS 终端）
❌ 不适合: 高并发公共 API (>10K QPS)、需 HTTP/2 的场景
```

---

### 2.2 WebSocket — 评分 5/10 ⚠️ 有限可用

#### 已具备能力

| 能力 | 实现情况 | 说明 |
|------|----------|------|
| WebSocket 握手 | ✅ 完整 | RFC 6455 标准握手，SHA1 + Base64 的 Accept Key |
| 帧读写 | ✅ 完整 | 支持 Text(1), Close(8), Ping(9), Pong(10) 操作码 |
| Masking | ✅ 完整 | 正确处理客户端 masked frames |
| 扩展长度 | ✅ 完整 | 支持 126 (2字节) 和 127 (8字节) 扩展长度 |
| Payload 限制 | ✅ 有 | 1MB 上限，防止内存耗尽 |
| 关闭帧 | ✅ 有 | `ws_close()` 发送 Close 帧后 shutdown |
| 线程模型 | ✅ 有 | 每 WS 连接 `phprs_thread_spawn` 独立线程 |
| 框架封装 | ✅ 有 | `ws_accept`, `ws_read`, `ws_send_text`, `ws_disconnect` |
| 示例 | ✅ 有 | Echo 服务器、Chat 广播 |

#### 缺失能力

| 缺失项 | 影响 | 紧迫度 |
|--------|------|--------|
| 心跳机制 | 无自动 Ping/Pong，无法检测僵死连接 | 🔴 高 |
| 连接管理器 | 无全局连接列表，无法广播或按房间推送 | 🔴 高 |
| 每线程模型瓶颈 | 1000 WS 连接 = 1000 线程，可扩展性差 | 🔴 高 |
| Binary frames | 不支持 Binary (opcode 2) 帧类型 | 🟡 中 |
| 分片消息 | 不支持 FIN=0 的分片消息 | 🟡 中 |
| 子协议协商 | 不支持 Sec-WebSocket-Protocol | 🟠 低 |
| 压缩扩展 | 不支持 permessage-deflate | 🟠 低 |

#### 关键限制: 每连接一线程

当前 WebSocket 使用 `phprs_thread_spawn()` 为每个连接创建 **独立 POSIX 线程**：

```
Client1 → Thread1 (ws_handle_echo)
Client2 → Thread2 (ws_handle_chat)  
Client3 → Thread3 (ws_handle_chat)
...
ClientN → ThreadN
```

- Linux 默认线程栈 8MB，1000 连接 ≈ 8GB 虚拟内存
- macOS 默认线程栈 512KB，但 `pthread_create` 有 2048 线程硬限制
- **无法支撑高并发 WebSocket 场景**（需改为 epoll/kqueue + 事件驱动模型）

#### WebSocket 场景生产建议

```
✅ 适合: 内部管理面板实时通知 (<100 连接)
⚠️ 慎用: 小型聊天室 (<500 连接，需加心跳和连接管理)
❌ 不适合: 大规模推送 (>1000 连接)、实时游戏、在线协作编辑
```

---

### 2.3 MySQL 交互 — 评分 1/10 ❌ 不可用

#### 当前状态

**无真实 MySQL 驱动。** 当前 DB 操作使用 **JSON 文件模拟**：

```phprs
// 实际实现 — 文件读写，非 MySQL 连接
function db_load_all(): string {
    return file_get_contents("data/records.json");  // ← 读本地文件
}
function db_save_all(string $json): int {
    return file_put_contents("data/records.json", $json);  // ← 写本地文件
}
```

虽然提供了 MySQL 配置模板 (`config/database.phprs`)，但这些只是返回 JSON 配置字符串的函数，**不执行任何网络连接或 SQL 查询**。

#### 缺失能力

| 缺失项 | 说明 | 实现难度 |
|--------|------|----------|
| MySQL 协议 | 需实现 MySQL 客户端协议 (COM_QUERY, 认证握手等) | 🔴 高 |
| 连接池 | 需 N 条预建连接，从池中借用/归还 | 🔴 高 |
| Prepared Statements | 防 SQL 注入的参数化查询 | 🔴 高 |
| 事务管理 | BEGIN/COMMIT/ROLLBACK | 🟡 中 |
| 结果集解析 | 将 MySQL 行数据转换为 PHPRS 数据类型 | 🟡 中 |
| ORM 层 | 对象关系映射 (可选，但提升开发效率) | 🟠 低优先 |

#### 可行实现方案

**方案 A: 链接 libmysqlclient（推荐）**
- 使用 MySQL C API (`mysql_init`, `mysql_real_connect`, `mysql_query`, `mysql_store_result`)
- 在 `phprs_runtime.c` 中封装为 `phprs_mysql_*` 系列函数
- 编译时链接 `-lmysqlclient`
- 工作量: ~800 行 C 代码
- 优势: 成熟、稳定、完整功能

**方案 B: 纯 C 实现 MySQL 协议**
- 实现最小 MySQL 客户端协议子集
- 零外部依赖
- 工作量: ~2000 行 C 代码
- 优势: 无依赖；劣势: 工作量大，需处理认证插件

**方案 C: 通过 HTTP 代理**
- 通过 `phprs_curl()` 调用中间层 REST API（如 PocketBase, PostgREST）
- 不直接连接数据库
- 工作量: 0 行新代码
- 优势: 立即可用；劣势: 增加延迟和依赖

---

### 2.4 Redis 交互 — 评分 1/10 ❌ 不可用

#### 当前状态

**无真实 Redis 驱动。** 当前 Redis 操作使用 **JSON 文件模拟**：

```phprs
// 实际实现 — 本地 JSON 文件，非 Redis 协议
function kv_load(): string {
    return file_get_contents("data/kv_store.json");  // ← 读本地文件
}
function kv_save(string $json): int {
    return file_put_contents("data/kv_store.json", $json);  // ← 写本地文件
}
```

同样，`config/redis.phprs` 只是返回配置 JSON 的函数，**未执行任何 TCP 连接或 RESP 协议通信**。

#### 缺失能力

| 缺失项 | 说明 | 实现难度 |
|--------|------|----------|
| RESP 协议 | Redis 序列化协议 (简单文本协议) | 🟡 中 |
| TCP 连接管理 | `phprs_tcp_connect` 已存在，可复用 | 🟢 低 |
| 连接池 | 需预建连接池 + 线程安全借用 | 🟡 中 |
| 基础命令 | GET/SET/DEL/EXISTS/EXPIRE/TTL/INCR | 🟡 中 |
| Pipeline | 批量命令发送以减少 RTT | 🟡 中 |
| Pub/Sub | 发布订阅（WebSocket 场景常用） | 🔴 高 |
| Lua Scripting | EVAL 命令 | 🟠 低优先 |

#### 可行实现方案 (推荐: 纯 C 实现)

Redis RESP 协议非常简单，比 MySQL 协议简单 10 倍：

```
发送: *3\r\n$3\r\nSET\r\n$5\r\nmykey\r\n$7\r\nmyvalue\r\n
接收: +OK\r\n
```

**推荐方案: 在 `phprs_runtime.c` 中实现 RESP 客户端**
- 复用已有的 `phprs_tcp_connect()` 建立连接
- 实现 RESP 编码/解码 (~200 行 C)
- 连接池 (~200 行 C)
- 基础命令封装 (~300 行 C)
- 总工作量: ~700 行 C 代码
- 暴露 PHPRS 函数: `phprs_redis_connect`, `phprs_redis_get`, `phprs_redis_set`, `phprs_redis_del`, etc.

---

## 3. 安全性评估 (v2)

### 已实现的安全措施

| 措施 | 状态 | 说明 |
|------|------|------|
| 安全响应头 | ✅ 新增 | X-Content-Type-Options, X-Frame-Options, X-XSS-Protection, Referrer-Policy |
| 请求体大小限制 | ✅ 新增 | 默认 10MB，可配置 |
| 连接数限制 | ✅ 新增 | 默认 10000，超限返回 503 |
| 读写超时 | ✅ 新增 | SO_RCVTIMEO=30s, SO_SNDTIMEO=60s |
| SIGPIPE 保护 | ✅ 新增 | 忽略 SIGPIPE，防止写断管崩溃 |
| CRLF 注入防护 | ✅ 已有 | `strip_crlf()` 防止头注入 |
| CORS 保护 | ✅ 已有 | Access-Control 头正确设置 |
| 内存上限 | ✅ 新增 | 线程池总内存 512MB 上限 |
| Request ID | ✅ 新增 | X-Request-Id 用于追踪和关联 |

### 仍缺失的安全能力

| 缺失项 | 风险等级 | 说明 |
|--------|----------|------|
| TLS 终端 | 🔴 高 | 不支持 HTTPS，需前置反代 |
| Rate Limiting | 🔴 高 | 无 IP 级别速率限制 |
| SQL 注入防护 | 🔴 高 | 无 Prepared Statements（MySQL 驱动不存在） |
| XSS 输出转义 | 🟡 中 | 无自动 HTML 转义 |
| CSRF Token | 🟡 中 | 无 CSRF 令牌验证 |
| JWT 验证 | 🟡 中 | 无内建 JWT 签名/验证 |
| 输入 sanitize | 🟡 中 | 无通用输入清洗函数 |

---

## 4. 内存安全评估 (v2)

### 改善项

| 改善 | 说明 |
|------|------|
| Arena 分配器 | 每请求 256KB 堆栈分配器，请求结束自动回收 |
| 有界字符串操作 | `phprs_safe_strdup()` 限制最大长度 |
| 池内存追踪 | `phprs_pool_memory` 全局计数，超 512MB 拒绝新请求 |
| String Builder | `phprs_strbuf` 减少 malloc/realloc 频率 |

### 残留风险

| 风险 | 说明 |
|------|------|
| C 运行时 110 处 malloc，98 处 free | 不能保证一一对应，存在泄漏可能 |
| `setjmp/longjmp` 异常机制 | 跳过析构路径可能泄漏异常帧间分配的内存 |
| 无 Valgrind/ASAN CI | 没有自动化内存检查 |
| WebSocket 每连接线程 | 无内存上限控制 |

---

## 5. 可观测性评估 (v2)

### 已实现

| 能力 | 说明 |
|------|------|
| Access Log | `[timestamp] IP "METHOD /path" status bytes duration req=ID` |
| Error Log | 时间戳 + 错误分类（超时、队列满、内存超限） |
| Health Endpoint | `GET /health` → `{"status":"ok","uptime":N,"connections":N,"queue_depth":N}` |
| Metrics Endpoint | `GET /metrics` → Prometheus 格式（请求数、连接数、队列、内存） |
| Request ID | `X-Request-Id` 头注入，日志关联 |
| 启动横幅 | 打印端口、线程数、超时配置、PID |

### 仍缺失

| 缺失项 | 说明 |
|--------|------|
| 分布式追踪 | 无 OpenTelemetry / Trace ID 传播 |
| 按路由统计 | 无 p50/p95/p99 延迟直方图 |
| 告警集成 | 无 webhook/邮件告警 |
| 结构化日志 | 当前纯文本，非 JSON 格式 |

---

## 6. 运维能力评估 (v2)

### 已实现

| 能力 | 说明 |
|------|------|
| 优雅关闭 | SIGTERM/SIGINT → 停止 accept → 排空队列 → 退出 |
| PID 文件 | `phprs_write_pidfile()` 支持 systemd 管理 |
| JSON 配置 | `phprs_config()` 一次性配置端口、线程、超时等 |
| 启动横幅 | 完整配置信息输出 |

### 仍缺失

| 缺失项 | 说明 |
|--------|------|
| 热重载 | 无 SIGHUP 重载配置/代码 |
| 多进程模型 | 无 master-worker fork，单进程单故障点 |
| systemd 单元 | 无自动生成 `.service` 文件 |
| Docker 镜像 | 无 Dockerfile 模板 |

---

## 7. 与目标场景差距分析

### API + WebSocket + MySQL + Redis 部署架构

```
┌─────────────┐     ┌──────────────┐     ┌──────────┐
│   Nginx     │────→│  PHPRS App   │────→│  MySQL   │
│  (TLS终端)  │     │  (8 threads) │────→│  Redis   │
│  (反向代理) │     │  Port 8080   │     └──────────┘
└─────────────┘     └──────────────┘
                         │
                    WebSocket 连接
                    (独立线程处理)
```

### 差距矩阵

| 能力 | 现状 | 目标 | 差距 | 工作量估计 |
|------|------|------|------|-----------|
| API REST | ✅ 可用 | 生产 | 需 keep-alive + rate limiting | 3-5 天 |
| WebSocket | ⚠️ 基础可用 | 1000+ 连接 | 需事件驱动 + 连接管理器 | 5-7 天 |
| MySQL 驱动 | ❌ 不存在 | CRUD + 事务 | libmysqlclient 封装 | 5-7 天 |
| Redis 驱动 | ❌ 不存在 | GET/SET/Pub-Sub | RESP 协议实现 | 3-5 天 |
| 连接池 | ❌ 不存在 | MySQL + Redis | 线程安全池管理 | 3-5 天 |
| TLS | ❌ 服务端不支持 | HTTPS 终端 | 可由 Nginx 代理 | 0 天 (反代) |
| JWT | ❌ 不存在 | Token 验证 | HMAC-SHA256 签名 | 2-3 天 |

**总估计工作量: 21-32 天（如实现全部目标）**

---

## 8. 生产部署建议

### 8.1 最小可行部署 (MVP) — 预计 10-14 天

优先实现以下项目即可进入受控生产环境：

1. **Redis RESP 客户端** (3-5 天) — 可立即解锁缓存、Session、队列能力
2. **MySQL libmysqlclient 封装** (5-7 天) — 解锁持久化存储能力
3. **连接池** (2-3 天) — MySQL 和 Redis 各维护 N 条预建连接

### 8.2 部署架构建议

```nginx
# Nginx 反向代理配置 (处理 TLS + 静态文件)
upstream phprs_backend {
    server 127.0.0.1:8080;
    keepalive 32;  # Nginx 到 PHPRS 的长连接池
}

server {
    listen 443 ssl http2;
    ssl_certificate /path/to/cert.pem;
    
    # WebSocket 支持
    location /ws/ {
        proxy_pass http://phprs_backend;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
    }
    
    # API 请求
    location /api/ {
        proxy_pass http://phprs_backend;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

### 8.3 监控接入

PHPRS 已提供 Prometheus 格式 `/metrics` 端点，可直接接入：

```yaml
# prometheus.yml
scrape_configs:
  - job_name: 'phprs'
    static_configs:
      - targets: ['127.0.0.1:8080']
    metrics_path: /metrics
    scrape_interval: 15s
```

---

## 9. 结论

### 评分汇总

| 场景 | 评分 | 可否部署 | 条件 |
|------|------|----------|------|
| API 服务 | **7/10** | ⚠️ 有条件可部署 | 需前置 Nginx 反代 + TLS |
| WebSocket | **5/10** | ⚠️ 小规模可用 | <500 连接，需加心跳机制 |
| MySQL 交互 | **1/10** | ❌ 不可部署 | 需实现真实驱动 |
| Redis 交互 | **1/10** | ❌ 不可部署 | 需实现 RESP 协议客户端 |
| **综合** | **6.5/10** | ❌ 暂不可用 | MySQL/Redis 是硬性阻断项 |

### 与 v1 (4/10) 对比改进

- ✅ 安全性: +3 (安全头、超时、连接限制、SIGPIPE)
- ✅ 可观测性: +5 (日志、健康检查、Prometheus 指标)
- ✅ 运维: +5 (优雅关闭、PID 文件、配置管理)
- ✅ 内存安全: +2 (Arena 分配器、内存上限)
- ❌ 数据持久化: 未变 (仍为文件模拟)

### 下一步优先级

| 优先级 | 任务 | 工期 | 解锁能力 |
|--------|------|------|----------|
| P0 | Redis RESP 客户端 | 3-5 天 | 缓存、Session、队列 |
| P0 | MySQL 驱动 (libmysqlclient) | 5-7 天 | 持久化存储、CRUD |
| P1 | 连接池 (MySQL + Redis) | 2-3 天 | 高并发数据库访问 |
| P1 | HTTP Keep-Alive | 2-3 天 | API 吞吐量提升 40%+ |
| P2 | WebSocket 连接管理器 + 心跳 | 3-4 天 | 可靠 WS 长连接 |
| P2 | JWT 签名/验证 | 2 天 | API 认证 |
| P3 | epoll/kqueue 事件驱动 | 7-10 天 | 万级 WS 并发 |

---

*评估人: PHPRS 开发团队*  
*评估方法: 代码审计 + 压力测试 + 功能验证*
