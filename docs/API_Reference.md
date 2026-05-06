# PHPRS API Reference

PHPRS 所有内置函数与运行时系统函数的完整参考。

## 目录

1. [语言内置函数](#语言内置函数)
2. [异常处理 (try/catch/throw)](#异常处理-trycatchthrow)
3. [类型检查函数](#类型检查函数)
4. [字符串处理](#字符串处理)
5. [URL 与编码函数](#url-与编码函数)
6. [数组函数](#数组函数)
7. [数学函数](#数学函数)
8. [日期时间函数](#日期时间函数)
9. [JSON 函数](#json-函数)
10. [文件系统函数](#文件系统函数)
11. [哈希与安全函数](#哈希与安全函数)
12. [语言特性](#语言特性)
13. [低级字符串函数 (phprs_* 前缀)](#低级字符串函数-phprs_-前缀)
14. [Socket 网络原语](#socket-网络原语)
15. [网络连接](#网络连接)
16. [HTTP 请求解析（服务端）](#http-请求解析服务端)
17. [HTTP 响应构建（服务端）](#http-响应构建服务端)
18. [HTTP 客户端](#http-客户端)
19. [WebSocket](#websocket)
20. [多线程](#多线程)
21. [curl HTTP 客户端](#curl--新一代-http-客户端)

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

```php
let $client_fd = phprs_server_accept($server_fd);
if ($client_fd >= 0) {
    // 处理客户端连接
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
