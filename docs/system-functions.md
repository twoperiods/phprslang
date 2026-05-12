# PHPRS System Functions Reference

This document covers all system-level built-in functions added to phprs for daemon/service development, network tool development, and CLI tool development.

## Table of Contents

- [Process Execution](#process-execution)
- [Stdin / Environment / Exit](#stdin--environment--exit)
- [Daemonize / Signal](#daemonize--signal)
- [UDP Socket](#udp-socket)
- [Enhanced Filesystem](#enhanced-filesystem)
- [Raw Socket Enhancements](#raw-socket-enhancements)

---

## Process Execution

### `phprs_exec(string $cmd): string`

Execute a shell command and return its stdout output (trimmed of trailing newlines).

```phprs
let $output = phprs_exec("ls -la /tmp");
echo $output . "\n";

let $date = phprs_exec("date +%Y-%m-%d");
echo "Today: " . $date . "\n";
```

### `phprs_shell_exec(string $cmd): string`

Alias for `phprs_exec()`. Executes a shell command and returns stdout.

```phprs
let $result = phprs_shell_exec("whoami");
echo "User: " . $result . "\n";
```

### `phprs_system(string $cmd): int`

Execute a shell command, print its stdout directly, and return the exit code.

```phprs
let $code = phprs_system("make build");
if ($code != 0) {
    echo "Build failed with code: " . $code . "\n";
}
```

### `phprs_popen(string $cmd, string $mode): int`

Open a process pipe. Returns a handle (fd) for reading/writing, or -1 on failure.

- `$mode = "r"` — read from the process stdout
- `$mode = "w"` — write to the process stdin

```phprs
let $fd = phprs_popen("sort", "w");
// write data to the process...
let $exit = phprs_pclose($fd);
```

### `phprs_pclose(int $handle): int`

Close a pipe opened with `phprs_popen()` and return the process exit code.

```phprs
let $fd = phprs_popen("grep error /var/log/app.log", "r");
let $exit_code = phprs_pclose($fd);
```

### `phprs_getpid(): int`

Return the current process ID.

```phprs
let $pid = phprs_getpid();
echo "PID: " . $pid . "\n";
```

### `phprs_posix_kill(int $pid, int $signal): int`

Send a signal to a process. Returns 1 on success, 0 on failure. On Windows, terminates the process.

```phprs
let $child_pid = 12345;
phprs_posix_kill($child_pid, 15);  // SIGTERM
```

---

## Stdin / Environment / Exit

### `phprs_readline(string $prompt): string`

Display a prompt and read one line from stdin (newline stripped).

```phprs
let $name = phprs_readline("Enter your name: ");
echo "Hello, " . $name . "\n";
```

### `phprs_fgets_stdin(): string`

Read one line from stdin without displaying a prompt (newline stripped).

```phprs
// Useful for piped input: echo "data" | ./myprog
let $line = phprs_fgets_stdin();
echo "Got: " . $line . "\n";
```

### `phprs_stdin_read(int $bytes): string`

Read up to `$bytes` raw bytes from stdin. Useful for binary input or fixed-size reads.

```phprs
let $data = phprs_stdin_read(1024);
echo "Read " . strlen($data) . " bytes\n";
```

### `phprs_stdin_eof(): int`

Check if stdin has reached end-of-file. Returns 1 if EOF, 0 otherwise.

```phprs
while (!phprs_stdin_eof()) {
    let $line = phprs_fgets_stdin();
    echo "Processing: " . $line . "\n";
}
```

### `phprs_getenv(string $name): string`

Get the value of an environment variable. Returns empty string if not set.

```phprs
let $home = phprs_getenv("HOME");
let $path = phprs_getenv("PATH");
let $db_url = phprs_getenv("DATABASE_URL");

if (strlen($db_url) == 0) {
    echo "ERROR: DATABASE_URL not set\n";
    phprs_exit(1);
}
```

### `phprs_putenv(string $pair): int`

Set an environment variable using "KEY=VALUE" format. Returns 1 on success, 0 on failure.

```phprs
phprs_putenv("APP_MODE=production");
phprs_putenv("LOG_LEVEL=debug");
```

### `phprs_exit(int $code): void`

Terminate the process immediately with the given exit code.

```phprs
if ($error) {
    echo "Fatal error\n";
    phprs_exit(1);
}
phprs_exit(0);
```

---

## Daemonize / Signal

### `phprs_daemonize(): int`

Fork the process into a background daemon. Returns 0 on success, -1 on failure. On Windows, always returns -1 (not supported).

This function:
1. Forks and exits the parent
2. Creates a new session (setsid)
3. Forks again
4. Redirects stdin/stdout/stderr to /dev/null

```phprs
phprs_write_pidfile("/var/run/myapp.pid");
let $ret = phprs_daemonize();
if ($ret < 0) {
    echo "Failed to daemonize\n";
    phprs_exit(1);
}
// Now running as a background daemon
```

### `phprs_signal(int $signal, string $action): void`

Register a signal handler. Actions:
- `"ignore"` — ignore the signal (SIG_IGN)
- `"default"` — restore default behavior (SIG_DFL)
- `"flag"` — set a flag that can be checked with `phprs_signal_check()`

```phprs
// Graceful shutdown on SIGTERM
phprs_signal(15, "flag");  // 15 = SIGTERM

// Ignore SIGHUP
phprs_signal(1, "ignore");

// Main loop
while (!phprs_signal_check(15)) {
    // do work...
    sleep(1);
}
echo "Shutting down gracefully\n";
```

### `phprs_signal_check(int $signal): int`

Check if a signal flag has been set (by a handler registered with `"flag"` action). Returns 1 if the signal was received, 0 otherwise.

```phprs
if (phprs_signal_check(15)) {
    echo "SIGTERM received, cleaning up...\n";
}
```

### `phprs_signal_clear(int $signal): void`

Clear a signal flag after handling it.

```phprs
if (phprs_signal_check(2)) {  // SIGINT
    phprs_signal_clear(2);
    echo "Caught Ctrl+C, continuing...\n";
}
```

### `phprs_setuid(int $uid): int`

Drop privileges by changing the effective user ID. Returns 1 on success, 0 on failure. Unix only (returns 0 on Windows).

```phprs
// Start as root, bind to port 80, then drop privileges
let $server = phprs_server_new(80);
phprs_setuid(1000);  // Switch to unprivileged user
```

### `phprs_chroot(string $path): int`

Change the root directory (chroot jail). Returns 1 on success, 0 on failure. Unix only.

```phprs
phprs_chroot("/var/www/app");
// Process can now only access files under /var/www/app
```

---

## UDP Socket

### `phprs_udp_socket(): int`

Create a new UDP socket. Returns a file descriptor, or -1 on failure.

```phprs
let $sock = phprs_udp_socket();
if ($sock < 0) {
    echo "Failed to create UDP socket\n";
    phprs_exit(1);
}
```

### `phprs_udp_bind(int $fd, int $port): int`

Bind a UDP socket to a port. Returns 0 on success, -1 on failure.

```phprs
let $sock = phprs_udp_socket();
let $ret = phprs_udp_bind($sock, 9999);
if ($ret < 0) {
    echo "Failed to bind to port 9999\n";
}
```

### `phprs_udp_sendto(int $fd, string $data, string $host, int $port): int`

Send data to a specific host:port via UDP. Returns bytes sent, or -1 on failure.

```phprs
let $sock = phprs_udp_socket();
let $sent = phprs_udp_sendto($sock, "hello", "127.0.0.1", 9999);
echo "Sent " . $sent . " bytes\n";
```

### `phprs_udp_recvfrom(int $fd, int $maxsize): string`

Receive data from a UDP socket. Returns a JSON string with fields `data`, `host`, and `port`.

```phprs
let $sock = phprs_udp_socket();
phprs_udp_bind($sock, 9999);

// Blocking receive (up to 4096 bytes)
let $result = phprs_udp_recvfrom($sock, 4096);
// $result = {"data":"hello","host":"127.0.0.1","port":54321}

let $data = phprs_json_get_string($result, "data");
let $from_host = phprs_json_get_string($result, "host");
let $from_port = phprs_json_get_int($result, "port");
echo "Received from " . $from_host . ":" . $from_port . ": " . $data . "\n";
```

### `phprs_udp_close(int $fd): void`

Close a UDP socket.

```phprs
phprs_udp_close($sock);
```

### Complete UDP Echo Server Example

```phprs
let $sock = phprs_udp_socket();
phprs_udp_bind($sock, 9999);
echo "UDP echo server listening on port 9999\n";

phprs_signal(15, "flag");

while (!phprs_signal_check(15)) {
    let $msg = phprs_udp_recvfrom($sock, 4096);
    let $data = phprs_json_get_string($msg, "data");
    let $host = phprs_json_get_string($msg, "host");
    let $port = phprs_json_get_int($msg, "port");

    echo "From " . $host . ":" . $port . " -> " . $data . "\n";
    phprs_udp_sendto($sock, $data, $host, $port);
}

phprs_udp_close($sock);
echo "Server stopped\n";
```

---

## Enhanced Filesystem

### `phprs_chdir(string $path): int`

Change the current working directory. Returns 1 on success, 0 on failure.

```phprs
phprs_chdir("/var/log");
let $cwd = getcwd();
echo "Now in: " . $cwd . "\n";
```

### `phprs_rmdir(string $path): int`

Remove an empty directory. Returns 1 on success, 0 on failure.

```phprs
let $ok = phprs_rmdir("/tmp/myapp_cache");
if (!$ok) {
    echo "Failed to remove directory (not empty?)\n";
}
```

### `phprs_glob(string $pattern): string`

Find files matching a glob pattern. Returns a JSON array of matching paths.

```phprs
let $files = phprs_glob("/var/log/*.log");
echo "Log files: " . $files . "\n";
// Output: ["/var/log/app.log","/var/log/error.log"]

let $configs = phprs_glob("/etc/myapp/*.conf");
```

### `phprs_chmod(string $path, int $mode): int`

Change file permissions (Unix only). Returns 1 on success, 0 on failure.

```phprs
// Make executable
phprs_chmod("./script.sh", 755);

// Read-only
phprs_chmod("./config.json", 444);
```

### `phprs_tempnam(string $dir, string $prefix): string`

Create a temporary file and return its path.

```phprs
let $tmp = phprs_tempnam("/tmp", "myapp_");
phprs_file_append($tmp, "temporary data\n");
// ... use the file ...
unlink($tmp);  // cleanup
```

### `phprs_file_append(string $path, string $data): int`

Append data to a file (creates the file if it doesn't exist). Returns bytes written, or -1 on failure.

```phprs
// Append to a log file
phprs_file_append("/var/log/myapp.log", "[2024-01-01] Event occurred\n");

// Build a file incrementally
phprs_file_append("output.csv", "name,age,city\n");
phprs_file_append("output.csv", "Alice,30,NYC\n");
phprs_file_append("output.csv", "Bob,25,LA\n");
```

---

## Raw Socket Enhancements

These functions extend the existing TCP socket API (`phprs_tcp_connect`, `phprs_socket_read`, `phprs_socket_write`, `phprs_socket_close`) with additional capabilities for implementing custom protocols.

### `phprs_socket_set_timeout(int $fd, int $seconds): void`

Set read and write timeout on a socket.

```phprs
let $fd = phprs_tcp_connect("example.com", 80);
phprs_socket_set_timeout($fd, 5);  // 5 second timeout
```

### `phprs_socket_read_bytes(int $fd, int $n): string`

Read exactly N bytes from a socket (blocks until all bytes are received or connection closes).

```phprs
// Read a fixed-size protocol header
let $header = phprs_socket_read_bytes($fd, 4);
let $payload_len = ord(substr($header, 0, 1)) * 256 + ord(substr($header, 1, 1));
let $payload = phprs_socket_read_bytes($fd, $payload_len);
```

### `phprs_socket_peek(int $fd, int $n): string`

Peek at data in the socket buffer without consuming it. The data remains available for subsequent reads.

```phprs
// Peek to determine protocol type
let $first_bytes = phprs_socket_peek($fd, 4);
if (str_starts_with($first_bytes, "HTTP")) {
    // Handle HTTP
} else {
    // Handle custom protocol
}
```

### `phprs_socket_available(int $fd): int`

Return the number of bytes available to read without blocking.

```phprs
let $avail = phprs_socket_available($fd);
if ($avail > 0) {
    let $data = phprs_socket_read($fd, $avail);
    // process data...
}
```

### Complete Custom Protocol Example

```phprs
// Simple length-prefixed protocol client
function send_message(int $fd, string $msg): int {
    let $len = strlen($msg);
    let $header = chr($len / 256) . chr($len % 256);
    phprs_socket_write($fd, $header . $msg);
    return $len;
}

function recv_message(int $fd): string {
    let $header = phprs_socket_read_bytes($fd, 2);
    let $len = ord(substr($header, 0, 1)) * 256 + ord(substr($header, 1, 1));
    return phprs_socket_read_bytes($fd, $len);
}

let $fd = phprs_tcp_connect("myserver.local", 5000);
phprs_socket_set_timeout($fd, 10);

send_message($fd, "HELLO");
let $response = recv_message($fd);
echo "Server says: " . $response . "\n";

phprs_socket_close($fd);
```

---

## Complete Examples

### CLI Tool: File Search

```phprs
// Usage: ./filesearch <pattern> <directory>
let $pattern = $argv[1];
let $dir = $argv[2];

if ($argc < 3) {
    echo "Usage: filesearch <pattern> <directory>\n";
    phprs_exit(1);
}

let $files = phprs_glob($dir . "/*");
let $decoded = json_decode($files);
// Process files...

echo "Searching for '" . $pattern . "' in " . $dir . "\n";
let $results = phprs_exec("grep -rl '" . $pattern . "' " . $dir);
echo $results . "\n";
```

### System Daemon

```phprs
// Simple daemon that monitors a directory
phprs_signal(15, "flag");
phprs_signal(2, "flag");

log_init("/var/log/monitor.log");
phprs_write_pidfile("/var/run/monitor.pid");

let $ret = phprs_daemonize();
if ($ret < 0) {
    log_error("Failed to daemonize");
    phprs_exit(1);
}

log_info("Monitor daemon started");

while (!phprs_signal_check(15)) {
    let $files = phprs_glob("/var/spool/incoming/*");
    if (strlen($files) > 2) {  // not just "[]"
        log_info("New files detected: " . $files);
        phprs_exec("process_files.sh");
    }
    sleep(5);
}

log_info("Monitor daemon stopping");
phprs_exit(0);
```

### Network Tool: Port Scanner

```phprs
// Simple TCP port scanner
let $host = $argv[1];
if ($argc < 2) {
    echo "Usage: portscan <host>\n";
    phprs_exit(1);
}

echo "Scanning " . $host . "...\n";

let $ports = [22, 80, 443, 3306, 5432, 6379, 8080, 8443];
for (let $i = 0; $i < count($ports); $i = $i + 1) {
    let $port = $ports[$i];
    let $fd = phprs_tcp_connect($host, $port);
    if ($fd >= 0) {
        echo "  Port " . $port . ": OPEN\n";
        phprs_socket_close($fd);
    }
}

echo "Scan complete\n";
```

### Network Tool: DNS Lookup

```phprs
let $hostname = $argv[1];
if ($argc < 2) {
    echo "Usage: dnslookup <hostname>\n";
    phprs_exit(1);
}

let $ip = phprs_dns_resolve($hostname);
if (strlen($ip) > 0) {
    echo $hostname . " -> " . $ip . "\n";
} else {
    echo "Failed to resolve: " . $hostname . "\n";
    phprs_exit(1);
}
```

### Interactive CLI Tool

```phprs
echo "PHPRS Calculator\n";
echo "Type 'quit' to exit\n\n";

while (1) {
    let $input = phprs_readline("calc> ");
    if ($input == "quit") {
        echo "Bye!\n";
        phprs_exit(0);
    }
    let $result = phprs_exec("echo '" . $input . "' | bc");
    echo "= " . $result . "\n";
}
```

---

## Platform Notes

| Function | Linux/macOS | Windows |
|----------|-------------|---------|
| `phprs_exec` / `phprs_system` | Uses `sh -c` | Uses `cmd /C` |
| `phprs_daemonize` | Full support (fork/setsid) | Returns -1 (not supported) |
| `phprs_setuid` | Full support | Returns 0 (no-op) |
| `phprs_chroot` | Full support | Returns 0 (no-op) |
| `phprs_chmod` | Full support | Returns 0 (no-op) |
| `phprs_posix_kill` | Sends signal via kill() | Terminates process |
| UDP functions | Full support | Full support (Winsock) |
| `phprs_glob` | POSIX glob() | FindFirstFile/FindNextFile |

## Interpreter vs Compiled Mode

All functions work in both modes. In interpreter mode (`phprs run`):
- `phprs_exec`, `phprs_system`, `phprs_popen` print a warning on first use
- `phprs_daemonize` is a no-op (returns -1)
- `phprs_signal` / `phprs_signal_check` are stubs (signal always returns 0)
- `phprs_setuid` / `phprs_chroot` are no-ops

For production use, always compile with `phprs build` to get full native functionality.
