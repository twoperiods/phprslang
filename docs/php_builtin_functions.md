# PHP 内置函数清单

> 数据来源: [PHP 官方手册 - 语言参考](https://www.php.net/manual/zh/langref.php)
> 共计 **3548** 个内置函数

---

## 按字母排序

### _

| 函数名 | 功能描述 |
|--------|----------|
| `_` | 别名 gettext |
| `__autoload` | 尝试加载未定义的类 |
| `__halt_compiler` | 中断编译器的执行 |

### A

| 函数名 | 功能描述 |
|--------|----------|
| `abs` | 绝对值 |
| `acos` | 反余弦 |
| `acosh` | 反双曲余弦 |
| `addcslashes` | 以 C 语言风格使用反斜线转义字符串中的字符 |
| `addslashes` | 使用反斜线引用字符串 |
| `apache_child_terminate` | 在本次请求结束后终止 apache 子进程 |
| `apache_get_modules` | 获得已加载的Apache模块列表 |
| `apache_get_version` | 获得Apache版本信息 |
| `apache_getenv` | 获取 Apache subprocess_env 变量 |
| `apache_lookup_uri` | 对指定的 URI 执行部分请求并返回所有有关信息 |
| `apache_note` | 取得或设置 apache 请求记录 |
| `apache_request_headers` | 获取全部 HTTP 请求 header |
| `apache_response_headers` | 获得全部 HTTP 响应 header |
| `apache_setenv` | 设置 Apache 子进程环境变量 |
| `apcu_add` | 缓存一个新变量到存储中 |
| `apcu_cache_info` | 从 APCu 存储中获取缓存信息 |
| `apcu_cas` | Updates an old value with a new value |
| `apcu_clear_cache` | Clears the APCu cache |
| `apcu_dec` | Decrease a stored number |
| `apcu_delete` | Removes a stored variable from the cache |
| `apcu_enabled` | Whether APCu is usable in the current environment |
| `apcu_entry` | Atomically fetch or generate a cache entry |
| `apcu_exists` | Checks if entry exists |
| `apcu_fetch` | Fetch a stored variable from the cache |
| `apcu_inc` | Increase a stored number |
| `apcu_key_info` | Get detailed information about the cache key |
| `apcu_sma_info` | Retrieves APCu Shared Memory Allocation information |
| `apcu_store` | 缓存一个变量到存储中 |
| `array` | 新建一个数组 |
| `array_all` | 检查数组所有元素是否都满足回调函数的条件 |
| `array_any` | Checks if at least one array element satisfies a callback function |
| `array_change_key_case` | 将数组中的所有键名修改为全大写或小写 |
| `array_chunk` | 将一个数组分割成多个 |
| `array_column` | 返回输入数组中指定列的值 |
| `array_combine` | 创建一个数组，用一个数组的值作为其键名，另一个数组的值作为其值 |
| `array_count_values` | 统计数组中每个不同值的出现次数 |
| `array_diff` | 计算数组的差集 |
| `array_diff_assoc` | 带索引检查计算数组的差集 |
| `array_diff_key` | 使用键名比较计算数组的差集 |
| `array_diff_uassoc` | 用用户提供的回调函数做索引检查来计算数组的差集 |
| `array_diff_ukey` | 用回调函数对键名比较计算数组的差集 |
| `array_fill` | 用给定的值填充数组 |
| `array_fill_keys` | 使用指定的键和值填充数组 |
| `array_filter` | 使用回调函数过滤数组的元素 |
| `array_find` | Returns the first element satisfying a callback function |
| `array_find_key` | Returns the key of the first element satisfying a callback function |
| `array_first` | 获取数组的第一个值 |
| `array_flip` | 交换数组中的键和值 |
| `array_intersect` | 计算数组的交集 |
| `array_intersect_assoc` | 带索引检查计算数组的交集 |
| `array_intersect_key` | 使用键名比较计算数组的交集 |
| `array_intersect_uassoc` | 带索引检查计算数组的交集，用回调函数比较索引 |
| `array_intersect_ukey` | 在键名上使用回调函数来比较计算数组的交集 |
| `array_is_list` | 判断指定 array 是否为 list |
| `array_key_exists` | 检查数组里是否有指定的键名或索引 |
| `array_key_first` | 获取指定数组的第一个键 |
| `array_key_last` | 获取一个数组的最后一个键值 |
| `array_keys` | 返回数组中部分的或所有的键名 |
| `array_last` | 获取数组的最后一个值 |
| `array_map` | 为数组的每个元素应用回调函数 |
| `array_merge` | 合并一个或多个数组 |
| `array_merge_recursive` | 递归地合并一个或多个数组 |
| `array_multisort` | 对多个数组或多维数组进行排序 |
| `array_pad` | 以指定长度将一个值填充进数组 |
| `array_pop` | 弹出数组最后一个单元（出栈） |
| `array_product` | 计算数组中所有值的乘积 |
| `array_push` | 将一个或多个单元压入数组的末尾（入栈） |
| `array_rand` | 从数组中随机取出一个或多个随机键 |
| `array_reduce` | 用回调函数迭代地将数组简化为单一的值 |
| `array_replace` | 使用传递的数组替换第一个数组的元素 |
| `array_replace_recursive` | 使用传递的数组递归替换第一个数组的元素 |
| `array_reverse` | 返回单元顺序相反的数组 |
| `array_search` | 在数组中搜索给定的值，如果成功则返回首个相应的键名 |
| `array_shift` | 将数组开头的单元移出数组 |
| `array_slice` | 从数组中取出一段 |
| `array_splice` | 去掉数组中的某一部分并用其它值取代 |
| `array_sum` | 对数组中所有值求和 |
| `array_udiff` | 用回调函数比较数据来计算数组的差集 |
| `array_udiff_assoc` | 带索引检查计算数组的差集，用回调函数比较数据 |
| `array_udiff_uassoc` | 带索引检查计算数组的差集，用回调函数比较数据和索引 |
| `array_uintersect` | 计算数组的交集，用回调函数比较数据 |
| `array_uintersect_assoc` | 带索引检查计算数组的交集，用回调函数比较数据 |
| `array_uintersect_uassoc` | 带索引检查计算数组的交集，用单独的回调函数比较数据和索引 |
| `array_unique` | 移除数组中重复的值 |
| `array_unshift` | 在数组开头插入一个或多个单元 |
| `array_values` | 返回数组中所有的值 |
| `array_walk` | 使用用户自定义函数对数组中的每个元素做回调处理 |
| `array_walk_recursive` | 对数组中的每个成员递归地应用用户函数 |
| `arsort` | 对数组进行降向排序并保持索引关系 |
| `asin` | 反正弦 |
| `asinh` | 反双曲正弦 |
| `asort` | 对数组进行升序排序并保持索引关系 |
| `assert` | 断言检测 |
| `assert_options` | 设置/获取各种断言 flag |
| `atan` | 反正切 |
| `atan2` | 两个参数的反正切 |
| `atanh` | 反双曲正切 |

### B

| 函数名 | 功能描述 |
|--------|----------|
| `base_convert` | 在任意进制之间转换数字 |
| `base64_decode` | 对使用 MIME base64 编码的数据进行解码 |
| `base64_encode` | 使用 MIME base64 对数据进行编码 |
| `basename` | 返回路径中的文件名部分 |
| `bcadd` | 两个任意精度数字的加法计算 |
| `bcceil` | Round up arbitrary precision number |
| `bccomp` | 比较两个任意精度的数字 |
| `bcdiv` | 两个任意精度的数字除法计算 |
| `bcdivmod` | Get the quotient and modulus of an arbitrary precision number |
| `bcfloor` | Round down arbitrary precision number |
| `bcmod` | 任意精度数字取模 |
| `bcmul` | 两个任意精度数字乘法计算 |
| `bcpow` | 任意精度数字的乘方 |
| `bcpowmod` | Raise an arbitrary precision number to another, reduced by a specified modulus |
| `bcround` | Round arbitrary precision number |
| `bcscale` | 设置/获取所有 bc math 函数的默认小数点保留位数 |
| `bcsqrt` | 任意精度数字的二次方根 |
| `bcsub` | 两个任意精度数字的减法 |
| `bin2hex` | 将二进制数据转换为十六进制表示 |
| `bind_textdomain_codeset` | Specify or get the character encoding in which the messages from the DOMAIN message catalog will be returned |
| `bindec` | 二进制转换为十进制 |
| `bindtextdomain` | Sets or gets the path for a domain |
| `boolval` | 获取变量的布尔值 |
| `bzclose` | 关闭一个 bzip2 文件 |
| `bzcompress` | 把一个字符串压缩成 bzip2 编码数据 |
| `bzdecompress` | 解压经 bzip2 编码过的数据 |
| `bzerrno` | 返回一个 bzip2 错误码 |
| `bzerror` | 返回包含 bzip2 错误号和错误字符串的一个 array |
| `bzerrstr` | 返回一个 bzip2 的错误字符串 |
| `bzflush` | 什么都不做 |
| `bzopen` | 打开 bzip2 压缩文件 |
| `bzread` | bzip2 文件二进制安全地读取 |
| `bzwrite` | 二进制安全地写入 bzip2 文件 |

### C

| 函数名 | 功能描述 |
|--------|----------|
| `cal_days_in_month` | 返回指定历法中某年某月的天数 |
| `cal_from_jd` | 从儒略日数转换为支持的历法 |
| `cal_info` | 返回选定历法的信息 |
| `cal_to_jd` | 从支持的历法转换为儒略日数 |
| `call_user_func` | 把第一个参数作为回调函数调用 |
| `call_user_func_array` | 调用回调函数，并把一个数组参数作为回调函数的参数 |
| `ceil` | 进一法取整 |
| `chdir` | 改变目录 |
| `checkdate` | 验证一个格里高里日期 |
| `checkdnsrr` | 给指定的主机（域名）或者IP地址做DNS通信检查 |
| `chgrp` | 改变文件所属的组 |
| `chmod` | 改变文件模式 |
| `chop` | rtrim 的别名 |
| `chown` | 改变文件的所有者 |
| `chr` | 从数字生成单字节字符串 |
| `chroot` | 改变根目录 |
| `chunk_split` | 将字符串分割成小块 |
| `class_alias` | 为类创建别名 |
| `class_exists` | 查类是否已经定义 |
| `class_implements` | 返回指定的类或接口实现的所有接口 |
| `class_parents` | 返回指定类的父类 |
| `class_uses` | Return the traits used by the given class |
| `clearstatcache` | 清除文件状态缓存 |
| `cli_get_process_title` | Returns the current process title |
| `cli_set_process_title` | Sets the process title |
| `closedir` | 关闭目录句柄 |
| `closelog` | 关闭系统日志链接 |
| `com_create_guid` | Generate a globally unique identifier (GUID) |
| `com_event_sink` | Connect events from a COM object to a PHP object |
| `com_get_active_object` | Returns a handle to an already running instance of a COM object |
| `com_load_typelib` | 载入 Typelib |
| `com_message_pump` | Process COM messages, sleeping for up to timeoutms milliseconds |
| `com_print_typeinfo` | Print out a PHP class definition for a dispatchable interface |
| `CommonMark\Parse` | Parsing |
| `CommonMark\Render` | Rendering |
| `CommonMark\Render\HTML` | Rendering |
| `CommonMark\Render\Latex` | Rendering |
| `CommonMark\Render\Man` | Rendering |
| `CommonMark\Render\XML` | Rendering |
| `compact` | 建立一个数组，包括变量名和它们的值 |
| `connection_aborted` | 检查客户端是否已经断开 |
| `connection_status` | 返回连接的状态位 |
| `constant` | 返回一个常量的值 |
| `convert_cyr_string` | 将字符由一种 Cyrillic 字符转换成另一种 |
| `convert_uudecode` | 解码一个 uuencode 编码的字符串 |
| `convert_uuencode` | 使用 uuencode 编码一个字符串 |
| `copy` | 拷贝文件 |
| `cos` | 余弦 |
| `cosh` | 双曲余弦 |
| `count` | 统计数组、Countable 对象中所有元素的数量 |
| `count_chars` | 返回字符串所用字符的信息 |
| `crc32` | 计算一个字符串的 crc32 多项式 |
| `create_function` | 通过执行代码字符串创建动态函数 |
| `crypt` | 单向字符串散列 |
| `crypt_checksalt` | Validate a crypt setting string |
| `crypt_gensalt` | Compile a string for use as the salt argument to crypt |
| `crypt_preferred_method` | Get the prefix of the preferred hash method |
| `ctype_alnum` | 检测字母数字式字符 |
| `ctype_alpha` | 检测字母字符 |
| `ctype_cntrl` | 检测控制字符 |
| `ctype_digit` | 检测数字字符 |
| `ctype_graph` | 检测除空格外的任何打印字符 |
| `ctype_lower` | 检测小写字符 |
| `ctype_print` | 检测可打印字符 |
| `ctype_punct` | 检测可打印的字符是不是不包含空白、数字和字母 |
| `ctype_space` | 检测空白字符 |
| `ctype_upper` | 检测大写字符 |
| `ctype_xdigit` | 检测字符是否只包含十六进制字符 |
| `cubrid_affected_rows` | Return the number of rows affected by the last SQL statement |
| `cubrid_bind` | Bind variables to a prepared statement as parameters |
| `cubrid_client_encoding` | Return the current CUBRID connection charset |
| `cubrid_close` | Close CUBRID connection |
| `cubrid_close_prepare` | Close the request handle |
| `cubrid_close_request` | Close the request handle |
| `cubrid_col_get` | Get contents of collection type column using OID |
| `cubrid_col_size` | Get the number of elements in collection type column using OID |
| `cubrid_column_names` | Get the column names in result |
| `cubrid_column_types` | Get column types in result |
| `cubrid_commit` | Commit a transaction |
| `cubrid_connect` | Open a connection to a CUBRID Server |
| `cubrid_connect_with_url` | Establish the environment for connecting to CUBRID server |
| `cubrid_current_oid` | Get OID of the current cursor location |
| `cubrid_data_seek` | Move the internal row pointer of the CUBRID result |
| `cubrid_db_name` | Get db name from results of cubrid_list_dbs |
| `cubrid_disconnect` | Close a database connection |
| `cubrid_drop` | Delete an instance using OID |
| `cubrid_errno` | Return the numerical value of the error message from previous CUBRID operation |
| `cubrid_error` | Get the error message |
| `cubrid_error_code` | Get error code for the most recent function call |
| `cubrid_error_code_facility` | Get the facility code of error |
| `cubrid_error_msg` | Get last error message for the most recent function call |
| `cubrid_execute` | Execute a prepared SQL statement |
| `cubrid_fetch` | Fetch the next row from a result set |
| `cubrid_fetch_array` | Fetch a result row as an associative array, a numeric array, or both |
| `cubrid_fetch_assoc` | Return the associative array that corresponds to the fetched row |
| `cubrid_fetch_field` | Get column information from a result and return as an object |
| `cubrid_fetch_lengths` | Return an array with the lengths of the values of each field from the current row |
| `cubrid_fetch_object` | Fetch the next row and return it as an object |
| `cubrid_fetch_row` | Return a numerical array with the values of the current row |
| `cubrid_field_flags` | Return a string with the flags of the given field offset |
| `cubrid_field_len` | Get the maximum length of the specified field |
| `cubrid_field_name` | Return the name of the specified field index |
| `cubrid_field_seek` | Move the result set cursor to the specified field offset |
| `cubrid_field_table` | Return the name of the table of the specified field |
| `cubrid_field_type` | Return the type of the column corresponding to the given field offset |
| `cubrid_free_result` | Free the memory occupied by the result data |
| `cubrid_get` | Get a column using OID |
| `cubrid_get_autocommit` | Get auto-commit mode of the connection |
| `cubrid_get_charset` | Return the current CUBRID connection charset |
| `cubrid_get_class_name` | Get the class name using OID |
| `cubrid_get_client_info` | Return the client library version |
| `cubrid_get_db_parameter` | Returns the CUBRID database parameters |
| `cubrid_get_query_timeout` | Get the query timeout value of the request |
| `cubrid_get_server_info` | Return the CUBRID server version |
| `cubrid_insert_id` | Return the ID generated for the last updated AUTO_INCREMENT column |
| `cubrid_is_instance` | Check whether the instance pointed by OID exists |
| `cubrid_list_dbs` | Return an array with the list of all existing CUBRID databases |
| `cubrid_load_from_glo` | Read data from a GLO instance and save it in a file |
| `cubrid_lob_close` | Close BLOB/CLOB data |
| `cubrid_lob_export` | Export BLOB/CLOB data to file |
| `cubrid_lob_get` | Get BLOB/CLOB data |
| `cubrid_lob_send` | Read BLOB/CLOB data and send straight to browser |
| `cubrid_lob_size` | Get BLOB/CLOB data size |
| `cubrid_lob2_bind` | Bind a lob object or a string as a lob object to a prepared statement as parameters |
| `cubrid_lob2_close` | Close LOB object |
| `cubrid_lob2_export` | Export the lob object to a file |
| `cubrid_lob2_import` | Import BLOB/CLOB data from a file |
| `cubrid_lob2_new` | Create a lob object |
| `cubrid_lob2_read` | Read from BLOB/CLOB data |
| `cubrid_lob2_seek` | Move the cursor of a lob object |
| `cubrid_lob2_seek64` | Move the cursor of a lob object |
| `cubrid_lob2_size` | Get a lob object's size |
| `cubrid_lob2_size64` | Get a lob object's size |
| `cubrid_lob2_tell` | Tell the cursor position of the LOB object |
| `cubrid_lob2_tell64` | Tell the cursor position of the LOB object |
| `cubrid_lob2_write` | Write to a lob object |
| `cubrid_lock_read` | Set a read lock on the given OID |
| `cubrid_lock_write` | Set a write lock on the given OID |
| `cubrid_move_cursor` | Move the cursor in the result |
| `cubrid_new_glo` | Create a glo instance |
| `cubrid_next_result` | Get result of next query when executing multiple SQL statements |
| `cubrid_num_cols` | Return the number of columns in the result set |
| `cubrid_num_fields` | Return the number of columns in the result set |
| `cubrid_num_rows` | Get the number of rows in the result set |
| `cubrid_pconnect` | Open a persistent connection to a CUBRID server |
| `cubrid_pconnect_with_url` | Open a persistent connection to CUBRID server |
| `cubrid_ping` | Ping a server connection or reconnect if there is no connection |
| `cubrid_prepare` | Prepare a SQL statement for execution |
| `cubrid_put` | Update a column using OID |
| `cubrid_query` | Send a CUBRID query |
| `cubrid_real_escape_string` | Escape special characters in a string for use in an SQL statement |
| `cubrid_result` | Return the value of a specific field in a specific row |
| `cubrid_rollback` | Roll back a transaction |
| `cubrid_save_to_glo` | Save requested file in a GLO instance |
| `cubrid_schema` | Get the requested schema information |
| `cubrid_send_glo` | Read data from glo and send it to std output |
| `cubrid_seq_drop` | Delete an element from sequence type column using OID |
| `cubrid_seq_insert` | Insert an element to a sequence type column using OID |
| `cubrid_seq_put` | Update the element value of sequence type column using OID |
| `cubrid_set_add` | Insert a single element to set type column using OID |
| `cubrid_set_autocommit` | Set autocommit mode of the connection |
| `cubrid_set_db_parameter` | Sets the CUBRID database parameters |
| `cubrid_set_drop` | Delete an element from set type column using OID |
| `cubrid_set_query_timeout` | Set the timeout time of query execution |
| `cubrid_unbuffered_query` | Perform a query without fetching the results into memory |
| `cubrid_version` | Get the CUBRID PHP module's version |
| `curl_close` | 关闭 cURL 会话 |
| `curl_copy_handle` | 复制 cURL 句柄及其所有选项 |
| `curl_errno` | 返回最后一次的错误代码 |
| `curl_error` | 返回当前会话最后一次错误的字符串 |
| `curl_escape` | 使用 URL 编码指定字符串 |
| `curl_exec` | 执行 cURL 会话 |
| `curl_getinfo` | 获取 cURL 连接资源句柄的信息 |
| `curl_init` | 初始化 cURL 会话 |
| `curl_multi_add_handle` | 添加普通 cURL 句柄到 cURL 多句柄 |
| `curl_multi_close` | 从多句柄中移除所有 cURL 句柄 |
| `curl_multi_errno` | 返回上一次 curl 批处理的错误码 |
| `curl_multi_exec` | 运行当前 cURL 句柄的子连接 |
| `curl_multi_getcontent` | 如果设置了 CURLOPT_RETURNTRANSFER，则返回 cURL 句柄的内容 |
| `curl_multi_info_read` | 获取当前传输的有关信息 |
| `curl_multi_init` | 返回新 cURL 批处理句柄 |
| `curl_multi_remove_handle` | 从一组 cURL 句柄中移除一个句柄 |
| `curl_multi_select` | 等待，直到任何 cURL 多句柄连接可以进行读取或写入 |
| `curl_multi_setopt` | 设置 cURL 并行选项 |
| `curl_multi_strerror` | 返回字符串描述的错误代码 |
| `curl_pause` | 暂停和取消暂停连接 |
| `curl_reset` | 重置一个 libcurl 会话句柄的所有的选项 |
| `curl_setopt` | 设置 cURL 传输选项 |
| `curl_setopt_array` | 为 cURL 传输会话批量设置选项 |
| `curl_share_close` | 关闭 cURL 共享句柄 |
| `curl_share_errno` | 返回共享 curl 句柄的最后一次错误编号 |
| `curl_share_init` | 初始化 cURL 共享句柄 |
| `curl_share_init_persistent` | 初始化 持久 cURL 共享句柄 |
| `curl_share_setopt` | 为 cURL 共享句柄设置选项 |
| `curl_share_strerror` | 返回错误编号对应的错误消息 |
| `curl_strerror` | 返回错误代码的字符串描述 |
| `curl_unescape` | 解码指定 URL 编码的字符串 |
| `curl_upkeep` | Performs any connection upkeep checks |
| `curl_version` | 获取 cURL 版本信息 |
| `current` | 返回数组中的当前值 |

### D

| 函数名 | 功能描述 |
|--------|----------|
| `date` | 格式化 Unix 时间戳 |
| `date_add` | 别名 DateTime::add |
| `date_create` | create a new DateTime object |
| `date_create_from_format` | 别名 DateTime::createFromFormat |
| `date_create_immutable` | create a new DateTimeImmutable object |
| `date_create_immutable_from_format` | 别名 DateTimeImmutable::createFromFormat |
| `date_date_set` | 别名 DateTime::setDate |
| `date_default_timezone_get` | 取得脚本中所有日期/时间函数所使用的默认时区 |
| `date_default_timezone_set` | 设置脚本中所有日期/时间函数使用的默认时区 |
| `date_diff` | 别名 DateTime::diff |
| `date_format` | 别名 DateTime::format |
| `date_get_last_errors` | 别名 DateTimeImmutable::getLastErrors |
| `date_interval_create_from_date_string` | 别名 DateInterval::createFromDateString |
| `date_interval_format` | 别名 DateInterval::format |
| `date_isodate_set` | 别名 DateTime::setISODate |
| `date_modify` | 别名 DateTime::modify |
| `date_offset_get` | 别名 DateTime::getOffset |
| `date_parse` | 返回指定日期/时间的详细信息的关联数组 |
| `date_parse_from_format` | Get info about given date formatted according to the specified format |
| `date_sub` | 别名 DateTime::sub |
| `date_sun_info` | Returns an array with information about sunset/sunrise and twilight begin/end |
| `date_sunrise` | 返回给定的日期与地点的日出时间 |
| `date_sunset` | 返回给定的日期与地点的日落时间 |
| `date_time_set` | 别名 DateTime::setTime |
| `date_timestamp_get` | 别名 DateTime::getTimestamp |
| `date_timestamp_set` | 别名 DateTime::setTimestamp |
| `date_timezone_get` | 别名 DateTime::getTimezone |
| `date_timezone_set` | 别名 DateTime::setTimezone |
| `db2_autocommit` | Returns or sets the AUTOCOMMIT state for a database connection |
| `db2_bind_param` | Binds a PHP variable to an SQL statement parameter |
| `db2_client_info` | Returns an object with properties that describe the DB2 database client |
| `db2_close` | Closes a database connection |
| `db2_column_privileges` | Returns a result set listing the columns and associated privileges for a table |
| `db2_columns` | Returns a result set listing the columns and associated metadata for a table |
| `db2_commit` | Commits a transaction |
| `db2_conn_error` | Returns a string containing the SQLSTATE returned by the last connection attempt |
| `db2_conn_errormsg` | Returns the last connection error message and SQLCODE value |
| `db2_connect` | Returns a connection to a database |
| `db2_cursor_type` | Returns the cursor type used by a statement resource |
| `db2_escape_string` | Used to escape certain characters |
| `db2_exec` | Executes an SQL statement directly |
| `db2_execute` | Executes a prepared SQL statement |
| `db2_fetch_array` | Returns an array, indexed by column position, representing a row in a result set |
| `db2_fetch_assoc` | Returns an array, indexed by column name, representing a row in a result set |
| `db2_fetch_both` | Returns an array, indexed by both column name and position, representing a row in a result set |
| `db2_fetch_object` | Returns an object with properties representing columns in the fetched row |
| `db2_fetch_row` | Sets the result set pointer to the next row or requested row |
| `db2_field_display_size` | Returns the maximum number of bytes required to display a column |
| `db2_field_name` | Returns the name of the column in the result set |
| `db2_field_num` | Returns the position of the named column in a result set |
| `db2_field_precision` | Returns the precision of the indicated column in a result set |
| `db2_field_scale` | Returns the scale of the indicated column in a result set |
| `db2_field_type` | Returns the data type of the indicated column in a result set |
| `db2_field_width` | Returns the width of the current value of the indicated column in a result set |
| `db2_foreign_keys` | Returns a result set listing the foreign keys for a table |
| `db2_free_result` | Frees resources associated with a result set |
| `db2_free_stmt` | Frees resources associated with the indicated statement resource |
| `db2_get_option` | Retrieves an option value for a statement resource or a connection resource |
| `db2_lob_read` | Gets a user defined size of LOB files with each invocation |
| `db2_next_result` | Requests the next result set from a stored procedure |
| `db2_num_fields` | Returns the number of fields contained in a result set |
| `db2_num_rows` | Returns the number of rows affected by an SQL statement |
| `db2_pclose` | Closes a persistent database connection |
| `db2_pconnect` | Returns a persistent connection to a database |
| `db2_prepare` | Prepares an SQL statement to be executed |
| `db2_primary_keys` | Returns a result set listing primary keys for a table |
| `db2_procedure_columns` | Returns a result set listing stored procedure parameters |
| `db2_procedures` | Returns a result set listing the stored procedures registered in a database |
| `db2_result` | Returns a single column from a row in the result set |
| `db2_rollback` | Rolls back a transaction |
| `db2_server_info` | Returns an object with properties that describe the DB2 database server |
| `db2_set_option` | Set options for connection or statement resources |
| `db2_special_columns` | Returns a result set listing the unique row identifier columns for a table |
| `db2_statistics` | Returns a result set listing the index and statistics for a table |
| `db2_stmt_error` | Returns a string containing the SQLSTATE returned by an SQL statement |
| `db2_stmt_errormsg` | Returns a string containing the last SQL statement error message |
| `db2_table_privileges` | Returns a result set listing the tables and associated privileges in a database |
| `db2_tables` | Returns a result set listing the tables and associated metadata in a database |
| `dba_close` | 关闭 DBA 数据库 |
| `dba_delete` | 删除由键指定的 DBA 条目 |
| `dba_exists` | 检查键是否存在 |
| `dba_fetch` | 获取由键指定的数据 |
| `dba_firstkey` | 获取第一个键 |
| `dba_handlers` | 列出所有可用的处理器 |
| `dba_insert` | 插入条目 |
| `dba_key_split` | 将键的字符串表示分割为数组表示 |
| `dba_list` | 列出所有打开的数据库文件 |
| `dba_nextkey` | 获取下一个键 |
| `dba_open` | 打开数据库 |
| `dba_optimize` | 优化数据库 |
| `dba_popen` | 打开数据库持久化 |
| `dba_replace` | 替换或插入条目 |
| `dba_sync` | 同步数据库 |
| `dbase_add_record` | Adds a record to a database |
| `dbase_close` | Closes a database |
| `dbase_create` | Creates a database |
| `dbase_delete_record` | Deletes a record from a database |
| `dbase_get_header_info` | Gets the header info of a database |
| `dbase_get_record` | Gets a record from a database as an indexed array |
| `dbase_get_record_with_names` | Gets a record from a database as an associative array |
| `dbase_numfields` | Gets the number of fields of a database |
| `dbase_numrecords` | Gets the number of records in a database |
| `dbase_open` | Opens a database |
| `dbase_pack` | Packs a database |
| `dbase_replace_record` | Replaces a record in a database |
| `dcgettext` | Overrides the domain for a single lookup |
| `dcngettext` | Plural version of dcgettext |
| `debug_backtrace` | 产生一条回溯跟踪(backtrace) |
| `debug_print_backtrace` | 打印一条回溯。 |
| `debug_zval_dump` | 将表示内部 zval 结构的字符串转储到输出 |
| `decbin` | 十进制转换为二进制 |
| `dechex` | 十进制转换为十六进制 |
| `decoct` | 十进制转换为八进制 |
| `define` | 定义一个常量 |
| `defined` | 检查给定名称的常量是否存在 |
| `deflate_add` | Incrementally deflate data |
| `deflate_init` | Initialize an incremental deflate context |
| `deg2rad` | 将角度转换为弧度 |
| `delete` | 参见 unlink 或 unset |
| `dgettext` | Override the current domain |
| `die` | 别名 exit |
| `dio_close` | 通过 fd 关闭文件描述符 |
| `dio_fcntl` | 在 fd 上执行 C 标准库的 fcntl |
| `dio_open` | 在 C 库输入/输出流函数允许的更低级别打开（必要时创建）文件 |
| `dio_read` | 从文件描述符读取字节 |
| `dio_seek` | 在 fd 指定 pos 位置 |
| `dio_stat` | 获取有关文件描述符 fd 的统计信息 |
| `dio_tcsetattr` | 设置串行端口的终端属性和波特率 |
| `dio_truncate` | 截断文件描述符 fd 为 offset 字节 |
| `dio_write` | 截取可选长度的数据写入文件 |
| `dir` | 返回一个 Directory 类实例 |
| `dirname` | 返回路径中的目录部分 |
| `disk_free_space` | 返回目录中的可用空间 |
| `disk_total_space` | 返回一个目录的磁盘总大小 |
| `diskfreespace` | disk_free_space 的别名 |
| `dl` | 运行时载入一个 PHP 扩展 |
| `dngettext` | Plural version of dgettext |
| `dns_check_record` | 别名 checkdnsrr |
| `dns_get_mx` | 别名 getmxrr |
| `dns_get_record` | 获取指定主机名的 DNS 纪录 |
| `doubleval` | 别名 floatval |

### E

| 函数名 | 功能描述 |
|--------|----------|
| `each` | 返回数组中当前的键／值对并将数组指针向前移动一步 |
| `easter_date` | 得到指定年份的复活节当地午夜时的 Unix 时间戳 |
| `easter_days` | 得到指定年份的 3 月 21 日到复活节之间的天数 |
| `echo` | 输出一个或多个字符串 |
| `eio_cancel` | Cancels a request |
| `eio_chmod` | Change file/directory permissions |
| `eio_chown` | Change file/directory permissions |
| `eio_close` | Close file |
| `eio_custom` | Execute custom request like any other eio_* call |
| `eio_dup2` | Duplicate a file descriptor |
| `eio_event_loop` | Polls libeio until all requests proceeded |
| `eio_fchmod` | Change file permissions |
| `eio_fchown` | Change file ownership |
| `eio_fdatasync` | Synchronize a file's in-core state with storage device |
| `eio_fstat` | Get file status |
| `eio_fstatvfs` | Get file system statistics |
| `eio_fsync` | Synchronize a file's in-core state with storage device |
| `eio_ftruncate` | Truncate a file |
| `eio_futime` | Change file last access and modification times |
| `eio_get_event_stream` | Get stream representing a variable used in internal communications with libeio |
| `eio_get_last_error` | Returns string describing the last error associated with a request resource |
| `eio_grp` | Creates a request group |
| `eio_grp_add` | Adds a request to the request group |
| `eio_grp_cancel` | Cancels a request group |
| `eio_grp_limit` | Set group limit |
| `eio_init` | (Re-)initialize Eio |
| `eio_link` | Create a hardlink for file |
| `eio_lstat` | Get file status |
| `eio_mkdir` | Create directory |
| `eio_mknod` | Create a special or ordinary file |
| `eio_nop` | Does nothing, except go through the whole request cycle |
| `eio_npending` | Returns number of finished, but unhandled requests |
| `eio_nready` | Returns number of not-yet handled requests |
| `eio_nreqs` | Returns number of requests to be processed |
| `eio_nthreads` | Returns number of threads currently in use |
| `eio_open` | Opens a file |
| `eio_poll` | Can be to be called whenever there are pending requests that need finishing |
| `eio_read` | Read from a file descriptor at given offset |
| `eio_readahead` | Perform file readahead into page cache |
| `eio_readdir` | Reads through a whole directory |
| `eio_readlink` | Read value of a symbolic link |
| `eio_realpath` | Get the canonicalized absolute pathname |
| `eio_rename` | Change the name or location of a file |
| `eio_rmdir` | Remove a directory |
| `eio_seek` | Seek to a position |
| `eio_sendfile` | Transfer data between file descriptors |
| `eio_set_max_idle` | Set maximum number of idle threads |
| `eio_set_max_parallel` | Set maximum parallel threads |
| `eio_set_max_poll_reqs` | Set maximum number of requests processed in a poll |
| `eio_set_max_poll_time` | Set maximum poll time |
| `eio_set_min_parallel` | Set minimum parallel thread number |
| `eio_stat` | Get file status |
| `eio_statvfs` | Get file system statistics |
| `eio_symlink` | Create a symbolic link |
| `eio_sync` | Commit buffer cache to disk |
| `eio_sync_file_range` | Sync a file segment with disk |
| `eio_syncfs` | Calls Linux' syncfs syscall, if available |
| `eio_truncate` | Truncate a file |
| `eio_unlink` | Delete a name and possibly the file it refers to |
| `eio_utime` | Change file last access and modification times |
| `eio_write` | Write to file |
| `empty` | 检查变量是否为空 |
| `enchant_broker_describe` | Enumerates the Enchant providers |
| `enchant_broker_dict_exists` | Whether a dictionary exists or not. Using non-empty tag |
| `enchant_broker_free` | Free the broker resource and its dictionaries |
| `enchant_broker_free_dict` | Free a dictionary resource |
| `enchant_broker_get_dict_path` | Get the directory path for a given backend |
| `enchant_broker_get_error` | Returns the last error of the broker |
| `enchant_broker_init` | Create a new broker object capable of requesting |
| `enchant_broker_list_dicts` | Returns a list of available dictionaries |
| `enchant_broker_request_dict` | Create a new dictionary using a tag |
| `enchant_broker_request_pwl_dict` | Creates a dictionary using a PWL file |
| `enchant_broker_set_dict_path` | Set the directory path for a given backend |
| `enchant_broker_set_ordering` | Declares a preference of dictionaries to use for the language |
| `enchant_dict_add` | Add a word to personal word list |
| `enchant_dict_add_to_personal` | 别名 enchant_dict_add |
| `enchant_dict_add_to_session` | Add 'word' to this spell-checking session |
| `enchant_dict_check` | Check whether a word is correctly spelled or not |
| `enchant_dict_describe` | Describes an individual dictionary |
| `enchant_dict_get_error` | Returns the last error of the current spelling-session |
| `enchant_dict_is_added` | Whether or not 'word' exists in this spelling-session |
| `enchant_dict_is_in_session` | 别名 enchant_dict_is_added |
| `enchant_dict_quick_check` | Check the word is correctly spelled and provide suggestions |
| `enchant_dict_store_replacement` | Add a correction for a word |
| `enchant_dict_suggest` | Will return a list of values if any of those pre-conditions are not met |
| `end` | 将数组的内部指针指向最后一个单元 |
| `enum_exists` | 检测是否定义对应的枚举 |
| `error_clear_last` | 清除最近一次错误 |
| `error_get_last` | 获取最后发生的错误 |
| `error_log` | 发送错误信息到某个地方 |
| `error_reporting` | 设置应该报告何种 PHP 错误 |
| `escapeshellarg` | 把字符串转义为可以在 shell 命令里使用的参数 |
| `escapeshellcmd` | shell 元字符转义 |
| `eval` | 把字符串作为PHP代码执行 |
| `exec` | 执行一个外部程序 |
| `exif_imagetype` | 判断一个图像的类型 |
| `exif_read_data` | 从一个图片文件中读取 EXIF 头信息 |
| `exif_tagname` | 获取指定索引的头名称 |
| `exif_thumbnail` | 检索图像的嵌入式缩略图 |
| `exit` | 使用状态 code 或消息终止当前脚本 |
| `exp` | 计算 e 的指数 |
| `explode` | 使用一个字符串分割另一个字符串 |
| `expm1` | 返回 exp($num) - 1，甚至当 number 的值接近零也能计算出准确结果 |
| `expression` | Bind prepared statement variables as parameters |
| `extension_loaded` | 检查一个扩展是否已经加载 |
| `extract` | 从数组中将变量导入到当前的符号表 |
| `ezmlm_hash` | 计算 EZMLM 所需的散列值 |

### F

| 函数名 | 功能描述 |
|--------|----------|
| `fann_cascadetrain_on_data` | 在整个数据集上训练，使用一段时间的 Cascade2 训练算法。 |
| `fann_cascadetrain_on_file` | 读取文件并在整个数据集上训练，使用 Cascade2 训练算法训练一段时间 |
| `fann_clear_scaling_params` | 清除缩放参数 |
| `fann_copy` | 创建一个 fann 结构体的副本。 |
| `fann_create_from_file` | 从配置文件中构建一个反向传播神经网络。 |
| `fann_create_shortcut` | 创建一个含快捷连接而非全连接的标准反向传播神经网络。 |
| `fann_create_shortcut_array` | 创建一个含快捷连接而非全连接的标准反向传播神经网络。 |
| `fann_create_sparse` | 创建一个标准的反向传播神经网络，该网络不是全连接。 |
| `fann_create_sparse_array` | 创建一个标准的反向传播神经网络，该网络使用一个表示每层大小的数组来构造，但是并不是全连接的。 |
| `fann_create_standard` | 创建标准的全连接反向传播神经网络。 |
| `fann_create_standard_array` | 创建一个全连接的反向传播神经网络，该网络使用一个表示每层大小的数组来构造。 |
| `fann_create_train` | 创建一个空的训练数据结构。 |
| `fann_create_train_from_callback` | 从用户提供的函数创建训练数据结构。 |
| `fann_descale_input` | 在获取基于先前计算的参数之后，在输入向量中缩小数据 |
| `fann_descale_output` | 在获取基于先前计算的参数之后，在输出向量中缩小数据 |
| `fann_descale_train` | 基于先前计算的参数来缩小输入和输出数据 |
| `fann_destroy` | 销毁整个网络并且适当地释放所有的关联内存。 |
| `fann_destroy_train` | 销毁训练数据。 |
| `fann_duplicate_train_data` | 返回 fann 训练数据精确的副本。 |
| `fann_get_activation_function` | 返回激励函数 |
| `fann_get_activation_steepness` | 为提供的神经和网络层数返回激活陡度 |
| `fann_get_bias_array` | 获取网络中每一层的偏差数 |
| `fann_get_bit_fail` | 失败位的数量 |
| `fann_get_bit_fail_limit` | 返回训练期间使用的误差限制 |
| `fann_get_cascade_activation_functions` | 返回级联激活函数 |
| `fann_get_cascade_activation_functions_count` | 返回级联激活函数的数量 |
| `fann_get_cascade_activation_steepnesses` | 返回级联激活陡度 |
| `fann_get_cascade_activation_steepnesses_count` | 激活陡度的数量 |
| `fann_get_cascade_candidate_change_fraction` | 返回级联候选变化分数 |
| `fann_get_cascade_candidate_limit` | 返回候选限度 |
| `fann_get_cascade_candidate_stagnation_epochs` | 返回层叠候选停滞周期的数量 |
| `fann_get_cascade_max_cand_epochs` | 返回候选周期的最大值 |
| `fann_get_cascade_max_out_epochs` | 返回输出周期的最大值 |
| `fann_get_cascade_min_cand_epochs` | 返回最小的候选周期 |
| `fann_get_cascade_min_out_epochs` | 返回最小输出周期 |
| `fann_get_cascade_num_candidate_groups` | 返回候选组的数量 |
| `fann_get_cascade_num_candidates` | 返回训练期间使用的候选数量 |
| `fann_get_cascade_output_change_fraction` | 返回级联输出变化分数 |
| `fann_get_cascade_output_stagnation_epochs` | 返回级联输出停滞周期的数量 |
| `fann_get_cascade_weight_multiplier` | 返回权重因子 |
| `fann_get_connection_array` | 获取网络中的连接。 |
| `fann_get_connection_rate` | 获取当网络创建时连接的使用率。 |
| `fann_get_errno` | 返回最后一个错误数字。 |
| `fann_get_errstr` | 返回最后的错误字符串。 |
| `fann_get_layer_array` | 获取网络中每层的神经元数量。 |
| `fann_get_learning_momentum` | 返回学习动量 |
| `fann_get_learning_rate` | 返回学习速率 |
| `fann_get_MSE` | 从网络中读取均方误差。 |
| `fann_get_network_type` | 获取所创建的神经网络类型。 |
| `fann_get_num_input` | 获取输入神经元的数量。 |
| `fann_get_num_layers` | 获取神经网络的层数。 |
| `fann_get_num_output` | 获取输出神经元的数量。 |
| `fann_get_quickprop_decay` | 返回衰退值，用于在 quickprop 训练迭代时衰减权重 |
| `fann_get_quickprop_mu` | 返回放大系数 |
| `fann_get_rprop_decrease_factor` | 返回 RPROP 训练期间的衰减系数 |
| `fann_get_rprop_delta_max` | 返回最大步长 |
| `fann_get_rprop_delta_min` | 返回最小步长 |
| `fann_get_rprop_delta_zero` | 返回初始步长 |
| `fann_get_rprop_increase_factor` | 返回 RPROP 训练的递增系数 |
| `fann_get_sarprop_step_error_shift` | 返回 sarprop 步值的误差偏移 |
| `fann_get_sarprop_step_error_threshold_factor` | 返回 sarprop 算法步值的误差阈值系数 |
| `fann_get_sarprop_temperature` | 返回 sarprop 算法温度 |
| `fann_get_sarprop_weight_decay_shift` | 返回 sarprop 算法权重衰减变化值 |
| `fann_get_total_connections` | 获取整个网络中所有的连接数。 |
| `fann_get_total_neurons` | 获取整个网络中神经元的数量。 |
| `fann_get_train_error_function` | 返回训练中使用的错误函数。 |
| `fann_get_train_stop_function` | 返回训练中使用的停止函数。 |
| `fann_get_training_algorithm` | 返回训练算法。 |
| `fann_init_weights` | 使用 Widrow 和 Nguyen 算法初始化权重。 |
| `fann_length_train_data` | 返回训练数据中训练模式的数量。 |
| `fann_merge_train_data` | 合并训练数据。 |
| `fann_num_input_train_data` | 返回训练数据中每个训练模式输入的数量。 |
| `fann_num_output_train_data` | 返回训练数据中每个训练模式输出的数量。 |
| `fann_print_error` | 打印错误字符串 |
| `fann_randomize_weights` | 给每个连接赋一个介于 min_weight 和 max_weight 之间的随机权重。 |
| `fann_read_train_from_file` | 读取存储训练数据的文件。 |
| `fann_reset_errno` | 重置最后的错误代码。 |
| `fann_reset_errstr` | 重置最后的错误字符串。 |
| `fann_reset_MSE` | 重置网络中的均方误差。 |
| `fann_run` | 将通过神经网络运行输入。 |
| `fann_save` | 将整个网络保存至配置文件。 |
| `fann_save_train` | 将训练结构体保存至文件。 |
| `fann_scale_input` | 在以前计算参数的基础上，在训练之前放大输入向量中的数据 |
| `fann_scale_input_train_data` | 在训练数据中缩放输入至指定范围 |
| `fann_scale_output` | 在以前计算参数的基础上，在训练之前放大输出向量中的数据 |
| `fann_scale_output_train_data` | 在训练数据中缩放输出至指定范围 |
| `fann_scale_train` | 在以前计算参数的基础上，缩放输入和输出数据 |
| `fann_scale_train_data` | 在训练数据中缩放输入和输出到指定的范围 |
| `fann_set_activation_function` | 为已应用的神经元和层设置激活函数 |
| `fann_set_activation_function_hidden` | 为所有隐藏层设置激活函数 |
| `fann_set_activation_function_layer` | 为已应用的层中所有的神经元设置激活函数 |
| `fann_set_activation_function_output` | 为输出层设置激活函数 |
| `fann_set_activation_steepness` | 为提供的神经元和层设置激活陡度 |
| `fann_set_activation_steepness_hidden` | 为所有隐藏层中所有的神经元设置激活函数陡度 |
| `fann_set_activation_steepness_layer` | 为提供的层中所有的神经元设置激活陡度 |
| `fann_set_activation_steepness_output` | 在输出层中设置激活陡度 |
| `fann_set_bit_fail_limit` | 设置训练期间使用的误差 |
| `fann_set_callback` | 设置训练期间使用的回调函数。 |
| `fann_set_cascade_activation_functions` | 设置级联候选激活函数的数组 |
| `fann_set_cascade_activation_steepnesses` | 设置级联候选激活陡度的数组。 |
| `fann_set_cascade_candidate_change_fraction` | 设置级联候选更改分数 |
| `fann_set_cascade_candidate_limit` | 设置候选限度 |
| `fann_set_cascade_candidate_stagnation_epochs` | 设置级联候选停止周期数 |
| `fann_set_cascade_max_cand_epochs` | 设置最大候选周期数 |
| `fann_set_cascade_max_out_epochs` | 设置最大输出周期 |
| `fann_set_cascade_min_cand_epochs` | 设置最小候选周期 |
| `fann_set_cascade_min_out_epochs` | 设置最小输出周期 |
| `fann_set_cascade_num_candidate_groups` | 设置候选组数量 |
| `fann_set_cascade_output_change_fraction` | 设置级联输出改变分数 |
| `fann_set_cascade_output_stagnation_epochs` | 设置级联输出停滞周期的值 |
| `fann_set_cascade_weight_multiplier` | 设置权重因子 |
| `fann_set_error_log` | 设置错误记录保存的位置。 |
| `fann_set_input_scaling_params` | 根据训练数据计算将来使用的输入比例参数 |
| `fann_set_learning_momentum` | 设置学习动量。 |
| `fann_set_learning_rate` | 设置学习速率。 |
| `fann_set_output_scaling_params` | 根据训练数据计算将来使用的输出缩放参数 |
| `fann_set_quickprop_decay` | 设置quickprop算法衰减因子 |
| `fann_set_quickprop_mu` | 设置 quickprop 算法放大因子 |
| `fann_set_rprop_decrease_factor` | 使用 RPROP 算法训练时，设置下降因子 |
| `fann_set_rprop_delta_max` | 设置最大步长 |
| `fann_set_rprop_delta_min` | 设置最小步长 |
| `fann_set_rprop_delta_zero` | 设置初始步长 |
| `fann_set_rprop_increase_factor` | 使用 RPROP 算法训练时，设置增长因子 |
| `fann_set_sarprop_step_error_shift` | 设置 sarprop 算法的步误差偏移量 |
| `fann_set_sarprop_step_error_threshold_factor` | 设置 sarprop 算法的步误差阈值因子 |
| `fann_set_sarprop_temperature` | 设置 sarprop 算法的温度 |
| `fann_set_sarprop_weight_decay_shift` | 设置 sarprop 算法的权重衰减偏移值 |
| `fann_set_scaling_params` | 根据训练数据计算输入和输出缩放参数以供将来使用 |
| `fann_set_train_error_function` | 设置训练期间使用的错误函数。 |
| `fann_set_train_stop_function` | 设置训练期间使用的停止函数。 |
| `fann_set_training_algorithm` | 设置训练算法。 |
| `fann_set_weight` | 在网络中设置一个连接。 |
| `fann_set_weight_array` | 在网络中设置一个连接。 |
| `fann_shuffle_train_data` | 打算训练数据，使顺序随机。 |
| `fann_subset_train_data` | 返回一个训练数据子集的副本。 |
| `fann_test` | 使用一组输入和一组期望的输出来测试。 |
| `fann_test_data` | 使用训练数据来测试并且计算出 MSE |
| `fann_train` | 使用一个输入集和一个期望的输出集来迭代训练一次。 |
| `fann_train_epoch` | 使用一组训练数据训练一个周期。 |
| `fann_train_on_data` | 在整个数据集上训练一段时间。 |
| `fann_train_on_file` | 在从某个文件读取的整个数据集上训练一段时间。 |
| `fastcgi_finish_request` | 冲刷(flush)所有响应的数据给客户端 |
| `fbird_add_user` | 别名 ibase_add_user |
| `fbird_affected_rows` | 别名 ibase_affected_rows |
| `fbird_backup` | 别名 ibase_backup |
| `fbird_blob_add` | 别名 ibase_blob_add |
| `fbird_blob_cancel` | Cancel creating blob |
| `fbird_blob_close` | 别名 ibase_blob_close |
| `fbird_blob_create` | 别名 ibase_blob_create |
| `fbird_blob_echo` | 别名 ibase_blob_echo |
| `fbird_blob_get` | 别名 ibase_blob_get |
| `fbird_blob_import` | 别名 ibase_blob_import |
| `fbird_blob_info` | 别名 ibase_blob_info |
| `fbird_blob_open` | 别名 ibase_blob_open |
| `fbird_close` | 别名 ibase_close |
| `fbird_commit` | 别名 ibase_commit |
| `fbird_commit_ret` | 别名 ibase_commit_ret |
| `fbird_connect` | 别名 ibase_connect |
| `fbird_db_info` | 别名 ibase_db_info |
| `fbird_delete_user` | 别名 ibase_delete_user |
| `fbird_drop_db` | 别名 ibase_drop_db |
| `fbird_errcode` | 别名 ibase_errcode |
| `fbird_errmsg` | 别名 ibase_errmsg |
| `fbird_execute` | 别名 ibase_execute |
| `fbird_fetch_assoc` | 别名 ibase_fetch_assoc |
| `fbird_fetch_object` | 别名 ibase_fetch_object |
| `fbird_fetch_row` | 别名 ibase_fetch_row |
| `fbird_field_info` | 别名 ibase_field_info |
| `fbird_free_event_handler` | 别名 ibase_free_event_handler |
| `fbird_free_query` | 别名 ibase_free_query |
| `fbird_free_result` | 别名 ibase_free_result |
| `fbird_gen_id` | 别名 ibase_gen_id |
| `fbird_maintain_db` | 别名 ibase_maintain_db |
| `fbird_modify_user` | 别名 ibase_modify_user |
| `fbird_name_result` | 别名 ibase_name_result |
| `fbird_num_fields` | 别名 ibase_num_fields |
| `fbird_num_params` | 别名 ibase_num_params |
| `fbird_param_info` | 别名 ibase_param_info |
| `fbird_pconnect` | 别名 ibase_pconnect |
| `fbird_prepare` | 别名 ibase_prepare |
| `fbird_query` | 别名 ibase_query |
| `fbird_restore` | 别名 ibase_restore |
| `fbird_rollback` | 别名 ibase_rollback |
| `fbird_rollback_ret` | 别名 ibase_rollback_ret |
| `fbird_server_info` | 别名 ibase_server_info |
| `fbird_service_attach` | 别名 ibase_service_attach |
| `fbird_service_detach` | 别名 ibase_service_detach |
| `fbird_set_event_handler` | 别名 ibase_set_event_handler |
| `fbird_trans` | 别名 ibase_trans |
| `fbird_wait_event` | 别名 ibase_wait_event |
| `fclose` | 关闭一个已打开的文件指针 |
| `fdatasync` | 同步文件数据（不包含元数据） |
| `fdf_add_doc_javascript` | Adds javascript code to the FDF document |
| `fdf_add_template` | Adds a template into the FDF document |
| `fdf_close` | Close an FDF document |
| `fdf_create` | Create a new FDF document |
| `fdf_enum_values` | Call a user defined function for each document value |
| `fdf_errno` | Return error code for last fdf operation |
| `fdf_error` | Return error description for FDF error code |
| `fdf_get_ap` | Get the appearance of a field |
| `fdf_get_attachment` | Extracts uploaded file embedded in the FDF |
| `fdf_get_encoding` | Get the value of the /Encoding key |
| `fdf_get_file` | Get the value of the /F key |
| `fdf_get_flags` | Gets the flags of a field |
| `fdf_get_opt` | Gets a value from the opt array of a field |
| `fdf_get_status` | Get the value of the /STATUS key |
| `fdf_get_value` | Get the value of a field |
| `fdf_get_version` | Gets version number for FDF API or file |
| `fdf_header` | Sets FDF-specific output headers |
| `fdf_next_field_name` | Get the next field name |
| `fdf_open` | Open a FDF document |
| `fdf_open_string` | Read a FDF document from a string |
| `fdf_remove_item` | Sets target frame for form |
| `fdf_save` | Save a FDF document |
| `fdf_save_string` | Returns the FDF document as a string |
| `fdf_set_ap` | Set the appearance of a field |
| `fdf_set_encoding` | Sets FDF character encoding |
| `fdf_set_file` | Set PDF document to display FDF data in |
| `fdf_set_flags` | Sets a flag of a field |
| `fdf_set_javascript_action` | Sets an javascript action of a field |
| `fdf_set_on_import_javascript` | Adds javascript code to be executed when Acrobat opens the FDF |
| `fdf_set_opt` | Sets an option of a field |
| `fdf_set_status` | Set the value of the /STATUS key |
| `fdf_set_submit_form_action` | Sets a submit form action of a field |
| `fdf_set_target_frame` | Set target frame for form display |
| `fdf_set_value` | Set the value of a field |
| `fdf_set_version` | Sets version number for a FDF file |
| `fdiv` | Divides two numbers, according to IEEE 754 |
| `feof` | 测试文件指针是否到了文件结束的位置 |
| `fflush` | 将缓冲内容输出到文件 |
| `fgetc` | 从文件指针中读取字符 |
| `fgetcsv` | 从文件指针中读入一行并解析 CSV 字段 |
| `fgets` | 从文件指针中读取一行 |
| `fgetss` | 从文件指针中读取一行并过滤掉 HTML 标记 |
| `file` | 把整个文件读入一个数组中 |
| `file_exists` | 检查文件或目录是否存在 |
| `file_get_contents` | 将整个文件读入一个字符串 |
| `file_put_contents` | 将数据写入文件 |
| `fileatime` | 取得文件的上次访问时间 |
| `filectime` | 取得文件的 inode 修改时间 |
| `filegroup` | 取得文件的组 |
| `fileinode` | 取得文件的 inode |
| `filemtime` | 取得文件修改时间 |
| `fileowner` | 取得文件的所有者 |
| `fileperms` | 获取文件权限 |
| `filesize` | 取得文件大小 |
| `filetype` | 取得文件类型 |
| `filter_has_var` | 检测是否存在指定类型的变量 |
| `filter_id` | 返回与某个特定名称的过滤器相关联的id |
| `filter_input` | 通过名称获取特定的外部变量，并且可以通过过滤器处理它 |
| `filter_input_array` | 获取一系列外部变量，并且可以通过过滤器处理它们 |
| `filter_list` | 返回所支持的过滤器列表 |
| `filter_var` | 使用特定的过滤器过滤一个变量 |
| `filter_var_array` | 获取多个变量并且过滤它们 |
| `finfo_buffer` | 返回一个字符串缓冲区的信息 |
| `finfo_close` | 关闭 finfo 实例 |
| `finfo_file` | 返回一个文件的信息 |
| `finfo_open` | 创建新 finfo 实例 |
| `finfo_set_flags` | 设置 libmagic 配置选项 |
| `floatval` | 获取变量的浮点值 |
| `flock` | 可移植的协同文件锁定 |
| `floor` | 舍去法取整 |
| `flush` | 冲刷系统输出缓冲区 |
| `fmod` | 返回除法的浮点数余数 |
| `fnmatch` | 用模式匹配文件名 |
| `fopen` | 打开文件或者 URL |
| `forward_static_call` | 调用静态方法 |
| `forward_static_call_array` | 调用静态方法且参数作为数组传递 |
| `fpassthru` | 输出文件指针处的所有剩余数据 |
| `fpm_get_status` | 返回当前 FPM 池状态 |
| `fpow` | Raise one number to the power of another, according to IEEE 754 |
| `fprintf` | 将格式化后的字符串写入到流 |
| `fputcsv` | 将行格式化为 CSV 并写入文件指针 |
| `fputs` | fwrite 的别名 |
| `fread` | 读取文件（可安全用于二进制文件） |
| `frenchtojd` | 从法国共和历日期转换为儒略日数 |
| `fscanf` | 从文件中格式化输入 |
| `fseek` | 在文件指针中定位 |
| `fsockopen` | 打开 Internet 或者 Unix 套接字连接 |
| `fstat` | 通过已打开的文件指针取得文件信息 |
| `fsync` | 同步文件变更（包括元数据） |
| `ftell` | 返回文件指针读/写的位置 |
| `ftok` | Convert a pathname and a project identifier to a System V IPC key |
| `ftp_alloc` | 为要上传的文件分配空间 |
| `ftp_append` | 将文件内容追加到 FTP 服务器上的指定文件 |
| `ftp_cdup` | 切换到当前目录的父目录 |
| `ftp_chdir` | 在 FTP 服务器上改变当前目录 |
| `ftp_chmod` | 设置 FTP 服务器上的文件权限 |
| `ftp_close` | 关闭 FTP 连接 |
| `ftp_connect` | 建立新 FTP 连接 |
| `ftp_delete` | 删除 FTP 服务器上的文件 |
| `ftp_exec` | 在 FTP 服务器运行指定的命令 |
| `ftp_fget` | 从 FTP 服务器上下载文件并保存到本地已打开的文件中 |
| `ftp_fput` | 上传已打开的文件到 FTP 服务器 |
| `ftp_get` | 从 FTP 服务器上下载文件 |
| `ftp_get_option` | 返回当前 FTP 连接的各种不同的选项设置 |
| `ftp_login` | 登录 FTP 服务器 |
| `ftp_mdtm` | 返回指定文件的最后修改时间 |
| `ftp_mkdir` | 建立新目录 |
| `ftp_mlsd` | 返回指定目录中的文件列表 |
| `ftp_nb_continue` | 连续获取／发送文件（以不分块的方式 non-blocking） |
| `ftp_nb_fget` | 从 FTP 服务器获取文件并写入到一个打开的文件（非阻塞） |
| `ftp_nb_fput` | 将文件存储到 FTP 服务器 （非阻塞） |
| `ftp_nb_get` | 从 FTP 服务器上获取文件并写入本地文件（non-blocking） |
| `ftp_nb_put` | 存储一个文件至 FTP 服务器（non-blocking） |
| `ftp_nlist` | 返回给定目录的文件列表 |
| `ftp_pasv` | 返回当前 FTP 被动模式是否打开 |
| `ftp_put` | 上传文件到 FTP 服务器 |
| `ftp_pwd` | 返回当前目录名 |
| `ftp_quit` | ftp_close 的 别名 |
| `ftp_raw` | 向 FTP 服务器发送命令 |
| `ftp_rawlist` | 返回指定目录下文件的详细列表 |
| `ftp_rename` | 更改 FTP 服务器上的文件或目录名 |
| `ftp_rmdir` | 删除目录 |
| `ftp_set_option` | 设置各种 FTP 运行时选项 |
| `ftp_site` | 向服务器发送 SITE 命令 |
| `ftp_size` | 返回指定文件的大小 |
| `ftp_ssl_connect` | 打开安全 SSL-FTP 连接 |
| `ftp_systype` | 返回远程 FTP 服务器的操作系统类型 |
| `ftruncate` | 将文件截断到指定的长度 |
| `func_get_arg` | 返回参数列表的某一项 |
| `func_get_args` | 返回一个包含函数参数列表的数组 |
| `func_num_args` | 返回传递给函数的参数数量 |
| `function_exists` | 如果给定的函数已经被定义就返回 true |
| `fwrite` | 写入文件（可安全用于二进制文件） |

### G

| 函数名 | 功能描述 |
|--------|----------|
| `gc_collect_cycles` | 强制收集所有现存的垃圾循环周期 |
| `gc_disable` | 停用循环引用收集器 |
| `gc_enable` | 激活循环引用收集器 |
| `gc_enabled` | 返回循环引用计数器的状态 |
| `gc_mem_caches` | Reclaims memory used by the Zend Engine memory manager |
| `gc_status` | 获取有关垃圾回收的信息 |
| `gd_info` | 取得当前安装的 GD 库的信息 |
| `geoip_asnum_by_name` | 获取自治系统号(ASN) |
| `geoip_continent_code_by_name` | 获取七大洲的大写字母简称 |
| `geoip_country_code_by_name` | 获取国家代码 |
| `geoip_country_code3_by_name` | 获取三个字母组成的国家简称 |
| `geoip_country_name_by_name` | 获取国家的全称 |
| `geoip_database_info` | 获取 GeoIP 数据库的信息 |
| `geoip_db_avail` | GeoIP 数据库是否可用 |
| `geoip_db_filename` | 返回 GeoIP 数据库相对应的文件名 |
| `geoip_db_get_all_info` | 返回所有 GeoIP 数据库类型的详细信息 |
| `geoip_domain_by_name` | 获取二级域名 |
| `geoip_id_by_name` | 获取网络连接类型 |
| `geoip_isp_by_name` | 获取 ISP (网络服务提供商)的名称 |
| `geoip_netspeedcell_by_name` | 获取网络连接速度 |
| `geoip_org_by_name` | 获取机构的名称 |
| `geoip_record_by_name` | 返回 GeoIP 数据库中详细的城市信息 |
| `geoip_region_by_name` | 获取国家和地区代码 |
| `geoip_region_name_by_code` | 返回给定的国家和地区代码组合所对应的地区名称 |
| `geoip_setup_custom_directory` | 自定义 GeoIP 数据库的目录 |
| `geoip_time_zone_by_country_and_region` | 返回国家和地区的时区 |
| `get_browser` | 获取浏览器具有的功能 |
| `get_called_class` | 后期静态绑定（&quot;Late Static Binding&quot;）类的名称 |
| `get_cfg_var` | 获取 PHP 配置选项的值 |
| `get_class` | 返回对象的类名 |
| `get_class_methods` | 返回由类的方法名组成的数组 |
| `get_class_vars` | 获取类的默认属性 |
| `get_current_user` | 获取当前 PHP 脚本所有者名称 |
| `get_debug_type` | 以适合调试的方式获取变量的类型名称 |
| `get_declared_classes` | 返回由已定义类的名字所组成的数组 |
| `get_declared_interfaces` | 返回一个数组包含所有已声明的接口 |
| `get_declared_traits` | 返回所有已定义的 traits 的数组 |
| `get_defined_constants` | 返回所有常量的关联数组，键是常量名，值是常量值 |
| `get_defined_functions` | 返回所有已定义函数的数组 |
| `get_defined_vars` | 返回由所有已定义变量所组成的数组 |
| `get_error_handler` | 获取用户定义的错误处理函数 |
| `get_exception_handler` | 获取用户定义的异常处理函数 |
| `get_extension_funcs` | 返回模块函数名称的数组 |
| `get_headers` | 取得服务器响应 HTTP 请求所发送的所有标头 |
| `get_html_translation_table` | 返回使用 htmlspecialchars 和 htmlentities 后的转换表 |
| `get_include_path` | 获取当前的 include_path 配置选项 |
| `get_included_files` | 返回被 include 和 require 文件名的 array |
| `get_loaded_extensions` | 返回所有编译并加载模块名的 array |
| `get_magic_quotes_gpc` | 获取当前 magic_quotes_gpc 的配置选项设置 |
| `get_magic_quotes_runtime` | 获取当前 magic_quotes_runtime 配置选项的激活状态 |
| `get_mangled_object_vars` | 返回将对象属性混在一起的数组 |
| `get_meta_tags` | 从一个文件中提取所有的 meta 标签 content 属性，返回一个数组 |
| `get_object_vars` | 获取指定对象的属性 |
| `get_parent_class` | 检索对象或者类的父级类名 |
| `get_required_files` | 别名 get_included_files |
| `get_resource_id` | 返回给定资源的整数标识符 |
| `get_resource_type` | 返回资源类型 |
| `get_resources` | Returns active resources |
| `getallheaders` | 获取全部 HTTP 请求 header |
| `getcwd` | 取得当前工作目录 |
| `getdate` | 获取日期/时间信息 |
| `getenv` | 获取单个或者全部环境变量 |
| `gethostbyaddr` | 获取指定 IP 地址对应的 Internet 主机名 |
| `gethostbyname` | 返回主机名对应的 IPv4地址。 |
| `gethostbynamel` | 获取互联网主机名对应的 IPv4 地址列表 |
| `gethostname` | 获取主机名 |
| `getimagesize` | 取得图像大小 |
| `getimagesizefromstring` | 从字符串中获取图像尺寸信息 |
| `getlastmod` | 获取页面最后修改的时间 |
| `getmxrr` | 获取 Internet 主机名对应的 MX 记录 |
| `getmygid` | 获取当前 PHP 脚本拥有者的 GID |
| `getmyinode` | 获取当前脚本的索引节点（inode） |
| `getmypid` | 获取 PHP 进程的 ID |
| `getmyuid` | 获取 PHP 脚本所有者的 UID |
| `getopt` | 从命令行参数列表中获取选项 |
| `getprotobyname` | Get protocol number associated with protocol name |
| `getprotobynumber` | Get protocol name associated with protocol number |
| `getrandmax` | 显示随机数最大的可能值 |
| `getrusage` | 获取当前资源使用状况 |
| `getservbyname` | 获取互联网服务协议对应的端口 |
| `getservbyport` | Get Internet service which corresponds to port and protocol |
| `getSession` | Connect to a MySQL server |
| `gettext` | Lookup a message in the current domain |
| `gettimeofday` | 取得当前时间 |
| `gettype` | 获取变量的类型 |
| `glob` | 寻找与模式匹配的文件路径 |
| `gmdate` | 格式化 GMT/UTC 日期／时间 |
| `gmmktime` | 取得 GMT 日期的 UNIX 时间戳 |
| `gmp_abs` | Absolute value |
| `gmp_add` | Add numbers |
| `gmp_and` | Bitwise AND |
| `gmp_binomial` | Calculates binomial coefficient |
| `gmp_clrbit` | Clear bit |
| `gmp_cmp` | Compare numbers |
| `gmp_com` | Calculates one's complement |
| `gmp_div` | 别名 gmp_div_q |
| `gmp_div_q` | Divide numbers |
| `gmp_div_qr` | Divide numbers and get quotient and remainder |
| `gmp_div_r` | Remainder of the division of numbers |
| `gmp_divexact` | Exact division of numbers |
| `gmp_export` | Export to a binary string |
| `gmp_fact` | Factorial |
| `gmp_gcd` | Calculate GCD |
| `gmp_gcdext` | Calculate GCD and multipliers |
| `gmp_hamdist` | Hamming distance |
| `gmp_import` | Import from a binary string |
| `gmp_init` | Create GMP number |
| `gmp_intval` | Convert GMP number to integer |
| `gmp_invert` | Inverse by modulo |
| `gmp_jacobi` | Jacobi symbol |
| `gmp_kronecker` | Kronecker symbol |
| `gmp_lcm` | Calculate LCM |
| `gmp_legendre` | Legendre symbol |
| `gmp_mod` | Modulo operation |
| `gmp_mul` | Multiply numbers |
| `gmp_neg` | Negate number |
| `gmp_nextprime` | Find next prime number |
| `gmp_or` | Bitwise OR |
| `gmp_perfect_power` | Perfect power check |
| `gmp_perfect_square` | Perfect square check |
| `gmp_popcount` | Population count |
| `gmp_pow` | Raise number into power |
| `gmp_powm` | Raise number into power with modulo |
| `gmp_prob_prime` | Check if number is &quot;probably prime&quot; |
| `gmp_random` | Random number |
| `gmp_random_bits` | Random number |
| `gmp_random_range` | Get a uniformly selected integer |
| `gmp_random_seed` | Sets the RNG seed |
| `gmp_root` | Take the integer part of nth root |
| `gmp_rootrem` | Take the integer part and remainder of nth root |
| `gmp_scan0` | Scan for 0 |
| `gmp_scan1` | Scan for 1 |
| `gmp_setbit` | Set bit |
| `gmp_sign` | Sign of number |
| `gmp_sqrt` | Calculate square root |
| `gmp_sqrtrem` | Square root with remainder |
| `gmp_strval` | Convert GMP number to string |
| `gmp_sub` | Subtract numbers |
| `gmp_testbit` | Tests if a bit is set |
| `gmp_xor` | Bitwise XOR |
| `gmstrftime` | 根据区域设置格式化 GMT/UTC 时间/日期 |
| `gnupg_adddecryptkey` | Add a key for decryption |
| `gnupg_addencryptkey` | Add a key for encryption |
| `gnupg_addsignkey` | Add a key for signing |
| `gnupg_cleardecryptkeys` | Removes all keys which were set for decryption before |
| `gnupg_clearencryptkeys` | Removes all keys which were set for encryption before |
| `gnupg_clearsignkeys` | Removes all keys which were set for signing before |
| `gnupg_decrypt` | Decrypts a given text |
| `gnupg_decryptverify` | Decrypts and verifies a given text |
| `gnupg_deletekey` | Delete a key from the keyring |
| `gnupg_encrypt` | Encrypts a given text |
| `gnupg_encryptsign` | Encrypts and signs a given text |
| `gnupg_export` | Exports a key |
| `gnupg_getengineinfo` | Returns the engine info |
| `gnupg_geterror` | Returns the errortext, if a function fails |
| `gnupg_geterrorinfo` | Returns the error info |
| `gnupg_getprotocol` | Returns the currently active protocol for all operations |
| `gnupg_gettrustlist` | Search the trust items |
| `gnupg_import` | Imports a key |
| `gnupg_init` | Initialize a connection |
| `gnupg_keyinfo` | Returns an array with information about all keys that matches the given pattern |
| `gnupg_listsignatures` | List key signatures |
| `gnupg_setarmor` | Toggle armored output |
| `gnupg_seterrormode` | Sets the mode for error_reporting |
| `gnupg_setsignmode` | Sets the mode for signing |
| `gnupg_sign` | Signs a given text |
| `gnupg_verify` | Verifies a signed text |
| `grapheme_extract` | Function to extract a sequence of default grapheme clusters from a text buffer, which must be encoded in UTF-8 |
| `grapheme_str_split` | Split a string into an array |
| `grapheme_stripos` | Find position (in grapheme units) of first occurrence of a case-insensitive string |
| `grapheme_stristr` | Returns part of haystack string from the first occurrence of case-insensitive needle to the end of haystack |
| `grapheme_strlen` | Get string length in grapheme units |
| `grapheme_strpos` | Find position (in grapheme units) of first occurrence of a string |
| `grapheme_strripos` | Find position (in grapheme units) of last occurrence of a case-insensitive string |
| `grapheme_strrpos` | Find position (in grapheme units) of last occurrence of a string |
| `grapheme_strstr` | Returns part of haystack string from the first occurrence of needle to the end of haystack |
| `grapheme_substr` | Return part of a string |
| `gregoriantojd` | 将公历日期转为儒略日数 |
| `gzclose` | Close an open gz-file pointer |
| `gzcompress` | Compress a string |
| `gzdecode` | Decodes a gzip compressed string |
| `gzdeflate` | Deflate a string |
| `gzencode` | Create a gzip compressed string |
| `gzeof` | Test for EOF on a gz-file pointer |
| `gzfile` | Read entire gz-file into an array |
| `gzgetc` | Get character from gz-file pointer |
| `gzgets` | Get line from file pointer |
| `gzgetss` | Get line from gz-file pointer and strip HTML tags |
| `gzinflate` | Inflate a deflated string |
| `gzopen` | Open gz-file |
| `gzpassthru` | Output all remaining data on a gz-file pointer |
| `gzputs` | 别名 gzwrite |
| `gzread` | Binary-safe gz-file read |
| `gzrewind` | Rewind the position of a gz-file pointer |
| `gzseek` | Seek on a gz-file pointer |
| `gztell` | Tell gz-file pointer read/write position |
| `gzuncompress` | Uncompress a compressed string |
| `gzwrite` | Binary-safe gz-file write |

### H

| 函数名 | 功能描述 |
|--------|----------|
| `hash` | 生成散列值（消息摘要） |
| `hash_algos` | 返回已注册的散列算法列表 |
| `hash_copy` | 拷贝哈希运算上下文 |
| `hash_equals` | 可防止时序攻击的字符串比较 |
| `hash_file` | 给指定文件的内容生成散列值 |
| `hash_final` | 结束增量散列且返回摘要结果 |
| `hash_hkdf` | Generate a HKDF key derivation of a supplied key input |
| `hash_hmac` | 使用 HMAC 方法生成带有密钥的散列值 |
| `hash_hmac_algos` | 返回适用于 hash_hmac 的已注册散列算法列表 |
| `hash_hmac_file` | 使用 HMAC 方法和给定文件的内容生成带密钥的散列值 |
| `hash_init` | 初始化增量散列运算上下文 |
| `hash_pbkdf2` | 生成所提供密码的 PBKDF2 密钥导出 |
| `hash_update` | 向活跃的哈希运算上下文中填充数据 |
| `hash_update_file` | 从文件向活跃的散列运算上下文中填充数据 |
| `hash_update_stream` | 从打开的流向活跃的散列运算上下文中填充数据 |
| `header` | 发送原生 HTTP 头 |
| `header_register_callback` | 调用一个 header 函数 |
| `header_remove` | 删除之前设置的 HTTP 头 |
| `headers_list` | 返回已发送的 HTTP 响应头（或准备发送的） |
| `headers_sent` | 检测消息头是否已经发送 |
| `hebrev` | 将逻辑顺序希伯来文（logical-Hebrew）转换为视觉顺序希伯来文（visual-Hebrew） |
| `hebrevc` | 将逻辑顺序希伯来文（logical-Hebrew）转换为视觉顺序希伯来文（visual-Hebrew），并且转换换行符 |
| `hex2bin` | 转换十六进制字符串为二进制字符串 |
| `hexdec` | 十六进制转换为十进制 |
| `highlight_file` | 语法高亮一个文件 |
| `highlight_string` | 字符串的语法高亮 |
| `hrtime` | 获取系统的高精度时间 |
| `html_entity_decode` | Convert HTML entities to their corresponding characters |
| `htmlentities` | 将字符转换为 HTML 转义字符 |
| `htmlspecialchars` | 将特殊字符转换为 HTML 实体 |
| `htmlspecialchars_decode` | 将特殊的 HTML 实体转换回普通字符 |
| `http_build_query` | 生成 URL-encode 之后的请求字符串 |
| `http_clear_last_response_headers` | Clears the stored HTTP response headers |
| `http_get_last_response_headers` | Retrieve last HTTP response headers |
| `http_response_code` | 获取/设置响应的 HTTP 状态码 |
| `hypot` | 计算直角三角形的斜边长度 |

### I

| 函数名 | 功能描述 |
|--------|----------|
| `ibase_add_user` | Add a user to a security database |
| `ibase_affected_rows` | Return the number of rows that were affected by the previous query |
| `ibase_backup` | Initiates a backup task in the service manager and returns immediately |
| `ibase_blob_add` | Add data into a newly created blob |
| `ibase_blob_cancel` | Cancel creating blob |
| `ibase_blob_close` | Close blob |
| `ibase_blob_create` | Create a new blob for adding data |
| `ibase_blob_echo` | Output blob contents to browser |
| `ibase_blob_get` | Get len bytes data from open blob |
| `ibase_blob_import` | Create blob, copy file in it, and close it |
| `ibase_blob_info` | Return blob length and other useful info |
| `ibase_blob_open` | Open blob for retrieving data parts |
| `ibase_close` | Close a connection to an InterBase database |
| `ibase_commit` | Commit a transaction |
| `ibase_commit_ret` | Commit a transaction without closing it |
| `ibase_connect` | Open a connection to a database |
| `ibase_db_info` | Request statistics about a database |
| `ibase_delete_user` | Delete a user from a security database |
| `ibase_drop_db` | Drops a database |
| `ibase_errcode` | Return an error code |
| `ibase_errmsg` | Return error messages |
| `ibase_execute` | Execute a previously prepared query |
| `ibase_fetch_assoc` | Fetch a result row from a query as an associative array |
| `ibase_fetch_object` | Get an object from a InterBase database |
| `ibase_fetch_row` | Fetch a row from an InterBase database |
| `ibase_field_info` | Get information about a field |
| `ibase_free_event_handler` | Cancels a registered event handler |
| `ibase_free_query` | Free memory allocated by a prepared query |
| `ibase_free_result` | Free a result set |
| `ibase_gen_id` | Increments the named generator and returns its new value |
| `ibase_maintain_db` | Execute a maintenance command on the database server |
| `ibase_modify_user` | Modify a user to a security database |
| `ibase_name_result` | Assigns a name to a result set |
| `ibase_num_fields` | Get the number of fields in a result set |
| `ibase_num_params` | Return the number of parameters in a prepared query |
| `ibase_param_info` | Return information about a parameter in a prepared query |
| `ibase_pconnect` | Open a persistent connection to an InterBase database |
| `ibase_prepare` | Prepare a query for later binding of parameter placeholders and execution |
| `ibase_query` | Execute a query on an InterBase database |
| `ibase_restore` | Initiates a restore task in the service manager and returns immediately |
| `ibase_rollback` | Roll back a transaction |
| `ibase_rollback_ret` | Roll back a transaction without closing it |
| `ibase_server_info` | Request information about a database server |
| `ibase_service_attach` | Connect to the service manager |
| `ibase_service_detach` | Disconnect from the service manager |
| `ibase_set_event_handler` | Register a callback function to be called when events are posted |
| `ibase_trans` | Begin a transaction |
| `ibase_wait_event` | Wait for an event to be posted by the database |
| `iconv` | 将字符串从一个字符编码转换到另一个字符编码 |
| `iconv_get_encoding` | 获取 iconv 扩展的内部配置变量 |
| `iconv_mime_decode` | 解码一个MIME头字段 |
| `iconv_mime_decode_headers` | 一次性解码多个 MIME 头字段 |
| `iconv_mime_encode` | Composes a MIME header field |
| `iconv_set_encoding` | 为字符编码转换设定当前设置 |
| `iconv_strlen` | 返回字符串的字符数统计 |
| `iconv_strpos` | Finds position of first occurrence of a needle within a haystack |
| `iconv_strrpos` | Finds the last occurrence of a needle within a haystack |
| `iconv_substr` | 截取字符串的部分 |
| `idate` | 将本地日期/时间格式化为整数 |
| `idn_to_ascii` | 将域名转换为 IDNA ASCII 格式 |
| `idn_to_utf8` | 将域名从 IDNA ASCII 转换为 Unicode |
| `igbinary_serialize` | Generates a compact, storable binary representation of a value |
| `igbinary_unserialize` | Creates a PHP value from a stored representation from igbinary_serialize |
| `ignore_user_abort` | 设置客户端断开连接时是否中断脚本的执行 |
| `image_type_to_extension` | 取得图像类型的文件后缀 |
| `image_type_to_mime_type` | 取得 getimagesize、exif_read_data、exif_thumbnail、exif_imagetype 所返回的图像类型的 MIME 类型 |
| `image2wbmp` | 输出图象到浏览器或文件。 |
| `imageaffine` | 返回经过仿射变换后的图像，剪切区域可选 |
| `imageaffinematrixconcat` | Concatenate two affine transformation matrices |
| `imageaffinematrixget` | Get an affine transformation matrix |
| `imagealphablending` | 设定图像的混色模式 |
| `imageantialias` | 是否使用抗锯齿（antialias）功能 |
| `imagearc` | 画弧线 |
| `imageavif` | 输出图象到浏览器或文件。 |
| `imagebmp` | Output a BMP image to browser or file |
| `imagechar` | 水平地绘制一个字符 |
| `imagecharup` | 垂直地绘制一个字符 |
| `imagecolorallocate` | 为图像分配颜色 |
| `imagecolorallocatealpha` | 为图像分配颜色 |
| `imagecolorat` | 取得某像素的颜色索引值 |
| `imagecolorclosest` | 取得与指定的颜色最接近的颜色索引值 |
| `imagecolorclosestalpha` | 获取最接近指定颜色 + alpha 的颜色索引 |
| `imagecolorclosesthwb` | 取得与给定颜色最接近的色度的黑白色的索引 |
| `imagecolordeallocate` | 取消图像颜色的分配 |
| `imagecolorexact` | 取得指定颜色的索引值 |
| `imagecolorexactalpha` | 取得指定的颜色加透明度的索引值 |
| `imagecolormatch` | 使一个图像中调色板版本的颜色与真彩色版本更能匹配 |
| `imagecolorresolve` | 取得指定颜色的索引值或有可能得到的最接近的替代值 |
| `imagecolorresolvealpha` | 取得指定颜色 + alpha 或其最接近的替代值 |
| `imagecolorset` | 给指定调色板索引设定颜色 |
| `imagecolorsforindex` | 获取索引的颜色 |
| `imagecolorstotal` | 取得图像调色板中的颜色数量 |
| `imagecolortransparent` | 将颜色定义为透明 |
| `imageconvolution` | 用系数 div 和 offset 申请一个 3x3 的卷积矩阵 |
| `imagecopy` | 拷贝图像的一部分 |
| `imagecopymerge` | 拷贝并合并图像的一部分 |
| `imagecopymergegray` | 用灰度复制并合并图像的一部分 |
| `imagecopyresampled` | 重采样拷贝部分图像并调整大小 |
| `imagecopyresized` | 拷贝部分图像并调整大小 |
| `imagecreate` | 创建新的基于调色板的图像 |
| `imagecreatefromavif` | 由文件或 URL 创建一个新图象。 |
| `imagecreatefrombmp` | 由文件或 URL 创建一个新图象。 |
| `imagecreatefromgd` | 从 GD 文件或 URL 新建一图像 |
| `imagecreatefromgd2` | 从 GD2 文件或 URL 新建一图像 |
| `imagecreatefromgd2part` | 从指定的 GD2 文件或 URL 的部分创建新图像 |
| `imagecreatefromgif` | 由文件或 URL 创建一个新图象。 |
| `imagecreatefromjpeg` | 由文件或 URL 创建一个新图象。 |
| `imagecreatefrompng` | 由文件或 URL 创建一个新图象。 |
| `imagecreatefromstring` | 从字符串的图像流中新建图像 |
| `imagecreatefromtga` | 由文件或 URL 创建一个新图象。 |
| `imagecreatefromwbmp` | 由文件或 URL 创建一个新图象。 |
| `imagecreatefromwebp` | 由文件或 URL 创建一个新图象。 |
| `imagecreatefromxbm` | 由文件或 URL 创建一个新图象。 |
| `imagecreatefromxpm` | 由文件或 URL 创建一个新图象。 |
| `imagecreatetruecolor` | 新建真彩色图像 |
| `imagecrop` | Crop an image to the given rectangle |
| `imagecropauto` | Crop an image automatically using one of the available modes |
| `imagedashedline` | 绘制虚线 |
| `imagedestroy` | 销毁图像 |
| `imageellipse` | 画椭圆 |
| `imagefill` | 漫水填充 |
| `imagefilledarc` | 绘制部分弧形并填充 |
| `imagefilledellipse` | 绘制椭圆并填充 |
| `imagefilledpolygon` | 绘制多边形并填充 |
| `imagefilledrectangle` | 绘制矩形并填充 |
| `imagefilltoborder` | 漫水填充特定颜色 |
| `imagefilter` | 对图像使用过滤器 |
| `imageflip` | Flips an image using a given mode |
| `imagefontheight` | 获取字体高度 |
| `imagefontwidth` | 获取字体宽度 |
| `imageftbbox` | 通过 freetype2 使用字体给出文本的边界框 |
| `imagefttext` | 使用 FreeType 2 字体将文本写入图像 |
| `imagegammacorrect` | 对 GD 图像应用伽玛校正 |
| `imagegd` | 将 GD 图像输出到浏览器或文件 |
| `imagegd2` | 将 GD2 图像输出到浏览器或文件 |
| `imagegetclip` | Get the clipping rectangle |
| `imagegetinterpolation` | Get the interpolation method |
| `imagegif` | 输出图象到浏览器或文件。 |
| `imagegrabscreen` | Captures the whole screen |
| `imagegrabwindow` | Captures a window |
| `imageinterlace` | 启用或禁用隔行扫描 |
| `imageistruecolor` | 检查图像是否为真彩色图像 |
| `imagejpeg` | 输出图象到浏览器或文件。 |
| `imagelayereffect` | 设定 alpha 混合标志以使用分层效果 |
| `imageline` | 绘制直线 |
| `imageloadfont` | 载入新字体 |
| `imageopenpolygon` | Draws an open polygon |
| `imagepalettecopy` | 将调色板从一个图像复制到另一个 |
| `imagepalettetotruecolor` | Converts a palette based image to true color |
| `imagepng` | 将 PNG 图像输出到浏览器或文件 |
| `imagepolygon` | 绘制多边形 |
| `imagerectangle` | 绘制矩形 |
| `imageresolution` | Get or set the resolution of the image |
| `imagerotate` | 用给定角度旋转图像 |
| `imagesavealpha` | 保存图像时是否保留完整的 alpha 通道信息 |
| `imagescale` | Scale an image using the given new width and height |
| `imagesetbrush` | 为线条设置笔刷图像 |
| `imagesetclip` | Set the clipping rectangle |
| `imagesetinterpolation` | Set the interpolation method |
| `imagesetpixel` | 设置单个像素 |
| `imagesetstyle` | 设定线条的样式 |
| `imagesetthickness` | 设定画线的粗细 |
| `imagesettile` | 设置要填充的平铺图像 |
| `imagestring` | 水平绘制字符串 |
| `imagestringup` | 垂直绘制字符串 |
| `imagesx` | 取得图像宽度 |
| `imagesy` | 取得图像高度 |
| `imagetruecolortopalette` | 将真彩色图像转换为调色板图像 |
| `imagettfbbox` | 取得使用 TrueType 字体的文本的边界框 |
| `imagettftext` | 用 TrueType 字体向图像写入文本 |
| `imagetypes` | 返回 PHP 内置支持的图像类型 |
| `imagewbmp` | 输出图象到浏览器或文件。 |
| `imagewebp` | 将 WebP 格式的图像输出到浏览器或文件 |
| `imagexbm` | 输出 XBM 图像到浏览器或文件 |
| `imap_8bit` | Convert an 8bit string to a quoted-printable string |
| `imap_alerts` | Returns all IMAP alert messages that have occurred |
| `imap_append` | Append a string message to a specified mailbox |
| `imap_base64` | Decode BASE64 encoded text |
| `imap_binary` | Convert an 8bit string to a base64 string |
| `imap_body` | Read the message body |
| `imap_bodystruct` | Read the structure of a specified body section of a specific message |
| `imap_check` | Check current mailbox |
| `imap_clearflag_full` | Clears flags on messages |
| `imap_close` | Close an IMAP stream |
| `imap_create` | 别名 imap_createmailbox |
| `imap_createmailbox` | Create a new mailbox |
| `imap_delete` | Mark a message for deletion from current mailbox |
| `imap_deletemailbox` | Delete a mailbox |
| `imap_errors` | Returns all of the IMAP errors that have occurred |
| `imap_expunge` | Delete all messages marked for deletion |
| `imap_fetch_overview` | Read an overview of the information in the headers of the given message |
| `imap_fetchbody` | Fetch a particular section of the body of the message |
| `imap_fetchheader` | Returns header for a message |
| `imap_fetchmime` | Fetch MIME headers for a particular section of the message |
| `imap_fetchstructure` | Read the structure of a particular message |
| `imap_fetchtext` | 别名 imap_body |
| `imap_gc` | Clears IMAP cache |
| `imap_get_quota` | Retrieve the quota level settings, and usage statics per mailbox |
| `imap_get_quotaroot` | Retrieve the quota settings per user |
| `imap_getacl` | Gets the ACL for a given mailbox |
| `imap_getmailboxes` | Read the list of mailboxes, returning detailed information on each one |
| `imap_getsubscribed` | List all the subscribed mailboxes |
| `imap_header` | 别名 imap_headerinfo |
| `imap_headerinfo` | Read the header of the message |
| `imap_headers` | Returns headers for all messages in a mailbox |
| `imap_is_open` | Check if the IMAP stream is still valid |
| `imap_last_error` | Gets the last IMAP error that occurred during this page request |
| `imap_list` | Read the list of mailboxes |
| `imap_listmailbox` | 别名 imap_list |
| `imap_listscan` | Returns the list of mailboxes that matches the given text |
| `imap_listsubscribed` | 别名 imap_lsub |
| `imap_lsub` | List all the subscribed mailboxes |
| `imap_mail` | Send an email message |
| `imap_mail_compose` | Create a MIME message based on given envelope and body sections |
| `imap_mail_copy` | Copy specified messages to a mailbox |
| `imap_mail_move` | Move specified messages to a mailbox |
| `imap_mailboxmsginfo` | Get information about the current mailbox |
| `imap_mime_header_decode` | Decode MIME header elements |
| `imap_msgno` | Gets the message sequence number for the given UID |
| `imap_mutf7_to_utf8` | Decode a modified UTF-7 string to UTF-8 |
| `imap_num_msg` | Gets the number of messages in the current mailbox |
| `imap_num_recent` | Gets the number of recent messages in current mailbox |
| `imap_open` | Open an IMAP stream to a mailbox |
| `imap_ping` | Check if the IMAP stream is still active |
| `imap_qprint` | Convert a quoted-printable string to an 8 bit string |
| `imap_rename` | 别名 imap_renamemailbox |
| `imap_renamemailbox` | Rename an old mailbox to new mailbox |
| `imap_reopen` | Reopen IMAP stream to new mailbox |
| `imap_rfc822_parse_adrlist` | Parses an address string |
| `imap_rfc822_parse_headers` | Parse mail headers from a string |
| `imap_rfc822_write_address` | Returns a properly formatted email address given the mailbox, host, and personal info |
| `imap_savebody` | Save a specific body section to a file |
| `imap_scan` | 别名 imap_listscan |
| `imap_scanmailbox` | 别名 imap_listscan |
| `imap_search` | This function returns an array of messages matching the given search criteria |
| `imap_set_quota` | Sets a quota for a given mailbox |
| `imap_setacl` | Sets the ACL for a given mailbox |
| `imap_setflag_full` | Sets flags on messages |
| `imap_sort` | Gets and sort messages |
| `imap_status` | Returns status information on a mailbox |
| `imap_subscribe` | Subscribe to a mailbox |
| `imap_thread` | Returns a tree of threaded message |
| `imap_timeout` | Set or fetch imap timeout |
| `imap_uid` | This function returns the UID for the given message sequence number |
| `imap_undelete` | Unmark the message which is marked deleted |
| `imap_unsubscribe` | Unsubscribe from a mailbox |
| `imap_utf7_decode` | Decodes a modified UTF-7 encoded string |
| `imap_utf7_encode` | Converts ISO-8859-1 string to modified UTF-7 text |
| `imap_utf8` | Converts MIME-encoded text to UTF-8 |
| `imap_utf8_to_mutf7` | Encode a UTF-8 string to modified UTF-7 |
| `implode` | 用字符串连接数组元素 |
| `in_array` | 检查数组中是否存在某个值 |
| `inet_ntop` | Converts a packed internet address to a human readable representation |
| `inet_pton` | Converts a human readable IP address to its packed in_addr representation |
| `inflate_add` | Incrementally inflate encoded data |
| `inflate_get_read_len` | Get number of bytes read so far |
| `inflate_get_status` | Get decompression status |
| `inflate_init` | Initialize an incremental inflate context |
| `ini_alter` | 别名 ini_set |
| `ini_get` | 获取一个配置选项的值 |
| `ini_get_all` | 获取所有配置选项 |
| `ini_parse_quantity` | Get interpreted size from ini shorthand syntax |
| `ini_restore` | 恢复配置选项的值 |
| `ini_set` | 为一个配置选项设置值 |
| `inotify_add_watch` | 添加监听到已初始化的 inotify 实例 |
| `inotify_init` | 初始化 inotify 实例 |
| `inotify_queue_len` | 如果有待处理事件，返回大于零的数字 |
| `inotify_read` | 从 inotify 实例读取事件 |
| `inotify_rm_watch` | 移除 inotify 实例的监听 |
| `intdiv` | 对除法结果取整 |
| `interface_exists` | 检查接口是否已被定义 |
| `intl_error_name` | Get symbolic name for a given error code |
| `intl_get_error_code` | Get the last error code |
| `intl_get_error_message` | Get description of the last error |
| `intl_is_failure` | Check whether the given error code indicates failure |
| `intval` | 获取变量的整数值 |
| `ip2long` | 将 IPV4 的字符串互联网协议转换成长整型数字 |
| `iptcembed` | 嵌入二进制 IPTC 数据到 JPEG 图像中 |
| `iptcparse` | 将二进制 IPTC 块解析为单个标签 |
| `is_a` | 检查对象是否属于一个给定的类型或子类型。 |
| `is_array` | 检测变量是否是数组 |
| `is_bool` | 检测变量是否是布尔值 |
| `is_callable` | 验证值是否可以在当前范围内作为函数调用 |
| `is_countable` | 验证变量内容是否为可数值 |
| `is_dir` | 判断给定文件名是否是一个目录 |
| `is_double` | 别名 is_float |
| `is_executable` | 判断给定文件名是否可执行 |
| `is_file` | 判断给定文件名是否为一个正常的文件 |
| `is_finite` | 判断浮点数是否是有效的有限值 |
| `is_float` | 检测变量是否是浮点型 |
| `is_infinite` | 判断浮点数是否为无限值 |
| `is_int` | 检测变量是否是整数 |
| `is_integer` | 别名 is_int |
| `is_iterable` | 验证变量的内容是否为可迭代值 |
| `is_link` | 判断给定文件名是否为一个符号连接 |
| `is_long` | 别名 is_int |
| `is_nan` | 判断浮点数是否是否为 NAN |
| `is_null` | 检测变量是否是 null |
| `is_numeric` | 检测变量是否是数字或数字字符串 |
| `is_object` | 检测变量是否是对象 |
| `is_readable` | 判断给定文件名是否可读 |
| `is_real` | 别名 is_float |
| `is_resource` | 查找变量是否为资源 |
| `is_scalar` | 查找变量是否是标量 |
| `is_soap_fault` | Checks if a SOAP call has failed |
| `is_string` | 检测变量的类型是否是字符串 |
| `is_subclass_of` | 检查对象是否继承或者实现（implement）此类 |
| `is_tainted` | Checks whether a string is tainted |
| `is_uploaded_file` | 判断文件是否是通过 HTTP POST 上传的 |
| `is_writable` | 判断给定的文件名是否可写 |
| `is_writeable` | is_writable 的别名 |
| `isset` | 检测变量是否已声明并且其值不为 null |
| `iterator_apply` | 为迭代器中每个元素调用函数 |
| `iterator_count` | 计算迭代器中元素的个数 |
| `iterator_to_array` | 复制迭代器中的元素到数组 |

### J

| 函数名 | 功能描述 |
|--------|----------|
| `jddayofweek` | 返回星期几 |
| `jdmonthname` | 返回月份名称 |
| `jdtofrench` | 从儒略日数转换为法国共和历 |
| `jdtogregorian` | 将儒略日数转换为公历日期 |
| `jdtojewish` | 将儒略日数转换为犹太历日期 |
| `jdtojulian` | 将儒略日数转换为儒略历日期 |
| `jdtounix` | 将儒略日数转换为 Unix 时间戳 |
| `jewishtojd` | 将犹太历日期转换为儒略日数 |
| `join` | 别名 implode |
| `jpeg2wbmp` | 将 JPEG 图像文件转换为 WBMP 图像文件 |
| `json_decode` | 对 JSON 格式的字符串进行解码 |
| `json_encode` | 对变量进行 JSON 编码 |
| `json_last_error` | 返回最后发生的错误 |
| `json_last_error_msg` | 返回最后一次调用 json_validate()、json_encode() 或 json_decode() 时产生的错误信息 |
| `json_validate` | 检查一个字符串是否包含有效的JSON |
| `juliantojd` | 儒略日期转为儒略日数 |

### K

| 函数名 | 功能描述 |
|--------|----------|
| `key` | 从关联数组中取得键名 |
| `key_exists` | 别名 array_key_exists |
| `krsort` | 对数组按照键名逆向排序 |
| `ksort` | 对数组根据键名升序排序 |

### L

| 函数名 | 功能描述 |
|--------|----------|
| `lcfirst` | 使字符串的第一个字符小写 |
| `lcg_value` | 组合线性同余发生器 |
| `lchgrp` | 修改符号链接的所有组 |
| `lchown` | 修改符号链接的所有者 |
| `ldap_8859_to_t61` | Translate 8859 characters to t61 characters |
| `ldap_add` | Add entries to LDAP directory |
| `ldap_add_ext` | Add entries to LDAP directory |
| `ldap_bind` | 绑定 LDAP 目录 |
| `ldap_bind_ext` | Bind to LDAP directory |
| `ldap_close` | 别名 ldap_unbind |
| `ldap_compare` | Compare value of attribute found in entry specified with DN |
| `ldap_connect` | Connect to an LDAP server |
| `ldap_connect_wallet` | Connect to an LDAP server |
| `ldap_control_paged_result` | Send LDAP pagination control |
| `ldap_control_paged_result_response` | Retrieve the LDAP pagination cookie |
| `ldap_count_entries` | Count the number of entries in a search |
| `ldap_count_references` | Counts the number of references in a search result |
| `ldap_delete` | Delete an entry from a directory |
| `ldap_delete_ext` | Delete an entry from a directory |
| `ldap_dn2ufn` | Convert DN to User Friendly Naming format |
| `ldap_err2str` | Convert LDAP error number into string error message |
| `ldap_errno` | Return the LDAP error number of the last LDAP command |
| `ldap_error` | Return the LDAP error message of the last LDAP command |
| `ldap_escape` | Escape a string for use in an LDAP filter or DN |
| `ldap_exop` | Performs an extended operation |
| `ldap_exop_passwd` | PASSWD extended operation helper |
| `ldap_exop_refresh` | Refresh extended operation helper |
| `ldap_exop_sync` | Performs an extended operation |
| `ldap_exop_whoami` | WHOAMI extended operation helper |
| `ldap_explode_dn` | Splits DN into its component parts |
| `ldap_first_attribute` | Return first attribute |
| `ldap_first_entry` | Return first result id |
| `ldap_first_reference` | Return first reference |
| `ldap_free_result` | Free result memory |
| `ldap_get_attributes` | Get attributes from a search result entry |
| `ldap_get_dn` | Get the DN of a result entry |
| `ldap_get_entries` | Get all result entries |
| `ldap_get_option` | Get the current value for given option |
| `ldap_get_values` | Get all values from a result entry |
| `ldap_get_values_len` | Get all binary values from a result entry |
| `ldap_list` | Single-level search |
| `ldap_mod_add` | Add attribute values to current attributes |
| `ldap_mod_add_ext` | Add attribute values to current attributes |
| `ldap_mod_del` | Delete attribute values from current attributes |
| `ldap_mod_del_ext` | Delete attribute values from current attributes |
| `ldap_mod_replace` | Replace attribute values with new ones |
| `ldap_mod_replace_ext` | Replace attribute values with new ones |
| `ldap_modify` | 别名 ldap_mod_replace |
| `ldap_modify_batch` | Batch and execute modifications on an LDAP entry |
| `ldap_next_attribute` | Get the next attribute in result |
| `ldap_next_entry` | Get next result entry |
| `ldap_next_reference` | Get next reference |
| `ldap_parse_exop` | Parse result object from an LDAP extended operation |
| `ldap_parse_reference` | Extract information from reference entry |
| `ldap_parse_result` | Extract information from result |
| `ldap_read` | Read an entry |
| `ldap_rename` | Modify the name of an entry |
| `ldap_rename_ext` | Modify the name of an entry |
| `ldap_sasl_bind` | Bind to LDAP directory using SASL |
| `ldap_search` | Search LDAP tree |
| `ldap_set_option` | Set the value of the given option |
| `ldap_set_rebind_proc` | Set a callback function to do re-binds on referral chasing |
| `ldap_sort` | Sort LDAP result entries on the client side |
| `ldap_start_tls` | Start TLS |
| `ldap_t61_to_8859` | Translate t61 characters to 8859 characters |
| `ldap_unbind` | Unbind from LDAP directory |
| `levenshtein` | 计算两个字符串之间的 Levenshtein 距离 |
| `libxml_clear_errors` | Clear libxml error buffer |
| `libxml_disable_entity_loader` | Disable the ability to load external entities |
| `libxml_get_errors` | Retrieve array of errors |
| `libxml_get_external_entity_loader` | Get the current external entity loader |
| `libxml_get_last_error` | Retrieve last error from libxml |
| `libxml_set_external_entity_loader` | Changes the default external entity loader |
| `libxml_set_streams_context` | Set the streams context for the next libxml document load or write |
| `libxml_use_internal_errors` | Disable libxml errors and allow user to fetch error information as needed |
| `link` | 建立一个硬连接 |
| `linkinfo` | 获取一个连接的信息 |
| `list` | 把数组中的值赋给一组变量 |
| `localeconv` | Get numeric formatting information |
| `localtime` | 取得本地时间 |
| `log` | 自然对数 |
| `log10` | 以 10 为底的对数 |
| `log1p` | 返回 log(1 + number)，甚至当 number 的值接近零也能计算出准确结果 |
| `long2ip` | 将长整型转化为字符串形式带点的互联网标准格式地址（IPV4） |
| `lstat` | 给出一个文件或符号连接的信息 |
| `ltrim` | 删除字符串开头的空白字符（或其他字符） |
| `lzf_compress` | LZF compression |
| `lzf_decompress` | LZF decompression |
| `lzf_optimized_for` | Determines what LZF extension was optimized for |

### M

| 函数名 | 功能描述 |
|--------|----------|
| `mail` | 发送邮件 |
| `mailparse_determine_best_xfer_encoding` | Gets the best way of encoding |
| `mailparse_msg_create` | Create a mime mail resource |
| `mailparse_msg_extract_part` | Extracts/decodes a message section |
| `mailparse_msg_extract_part_file` | Extracts/decodes a message section |
| `mailparse_msg_extract_whole_part_file` | Extracts a message section including headers without decoding the transfer encoding |
| `mailparse_msg_free` | Frees a MIME resource |
| `mailparse_msg_get_part` | Returns a handle on a given section in a mimemessage |
| `mailparse_msg_get_part_data` | Returns an associative array of info about the message |
| `mailparse_msg_get_structure` | Returns an array of mime section names in the supplied message |
| `mailparse_msg_parse` | Incrementally parse data into buffer |
| `mailparse_msg_parse_file` | Parses a file |
| `mailparse_rfc822_parse_addresses` | Parse RFC 822 compliant addresses |
| `mailparse_stream_encode` | Streams data from source file pointer, apply encoding and write to destfp |
| `mailparse_uudecode_all` | Scans the data from fp and extract each embedded uuencoded file |
| `max` | 找出最大值 |
| `mb_check_encoding` | 检查字符串在指定的编码里是否有效 |
| `mb_chr` | 按 Unicode 码位值返回字符 |
| `mb_convert_case` | 对字符串进行大小写转换 |
| `mb_convert_encoding` | 转换字符串，从一个字符编码到另一个字符编码 |
| `mb_convert_kana` | Convert &quot;kana&quot; one from another (&quot;zen-kaku&quot;, &quot;han-kaku&quot; and more) |
| `mb_convert_variables` | 转换一个或多个变量的字符编码 |
| `mb_decode_mimeheader` | 解码 MIME 头字段中的字符串 |
| `mb_decode_numericentity` | 根据 HTML 数字字符串解码成字符 |
| `mb_detect_encoding` | 检测字符的编码 |
| `mb_detect_order` | 设置/获取 字符编码的检测顺序 |
| `mb_encode_mimeheader` | 为 MIME 头编码字符串 |
| `mb_encode_numericentity` | Encode character to HTML numeric string reference |
| `mb_encoding_aliases` | Get aliases of a known encoding type |
| `mb_ereg` | Regular expression match with multibyte support |
| `mb_ereg_match` | Regular expression match for multibyte string |
| `mb_ereg_replace` | Replace regular expression with multibyte support |
| `mb_ereg_replace_callback` | Perform a regular expression search and replace with multibyte support using a callback |
| `mb_ereg_search` | Multibyte regular expression match for predefined multibyte string |
| `mb_ereg_search_getpos` | Returns start point for next regular expression match |
| `mb_ereg_search_getregs` | Retrieve the result from the last multibyte regular expression match |
| `mb_ereg_search_init` | Setup string and regular expression for a multibyte regular expression match |
| `mb_ereg_search_pos` | Returns position and length of a matched part of the multibyte regular expression for a predefined multibyte string |
| `mb_ereg_search_regs` | Returns the matched part of a multibyte regular expression |
| `mb_ereg_search_setpos` | Set start point of next regular expression match |
| `mb_eregi` | Regular expression match ignoring case with multibyte support |
| `mb_eregi_replace` | Replace regular expression with multibyte support ignoring case |
| `mb_get_info` | 获取 mbstring 的内部设置 |
| `mb_http_input` | 检测 HTTP 输入字符编码 |
| `mb_http_output` | 设置/获取 HTTP 输出字符编码 |
| `mb_internal_encoding` | 设置/获取内部字符编码 |
| `mb_language` | 设置/获取当前的语言 |
| `mb_lcfirst` | Make a string's first character lowercase |
| `mb_list_encodings` | 返回所有支持编码的数组 |
| `mb_ltrim` | Strip whitespace (or other characters) from the beginning of a string |
| `mb_ord` | 获取字符的 Unicode 码位值 |
| `mb_output_handler` | 在输出缓冲中转换字符编码的回调函数 |
| `mb_parse_str` | 解析 GET/POST/COOKIE 数据并设置全局变量 |
| `mb_preferred_mime_name` | 获取 MIME 字符串 |
| `mb_regex_encoding` | Set/Get character encoding for multibyte regex |
| `mb_regex_set_options` | Set/Get the default options for mbregex functions |
| `mb_rtrim` | Strip whitespace (or other characters) from the end of a string |
| `mb_scrub` | Replace ill-formed byte sequences with the substitute character |
| `mb_send_mail` | 发送编码过的邮件 |
| `mb_split` | 使用正则表达式分割多字节字符串 |
| `mb_str_pad` | Pad a multibyte string to a certain length with another multibyte string |
| `mb_str_split` | 指定多字节字符串，返回其字符数组 |
| `mb_strcut` | 获取字符的一部分 |
| `mb_strimwidth` | 获取按指定宽度截断的字符串 |
| `mb_stripos` | 大小写不敏感地查找字符串在另一个字符串中首次出现的位置 |
| `mb_stristr` | 大小写不敏感地查找字符串在另一个字符串里的首次出现 |
| `mb_strlen` | 获取字符串的长度 |
| `mb_strpos` | 查找字符串在另一个字符串中首次出现的位置 |
| `mb_strrchr` | 查找指定字符在另一个字符串中最后一次的出现 |
| `mb_strrichr` | 大小写不敏感地查找指定字符在另一个字符串中最后一次的出现 |
| `mb_strripos` | 大小写不敏感地在字符串中查找一个字符串最后出现的位置 |
| `mb_strrpos` | 查找字符串在一个字符串中最后出现的位置 |
| `mb_strstr` | 查找字符串在另一个字符串里的首次出现 |
| `mb_strtolower` | 使字符串小写 |
| `mb_strtoupper` | 使字符串大写 |
| `mb_strwidth` | 返回字符串的宽度 |
| `mb_substitute_character` | 设置/获取替代字符 |
| `mb_substr` | 获取部分字符串 |
| `mb_substr_count` | 统计字符串出现的次数 |
| `mb_trim` | Strip whitespace (or other characters) from the beginning and end of a string |
| `mb_ucfirst` | Make a string's first character uppercase |
| `mcrypt_create_iv` | 从随机源创建初始向量 |
| `mcrypt_decrypt` | 使用给定参数解密密文 |
| `mcrypt_enc_get_algorithms_name` | 返回打开的算法名称 |
| `mcrypt_enc_get_block_size` | 返回打开的算法的分组大小 |
| `mcrypt_enc_get_iv_size` | 返回打开的算法的初始向量大小 |
| `mcrypt_enc_get_key_size` | 返回打开的模式所能支持的最长密钥 |
| `mcrypt_enc_get_modes_name` | 返回打开的模式的名称 |
| `mcrypt_enc_get_supported_key_sizes` | 以数组方式返回打开的算法所支持的密钥长度 |
| `mcrypt_enc_is_block_algorithm` | 检测打开模式的算法是否为分组算法 |
| `mcrypt_enc_is_block_algorithm_mode` | 检测打开的模式是否支持分组加密 |
| `mcrypt_enc_is_block_mode` | 检测打开的模式是否以分组方式输出 |
| `mcrypt_enc_self_test` | 在打开的模块上进行自检 |
| `mcrypt_encrypt` | 使用给定参数加密明文 |
| `mcrypt_generic` | 加密数据 |
| `mcrypt_generic_deinit` | 对加密模块进行清理工作 |
| `mcrypt_generic_init` | 初始化加密所需的缓冲区 |
| `mcrypt_get_block_size` | 获得加密算法的分组大小 |
| `mcrypt_get_cipher_name` | 获取加密算法名称 |
| `mcrypt_get_iv_size` | 返回指定算法/模式组合的初始向量大小 |
| `mcrypt_get_key_size` | 获取指定加密算法的密钥大小 |
| `mcrypt_list_algorithms` | 获取支持的加密算法 |
| `mcrypt_list_modes` | 获取所支持的模式 |
| `mcrypt_module_close` | 关闭加密模块 |
| `mcrypt_module_get_algo_block_size` | 返回指定算法的分组大小 |
| `mcrypt_module_get_algo_key_size` | 获取打开模式所支持的最大密钥大小 |
| `mcrypt_module_get_supported_key_sizes` | 以数组形式返回打开的算法所支持的密钥大小 |
| `mcrypt_module_is_block_algorithm` | 检测指定算法是否为分组加密算法 |
| `mcrypt_module_is_block_algorithm_mode` | 返回指定模块是否是分组加密模式 |
| `mcrypt_module_is_block_mode` | 检测指定模式是否以分组方式输出 |
| `mcrypt_module_open` | 打开算法和模式对应的模块 |
| `mcrypt_module_self_test` | 在指定模块上执行自检 |
| `md5` | 计算字符串的 MD5 散列值 |
| `md5_file` | 计算指定文件的 MD5 散列值 |
| `mdecrypt_generic` | 解密数据 |
| `memcache_debug` | 打开/关闭调试输出 |
| `memory_get_peak_usage` | 返回分配给 PHP 内存的峰值 |
| `memory_get_usage` | 返回分配给 PHP 的内存量 |
| `memory_reset_peak_usage` | Reset the peak memory usage |
| `metaphone` | Calculate the metaphone key of a string |
| `method_exists` | 检查类的方法是否存在 |
| `mhash` | Computes hash |
| `mhash_count` | Gets the highest available hash ID |
| `mhash_get_block_size` | Gets the block size of the specified hash |
| `mhash_get_hash_name` | Gets the name of the specified hash |
| `mhash_keygen_s2k` | Generates a key |
| `microtime` | 返回当前 Unix 时间戳和微秒数 |
| `mime_content_type` | 检测文件的 MIME 类型 |
| `min` | 找出最小值 |
| `mkdir` | 新建目录 |
| `mktime` | 取得一个日期的 Unix 时间戳 |
| `money_format` | 将数字格式化成货币字符串 |
| `MongoDB\BSON\fromJSON` | Returns the BSON representation of a JSON value |
| `MongoDB\BSON\fromPHP` | Returns the BSON representation of a PHP value |
| `MongoDB\BSON\toCanonicalExtendedJSON` | Returns the Canonical Extended JSON representation of a BSON value |
| `MongoDB\BSON\toJSON` | Returns the Legacy Extended JSON representation of a BSON value |
| `MongoDB\BSON\toPHP` | Returns the PHP representation of a BSON value |
| `MongoDB\BSON\toRelaxedExtendedJSON` | Returns the Relaxed Extended JSON representation of a BSON value |
| `MongoDB\Driver\Monitoring\addSubscriber` | Registers a monitoring event subscriber globally |
| `MongoDB\Driver\Monitoring\removeSubscriber` | Unregisters a monitoring event subscriber globally |
| `move_uploaded_file` | 将上传的文件移动到新位置 |
| `mqseries_back` | MQSeries MQBACK |
| `mqseries_begin` | MQseries MQBEGIN |
| `mqseries_close` | MQSeries MQCLOSE |
| `mqseries_cmit` | MQSeries MQCMIT |
| `mqseries_conn` | MQSeries MQCONN |
| `mqseries_connx` | MQSeries MQCONNX |
| `mqseries_disc` | MQSeries MQDISC |
| `mqseries_get` | MQSeries MQGET |
| `mqseries_inq` | MQSeries MQINQ |
| `mqseries_open` | MQSeries MQOPEN |
| `mqseries_put` | MQSeries MQPUT |
| `mqseries_put1` | MQSeries MQPUT1 |
| `mqseries_set` | MQSeries MQSET |
| `mqseries_strerror` | Returns the error message corresponding to a result code (MQRC) |
| `msg_get_queue` | Create or attach to a message queue |
| `msg_queue_exists` | Check whether a message queue exists |
| `msg_receive` | Receive a message from a message queue |
| `msg_remove_queue` | Destroy a message queue |
| `msg_send` | Send a message to a message queue |
| `msg_set_queue` | Set information in the message queue data structure |
| `msg_stat_queue` | Returns information from the message queue data structure |
| `mt_getrandmax` | 显示随机数的最大可能值 |
| `mt_rand` | 通过梅森旋转（Mersenne Twister）随机数生成器生成随机值 |
| `mt_srand` | 播下一个更好的随机数发生器种子 |
| `mysql_affected_rows` | 取得前一次 MySQL 操作所影响的记录行数 |
| `mysql_client_encoding` | 返回字符集的名称 |
| `mysql_close` | 关闭 MySQL 连接 |
| `mysql_connect` | 打开一个到 MySQL 服务器的连接 |
| `mysql_create_db` | 新建一个 MySQL 数据库 |
| `mysql_data_seek` | 移动内部结果的指针 |
| `mysql_db_name` | 取得 mysql_list_dbs 返回的结果数据 |
| `mysql_db_query` | 选择数据库并执行查询 |
| `mysql_drop_db` | 丢弃（删除）一个 MySQL 数据库 |
| `mysql_errno` | 返回上一个 MySQL 操作中的错误信息的数值 |
| `mysql_error` | 返回上一个 MySQL 操作中的错误信息的文本 |
| `mysql_escape_string` | 转义字符串用于 mysql_query |
| `mysql_fetch_array` | 从结果集中取得一行作为关联数组 |
| `mysql_fetch_assoc` | 从结果集中取得一行作为关联数组 |
| `mysql_fetch_field` | 从结果集中取得列信息并作为对象返回 |
| `mysql_fetch_lengths` | 取得结果集中每个输出的长度 |
| `mysql_fetch_object` | 从结果集中取得一行作为对象返回 |
| `mysql_fetch_row` | 从结果集中取得一行作为枚举数组 |
| `mysql_field_flags` | 从结果中取得和指定字段关联的 flags |
| `mysql_field_len` | 返回指定字段的长度 |
| `mysql_field_name` | 取得结果中指定字段的字段名 |
| `mysql_field_seek` | 将结果指针设置为指定的字段偏移量 |
| `mysql_field_table` | 取得指定字段所在的表名 |
| `mysql_field_type` | 取得结果集中指定字段的类型 |
| `mysql_free_result` | 释放结果内存 |
| `mysql_get_client_info` | 取得 MySQL 客户端信息 |
| `mysql_get_host_info` | 取得 MySQL 主机信息 |
| `mysql_get_proto_info` | 取得 MySQL 协议信息 |
| `mysql_get_server_info` | 取得 MySQL 服务器信息 |
| `mysql_info` | 获取最近查询的有关信息 |
| `mysql_insert_id` | 取得上一条查询生成的 ID |
| `mysql_list_dbs` | 列出 MySQL 服务器中可用的数据库 |
| `mysql_list_fields` | 列出 MySQL 表字段 |
| `mysql_list_processes` | 列出 MySQL 进程 |
| `mysql_list_tables` | 列出 MySQL 数据库中的表 |
| `mysql_num_fields` | 取得结果中字段的数量 |
| `mysql_num_rows` | 获取结果中行数 |
| `mysql_pconnect` | 打开一个到 MySQL 服务器的持久连接 |
| `mysql_ping` | Ping 服务器连接的状态，如果没有连接则重新连接 |
| `mysql_query` | 发送一条 MySQL 查询 |
| `mysql_real_escape_string` | 将字符串中的特殊字符进行转义，以在 SQL 语句中使用 |
| `mysql_result` | 取得结果数据 |
| `mysql_select_db` | 选择 MySQL 数据库 |
| `mysql_set_charset` | 设置客户端的字符集 |
| `mysql_stat` | 获取当前系统状态 |
| `mysql_tablename` | 取得表名 |
| `mysql_thread_id` | 返回当前线程的 ID |
| `mysql_unbuffered_query` | 向 MySQL 发送 SQL 查询，无需获取和缓冲结果行 |
| `mysqli_connect` | 别名 mysqli::__construct |
| `mysqli_execute` | 别名 mysqli_stmt_execute |
| `mysqli_get_client_stats` | 返回客户端进程统计信息 |
| `mysqli_get_links_stats` | 返回打开和缓存的链接相关信息 |
| `mysqli_report` | 别名 mysqli_driver-&gt;report_mode |
| `mysqli::escape_string` | 别名 mysqli_real_escape_string |
| `mysqli::set_opt` | 别名 mysqli_options |

### N

| 函数名 | 功能描述 |
|--------|----------|
| `natcasesort` | 用&ldquo;自然排序&rdquo;算法对数组进行不区分大小写字母的排序 |
| `natsort` | 用&ldquo;自然排序&rdquo;算法对数组排序 |
| `net_get_interfaces` | 获取网络接口 |
| `next` | 将数组中的内部指针向前移动一位 |
| `ngettext` | Plural version of gettext |
| `nl_langinfo` | Query language and locale information |
| `nl2br` | 在字符串所有新行之前插入 HTML 换行标记 |
| `number_format` | 以千位分隔符方式格式化一个数字 |

### O

| 函数名 | 功能描述 |
|--------|----------|
| `oauth_get_sbs` | 生成一个签名字符基串 |
| `oauth_urlencode` | 将 URI 编码为 RFC 3986 规范 |
| `ob_clean` | 清空（擦掉）活动输出缓冲区的内容 |
| `ob_end_clean` | 清空（擦除）活动缓冲区的内容并关闭它 |
| `ob_end_flush` | 冲刷（发送）活动输出处理程序的返回值，并关闭活动输出缓冲区 |
| `ob_flush` | 冲刷（发送）活动输出处理程序的返回值 |
| `ob_get_clean` | 获取活动缓冲区的内容并将其关闭 |
| `ob_get_contents` | 返回输出缓冲区的内容 |
| `ob_get_flush` | 冲刷（发送）活动输出处理程序的返回值，返回活动输出缓冲区的内容并将其关闭 |
| `ob_get_length` | 返回输出缓冲区内容的长度 |
| `ob_get_level` | 返回输出缓冲机制的嵌套级别 |
| `ob_get_status` | 得到所有输出缓冲区的状态 |
| `ob_gzhandler` | ob_start 回调函数压缩输出缓冲区 |
| `ob_iconv_handler` | 以输出缓冲处理程序转换字符编码 |
| `ob_implicit_flush` | 打开/关闭绝对刷送 |
| `ob_list_handlers` | 列出所有使用的输出处理程序 |
| `ob_start` | 打开输出控制缓冲 |
| `ob_tidyhandler` | ob_start callback function to repair the buffer |
| `oci_bind_array_by_name` | Binds a PHP array to an Oracle PL/SQL array parameter |
| `oci_bind_by_name` | 绑定 PHP 变量到 Oracle 位置标志符 |
| `oci_cancel` | 中断游标读取数据 |
| `oci_client_version` | Returns the Oracle client library version |
| `oci_close` | 关闭 Oracle 连接 |
| `oci_commit` | 提交未完成的数据库事务 |
| `oci_connect` | 建立一个到 Oracle 服务器的连接 |
| `oci_define_by_name` | 将 PHP 变量与查询读取的列相关联 |
| `oci_error` | 返回最后发现的错误 |
| `oci_execute` | 执行语句 |
| `oci_fetch` | 从查询中读取下一行到内部缓冲区 |
| `oci_fetch_all` | 从查询中读取多行到二维数组中 |
| `oci_fetch_array` | Returns the next row from a query as an associative or numeric array |
| `oci_fetch_assoc` | Returns the next row from a query as an associative array |
| `oci_fetch_object` | Returns the next row from a query as an object |
| `oci_fetch_row` | Returns the next row from a query as a numeric array |
| `oci_field_is_null` | 检测当前获取的行中，字段是否为 null |
| `oci_field_name` | 返回 statement 中的字段名 |
| `oci_field_precision` | 返回字段精度 |
| `oci_field_scale` | 返回字段范围 |
| `oci_field_size` | 返回字段大小 |
| `oci_field_type` | 返回字段的数据类型名称 |
| `oci_field_type_raw` | 返回字段的原始 Oracle 数据类型 |
| `oci_free_descriptor` | Frees a descriptor |
| `oci_free_statement` | 释放关联于语句或游标的所有资源 |
| `oci_get_implicit_resultset` | Returns the next child statement resource from a parent statement resource that has Oracle Database Implicit Result Sets |
| `oci_internal_debug` | Enables or disables internal debug output |
| `oci_lob_copy` | Copies large object |
| `oci_lob_is_equal` | Compares two LOB/FILE locators for equality |
| `oci_new_collection` | 分配新的 collection 对象 |
| `oci_new_connect` | 使用唯一连接，连接到 Oracle 服务器 |
| `oci_new_cursor` | 分配并返回新的游标（语句句柄） |
| `oci_new_descriptor` | 初始化新的空 LOB 或 FILE 描述符 |
| `oci_num_fields` | 返回语句中结果列的数量 |
| `oci_num_rows` | 返回语句执行后受影响的行数 |
| `oci_parse` | 预处理用于执行的 Oracle 语句 |
| `oci_password_change` | 修改 Oracle 用户的密码 |
| `oci_pconnect` | 使用持久连接连，连接到 Oracle 数据库 |
| `oci_register_taf_callback` | Register a user-defined callback function for Oracle Database TAF |
| `oci_result` | 返回所取得行中字段的值 |
| `oci_rollback` | 回滚未完成的事务 |
| `oci_server_version` | 返回 Oracle 数据库版本 |
| `oci_set_action` | Sets the action name |
| `oci_set_call_timeout` | Sets a millisecond timeout for database calls |
| `oci_set_client_identifier` | Sets the client identifier |
| `oci_set_client_info` | Sets the client information |
| `oci_set_db_operation` | Sets the database operation |
| `oci_set_edition` | Sets the database edition |
| `oci_set_module_name` | Sets the module name |
| `oci_set_prefetch` | 设置查询预读取的行数 |
| `oci_set_prefetch_lob` | Sets the amount of data prefetched for each CLOB or BLOB. |
| `oci_statement_type` | 返回语句的类型 |
| `oci_unregister_taf_callback` | Unregister a user-defined callback function for Oracle Database TAF |
| `ocibindbyname` | 别名 oci_bind_by_name |
| `ocicancel` | 别名 oci_cancel |
| `ocicloselob` | 别名 OCILob::close |
| `ocicollappend` | 别名 OCICollection::append |
| `ocicollassign` | 别名 OCICollection::assign |
| `ocicollassignelem` | 别名 OCICollection::assignElem |
| `ocicollgetelem` | 别名 OCICollection::getElem |
| `ocicollmax` | 别名 OCICollection::max |
| `ocicollsize` | 别名 OCICollection::size |
| `ocicolltrim` | 别名 OCICollection::trim |
| `ocicolumnisnull` | 别名 oci_field_is_null |
| `ocicolumnname` | 别名 oci_field_name |
| `ocicolumnprecision` | 别名 oci_field_precision |
| `ocicolumnscale` | 别名 oci_field_scale |
| `ocicolumnsize` | 别名 oci_field_size |
| `ocicolumntype` | 别名 oci_field_type |
| `ocicolumntyperaw` | 别名 oci_field_type_raw |
| `ocicommit` | 别名 oci_commit |
| `ocidefinebyname` | 别名 oci_define_by_name |
| `ocierror` | 别名 oci_error |
| `ociexecute` | 别名 oci_execute |
| `ocifetch` | 别名 oci_fetch |
| `ocifetchstatement` | 别名 oci_fetch_all |
| `ocifreecollection` | 别名 OCICollection::free |
| `ocifreecursor` | 别名 oci_free_statement |
| `ocifreedesc` | 别名 OCILob::free |
| `ocifreestatement` | 别名 oci_free_statement |
| `ociinternaldebug` | 别名 oci_internal_debug |
| `ociloadlob` | 别名 OCILob::load |
| `ocilogoff` | 别名 oci_close |
| `ocilogon` | 别名 oci_connect |
| `ocinewcollection` | 别名 oci_new_collection |
| `ocinewcursor` | 别名 oci_new_cursor |
| `ocinewdescriptor` | 别名 oci_new_descriptor |
| `ocinlogon` | 别名 oci_new_connect |
| `ocinumcols` | 别名 oci_num_fields |
| `ociparse` | 别名 oci_parse |
| `ociplogon` | 别名 oci_pconnect |
| `ociresult` | 别名 oci_result |
| `ocirollback` | 别名 oci_rollback |
| `ocirowcount` | 别名 oci_num_rows |
| `ocisavelob` | 别名 OCILob::save |
| `ocisavelobfile` | 别名 OCILob::import |
| `ociserverversion` | 别名 oci_server_version |
| `ocisetprefetch` | 别名 oci_set_prefetch |
| `ocistatementtype` | 别名 oci_statement_type |
| `ociwritelobtofile` | 别名 OCILob::export |
| `ociwritetemporarylob` | 别名 OCILob::writeTemporary |
| `octdec` | 八进制转换为十进制 |
| `odbc_autocommit` | Toggle autocommit behaviour |
| `odbc_binmode` | Handling of binary column data |
| `odbc_close` | Close an ODBC connection |
| `odbc_close_all` | Close all ODBC connections |
| `odbc_columnprivileges` | Lists columns and associated privileges for the given table |
| `odbc_columns` | Lists the column names in specified tables |
| `odbc_commit` | Commit an ODBC transaction |
| `odbc_connect` | Connect to a datasource |
| `odbc_connection_string_is_quoted` | Determines if an ODBC connection string value is quoted |
| `odbc_connection_string_quote` | Quotes an ODBC connection string value |
| `odbc_connection_string_should_quote` | Determines if an ODBC connection string value should be quoted |
| `odbc_cursor` | Get cursorname |
| `odbc_data_source` | Returns information about available DSNs |
| `odbc_do` | 别名 odbc_exec |
| `odbc_error` | Get the last error code |
| `odbc_errormsg` | Get the last error message |
| `odbc_exec` | Directly execute an SQL statement |
| `odbc_execute` | Execute a prepared statement |
| `odbc_fetch_array` | Fetch a result row as an associative array |
| `odbc_fetch_into` | Fetch one result row into array |
| `odbc_fetch_object` | Fetch a result row as an object |
| `odbc_fetch_row` | Fetch a row |
| `odbc_field_len` | Get the length (precision) of a field |
| `odbc_field_name` | Get the columnname |
| `odbc_field_num` | Return column number |
| `odbc_field_precision` | 别名 odbc_field_len |
| `odbc_field_scale` | Get the scale of a field |
| `odbc_field_type` | Datatype of a field |
| `odbc_foreignkeys` | Retrieves a list of foreign keys |
| `odbc_free_result` | Free objects associated with a result |
| `odbc_gettypeinfo` | Retrieves information about data types supported by the data source |
| `odbc_longreadlen` | Handling of LONG columns |
| `odbc_next_result` | Checks if multiple results are available |
| `odbc_num_fields` | Number of columns in a result |
| `odbc_num_rows` | Number of rows in a result |
| `odbc_pconnect` | Open a persistent database connection |
| `odbc_prepare` | Prepares a statement for execution |
| `odbc_primarykeys` | Gets the primary keys for a table |
| `odbc_procedurecolumns` | Retrieve information about parameters to procedures |
| `odbc_procedures` | Get the list of procedures stored in a specific data source |
| `odbc_result` | Get result data |
| `odbc_result_all` | Print result as HTML table |
| `odbc_rollback` | Rollback a transaction |
| `odbc_setoption` | Adjust ODBC settings |
| `odbc_specialcolumns` | Retrieves special columns |
| `odbc_statistics` | Retrieve statistics about a table |
| `odbc_tableprivileges` | Lists tables and the privileges associated with each table |
| `odbc_tables` | Get the list of table names stored in a specific data source |
| `opcache_compile_file` | 无需运行，即可编译并缓存 PHP 脚本 |
| `opcache_get_configuration` | 获取缓存的配置信息 |
| `opcache_get_status` | 获取缓存的状态信息 |
| `opcache_invalidate` | 废除脚本缓存 |
| `opcache_is_script_cached` | 用于判断脚本是否已缓存在 OPCache 中 |
| `opcache_is_script_cached_in_file_cache` | 用于判断脚本是否已缓存在 OPCache 文件缓存中 |
| `opcache_jit_blacklist` | 将某个函数加入黑名单，使其不参与 JIT 编译 |
| `opcache_reset` | 重置字节码缓存的内容 |
| `openal_buffer_create` | Generate OpenAL buffer |
| `openal_buffer_data` | Load a buffer with data |
| `openal_buffer_destroy` | Destroys an OpenAL buffer |
| `openal_buffer_get` | Retrieve an OpenAL buffer property |
| `openal_buffer_loadwav` | Load a .wav file into a buffer |
| `openal_context_create` | Create an audio processing context |
| `openal_context_current` | Make the specified context current |
| `openal_context_destroy` | Destroys a context |
| `openal_context_process` | Process the specified context |
| `openal_context_suspend` | Suspend the specified context |
| `openal_device_close` | Close an OpenAL device |
| `openal_device_open` | Initialize the OpenAL audio layer |
| `openal_listener_get` | Retrieve a listener property |
| `openal_listener_set` | Set a listener property |
| `openal_source_create` | Generate a source resource |
| `openal_source_destroy` | Destroy a source resource |
| `openal_source_get` | Retrieve an OpenAL source property |
| `openal_source_pause` | Pause the source |
| `openal_source_play` | Start playing the source |
| `openal_source_rewind` | Rewind the source |
| `openal_source_set` | Set source property |
| `openal_source_stop` | Stop playing the source |
| `openal_stream` | Begin streaming on a source |
| `opendir` | 打开目录句柄 |
| `openlog` | Open connection to system logger |
| `openssl_cipher_iv_length` | 获取密码iv长度 |
| `openssl_cipher_key_length` | Gets the cipher key length |
| `openssl_cms_decrypt` | Decrypt a CMS message |
| `openssl_cms_encrypt` | Encrypt a CMS message |
| `openssl_cms_read` | Export the CMS file to an array of PEM certificates |
| `openssl_cms_sign` | Sign a file |
| `openssl_cms_verify` | Verify a CMS signature |
| `openssl_csr_export` | 将 CSR 作为字符串导出 |
| `openssl_csr_export_to_file` | 将 CSR 导出到文件 |
| `openssl_csr_get_public_key` | 返回 CSR 的公钥 |
| `openssl_csr_get_subject` | 返回 CSR 的主题 |
| `openssl_csr_new` | 生成一个 CSR |
| `openssl_csr_sign` | 用另一个证书签署 CSR（或者本身）并且生成一个证书 |
| `openssl_decrypt` | 解密数据 |
| `openssl_dh_compute_key` | 计算远程 DH 公钥和本地 DH 密钥的共享密钥 |
| `openssl_digest` | 计算摘要 |
| `openssl_encrypt` | 加密数据 |
| `openssl_error_string` | 返回 openSSL 错误消息 |
| `openssl_free_key` | 释放密钥资源 |
| `openssl_get_cert_locations` | 检索可用的证书位置 |
| `openssl_get_cipher_methods` | 获取可用的加密算法 |
| `openssl_get_curve_names` | 获得ECC的可用曲线名称列表 |
| `openssl_get_md_methods` | 获取可用的摘要算法 |
| `openssl_get_privatekey` | 别名 openssl_pkey_get_private |
| `openssl_get_publickey` | 别名 openssl_pkey_get_public |
| `openssl_open` | 打开密封的数据 |
| `openssl_password_hash` | Create a password hash using OpenSSL's Argon2 implementation |
| `openssl_password_verify` | Verify a password against a hash using OpenSSL's Argon2 implementation |
| `openssl_pbkdf2` | 生成 PKCS5 v2 PBKDF2 字符串 |
| `openssl_pkcs12_export` | 将 PKCS#12 兼容证书存储文件导出到变量 |
| `openssl_pkcs12_export_to_file` | 输出一个 PKCS#12 兼容的证书存储文件 |
| `openssl_pkcs12_read` | 将 PKCS#12 证书存储区解析到数组中 |
| `openssl_pkcs7_decrypt` | 解密一个 S/MIME 加密的消息 |
| `openssl_pkcs7_encrypt` | 加密一个 S/MIME 消息 |
| `openssl_pkcs7_read` | 将 PKCS7 文件导出为 PEM 格式证书的数组 |
| `openssl_pkcs7_sign` | 对一个 S/MIME 消息进行签名 |
| `openssl_pkcs7_verify` | 校验一个已签名的 S/MIME 消息的签名 |
| `openssl_pkey_derive` | Computes shared secret for public value of remote and local DH or ECDH key |
| `openssl_pkey_export` | 将一个密钥的可输出表示转换为字符串 |
| `openssl_pkey_export_to_file` | 将密钥导出到文件中 |
| `openssl_pkey_free` | 释放一个私钥 |
| `openssl_pkey_get_details` | 返回包含密钥详情的数组 |
| `openssl_pkey_get_private` | 获取私钥 |
| `openssl_pkey_get_public` | 从证书中解析公钥，以供使用 |
| `openssl_pkey_new` | 生成新的私钥 |
| `openssl_private_decrypt` | 使用私钥解密数据 |
| `openssl_private_encrypt` | 使用私钥加密数据 |
| `openssl_public_decrypt` | 使用公钥解密数据 |
| `openssl_public_encrypt` | 使用公钥加密数据 |
| `openssl_random_pseudo_bytes` | 生成一个伪随机字节串 |
| `openssl_seal` | 密封 (加密) 数据 |
| `openssl_sign` | Generate signature |
| `openssl_spki_export` | 通过签名公钥和 challenge 导出一个可用的 PEM 格式的公钥 |
| `openssl_spki_export_challenge` | 导出与签名公钥和 challenge 相关的 challenge |
| `openssl_spki_new` | 生成新的签名公钥和 challenge |
| `openssl_spki_verify` | 验证签名公钥和 challenge |
| `openssl_verify` | 验证签名 |
| `openssl_x509_check_private_key` | 检查私钥是否对应于证书 |
| `openssl_x509_checkpurpose` | 验证是否可以为特定目的使用证书 |
| `openssl_x509_export` | 以字符串格式导出证书 |
| `openssl_x509_export_to_file` | 导出证书至文件 |
| `openssl_x509_fingerprint` | 计算一个给定的 x.509 证书的指纹或摘要 |
| `openssl_x509_free` | 释放证书资源 |
| `openssl_x509_parse` | 解析 X509 证书并作为一个数组返回信息 |
| `openssl_x509_read` | 解析 x.509 证书并返回对象 |
| `openssl_x509_verify` | Verifies digital signature of x509 certificate against a public key |
| `ord` | 转换字符串第一个字节为 0-255 之间的值 |
| `output_add_rewrite_var` | 添加 URL 重写器的值 |
| `output_reset_rewrite_vars` | 重设 URL 重写器的值 |

### P

| 函数名 | 功能描述 |
|--------|----------|
| `pack` | 将数据打包成二进制字符串 |
| `parse_ini_file` | 解析一个配置文件 |
| `parse_ini_string` | 解析配置字符串 |
| `parse_str` | 解析 URL 查询字符串 |
| `parse_url` | 解析 URL，返回其组成部分 |
| `passthru` | 执行外部程序并且显示原始输出 |
| `password_algos` | Get available password hashing algorithm IDs |
| `password_get_info` | 返回指定散列（hash）的相关信息 |
| `password_hash` | 创建密码的散列（hash） |
| `password_needs_rehash` | 检测散列值是否匹配指定的选项 |
| `password_verify` | 验证密码是否和散列值匹配 |
| `pathinfo` | 返回文件路径的信息 |
| `pclose` | 关闭进程文件指针 |
| `pcntl_alarm` | 为进程设置 alarm 闹钟信号 |
| `pcntl_async_signals` | Enable/disable asynchronous signal handling or return the old setting |
| `pcntl_errno` | 别名 pcntl_get_last_error |
| `pcntl_exec` | 在当前进程空间执行指定程序 |
| `pcntl_fork` | 在当前进程当前位置产生分叉（fork） |
| `pcntl_get_last_error` | Retrieve the error number set by the last pcntl function which failed |
| `pcntl_getcpuaffinity` | Get the cpu affinity of a process |
| `pcntl_getpriority` | 获取任意进程的优先级 |
| `pcntl_getqos_class` | Get the QoS class of the current thread |
| `pcntl_rfork` | Manipulates process resources |
| `pcntl_setcpuaffinity` | Set the cpu affinity of a process |
| `pcntl_setpriority` | 修改任意进程的优先级 |
| `pcntl_setqos_class` | Set the QoS class of the current thread |
| `pcntl_signal` | 安装信号处理程序 |
| `pcntl_signal_dispatch` | 调用等待信号的处理程序 |
| `pcntl_signal_get_handler` | Get the current handler for specified signal |
| `pcntl_sigprocmask` | 设置或检索阻塞信号 |
| `pcntl_sigtimedwait` | 带超时机制的信号等待 |
| `pcntl_sigwaitinfo` | 等待信号 |
| `pcntl_strerror` | Retrieve the system error message associated with the given errno |
| `pcntl_unshare` | Dissociates parts of the process execution context |
| `pcntl_wait` | 等待或返回 fork 的子进程状态 |
| `pcntl_waitid` | 等待子进程改变状态 |
| `pcntl_waitpid` | 等待或返回 fork 的子进程状态 |
| `pcntl_wexitstatus` | 返回一个中断的子进程的返回代码 |
| `pcntl_wifexited` | 检查状态代码是否代表一个正常的退出 |
| `pcntl_wifsignaled` | 检查子进程状态码是否代表由于某个信号而中断 |
| `pcntl_wifstopped` | 检查子进程当前是否已经停止 |
| `pcntl_wstopsig` | 返回导致子进程停止的信号 |
| `pcntl_wtermsig` | 返回导致子进程中断的信号 |
| `pfsockopen` | 打开持久的 Internet 或 Unix 套接字连接 |
| `pg_affected_rows` | 返回受影响的记录数（元组） |
| `pg_cancel_query` | 取消异步查询 |
| `pg_client_encoding` | 获取客户端编码 |
| `pg_close` | 关闭 PostgreSQL 连接 |
| `pg_connect` | 打开 PostgreSQL 连接 |
| `pg_connect_poll` | 对正在进行尝试进行异步的 PostgreSQL 连接轮询其状态。 |
| `pg_connection_busy` | 获取连接是否繁忙 |
| `pg_connection_reset` | 重置连接（再次连接） |
| `pg_connection_status` | 获取连接状态 |
| `pg_consume_input` | Reads input on the connection |
| `pg_convert` | 将关联的数组值转换为适合 SQL 语句的格式 |
| `pg_copy_from` | 将数组中的记录插入到表 |
| `pg_copy_to` | 将表复制到数组 |
| `pg_dbname` | 获取数据库名称 |
| `pg_delete` | 删除记录 |
| `pg_end_copy` | 与 PostgreSQL 后端同步 |
| `pg_escape_bytea` | 转义字符串以插入到 bytea 字段 |
| `pg_escape_identifier` | Escape an identifier for insertion into a text field |
| `pg_escape_literal` | Escape a literal for insertion into a text field |
| `pg_escape_string` | 转义字符串以供查询 |
| `pg_execute` | Sends a request to execute a prepared statement with given parameters, and waits for the result |
| `pg_fetch_all` | 从结果中获取所有行作为数组 |
| `pg_fetch_all_columns` | Fetches all rows in a particular result column as an array |
| `pg_fetch_array` | 获取一行作为数组 |
| `pg_fetch_assoc` | 获取一行作为关联数组 |
| `pg_fetch_object` | 获取一行作为对象 |
| `pg_fetch_result` | 从结果实例返回值 |
| `pg_fetch_row` | 提取一行作为枚举数组 |
| `pg_field_is_null` | 测试字段是否为 SQL NULL |
| `pg_field_name` | 返回字段名 |
| `pg_field_num` | 返回名为 field 的字段编号 |
| `pg_field_prtlen` | 返回打印的长度 |
| `pg_field_size` | 返回指定字段的内部存储大小 |
| `pg_field_table` | Returns the name or oid of the tables field |
| `pg_field_type` | 返回相应字段编号的类型名称 |
| `pg_field_type_oid` | Returns the type ID (OID) for the corresponding field number |
| `pg_flush` | 刷新链接中已处理的数据查询 |
| `pg_free_result` | 释放查询结果占用的内存 |
| `pg_get_notify` | 获取 SQL NOTIFY 消息 |
| `pg_get_pid` | 获取后端的进程 ID |
| `pg_get_result` | 取得异步查询结果 |
| `pg_host` | 返回和某连接关联的主机名 |
| `pg_insert` | 将数组插入到表中 |
| `pg_last_error` | 得到某连接的最后一条错误信息 |
| `pg_last_notice` | 返回 PostgreSQL 服务器最新一条公告信息 |
| `pg_last_oid` | 返回上一条记录的 oid |
| `pg_lo_close` | 关闭大对象 |
| `pg_lo_create` | 新建大对象 |
| `pg_lo_export` | 将大型对象导出到文件 |
| `pg_lo_import` | 将文件导入为大型对象 |
| `pg_lo_open` | 打开大对象 |
| `pg_lo_read` | 读取大对象 |
| `pg_lo_read_all` | 读取整个大对象并直接发送到浏览器 |
| `pg_lo_seek` | 在大对象中寻找位置 |
| `pg_lo_tell` | 返回当前大型对象的指针位置 |
| `pg_lo_truncate` | Truncates a large object |
| `pg_lo_unlink` | 删除一个大型对象 |
| `pg_lo_write` | 向大对象写入数据 |
| `pg_meta_data` | 获得表的元数据 |
| `pg_num_fields` | 返回结果中字段的数量 |
| `pg_num_rows` | 返回结果中行的数量 |
| `pg_options` | 获得和 connection 相关的选项 |
| `pg_parameter_status` | Looks up a current parameter setting of the server |
| `pg_pconnect` | 打开一个持久的 PostgreSQL 连接 |
| `pg_ping` | Ping 数据库连接 |
| `pg_port` | 返回 connection 相关的端口号 |
| `pg_put_line` | 向 PostgreSQL 后端发送以 NULL 字符结尾的字符串 |
| `pg_query` | 执行查询 |
| `pg_query_params` | Submits a command to the server and waits for the result, with the ability to pass parameters separately from the SQL command text |
| `pg_result_error` | 获得跟 result 相关的错误信息 |
| `pg_result_error_field` | Returns an individual field of an error report |
| `pg_result_memory_size` | Returns the amount of memory allocated for a query result |
| `pg_result_seek` | 在 result 实例中设定内部行偏移量 |
| `pg_result_status` | 获得查询结果的状态 |
| `pg_select` | 选择记录 |
| `pg_send_execute` | Sends a request to execute a prepared statement with given parameters, without waiting for the result(s) |
| `pg_send_prepare` | Sends a request to create a prepared statement with the given parameters, without waiting for completion |
| `pg_send_query` | 发送异步查询 |
| `pg_send_query_params` | Submits a command and separate parameters to the server without waiting for the result(s) |
| `pg_set_chunked_rows_size` | Set the query results to be retrieved in chunk mode |
| `pg_set_client_encoding` | 设定客户端编码 |
| `pg_socket` | Get a read only handle to the socket underlying a PostgreSQL connection |
| `pg_trace` | 启用 PostgreSQL 连接的追踪 |
| `pg_transaction_status` | Returns the current in-transaction status of the server |
| `pg_tty` | 返回跟连接相关的 tty 名 |
| `pg_unescape_bytea` | 反转义 bytea 类型的二进制数据 |
| `pg_untrace` | 禁用 PostgreSQL 连接的追踪 |
| `pg_update` | 更新表 |
| `pg_version` | Returns an array with client, protocol and server version (when available) |
| `php_ini_loaded_file` | 取得已加载的 php.ini 文件的路径 |
| `php_ini_scanned_files` | 返回从额外 ini 目录里解析的 .ini 文件列表 |
| `php_sapi_name` | 返回 web 服务器和 PHP 之间的接口类型 |
| `php_strip_whitespace` | 返回删除注释和空格后的PHP源码 |
| `php_uname` | 返回运行 PHP 的系统的有关信息 |
| `phpcredits` | 打印 PHP 贡献者名单 |
| `phpdbg_break_file` | Inserts a breakpoint at a line in a file |
| `phpdbg_break_function` | Inserts a breakpoint at entry to a function |
| `phpdbg_break_method` | Inserts a breakpoint at entry to a method |
| `phpdbg_break_next` | Inserts a breakpoint at the next opcode |
| `phpdbg_clear` | Clears all breakpoints |
| `phpdbg_color` | Sets the color of certain elements |
| `phpdbg_end_oplog` | Ends an oplog |
| `phpdbg_exec` | Attempts to set the execution context |
| `phpdbg_get_executable` | Gets executable |
| `phpdbg_prompt` | Sets the command prompt |
| `phpdbg_start_oplog` | Starts an oplog |
| `phpinfo` | 输出关于 PHP 配置的信息 |
| `phpversion` | 获取当前的PHP版本 |
| `pi` | 得到圆周率值 |
| `png2wbmp` | 将 PNG 图像文件转换为 WBMP 图像文件 |
| `popen` | 打开进程文件指针 |
| `pos` | current 的别名 |
| `posix_access` | Determine accessibility of a file |
| `posix_ctermid` | Get path name of controlling terminal |
| `posix_eaccess` | Determine accessibility of a file |
| `posix_errno` | 别名 posix_get_last_error |
| `posix_fpathconf` | Returns the value of a configurable limit |
| `posix_get_last_error` | Retrieve the error number set by the last posix function that failed |
| `posix_getcwd` | Pathname of current directory |
| `posix_getegid` | Return the effective group ID of the current process |
| `posix_geteuid` | Return the effective user ID of the current process |
| `posix_getgid` | Return the real group ID of the current process |
| `posix_getgrgid` | Return info about a group by group id |
| `posix_getgrnam` | Return info about a group by name |
| `posix_getgroups` | Return the group set of the current process |
| `posix_getlogin` | Return login name |
| `posix_getpgid` | Get process group id for job control |
| `posix_getpgrp` | Return the current process group identifier |
| `posix_getpid` | 返回当前进程 id |
| `posix_getppid` | Return the parent process identifier |
| `posix_getpwnam` | Return info about a user by username |
| `posix_getpwuid` | Return info about a user by user id |
| `posix_getrlimit` | Return info about system resource limits |
| `posix_getsid` | Get the current sid of the process |
| `posix_getuid` | Return the real user ID of the current process |
| `posix_initgroups` | Calculate the group access list |
| `posix_isatty` | Determine if a file descriptor is an interactive terminal |
| `posix_kill` | Send a signal to a process |
| `posix_mkfifo` | Create a fifo special file (a named pipe) |
| `posix_mknod` | Create a special or ordinary file (POSIX.1) |
| `posix_pathconf` | Returns the value of a configurable limit |
| `posix_setegid` | Set the effective GID of the current process |
| `posix_seteuid` | Set the effective UID of the current process |
| `posix_setgid` | Set the GID of the current process |
| `posix_setpgid` | Set process group id for job control |
| `posix_setrlimit` | Set system resource limits |
| `posix_setsid` | Make the current process a session leader |
| `posix_setuid` | Set the UID of the current process |
| `posix_strerror` | Retrieve the system error message associated with the given errno |
| `posix_sysconf` | Returns system runtime information |
| `posix_times` | Get process times |
| `posix_ttyname` | Determine terminal device name |
| `posix_uname` | Get system name |
| `pow` | 指数表达式 |
| `preg_filter` | 执行一个正则表达式搜索和替换 |
| `preg_grep` | 返回匹配模式的数组条目 |
| `preg_last_error` | 返回最后一个PCRE正则执行产生的错误代码 |
| `preg_last_error_msg` | Returns the error message of the last PCRE regex execution |
| `preg_match` | 执行匹配正则表达式 |
| `preg_match_all` | 执行一个全局正则表达式匹配 |
| `preg_quote` | 转义正则表达式字符 |
| `preg_replace` | 执行一个正则表达式的搜索和替换 |
| `preg_replace_callback` | 执行一个正则表达式搜索并且使用一个回调进行替换 |
| `preg_replace_callback_array` | Perform a regular expression search and replace using callbacks |
| `preg_split` | 通过一个正则表达式分隔字符串 |
| `prev` | 将数组的内部指针倒回一位 |
| `print` | 输出字符串 |
| `print_r` | 打印人类可读的变量信息 |
| `printf` | 输出格式化字符串 |
| `proc_close` | 关闭由 proc_open 打开的进程并且返回进程退出码 |
| `proc_get_status` | 获取由 proc_open 函数打开的进程的信息 |
| `proc_nice` | 修改当前进程的优先级 |
| `proc_open` | 执行一个命令，并且打开用来输入/输出的文件指针。 |
| `proc_terminate` | 杀死由 proc_open 打开的进程 |
| `property_exists` | 检查对象或类是否具有该属性 |
| `ps_add_bookmark` | Add bookmark to current page |
| `ps_add_launchlink` | Adds link which launches file |
| `ps_add_locallink` | Adds link to a page in the same document |
| `ps_add_note` | Adds note to current page |
| `ps_add_pdflink` | Adds link to a page in a second pdf document |
| `ps_add_weblink` | Adds link to a web location |
| `ps_arc` | Draws an arc counterclockwise |
| `ps_arcn` | Draws an arc clockwise |
| `ps_begin_page` | Start a new page |
| `ps_begin_pattern` | Start a new pattern |
| `ps_begin_template` | Start a new template |
| `ps_circle` | Draws a circle |
| `ps_clip` | Clips drawing to current path |
| `ps_close` | Closes a PostScript document |
| `ps_close_image` | Closes image and frees memory |
| `ps_closepath` | Closes path |
| `ps_closepath_stroke` | Closes and strokes path |
| `ps_continue_text` | Continue text in next line |
| `ps_curveto` | Draws a curve |
| `ps_delete` | Deletes all resources of a PostScript document |
| `ps_end_page` | End a page |
| `ps_end_pattern` | End a pattern |
| `ps_end_template` | End a template |
| `ps_fill` | Fills the current path |
| `ps_fill_stroke` | Fills and strokes the current path |
| `ps_findfont` | Loads a font |
| `ps_get_buffer` | Fetches the full buffer containig the generated PS data |
| `ps_get_parameter` | Gets certain parameters |
| `ps_get_value` | Gets certain values |
| `ps_hyphenate` | Hyphenates a word |
| `ps_include_file` | Reads an external file with raw PostScript code |
| `ps_lineto` | Draws a line |
| `ps_makespotcolor` | Create spot color |
| `ps_moveto` | Sets current point |
| `ps_new` | Creates a new PostScript document object |
| `ps_open_file` | Opens a file for output |
| `ps_open_image` | Reads an image for later placement |
| `ps_open_image_file` | Opens image from file |
| `ps_open_memory_image` | Takes an GD image and returns an image for placement in a PS document |
| `ps_place_image` | Places image on the page |
| `ps_rect` | Draws a rectangle |
| `ps_restore` | Restore previously save context |
| `ps_rotate` | Sets rotation factor |
| `ps_save` | Save current context |
| `ps_scale` | Sets scaling factor |
| `ps_set_border_color` | Sets color of border for annotations |
| `ps_set_border_dash` | Sets length of dashes for border of annotations |
| `ps_set_border_style` | Sets border style of annotations |
| `ps_set_info` | Sets information fields of document |
| `ps_set_parameter` | Sets certain parameters |
| `ps_set_text_pos` | Sets position for text output |
| `ps_set_value` | Sets certain values |
| `ps_setcolor` | Sets current color |
| `ps_setdash` | Sets appearance of a dashed line |
| `ps_setflat` | Sets flatness |
| `ps_setfont` | Sets font to use for following output |
| `ps_setgray` | Sets gray value |
| `ps_setlinecap` | Sets appearance of line ends |
| `ps_setlinejoin` | Sets how contected lines are joined |
| `ps_setlinewidth` | Sets width of a line |
| `ps_setmiterlimit` | Sets the miter limit |
| `ps_setoverprintmode` | Sets overprint mode |
| `ps_setpolydash` | Sets appearance of a dashed line |
| `ps_shading` | Creates a shading for later use |
| `ps_shading_pattern` | Creates a pattern based on a shading |
| `ps_shfill` | Fills an area with a shading |
| `ps_show` | Output text |
| `ps_show_boxed` | Output text in a box |
| `ps_show_xy` | Output text at given position |
| `ps_show_xy2` | Output text at position |
| `ps_show2` | Output a text at current position |
| `ps_string_geometry` | Gets geometry of a string |
| `ps_stringwidth` | Gets width of a string |
| `ps_stroke` | Draws the current path |
| `ps_symbol` | Output a glyph |
| `ps_symbol_name` | Gets name of a glyph |
| `ps_symbol_width` | Gets width of a glyph |
| `ps_translate` | Sets translation |
| `pspell_add_to_personal` | Add the word to a personal wordlist |
| `pspell_add_to_session` | Add the word to the wordlist in the current session |
| `pspell_check` | Check a word |
| `pspell_clear_session` | Clear the current session |
| `pspell_config_create` | Create a config used to open a dictionary |
| `pspell_config_data_dir` | Location of language data files |
| `pspell_config_dict_dir` | Location of the main word list |
| `pspell_config_ignore` | Ignore words less than N characters long |
| `pspell_config_mode` | Change the mode number of suggestions returned |
| `pspell_config_personal` | Set a file that contains personal wordlist |
| `pspell_config_repl` | Set a file that contains replacement pairs |
| `pspell_config_runtogether` | Consider run-together words as valid compounds |
| `pspell_new` | Load a new dictionary |
| `pspell_new_config` | Load a new dictionary with settings based on a given config |
| `pspell_new_personal` | Load a new dictionary with personal wordlist |
| `pspell_save_wordlist` | Save the personal wordlist to a file |
| `pspell_store_replacement` | Store a replacement pair for a word |
| `pspell_suggest` | Suggest spellings of a word |
| `putenv` | 设置环境变量的值 |

### Q

| 函数名 | 功能描述 |
|--------|----------|
| `quoted_printable_decode` | 将 quoted-printable 字符串转换为 8-bit 字符串 |
| `quoted_printable_encode` | 将 8-bit 字符串转换成 quoted-printable 字符串 |
| `quotemeta` | 转义元字符集 |

### R

| 函数名 | 功能描述 |
|--------|----------|
| `rad2deg` | 将弧度数转换为相应的角度数 |
| `radius_acct_open` | Creates a Radius handle for accounting |
| `radius_add_server` | Adds a server |
| `radius_auth_open` | Creates a Radius handle for authentication |
| `radius_close` | Frees all ressources |
| `radius_config` | Causes the library to read the given configuration file |
| `radius_create_request` | Create accounting or authentication request |
| `radius_cvt_addr` | Converts raw data to IP-Address |
| `radius_cvt_int` | Converts raw data to integer |
| `radius_cvt_string` | Converts raw data to string |
| `radius_demangle` | Demangles data |
| `radius_demangle_mppe_key` | Derives mppe-keys from mangled data |
| `radius_get_attr` | Extracts an attribute |
| `radius_get_tagged_attr_data` | Extracts the data from a tagged attribute |
| `radius_get_tagged_attr_tag` | Extracts the tag from a tagged attribute |
| `radius_get_vendor_attr` | Extracts a vendor specific attribute |
| `radius_put_addr` | Attaches an IP address attribute |
| `radius_put_attr` | Attaches a binary attribute |
| `radius_put_int` | Attaches an integer attribute |
| `radius_put_string` | Attaches a string attribute |
| `radius_put_vendor_addr` | Attaches a vendor specific IP address attribute |
| `radius_put_vendor_attr` | Attaches a vendor specific binary attribute |
| `radius_put_vendor_int` | Attaches a vendor specific integer attribute |
| `radius_put_vendor_string` | Attaches a vendor specific string attribute |
| `radius_request_authenticator` | Returns the request authenticator |
| `radius_salt_encrypt_attr` | Salt-encrypts a value |
| `radius_send_request` | Sends the request and waits for a reply |
| `radius_server_secret` | Returns the shared secret |
| `radius_strerror` | Returns an error message |
| `rand` | 产生一个随机整数 |
| `random_bytes` | Get cryptographically secure random bytes |
| `random_int` | 获取生成加密安全、均匀分布的整数 |
| `range` | 根据范围创建数组，包含指定的元素 |
| `rar_wrapper_cache_stats` | Cache hits and misses for the URL wrapper |
| `rawurldecode` | 对已编码的 URL 字符串进行解码 |
| `rawurlencode` | 按照 RFC 3986 对 URL 进行编码 |
| `read_exif_data` | 别名 exif_read_data |
| `readdir` | 从目录句柄中读取条目 |
| `readfile` | 输出文件 |
| `readgzfile` | Output a gz-file |
| `readline` | 读取一行 |
| `readline_add_history` | 添加一行到历史 |
| `readline_callback_handler_install` | 初始化 readline 回调接口和终端，然后打印提示并立即返回 |
| `readline_callback_handler_remove` | 移除之前已安装的回调函数句柄并且恢复终端设置 |
| `readline_callback_read_char` | 当一个行被接收时读取一个字符并且通知 readline 回调接口 |
| `readline_clear_history` | 清除历史 |
| `readline_completion_function` | 注册完成函数 |
| `readline_info` | 获取/设置各种 readline 内部变量 |
| `readline_list_history` | 获取历史 |
| `readline_on_new_line` | 通知 readline 将光标移动到新行 |
| `readline_read_history` | 读取历史 |
| `readline_redisplay` | 重绘显示区 |
| `readline_write_history` | 写入历史记录 |
| `readlink` | 返回符号连接指向的目标 |
| `realpath` | 返回规范化的绝对路径名 |
| `realpath_cache_get` | 获取真实目录缓存的详情 |
| `realpath_cache_size` | 获取真实路径缓冲区的大小 |
| `recode` | 别名 recode_string |
| `recode_file` | Recode from file to file according to recode request |
| `recode_string` | Recode a string according to a recode request |
| `register_shutdown_function` | 注册在关闭时执行的函数 |
| `register_tick_function` | 注册一个函数以便在每个 tick 上执行 |
| `rename` | 重命名一个文件或目录 |
| `request_parse_body` | Read and parse the request body and return the result |
| `reset` | 将数组的内部指针指向第一个单元 |
| `restore_error_handler` | 还原之前的错误处理函数 |
| `restore_exception_handler` | 恢复之前定义过的异常处理函数。 |
| `restore_include_path` | 还原 include_path 配置选项的值 |
| `rewind` | 倒回文件指针的位置 |
| `rewinddir` | 倒回目录句柄 |
| `rmdir` | 删除目录 |
| `rnp_backend_string` | Return cryptographic backend library name |
| `rnp_backend_version` | Return cryptographic backend library version |
| `rnp_decrypt` | Decrypt PGP message |
| `rnp_dump_packets` | Dump OpenPGP packets stream information in humand-readable format |
| `rnp_dump_packets_to_json` | Dump OpenPGP packets stream information to the JSON string |
| `rnp_ffi_create` | Create the top-level object used for interacting with the library |
| `rnp_ffi_destroy` | Destroy the top-level object used for interacting with the library |
| `rnp_ffi_set_pass_provider` | Set password provider callback function |
| `rnp_import_keys` | Import keys from PHP string to the keyring and receive JSON describing new/updated keys |
| `rnp_import_signatures` | Import standalone signatures to the keyring and receive JSON describing updated keys |
| `rnp_key_export` | Export a key |
| `rnp_key_export_revocation` | Generate and export primary key revocation signature |
| `rnp_key_get_info` | Get information about the key |
| `rnp_key_remove` | Remove a key from keyring(s) |
| `rnp_key_revoke` | Revoke a key or subkey by generating and adding revocation signature |
| `rnp_list_keys` | Enumerate all keys present in a keyring by specified identifer type |
| `rnp_load_keys` | Load keys from PHP string |
| `rnp_load_keys_from_path` | Load keys from specified path |
| `rnp_locate_key` | Search for the key |
| `rnp_op_encrypt` | Encrypt message |
| `rnp_op_generate_key` | Generate key |
| `rnp_op_sign` | Perform signing operation on a binary data, return embedded signature(s) |
| `rnp_op_sign_cleartext` | Perform signing operation on a textual data, return cleartext signed message |
| `rnp_op_sign_detached` | Perform signing operation, return detached signature(s) |
| `rnp_op_verify` | Verify embedded or cleartext signatures |
| `rnp_op_verify_detached` | Verify detached signatures |
| `rnp_save_keys` | Save keys to PHP string |
| `rnp_save_keys_to_path` | Save keys to specified path |
| `rnp_supported_features` | Get supported features in JSON format |
| `rnp_version_string` | RNP library version |
| `rnp_version_string_full` | Full version string of RNP library |
| `round` | 对浮点数进行四舍五入 |
| `rpmaddtag` | Add tag retrieved in query |
| `rpmdbinfo` | Get information from installed RPM |
| `rpmdbsearch` | Search RPM packages |
| `rpmdefine` | Define or change a RPM macro value |
| `rpmexpand` | Retrieve expanded value of a RPM macro |
| `rpmexpandnumeric` | Retrieve numerical value of a RPM macro |
| `rpmgetsymlink` | Get target of a symlink |
| `rpminfo` | Get information from a RPM file |
| `rpmvercmp` | RPM version comparison |
| `rrd_create` | 创建 rrd 数据库文件 |
| `rrd_error` | 获取最新的错误信息 |
| `rrd_fetch` | 获取图表数据数组 |
| `rrd_first` | 从 rrd 文件中获取第一个样本的时间戳 |
| `rrd_graph` | 从数据创建图像 |
| `rrd_info` | 获取 rrd 文件有关信息 |
| `rrd_last` | 获取最后一个样本的 Unix 时间戳 |
| `rrd_lastupdate` | 获取有关上次更新数据的信息 |
| `rrd_restore` | 从 XML 转储中恢复 RRD 文件 |
| `rrd_tune` | 调整 RRD 数据库文件头选项 |
| `rrd_update` | 更新 RRD 数据库 |
| `rrd_version` | 获取底层 rrdtool 库的相关信息 |
| `rrd_xport` | 导出 RRD 数据库的相关信息 |
| `rrdc_disconnect` | 关闭所有未完成的 rrd 缓存守护进程连接 |
| `rsort` | 对数组降序排序 |
| `rtrim` | 去除字符串末尾的空白字符（或者其他字符） |
| `runkit7_constant_add` | Similar to define(), but allows defining in class definitions as well |
| `runkit7_constant_redefine` | Redefine an already defined constant |
| `runkit7_constant_remove` | Remove/Delete an already defined constant |
| `runkit7_function_add` | Add a new function, similar to create_function |
| `runkit7_function_copy` | Copy a function to a new function name |
| `runkit7_function_redefine` | Replace a function definition with a new implementation |
| `runkit7_function_remove` | Remove a function definition |
| `runkit7_function_rename` | Change a function's name |
| `runkit7_import` | Process a PHP file importing function and class definitions, overwriting where appropriate |
| `runkit7_method_add` | Dynamically adds a new method to a given class |
| `runkit7_method_copy` | Copies a method from class to another |
| `runkit7_method_redefine` | Dynamically changes the code of the given method |
| `runkit7_method_remove` | Dynamically removes the given method |
| `runkit7_method_rename` | Dynamically changes the name of the given method |
| `runkit7_object_id` | Return the integer object handle for given object |
| `runkit7_superglobals` | Return numerically indexed array of registered superglobals |
| `runkit7_zval_inspect` | Returns information about the passed in value with data types, reference counts, etc |

### S

| 函数名 | 功能描述 |
|--------|----------|
| `sapi_windows_cp_conv` | Convert string from one codepage to another |
| `sapi_windows_cp_get` | Get current codepage |
| `sapi_windows_cp_is_utf8` | Indicates whether the codepage is UTF-8 compatible |
| `sapi_windows_cp_set` | Set process codepage |
| `sapi_windows_generate_ctrl_event` | Send a CTRL event to another process |
| `sapi_windows_set_ctrl_handler` | Set or remove a CTRL event handler |
| `sapi_windows_vt100_support` | Get or set VT100 support for the specified stream associated to an output buffer of a Windows console. |
| `scandir` | 列出指定路径中的文件和目录 |
| `scoutapm_get_calls` | Returns a list of instrumented calls that have occurred |
| `scoutapm_list_instrumented_functions` | List functions scoutapm will instrument. |
| `seaslog_get_author` | 获取 SeasLog 作者 |
| `seaslog_get_version` | 获取 SeasLog 的版本 |
| `sem_acquire` | Acquire a semaphore |
| `sem_get` | Get a semaphore id |
| `sem_release` | Release a semaphore |
| `sem_remove` | Remove a semaphore |
| `serialize` | 生成值的可存储表示 |
| `session_abort` | Discard session array changes and finish session |
| `session_cache_expire` | 返回/设置当前缓存的到期时间 |
| `session_cache_limiter` | 读取/设置缓存限制器 |
| `session_commit` | session_write_close 的别名 |
| `session_create_id` | Create new session id |
| `session_decode` | 解码会话数据 |
| `session_destroy` | 销毁一个会话中的全部数据 |
| `session_encode` | 将当前会话数据编码为字符串 |
| `session_gc` | Perform session data garbage collection |
| `session_get_cookie_params` | 获取会话 cookie 参数 |
| `session_id` | 获取/设置当前会话 ID |
| `session_module_name` | 获取/设置会话模块名称 |
| `session_name` | 读取/设置会话名称 |
| `session_regenerate_id` | 使用新生成的会话 ID 更新现有会话 ID |
| `session_register_shutdown` | 关闭会话 |
| `session_reset` | Re-initialize session array with original values |
| `session_save_path` | 读取/设置当前会话的保存路径 |
| `session_set_cookie_params` | 设置会话 cookie 参数 |
| `session_set_save_handler` | 设置用户自定义会话存储函数 |
| `session_start` | 启动新会话或者重用现有会话 |
| `session_status` | 返回当前会话状态 |
| `session_unset` | 释放所有的会话变量 |
| `session_write_close` | Write session data and end session |
| `set_error_handler` | 设置用户自定义的错误处理函数 |
| `set_exception_handler` | 设置用户自定义的异常处理函数 |
| `set_file_buffer` | stream_set_write_buffer 的别名 |
| `set_include_path` | 设置 include_path 配置选项 |
| `set_time_limit` | 设置脚本最大执行时间 |
| `setcookie` | 发送 Cookie |
| `setlocale` | 设置区域信息 |
| `setrawcookie` | 发送未经 URL 编码的 cookie |
| `settype` | 设置变量的类型 |
| `sha1` | 计算字符串的 sha1 散列值 |
| `sha1_file` | 计算文件的 sha1 散列值 |
| `shell_exec` | 通过 shell 执行命令并将完整的输出以字符串的方式返回 |
| `shm_attach` | Creates or open a shared memory segment |
| `shm_detach` | Disconnects from shared memory segment |
| `shm_get_var` | Returns a variable from shared memory |
| `shm_has_var` | Check whether a specific entry exists |
| `shm_put_var` | Inserts or updates a variable in shared memory |
| `shm_remove` | Removes shared memory from Unix systems |
| `shm_remove_var` | Removes a variable from shared memory |
| `shmop_close` | Close shared memory block |
| `shmop_delete` | Delete shared memory block |
| `shmop_open` | Create or open shared memory block |
| `shmop_read` | Read data from shared memory block |
| `shmop_size` | Get size of shared memory block |
| `shmop_write` | Write data into shared memory block |
| `show_source` | 别名 highlight_file |
| `shuffle` | 打乱数组 |
| `simdjson_decode` | Decodes a JSON string |
| `simdjson_is_valid` | Check if a JSON string is valid |
| `simdjson_key_count` | Returns the value at a JSON pointer. |
| `simdjson_key_exists` | Check if the JSON contains the value referred to by a JSON pointer. |
| `simdjson_key_value` | Decodes the value of a JSON string located at the requested JSON pointer. |
| `similar_text` | 计算两个字符串的相似度 |
| `simplexml_import_dom` | Get a SimpleXMLElement object from an XML or HTML node |
| `simplexml_load_file` | Interprets an XML file into an object |
| `simplexml_load_string` | Interprets a string of XML into an object |
| `sin` | 正弦 |
| `sinh` | 双曲正弦 |
| `sizeof` | count 的别名 |
| `sleep` | 延缓执行 |
| `snmp_get_quick_print` | 获取当前 NET-SNMP 库的 quick_print 设置的值 |
| `snmp_get_valueretrieval` | Return the method how the SNMP values will be returned |
| `snmp_read_mib` | Reads and parses a MIB file into the active MIB tree |
| `snmp_set_enum_print` | Return all values that are enums with their enum value instead of the raw integer |
| `snmp_set_oid_numeric_print` | 别名 snmp_set_oid_output_format |
| `snmp_set_oid_output_format` | Set the OID output format |
| `snmp_set_quick_print` | 在 NET-SNMP 库中设置 enable 的值 |
| `snmp_set_valueretrieval` | Specify the method how the SNMP values will be returned |
| `snmp2_get` | Fetch an SNMP object |
| `snmp2_getnext` | Fetch the SNMP object which follows the given object id |
| `snmp2_real_walk` | Return all objects including their respective object ID within the specified one |
| `snmp2_set` | Set the value of an SNMP object |
| `snmp2_walk` | Fetch all the SNMP objects from an agent |
| `snmp3_get` | Fetch an SNMP object |
| `snmp3_getnext` | Fetch the SNMP object which follows the given object id |
| `snmp3_real_walk` | Return all objects including their respective object ID within the specified one |
| `snmp3_set` | Set the value of an SNMP object |
| `snmp3_walk` | Fetch all the SNMP objects from an agent |
| `snmpget` | 获取 SNMP 对象 |
| `snmpgetnext` | Fetch the SNMP object which follows the given object id |
| `snmprealwalk` | 返回指定的所有对象，包括它们各自的对象 ID |
| `snmpset` | 设置 SNMP 对象的值 |
| `snmpwalk` | 从代理获取所有 SNMP 对象 |
| `snmpwalkoid` | 查询有关网络实体的信息树 |
| `socket_accept` | 接受套接字上的连接 |
| `socket_addrinfo_bind` | 从给定的 addrinfo 创建并绑定一个套接字 |
| `socket_addrinfo_connect` | 指定 addrinfo 创建并连接套接字 |
| `socket_addrinfo_explain` | 获取有关 addrinfo 的信息 |
| `socket_addrinfo_lookup` | 获取数组，包含有关给定主机名的 getaddrinfo 内容 |
| `socket_atmark` | 确认 socket 是否处于带外数据标记 |
| `socket_bind` | 给套接字绑定名字 |
| `socket_clear_error` | 清除套接字或者最后的错误代码上的错误 |
| `socket_close` | 关闭 Socket 实例 |
| `socket_cmsg_space` | Calculate message buffer size |
| `socket_connect` | 开启一个套接字连接 |
| `socket_create` | 创建一个套接字（通讯节点） |
| `socket_create_listen` | 在端口上打开一个套接字以接受连接 |
| `socket_create_pair` | 创建一对彼此连接的套接字，并用数组存储 |
| `socket_export_stream` | Export a socket into a stream that encapsulates a socket |
| `socket_get_option` | 获取套接字的套接字选项 |
| `socket_get_status` | 别名 stream_get_meta_data |
| `socket_getopt` | 别名 socket_get_option |
| `socket_getpeername` | 获取套接字远端名字 |
| `socket_getsockname` | 获取套接字本地端的名字，返回主机名和端口号或是 Unix 文件系统路径，具体取决于套接字类型 |
| `socket_import_stream` | 导入 stream |
| `socket_last_error` | 返回套接字上的最后一个错误 |
| `socket_listen` | 监听套接字的连接 |
| `socket_read` | 从套接字中读取最大长度的数据 |
| `socket_recv` | 从已连接的 socket 接收数据 |
| `socket_recvfrom` | 从套接字接收数据，无论它是否是面向连接的 |
| `socket_recvmsg` | Read a message |
| `socket_select` | 从给定套接字数组运行带指定超时时间的 select() 系统调用 |
| `socket_send` | 向已连接的套接字发送数据 |
| `socket_sendmsg` | Send a message |
| `socket_sendto` | 向套接字发送消息，无论它是否已建立连接 |
| `socket_set_block` | 设置套接字为阻塞模式 |
| `socket_set_blocking` | 别名 stream_set_blocking |
| `socket_set_nonblock` | 设置套接字为非阻塞模式 |
| `socket_set_option` | 为套接字设置套接字选项 |
| `socket_set_timeout` | 别名 stream_set_timeout |
| `socket_setopt` | 别名 socket_set_option |
| `socket_shutdown` | 关闭套接字接收或发送，或两者都关闭 |
| `socket_strerror` | 返回描述套接字错误的字符串 |
| `socket_write` | 向套接字写数据 |
| `socket_wsaprotocol_info_export` | 导出 WSAPROTOCOL_INFO 结构体 |
| `socket_wsaprotocol_info_import` | 从另一个进程导入套接字 |
| `socket_wsaprotocol_info_release` | 释放已导出的 WSAPROTOCOL_INFO 结构体 |
| `sodium_add` | Add large numbers |
| `sodium_base642bin` | Decodes a base64-encoded string into raw binary. |
| `sodium_bin2base64` | Encodes a raw binary string with base64. |
| `sodium_bin2hex` | Encode to hexadecimal |
| `sodium_compare` | Compare large numbers |
| `sodium_crypto_aead_aegis128l_decrypt` | Verify then decrypt a message with AEGIS-128L |
| `sodium_crypto_aead_aegis128l_encrypt` | Encrypt then authenticate a message with AEGIS-128L |
| `sodium_crypto_aead_aegis128l_keygen` | Generate a random AEGIS-128L key |
| `sodium_crypto_aead_aegis256_decrypt` | Verify then decrypt a message with AEGIS-256 |
| `sodium_crypto_aead_aegis256_encrypt` | Encrypt then authenticate a message with AEGIS-256 |
| `sodium_crypto_aead_aegis256_keygen` | Generate a random AEGIS-256 key |
| `sodium_crypto_aead_aes256gcm_decrypt` | Verify then decrypt a message with AES-256-GCM |
| `sodium_crypto_aead_aes256gcm_encrypt` | Encrypt then authenticate with AES-256-GCM |
| `sodium_crypto_aead_aes256gcm_is_available` | Check if hardware supports AES256-GCM |
| `sodium_crypto_aead_aes256gcm_keygen` | Generate a random AES-256-GCM key |
| `sodium_crypto_aead_chacha20poly1305_decrypt` | Verify then decrypt with ChaCha20-Poly1305 |
| `sodium_crypto_aead_chacha20poly1305_encrypt` | Encrypt then authenticate with ChaCha20-Poly1305 |
| `sodium_crypto_aead_chacha20poly1305_ietf_decrypt` | Verify that the ciphertext includes a valid tag |
| `sodium_crypto_aead_chacha20poly1305_ietf_encrypt` | Encrypt a message |
| `sodium_crypto_aead_chacha20poly1305_ietf_keygen` | Generate a random ChaCha20-Poly1305 (IETF) key. |
| `sodium_crypto_aead_chacha20poly1305_keygen` | Generate a random ChaCha20-Poly1305 key. |
| `sodium_crypto_aead_xchacha20poly1305_ietf_decrypt` | (Preferred) Verify then decrypt with XChaCha20-Poly1305 |
| `sodium_crypto_aead_xchacha20poly1305_ietf_encrypt` | (Preferred) Encrypt then authenticate with XChaCha20-Poly1305 |
| `sodium_crypto_aead_xchacha20poly1305_ietf_keygen` | Generate a random XChaCha20-Poly1305 key. |
| `sodium_crypto_auth` | Compute a tag for the message |
| `sodium_crypto_auth_keygen` | Generate a random key for sodium_crypto_auth |
| `sodium_crypto_auth_verify` | Verifies that the tag is valid for the message |
| `sodium_crypto_box` | Authenticated public-key encryption |
| `sodium_crypto_box_keypair` | Randomly generate a secret key and a corresponding public key |
| `sodium_crypto_box_keypair_from_secretkey_and_publickey` | Create a unified keypair string from a secret key and public key |
| `sodium_crypto_box_open` | Authenticated public-key decryption |
| `sodium_crypto_box_publickey` | Extract the public key from a crypto_box keypair |
| `sodium_crypto_box_publickey_from_secretkey` | Calculate the public key from a secret key |
| `sodium_crypto_box_seal` | Anonymous public-key encryption |
| `sodium_crypto_box_seal_open` | Anonymous public-key decryption |
| `sodium_crypto_box_secretkey` | Extracts the secret key from a crypto_box keypair |
| `sodium_crypto_box_seed_keypair` | Deterministically derive the key pair from a single key |
| `sodium_crypto_core_ristretto255_add` | Adds an element |
| `sodium_crypto_core_ristretto255_from_hash` | Maps a vector |
| `sodium_crypto_core_ristretto255_is_valid_point` | Determines if a point on the ristretto255 curve |
| `sodium_crypto_core_ristretto255_random` | Generates a random key |
| `sodium_crypto_core_ristretto255_scalar_add` | Adds a scalar value |
| `sodium_crypto_core_ristretto255_scalar_complement` | The sodium_crypto_core_ristretto255_scalar_complement purpose |
| `sodium_crypto_core_ristretto255_scalar_invert` | Inverts a scalar value |
| `sodium_crypto_core_ristretto255_scalar_mul` | Multiplies a scalar value |
| `sodium_crypto_core_ristretto255_scalar_negate` | Negates a scalar value |
| `sodium_crypto_core_ristretto255_scalar_random` | Generates a random key |
| `sodium_crypto_core_ristretto255_scalar_reduce` | Reduces a scalar value |
| `sodium_crypto_core_ristretto255_scalar_sub` | Subtracts a scalar value |
| `sodium_crypto_core_ristretto255_sub` | Subtracts an element |
| `sodium_crypto_generichash` | Get a hash of the message |
| `sodium_crypto_generichash_final` | Complete the hash |
| `sodium_crypto_generichash_init` | Initialize a hash for streaming |
| `sodium_crypto_generichash_keygen` | Generate a random generichash key |
| `sodium_crypto_generichash_update` | Add message to a hash |
| `sodium_crypto_kdf_derive_from_key` | Derive a subkey |
| `sodium_crypto_kdf_keygen` | Generate a random root key for the KDF interface |
| `sodium_crypto_kx_client_session_keys` | Calculate the client-side session keys. |
| `sodium_crypto_kx_keypair` | Creates a new sodium keypair |
| `sodium_crypto_kx_publickey` | Extract the public key from a crypto_kx keypair |
| `sodium_crypto_kx_secretkey` | Extract the secret key from a crypto_kx keypair. |
| `sodium_crypto_kx_seed_keypair` | Description |
| `sodium_crypto_kx_server_session_keys` | Calculate the server-side session keys. |
| `sodium_crypto_pwhash` | Derive a key from a password, using Argon2 |
| `sodium_crypto_pwhash_scryptsalsa208sha256` | Derives a key from a password, using scrypt |
| `sodium_crypto_pwhash_scryptsalsa208sha256_str` | Get an ASCII encoded hash |
| `sodium_crypto_pwhash_scryptsalsa208sha256_str_verify` | Verify that the password is a valid password verification string |
| `sodium_crypto_pwhash_str` | Get an ASCII-encoded hash |
| `sodium_crypto_pwhash_str_needs_rehash` | Determine whether or not to rehash a password |
| `sodium_crypto_pwhash_str_verify` | Verifies that a password matches a hash |
| `sodium_crypto_scalarmult` | Compute a shared secret given a user's secret key and another user's public key |
| `sodium_crypto_scalarmult_base` | 别名 sodium_crypto_box_publickey_from_secretkey |
| `sodium_crypto_scalarmult_ristretto255` | Computes a shared secret |
| `sodium_crypto_scalarmult_ristretto255_base` | Calculates the public key from a secret key |
| `sodium_crypto_secretbox` | Authenticated shared-key encryption |
| `sodium_crypto_secretbox_keygen` | Generate random key for sodium_crypto_secretbox |
| `sodium_crypto_secretbox_open` | Authenticated shared-key decryption |
| `sodium_crypto_secretstream_xchacha20poly1305_init_pull` | Initialize a secretstream context for decryption |
| `sodium_crypto_secretstream_xchacha20poly1305_init_push` | Initialize a secretstream context for encryption |
| `sodium_crypto_secretstream_xchacha20poly1305_keygen` | Generate a random secretstream key. |
| `sodium_crypto_secretstream_xchacha20poly1305_pull` | Decrypt a chunk of data from an encrypted stream |
| `sodium_crypto_secretstream_xchacha20poly1305_push` | Encrypt a chunk of data so that it can safely be decrypted in a streaming API |
| `sodium_crypto_secretstream_xchacha20poly1305_rekey` | Explicitly rotate the key in the secretstream state |
| `sodium_crypto_shorthash` | Compute a short hash of a message and key |
| `sodium_crypto_shorthash_keygen` | Get random bytes for key |
| `sodium_crypto_sign` | Sign a message |
| `sodium_crypto_sign_detached` | Sign the message |
| `sodium_crypto_sign_ed25519_pk_to_curve25519` | Convert an Ed25519 public key to a Curve25519 public key |
| `sodium_crypto_sign_ed25519_sk_to_curve25519` | Convert an Ed25519 secret key to a Curve25519 secret key |
| `sodium_crypto_sign_keypair` | Randomly generate a secret key and a corresponding public key |
| `sodium_crypto_sign_keypair_from_secretkey_and_publickey` | Join a secret key and public key together |
| `sodium_crypto_sign_open` | Check that the signed message has a valid signature |
| `sodium_crypto_sign_publickey` | Extract the Ed25519 public key from a keypair |
| `sodium_crypto_sign_publickey_from_secretkey` | Extract the Ed25519 public key from the secret key |
| `sodium_crypto_sign_secretkey` | Extract the Ed25519 secret key from a keypair |
| `sodium_crypto_sign_seed_keypair` | Deterministically derive the key pair from a single key |
| `sodium_crypto_sign_verify_detached` | Verify signature for the message |
| `sodium_crypto_stream` | Generate a deterministic sequence of bytes from a seed |
| `sodium_crypto_stream_keygen` | Generate a random sodium_crypto_stream key. |
| `sodium_crypto_stream_xchacha20` | Expands the key and nonce into a keystream of pseudorandom bytes |
| `sodium_crypto_stream_xchacha20_keygen` | Returns a secure random key |
| `sodium_crypto_stream_xchacha20_xor` | Encrypts a message using a nonce and a secret key (no authentication) |
| `sodium_crypto_stream_xchacha20_xor_ic` | Encrypts a message using a nonce and a secret key (no authentication) |
| `sodium_crypto_stream_xor` | Encrypt a message without authentication |
| `sodium_hex2bin` | Decodes a hexadecimally encoded binary string |
| `sodium_increment` | Increment large number |
| `sodium_memcmp` | Test for equality in constant-time |
| `sodium_memzero` | Overwrite a string with NUL characters |
| `sodium_pad` | Add padding data |
| `sodium_unpad` | Remove padding data |
| `solr_get_version` | 返回当前Solr扩展的版本 |
| `sort` | 对数组升序排序 |
| `soundex` | Calculate the soundex key of a string |
| `spl_autoload` | __autoload() 函数的默认实现 |
| `spl_autoload_call` | 尝试所有已注册的 __autoload() 函数来装载请求类 |
| `spl_autoload_extensions` | 注册并返回 spl_autoload 的默认文件扩展名 |
| `spl_autoload_functions` | 返回所有已注册的 __autoload() 函数 |
| `spl_autoload_register` | 注册指定的函数作为 __autoload 的实现 |
| `spl_autoload_unregister` | 注销已实现的 __autoload() 函数 |
| `spl_classes` | 返回所有可用的SPL类 |
| `spl_object_hash` | 返回指定对象的 hash id |
| `spl_object_id` | Return the integer object handle for given object |
| `sprintf` | 返回格式化字符串 |
| `sqlsrv_begin_transaction` | Begins a database transaction |
| `sqlsrv_cancel` | Cancels a statement |
| `sqlsrv_client_info` | Returns information about the client and specified connection |
| `sqlsrv_close` | Closes an open connection and releases resourses associated with the connection |
| `sqlsrv_commit` | Commits a transaction that was begun with sqlsrv_begin_transaction |
| `sqlsrv_configure` | Changes the driver error handling and logging configurations |
| `sqlsrv_connect` | Opens a connection to a Microsoft SQL Server database |
| `sqlsrv_errors` | Returns error and warning information about the last SQLSRV operation performed |
| `sqlsrv_execute` | Executes a statement prepared with sqlsrv_prepare |
| `sqlsrv_fetch` | Makes the next row in a result set available for reading |
| `sqlsrv_fetch_array` | Returns a row as an array |
| `sqlsrv_fetch_object` | Retrieves the next row of data in a result set as an object |
| `sqlsrv_free_stmt` | Frees all resources for the specified statement |
| `sqlsrv_get_config` | Returns the value of the specified configuration setting |
| `sqlsrv_get_field` | Gets field data from the currently selected row |
| `sqlsrv_has_rows` | Indicates whether the specified statement has rows |
| `sqlsrv_next_result` | Makes the next result of the specified statement active |
| `sqlsrv_num_fields` | Retrieves the number of fields (columns) on a statement |
| `sqlsrv_num_rows` | Retrieves the number of rows in a result set |
| `sqlsrv_prepare` | Prepares a query for execution |
| `sqlsrv_query` | Prepares and executes a query |
| `sqlsrv_send_stream_data` | Sends data from parameter streams to the server |
| `sqlsrv_server_info` | Returns information about the server |
| `sqrt` | 平方根 |
| `srand` | 播下随机数发生器种子 |
| `sscanf` | 根据指定格式解析输入的字符 |
| `ssdeep_fuzzy_compare` | Calculates the match score between two fuzzy hash signatures |
| `ssdeep_fuzzy_hash` | Create a fuzzy hash from a string |
| `ssdeep_fuzzy_hash_filename` | Create a fuzzy hash from a file |
| `ssh2_auth_agent` | Authenticate over SSH using the ssh agent |
| `ssh2_auth_hostbased_file` | Authenticate using a public hostkey |
| `ssh2_auth_none` | Authenticate as &quot;none&quot; |
| `ssh2_auth_password` | Authenticate over SSH using a plain password |
| `ssh2_auth_pubkey` | Authenticate using a public key in a variable |
| `ssh2_auth_pubkey_file` | Authenticate using a public key read from a file |
| `ssh2_connect` | Connect to an SSH server |
| `ssh2_disconnect` | Close a connection to a remote SSH server |
| `ssh2_exec` | Execute a command on a remote server |
| `ssh2_fetch_stream` | Fetch an extended data stream |
| `ssh2_fingerprint` | Retrieve fingerprint of remote server |
| `ssh2_forward_accept` | Accept a connection created by a listener |
| `ssh2_forward_listen` | Bind a port on the remote server and listen for connections |
| `ssh2_methods_negotiated` | Return list of negotiated methods |
| `ssh2_poll` | Poll the channels/listeners/streams for events |
| `ssh2_publickey_add` | Add an authorized publickey |
| `ssh2_publickey_init` | Initialize Publickey subsystem |
| `ssh2_publickey_list` | List currently authorized publickeys |
| `ssh2_publickey_remove` | Remove an authorized publickey |
| `ssh2_scp_recv` | Request a file via SCP |
| `ssh2_scp_send` | Send a file via SCP |
| `ssh2_send_eof` | Send EOF to stream |
| `ssh2_sftp` | Initialize SFTP subsystem |
| `ssh2_sftp_chmod` | Changes file mode |
| `ssh2_sftp_lstat` | Stat a symbolic link |
| `ssh2_sftp_mkdir` | Create a directory |
| `ssh2_sftp_readlink` | Return the target of a symbolic link |
| `ssh2_sftp_realpath` | Resolve the realpath of a provided path string |
| `ssh2_sftp_rename` | Rename a remote file |
| `ssh2_sftp_rmdir` | Remove a directory |
| `ssh2_sftp_stat` | Stat a file on a remote filesystem |
| `ssh2_sftp_symlink` | Create a symlink |
| `ssh2_sftp_unlink` | Delete a file |
| `ssh2_shell` | Request an interactive shell |
| `ssh2_tunnel` | Open a tunnel through a remote server |
| `stat` | 给出文件的信息 |
| `stats_absolute_deviation` | Returns the absolute deviation of an array of values |
| `stats_cdf_beta` | Calculates any one parameter of the beta distribution given values for the others |
| `stats_cdf_binomial` | Calculates any one parameter of the binomial distribution given values for the others |
| `stats_cdf_cauchy` | Calculates any one parameter of the Cauchy distribution given values for the others |
| `stats_cdf_chisquare` | Calculates any one parameter of the chi-square distribution given values for the others |
| `stats_cdf_exponential` | Calculates any one parameter of the exponential distribution given values for the others |
| `stats_cdf_f` | Calculates any one parameter of the F distribution given values for the others |
| `stats_cdf_gamma` | Calculates any one parameter of the gamma distribution given values for the others |
| `stats_cdf_laplace` | Calculates any one parameter of the Laplace distribution given values for the others |
| `stats_cdf_logistic` | Calculates any one parameter of the logistic distribution given values for the others |
| `stats_cdf_negative_binomial` | Calculates any one parameter of the negative binomial distribution given values for the others |
| `stats_cdf_noncentral_chisquare` | Calculates any one parameter of the non-central chi-square distribution given values for the others |
| `stats_cdf_noncentral_f` | Calculates any one parameter of the non-central F distribution given values for the others |
| `stats_cdf_noncentral_t` | Calculates any one parameter of the non-central t-distribution give values for the others |
| `stats_cdf_normal` | Calculates any one parameter of the normal distribution given values for the others |
| `stats_cdf_poisson` | Calculates any one parameter of the Poisson distribution given values for the others |
| `stats_cdf_t` | Calculates any one parameter of the t-distribution given values for the others |
| `stats_cdf_uniform` | Calculates any one parameter of the uniform distribution given values for the others |
| `stats_cdf_weibull` | Calculates any one parameter of the Weibull distribution given values for the others |
| `stats_covariance` | Computes the covariance of two data sets |
| `stats_dens_beta` | Probability density function of the beta distribution |
| `stats_dens_cauchy` | Probability density function of the Cauchy distribution |
| `stats_dens_chisquare` | Probability density function of the chi-square distribution |
| `stats_dens_exponential` | Probability density function of the exponential distribution |
| `stats_dens_f` | Probability density function of the F distribution |
| `stats_dens_gamma` | Probability density function of the gamma distribution |
| `stats_dens_laplace` | Probability density function of the Laplace distribution |
| `stats_dens_logistic` | Probability density function of the logistic distribution |
| `stats_dens_normal` | Probability density function of the normal distribution |
| `stats_dens_pmf_binomial` | Probability mass function of the binomial distribution |
| `stats_dens_pmf_hypergeometric` | Probability mass function of the hypergeometric distribution |
| `stats_dens_pmf_negative_binomial` | Probability mass function of the negative binomial distribution |
| `stats_dens_pmf_poisson` | Probability mass function of the Poisson distribution |
| `stats_dens_t` | Probability density function of the t-distribution |
| `stats_dens_uniform` | Probability density function of the uniform distribution |
| `stats_dens_weibull` | Probability density function of the Weibull distribution |
| `stats_harmonic_mean` | Returns the harmonic mean of an array of values |
| `stats_kurtosis` | Computes the kurtosis of the data in the array |
| `stats_rand_gen_beta` | Generates a random deviate from the beta distribution |
| `stats_rand_gen_chisquare` | Generates a random deviate from the chi-square distribution |
| `stats_rand_gen_exponential` | Generates a random deviate from the exponential distribution |
| `stats_rand_gen_f` | Generates a random deviate from the F distribution |
| `stats_rand_gen_funiform` | Generates uniform float between low (exclusive) and high (exclusive) |
| `stats_rand_gen_gamma` | Generates a random deviate from the gamma distribution |
| `stats_rand_gen_ibinomial` | Generates a random deviate from the binomial distribution |
| `stats_rand_gen_ibinomial_negative` | Generates a random deviate from the negative binomial distribution |
| `stats_rand_gen_int` | Generates random integer between 1 and 2147483562 |
| `stats_rand_gen_ipoisson` | Generates a single random deviate from a Poisson distribution |
| `stats_rand_gen_iuniform` | Generates integer uniformly distributed between LOW (inclusive) and HIGH (inclusive) |
| `stats_rand_gen_noncentral_chisquare` | Generates a random deviate from the non-central chi-square distribution |
| `stats_rand_gen_noncentral_f` | Generates a random deviate from the noncentral F distribution |
| `stats_rand_gen_noncentral_t` | Generates a single random deviate from a non-central t-distribution |
| `stats_rand_gen_normal` | Generates a single random deviate from a normal distribution |
| `stats_rand_gen_t` | Generates a single random deviate from a t-distribution |
| `stats_rand_get_seeds` | Get the seed values of the random number generator |
| `stats_rand_phrase_to_seeds` | Generate two seeds for the RGN random number generator |
| `stats_rand_ranf` | Generates a random floating point number between 0 and 1 |
| `stats_rand_setall` | Set seed values to the random generator |
| `stats_skew` | Computes the skewness of the data in the array |
| `stats_standard_deviation` | Returns the standard deviation |
| `stats_stat_binomial_coef` | Returns a binomial coefficient |
| `stats_stat_correlation` | Returns the Pearson correlation coefficient of two data sets |
| `stats_stat_factorial` | Returns the factorial of an integer |
| `stats_stat_independent_t` | Returns the t-value from the independent two-sample t-test |
| `stats_stat_innerproduct` | Returns the inner product of two vectors |
| `stats_stat_paired_t` | Returns the t-value of the dependent t-test for paired samples |
| `stats_stat_percentile` | Returns the percentile value |
| `stats_stat_powersum` | Returns the power sum of a vector |
| `stats_variance` | Returns the variance |
| `stomp_connect_error` | Returns a string description of the last connect error |
| `stomp_version` | Gets the current stomp extension version |
| `str_contains` | 确定字符串是否包含指定子串 |
| `str_decrement` | Decrement an alphanumeric string |
| `str_ends_with` | 检查字符串是否以指定子串结尾 |
| `str_getcsv` | 解析 CSV 字符串为一个数组 |
| `str_increment` | Increment an alphanumeric string |
| `str_ireplace` | str_replace 的忽略大小写版本 |
| `str_pad` | 使用另一个字符串填充字符串为指定长度 |
| `str_repeat` | 重复一个字符串 |
| `str_replace` | 子字符串替换 |
| `str_rot13` | 对字符串执行 ROT13 转换 |
| `str_shuffle` | 随机打乱一个字符串 |
| `str_split` | 将字符串转换为数组 |
| `str_starts_with` | 检查字符串是否以指定子串开头 |
| `str_word_count` | 返回字符串中单词的使用情况 |
| `strcasecmp` | 二进制安全比较字符串（不区分大小写） |
| `strchr` | 别名 strstr |
| `strcmp` | 二进制安全字符串比较 |
| `strcoll` | 基于区域设置的字符串比较 |
| `strcspn` | 获取不匹配遮罩的起始子字符串的长度 |
| `stream_bucket_append` | Append bucket to brigade |
| `stream_bucket_make_writeable` | Returns a bucket object from the brigade to operate on |
| `stream_bucket_new` | Create a new bucket for use on the current stream |
| `stream_bucket_prepend` | Prepend bucket to brigade |
| `stream_context_create` | 创建资源流上下文 |
| `stream_context_get_default` | Retrieve the default stream context |
| `stream_context_get_options` | 获取资源流/数据包/上下文的参数 |
| `stream_context_get_params` | Retrieves parameters from a context |
| `stream_context_set_default` | Set the default stream context |
| `stream_context_set_option` | 对资源流、数据包或者上下文设置参数 |
| `stream_context_set_options` | Sets options on the specified context |
| `stream_context_set_params` | Set parameters for a stream/wrapper/context |
| `stream_copy_to_stream` | Copies data from one stream to another |
| `stream_filter_append` | Attach a filter to a stream |
| `stream_filter_prepend` | Attach a filter to a stream |
| `stream_filter_register` | Register a user defined stream filter |
| `stream_filter_remove` | 从资源流里移除某个过滤器 |
| `stream_get_contents` | 读取资源流到一个字符串 |
| `stream_get_filters` | 获取已注册的数据流过滤器列表 |
| `stream_get_line` | 从资源流里读取一行直到给定的定界符 |
| `stream_get_meta_data` | 从流或文件指针中获取 header/meta 数据 |
| `stream_get_transports` | 获取已注册的套接字传输协议列表 |
| `stream_get_wrappers` | 获取已注册的流类型 |
| `stream_is_local` | Checks if a stream is a local stream |
| `stream_isatty` | Check if a stream is a TTY |
| `stream_notification_callback` | A callback function for the notification context parameter |
| `stream_register_wrapper` | 别名 stream_wrapper_register |
| `stream_resolve_include_path` | Resolve filename against the include path |
| `stream_set_blocking` | 为资源流设置阻塞或者阻塞模式 |
| `stream_set_chunk_size` | 设置资源流区块大小 |
| `stream_set_read_buffer` | Set read file buffering on the given stream |
| `stream_set_timeout` | Set timeout period on a stream |
| `stream_set_write_buffer` | Sets write file buffering on the given stream |
| `stream_socket_accept` | 接受由 stream_socket_server 创建的套接字连接 |
| `stream_socket_client` | Open Internet or Unix domain socket connection |
| `stream_socket_enable_crypto` | Turns encryption on/off on an already connected socket |
| `stream_socket_get_name` | 获取本地或者远程的套接字名称 |
| `stream_socket_pair` | 创建一对完全一样的网络套接字连接流 |
| `stream_socket_recvfrom` | Receives data from a socket, connected or not |
| `stream_socket_sendto` | Sends a message to a socket, whether it is connected or not |
| `stream_socket_server` | 创建 Internet 或 Unix 域服务器套接字 |
| `stream_socket_shutdown` | Shutdown a full-duplex connection |
| `stream_supports_lock` | Tells whether the stream supports locking |
| `stream_wrapper_register` | 注册一个用 PHP 类实现的 URL 封装协议 |
| `stream_wrapper_restore` | Restores a previously unregistered built-in wrapper |
| `stream_wrapper_unregister` | Unregister a URL wrapper |
| `strftime` | 根据区域设置格式化本地时间/日期 |
| `strip_tags` | 从字符串中去除 HTML 和 PHP 标签 |
| `stripcslashes` | 反引用一个使用 addcslashes 转义的字符串 |
| `stripos` | 查找字符串首次出现的位置（不区分大小写） |
| `stripslashes` | 反引用一个引用字符串 |
| `stristr` | strstr 函数的忽略大小写版本 |
| `strlen` | 获取字符串长度 |
| `strnatcasecmp` | 使用&ldquo;自然顺序&rdquo;算法比较字符串（不区分大小写） |
| `strnatcmp` | 使用自然排序算法比较字符串 |
| `strncasecmp` | 二进制安全比较字符串开头的若干个字符（不区分大小写） |
| `strncmp` | 二进制安全比较字符串开头的若干个字符 |
| `strpbrk` | 在字符串中查找一组字符的任何一个字符 |
| `strpos` | 查找字符串首次出现的位置 |
| `strptime` | 解析由 strftime 生成的日期／时间 |
| `strrchr` | 查找指定字符在字符串中的最后一次出现 |
| `strrev` | 反转字符串 |
| `strripos` | 计算指定字符串在目标字符串中最后一次出现的位置（不区分大小写） |
| `strrpos` | 计算指定字符串在目标字符串中最后一次出现的位置 |
| `strspn` | 计算字符串中全部字符都存在于指定字符集合中的第一段子串的长度 |
| `strstr` | 查找字符串的首次出现 |
| `strtok` | 标记分割字符串 |
| `strtolower` | 将字符串转化为小写 |
| `strtotime` | 将任何英文文本日期时间描述解析为 Unix 时间戳 |
| `strtoupper` | 将字符串转化为大写 |
| `strtr` | 转换字符或替换字串 |
| `strval` | 获取变量的字符串值 |
| `substr` | 返回字符串的子串 |
| `substr_compare` | 二进制安全比较字符串（从偏移位置比较指定长度） |
| `substr_count` | 计算字串出现的次数 |
| `substr_replace` | 替换字符串的子串 |
| `svn_add` | 在工作目录列入新增项 |
| `svn_auth_get_parameter` | Retrieves authentication parameter |
| `svn_auth_set_parameter` | Sets an authentication parameter |
| `svn_blame` | Get the SVN blame for a file |
| `svn_cat` | Returns the contents of a file in a repository |
| `svn_checkout` | Checks out a working copy from the repository |
| `svn_cleanup` | Recursively cleanup a working copy directory, finishing incomplete operations and removing locks |
| `svn_client_version` | Returns the version of the SVN client libraries |
| `svn_commit` | 将修改的本地文件副本发送至版本库 |
| `svn_delete` | Delete items from a working copy or repository |
| `svn_diff` | Recursively diffs two paths |
| `svn_export` | Export the contents of a SVN directory |
| `svn_fs_abort_txn` | Aborts a transaction |
| `svn_fs_apply_text` | Creates and returns a stream that will be used to replace |
| `svn_fs_begin_txn2` | Create a new transaction |
| `svn_fs_change_node_prop` | Return true if everything is ok, false otherwise |
| `svn_fs_check_path` | Determines what kind of item lives at path in a given repository fsroot |
| `svn_fs_contents_changed` | Return true if content is different, false otherwise |
| `svn_fs_copy` | Copies a file or a directory |
| `svn_fs_delete` | Deletes a file or a directory |
| `svn_fs_dir_entries` | Enumerates the directory entries under path; returns a hash of dir names to file type |
| `svn_fs_file_contents` | Returns a stream to access the contents of a file from a given version of the fs |
| `svn_fs_file_length` | Returns the length of a file from a given version of the fs |
| `svn_fs_is_dir` | Determines if a path points to a directory |
| `svn_fs_is_file` | Determines if a path points to a file |
| `svn_fs_make_dir` | Creates a new empty directory |
| `svn_fs_make_file` | Creates a new empty file |
| `svn_fs_node_created_rev` | Returns the revision in which path under fsroot was created |
| `svn_fs_node_prop` | Returns the value of a property for a node |
| `svn_fs_props_changed` | Return true if props are different, false otherwise |
| `svn_fs_revision_prop` | Fetches the value of a named property |
| `svn_fs_revision_root` | Get a handle on a specific version of the repository root |
| `svn_fs_txn_root` | Creates and returns a transaction root |
| `svn_fs_youngest_rev` | Returns the number of the youngest revision in the filesystem |
| `svn_import` | Imports an unversioned path into a repository |
| `svn_log` | Returns the commit log messages of a repository URL |
| `svn_ls` | Returns list of directory contents in repository URL, optionally at revision number |
| `svn_mkdir` | Creates a directory in a working copy or repository |
| `svn_repos_create` | Create a new subversion repository at path |
| `svn_repos_fs` | Gets a handle on the filesystem for a repository |
| `svn_repos_fs_begin_txn_for_commit` | Create a new transaction |
| `svn_repos_fs_commit_txn` | Commits a transaction and returns the new revision |
| `svn_repos_hotcopy` | Make a hot-copy of the repos at repospath; copy it to destpath |
| `svn_repos_open` | Open a shared lock on a repository |
| `svn_repos_recover` | Run recovery procedures on the repository located at path |
| `svn_revert` | Revert changes to the working copy |
| `svn_status` | Returns the status of working copy files and directories |
| `svn_update` | Update working copy |
| `swoole_async_dns_lookup` | Async and non-blocking hostname to IP lookup |
| `swoole_async_read` | Read file stream asynchronously |
| `swoole_async_readfile` | Read a file asynchronously |
| `swoole_async_set` | Update the async I/O options |
| `swoole_async_write` | Write data to a file stream asynchronously |
| `swoole_async_writefile` | Write data to a file asynchronously |
| `swoole_clear_error` | Clear errors in the socket or on the last error code |
| `swoole_client_select` | Get the file description which are ready to read/write or error |
| `swoole_cpu_num` | Get the number of CPU |
| `swoole_errno` | Get the error code of the latest system call |
| `swoole_error_log` | Output error messages to the log |
| `swoole_event_add` | Add new callback functions of a socket into the EventLoop |
| `swoole_event_defer` | Add callback function to the next event loop |
| `swoole_event_del` | Remove all event callback functions of a socket |
| `swoole_event_exit` | Exit the eventloop, only available at the client side |
| `swoole_event_set` | Update the event callback functions of a socket |
| `swoole_event_wait` | Start the event loop |
| `swoole_event_write` | Write data to a socket |
| `swoole_get_local_ip` | Get the IPv4 IP addresses of each NIC on the machine |
| `swoole_last_error` | Get the lastest error message |
| `swoole_load_module` | Load a swoole extension |
| `swoole_select` | Select the file descriptions which are ready to read/write or error in the eventloop |
| `swoole_set_process_name` | Set the process name |
| `swoole_strerror` | Convert the Errno into error messages |
| `swoole_timer_after` | Trigger a one time callback function in the future |
| `swoole_timer_exists` | Check if a timer callback function is existed |
| `swoole_timer_tick` | Trigger a timer tick callback function by time interval |
| `swoole_version` | Get the version of Swoole |
| `symlink` | 建立符号连接 |
| `sys_get_temp_dir` | 返回用于临时文件的目录 |
| `sys_getloadavg` | 获取系统的负载（load average） |
| `syslog` | Generate a system log message |
| `system` | 执行外部程序，并且显示输出 |

### T

| 函数名 | 功能描述 |
|--------|----------|
| `taint` | Taint a string |
| `tan` | 正切 |
| `tanh` | 双曲正切 |
| `tcpwrap_check` | Performs a tcpwrap check |
| `tempnam` | 建立一个具有唯一文件名的文件 |
| `textdomain` | Sets the default domain |
| `tidy_access_count` | Returns the Number of Tidy accessibility warnings encountered for specified document |
| `tidy_config_count` | Returns the Number of Tidy configuration errors encountered for specified document |
| `tidy_error_count` | Returns the Number of Tidy errors encountered for specified document |
| `tidy_get_output` | Return a string representing the parsed tidy markup |
| `tidy_warning_count` | Returns the Number of Tidy warnings encountered for specified document |
| `time` | 返回当前的 Unix 时间戳 |
| `time_nanosleep` | 延缓执行若干秒和纳秒 |
| `time_sleep_until` | 使脚本睡眠到指定的时间为止 |
| `timezone_abbreviations_list` | 别名 DateTimeZone::listAbbreviations |
| `timezone_identifiers_list` | 别名 DateTimeZone::listIdentifiers |
| `timezone_location_get` | 别名 DateTimeZone::getLocation |
| `timezone_name_from_abbr` | Returns a timezone name by guessing from abbreviation and UTC offset |
| `timezone_name_get` | 别名 DateTimeZone::getName |
| `timezone_offset_get` | 别名 DateTimeZone::getOffset |
| `timezone_open` | 别名 DateTimeZone::__construct |
| `timezone_transitions_get` | 别名 DateTimeZone::getTransitions |
| `timezone_version_get` | 获取 timezonedb 版本 |
| `tmpfile` | 建立一个临时文件 |
| `token_get_all` | 将提供的源码按 PHP 标记进行分割 |
| `token_name` | 获取提供的 PHP 解析器代号的符号名称 |
| `touch` | 设定文件的访问和修改时间 |
| `trader_acos` | Vector Trigonometric ACos |
| `trader_ad` | Chaikin A/D Line |
| `trader_add` | Vector Arithmetic Add |
| `trader_adosc` | Chaikin A/D Oscillator |
| `trader_adx` | Average Directional Movement Index |
| `trader_adxr` | Average Directional Movement Index Rating |
| `trader_apo` | Absolute Price Oscillator |
| `trader_aroon` | Aroon |
| `trader_aroonosc` | Aroon Oscillator |
| `trader_asin` | Vector Trigonometric ASin |
| `trader_atan` | Vector Trigonometric ATan |
| `trader_atr` | Average True Range |
| `trader_avgprice` | Average Price |
| `trader_bbands` | Bollinger Bands |
| `trader_beta` | Beta |
| `trader_bop` | Balance Of Power |
| `trader_cci` | Commodity Channel Index |
| `trader_cdl2crows` | Two Crows |
| `trader_cdl3blackcrows` | Three Black Crows |
| `trader_cdl3inside` | Three Inside Up/Down |
| `trader_cdl3linestrike` | Three-Line Strike |
| `trader_cdl3outside` | Three Outside Up/Down |
| `trader_cdl3starsinsouth` | Three Stars In The South |
| `trader_cdl3whitesoldiers` | Three Advancing White Soldiers |
| `trader_cdlabandonedbaby` | Abandoned Baby |
| `trader_cdladvanceblock` | Advance Block |
| `trader_cdlbelthold` | Belt-hold |
| `trader_cdlbreakaway` | Breakaway |
| `trader_cdlclosingmarubozu` | Closing Marubozu |
| `trader_cdlconcealbabyswall` | Concealing Baby Swallow |
| `trader_cdlcounterattack` | Counterattack |
| `trader_cdldarkcloudcover` | Dark Cloud Cover |
| `trader_cdldoji` | Doji |
| `trader_cdldojistar` | Doji Star |
| `trader_cdldragonflydoji` | Dragonfly Doji |
| `trader_cdlengulfing` | Engulfing Pattern |
| `trader_cdleveningdojistar` | Evening Doji Star |
| `trader_cdleveningstar` | Evening Star |
| `trader_cdlgapsidesidewhite` | Up/Down-gap side-by-side white lines |
| `trader_cdlgravestonedoji` | Gravestone Doji |
| `trader_cdlhammer` | Hammer |
| `trader_cdlhangingman` | Hanging Man |
| `trader_cdlharami` | Harami Pattern |
| `trader_cdlharamicross` | Harami Cross Pattern |
| `trader_cdlhighwave` | High-Wave Candle |
| `trader_cdlhikkake` | Hikkake Pattern |
| `trader_cdlhikkakemod` | Modified Hikkake Pattern |
| `trader_cdlhomingpigeon` | Homing Pigeon |
| `trader_cdlidentical3crows` | Identical Three Crows |
| `trader_cdlinneck` | In-Neck Pattern |
| `trader_cdlinvertedhammer` | Inverted Hammer |
| `trader_cdlkicking` | Kicking |
| `trader_cdlkickingbylength` | Kicking - bull/bear determined by the longer marubozu |
| `trader_cdlladderbottom` | Ladder Bottom |
| `trader_cdllongleggeddoji` | Long Legged Doji |
| `trader_cdllongline` | Long Line Candle |
| `trader_cdlmarubozu` | Marubozu |
| `trader_cdlmatchinglow` | Matching Low |
| `trader_cdlmathold` | Mat Hold |
| `trader_cdlmorningdojistar` | Morning Doji Star |
| `trader_cdlmorningstar` | Morning Star |
| `trader_cdlonneck` | On-Neck Pattern |
| `trader_cdlpiercing` | Piercing Pattern |
| `trader_cdlrickshawman` | Rickshaw Man |
| `trader_cdlrisefall3methods` | Rising/Falling Three Methods |
| `trader_cdlseparatinglines` | Separating Lines |
| `trader_cdlshootingstar` | Shooting Star |
| `trader_cdlshortline` | Short Line Candle |
| `trader_cdlspinningtop` | Spinning Top |
| `trader_cdlstalledpattern` | Stalled Pattern |
| `trader_cdlsticksandwich` | Stick Sandwich |
| `trader_cdltakuri` | Takuri (Dragonfly Doji with very long lower shadow) |
| `trader_cdltasukigap` | Tasuki Gap |
| `trader_cdlthrusting` | Thrusting Pattern |
| `trader_cdltristar` | Tristar Pattern |
| `trader_cdlunique3river` | Unique 3 River |
| `trader_cdlupsidegap2crows` | Upside Gap Two Crows |
| `trader_cdlxsidegap3methods` | Upside/Downside Gap Three Methods |
| `trader_ceil` | Vector Ceil |
| `trader_cmo` | Chande Momentum Oscillator |
| `trader_correl` | Pearson's Correlation Coefficient (r) |
| `trader_cos` | Vector Trigonometric Cos |
| `trader_cosh` | Vector Trigonometric Cosh |
| `trader_dema` | Double Exponential Moving Average |
| `trader_div` | Vector Arithmetic Div |
| `trader_dx` | Directional Movement Index |
| `trader_ema` | Exponential Moving Average |
| `trader_errno` | Get error code |
| `trader_exp` | Vector Arithmetic Exp |
| `trader_floor` | Vector Floor |
| `trader_get_compat` | Get compatibility mode |
| `trader_get_unstable_period` | Get unstable period |
| `trader_ht_dcperiod` | Hilbert Transform - Dominant Cycle Period |
| `trader_ht_dcphase` | Hilbert Transform - Dominant Cycle Phase |
| `trader_ht_phasor` | Hilbert Transform - Phasor Components |
| `trader_ht_sine` | Hilbert Transform - SineWave |
| `trader_ht_trendline` | Hilbert Transform - Instantaneous Trendline |
| `trader_ht_trendmode` | Hilbert Transform - Trend vs Cycle Mode |
| `trader_kama` | Kaufman Adaptive Moving Average |
| `trader_linearreg` | Linear Regression |
| `trader_linearreg_angle` | Linear Regression Angle |
| `trader_linearreg_intercept` | Linear Regression Intercept |
| `trader_linearreg_slope` | Linear Regression Slope |
| `trader_ln` | Vector Log Natural |
| `trader_log10` | Vector Log10 |
| `trader_ma` | Moving average |
| `trader_macd` | Moving Average Convergence/Divergence |
| `trader_macdext` | MACD with controllable MA type |
| `trader_macdfix` | Moving Average Convergence/Divergence Fix 12/26 |
| `trader_mama` | MESA Adaptive Moving Average |
| `trader_mavp` | Moving average with variable period |
| `trader_max` | Highest value over a specified period |
| `trader_maxindex` | Index of highest value over a specified period |
| `trader_medprice` | Median Price |
| `trader_mfi` | Money Flow Index |
| `trader_midpoint` | MidPoint over period |
| `trader_midprice` | Midpoint Price over period |
| `trader_min` | Lowest value over a specified period |
| `trader_minindex` | Index of lowest value over a specified period |
| `trader_minmax` | Lowest and highest values over a specified period |
| `trader_minmaxindex` | Indexes of lowest and highest values over a specified period |
| `trader_minus_di` | Minus Directional Indicator |
| `trader_minus_dm` | Minus Directional Movement |
| `trader_mom` | Momentum |
| `trader_mult` | Vector Arithmetic Mult |
| `trader_natr` | Normalized Average True Range |
| `trader_obv` | On Balance Volume |
| `trader_plus_di` | Plus Directional Indicator |
| `trader_plus_dm` | Plus Directional Movement |
| `trader_ppo` | Percentage Price Oscillator |
| `trader_roc` | Rate of change : ((price/prevPrice)-1)*100 |
| `trader_rocp` | Rate of change Percentage: (price-prevPrice)/prevPrice |
| `trader_rocr` | Rate of change ratio: (price/prevPrice) |
| `trader_rocr100` | Rate of change ratio 100 scale: (price/prevPrice)*100 |
| `trader_rsi` | Relative Strength Index |
| `trader_sar` | Parabolic SAR |
| `trader_sarext` | Parabolic SAR - Extended |
| `trader_set_compat` | Set compatibility mode |
| `trader_set_unstable_period` | Set unstable period |
| `trader_sin` | Vector Trigonometric Sin |
| `trader_sinh` | Vector Trigonometric Sinh |
| `trader_sma` | Simple Moving Average |
| `trader_sqrt` | Vector Square Root |
| `trader_stddev` | Standard Deviation |
| `trader_stoch` | Stochastic |
| `trader_stochf` | Stochastic Fast |
| `trader_stochrsi` | Stochastic Relative Strength Index |
| `trader_sub` | Vector Arithmetic Subtraction |
| `trader_sum` | Summation |
| `trader_t3` | Triple Exponential Moving Average (T3) |
| `trader_tan` | Vector Trigonometric Tan |
| `trader_tanh` | Vector Trigonometric Tanh |
| `trader_tema` | Triple Exponential Moving Average |
| `trader_trange` | True Range |
| `trader_trima` | Triangular Moving Average |
| `trader_trix` | 1-day Rate-Of-Change (ROC) of a Triple Smooth EMA |
| `trader_tsf` | Time Series Forecast |
| `trader_typprice` | Typical Price |
| `trader_ultosc` | Ultimate Oscillator |
| `trader_var` | Variance |
| `trader_wclprice` | Weighted Close Price |
| `trader_willr` | Williams' %R |
| `trader_wma` | Weighted Moving Average |
| `trait_exists` | 检查指定的 trait 是否存在 |
| `trigger_error` | 产生一个用户级别的 error/warning/notice 信息 |
| `trim` | 去除字符串首尾处的空白字符（或者其他字符） |

### U

| 函数名 | 功能描述 |
|--------|----------|
| `uasort` | 使用用户定义的比较函数对数组进行排序并保持索引关联 |
| `ucfirst` | 将字符串的首字母转换为大写 |
| `ucwords` | 将字符串中每个单词的首字母转换为大写 |
| `UI\Draw\Text\Font\fontFamilies` | Retrieve Font Families |
| `UI\quit` | Quit UI Loop |
| `UI\run` | Enter UI Loop |
| `uksort` | 使用用户自定义的比较函数对数组中的键名进行排序 |
| `umask` | 改变当前的 umask |
| `uniqid` | 生成基于时间的标识符 |
| `unixtojd` | 将 Unix 时间戳转换为儒略日数 |
| `unlink` | 删除文件 |
| `unpack` | Unpack data from binary string |
| `unregister_tick_function` | 注销每个 tick 上需要执行的函数 |
| `unserialize` | 从已存储的表示中创建 PHP 的值 |
| `unset` | unset 指定变量 |
| `untaint` | Untaint strings |
| `uopz_add_function` | Adds non-existent function or method |
| `uopz_allow_exit` | Allows control over disabled exit opcode |
| `uopz_backup` | Backup a function |
| `uopz_compose` | Compose a class |
| `uopz_copy` | Copy a function |
| `uopz_del_function` | Deletes previously added function or method |
| `uopz_delete` | Delete a function |
| `uopz_extend` | Extend a class at runtime |
| `uopz_flags` | Get or set flags on function or class |
| `uopz_function` | Creates a function at runtime |
| `uopz_get_exit_status` | Retrieve the last set exit status |
| `uopz_get_hook` | Gets previously set hook on function or method |
| `uopz_get_mock` | Get the current mock for a class |
| `uopz_get_property` | Gets value of class or instance property |
| `uopz_get_return` | Gets a previous set return value for a function |
| `uopz_get_static` | Gets the static variables from function or method scope |
| `uopz_implement` | Implements an interface at runtime |
| `uopz_overload` | Overload a VM opcode |
| `uopz_redefine` | Redefine a constant |
| `uopz_rename` | Rename a function at runtime |
| `uopz_restore` | Restore a previously backed up function |
| `uopz_set_hook` | Sets hook to execute when entering a function or method |
| `uopz_set_mock` | Use mock instead of class for new objects |
| `uopz_set_property` | Sets value of existing class or instance property |
| `uopz_set_return` | Provide a return value for an existing function |
| `uopz_set_static` | Sets the static variables in function or method scope |
| `uopz_undefine` | Undefine a constant |
| `uopz_unset_hook` | Removes previously set hook on function or method |
| `uopz_unset_mock` | Unset previously set mock |
| `uopz_unset_return` | Unsets a previously set return value for a function |
| `urldecode` | 解码已编码的 URL 字符串 |
| `urlencode` | 编码 URL 字符串 |
| `use_soap_error_handler` | Set whether to use the SOAP error handler |
| `user_error` | 别名 trigger_error |
| `usleep` | 以指定的微秒数延迟执行 |
| `usort` | 使用用户自定义的比较函数对数组中的值进行排序 |
| `utf8_decode` | 将字符串从 UTF-8 转换为 ISO-8859-1，替换无效或者无法表示的字符。 |
| `utf8_encode` | 将字符串从 ISO-8859-1 转换为 UTF-8 编码 |

### V

| 函数名 | 功能描述 |
|--------|----------|
| `var_dump` | 打印变量的相关信息 |
| `var_export` | 输出或返回变量的可解析字符串表示 |
| `var_representation` | Returns a short, readable, parsable string representation of a variable |
| `variant_abs` | Returns the absolute value of a variant |
| `variant_add` | &quot;Adds&quot; two variant values together and returns the result |
| `variant_and` | Performs a bitwise AND operation between two variants |
| `variant_cast` | Convert a variant into a new variant object of another type |
| `variant_cat` | Concatenates two variant values together and returns the result |
| `variant_cmp` | Compares two variants |
| `variant_date_from_timestamp` | Returns a variant date representation of a Unix timestamp |
| `variant_date_to_timestamp` | Converts a variant date/time value to Unix timestamp |
| `variant_div` | Returns the result from dividing two variants |
| `variant_eqv` | Performs a bitwise equivalence on two variants |
| `variant_fix` | Returns the integer portion of a variant |
| `variant_get_type` | Returns the type of a variant object |
| `variant_idiv` | Converts variants to integers and then returns the result from dividing them |
| `variant_imp` | Performs a bitwise implication on two variants |
| `variant_int` | Returns the integer portion of a variant |
| `variant_mod` | Divides two variants and returns only the remainder |
| `variant_mul` | Multiplies the values of the two variants |
| `variant_neg` | Performs logical negation on a variant |
| `variant_not` | Performs bitwise not negation on a variant |
| `variant_or` | Performs a logical disjunction on two variants |
| `variant_pow` | Returns the result of performing the power function with two variants |
| `variant_round` | Rounds a variant to the specified number of decimal places |
| `variant_set` | Assigns a new value for a variant object |
| `variant_set_type` | Convert a variant into another type &quot;in-place&quot; |
| `variant_sub` | Subtracts the value of the right variant from the left variant value |
| `variant_xor` | Performs a logical exclusion on two variants |
| `version_compare` | 对比两个「PHP 规范化」的版本数字字符串 |
| `vfprintf` | 将格式化字符串写入流 |
| `virtual` | 执行 Apache 子请求 |
| `vprintf` | 输出格式化字符串 |
| `vsprintf` | 返回格式化字符串 |

### W

| 函数名 | 功能描述 |
|--------|----------|
| `wddx_add_vars` | Add variables to a WDDX packet with the specified ID |
| `wddx_deserialize` | Unserializes a WDDX packet |
| `wddx_packet_end` | Ends a WDDX packet with the specified ID |
| `wddx_packet_start` | Starts a new WDDX packet with structure inside it |
| `wddx_serialize_value` | Serialize a single value into a WDDX packet |
| `wddx_serialize_vars` | Serialize variables into a WDDX packet |
| `win32_add_right_access_service` | Add rights access for an username to the service |
| `win32_add_service_env_var` | Add a custom environment variables on service |
| `win32_continue_service` | Resumes a paused service |
| `win32_create_service` | Creates a new service entry in the SCM database |
| `win32_delete_service` | Deletes a service entry from the SCM database |
| `win32_get_last_control_message` | Returns the last control message that was sent to this service |
| `win32_get_service_env_vars` | Read all custom environment variables on service |
| `win32_pause_service` | Pauses a service |
| `win32_query_service_status` | Queries the status of a service |
| `win32_read_all_rights_access_service` | Read all service rights access |
| `win32_read_right_access_service` | Read the service rights access for an username |
| `win32_remove_right_access_service` | Remove the service rights access for an username |
| `win32_remove_service_env_var` | Remove a custom environment variables on service |
| `win32_send_custom_control` | Send a custom control to the service |
| `win32_set_service_exit_code` | Define or return the exit code for the current running service |
| `win32_set_service_exit_mode` | Define or return the exit mode for the current running service |
| `win32_set_service_pause_resume_state` | Define or return the pause/resume capability for the current running service |
| `win32_set_service_status` | Update the service status |
| `win32_start_service` | Starts a service |
| `win32_start_service_ctrl_dispatcher` | Registers the script with the SCM, so that it can act as the service with the given name |
| `win32_stop_service` | Stops a service |
| `wincache_fcache_fileinfo` | Retrieves information about files cached in the file cache |
| `wincache_fcache_meminfo` | Retrieves information about file cache memory usage |
| `wincache_lock` | Acquires an exclusive lock on a given key |
| `wincache_ocache_fileinfo` | Retrieves information about files cached in the opcode cache |
| `wincache_ocache_meminfo` | Retrieves information about opcode cache memory usage |
| `wincache_refresh_if_changed` | Refreshes the cache entries for the cached files |
| `wincache_rplist_fileinfo` | Retrieves information about resolve file path cache |
| `wincache_rplist_meminfo` | Retrieves information about memory usage by the resolve file path cache |
| `wincache_scache_info` | Retrieves information about files cached in the session cache |
| `wincache_scache_meminfo` | Retrieves information about session cache memory usage |
| `wincache_ucache_add` | Adds a variable in user cache only if variable does not already exist in the cache |
| `wincache_ucache_cas` | Compares the variable with old value and assigns new value to it |
| `wincache_ucache_clear` | Deletes entire content of the user cache |
| `wincache_ucache_dec` | Decrements the value associated with the key |
| `wincache_ucache_delete` | Deletes variables from the user cache |
| `wincache_ucache_exists` | Checks if a variable exists in the user cache |
| `wincache_ucache_get` | Gets a variable stored in the user cache |
| `wincache_ucache_inc` | Increments the value associated with the key |
| `wincache_ucache_info` | Retrieves information about data stored in the user cache |
| `wincache_ucache_meminfo` | Retrieves information about user cache memory usage |
| `wincache_ucache_set` | Adds a variable in user cache and overwrites a variable if it already exists in the cache |
| `wincache_unlock` | Releases an exclusive lock on a given key |
| `wordwrap` | 打断字符串为指定数量的字串 |

### X

| 函数名 | 功能描述 |
|--------|----------|
| `xattr_get` | Get an extended attribute |
| `xattr_list` | Get a list of extended attributes |
| `xattr_remove` | Remove an extended attribute |
| `xattr_set` | Set an extended attribute |
| `xattr_supported` | Check if filesystem supports extended attributes |
| `xdiff_file_bdiff` | Make binary diff of two files |
| `xdiff_file_bdiff_size` | Read a size of file created by applying a binary diff |
| `xdiff_file_bpatch` | Patch a file with a binary diff |
| `xdiff_file_diff` | Make unified diff of two files |
| `xdiff_file_diff_binary` | 别名 xdiff_file_bdiff |
| `xdiff_file_merge3` | Merge 3 files into one |
| `xdiff_file_patch` | Patch a file with an unified diff |
| `xdiff_file_patch_binary` | 别名 xdiff_file_bpatch |
| `xdiff_file_rabdiff` | Make binary diff of two files using the Rabin's polynomial fingerprinting algorithm |
| `xdiff_string_bdiff` | Make binary diff of two strings |
| `xdiff_string_bdiff_size` | Read a size of file created by applying a binary diff |
| `xdiff_string_bpatch` | Patch a string with a binary diff |
| `xdiff_string_diff` | Make unified diff of two strings |
| `xdiff_string_diff_binary` | 别名 xdiff_string_bdiff |
| `xdiff_string_merge3` | Merge 3 strings into one |
| `xdiff_string_patch` | Patch a string with an unified diff |
| `xdiff_string_patch_binary` | 别名 xdiff_string_bpatch |
| `xdiff_string_rabdiff` | Make a binary diff of two strings using the Rabin's polynomial fingerprinting algorithm |
| `xhprof_disable` | 停止 xhprof 分析器 |
| `xhprof_enable` | 启动 xhprof 性能分析器 |
| `xhprof_sample_disable` | 停止 xhprof 性能采样分析器 |
| `xhprof_sample_enable` | 以采样模式启动 XHProf 性能分析 |
| `xml_error_string` | 获取 XML 解析器的错误字符串 |
| `xml_get_current_byte_index` | 获取 XML 解析器的当前字节索引 |
| `xml_get_current_column_number` | 获取 XML 解析器的当前列号 |
| `xml_get_current_line_number` | 获取 XML 解析器的当前行号 |
| `xml_get_error_code` | 获取 XML 解析器错误代码 |
| `xml_parse` | 开始解析 XML 文档 |
| `xml_parse_into_struct` | 解析 XML 数据到数组中 |
| `xml_parser_create` | 创建 XML 解析器 |
| `xml_parser_create_ns` | 创建支持命名空间的 XML 解析器 |
| `xml_parser_free` | 释放 XML 解析器 |
| `xml_parser_get_option` | 从 XML 解析器获取选项 |
| `xml_parser_set_option` | 在 XML 解析器中设置选项 |
| `xml_set_character_data_handler` | 建立字符数据处理器 |
| `xml_set_default_handler` | 建立默认处理程序 |
| `xml_set_element_handler` | 建立起始和终止元素处理程序 |
| `xml_set_end_namespace_decl_handler` | 设置终止命名空间声明处理程序 |
| `xml_set_external_entity_ref_handler` | 设置外部实体引用处理程序 |
| `xml_set_notation_decl_handler` | 设置符号声明处理程序 |
| `xml_set_object` | 在对象中使用 XML 解析器 |
| `xml_set_processing_instruction_handler` | 建立处理指令（PI）处理程序 |
| `xml_set_start_namespace_decl_handler` | 设置起始命名空间声明处理程序 |
| `xml_set_unparsed_entity_decl_handler` | 建立未解析实体定义声明处理程序 |
| `xmlrpc_decode` | 将 XML 解码为原生 PHP 类型 |
| `xmlrpc_decode_request` | 将 XML 解码为原生 PHP 类型 |
| `xmlrpc_encode` | 为 PHP 值生成 XML |
| `xmlrpc_encode_request` | 为方法请求生成 XML |
| `xmlrpc_get_type` | 获取 PHP 值的 xmlrpc 类型 |
| `xmlrpc_is_fault` | Determines if an array value represents an XMLRPC fault |
| `xmlrpc_parse_method_descriptions` | 将 XML 解码为方法描述列表 |
| `xmlrpc_server_add_introspection_data` | 添加自我描述文档 |
| `xmlrpc_server_call_method` | 解析 XML 请求和调用方法 |
| `xmlrpc_server_create` | 创建 xmlrpc 服务器 |
| `xmlrpc_server_destroy` | 销毁服务端资源 |
| `xmlrpc_server_register_introspection_callback` | 注册 PHP 函数来生成文档 |
| `xmlrpc_server_register_method` | 注册 PHP 函数搭到资源用于匹配 method_name |
| `xmlrpc_set_type` | 为 PHP 字符串值设置 xmlrpc 类型，base64 或 datetime |

### Y

| 函数名 | 功能描述 |
|--------|----------|
| `yaml_emit` | Returns the YAML representation of a value |
| `yaml_emit_file` | Send the YAML representation of a value to a file |
| `yaml_parse` | Parse a YAML stream |
| `yaml_parse_file` | Parse a YAML stream from a file |
| `yaml_parse_url` | Parse a Yaml stream from a URL |
| `yaz_addinfo` | Returns additional error information |
| `yaz_ccl_conf` | Configure CCL parser |
| `yaz_ccl_parse` | Invoke CCL Parser |
| `yaz_close` | Close YAZ connection |
| `yaz_connect` | Prepares for a connection to a Z39.50 server |
| `yaz_database` | Specifies the databases within a session |
| `yaz_element` | Specifies Element-Set Name for retrieval |
| `yaz_errno` | Returns error number |
| `yaz_error` | Returns error description |
| `yaz_es` | Prepares for an Extended Service Request |
| `yaz_es_result` | Inspects Extended Services Result |
| `yaz_get_option` | Returns value of option for connection |
| `yaz_hits` | Returns number of hits for last search |
| `yaz_itemorder` | Prepares for Z39.50 Item Order with an ILL-Request package |
| `yaz_present` | Prepares for retrieval (Z39.50 present) |
| `yaz_range` | Specifies a range of records to retrieve |
| `yaz_record` | Returns a record |
| `yaz_scan` | Prepares for a scan |
| `yaz_scan_result` | Returns Scan Response result |
| `yaz_schema` | Specifies schema for retrieval |
| `yaz_search` | Prepares for a search |
| `yaz_set_option` | Sets one or more options for connection |
| `yaz_sort` | Sets sorting criteria |
| `yaz_syntax` | Specifies the preferred record syntax for retrieval |
| `yaz_wait` | Wait for Z39.50 requests to complete |

### Z

| 函数名 | 功能描述 |
|--------|----------|
| `zend_thread_id` | 返回当前线程的唯一识别符 |
| `zend_version` | 获取当前 Zend 引擎的版本 |
| `zip_close` | 关闭一个ZIP档案文件 |
| `zip_entry_close` | 关闭目录项 |
| `zip_entry_compressedsize` | 检索目录项压缩过后的大小 |
| `zip_entry_compressionmethod` | 检索目录实体的压缩方法 |
| `zip_entry_filesize` | 检索目录实体的实际大小 |
| `zip_entry_name` | 检索目录项的名称 |
| `zip_entry_open` | 打开用于读取的目录实体 |
| `zip_entry_read` | 读取一个打开了的压缩目录实体 |
| `zip_open` | 打开 ZIP 文件归档 |
| `zip_read` | 读取 ZIP 文件归档中下一项 |
| `zlib_decode` | Uncompress any raw/gzip/zlib encoded data |
| `zlib_encode` | Compress data with the specified encoding |
| `zlib_get_coding_type` | Returns the coding type used for output compression |
| `zookeeper_dispatch` | Calls callbacks for pending operations |

