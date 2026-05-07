# PHPRS 编译模式性能分析：为何与 Go/Rust 仍有差距

> 分析日期: 2025-07  
> 对比对象: Go `net/http` (~50,000 req/s), Rust `actix-web` (~100,000+ req/s)  
> PHPRS 编译模式实测: ~27,700 req/s (峰值), ~14,000 req/s (c=1000)

---

## 1. 差距量化

```
框架                  QPS (c=10)    QPS (c=100)    QPS (c=1000)
────────────────────────────────────────────────────────────────
Rust actix-web        80,000-150K   100,000-200K   80,000-150K
Go net/http           30,000-60K    40,000-80K     30,000-60K
PHPRS 编译模式         ~27,700       ~22,000        ~14,000
────────────────────────────────────────────────────────────────
vs Go:                 0.5-0.9x      0.3-0.6x       0.2-0.5x
vs Rust:               0.2-0.3x      0.1-0.2x       0.1-0.2x
```

PHPRS 编译后在低并发 (c=10) 接近 Go 下限，但**高并发时差距迅速拉大**。核心原因分为两大类：**架构瓶颈**和**代码生成效率**。

---

## 2. 架构级瓶颈（影响最大）

### 2.1 单线程串行 accept 循环 — 最关键瓶颈

```c
// PHPRS 生成的主循环 (output.c:6290)
for (int64_t v_274 = 1LL; (v_274 == 1LL); ) {
    int64_t v_275 = phprs_server_accept(v_272);  // 阻塞
    const char* v_276 = phprs_socket_read(v_275, 65536LL);
    // ... 处理请求 ...
    phprs_socket_write(v_275, response);
    phprs_socket_close(v_275);
}
```

**问题**: 整个服务器是一个单线程的 `accept → read → process → write → close` 循环。所有请求**串行处理**，无法利用 M2 的 8 个核心。

**Go 的做法**:
```go
for {
    conn, _ := listener.Accept()
    go handleConn(conn)  // 每个连接一个 goroutine，由 runtime 调度到多核
}
```
Go 的 goroutine 在 M2 上可以并行处理 8 个请求；PHPRS 只能处理 1 个。

**Rust actix-web**:
- 默认启动 CPU 数量个 worker 线程
- 每个 worker 运行独立的 epoll/kqueue 事件循环
- 单个 worker 可通过异步 I/O 处理数千并发连接

**量化影响**: 这是 PHPRS 在高并发下 QPS 不增反降的根本原因。理论上限 = 1/请求处理时间。单连接 ~11,000 req/s（即单请求 ~90μs），但并发增加后因排队导致延迟上升，而吞吐量无法超过单核极限。

**预期收益**: 如果实现多线程 accept（8 线程），理论 QPS 可从 ~27K 提升到 **100K-200K**。

### 2.2 listen backlog = 10

```c
// output.c:480
listen(sock, 10);  // backlog 仅 10！
```

Go 默认 backlog = `SOMAXCONN`（macOS 上 128，Linux 上通常 4096）。当并发 1000 时，PHPRS 的 backlog 队列极小，OS 需要频繁 SYN-retry，连接建立延迟增加。

**预期收益**: 改为 `listen(sock, 4096)` 可改善高并发下的连接建立延迟，但不提升单核 QPS。

### 2.3 无 Keep-Alive 支持

PHPRS 每个请求完整走 `TCP 三次握手 → 请求 → 响应 → close` 流程。Go/Rust 框架默认支持 HTTP keep-alive，单 TCP 连接复用处理多个请求。

**开销对比**:
```
                    PHPRS           Go/Rust
每请求 syscall:     accept+read+    read+write (连接复用)
                    write+close
TCP 握手:           每请求一次       首次一次
```

Apache Bench 默认不使用 keep-alive（`-k` 标志开启），所以在本次测试中影响不大。但在真实生产环境中，keep-alive 可减少 30-50% 的系统调用开销。

---

## 3. 代码生成效率问题

### 3.1 疯狂的堆分配 — 最大的代码生成缺陷

PHPRS 所有字符串操作都通过 `malloc` 在堆上分配新内存，且**从不释放**（依赖 OS 在进程退出时回收）。

**一次 GET / 请求的 malloc 链**:

```c
// HTTP 解析
phprs_copy_until(raw, ' ')        → malloc (method: "GET")
phprs_copy_until(after, ' ')      → malloc (path: "/")
phprs_http_header(raw, "Host")    → malloc (strdup)
phprs_http_header(raw, "User-Agent") → malloc
phprs_http_header(raw, "Accept")  → malloc
phprs_http_header(raw, "Content-Type") → malloc
phprs_http_header(raw, "Content-Length") → malloc
phprs_http_header(raw, "Referer") → malloc

// request_server() 构建 SERVER 变量
__concat(str1, str2)              → malloc × 12+ 次

// route_match() — 遍历 15 条路由
phprs_str_split(routes, "\n", 0)  → malloc
phprs_str_contains(...)           → (no alloc, good)
phprs_str_split(entry, "|", 0)    → malloc
phprs_str_split(entry, "|", 1)    → malloc
// ... 每条路由 3-5 次 malloc，15 条 = 45-75 次 malloc

// CSRF 检查 (每个请求都会调 csrf_token → session_start)
// session_valid_id() — 64 次 phprs_str_replace 调用
// 每次 str_replace: count+malloc+memcpy = 64 次循环!
```

**保守估算**: 一个简单的 GET / 请求至少触发 **100-200 次 malloc**，且**没有任何 free**。

**Go 的做法**: `net/http` 使用 `sync.Pool` 复用 buffer，HTTP 解析使用栈上 slice 切割（零拷贝），路由使用 radix tree（O(path_len) 无分配）。

**Rust 的做法**: actix-web 使用 `Bytes`（引用计数零拷贝），请求头通过 `httparse` 在栈上解析（零分配），路由使用前缀树。

**量化影响**: 在 M2 上，单次 `malloc` 约 20-50ns，100 次 = 2-5μs。对于 90μs 的请求处理时间，这意味着 **2-5% 开销来自纯分配器**。但更严重的是 cache 污染 — 频繁 malloc 导致 L1/L2 cache 频繁 miss。

### 3.2 字符串作为万能数据结构 — O(N) 无处不在

PHPRS 的路由表、SERVER 变量、session 数据全部用 `&` 分隔的字符串存储：

```c
// 路由表是一个巨大的字符串
"GET|/\nhome_index\nGET|/about\nhome_about\n..."

// SERVER 变量
"REQUEST_METHOD=GET&REQUEST_URI=/&QUERY_STRING=&HTTP_HOST=..."
```

查找任何值都需要 **O(N) 线性扫描 + 字符串分割**:

```c
// request_param() — 查找某个字段
for (int64_t v_7 = 0; v_7 < 100; v_7++) {
    const char* v_8 = phprs_str_split(params, "&", v_7);  // O(N) 每次!
    if (phprs_str_starts_with(v_8, search)) {              // 再 O(M)
        return phprs_url_decode(phprs_str_replace(v_8, search, ""));
    }
}
```

`phprs_str_split(s, "&", i)` 本身是 O(i×len) — 它每次从头扫描到第 i 个分隔符。在循环中调用意味着总复杂度 O(N²)。

**Go 对比**: `http.Request.Header` 是 `map[string][]string`（O(1) 查找），路由使用 ServeMux（Go 1.22+ 用 radix tree）。

**量化影响**: 每请求路由匹配遍历 15 条路由，每条路由 ~5 次 str_split → O(15×5×avg_len) ≈ O(7500) 字符操作。用哈希表可降到 O(1)。

### 3.3 session_valid_id() — 每个 GET 请求 64 次 str_replace

CSRF 检查在 **每个请求** 都会执行（即使是 GET 请求也会调 `csrf_token → session_start → session_valid_id`）：

```c
// output.c:4983-5040 — 64 次 phprs_str_replace 调用
v_29 = phprs_str_replace(v_29, "a", "");
v_29 = phprs_str_replace(v_29, "b", "");
// ... 重复 64 次
```

每次 `phprs_str_replace`:
1. `strlen(s)` — O(N)
2. `strstr(s, from)` 遍历 — O(N)  
3. `malloc(result_len)` — 堆分配
4. 复制整个字符串 — O(N)

对一个 37 字符的 session ID 做 64 次 replace: **~64 × (3×37 + malloc) ≈ 7,000+ 操作 + 64 次 malloc**。

**Go/Rust 等价实现**: 一个简单的 `for range` 循环检查每个 byte 是否在 `[a-zA-Z0-9_-]` 中，O(N) 一次遍历，零分配。

### 3.4 路由匹配 O(N) 线性扫描

```c
// route_match() — 遍历所有路由
for (int64_t v_115 = 0; v_115 < v_113; v_115++) {
    const char* v_116 = route_entry_get(routes, v_115);  // 从头扫描字符串
    const char* v_117 = route_match_one(method, url, v_116);
    if (strcmp(v_117, "") != 0) return v_117;
}
```

15 条路由，最坏情况（如 `/nonexistent`）需要匹配全部 15 条才返回 404。

**Go/Rust**: 使用前缀树 (radix tree) 或哈希表，路由查找 O(path_length)，与路由数量无关。

### 3.5 每次写 socket 都检查 TLS 上下文

```c
// phprs_socket_write() — 每次写入都线性扫描 TLS 表
int64_t phprs_socket_write(int64_t fd, const char* data) {
    phprs_tls_ctx* tls = phprs_tls_find(fd);  // O(N) 线性扫描
    ...
}
```

即使不使用 TLS，每次 `socket_write` 也要遍历 TLS entries 数组。

---

## 4. 系统调用效率

### 4.1 每请求 syscall 数对比

```
PHPRS (每请求):                    Go net/http (keep-alive):
1. accept()                        (连接已建立)
2. read()                          1. epoll_wait() / kqueue()
3. write()                         2. read()
4. close()                         3. writev() (可能 sendfile)
                                   (不 close，复用连接)
合计: 4 syscall/req                合计: 2-3 syscall/req
```

### 4.2 阻塞 I/O vs 事件驱动

PHPRS 使用阻塞 `accept()` + `read()` + `write()`。在等待 I/O 时，线程完全阻塞，无法处理其他请求。

Go: goroutine + netpoll (epoll/kqueue 包装)，在 I/O 等待时自动切换到其他 goroutine。  
Rust: tokio runtime + epoll/kqueue，单线程可管理数千并发连接。

---

## 5. 内存管理对比

```
                    PHPRS           Go              Rust
分配策略:           malloc/从不free   GC (三色标记)    栈优先+引用计数/移动语义
每请求分配:         100-200 次       5-20 次          0-5 次 (大部分栈上)
分配来源:           全部堆上          escape analysis  大部分栈上
内存释放:           永不释放          GC 批量回收      编译期确定性释放
cache 友好性:       差 (碎片化)       中等             好 (连续内存)
```

PHPRS 的 "从不 free" 策略意味着:
- **短期无害**: 7.3MB RSS 处理 100K 请求，实际很小
- **长期问题**: 生产运行 24h+ 会持续增长
- **性能影响**: 大量小对象分散在堆中，cache 效率低

---

## 6. 具体优化路径与预期收益

| 优化项 | 难度 | 预期 QPS 提升 | 说明 |
|--------|------|--------------|------|
| **多线程 accept (8 workers)** | 高 | **4-8x** → 100K+ | 最关键，利用多核 |
| **epoll/kqueue 事件驱动** | 高 | **2-3x** (在多线程基础上) | 单线程管理多连接 |
| **Keep-Alive** | 中 | **1.3-1.5x** | 减少 TCP 握手/close |
| **哈希路由表** | 低 | **1.1-1.3x** | 消除线性扫描 |
| **栈分配/arena 分配** | 中 | **1.2-1.5x** | 减少 malloc 碎片 |
| **零拷贝 HTTP 解析** | 中 | **1.3-1.5x** | 返回 slice 而非 strdup |
| **修复 session_valid_id** | 极低 | **1.05-1.1x** | 一行 C 代码替代 64 次 str_replace |
| **listen backlog = 4096** | 极低 | **1.02-1.05x** (高并发) | 一行改动 |
| **消除 TLS 查找** | 低 | **1.01-1.02x** | 非 TLS 时跳过 |

**组合预期**:
- 仅多线程: ~27K × 6 = **~160K req/s** (接近 Rust 级)
- 多线程 + 事件驱动 + Keep-Alive: **~200K-400K req/s** (超越 Go)
- 全部优化: **~300K-500K req/s** (接近 Rust 裸框架)

---

## 7. 根本原因总结

```
┌───────────────────────────────────────────────────────────┐
│              影响程度排序 (从大到小)                        │
├───────────────────────────────────────────────────────────┤
│ 1. 单线程串行处理 (无法利用多核)          — 4-8x 差距     │
│ 2. 无 epoll/kqueue (阻塞 I/O)           — 2-3x 差距     │
│ 3. 疯狂的 malloc (100+ 次/请求, 无 free) — 1.3-1.5x 差距 │
│ 4. 字符串作为数据结构 (O(N²) 查找)       — 1.2-1.3x 差距 │
│ 5. 无 Keep-Alive                        — 1.3x 差距     │
│ 6. 低效算法 (64 次 str_replace)          — 1.05x 差距    │
│ 7. listen backlog 太小                   — 微小          │
├───────────────────────────────────────────────────────────┤
│ 总计: ~27K vs Go ~50K (1.8x) / Rust ~150K (5.4x)        │
└───────────────────────────────────────────────────────────┘
```

**本质差异**: Go/Rust 的高性能不仅来自编译优化，更来自**架构设计** — 多核并行、事件驱动 I/O、零拷贝数据处理。PHPRS 当前只做了第一步（编译为原生代码），但网络架构仍是最朴素的单线程阻塞模型。

---

## 8. 结论

PHPRS 编译模式 ~27,000 req/s 对于**单线程阻塞 I/O 服务器**已经是接近理论极限的表现。差距主要不在"编译质量"，而在:

1. **并发模型**: 单线程 vs 多线程/协程 (4-8x 差距)
2. **I/O 模型**: 阻塞 vs 事件驱动 (2-3x 差距)  
3. **数据结构**: 字符串 vs 哈希表/树 (1.2-1.5x 差距)
4. **内存管理**: malloc 泛滥 vs 栈/pool 分配 (1.3x 差距)

如果优先实现**多线程 accept + worker pool**（难度可控），PHPRS 编译模式可立即达到 Go 级别 (~100K req/s)。后续加入 epoll/kqueue 和 keep-alive 可进一步接近 Rust 级别。
