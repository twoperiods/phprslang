# PHPRS API Reference

> PHPRS 所有内置函数与运行时系统函数的完整参考 — 含示例代码  
> Complete API reference for the PHPRS language runtime with examples  
> Version: 2.0 | Updated: 2025-07-08

## 目录

1. [语言内置函数](#语言内置函数)
2. [异常处理 (try/catch/throw)](#异常处理-trycatchthrow)
3. [类型转换函数](#类型转换函数)
4. [类型检查函数](#类型检查函数)
5. [字符串处理](#字符串处理)
6. [URL 与编码函数](#url-与编码函数)
7. [数组函数](#数组函数)
8. [数学函数](#数学函数)
9. [日期时间函数](#日期时间函数)
10. [JSON 函数](#json-函数)
11. [文件系统函数](#文件系统函数)
12. [哈希与安全函数](#哈希与安全函数)
13. [语言特性](#语言特性)
14. [低级字符串函数 (phprs_* 前缀)](#低级字符串函数-phprs_-前缀)
15. [Socket 网络原语](#socket-网络原语)
16. [网络连接](#网络连接)
17. [HTTP 请求解析（服务端）](#http-请求解析服务端)
18. [HTTP 响应构建（服务端）](#http-响应构建服务端)
19. [HTTP 客户端](#http-客户端)
20. [WebSocket](#websocket)
21. [多线程](#多线程)
22. [curl HTTP 客户端](#curl--新一代-http-客户端)
23. [服务器配置与生产特性](#服务器配置与生产特性)
24. [日志与可观测性](#日志与可观测性)
25. [限流器](#限流器)
26. [CORS 跨域资源共享](#cors-跨域资源共享)
27. [线程池](#线程池)
28. [应用状态（线程安全全局变量）](#应用状态线程安全全局变量)
29. [请求解析](#请求解析)
30. [Redis 客户端（连接池）](#redis-客户端连接池)
31. [MySQL 客户端（连接池）](#mysql-客户端连接池)
32. [WebSocket 连接管理器](#websocket-连接管理器)
33. [生产特性总览](#生产特性总览)

---

## 语言内置函数

这些函数是 PHPRS 语言的直接内置函数，无需 `include` 即可使用。

### `strlen(string $s): int`

返回字符串的字符数（Unicode 字符数）。

```php
let $len = strlen("你好世界");
echo $len;  // 4
```

### `count(array|dict $arr): int`

返回数组或字典的元素个数。

```php
let $items = [10, 20, 30];
echo count($items);  // 3

let $map = ["a" => 1, "b" => 2];
echo count($map);    // 2
```

### `trim(string $s): string`

去除字符串首尾的空白字符（空格、制表符、换行符等）。

```php
let $name = trim("  hello  ");
echo $name;  // "hello"
```

### `str_contains(string $haystack, string $needle): bool`

检查字符串是否包含指定子串，返回 `true` 或 `false`。

```php
if (str_contains("hello world", "world")) {
    echo "found";
}
```

### `var_dump(any $value): void`

打印变量的详细信息，包括类型和值。适用于调试场景。对于数组和字典，会递归显示所有元素。

```php
var_dump(42);
// 输出: int(42)

var_dump("hello");
// 输出: string(5): "hello"

let $arr = [1, 2, 3];
var_dump($arr);
// 输出:
// array(3) {
//   [0] => int(1)
//   [1] => int(2)
//   [2] => int(3)
// }

let $map = ["name" => "Alice", "age" => 30];
var_dump($map);
// 输出:
// dict(2) {
//   ["name"] => string(5): "Alice"
//   ["age"] => int(30)
// }
```

### `print_r(any $value): void`

打印变量的可读形式。数组和字典以 PHP 风格格式化输出。

```php
print_r(42);
// 输出: 42

print_r("hello");
// 输出: hello

let $arr = [1, 2, 3];
print_r($arr);
// 输出:
// Array
// (
//   [0] => 1
//   [1] => 2
//   [2] => 3
// )

let $map = ["name" => "Alice", "age" => 30];
print_r($map);
// 输出:
// Array
// (
//   [name] => Alice
//   [age] => 30
// )
```

---

## 异常处理 (try/catch/throw)

PHPRS 支持 `try`/`catch`/`throw` 异常处理机制，与 PHP 语法兼容。`throw` 可以抛出任意类型的值（字符串、整数等），`catch` 块接收抛出的值并处理错误。

### 语法

```php
try {
    // 可能抛出异常的代码
    throw "错误信息";
} catch ($变量名) {
    // 异常处理代码
    // $变量名 包含 throw 抛出的值
}
```

### `throw` 语句

`throw` 可以抛出任意类型的值：字符串、整数、浮点数、布尔值、数组等。抛出后，程序执行立即中断，跳转到最近的 `catch` 块。

```php
throw "Something went wrong!";
throw 404;
throw ["error" => "Not found"];
```

### 基本用法

```php
try {
    echo "进入 try 块\n";
    throw "发生错误!";
    echo "这行不会执行\n";
} catch ($e) {
    echo "捕获到: ";
    echo $e;
    echo "\n";
}
echo "继续执行\n";

// 输出:
// 进入 try 块
// 捕获到: 发生错误!
// 继续执行
```

### 无异常的 try 块

如果 `try` 块中没有抛出异常，`catch` 块不会执行：

```php
try {
    echo "正常执行\n";
} catch ($e) {
    echo "不会执行到这里\n";
}
// 输出: 正常执行
```

### 函数内部抛出异常

`throw` 可以在函数内部使用，异常会穿透调用栈向上传播，直到被 `catch` 捕获：

```php
function divide(int $a, int $b): float {
    if ($b == 0) {
        throw "除数不能为零";
    }
    // 注意：PHPRS 的除法需要显式转换为 float
    return $a / $b;
}

try {
    let $result = divide(10, 0);
    echo "结果: ";
    echo $result;
} catch ($e) {
    echo "错误: ";
    echo $e;  // "除数不能为零"
}
```

### 未捕获的异常

如果 `throw` 没有被任何 `try`/`catch` 捕获，程序将终止并显示错误信息：

- **解释器模式** (`phprs run`)：打印 `Error: Uncaught exception: <信息>` 并以退出码 1 退出
- **编译模式** (`phprs build`)：打印 `Uncaught exception: <信息>` 到 stderr 并以退出码 1 退出

```php
echo "这行会执行\n";
throw "致命错误!";
echo "这行不会执行\n";

// 输出:
// 这行会执行
// Error: Uncaught exception: 致命错误!
```

### 嵌套 try/catch

可以在 `catch` 块中再次 `throw` 异常，由外层的 `try`/`catch` 捕获：

```php
try {
    try {
        throw "内层错误";
    } catch ($e) {
        echo "内层捕获: ";
        echo $e;
        echo "\n";
        throw "外层错误";  // 重新抛出
    }
} catch ($e) {
    echo "外层捕获: ";
    echo $e;
    echo "\n";
}

// 输出:
// 内层捕获: 内层错误
// 外层捕获: 外层错误
```

---

## 类型转换函数

### `intval(mixed $value, int $base = 10): int`

将值转换为整数。字符串支持指定进制解析。

```php
echo intval("42");           // 42
echo intval("0xFF", 16);    // 255
echo intval("0b1010", 2);   // 10
echo intval("077", 8);      // 63
echo intval(3.14);           // 3
echo intval(true);           // 1
echo intval(false);          // 0
```

### `floatval(mixed $value): float`

将值转换为浮点数。

```php
echo floatval("3.14");    // 3.14
echo floatval("42");      // 42.0
echo floatval("1.2e3");   // 1200.0
echo floatval(true);      // 1.0
```

### `strval(mixed $value): string`

将值转换为字符串。

```php
echo strval(123);      // "123"
echo strval(3.14);     // "3.14"
echo strval(true);     // "1"
echo strval(false);    // ""
echo strval(null);     // ""
```

### `boolval(mixed $value): bool`

将值转换为布尔值。空字符串、`0`、`null` 为 `false`，其余为 `true`。

```php
if (boolval("hello") == true) { echo "truthy\n"; }   // truthy
if (boolval("") == false) { echo "falsy\n"; }        // falsy
if (boolval(0) == false) { echo "zero is false\n"; } // zero is false
if (boolval(1) == true) { echo "one is true\n"; }    // one is true
```

---

## 类型检查函数

### `is_null(any $var): bool`

检查变量是否为 `null`。

```php
let $x = null;
if (is_null($x)) {
    echo "x is null";
}
```

### `is_int(any $var): bool`

检查变量是否为整数类型。

```php
let $x = 42;
if (is_int($x)) {
    echo "x is int";
}
```

### `is_string(any $var): bool`

检查变量是否为字符串类型。

```php
let $x = "hello";
if (is_string($x)) {
    echo "x is string";
}
```

### `is_bool(any $var): bool`

检查变量是否为布尔类型。

```php
let $x = true;
if (is_bool($x)) {
    echo "x is bool";
}
```

### `is_float(any $var): bool`

检查变量是否为浮点数类型。

```php
let $x = 3.14;
if (is_float($x)) {
    echo "x is float";
}
```

### `is_array(any $var): bool`

检查变量是否为数组或字典。

```php
let $arr = [1, 2, 3];
if (is_array($arr)) {
    echo "arr is array";
}
```

### `gettype(any $var): string`

返回变量的类型名称。可能的返回值：`"null"`、`"int"`、`"float"`、`"string"`、`"bool"`、`"array"`、`"dict"`、`"function"`。

```php
echo gettype(42);        // "int"
echo gettype("hello");   // "string"
echo gettype([1, 2]);    // "array"
```

### `isset(any $var): bool`

检查变量是否已设置且不为 `null`。

```php
let $x = 5;
if (isset($x)) {
    echo "x is set";
}
```

### `empty(any $var): bool`

检查变量是否为空（`null`、`0`、`""`、`false`、空数组均视为空）。

```php
let $x = "";
if (empty($x)) {
    echo "x is empty";
}
```

### `unset(any $var): void`

销毁变量。

```php
let $x = 5;
unset($x);
// $x 不再可用
```

---

## 字符串处理

以下为 PHP 兼容的字符串处理函数。所有函数均声明在 `framework/runtime.phprs` 中。

### `substr(string $s, int $start, int $length): string`

截取字符串的子串。

```php
echo substr("hello world", 0, 5);   // "hello"
echo substr("hello world", 6, 5);   // "world"
```

### `strpos(string $haystack, string $needle): int`

查找子串首次出现的位置（0-based），未找到返回 `-1`。

```php
echo strpos("hello world", "world");  // 6
```

### `stripos(string $haystack, string $needle): int`

不区分大小写地查找子串首次出现的位置。

```php
echo stripos("Hello World", "world");  // 6
```

### `explode(string $delimiter, string $s): string`

用分隔符分割字符串。在编译模式下返回逗号分隔的字符串；在解释器模式下返回数组。

```php
let $result = explode(",", "a,b,c,d");
echo $result;  // "a,b,c,d"
```

### `implode(string $glue, string $list): string`

用连接符将逗号分隔的字符串连接。

```php
echo implode("-", "a,b,c");  // "a-b-c"
```

### `str_repeat(string $s, int $count): string`

重复字符串指定次数。

```php
echo str_repeat("ab", 3);  // "ababab"
```

### `strtolower(string $s): string`

将字符串转换为小写。

```php
echo strtolower("HELLO");  // "hello"
```

### `strtoupper(string $s): string`

将字符串转换为大写。

```php
echo strtoupper("hello");  // "HELLO"
```

### `htmlspecialchars(string $s): string`

将 HTML 特殊字符转换为实体（`&`、`<`、`>`、`"`、`'`）。

```php
echo htmlspecialchars("<a href='test'>link & text</a>");
// &lt;a href=&#039;test&#039;&gt;link &amp; text&lt;/a&gt;
```

### `strip_tags(string $s): string`

去除字符串中的 HTML 和 PHP 标签。

```php
echo strip_tags("<p>Hello <b>World</b></p>");  // "Hello World"
```

### `nl2br(string $s): string`

将换行符 `\n` 转换为 `<br>` 标签。

```php
echo nl2br("line1\nline2");  // "line1<br>line2"
```

### `str_replace(string $search, string $replace, string $subject): string`

替换字符串中所有匹配的子串。

```php
echo str_replace("world", "PHPRS", "hello world");  // "hello PHPRS"
```

### `ltrim(string $s): string`

去除字符串开头的空白字符。

```php
echo ltrim("  hello  ");  // "hello  "
```

### `rtrim(string $s): string`

去除字符串末尾的空白字符。

```php
echo rtrim("  hello  ");  // "  hello"
```

### `strrpos(string $haystack, string $needle): int`

查找子串最后一次出现的位置（0-based），未找到返回 `-1`。

```php
echo strrpos("hello world world", "world");  // 12
```

### `ucfirst(string $s): string`

将字符串首字母转为大写。

```php
echo ucfirst("hello world");  // "Hello world"
```

### `sprintf(string $fmt, string $a1, string $a2, string $a3, string $a4): string`

格式化字符串。支持最多 4 个参数，格式说明符与 C 语言 `sprintf` 兼容。

```php
echo sprintf("Hello %s, you have %s new messages", "Alice", "5", "", "");
// "Hello Alice, you have 5 new messages"
```

### `number_format(any $num, int $decimals): string`

格式化数字为千位分隔的字符串。

```php
echo number_format(1234567.89, 2);  // "1,234,567.89"
```

### `chr(int $codepoint): string`

将 Unicode 码点转换为对应的 UTF-8 字符串。

```php
echo chr(65);     // "A"
echo chr(20013);  // "中"
```

### `ord(string $char): int`

返回字符串第一个字符的 Unicode 码点值。

```php
echo ord("ABC");   // 65
echo ord("中");    // 20013
```

### `addslashes(string $str): string`

使用反斜线转义字符串中的特殊字符（单引号、双引号、反斜线、NUL）。

```php
let $escaped = addslashes("It's \"cool\"");
echo $escaped;  // "It\'s \"cool\""
```

### `stripslashes(string $str): string`

去除 `addslashes` 添加的反斜线转义。

```php
let $original = stripslashes("It\\'s \\\"cool\\\"");
echo $original;  // "It's \"cool\""
```

### `str_pad(string $input, int $length, string $pad = " ", int $type = 0): string`

用指定字符将字符串填充到指定长度。`type`: 0=右填充, 1=左填充, 2=两侧填充。

```php
echo str_pad("42", 5, "0", 1);     // "00042" (左填充)
echo str_pad("hi", 6, ".");         // "hi...." (右填充)
echo str_pad("hi", 8, "-+", 2);    // "-+-hi-+-" (两侧填充)
echo str_pad("hello", 10);          // "hello     " (默认空格右填充)
```

### `wordwrap(string $str, int $width = 75, string $break = "\n", bool $cut = false): string`

在指定宽度处对文本进行换行。`cut=true` 时会在单词中间截断。

```php
let $text = "The quick brown fox jumped over the lazy dog";
echo wordwrap($text, 15, "\n", false);
// 输出:
// The quick brown
// fox jumped over
// the lazy dog

echo wordwrap("ABCDEFGHIJ", 3, "-", true);
// "ABC-DEF-GHI-J"
```

### `str_word_count(string $str): int`

统计字符串中的单词数量（以空格分隔）。

```php
echo str_word_count("Hello World how are you");  // 5
echo str_word_count("one");                       // 1
echo str_word_count("");                           // 0
```

### `chunk_split(string $body, int $chunklen = 76, string $end = "\r\n"): string`

每隔 N 个字符插入一个分隔符。

```php
echo chunk_split("abcdefgh", 3, "-");   // "abc-def-gh-"
echo chunk_split("Hello", 2, ".");      // "He.ll.o."

// 常用于 Base64 格式化：
let $b64 = base64_encode("some long data...");
echo chunk_split($b64, 76, "\r\n");
```

### `printf(string $format, ...args): void`

格式化输出到标准输出（同 `sprintf` + `echo`）。

```php
printf("Name: %s, Age: %d\n", "Alice", 25);
// 输出: Name: Alice, Age: 25

printf("Price: %.2f\n", 9.99);
// 输出: Price: 9.99
```

### `str_starts_with(string $haystack, string $needle): bool`

检查字符串是否以指定前缀开头（PHP 8 标准命名）。

```php
if (str_starts_with("hello world", "hello")) {
    echo "starts with hello\n";  // 输出
}
if (str_starts_with("/api/users", "/api/") == true) {
    echo "is API route\n";  // 输出
}
```

### `str_ends_with(string $haystack, string $needle): bool`

检查字符串是否以指定后缀结尾（PHP 8 标准命名）。

```php
if (str_ends_with("report.pdf", ".pdf")) {
    echo "is PDF file\n";  // 输出
}
if (str_ends_with("/api/data.json", ".json") == true) {
    echo "is JSON endpoint\n";  // 输出
}
```

---

## URL 与编码函数

### `urlencode(string $s): string`

对字符串进行 URL 编码。

```php
echo urlencode("hello world");  // "hello%20world"
```

### `urldecode(string $s): string`

对 URL 编码的字符串进行解码。

```php
echo urldecode("hello%20world");  // "hello world"
```

### `parse_url(string $url): string`

解析 URL 并返回结构化字符串（格式：`proto=...&host=...&path=...`）。

```php
let $info = parse_url("https://example.com/path?q=1");
echo $info;  // "proto=https&host=example.com&path=/path?q=1"
```

### `http_build_query(any $data): string`

将字典或数组构建为 URL 编码的查询字符串。

```php
let $data = ["name" => "Alice", "age" => "30"];
echo http_build_query($data);  // "name=Alice&age=30"
```

### `base64_encode(string $s): string`

对字符串进行 Base64 编码。

```php
echo base64_encode("hello");  // "aGVsbG8="
```

### `base64_decode(string $s): string`

对 Base64 编码的字符串进行解码。

```php
echo base64_decode("aGVsbG8=");  // "hello"
```

---

## 数组函数

### `array_push(any $arr, any $val): array`

向数组末尾添加元素。

```php
let $arr = [1, 2];
$arr = array_push($arr, 3);
// $arr 现在是 [1, 2, 3]
```

### `array_pop(array $arr): any`

弹出并返回数组最后一个元素。

```php
let $arr = [1, 2, 3];
let $last = array_pop($arr);
echo $last;  // 3
```

### `array_shift(array $arr): any`

移除并返回数组第一个元素。

```php
let $arr = [1, 2, 3];
let $first = array_shift($arr);
echo $first;  // 1
```

### `array_unshift(array $arr, any $val): array`

向数组开头添加元素。

```php
let $arr = [2, 3];
$arr = array_unshift($arr, 1);
// $arr 现在是 [1, 2, 3]
```

### `array_keys(any $arr): array`

返回数组或字典的所有键名。

```php
let $map = ["a" => 1, "b" => 2];
let $keys = array_keys($map);
// $keys 是 ["a", "b"]
```

### `array_values(any $arr): array`

返回数组或字典的所有值。

```php
let $map = ["a" => 1, "b" => 2];
let $vals = array_values($map);
// $vals 是 [1, 2]
```

### `array_merge(any $arr1, any $arr2): array`

合并两个数组。

```php
let $a = [1, 2];
let $b = [3, 4];
let $c = array_merge($a, $b);
// $c 是 [1, 2, 3, 4]
```

### `array_flip(any $arr): dict`

交换数组的键和值。

```php
let $arr = ["a" => 1, "b" => 2];
let $flipped = array_flip($arr);
// $flipped 是 [1 => "a", 2 => "b"]
```

### `in_array(any $needle, array $haystack, bool $strict): bool`

检查值是否存在于数组中。

```php
if (in_array(2, [1, 2, 3], false)) {
    echo "found";
}
```

### `array_search(any $needle, any $haystack, bool $strict): any`

搜索值并返回对应的键名。

```php
let $key = array_search("apple", ["a" => "apple", "b" => "banana"], false);
echo $key;  // "a"
```

### `array_key_exists(any $key, any $arr): bool`

检查键是否存在于数组中。

```php
if (array_key_exists("name", ["name" => "Alice"])) {
    echo "key exists";
}
```

### `array_slice(array $arr, int $offset, int $length): array`

截取数组的一部分。

```php
let $arr = [1, 2, 3, 4, 5];
let $slice = array_slice($arr, 1, 3);
// $slice 是 [2, 3, 4]
```

### `array_sum(array $arr): float`

计算数组所有值的和。

```php
echo array_sum([1, 2, 3, 4]);  // 10.0
```

### `array_unique(array $arr): array`

移除数组中的重复值。

```php
let $arr = [1, 2, 2, 3, 1];
let $unique = array_unique($arr);
// $unique 是 [1, 2, 3]
```

### `array_reverse(array $arr): array`

反转数组顺序。

```php
let $arr = [1, 2, 3];
let $rev = array_reverse($arr);
// $rev 是 [3, 2, 1]
```

### `array_filter(array $arr): array`

过滤数组中的假值（`null`、`0`、`""`、`false`）。

```php
let $arr = [0, 1, "", "hello", false, 2];
let $filtered = array_filter($arr);
// $filtered 是 [1, "hello", 2]
```

### `array_map(any $callback, array $arr): array`

对数组每个元素应用回调函数。

```php
function double($x) {
    return $x * 2;
}
let $arr = [1, 2, 3];
let $mapped = array_map("double", $arr);
// $mapped 是 [2, 4, 6]
```

### `array_reduce(array $arr, any $callback, any $initial): any`

用回调函数归约数组。

```php
function sum($carry, $item) {
    return $carry + $item;
}
let $arr = [1, 2, 3, 4];
let $total = array_reduce($arr, "sum", 0);
// $total 是 10
```

### `range(int $start, int $end, int $step): array`

创建包含指定范围元素的数组。

```php
let $r = range(0, 5, 1);
// $r 是 [0, 1, 2, 3, 4, 5]
```

### `sort(array $arr): array`

对数组排序（升序）。

```php
let $arr = [3, 1, 4, 2];
let $sorted = sort($arr);
// $sorted 是 [1, 2, 3, 4]
```

### `rsort(array $arr): array`

对数组排序（降序）。

```php
let $arr = [3, 1, 4, 2];
let $sorted = rsort($arr);
// $sorted 是 [4, 3, 2, 1]
```

### `array_diff(array $a, array $b): array`

计算数组的差集（返回在 `$a` 中但不在 `$b` 中的值）。

```php
let $a = [1, 2, 3, 4];
let $b = [2, 4];
let $diff = array_diff($a, $b);
// $diff 是 [1, 3]
```

### `array_combine(array $keys, array $vals): dict`

用一组键和一组值创建字典。

```php
let $keys = ["name", "age"];
let $vals = ["Alice", 30];
let $map = array_combine($keys, $vals);
// $map 是 ["name" => "Alice", "age" => 30]
```

### `array_column(array $rows, string $col): array`

返回二维数组中指定列的所有值。

```php
let $rows = [
    ["name" => "Alice", "age" => 30],
    ["name" => "Bob", "age" => 25]
];
let $names = array_column($rows, "name");
// $names 是 ["Alice", "Bob"]
```

### `array_fill(int $start, int $count, any $val): array`

用指定值填充数组。

```php
let $arr = array_fill(0, 5, "x");
// $arr 是 ["x", "x", "x", "x", "x"]
```

### `array_rand(any $arr, int $count): any`

从数组中随机取出一个或多个键名。

```php
let $arr = ["a" => 1, "b" => 2, "c" => 3];
let $key = array_rand($arr, 1);
echo $key;  // 随机输出 "a"、"b" 或 "c"
```

### `array_chunk(array $arr, int $size, bool $preserve_keys): array`

将数组分割成多个指定大小的块。

```php
let $arr = [1, 2, 3, 4, 5, 6, 7];
let $chunks = array_chunk($arr, 2, false);
// $chunks 是 [[1, 2], [3, 4], [5, 6], [7]]
```

### `array_count_values(array $arr): dict`

统计数组中每个值的出现次数，返回以值为键、次数为值的字典。

```php
let $arr = ["a", "b", "a", "c", "b", "a"];
let $counts = array_count_values($arr);
// $counts 是 ["a" => 3, "b" => 2, "c" => 1]
```

### `array_product(array $arr): int|float`

计算数组中所有值的乘积。

```php
echo array_product([2, 3, 4]);  // 24
echo array_product([1.5, 2.0]); // 3.0
```

### `array_intersect(array $arr1, array $arr2): array`

返回两个数组的交集（在 `$arr1` 中且值也存在于 `$arr2` 中的元素，保留 `$arr1` 的键）。

```php
let $a = [1, 2, 3, 4, 5];
let $b = [3, 4, 5, 6, 7];
let $inter = array_intersect($a, $b);
// $inter 是 [3, 4, 5]
```

### `array_splice(array $arr, int $offset, int $length): array`

从数组中移除指定范围的元素，返回被移除的元素。

```php
let $arr = [10, 20, 30, 40, 50];
let $removed = array_splice($arr, 1, 2);
echo json_encode($removed);  // [20,30]
// 注意：PHPRS 中值不可变，原数组不变

let $data = ["a", "b", "c", "d", "e"];
let $cut = array_splice($data, 2, 3);
echo json_encode($cut);      // ["c","d","e"]
```

### `array_pad(array $arr, int $size, mixed $value): array`

用指定值将数组填充到指定长度。正数右填充，负数左填充。

```php
let $a = [1, 2, 3];
echo json_encode(array_pad($a, 5, 0));    // [1,2,3,0,0]
echo json_encode(array_pad($a, -5, 0));   // [0,0,1,2,3]
echo json_encode(array_pad($a, 2, 0));    // [1,2,3] (已够长，不变)
```

### `array_key_first(array $arr): mixed`

返回数组的第一个键。列表数组返回 `0`，字典返回第一个键名。

```php
echo array_key_first([10, 20, 30]);       // 0
echo array_key_first(["x" => 1, "y" => 2]); // "x"
```

### `array_key_last(array $arr): mixed`

返回数组的最后一个键。

```php
echo array_key_last([10, 20, 30]);        // 2
echo array_key_last(["a" => 1, "b" => 2]); // "b"
```

### `array_is_list(array $arr): bool`

检查数组是否为从 0 开始的连续整数索引列表。

```php
if (array_is_list([1, 2, 3]) == true) {
    echo "是列表\n";  // 输出
}
if (array_is_list(["a" => 1]) == false) {
    echo "不是列表\n";  // 输出
}
```

---

## 数学函数

### `abs(any $n): int`

返回数的绝对值。

```php
echo abs(-42);   // 42
echo abs(3.14);  // 3
```

### `ceil(float $n): int`

向上取整。

```php
echo ceil(3.14);   // 4
echo ceil(-3.14);  // -3
```

### `floor(float $n): int`

向下取整。

```php
echo floor(3.99);   // 3
echo floor(-3.14);  // -4
```

### `round(float $n, int $precision): float`

四舍五入到指定小数位数。

```php
echo round(3.14159, 2);  // 3.14
echo round(3.99, 0);     // 4.0
```

### `max(any $a, any $b): any`

返回两个数中的较大值。

```php
echo max(10, 20);     // 20
echo max(3.14, 2.7);  // 3.14
```

### `min(any $a, any $b): any`

返回两个数中的较小值。

```php
echo min(10, 20);    // 10
echo min(3.14, 2.7); // 2.7
```

### `rand(int $min, int $max): int`

返回指定范围内的随机整数（含边界）。

```php
echo rand(1, 10);  // 随机输出 1-10 之间的整数
```

### `mt_rand(int $min, int $max): int`

与 `rand` 相同的随机整数函数（Mersenne Twister 别名）。

```php
echo mt_rand(1, 100);
```

### `pow(any $base, any $exp): float`

返回 `$base` 的 `$exp` 次幂。

```php
echo pow(2, 3);   // 8.0
echo pow(3, 2);   // 9.0
```

### `sqrt(float $n): float`

返回平方根。

```php
echo sqrt(16);   // 4.0
echo sqrt(2);    // 1.4142...
```

### `fmod(float $x, float $y): float`

返回浮点数除法的余数。

```php
echo fmod(10.5, 3.2);   // 0.9 (约数)
echo fmod(7.0, 2.5);    // 2.0
echo fmod(-5.5, 2.0);   // -1.5
```

### `intdiv(int $a, int $b): int`

返回整数除法的商（向零取整）。

```php
echo intdiv(7, 2);     // 3
echo intdiv(10, 3);    // 3
echo intdiv(-7, 2);    // -3
echo intdiv(0, 5);     // 0
```

---

## 日期时间函数

### `time(): int`

返回当前 Unix 时间戳（自 1970-01-01 00:00:00 UTC 以来的秒数）。

```php
echo time();  // 例如 1715678901
```

### `date(string $format, int $timestamp): string`

使用指定格式格式化时间戳。支持常见 PHP 日期格式字符（`Y`、`m`、`d`、`H`、`i`、`s` 等）。

```php
echo date("Y-m-d H:i:s", time());
// 输出: "2026-05-04 12:30:45"（示例）
```

### `strtotime(string $datetime): int`

将日期时间字符串解析为 Unix 时间戳。

```php
let $ts = strtotime("2026-01-01 00:00:00");
echo $ts;
```

### `microtime(): string`

返回当前 Unix 时间戳（含微秒），格式为 `"sec usec"`。

```php
echo microtime();  // "1715678901 123456"
```

### `checkdate(int $month, int $day, int $year): bool`

验证日期是否合法（考虑闰年）。

```php
if (checkdate(2, 29, 2024) == true) {
    echo "2024 是闰年\n";  // 输出
}
if (checkdate(2, 29, 2023) == false) {
    echo "2023 不是闰年\n";  // 输出
}
if (checkdate(13, 1, 2024) == false) {
    echo "没有13月\n";  // 输出
}
echo checkdate(12, 31, 2024);  // true
echo checkdate(4, 31, 2024);   // false (4月只有30天)
```

### `mktime(int $hour, int $min, int $sec, int $month, int $day, int $year): int`

根据日期时间各部分创建 Unix 时间戳。

```php
let $ts = mktime(0, 0, 0, 1, 1, 1970);
echo $ts;   // 0 (Unix 纪元)

let $ts2 = mktime(12, 30, 0, 6, 15, 2024);
echo $ts2;  // 1718451000 (2024-06-15 12:30:00 UTC)

// 可用于日期运算：
let $tomorrow = mktime(0, 0, 0, 5, 8, 2026);
echo date("Y-m-d", $tomorrow);  // "2026-05-08"
```

---

## JSON 函数

### `json_encode(any $value): string`

将 PHPRS 值编码为 JSON 字符串。支持 `null`、`int`、`float`、`string`、`bool`、`array` 和 `dict` 类型。

```php
let $data = ["name" => "Alice", "age" => 30, "active" => true];
let $json = json_encode($data);
echo $json;  // {"name":"Alice","age":30,"active":true}
```

### `json_decode(string $json): any`

将 JSON 字符串解码为 PHPRS 值。

```php
let $json = "{\"name\":\"Alice\",\"age\":30}";
let $data = json_decode($json);
// $data 是一个 dict: ["name" => "Alice", "age" => 30]
```

---

## 文件系统函数

### `file_get_contents(string $path): string`

读取文件全部内容。文件不存在时抛出异常（可被 try/catch 捕获）。

```php
let $content = file_get_contents("config.txt");
echo $content;
```

### `file_put_contents(string $path, string $content): int`

将字符串写入文件（覆盖写入）。返回写入的字节数。失败时抛出异常。

```php
let $bytes = file_put_contents("output.txt", "Hello World");
echo "Written: ";
echo $bytes;
```

### `file_exists(string $path): bool`

检查文件是否存在。

```php
if (file_exists("config.txt")) {
    echo "file exists";
}
```

### `is_dir(string $path): bool`

检查路径是否为目录。

```php
if (is_dir("/path/to/dir")) {
    echo "is directory";
}
```

### `mkdir(string $path): bool`

创建目录。成功返回 `true`，失败返回 `false`。

```php
mkdir("/path/to/new/dir");
```

### `unlink(string $path): bool`

删除文件。成功返回 `true`，失败返回 `false`。

```php
unlink("old_file.txt");
```

### `basename(string $path): string`

提取路径中的文件名部分。

```php
echo basename("/path/to/file.txt");  // "file.txt"
```

### `dirname(string $path): string`

提取路径中的目录名部分。

```php
echo dirname("/path/to/file.txt");  // "/path/to"
```

### `scandir(string $path): array`

列出目录中的文件和子目录名称（返回数组）。失败时抛出异常。

```php
let $files = scandir("/path/to/dir");
// $files 是 ["file1.txt", "file2.txt", "subdir"]
```

### `copy(string $source, string $dest): bool`

拷贝文件。成功返回 `true`，失败返回 `false`。

```php
if (copy("source.txt", "dest.txt")) {
    echo "copied";
}
```

### `rename(string $old, string $new): bool`

重命名或移动文件。成功返回 `true`，失败返回 `false`。

```php
if (rename("old_name.txt", "new_name.txt")) {
    echo "renamed";
}
```

### `filesize(string $path): int`

返回文件大小（字节），失败返回 `-1`。

```php
let $size = filesize("data.json");
echo $size;  // 例如 1024
```

### `filemtime(string $path): int`

返回文件最后修改时间的 Unix 时间戳，失败返回 `-1`。

```php
let $mtime = filemtime("data.json");
echo date("Y-m-d H:i:s", $mtime);
```

### `pathinfo(string $path): string`

解析文件路径，返回包含 `dirname`、`basename`、`extension`、`filename` 的 JSON 字符串。

```php
let $info = pathinfo("/var/www/index.html");
echo $info;
// {"dirname":"/var/www","basename":"index.html","extension":"html","filename":"index"}
```

### `move_uploaded_file(string $tmp, string $dest): bool`

将上传的临时文件移动到目标位置。移动前会检查源文件是否存在。

```php
if (move_uploaded_file("/tmp/upload_abc", "uploads/file.txt")) {
    echo "upload saved";
}
```

---

## 哈希与安全函数

### `md5(string $s): string`

计算字符串的 MD5 哈希值（32 位十六进制小写字符串）。

```php
echo md5("hello");  // "5d41402abc4b2a76b9719d911017c592"
```

### `sha1(string $s): string`

计算字符串的 SHA1 哈希值（40 位十六进制小写字符串）。

```php
echo sha1("hello");  // "aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d"
```

### `uniqid(string $prefix): string`

基于当前时间生成唯一 ID（含微秒）。

```php
echo uniqid("id_");  // "id_664f1a2b3c4d5"
```

### `sleep(int $seconds): void`

暂停执行指定的秒数。

```php
echo "start\n";
sleep(1);
echo "1 second later\n";
```

### `usleep(int $microseconds): void`

暂停执行指定的微秒数（1 秒 = 1,000,000 微秒）。

```php
usleep(500000);  // 暂停 0.5 秒
```

### `realpath(string $path): string`

返回规范化的绝对路径名。

```php
echo realpath("./config.txt");  // "/home/user/project/config.txt"
```

### `is_file(string $path): int`

检查路径是否为普通文件。返回 `1`（是文件）或 `0`（不是文件）。

```php
if (is_file("data.txt") == 1) {
    echo "is a regular file";
}
```

### `getcwd(): string`

获取当前工作目录。

```php
echo getcwd();  // "/home/user/project"
```

### `password_hash(string $password, string $algo): string`

对密码进行哈希处理。支持 `"sha1"`（默认）、`"sha256"`、`"bcrypt"` 算法。返回格式为 `算法$盐$哈希值` 的字符串。

```php
let $hash = password_hash("my_secret", "sha1");
echo $hash;  // "sha1$a1b2c3...$d4e5f6..."
```

### `password_verify(string $password, string $hash): bool`

验证密码是否与存储的哈希值匹配。使用常量时间比较以防止时序攻击。

```php
let $hash = password_hash("my_secret", "sha1");
if (password_verify("my_secret", $hash)) {
    echo "password correct";
}
```

### `random_bytes(int $length): string`

生成加密安全的随机字节，返回十六进制字符串（长度为 `$length * 2`）。

```php
let $token = random_bytes(16);
echo $token;  // "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6"（32 位十六进制）
```

### `random_int(int $min, int $max): int`

生成加密安全的随机整数（包含边界）。

```php
let $dice = random_int(1, 6);
echo $dice;  // 1-6 之间的随机整数
```

---

## 语言特性

### for 循环

`for (init; condition; update) { ... }` — 经典的三段式循环。

```php
for (let $i = 0; $i < 5; $i += 1) {
    echo "i = ";
    echo $i;
    echo "\n";
}
// 输出: i = 0
//       i = 1
//       i = 2
//       i = 3
//       i = 4
```

支持 `break` 和 `continue`：

```php
for (let $i = 0; $i < 10; $i += 1) {
    if ($i == 3) {
        continue;  // 跳过 3
    }
    if ($i == 7) {
        break;     // 在 7 时退出
    }
    echo $i;
}
// 输出: 012456
```

### foreach 循环

`foreach ($array as $value)` — 遍历数组或字典。

遍历数组：
```php
let $arr = [10, 20, 30];
foreach ($arr as $item) {
    echo "item: ";
    echo $item;
    echo "\n";
}
// 输出: item: 10
//       item: 20
//       item: 30
```

遍历数组并获取键名：
```php
let $arr = ["a", "b", "c"];
foreach ($arr as $idx => $val) {
    echo $idx;
    echo " => ";
    echo $val;
    echo "\n";
}
// 输出: 0 => a
//       1 => b
//       2 => c
```

遍历字典：
```php
let $map = ["name" => "Alice", "age" => "30"];
foreach ($map as $key => $value) {
    echo $key;
    echo ": ";
    echo $value;
    echo "\n";
}
// 输出: name: Alice
//       age: 30
```

支持 `break` 和 `continue`：
```php
foreach ($arr as $item) {
    if ($item == 20) {
        continue;  // 跳过
    }
    echo $item;
}
```

### do-while 循环

`do { ... } while (condition);` — 先执行循环体再检查条件，循环体至少执行一次。

```php
let $i = 0;
do {
    echo "i = ";
    echo $i;
    echo "\n";
    $i += 1;
} while ($i < 3);
// 输出: i = 0
//       i = 1
//       i = 2
```

支持 `break` 和 `continue`：

```php
let $x = 0;
do {
    $x += 1;
    if ($x == 3) {
        continue;
    }
    echo $x;
    if ($x >= 5) {
        break;
    }
} while ($x < 10);
// 输出: 1 2 4 5
```

### elseif 语句

`elseif` 用于在 `if` 结构中链接多个条件判断：

```php
let $score = 85;
if ($score >= 90) {
    echo "A\n";
} elseif ($score >= 80) {
    echo "B\n";
} elseif ($score >= 70) {
    echo "C\n";
} else {
    echo "F\n";
}
```

### include_once / require_once

`include_once` 和 `require_once` 与 `include` / `require` 类似，但会检查文件是否已被包含过，避免重复包含。

`include_once` — 文件不存在时产生警告并继续执行（非致命）：
```php
include_once "shared_functions.phprs";
```

`require_once` — 文件不存在时产生致命错误并终止执行：
```php
require_once "required_config.phprs";
```

`include_once` 和 `require_once` 确保同一个文件只会被包含一次，即使调用多次：
```php
include_once "framework/runtime.phprs";  // 第一次：包含
include_once "framework/runtime.phprs";  // 第二次：跳过（已包含）
```

---

## 低级字符串函数 (phprs_* 前缀)

以下是需要通过 `include "framework/runtime.phprs"` 引入的低级字符串处理函数。推荐优先使用上方的 PHP 兼容字符串函数（`substr`、`str_replace`、`strtolower` 等）。

```php
include "framework/runtime.phprs";
```

### `phprs_str_replace(string $s, string $from, string $to): string`

将字符串中所有匹配的 `$from` 子串替换为 `$to`。替换所有出现。

```php
let $result = phprs_str_replace("hello world", "world", "PHPRS");
echo $result;  // "hello PHPRS"

// 常用于 URL 解析中去掉协议前缀
let $url = "https://example.com";
let $rest = phprs_str_replace($url, "https://", "");
echo $rest;  // "example.com"
```

### `phprs_str_contains(string $haystack, string $needle): int`

检查字符串是否包含指定子串。返回 `1`（包含）或 `0`（不包含）。

```php
if (phprs_str_contains("hello world", "world") == 1) {
    echo "found";
}
```

### `phprs_str_split(string $s, string $delim, int $index): string`

用分隔符 `$delim` 分割字符串，返回第 `$index` 个片段（0-based）。如果索引超出范围，返回空字符串。

```php
// 解析 "proto=https&host=example.com&path=/api" 格式
let $parsed = "proto=https&host=example.com&path=/api";

let $proto_part = phprs_str_split($parsed, "&", 0);  // "proto=https"
let $host_part  = phprs_str_split($parsed, "&", 1);  // "host=example.com"
let $path_part  = phprs_str_split($parsed, "&", 2);  // "path=/api"
```

### `phprs_str_starts_with(string $s, string $prefix): int`

检查字符串是否以指定前缀开头。返回 `1` 或 `0`。

```php
if (phprs_str_starts_with("https://example.com", "https://") == 1) {
    echo "secure connection";
}
```

### `phprs_str_ends_with(string $s, string $suffix): int`

检查字符串是否以指定后缀结尾。返回 `1` 或 `0`。

```php
if (phprs_str_ends_with("/api/data.json", ".json") == 1) {
    echo "JSON endpoint";
}
```

### `phprs_str_upper(string $s): string`

将字符串全部转换为大写。

```php
echo phprs_str_upper("hello");  // "HELLO"
```

### `phprs_str_lower(string $s): string`

将字符串全部转换为小写。

```php
echo phprs_str_lower("HELLO");  // "hello"
```

---

## Socket 网络原语

底层 TCP socket 操作函数，用于构建自定义网络服务。

### `phprs_server_new(int $port): int`

创建一个监听指定端口的 TCP 服务器 socket。返回 socket 文件描述符（fd），失败返回 `-1`。

```php
let $server_fd = phprs_server_new(8080);
if ($server_fd < 0) {
    echo "Failed to start server";
}
```

### `phprs_server_accept(int $fd): int`

接受一个客户端连接。阻塞直到有客户端连接进来。返回客户端 socket fd，失败返回 `-1`。
同时会保存客户端 IP 地址，可通过 `phprs_client_ip()` 获取。

```php
let $client_fd = phprs_server_accept($server_fd);
if ($client_fd >= 0) {
    // 处理客户端连接
    let $ip = phprs_client_ip($client_fd);
    echo "Client connected from: " . $ip . "\n";
}
```

### `phprs_client_ip(int $fd): string`

获取最近一次 `phprs_server_accept()` 接受的客户端的 IP 地址。

```php
let $client = phprs_server_accept($sock);
let $ip = phprs_client_ip($client);
echo "Connected from: " . $ip . "\n";  // 如 "192.168.1.100"

// 常用于速率限制、日志记录：
if (rate_limit_allow($ip) == 0) {
    echo "Rate limited: " . $ip . "\n";
}
```

### `phprs_socket_read(int $fd, int $max_size): string`

从 socket 读取最多 `$max_size` 字节数据。返回读取到的字符串。

```php
let $data = phprs_socket_read($client_fd, 4096);
echo $data;
```

### `phprs_socket_write(int $fd, string $data): int`

向 socket 写入数据。返回成功写入的字节数，失败返回 `-1`。

```php
let $response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<h1>Hello</h1>";
phprs_socket_write($client_fd, $response);
```

### `phprs_socket_close(int $fd): void`

关闭 socket 连接。

```php
phprs_socket_close($client_fd);
```

### `phprs_socket_read_all(int $fd): string`

读取 socket 上所有可用数据直到连接关闭或 HTTP 响应完成。自动处理 `Content-Length` 和 `Transfer-Encoding: chunked`。

这是 HTTP 客户端的核心函数，用于读取完整的 HTTP 响应。

```php
let $response = phprs_socket_read_all($fd);
let $status = phprs_http_response_status($response);
let $body = phprs_http_response_body($response);
```

---

## 网络连接

### `phprs_dns_resolve(string $hostname): string`

将域名解析为 IPv4 地址字符串。失败返回空字符串。

```php
let $ip = phprs_dns_resolve("www.qq.com");
echo $ip;  // 例如 "61.135.157.156"
```

### `phprs_tcp_connect(string $host, int $port): int`

创建到指定主机和端口的 TCP 连接。返回 socket fd，失败返回 `-1`。

```php
let $fd = phprs_tcp_connect("example.com", 80);
if ($fd >= 0) {
    // 发送 HTTP 请求
    phprs_socket_write($fd, "GET / HTTP/1.1\r\nHost: example.com\r\n\r\n");
    let $resp = phprs_socket_read_all($fd);
    phprs_socket_close($fd);
}
```

### `phprs_tls_connect(string $host, int $port): int`

创建到指定主机的 TLS/SSL 加密连接（HTTPS）。返回 socket fd，失败返回 `-1`。

在 Windows 上使用系统 Schannel（无需额外依赖），在 Linux/macOS 上使用 OpenSSL。

```php
let $fd = phprs_tls_connect("www.qq.com", 443);
if ($fd >= 0) {
    // 发送 HTTPS 请求
    let $req = phprs_http_build_request("GET", "www.qq.com", "/", "", "");
    phprs_socket_write($fd, $req);
    let $resp = phprs_socket_read_all($fd);
    phprs_socket_close($fd);
}
```

---

## 文件 I/O

### `phprs_file_read(string $path): string`

读取文件全部内容并以字符串返回。文件不存在返回空字符串。

```php
let $content = phprs_file_read("config.json");
echo $content;
```

### `phprs_file_write(string $path, string $content): int`

将字符串内容写入文件（覆盖写入）。返回写入的字节数，失败返回 `-1`。

```php
let $bytes = phprs_file_write("output.txt", "Hello World");
echo "Wrote: ";
echo $bytes;
```

### `phprs_file_exists(string $path): int`

检查文件是否存在。存在返回 `1`，不存在返回 `0`。

```php
if (phprs_file_exists("config.json") == 1) {
    let $config = phprs_file_read("config.json");
}
```

---

## HTTP 请求解析（服务端）

用于在 PHPRS Web 服务器中解析原始 HTTP 请求字符串。

### `phprs_http_method(string $raw): string`

从原始 HTTP 请求中提取 HTTP 方法（GET、POST 等）。

```php
// raw = "GET /api/users HTTP/1.1\r\nHost: ..."
let $method = phprs_http_method($raw);
echo $method;  // "GET"
```

### `phprs_http_path(string $raw): string`

从原始 HTTP 请求中提取请求路径。

```php
// raw = "GET /api/users?id=1 HTTP/1.1\r\nHost: ..."
let $path = phprs_http_path($raw);
echo $path;  // "/api/users?id=1"
```

### `phprs_http_header(string $raw, string $name): string`

从原始 HTTP 请求中提取指定名称的请求头值。如果该头部不存在，返回空字符串。

```php
let $content_type = phprs_http_header($raw, "Content-Type");
echo $content_type;  // 例如 "application/json"

let $auth = phprs_http_header($raw, "Authorization");
```

### `phprs_http_body(string $raw): string`

从原始 HTTP 请求中提取请求体（`\r\n\r\n` 之后的内容）。用于获取 POST 请求的 body。

```php
if ($method == "POST") {
    let $body = phprs_http_body($raw);
    // 解析 JSON body
    let $name = phprs_json_get_string($body, "name");
}
```

### `phprs_url_decode(string $encoded): string`

对 URL 编码（百分号编码）的字符串进行解码。

```php
echo phprs_url_decode("hello%20world");  // "hello world"
echo phprs_url_decode("%E4%BD%A0%E5%A5%BD");  // "你好"
```

---

## HTTP 响应构建（服务端）

### `phprs_http_response(int $status_code, string $content_type, string $body): string`

构建完整的 HTTP 响应字符串，可直接通过 `phprs_socket_write` 发送。

自动添加 `Content-Length` 头部。

支持的状态码：200, 201, 204, 301, 302, 400, 401, 403, 404, 405, 500, 502, 503。

```php
let $body = "<h1>Hello World</h1>";
let $response = phprs_http_response(200, "text/html; charset=utf-8", $body);
phprs_socket_write($client_fd, $response);
phprs_socket_close($client_fd);
```

---

## HTTP 客户端

用于发起 HTTP 请求。推荐使用 `framework/http_client.phprs` 中的高级封装函数，而不是直接调用底层函数。

### 底层函数

#### `phprs_http_build_request(string $method, string $host, string $path, string $headers, string $body): string`

构建原始 HTTP 请求字符串。自动添加 `Host` 和 `Connection: close` 头部。

```php
// 构建 GET 请求
let $req = phprs_http_build_request("GET", "example.com", "/api/data", "", "");
// 结果: "GET /api/data HTTP/1.1\r\nConnection: close\r\nHost: example.com\r\n\r\n"

// 构建 POST 请求（带自定义头部）
let $headers = "Content-Type: application/json\r\nAuthorization: Bearer token123\r\n";
let $body = "{\"name\": \"test\"}";
let $req = phprs_http_build_request("POST", "example.com", "/api/create", $headers, $body);
```

#### `phprs_http_response_status(string $raw): int`

从原始 HTTP 响应字符串中提取状态码。

```php
// raw = "HTTP/1.1 200 OK\r\n..."
let $status = phprs_http_response_status($raw);
echo $status;  // 200
```

#### `phprs_http_response_body(string $raw): string`

从原始 HTTP 响应中提取响应体（`\r\n\r\n` 之后的部分）。等价于 `phprs_http_body`。

```php
let $body = phprs_http_response_body($raw);
```

### 高级封装函数

需要 `include "framework/http_client.phprs"`（会自动包含 `runtime.phprs`）。

#### `http_get(string $url): string`

发起 HTTP GET 请求并返回完整的原始 HTTP 响应。自动处理 HTTP 和 HTTPS、DNS 解析、端口选择。

```php
include "framework/http_client.phprs";

let $response = http_get("https://api.github.com/users/octocat");
let $status = http_status($response);
let $body = http_body($response);

echo "Status: ";
echo $status;
echo "\nBody: ";
echo $body;
```

#### `http_post(string $url, string $body, string $content_type): string`

发起 HTTP POST 请求，发送 JSON（或其它格式）的请求体。

```php
include "framework/http_client.phprs";

let $json_body = "{\"title\": \"foo\", \"body\": \"bar\", \"userId\": 1}";
let $response = http_post("https://jsonplaceholder.typicode.com/posts", $json_body, "application/json");
let $status = http_status($response);
let $body = http_body($response);

echo "Status: ";
echo $status;
echo "\nResponse: ";
echo $body;
```

#### `http_status(string $response): int`

提取 HTTP 状态码。是 `phprs_http_response_status` 的简短别名。

```php
let $status = http_status($response);
if ($status == 200) {
    echo "Success";
}
```

#### `http_body(string $response): string`

提取 HTTP 响应体。是 `phprs_http_response_body` 的简短别名。

```php
let $body = http_body($response);
```

#### `http_parse_url(string $url): string`

将 URL 解析为结构化字符串，格式为 `proto=PROTO&host=HOST&path=PATH`。

```php
let $parsed = http_parse_url("https://example.com/api/data");
echo $parsed;  // "proto=https&host=example.com&path=/api/data"
```

#### `http_parsed_proto(string $parsed): string`

从解析后的 URL 字符串中提取协议（`http` 或 `https`）。

```php
let $proto = http_parsed_proto($parsed);
echo $proto;  // "https"
```

#### `http_parsed_host(string $parsed): string`

从解析后的 URL 字符串中提取主机名。

```php
let $host = http_parsed_host($parsed);
echo $host;  // "example.com"
```

#### `http_parsed_path(string $parsed): string`

从解析后的 URL 字符串中提取路径。

```php
let $path = http_parsed_path($parsed);
echo $path;  // "/api/data"
```

### 完整 HTTP 客户端示例

```php
<?phprs
include "framework/http_client.phprs";

// 发起 GET 请求
let $data = http_get("http://wfx.one");
let $status = http_status($data);

echo "Status: ";
echo $status;
echo "\n";

if ($status == 200) {
    let $body = http_body($data);
    echo "Response length: ";
    echo strlen($body);
    echo " bytes\n";
}
?>
```

### curl — 新一代 HTTP 客户端

需要 `include "framework/runtime.phprs"`。

`curl` 提供同步和异步两种模式的 HTTP/HTTPS 请求，自动处理 DNS 解析、TCP/TLS 连接和 HTTP 协议。返回值均为 dict：

```php
{"status": 200, "headers": "...", "body": "...", "error": "..."}
```

> `error` 字段仅在连接失败时存在。

**Options dict 参数：**

| Key | 类型 | 默认值 | 说明 |
|-----|------|--------|------|
| `method` | string | `"GET"` | HTTP 方法：GET、POST、PUT、DELETE 等 |
| `body` | string | `""` | 请求体内容 |
| `headers` | string | `""` | 附加请求头，每个 header 用 `\r\n` 分隔 |
| `timeout` | int | `30` | 连接和读取的超时秒数 |

---

#### `curl(string $url, dict $opts): dict`

发起同步 HTTP 请求，阻塞直到响应完成。

##### 示例：GET 请求

```php
<?phprs
// 最简用法：获取网页
let $resp = curl("http://www.baidu.com", []);
echo $resp["status"];   // 200
echo "\n";
echo strlen($resp["body"]);  // 内容字节数
echo " 字节\n";
?>
```

```php
<?phprs
// 带超时的 GET
let $resp = curl("https://api.github.com/users/octocat", [
    "timeout" => 15
]);

if ($resp["status"] == 200) {
    echo $resp["body"];  // GitHub API 返回的 JSON
}
?>
```

```php
<?phprs
// GET + 自定义 Headers
let $resp = curl("https://httpbin.org/headers", [
    "headers" => "X-API-Key: abc123\r\nAccept: application/json\r\n"
]);
echo $resp["status"];  // 200
echo $resp["body"];
?>
```

##### 示例：POST 请求

```php
<?phprs
// POST 表单数据 (application/x-www-form-urlencoded)
let $resp = curl("https://httpbin.org/post", [
    "method" => "POST",
    "body" => "username=admin&password=secret"
]);
echo $resp["status"];  // 200
echo $resp["body"];
?>
```

```php
<?phprs
// POST JSON 数据
let $json_body = "{\"title\": \"Hello\", \"body\": \"World\", \"userId\": 1}";
let $resp = curl("https://jsonplaceholder.typicode.com/posts", [
    "method" => "POST",
    "body" => $json_body,
    "headers" => "Content-Type: application/json\r\n"
]);
echo $resp["status"];  // 201
echo $resp["body"];
?>
```

```php
<?phprs
// POST 带 Authorization 头
let $resp = curl("https://api.example.com/orders", [
    "method" => "POST",
    "body" => "{\"product_id\": 42}",
    "headers" => "Content-Type: application/json\r\nAuthorization: Bearer eyJhbGciOi...\r\n"
]);
?>
```

##### 示例：PUT 请求

```php
<?phprs
// PUT 更新资源
let $resp = curl("https://jsonplaceholder.typicode.com/posts/1", [
    "method" => "PUT",
    "body" => "{\"id\": 1, \"title\": \"Updated\"}",
    "headers" => "Content-Type: application/json\r\n"
]);
echo $resp["status"];  // 200
?>
```

##### 示例：DELETE 请求

```php
<?phprs
let $resp = curl("https://jsonplaceholder.typicode.com/posts/1", [
    "method" => "DELETE"
]);
echo $resp["status"];  // 200
?>
```

##### 示例：错误处理

```php
<?phprs
let $resp = curl("http://invalid-host.xyz", ["timeout" => 5]);

if (isset($resp["error"])) {
    echo "请求失败: " . $resp["error"];
    // 输出: 请求失败: TCP connect failed: ...
} else {
    echo "状态: ";
    echo $resp["status"];
    echo "\n";
    echo "内容: ";
    echo $resp["body"];
}
?>
```

---

#### `curl_async(string $url, dict $opts): int`

发起异步 HTTP 请求，**立即返回**一个 int 句柄，在后台线程执行。不阻塞当前代码。

```php
<?phprs
// 同时发出 3 个请求
let $h1 = curl_async("https://httpbin.org/delay/3", ["timeout" => 10]);
let $h2 = curl_async("https://httpbin.org/get",     []);
let $h3 = curl_async("https://httpbin.org/get",     []);

echo "请求已发出，等待返回...\n";

// 逐个等待结果
let $r1 = curl_wait($h1);
let $r2 = curl_wait($h2);
let $r3 = curl_wait($h3);

echo "全部完成！\n";
?>
```

---

#### `curl_wait(int $handle): dict`

等待一个异步请求完成，返回响应 dict。**会阻塞**直到该请求完成。

```php
<?phprs
let $h = curl_async("https://httpbin.org/get", []);
echo "等待响应...\n";
let $resp = curl_wait($h);  // 阻塞在此
echo $resp["status"];        // 200
echo "\n";
?>
```

---

#### `curl_is_done(int $handle): bool`

非阻塞检查异步请求是否已完成。返回 `true` / `false`。

```php
<?phprs
let $h = curl_async("https://httpbin.org/delay/5", ["timeout" => 10]);

// 轮询
while (!curl_is_done($h)) {
    echo "等待中...\n";
    sleep(1);
}
echo "完成！\n";
let $resp = curl_wait($h);
?>
```

---

### 完整示例：并 行爬虫

```php
<?phprs
let $urls = [
    "https://www.baidu.com",
    "https://www.qq.com",
    "https://www.taobao.com"
];

// 全部发出
let mut $handles = [];
let $count = count($urls);
for (let $i = 0; $i < $count; $i++) {
    let $h = curl_async($urls[$i], ["timeout" => 10]);
    let $handles = array_push($handles, $h);
}

// 收集结果
let $h_count = count($handles);
for (let $i = 0; $i < $h_count; $i++) {
    let $resp = curl_wait($handles[$i]);
    echo "[";
    echo $i + 1;
    echo "] ";
    echo $urls[$i];
    echo " -> ";
    echo $resp["status"];
    echo " (";
    echo strlen($resp["body"]);
    echo " 字节)\n";
}
?>
```

---

### `curl` 与旧的 `http_get`/`http_post` 对比

| 特性 | `curl` | `http_get`/`http_post` |
|------|--------|------------------------|
| 返回值 | dict（结构化） | string（原始 HTTP 文本） |
| 异步 | 支持 `curl_async` | 不支持 |
| 自定义 Headers | 参数直接传入 | 需自行修改函数源码 |
| 超时控制 | `timeout` 参数 | 不支持 |
| 错误信息 | `error` 字段 | 需手动解析 |
| 方法切换 | `method` 参数 | 独立函数 `http_get` / `http_post` |

> 推荐新代码使用 `curl` 系列函数。

---

## JSON 解析

提供扁平 JSON 对象的简单字段提取，无需引入完整的 JSON 解析器。

### `phprs_json_get_string(string $json, string $key): string`

从 JSON 字符串中提取指定 key 对应的字符串值。key 必须在 JSON 中是字符串类型。

```php
let $json = "{\"name\": \"Alice\", \"city\": \"Beijing\"}";
let $name = phprs_json_get_string($json, "name");
echo $name;  // "Alice"
```

### `phprs_json_get_int(string $json, string $key): int`

从 JSON 字符串中提取指定 key 对应的整数值。

```php
let $json = "{\"count\": 42, \"name\": \"test\"}";
let $count = phprs_json_get_int($json, "count");
echo $count;  // 42
```

---

## WebSocket

WebSocket 协议支持函数，用于构建实时通信服务。

### `phprs_is_websocket_upgrade(string $raw): int`

检查 HTTP 请求是否为 WebSocket 升级请求。是返回 `1`，否返回 `0`。

```php
let $raw = phprs_socket_read($client_fd, 4096);
if (phprs_is_websocket_upgrade($raw) == 1) {
    // 处理 WebSocket 升级
    let $handshake = phprs_ws_handshake_response($raw);
    phprs_socket_write($client_fd, $handshake);
}
```

### `phprs_ws_handshake_response(string $raw): string`

根据 WebSocket 升级请求生成握手响应。

```php
let $response = phprs_ws_handshake_response($ws_request);
phprs_socket_write($client_fd, $response);
// 连接已升级为 WebSocket
```

### `phprs_ws_read_frame(int $fd, int $timeout_ms): string`

从 WebSocket 连接读取一个帧。`$timeout_ms` 指定超时时间（毫秒）。

```php
// 读取帧，超时 5 秒
let $frame = phprs_ws_read_frame($client_fd, 5000);
if ($frame != "") {
    echo "Received: ";
    echo $frame;
}
```

### `phprs_ws_write_frame(int $fd, string $payload, int $opcode): int`

向 WebSocket 连接发送一个帧。`$opcode` 为 1 表示文本帧，为 2 表示二进制帧。返回发送的字节数。

```php
// 发送文本帧
phprs_ws_write_frame($client_fd, "Hello, client!", 1);
```

### `phprs_ws_send_pong(int $fd, string $payload): int`

发送 Pong 帧，用于响应 Ping 保持连接活跃。

```php
phprs_ws_send_pong($client_fd, "pong");
```

### `phprs_ws_close(int $fd): void`

关闭 WebSocket 连接（发送 CLOSE 帧并关闭 socket）。

```php
phprs_ws_close($client_fd);
```

---

## 多线程

线程和互斥锁支持（实验性功能）。

### `phprs_thread_spawn(string $func_name, int $client_fd, string $raw_request): int`

在新线程中调用指定的函数。`$func_name` 必须是通过 `phprs_register_handler` 注册的函数名。返回线程句柄。

> **注意**：此函数在解释器模式（`phprs run`）下为 no-op，仅在编译模式（`phprs build`）下生效，因为编译模式会通过 `phprs_register_handler` 注册函数到调度表。

### `phprs_mutex_new(): int`

创建新的互斥锁，返回互斥锁句柄。

```php
let $mutex = phprs_mutex_new();
```

### `phprs_mutex_lock(int $handle): void`

获取互斥锁（阻塞直到成功获取）。

```php
phprs_mutex_lock($mutex);
// 临界区代码
phprs_mutex_unlock($mutex);
```

### `phprs_mutex_unlock(int $handle): void`

释放互斥锁。

```php
phprs_mutex_unlock($mutex);
```

---

## 快速索引

| 函数 | 签名 | 用途 |
|------|------|------|
| `strlen` | `(string): int` | 字符串长度 |
| `count` | `(array\|dict): int` | 数组/字典元素数 |
| `trim` | `(string): string` | 去除首尾空白 |
| `str_contains` | `(string, string): bool` | 检查子串 |
| `var_dump` | `(any): void` | 打印变量详情（含类型） |
| `print_r` | `(any): void` | 打印变量可读形式 |
| `phprs_str_replace` | `(string, string, string): string` | 字符串替换 |
| `phprs_str_contains` | `(string, string): int` | 检查子串（返回 0/1） |
| `phprs_str_split` | `(string, string, int): string` | 分割取第 N 段 |
| `phprs_str_starts_with` | `(string, string): int` | 检查前缀 |
| `phprs_str_ends_with` | `(string, string): int` | 检查后缀 |
| `phprs_str_upper` | `(string): string` | 转大写 |
| `phprs_str_lower` | `(string): string` | 转小写 |
| `phprs_server_new` | `(int): int` | 创建 TCP 服务器 |
| `phprs_server_accept` | `(int): int` | 接受客户端连接 |
| `phprs_socket_read` | `(int, int): string` | 读指定字节 |
| `phprs_socket_write` | `(int, string): int` | 写数据到 socket |
| `phprs_socket_close` | `(int): void` | 关闭 socket |
| `phprs_socket_read_all` | `(int): string` | 读全部数据 |
| `phprs_tcp_connect` | `(string, int): int` | TCP 连接 |
| `phprs_tls_connect` | `(string, int): int` | TLS 加密连接 |
| `phprs_dns_resolve` | `(string): string` | DNS 解析 |
| `phprs_file_read` | `(string): string` | 读文件内容 |
| `phprs_file_write` | `(string, string): int` | 写文件 |
| `phprs_file_exists` | `(string): int` | 检查文件存在 |
| `phprs_http_method` | `(string): string` | 提取 HTTP 方法 |
| `phprs_http_path` | `(string): string` | 提取请求路径 |
| `phprs_http_header` | `(string, string): string` | 提取请求头 |
| `phprs_http_body` | `(string): string` | 提取请求/响应体 |
| `phprs_url_decode` | `(string): string` | URL 解码 |
| `phprs_http_response` | `(int, string, string): string` | 构建 HTTP 响应 |
| `phprs_http_build_request` | `(string, string, string, string, string): string` | 构建 HTTP 请求 |
| `phprs_http_response_status` | `(string): int` | 提取响应状态码 |
| `phprs_http_response_body` | `(string): string` | 提取响应体 |
| `http_get` | `(string): string` | HTTP GET 请求 |
| `http_post` | `(string, string, string): string` | HTTP POST 请求 |
| `http_status` | `(string): int` | 提取 HTTP 状态码 |
| `http_body` | `(string): string` | 提取 HTTP 响应体 |
| `http_parse_url` | `(string): string` | 解析 URL |
| `http_parsed_proto` | `(string): string` | 提取协议 |
| `http_parsed_host` | `(string): string` | 提取主机名 |
| `http_parsed_path` | `(string): string` | 提取路径 |
| `phprs_json_get_string` | `(string, string): string` | 提取 JSON 字符串字段 |
| `phprs_json_get_int` | `(string, string): int` | 提取 JSON 整数字段 |
| `phprs_is_websocket_upgrade` | `(string): int` | 检测 WebSocket 升级 |
| `phprs_ws_handshake_response` | `(string): string` | 生成握手响应 |
| `phprs_ws_read_frame` | `(int, int): string` | 读 WebSocket 帧 |
| `phprs_ws_write_frame` | `(int, string, int): int` | 写 WebSocket 帧 |
| `phprs_ws_send_pong` | `(int, string): int` | 发送 Pong 帧 |
| `phprs_ws_close` | `(int): void` | 关闭 WebSocket |
| `phprs_thread_spawn` | `(string, int, string): int` | 创建线程 |
| `phprs_mutex_new` | `(): int` | 创建互斥锁 |
| `phprs_mutex_lock` | `(int): void` | 获取锁 |
| `phprs_mutex_unlock` | `(int): void` | 释放锁 |
| `substr` | `(string, int, int): string` | 截取子串 |
| `strpos` | `(string, string): int` | 查找子串位置 |
| `stripos` | `(string, string): int` | 查找子串位置(不区分大小写) |
| `explode` | `(string, string): array` | 分割字符串为数组 |
| `implode` | `(string, array): string` | 连接数组为字符串 |
| `str_repeat` | `(string, int): string` | 重复字符串 |
| `strtolower` | `(string): string` | 转小写 |
| `strtoupper` | `(string): string` | 转大写 |
| `htmlspecialchars` | `(string): string` | HTML 特殊字符转义 |
| `strip_tags` | `(string): string` | 去除 HTML 标签 |
| `nl2br` | `(string): string` | 换行符转 `<br>` |
| `is_null` | `(any): bool` | 检查是否为 null |
| `is_int` | `(any): bool` | 检查是否为整数 |
| `is_string` | `(any): bool` | 检查是否为字符串 |
| `is_bool` | `(any): bool` | 检查是否为布尔值 |
| `is_float` | `(any): bool` | 检查是否为浮点数 |
| `is_array` | `(any): bool` | 检查是否为数组或字典 |
| `gettype` | `(any): string` | 获取变量类型名 |
| `isset` | `(any): bool` | 检查变量是否非 null |
| `empty` | `(any): bool` | 检查变量是否为空 |
| `unset` | `(any): void` | 销毁变量 (暂为 no-op) |
| `abs` | `(any): int` | 绝对值 |
| `ceil` | `(float): int` | 向上取整 |
| `floor` | `(float): int` | 向下取整 |
| `round` | `(float, int): float` | 四舍五入 |
| `max` | `(any, any): any` | 取最大值 |
| `min` | `(any, any): any` | 取最小值 |
| `rand` | `(int, int): int` | 随机整数 |
| `mt_rand` | `(int, int): int` | 随机整数 (同 rand) |
| `pow` | `(any, any): float` | 求幂 |
| `sqrt` | `(float): float` | 平方根 |
| `time` | `(): int` | 当前 Unix 时间戳 |
| `date` | `(string, int): string` | 格式化日期时间 |
| `strtotime` | `(string): int` | 日期字符串转时间戳 |
| `microtime` | `(): string` | 当前时间戳(含微秒) |
| `json_encode` | `(any): string` | 值转 JSON 字符串 |
| `json_decode` | `(string): any` | JSON 字符串转值 |
| `file_get_contents` | `(string): string` | 读取文件内容 |
| `file_put_contents` | `(string, string): int` | 写入文件 |
| `file_exists` | `(string): bool` | 检查文件是否存在 |
| `is_dir` | `(string): bool` | 检查是否为目录 |
| `mkdir` | `(string): bool` | 创建目录 |
| `unlink` | `(string): bool` | 删除文件 |
| `basename` | `(string): string` | 提取路径中的文件名 |
| `dirname` | `(string): string` | 提取路径中的目录名 |
| `scandir` | `(string): array` | 列出目录内容 |
| `copy` | `(string, string): bool` | 拷贝文件 |
| `rename` | `(string, string): bool` | 重命名/移动文件 |
| `filesize` | `(string): int` | 获取文件大小 |
| `filemtime` | `(string): int` | 获取文件修改时间 |
| `pathinfo` | `(string): string` | 解析路径信息(JSON) |
| `move_uploaded_file` | `(string, string): bool` | 移动上传文件 |
| `array_push` | `(array, any): array` | 向数组末尾添加元素 |
| `array_pop` | `(array): any` | 弹出数组最后一个元素 |
| `array_shift` | `(array): any` | 移除数组第一个元素 |
| `array_unshift` | `(array, any): array` | 向数组开头添加元素 |
| `array_keys` | `(any): array` | 返回所有键名 |
| `array_values` | `(any): array` | 返回所有值 |
| `array_merge` | `(any, any): array` | 合并数组 |
| `array_flip` | `(any): dict` | 交换键和值 |
| `in_array` | `(any, array, bool): bool` | 检查值是否在数组中 |
| `array_search` | `(any, any, bool): any` | 搜索值并返回键名 |
| `array_key_exists` | `(any, any): bool` | 检查键是否存在 |
| `array_slice` | `(array, int, int): array` | 截取数组的一部分 |
| `array_sum` | `(array): float` | 计算数组值的和 |
| `array_unique` | `(array): array` | 移除重复值 |
| `array_reverse` | `(array): array` | 反转数组 |
| `array_filter` | `(array): array` | 过滤数组中的假值 |
| `array_map` | `(any, array): array` | 对每个元素应用回调 |
| `array_reduce` | `(array, any, any): any` | 用回调归约数组 |
| `range` | `(int, int, int): array` | 创建范围数组 |
| `sort` | `(array): array` | 对数组排序 |
| `rsort` | `(array): array` | 对数组逆序排序 |
| `array_diff` | `(array, array): array` | 数组差集 |
| `array_combine` | `(array, array): dict` | 两数组合并为字典 |
| `array_column` | `(array, string): array` | 提取二维数组某一列 |
| `array_fill` | `(int, int, any): array` | 填充数组 |
| `array_rand` | `(any, int): any` | 随机取出键名 |
| `array_chunk` | `(array, int, bool): array` | 数组分块 |
| `array_count_values` | `(array): dict` | 统计值出现次数 |
| `array_product` | `(array): int\|float` | 数组值乘积 |
| `array_intersect` | `(array, array): array` | 数组交集 |
| `str_replace` | `(string, string, string): string` | 子串替换 (PHP 兼容) |
| `ltrim` | `(string): string` | 去除左侧空白 |
| `rtrim` | `(string): string` | 去除右侧空白 |
| `strrpos` | `(string, string): int` | 查找子串最后位置 |
| `ucfirst` | `(string): string` | 首字母大写 |
| `sprintf` | `(string, string, string, string, string): string` | 格式化字符串 |
| `number_format` | `(any, int): string` | 数字千位格式化 |
| `chr` | `(int): string` | 码点转 UTF-8 字符 |
| `ord` | `(string): int` | 字符转码点 |
| `addslashes` | `(string): string` | 转义特殊字符 |
| `stripslashes` | `(string): string` | 去除转义 |
| `urlencode` | `(string): string` | URL 编码 |
| `urldecode` | `(string): string` | URL 解码 |
| `parse_url` | `(string): string` | 解析 URL |
| `http_build_query` | `(any): string` | 构建查询字符串 |
| `base64_encode` | `(string): string` | Base64 编码 |
| `base64_decode` | `(string): string` | Base64 解码 |
| `md5` | `(string): string` | MD5 哈希 |
| `sha1` | `(string): string` | SHA1 哈希 |
| `uniqid` | `(string): string` | 生成唯一 ID |
| `password_hash` | `(string, string): string` | 密码哈希 |
| `password_verify` | `(string, string): bool` | 验证密码 |
| `random_bytes` | `(int): string` | 生成安全随机字节(hex) |
| `random_int` | `(int, int): int` | 生成安全随机整数 |
| `sleep` | `(int): void` | 暂停(秒) |
| `usleep` | `(int): void` | 暂停(微秒) |
| `realpath` | `(string): string` | 规范化路径 |
| `is_file` | `(string): int` | 检查是否为文件 |
| `getcwd` | `(): string` | 当前工作目录 |
| `curl` | `(string, dict): dict` | 同步 HTTP 请求 |
| `curl_async` | `(string, dict): int` | 异步 HTTP 请求（返回句柄） |
| `curl_wait` | `(int): dict` | 等待异步请求完成 |
| `curl_is_done` | `(int): bool` | 检查异步请求是否完成 |
| `intval` | `(mixed, int): int` | 转换为整数 |
| `floatval` | `(mixed): float` | 转换为浮点数 |
| `strval` | `(mixed): string` | 转换为字符串 |
| `boolval` | `(mixed): bool` | 转换为布尔值 |
| `str_pad` | `(string, int, string, int): string` | 填充字符串 |
| `wordwrap` | `(string, int, string, bool): string` | 文本换行 |
| `str_word_count` | `(string): int` | 统计单词数 |
| `chunk_split` | `(string, int, string): string` | 分块插入分隔符 |
| `printf` | `(string, ...): void` | 格式化输出 |
| `str_starts_with` | `(string, string): bool` | 检查前缀 |
| `str_ends_with` | `(string, string): bool` | 检查后缀 |
| `array_splice` | `(array, int, int): array` | 移除数组切片 |
| `array_pad` | `(array, int, mixed): array` | 填充数组 |
| `array_key_first` | `(array): mixed` | 第一个键 |
| `array_key_last` | `(array): mixed` | 最后一个键 |
| `array_is_list` | `(array): bool` | 是否为列表 |
| `fmod` | `(float, float): float` | 浮点取模 |
| `intdiv` | `(int, int): int` | 整数除法 |
| `checkdate` | `(int, int, int): bool` | 验证日期合法性 |
| `mktime` | `(int, int, int, int, int, int): int` | 构造时间戳 |
| `phprs_client_ip` | `(int): string` | 获取客户端 IP |
| `phprs_config` | `(string): void` | 服务器 JSON 配置 |
| `phprs_config_max_body` | `(int): void` | 设置最大请求体大小 |
| `phprs_config_timeout` | `(int, int): void` | 设置读写超时 |
| `phprs_config_max_connections` | `(int): void` | 设置最大连接数 |
| `phprs_is_shutting_down` | `(): int` | 检查是否正在关闭 |
| `phprs_write_pidfile` | `(string): void` | 写入 PID 文件 |
| `phprs_log` | `(string): void` | 写入访问日志 |
| `phprs_log_error_msg` | `(string): void` | 写入错误日志 |
| `phprs_log_init` | `(string): void` | 设置日志文件路径 |
| `phprs_rate_limit_init` | `(int, int): void` | 初始化限流器 |
| `phprs_rate_limit_check` | `(string): int` | 检查 IP 限流 |
| `phprs_cors_set_config` | `(string, string, string): void` | 配置 CORS |
| `phprs_cors_get_origin` | `(): string` | 获取 CORS Origin |
| `phprs_cors_get_methods` | `(): string` | 获取 CORS Methods |
| `phprs_cors_get_headers` | `(): string` | 获取 CORS Headers |
| `phprs_cors_is_preflight` | `(string): int` | 检测 CORS 预检请求 |
| `phprs_thread_pool_init` | `(int): void` | 初始化线程池 |
| `phprs_thread_pool_enqueue` | `(int, string, string): void` | 入队请求 |
| `phprs_thread_pool_shutdown` | `(): void` | 关闭线程池 |
| `phprs_app_set_routes` | `(string): void` | 存储路由配置 |
| `phprs_app_get_routes` | `(): string` | 获取路由配置 |
| `phprs_app_set_port` | `(int): void` | 存储服务器端口 |
| `phprs_app_get_port` | `(): int` | 获取服务器端口 |
| `phprs_request_parse` | `(string): string` | 解析原始请求为 JSON |
| `phprs_redis_init` | `(string, int, string): void` | 初始化 Redis 连接池 |
| `phprs_redis_close` | `(): void` | 关闭所有 Redis 连接 |
| `phprs_redis_get` | `(string): string` | Redis GET |
| `phprs_redis_set` | `(string, string): string` | Redis SET |
| `phprs_redis_setex` | `(string, int, string): string` | Redis SETEX |
| `phprs_redis_del` | `(string): string` | Redis DEL |
| `phprs_redis_exists` | `(string): int` | Redis EXISTS |
| `phprs_redis_expire` | `(string, int): int` | Redis EXPIRE |
| `phprs_redis_ttl` | `(string): int` | Redis TTL |
| `phprs_redis_incr` | `(string): int` | Redis INCR |
| `phprs_redis_decr` | `(string): int` | Redis DECR |
| `phprs_redis_hset` | `(string, string, string): string` | Redis HSET |
| `phprs_redis_hget` | `(string, string): string` | Redis HGET |
| `phprs_redis_hgetall` | `(string): string` | Redis HGETALL |
| `phprs_redis_lpush` | `(string, string): string` | Redis LPUSH |
| `phprs_redis_rpush` | `(string, string): string` | Redis RPUSH |
| `phprs_redis_lrange` | `(string, int, int): string` | Redis LRANGE |
| `phprs_redis_keys` | `(string): string` | Redis KEYS |
| `phprs_redis_ping` | `(): string` | Redis PING |
| `phprs_redis_select` | `(int): string` | Redis SELECT |
| `phprs_redis_cmd` | `(string): string` | Redis 原始命令 |
| `phprs_mysql_init` | `(string, int, string, string, string): void` | 初始化 MySQL 连接池 |
| `phprs_mysql_close` | `(): void` | 关闭所有 MySQL 连接 |
| `phprs_mysql_query` | `(string): string` | 执行 SQL 查询 |
| `phprs_mysql_exec` | `(string): string` | 执行非查询 SQL |
| `phprs_mysql_select` | `(string, string): string` | 便捷 SELECT |
| `phprs_mysql_insert` | `(string, string): string` | 便捷 INSERT |
| `phprs_mysql_update` | `(string, string, string): string` | 便捷 UPDATE |
| `phprs_mysql_delete` | `(string, string): string` | 便捷 DELETE |
| `phprs_mysql_escape` | `(string): string` | SQL 转义 |
| `phprs_ws_manager_init` | `(int): void` | 初始化 WS 连接管理器 |
| `phprs_ws_register` | `(int, string): int` | 注册 WS 连接 |
| `phprs_ws_unregister` | `(int): void` | 注销 WS 连接 |
| `phprs_ws_update_pong` | `(int): void` | 更新 Pong 时间戳 |
| `phprs_ws_broadcast` | `(string, string, int): int` | 房间广播 |
| `phprs_ws_broadcast_all` | `(string, int): int` | 全局广播 |
| `phprs_ws_count` | `(string): int` | WS 连接数 |
| `phprs_ws_rooms` | `(): string` | 获取房间列表 |
| `phprs_ws_start_heartbeat` | `(int): void` | 启动心跳线程 |

---

## 服务器配置与生产特性

### `phprs_config(string $json): void`

通过 JSON 字符串一次性配置服务器参数。

| 键 | 类型 | 默认值 | 说明 |
|-----|------|---------|-------------|
| `threads` | int | 8 | 线程池工作线程数 |
| `max_body` | string | `"10m"` | 最大请求体大小 |
| `read_timeout` | int | 30 | 读超时（秒） |
| `write_timeout` | int | 60 | 写超时（秒） |
| `max_connections` | int | 10000 | 最大并发连接数 |
| `log` | string | `"-"` | 访问日志路径（`"-"` 为 stdout） |
| `error_log` | string | `"-"` | 错误日志路径（`"-"` 为 stderr） |
| `pidfile` | string | — | PID 文件路径 |

```php
phprs_config("{\"threads\":8,\"max_body\":\"10m\",\"read_timeout\":30,\"log\":\"-\"}");
```

### `phprs_config_max_body(int $bytes): void`

设置请求体大小上限（字节）。超过限制的请求返回 413。

```php
phprs_config_max_body(20 * 1024 * 1024);  // 20 MB
```

### `phprs_config_timeout(int $read_sec, int $write_sec): void`

设置 socket 读写超时。

```php
phprs_config_timeout(30, 60);
```

### `phprs_config_max_connections(int $max): void`

设置最大并发连接数。超出限制的连接被拒绝。

```php
phprs_config_max_connections(5000);
```

### `phprs_is_shutting_down(): int`

检查服务器是否正在优雅关闭（收到 SIGTERM/SIGINT）。返回 1 表示正在关闭。

```php
while (phprs_is_shutting_down() == 0) {
    let $client = phprs_server_accept($server);
    // 处理请求...
}
echo "Server shutting down gracefully...";
```

### `phprs_write_pidfile(string $path): void`

将当前进程 PID 写入文件，便于进程管理。

```php
phprs_write_pidfile("/var/run/myapp.pid");
```

### 信号处理

运行时自动安装以下信号处理：

| 信号 | 行为 |
|--------|----------|
| `SIGTERM` | 优雅关闭（设置 `phprs_is_shutting_down()` 标志） |
| `SIGINT` | 优雅关闭（Ctrl+C） |
| `SIGHUP` | 重新打开日志文件（日志轮转） |
| `SIGPIPE` | 忽略（防止写入断开连接时崩溃） |

### 安全响应头

运行时自动注入以下安全响应头：

- `X-Content-Type-Options: nosniff`
- `X-Frame-Options: SAMEORIGIN`
- `X-XSS-Protection: 1; mode=block`
- `Referrer-Policy: strict-origin-when-cross-origin`
- `X-Request-Id: {自增ID}`（请求追踪）

---

## 日志与可观测性

### `phprs_log(string $msg): void`

写入访问日志（带时间戳）。

```php
phprs_log("Application started");
phprs_log("Processing request for user: " . $user_id);
```

### `phprs_log_error_msg(string $msg): void`

写入错误日志（带时间戳）。

```php
phprs_log_error_msg("Database connection failed");
phprs_log_error_msg("Invalid input: " . $raw_input);
```

### `phprs_log_init(string $path): void`

设置访问日志文件路径。`"-"` 表示标准输出。

```php
phprs_log_init("/var/log/myapp/access.log");
```

### 内置端点

运行时自动处理以下端点（在用户请求处理器之前）：

#### `GET /health` — 健康检查

返回 JSON：
```json
{"status":"ok","uptime":3600,"active_connections":42,"total_requests":12345,"queue_depth":2}
```

#### `GET /metrics` — Prometheus 指标

返回 Prometheus 格式文本：
```
# HELP phprs_requests_total Total HTTP requests processed
phprs_requests_total 12345
# HELP phprs_active_connections Current active connections
phprs_active_connections 42
# HELP phprs_uptime_seconds Server uptime in seconds
phprs_uptime_seconds 3600
```

### 访问日志格式

每个请求自动记录：
```
[2025-07-08T14:30:00] 127.0.0.1 "GET /api/users" 200 1234 0.5ms req_id=42
```

### 日志轮转

发送 `SIGHUP` 信号重新打开日志文件（配合 logrotate 使用）：
```bash
kill -HUP $(cat /var/run/myapp.pid)
```

---

## 限流器

### `phprs_rate_limit_init(int $max_requests, int $window_sec): void`

初始化 IP 限流器。使用 4096 个哈希桶的滑动窗口算法。

```php
phprs_rate_limit_init(100, 60);  // 每分钟最多 100 次请求
```

### `phprs_rate_limit_check(string $ip): int`

检查 IP 是否在限流范围内。返回 1 表示允许，0 表示被限制。

```php
let $ip = phprs_client_ip($client);
if (phprs_rate_limit_check($ip) == 0) {
    let $resp = phprs_http_response(429, "text/plain", "Too Many Requests");
    phprs_socket_write($client, $resp);
    phprs_socket_close($client);
    return;
}
```

---

## CORS 跨域资源共享

### `phprs_cors_set_config(string $origin, string $methods, string $headers): void`

配置 CORS 策略。

```php
phprs_cors_set_config(
    "https://myapp.com",
    "GET,POST,PUT,DELETE,OPTIONS",
    "Content-Type,Authorization,X-Custom-Header"
);
```

### `phprs_cors_get_origin(): string`

获取配置的 CORS Origin。默认：`"*"`。

### `phprs_cors_get_methods(): string`

获取配置的 CORS Methods。默认：`"GET,POST,PUT,DELETE,PATCH,OPTIONS"`。

### `phprs_cors_get_headers(): string`

获取配置的 CORS Headers。默认：`"Content-Type,Authorization"`。

### `phprs_cors_is_preflight(string $raw): int`

检测是否为 CORS 预检请求（OPTIONS）。返回 1 表示是。

```php
if (phprs_cors_is_preflight($raw) == 1) {
    let $resp = phprs_http_response(204, "", "");
    // 添加 CORS 头并响应...
    phprs_socket_write($client, $resp);
    phprs_socket_close($client);
    return;
}
```

---

## 线程池

### `phprs_thread_pool_init(int $num_workers): void`

初始化工作线程池。每个线程有独立的 256KB 内存竞技场。

```php
phprs_thread_pool_init(8);  // 8 个工作线程
```

### `phprs_thread_pool_enqueue(int $fd, string $raw_request, string $client_ip): void`

将请求入队，由线程池中的工作线程处理。自动完成：
- 请求计时（`clock_gettime`）
- 内存竞技场重置
- 安全头注入
- 请求 ID 生成
- 访问日志记录

```php
phprs_register_handler();  // 注册请求处理函数
phprs_thread_pool_init(8);

while (phprs_is_shutting_down() == 0) {
    let $client = phprs_server_accept($server);
    if ($client >= 0) {
        let $raw = phprs_socket_read($client, 65536);
        let $ip = phprs_client_ip($client);
        phprs_thread_pool_enqueue($client, $raw, $ip);
    }
}
```

### `phprs_thread_pool_shutdown(): void`

优雅关闭线程池，等待所有队列请求处理完成。

```php
phprs_thread_pool_shutdown();
```

---

## 应用状态（线程安全全局变量）

### `phprs_app_set_routes(string $routes): void`

存储路由配置（线程安全，互斥锁保护）。

```php
phprs_app_set_routes("GET /=home,GET /api/users=api_users,POST /api/users=api_create_user");
```

### `phprs_app_get_routes(): string`

获取已存储的路由配置。

### `phprs_app_set_port(int $port): void`

存储服务器端口号。

### `phprs_app_get_port(): int`

获取已存储的端口号。

---

## 请求解析

### `phprs_request_parse(string $raw): string`

将原始 HTTP 请求解析为 JSON 对象。

```php
let $req = phprs_request_parse($raw);
// 返回: {"method":"POST","path":"/api/users","query":"page=1","content_type":"application/json","body":"{...}","cookie":"sid=abc123"}

let $method = phprs_json_get_string($req, "method");
let $path = phprs_json_get_string($req, "path");
let $body = phprs_json_get_string($req, "body");
```

---

## Redis 客户端（连接池）

使用 RESP 协议的 Redis 客户端。连接池大小：8，带缓冲读取器（8KB）和 PING 健康检查。

### `phprs_redis_init(string $host, int $port, string $password): void`

初始化 Redis 连接池。

```php
phprs_redis_init("127.0.0.1", 6379, "my_password");
```

### `phprs_redis_close(): void`

关闭所有连接池中的连接。

```php
phprs_redis_close();
```

### 字符串命令

#### `phprs_redis_get(string $key): string`

获取键值。未找到返回 `"(nil)"`。

```php
let $value = phprs_redis_get("user:1:name");
if ($value != "(nil)") {
    echo "Name: " . $value;
}
```

#### `phprs_redis_set(string $key, string $value): string`

设置键值对。

```php
phprs_redis_set("user:1:name", "Alice");
```

#### `phprs_redis_setex(string $key, int $seconds, string $value): string`

设置带过期时间的键值对。

```php
phprs_redis_setex("session:abc123", 3600, $session_data);  // 1 小时过期
```

#### `phprs_redis_del(string $key): string`

删除键。

```php
phprs_redis_del("user:1:name");
```

#### `phprs_redis_exists(string $key): int`

检查键是否存在。返回 1 存在，0 不存在。

```php
if (phprs_redis_exists("session:abc123") == 1) {
    echo "Session is active";
}
```

#### `phprs_redis_expire(string $key, int $seconds): int`

设置键的 TTL。

```php
phprs_redis_expire("cache:page:home", 300);  // 5 分钟
```

#### `phprs_redis_ttl(string $key): int`

获取剩余 TTL（秒）。-1 表示永不过期，-2 表示不存在。

```php
let $ttl = phprs_redis_ttl("session:abc123");
echo "Expires in: " . $ttl . " seconds";
```

#### `phprs_redis_incr(string $key): int`

原子递增。

```php
let $views = phprs_redis_incr("page:views:home");
echo "Total views: " . $views;
```

#### `phprs_redis_decr(string $key): int`

原子递减。

```php
let $stock = phprs_redis_decr("product:1:stock");
```

### 哈希命令

#### `phprs_redis_hset(string $key, string $field, string $value): string`

设置哈希字段。

```php
phprs_redis_hset("user:1", "name", "Alice");
phprs_redis_hset("user:1", "email", "alice@example.com");
phprs_redis_hset("user:1", "age", "30");
```

#### `phprs_redis_hget(string $key, string $field): string`

获取哈希字段值。

```php
let $name = phprs_redis_hget("user:1", "name");  // "Alice"
```

#### `phprs_redis_hgetall(string $key): string`

获取哈希所有字段和值。返回 JSON 数组 `["field1","val1","field2","val2",...]`。

```php
let $user = phprs_redis_hgetall("user:1");
// ["name","Alice","email","alice@example.com","age","30"]
```

### 列表命令

#### `phprs_redis_lpush(string $key, string $value): string`

从左端插入列表。

```php
phprs_redis_lpush("queue:tasks", "send_email");
phprs_redis_lpush("queue:tasks", "process_image");
```

#### `phprs_redis_rpush(string $key, string $value): string`

从右端插入列表。

```php
phprs_redis_rpush("log:events", "user_login");
```

#### `phprs_redis_lrange(string $key, int $start, int $stop): string`

获取列表指定范围的元素。返回 JSON 数组。

```php
let $tasks = phprs_redis_lrange("queue:tasks", 0, -1);
// ["process_image","send_email"]
```

### 工具命令

#### `phprs_redis_keys(string $pattern): string`

查找匹配模式的键。返回 JSON 数组。

```php
let $user_keys = phprs_redis_keys("user:*");
// ["user:1","user:2","user:3"]
```

#### `phprs_redis_ping(): string`

Ping 服务器。连接正常返回 `"PONG"`。

```php
let $pong = phprs_redis_ping();
echo $pong;  // "PONG"
```

#### `phprs_redis_select(int $db): string`

切换数据库（0-15）。

```php
phprs_redis_select(1);  // 切换到数据库 1
```

#### `phprs_redis_cmd(string $command): string`

执行原始 Redis 命令（空格分隔）。

```php
let $result = phprs_redis_cmd("SET mykey myvalue EX 3600");
let $info = phprs_redis_cmd("INFO server");
```

### Redis 完整示例

```php
<?phprs
// 初始化
phprs_redis_init("127.0.0.1", 6379, "redis_123");

// 字符串操作
phprs_redis_set("counter", "0");
let $val = phprs_redis_incr("counter");  // 1
let $val = phprs_redis_incr("counter");  // 2

// 带过期时间的缓存
phprs_redis_setex("cache:user:1", 300, "{\"name\":\"Alice\",\"age\":30}");

// 哈希操作
phprs_redis_hset("config", "app_name", "MyApp");
phprs_redis_hset("config", "version", "1.0");
let $name = phprs_redis_hget("config", "app_name");

// 列表作为队列
phprs_redis_rpush("queue:emails", "user1@example.com");
phprs_redis_rpush("queue:emails", "user2@example.com");
let $pending = phprs_redis_lrange("queue:emails", 0, -1);

// 关闭
phprs_redis_close();
?>
```

---

## MySQL 客户端（连接池）

通过 `libmysqlclient` 实现的 MySQL 客户端。连接池大小：8，带 `mysql_ping()` 健康检查。

> **注意**：编译时需要 `-DPHPRS_HAS_MYSQL -lmysqlclient`。PHPRS 编译器会自动检测已安装的 MySQL。

### `phprs_mysql_init(string $host, int $port, string $user, string $pass, string $dbname): void`

初始化 MySQL 连接池。

```php
phprs_mysql_init("127.0.0.1", 3306, "root", "password", "myapp");
```

### `phprs_mysql_close(): void`

关闭所有连接池中的连接。

```php
phprs_mysql_close();
```

### `phprs_mysql_query(string $sql): string`

执行 SQL 查询。SELECT 返回 JSON 对象数组，INSERT/UPDATE/DELETE 返回 `{"affected_rows":N,"insert_id":M}`。

```php
// SELECT
let $users = phprs_mysql_query("SELECT id, name, email FROM users WHERE active = 1");
// [{"id":1,"name":"Alice","email":"alice@example.com"},{"id":2,"name":"Bob","email":"bob@example.com"}]

// INSERT
let $result = phprs_mysql_query("INSERT INTO users (name, email) VALUES ('Carol', 'carol@example.com')");
// {"affected_rows":1,"insert_id":3}

// UPDATE
let $result = phprs_mysql_query("UPDATE users SET active = 0 WHERE id = 5");
// {"affected_rows":1,"insert_id":0}

// DELETE
let $result = phprs_mysql_query("DELETE FROM sessions WHERE expires_at < NOW()");
// {"affected_rows":15,"insert_id":0}
```

### `phprs_mysql_exec(string $sql): string`

`phprs_mysql_query()` 的别名 — 用于非查询语句提高可读性。

```php
phprs_mysql_exec("CREATE TABLE IF NOT EXISTS users (id INT AUTO_INCREMENT PRIMARY KEY, name VARCHAR(255))");
```

### `phprs_mysql_select(string $table, string $where): string`

便捷方法：`SELECT * FROM table WHERE ...`。返回 JSON 数组。

```php
let $users = phprs_mysql_select("users", "active = 1 ORDER BY name");
let $all = phprs_mysql_select("products", "");  // 无 WHERE 条件
```

### `phprs_mysql_insert(string $table, string $json_data): string`

便捷方法：从 JSON 对象插入行。

```php
let $result = phprs_mysql_insert("users", "{\"name\":\"Dave\",\"email\":\"dave@example.com\",\"age\":25}");
// {"affected_rows":1,"insert_id":4}
```

### `phprs_mysql_update(string $table, string $set, string $where): string`

便捷方法：`UPDATE table SET ... WHERE ...`。

```php
let $result = phprs_mysql_update("users", "name='David'", "id = 4");
```

### `phprs_mysql_delete(string $table, string $where): string`

便捷方法：`DELETE FROM table WHERE ...`。

```php
let $result = phprs_mysql_delete("sessions", "expires_at < NOW()");
```

### `phprs_mysql_escape(string $s): string`

转义字符串用于安全 SQL（防止 SQL 注入）。

```php
let $safe_name = phprs_mysql_escape($user_input);
let $sql = "SELECT * FROM users WHERE name = '" . $safe_name . "'";
let $result = phprs_mysql_query($sql);
```

### MySQL 完整示例

```php
<?phprs
// 初始化
phprs_mysql_init("127.0.0.1", 3306, "root", "password", "myapp");

// 创建表
phprs_mysql_exec("CREATE TABLE IF NOT EXISTS users (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    age INT DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
)");

// 插入
let $r = phprs_mysql_insert("users", "{\"name\":\"Alice\",\"email\":\"alice@example.com\",\"age\":30}");
let $user_id = phprs_json_get_int($r, "insert_id");

// 查询
let $users = phprs_mysql_query("SELECT * FROM users ORDER BY id DESC LIMIT 10");

// 更新
phprs_mysql_update("users", "age = 31", "id = " . $user_id);

// 安全查询
let $name = phprs_mysql_escape($input);
let $results = phprs_mysql_query("SELECT * FROM users WHERE name = '" . $name . "'");

// 关闭
phprs_mysql_close();
?>
```

---

## WebSocket 连接管理器

在基本 WebSocket API 之上提供连接管理、房间系统和自动心跳功能。最大支持 1024 个并发连接。

### `phprs_ws_manager_init(int $heartbeat_sec): void`

初始化 WebSocket 连接管理器。

```php
phprs_ws_manager_init(30);  // 30 秒心跳间隔
```

### `phprs_ws_register(int $fd, string $room): int`

将 WebSocket 连接注册到管理器中，分配到指定房间。返回槽位索引，-1 表示已满。

```php
let $slot = phprs_ws_register($client, "chat:general");
```

### `phprs_ws_unregister(int $fd): void`

从管理器中移除连接。

```php
phprs_ws_unregister($client);
```

### `phprs_ws_update_pong(int $fd): void`

更新连接的最后 Pong 时间戳（收到 Pong 帧时调用）。

```php
phprs_ws_update_pong($client);
```

### `phprs_ws_broadcast(string $room, string $message, int $exclude_fd): int`

向指定房间内的所有客户端广播消息，可排除一个连接。返回发送成功的数量。

```php
let $sent = phprs_ws_broadcast("chat:general", "{\"type\":\"message\",\"text\":\"Hello!\"}", $sender_fd);
echo "Sent to " . $sent . " clients";
```

### `phprs_ws_broadcast_all(string $message, int $exclude_fd): int`

向所有连接的客户端广播（跨所有房间）。

```php
phprs_ws_broadcast_all("{\"type\":\"announcement\",\"text\":\"Server will restart\"}", -1);
```

### `phprs_ws_count(string $room): int`

获取活跃连接数。传空字符串获取总数。

```php
let $total = phprs_ws_count("");            // 所有连接
let $chat = phprs_ws_count("chat:general"); // 某房间连接
```

### `phprs_ws_rooms(): string`

获取所有活跃房间名称的 JSON 数组。

```php
let $rooms = phprs_ws_rooms();
// ["chat:general","chat:vip","game:lobby"]
```

### `phprs_ws_start_heartbeat(int $interval_sec): void`

启动心跳后台线程。按设定间隔发送 Ping 帧，超过 3 倍间隔未响应的连接自动断开。

```php
phprs_ws_start_heartbeat(30);
```

### WebSocket 完整示例

```php
<?phprs
include "system/runtime.phprs";

// 初始化 WebSocket 管理器和心跳
phprs_ws_manager_init(30);
phprs_ws_start_heartbeat(30);

function handle_websocket(int $client, string $raw): void {
    // WebSocket 握手
    let $ws_resp = phprs_ws_handshake_response($raw);
    phprs_socket_write($client, $ws_resp);

    // 注册到聊天房间
    phprs_ws_register($client, "chat:general");

    // 消息循环
    while (true) {
        let $msg = phprs_ws_read_frame($client, 30000);
        if ($msg == "") {
            break;  // 连接关闭或超时
        }

        // 广播给房间内其他人
        let $broadcast_msg = "{\"type\":\"message\",\"data\":\"" . $msg . "\"}";
        phprs_ws_broadcast("chat:general", $broadcast_msg, $client);
    }

    // 清理
    phprs_ws_unregister($client);
    phprs_ws_close($client);
}
?>
```

---

## 生产特性总览

| 特性 | 详情 |
|---------|---------|
| **线程池** | 固定大小工作线程池，请求队列，每请求 256KB 内存竞技场 |
| **连接限制** | 可配置最大并发连接数（默认 10,000） |
| **请求体限制** | 最大请求体大小（默认 10 MB），超过返回 413 |
| **Socket 超时** | 读（30s）写（60s）超时 |
| **安全响应头** | 自动注入 X-Content-Type-Options、X-Frame-Options、X-XSS-Protection、Referrer-Policy |
| **请求 ID** | 每个响应自动生成 X-Request-Id |
| **访问日志** | 每请求记录，包含真实延迟测量 |
| **错误日志** | 带时间戳的错误日志，可配置输出 |
| **健康检查** | GET /health — 上报 uptime、连接数、队列深度 |
| **Prometheus 指标** | GET /metrics — Prometheus 格式指标 |
| **IP 限流** | 基于 IP 的滑动窗口限流（4096 桶） |
| **优雅关闭** | SIGTERM/SIGINT 处理，队列排空 |
| **日志轮转** | SIGHUP 触发日志文件重新打开 |
| **PID 文件** | 进程管理 |
| **内存池** | 每线程 256KB bump allocator，每请求重置 |
| **内存监控** | 总池内存追踪，超过 512MB 拒绝新请求 |
| **Redis 连接池** | 8 连接，PING 健康检查 |
| **MySQL 连接池** | 8 连接，mysql_ping() 健康检查 |
| **WebSocket 管理器** | 1024 客户端，房间系统，自动心跳 |
| **CORS** | 可配置 origin、methods、headers |
| **TLS/HTTPS 客户端** | OpenSSL (POSIX) / Schannel (Windows) |
