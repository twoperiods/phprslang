// PHPRS C Runtime — embedded by the compiler into every build output
// Provides: TCP sockets, file I/O, HTTP parsing, JSON helpers, string utilities

// Built-in: array length. The transpiler stores lengths as separate _len variables.
// This function acts as a linkable symbol; the transpiler may override with direct var access.
static int64_t __array_len_dummy = 0;
int64_t count(int64_t* arr) {
    (void)arr;
    return __array_len_dummy;
}

#ifdef _WIN32
    #define WIN32_LEAN_AND_MEAN
    #include <winsock2.h>
    #include <ws2tcpip.h>
    #pragma comment(lib, "ws2_32.lib")
    #define SECURITY_WIN32
    #include <security.h>
    #include <schannel.h>
    #include <sspi.h>
    #pragma comment(lib, "secur32.lib")
    typedef SOCKET phprs_socket_t;
    #define PHPRS_INVALID_SOCKET INVALID_SOCKET
    #define PHPRS_SOCKET_ERROR SOCKET_ERROR
    #define phprs_closesocket closesocket
    static int phprs_winsock_ready = 0;
    static void phprs_winsock_init(void) {
        if (!phprs_winsock_ready) {
            WSADATA wsa;
            WSAStartup(MAKEWORD(2, 2), &wsa);
            phprs_winsock_ready = 1;
        }
    }
#else
    #include <unistd.h>
    #include <sys/socket.h>
    #include <sys/stat.h>
    #include <arpa/inet.h>
    #include <netinet/in.h>
    #include <fcntl.h>
    #include <openssl/ssl.h>
    #include <openssl/err.h>
    typedef int phprs_socket_t;
    #define PHPRS_INVALID_SOCKET (-1)
    #define PHPRS_SOCKET_ERROR (-1)
    #define phprs_closesocket close
#endif
#include <stdarg.h>
#include <stdint.h>
#include <string.h>
#include <stdlib.h>
#include <errno.h>
#include <setjmp.h>
#include <time.h>
#include <math.h>

#ifndef _WIN32
typedef int BOOL;
#define TRUE 1
#define FALSE 0
#endif

// strtok_r compatibility for MSVC
#ifdef _MSC_VER
#define strtok_r strtok_s
#endif

// ---- TLS Context Management ----
// Must be declared before socket primitives because phprs_socket_write,
// phprs_socket_read_all, and phprs_socket_close reference these types.

#define PHPRS_MAX_TLS 64

#ifdef _WIN32
typedef struct {
    CtxtHandle ctxt_handle;
    SecPkgContext_StreamSizes sizes;
    BOOL handshake_done;
    unsigned char* decrypt_buf;
    size_t decrypt_buf_len;
    size_t decrypt_buf_offset;
} phprs_tls_ctx;
#else
typedef struct {
    SSL* ssl;
} phprs_tls_ctx;
#endif

typedef struct {
    int64_t fd;
    phprs_tls_ctx* ctx;
} phprs_tls_entry;

static phprs_tls_entry phprs_tls_entries[PHPRS_MAX_TLS];
static int phprs_tls_count = 0;
static BOOL phprs_tls_zinit = FALSE;

static void phprs_tls_init_entries(void) {
    if (!phprs_tls_zinit) {
        memset(phprs_tls_entries, 0, sizeof(phprs_tls_entries));
        phprs_tls_zinit = TRUE;
    }
}

static phprs_tls_ctx* phprs_tls_find(int64_t fd) {
    phprs_tls_init_entries();
    for (int i = 0; i < phprs_tls_count; i++) {
        if (phprs_tls_entries[i].fd == fd)
            return phprs_tls_entries[i].ctx;
    }
    return NULL;
}

static int phprs_tls_add(int64_t fd, phprs_tls_ctx* ctx) {
    phprs_tls_init_entries();
    if (phprs_tls_count >= PHPRS_MAX_TLS) return -1;
    phprs_tls_entries[phprs_tls_count].fd = fd;
    phprs_tls_entries[phprs_tls_count].ctx = ctx;
    phprs_tls_count++;
    return 0;
}

static void phprs_tls_remove(int64_t fd) {
    phprs_tls_init_entries();
    for (int i = 0; i < phprs_tls_count; i++) {
        if (phprs_tls_entries[i].fd == fd) {
            phprs_tls_ctx* ctx = phprs_tls_entries[i].ctx;
            if (ctx) {
#ifdef _WIN32
                DeleteSecurityContext(&ctx->ctxt_handle);
                free(ctx->decrypt_buf);
#else
                if (ctx->ssl) {
                    SSL_shutdown(ctx->ssl);
                    SSL_free(ctx->ssl);
                }
#endif
                free(ctx);
            }
            phprs_tls_entries[i] = phprs_tls_entries[phprs_tls_count - 1];
            phprs_tls_entries[phprs_tls_count - 1].fd = 0;
            phprs_tls_entries[phprs_tls_count - 1].ctx = NULL;
            phprs_tls_count--;
            return;
        }
    }
}

// ---- Socket Primitives ----

int64_t phprs_server_new(int64_t port) {
#ifdef _WIN32
    phprs_winsock_init();
#endif
    phprs_socket_t sock = socket(AF_INET, SOCK_STREAM, 0);
    if (sock == PHPRS_INVALID_SOCKET) return -1;

    int opt = 1;
    setsockopt(sock, SOL_SOCKET, SO_REUSEADDR,
#ifdef _WIN32
        (const char*)&opt, sizeof(opt));
#else
        &opt, sizeof(opt));
#endif

    struct sockaddr_in addr;
    addr.sin_family = AF_INET;
    addr.sin_addr.s_addr = INADDR_ANY;
    addr.sin_port = htons((unsigned short)port);

    if (bind(sock, (struct sockaddr*)&addr, sizeof(addr)) == PHPRS_SOCKET_ERROR) {
        phprs_closesocket(sock);
        return -1;
    }
    if (listen(sock, 10) == PHPRS_SOCKET_ERROR) {
        phprs_closesocket(sock);
        return -1;
    }
    return (int64_t)sock;
}

static char phprs_last_client_ip[64] = "127.0.0.1";

int64_t phprs_server_accept(int64_t server_fd) {
    struct sockaddr_in client_addr;
    socklen_t len = sizeof(client_addr);
    phprs_socket_t client = accept((phprs_socket_t)server_fd,
        (struct sockaddr*)&client_addr, &len);
    if (client == PHPRS_INVALID_SOCKET) return -1;
    unsigned char *ip = (unsigned char*)&client_addr.sin_addr;
    snprintf(phprs_last_client_ip, sizeof(phprs_last_client_ip),
        "%u.%u.%u.%u", ip[0], ip[1], ip[2], ip[3]);
    return (int64_t)client;
}

const char* phprs_client_ip(int64_t fd) {
    (void)fd;
    return strdup(phprs_last_client_ip);
}

#ifdef _WIN32
#define PHPRS_THREAD_LOCAL __declspec(thread)
#else
#define PHPRS_THREAD_LOCAL _Thread_local
#endif

const char* phprs_socket_read(int64_t fd, int64_t max_size) {
    static PHPRS_THREAD_LOCAL char* buf = NULL;
    static PHPRS_THREAD_LOCAL size_t buf_cap = 0;

    if (max_size <= 0) max_size = 65536;
    if ((size_t)max_size + 1 > buf_cap) {
        size_t new_cap = (size_t)max_size + 1;
        char* new_buf = (char*)realloc(buf, new_cap);
        if (!new_buf) return strdup("");
        buf = new_buf;
        buf_cap = new_cap;
    }

#ifdef _WIN32
    int n = recv((SOCKET)fd, buf, (int)max_size, 0);
#else
    ssize_t n = read((int)fd, buf, (size_t)max_size);
#endif
    if (n <= 0) { buf[0] = '\0'; return buf; }
    buf[n] = '\0';
    return buf;
}

int64_t phprs_socket_write(int64_t fd, const char* data) {
    if (!data) return -1;
    size_t len = strlen(data);

    // Check for TLS context
    phprs_tls_ctx* tls = phprs_tls_find(fd);
    if (tls) {
#ifdef _WIN32
        unsigned char* enc = NULL;
        size_t enc_len = 0;
        if (phprs_tls_encrypt(tls, data, len, &enc, &enc_len) != 0)
            return -1;
        int n = send((SOCKET)fd, (const char*)enc, (int)enc_len, 0);
        free(enc);
        return (n > 0) ? (int64_t)len : (int64_t)n;
#else
        int n = SSL_write(tls->ssl, data, (int)len);
        return (int64_t)n;
#endif
    }

    // Plain socket write
#ifdef _WIN32
    int n = send((SOCKET)fd, data, (int)len, 0);
#else
    ssize_t n = write((int)fd, data, len);
#endif
    return (int64_t)n;
}

void phprs_socket_close(int64_t fd) {
    if (fd >= 0) {
        phprs_tls_remove(fd);
#ifdef _WIN32
        closesocket((SOCKET)fd);
#else
        close((int)fd);
#endif
    }
}

// Forward declaration for exception handling (defined later)
void __throw(const char* message);

// ---- File I/O ----

static const char* phprs_read_file_contents(FILE* f) {
    fseek(f, 0, SEEK_END);
    long sz = ftell(f);
    if (sz < 0) { sz = 0; }
    fseek(f, 0, SEEK_SET);
    char* buf = (char*)malloc((size_t)sz + 1);
    if (!buf) return strdup("");
    size_t total = 0;
    while (total < (size_t)sz) {
        size_t n = fread(buf + total, 1, (size_t)sz - total, f);
        if (n == 0) break;
        total += n;
    }
    buf[total] = '\0';
    return buf;
}

const char* phprs_file_read(const char* path) {
    if (!path) return strdup("");
    FILE* f = fopen(path, "rb");
    if (!f) return strdup("");
    char* content = phprs_read_file_contents(f);
    fclose(f);
    return content;
}

int64_t phprs_file_write(const char* path, const char* content) {
    if (!path || !content) return -1;
    FILE* f = fopen(path, "wb");
    if (!f) return -1;
    size_t len = strlen(content);
    size_t n = fwrite(content, 1, len, f);
    fclose(f);
    return (int64_t)n;
}

int64_t phprs_file_exists(const char* path) {
    if (!path) return 0;
    FILE* f = fopen(path, "rb");
    if (f) { fclose(f); return 1; }
    return 0;
}

bool file_exists(const char* path) {
    return phprs_file_exists(path) ? true : false;
}

const char* file_get_contents(const char* path) {
    if (!path) {
        __throw("file_get_contents(): path is null");
        return strdup("");
    }
    FILE* f = fopen(path, "rb");
    if (!f) {
        char buf[512];
        snprintf(buf, sizeof(buf), "%s: %s", strerror(errno), path);
        __throw(buf);
        return strdup("");
    }
    char* content = phprs_read_file_contents(f);
    fclose(f);
    return content;
}

int64_t file_put_contents(const char* path, const char* content) {
    if (!path) {
        __throw("file_put_contents(): path is null");
        return -1;
    }
    if (!content) {
        __throw("file_put_contents(): content is null");
        return -1;
    }
    FILE* f = fopen(path, "wb");
    if (!f) {
        char buf[512];
        snprintf(buf, sizeof(buf), "%s: %s", strerror(errno), path);
        __throw(buf);
        return -1;
    }
    size_t len = strlen(content);
    size_t n = fwrite(content, 1, len, f);
    fclose(f);
    return (int64_t)n;
}

// ---- URL & Encoding Functions ----

const char* urlencode(const char* s) {
    if (!s) return strdup("");
    size_t len = strlen(s);
    char* r = malloc(len * 3 + 1);
    if (!r) return strdup("");
    size_t j = 0;
    for (size_t i = 0; i < len; i++) {
        unsigned char c = (unsigned char)s[i];
        if ((c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z') || (c >= '0' && c <= '9')
            || c == '-' || c == '_' || c == '.' || c == '~') {
            r[j++] = c;
        } else if (c == ' ') {
            r[j++] = '+';
        } else {
            snprintf(r + j, 4, "%%%02X", c);
            j += 3;
        }
    }
    r[j] = '\0';
    return r;
}

static int hex_val(char c) {
    if (c >= '0' && c <= '9') return c - '0';
    if (c >= 'A' && c <= 'F') return c - 'A' + 10;
    if (c >= 'a' && c <= 'f') return c - 'a' + 10;
    return -1;
}

const char* urldecode(const char* s) {
    if (!s) return strdup("");
    size_t len = strlen(s);
    char* r = malloc(len + 1);
    if (!r) return strdup("");
    size_t j = 0;
    for (size_t i = 0; i < len; i++) {
        if (s[i] == '+') {
            r[j++] = ' ';
        } else if (s[i] == '%' && i + 2 < len) {
            int h1 = hex_val(s[i + 1]);
            int h2 = hex_val(s[i + 2]);
            if (h1 >= 0 && h2 >= 0) {
                r[j++] = (char)((h1 << 4) | h2);
                i += 2;
            } else {
                r[j++] = '%';
            }
        } else {
            r[j++] = s[i];
        }
    }
    r[j] = '\0';
    return r;
}

const char* parse_url(const char* url) {
    // Returns a simple string representation: scheme=...&host=...&path=...&port=...&query=...
    // For integration with existing http_parse_url system
    if (!url) return strdup("");
    const char* proto = "";
    const char* host = "";
    const char* path = "/";
    const char* port_str = "";
    const char* query = "";

    const char* scheme_end = strstr(url, "://");
    char* proto_buf = NULL;
    char* host_buf = NULL;
    char* path_buf = NULL;
    char* query_buf = NULL;

    if (scheme_end) {
        size_t proto_len = scheme_end - url;
        proto_buf = malloc(proto_len + 1);
        memcpy(proto_buf, url, proto_len);
        proto_buf[proto_len] = '\0';
        proto = proto_buf;
        url = scheme_end + 3;
    }

    // Find host (up to / : ? or end)
    const char* p = url;
    while (*p && *p != '/' && *p != ':' && *p != '?') p++;
    size_t host_len = p - url;
    host_buf = malloc(host_len + 1);
    memcpy(host_buf, url, host_len);
    host_buf[host_len] = '\0';
    host = host_buf;

    url = p;
    if (*url == ':') {
        url++;
        // Parse port
        p = url;
        while (*p && *p != '/' && *p != '?') p++;
        size_t port_len = p - url;
        if (port_len > 0) {
            port_str = url;
        }
        url = p;
    }

    if (*url == '/') {
        const char* q = strchr(url, '?');
        if (q) {
            size_t path_len = q - url;
            path_buf = malloc(path_len + 1);
            memcpy(path_buf, url, path_len);
            path_buf[path_len] = '\0';
            path = path_buf;
            query_buf = strdup(q + 1);
            query = query_buf;
        } else {
            path_buf = strdup(url);
            path = path_buf;
        }
    } else if (*url == '?') {
        query_buf = strdup(url + 1);
        query = query_buf;
    }

    char result[2048];
    snprintf(result, sizeof(result), "proto=%s&host=%s&path=%s&port=%s&query=%s",
             proto, host, path, port_str, query);

    free(proto_buf);
    free(host_buf);
    free(path_buf);
    free(query_buf);
    return strdup(result);
}

const char* http_build_query(const char* query_string) {
    // Simple pass-through for already-formatted query strings, or format from structured input
    if (!query_string) return strdup("");
    return strdup(query_string);
}

static const char base64_chars[] = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

const char* base64_encode(const char* s) {
    if (!s) return strdup("");
    size_t len = strlen(s);
    size_t out_len = ((len + 2) / 3) * 4;
    char* r = malloc(out_len + 1);
    if (!r) return strdup("");
    size_t j = 0;
    for (size_t i = 0; i < len; i += 3) {
        unsigned char b0 = (unsigned char)s[i];
        unsigned char b1 = (i + 1 < len) ? (unsigned char)s[i + 1] : 0;
        unsigned char b2 = (i + 2 < len) ? (unsigned char)s[i + 2] : 0;
        unsigned int triple = (b0 << 16) | (b1 << 8) | b2;
        r[j++] = base64_chars[(triple >> 18) & 0x3F];
        r[j++] = base64_chars[(triple >> 12) & 0x3F];
        r[j++] = (i + 1 < len) ? base64_chars[(triple >> 6) & 0x3F] : '=';
        r[j++] = (i + 2 < len) ? base64_chars[triple & 0x3F] : '=';
    }
    r[j] = '\0';
    return r;
}

const char* base64_decode(const char* s) {
    if (!s) return strdup("");
    size_t len = strlen(s);
    char* r = malloc(len + 1);
    if (!r) return strdup("");
    size_t j = 0;
    unsigned int buf = 0;
    int bits = 0;
    for (size_t i = 0; i < len; i++) {
        char c = s[i];
        if (c == '=') break;
        int val = -1;
        if (c >= 'A' && c <= 'Z') val = c - 'A';
        else if (c >= 'a' && c <= 'z') val = c - 'a' + 26;
        else if (c >= '0' && c <= '9') val = c - '0' + 52;
        else if (c == '+') val = 62;
        else if (c == '/') val = 63;
        if (val < 0) continue;
        buf = (buf << 6) | val;
        bits += 6;
        if (bits >= 8) {
            bits -= 8;
            r[j++] = (char)((buf >> bits) & 0xFF);
        }
    }
    r[j] = '\0';
    return r;
}

// ---- Hash Functions ----

// MD5 implementation (RFC 1321)
const char* md5(const char* s) {
    if (!s) s = "";

    static const uint32_t K[64] = {
        0xD76AA478, 0xE8C7B756, 0x242070DB, 0xC1BDCEEE, 0xF57C0FAF, 0x4787C62A, 0xA8304613, 0xFD469501,
        0x698098D8, 0x8B44F7AF, 0xFFFF5BB1, 0x895CD7BE, 0x6B901122, 0xFD987193, 0xA679438E, 0x49B40821,
        0xF61E2562, 0xC040B340, 0x265E5A51, 0xE9B6C7AA, 0xD62F105D, 0x02441453, 0xD8A1E681, 0xE7D3FBC8,
        0x21E1CDE6, 0xC33707D6, 0xF4D50D87, 0x455A14ED, 0xA9E3E905, 0xFCEFA3F8, 0x676F02D9, 0x8D2A4C8A,
        0xFFFA3942, 0x8771F681, 0x6D9D6122, 0xFDE5380C, 0xA4BEEA44, 0x4BDECFA9, 0xF6BB4B60, 0xBEBFBC70,
        0x289B7EC6, 0xEAA127FA, 0xD4EF3085, 0x04881D05, 0xD9D4D039, 0xE6DB99E5, 0x1FA27CF8, 0xC4AC5665,
        0xF4292244, 0x432AFF97, 0xAB9423A7, 0xFC93A039, 0x655B59C3, 0x8F0CCC92, 0xFFEFF47D, 0x85845DD1,
        0x6FA87E4F, 0xFE2CE6E0, 0xA3014314, 0x4E0811A1, 0xF7537E82, 0xBD3AF235, 0x2AD7D2BB, 0xEB86D391,
    };

    static const uint8_t S[64] = {
        7,12,17,22, 7,12,17,22, 7,12,17,22, 7,12,17,22,
        5, 9,14,20, 5, 9,14,20, 5, 9,14,20, 5, 9,14,20,
        4,11,16,23, 4,11,16,23, 4,11,16,23, 4,11,16,23,
        6,10,15,21, 6,10,15,21, 6,10,15,21, 6,10,15,21
    };

    size_t orig_len = strlen(s);
    size_t pad_len = orig_len;
    size_t total_len;

    // Padding: append 0x80, then zeros until (len % 64) == 56, then 8-byte length in little-endian
    pad_len++;
    while ((pad_len % 64) != 56) pad_len++;
    total_len = pad_len + 8;

    uint8_t* msg = (uint8_t*)calloc(1, total_len);
    if (!msg) return strdup("");
    memcpy(msg, s, orig_len);
    msg[orig_len] = 0x80;

    // Append original length in bits as 64-bit little-endian
    uint64_t bits = (uint64_t)orig_len * 8;
    memcpy(msg + pad_len, &bits, 8);

    uint32_t a = 0x67452301;
    uint32_t b = 0xEFCDAB89;
    uint32_t c = 0x98BADCFE;
    uint32_t d = 0x10325476;

    for (size_t chunk_off = 0; chunk_off < total_len; chunk_off += 64) {
        uint32_t M[16];
        for (int i = 0; i < 16; i++) {
            size_t off = chunk_off + (size_t)i * 4;
            M[i] = (uint32_t)msg[off] | ((uint32_t)msg[off+1] << 8) |
                   ((uint32_t)msg[off+2] << 16) | ((uint32_t)msg[off+3] << 24);
        }

        uint32_t aa = a, bb = b, cc = c, dd = d;

        for (int i = 0; i < 64; i++) {
            uint32_t f;
            int g;
            if (i < 16) {
                f = (bb & cc) | ((~bb) & dd);
                g = i;
            } else if (i < 32) {
                f = (bb & dd) | (cc & (~dd));
                g = (5 * i + 1) % 16;
            } else if (i < 48) {
                f = bb ^ cc ^ dd;
                g = (3 * i + 5) % 16;
            } else {
                f = cc ^ (bb | (~dd));
                g = (7 * i) % 16;
            }

            uint32_t temp = dd;
            dd = cc;
            cc = bb;
            bb = bb + ((aa + f + K[i] + M[g]) << S[i] | (aa + f + K[i] + M[g]) >> (32 - S[i]));
            aa = temp;
        }

        a += aa; b += bb; c += cc; d += dd;
    }

    free(msg);

    // Format as 32-char lowercase hex string
    char* result = (char*)malloc(33);
    if (!result) return strdup("");
    snprintf(result, 33, "%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x",
        (uint8_t)(a), (uint8_t)(a>>8), (uint8_t)(a>>16), (uint8_t)(a>>24),
        (uint8_t)(b), (uint8_t)(b>>8), (uint8_t)(b>>16), (uint8_t)(b>>24),
        (uint8_t)(c), (uint8_t)(c>>8), (uint8_t)(c>>16), (uint8_t)(c>>24),
        (uint8_t)(d), (uint8_t)(d>>8), (uint8_t)(d>>16), (uint8_t)(d>>24));
    return result;
}

// SHA1 implementation (FIPS 180-1)
const char* sha1(const char* s) {
    if (!s) s = "";

    size_t orig_len = strlen(s);
    size_t pad_len = orig_len;

    // Padding: append 0x80, then zeros until (len % 64) == 56, then 8-byte length in big-endian
    pad_len++;
    while ((pad_len % 64) != 56) pad_len++;
    size_t total_len = pad_len + 8;

    uint8_t* msg = (uint8_t*)calloc(1, total_len);
    if (!msg) return strdup("");
    memcpy(msg, s, orig_len);
    msg[orig_len] = 0x80;

    // Append original length in bits as 64-bit big-endian
    uint64_t bits = (uint64_t)orig_len * 8;
    for (int i = 7; i >= 0; i--) {
        msg[pad_len + 7 - i] = (uint8_t)(bits >> (i * 8));
    }

    uint32_t h[5] = {0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0};

    for (size_t chunk_off = 0; chunk_off < total_len; chunk_off += 64) {
        uint32_t w[80];

        for (int i = 0; i < 16; i++) {
            size_t off = chunk_off + (size_t)i * 4;
            w[i] = ((uint32_t)msg[off] << 24) | ((uint32_t)msg[off+1] << 16) |
                   ((uint32_t)msg[off+2] << 8) | (uint32_t)msg[off+3];
        }

        for (int i = 16; i < 80; i++) {
            w[i] = (w[i-3] ^ w[i-8] ^ w[i-14] ^ w[i-16]);
            w[i] = (w[i] << 1) | (w[i] >> 31); // rotate left by 1
        }

        uint32_t a = h[0], b = h[1], c = h[2], d = h[3], e = h[4];

        for (int i = 0; i < 80; i++) {
            uint32_t f, k;
            if (i < 20) {
                f = (b & c) | ((~b) & d);
                k = 0x5A827999;
            } else if (i < 40) {
                f = b ^ c ^ d;
                k = 0x6ED9EBA1;
            } else if (i < 60) {
                f = (b & c) | (b & d) | (c & d);
                k = 0x8F1BBCDC;
            } else {
                f = b ^ c ^ d;
                k = 0xCA62C1D6;
            }

            uint32_t temp = ((a << 5) | (a >> 27)) + f + e + k + w[i];
            e = d;
            d = c;
            c = (b << 30) | (b >> 2); // rotate left by 30
            b = a;
            a = temp;
        }

        h[0] += a; h[1] += b; h[2] += c; h[3] += d; h[4] += e;
    }

    free(msg);

    char* result = (char*)malloc(41);
    if (!result) return strdup("");
    snprintf(result, 41, "%08x%08x%08x%08x%08x", h[0], h[1], h[2], h[3], h[4]);
    return result;
}

// ---- Misc Functions ----
const char* uniqid(const char* prefix) {
    char buf[64];
    snprintf(buf, sizeof(buf), "%s%lu", prefix ? prefix : "", (unsigned long)time(NULL));
    return strdup(buf);
}
void sleep_(int64_t seconds) {
    if (seconds < 0) seconds = 0;
#ifdef _WIN32
    Sleep((DWORD)(seconds * 1000));
#else
    sleep((unsigned int)seconds);
#endif
}
void usleep_(int64_t microseconds) {
    if (microseconds < 0) microseconds = 0;
#ifdef _WIN32
    Sleep((DWORD)(microseconds / 1000));
#else
    usleep((useconds_t)microseconds);
#endif
}
const char* realpath_(const char* path) {
    if (!path) return strdup("");
#ifdef _WIN32
    char resolved[MAX_PATH];
    if (_fullpath(resolved, path, MAX_PATH)) return strdup(resolved);
    return strdup(path);
#else
    char* r = realpath(path, NULL);
    if (r) { char* dup = strdup(r); free(r); return dup; }
    return strdup(path);
#endif
}
int64_t is_file(const char* path) {
    if (!path) return 0;
#ifdef _WIN32
    DWORD attr = GetFileAttributesA(path);
    return (attr != INVALID_FILE_ATTRIBUTES && !(attr & FILE_ATTRIBUTE_DIRECTORY)) ? 1 : 0;
#else
    struct stat st;
    return (stat(path, &st) == 0 && S_ISREG(st.st_mode)) ? 1 : 0;
#endif
}

bool is_dir(const char* path) {
    if (!path) return false;
#ifdef _WIN32
    DWORD attr = GetFileAttributesA(path);
    return (attr != INVALID_FILE_ATTRIBUTES && (attr & FILE_ATTRIBUTE_DIRECTORY)) ? true : false;
#else
    struct stat st;
    return (stat(path, &st) == 0 && S_ISDIR(st.st_mode)) ? true : false;
#endif
}

bool mkdir_(const char* path) {
    if (!path) return false;
#ifdef _WIN32
    return CreateDirectoryA(path, NULL) ? true : false;
#else
    return mkdir(path, 0755) == 0 ? true : false;
#endif
}

bool unlink_(const char* path) {
    if (!path) return false;
#ifdef _WIN32
    return DeleteFileA(path) ? true : false;
#else
    return unlink(path) == 0 ? true : false;
#endif
}

const char* basename_(const char* path) {
    if (!path) return strdup("");
    const char* last = path;
    const char* p = path;
    while (*p) {
        if (*p == '/' || *p == '\\') {
            const char* next = p + 1;
            if (*next) last = next;
        }
        p++;
    }
    // Strip trailing slashes from last
    size_t len = strlen(last);
    while (len > 0 && (last[len - 1] == '/' || last[len - 1] == '\\')) len--;
    char* r = malloc(len + 1);
    memcpy(r, last, len);
    r[len] = '\0';
    return r;
}

const char* dirname_(const char* path) {
    if (!path) return strdup("");
    size_t len = strlen(path);
    // Strip trailing separators
    while (len > 1 && (path[len - 1] == '/' || path[len - 1] == '\\')) len--;
    // Find last separator
    const char* last_sep = NULL;
    for (size_t i = 0; i < len; i++) {
        if (path[i] == '/' || path[i] == '\\') last_sep = path + i;
    }
    if (!last_sep) return strdup(".");
    size_t dlen = (size_t)(last_sep - path);
    if (dlen == 0) return strdup("/");
    // Strip trailing separators from dirname
    while (dlen > 1 && (path[dlen - 1] == '/' || path[dlen - 1] == '\\')) dlen--;
    char* r = malloc(dlen + 1);
    memcpy(r, path, dlen);
    r[dlen] = '\0';
    return r;
}

// Returns serialized array: "count\0entry1\0entry2\0..."
const char* scandir_(const char* path) {
    if (!path) { char* r = malloc(10); snprintf(r, 10, "0"); return r; }
#ifdef _WIN32
    WIN32_FIND_DATAA fd;
    char search_path[MAX_PATH];
    snprintf(search_path, sizeof(search_path), "%s\\*", path);
    HANDLE h = FindFirstFileA(search_path, &fd);
    if (h == INVALID_HANDLE_VALUE) { char* r = malloc(10); snprintf(r, 10, "0"); return r; }
    // First pass: count entries
    size_t count = 0;
    size_t total = 0;
    do {
        if (strcmp(fd.cFileName, ".") == 0 || strcmp(fd.cFileName, "..") == 0) continue;
        count++;
        total += strlen(fd.cFileName) + 1;
    } while (FindNextFileA(h, &fd));
    FindClose(h);
    // Second pass: build result
    char* r = malloc(total + 20);
    int off = snprintf(r, 20, "%zu", count);
    h = FindFirstFileA(search_path, &fd);
    if (h == INVALID_HANDLE_VALUE) { r[0] = '0'; r[1] = '\0'; return r; }
    do {
        if (strcmp(fd.cFileName, ".") == 0 || strcmp(fd.cFileName, "..") == 0) continue;
        r[off++] = '\0';
        size_t elen = strlen(fd.cFileName);
        memcpy(r + off, fd.cFileName, elen);
        off += (int)elen;
    } while (FindNextFileA(h, &fd));
    FindClose(h);
    r[off] = '\0';
    return r;
#else
    DIR* dir = opendir(path);
    if (!dir) { char* r = malloc(10); snprintf(r, 10, "0"); return r; }
    // First pass: count
    size_t count = 0;
    size_t total = 0;
    struct dirent* entry;
    while ((entry = readdir(dir)) != NULL) {
        if (strcmp(entry->d_name, ".") == 0 || strcmp(entry->d_name, "..") == 0) continue;
        count++;
        total += strlen(entry->d_name) + 1;
    }
    rewinddir(dir);
    // Build result
    char* r = malloc(total + 20);
    int off = snprintf(r, 20, "%zu", count);
    while ((entry = readdir(dir)) != NULL) {
        if (strcmp(entry->d_name, ".") == 0 || strcmp(entry->d_name, "..") == 0) continue;
        r[off++] = '\0';
        size_t elen = strlen(entry->d_name);
        memcpy(r + off, entry->d_name, elen);
        off += (int)elen;
    }
    closedir(dir);
    r[off] = '\0';
    return r;
#endif
}
const char* getcwd_(void) {
    char buf[4096];
#ifdef _WIN32
    return _getcwd(buf, sizeof(buf)) ? strdup(buf) : strdup(".");
#else
    return getcwd(buf, sizeof(buf)) ? strdup(buf) : strdup(".");
#endif
}

// ---- Advanced Array Functions ----
// These are simplified C stubs; full implementations in interpreter mode

int64_t array_diff(int64_t* a, int64_t a_len, int64_t* b, int64_t b_len) {
    (void)a; (void)a_len; (void)b; (void)b_len;
    return 0; // stub — returns count of diff
}

int64_t array_combine(int64_t* keys, int64_t k_len, int64_t* vals, int64_t v_len) {
    (void)keys; (void)k_len; (void)vals; (void)v_len;
    return 0; // stub
}

int64_t array_column(int64_t* rows, int64_t r_len, const char* col) {
    (void)rows; (void)r_len; (void)col;
    return 0; // stub
}

int64_t array_fill(int64_t start, int64_t count, int64_t value) {
    (void)start; (void)count; (void)value;
    return 0; // stub
}

int64_t array_rand(int64_t* arr, int64_t len, int64_t count) {
    (void)arr; (void)len;
    if (count <= 1) return 0;
    return 0; // stub
}

// ---- Debug Output ----

void var_dump(const char* type_tag, const char* value) {
    if (!type_tag) type_tag = "unknown";
    if (!value) value = "";
    printf("%s(%s)\n", type_tag, value);
}

void print_r(const char* value) {
    if (!value) value = "";
    printf("%s\n", value);
}

// ---- HTTP Parsing ----

static const char* phprs_copy_until(const char* src, char delim) {
    const char* end = strchr(src, delim);
    if (!end) return strdup(src);
    size_t len = (size_t)(end - src);
    char* r = (char*)malloc(len + 1);
    memcpy(r, src, len);
    r[len] = '\0';
    return r;
}

const char* phprs_http_method(const char* raw) {
    if (!raw) return strdup("");
    return phprs_copy_until(raw, ' ');
}

const char* phprs_http_path(const char* raw) {
    if (!raw) return strdup("");
    const char* after_method = strchr(raw, ' ');
    if (!after_method) return strdup("/");
    after_method++;
    return phprs_copy_until(after_method, ' ');
}

static int phprs_casecmp(const char* a, const char* b) {
    while (*a && *b) {
        char ca = (*a >= 'A' && *a <= 'Z') ? (*a + 32) : *a;
        char cb = (*b >= 'A' && *b <= 'Z') ? (*b + 32) : *b;
        if (ca != cb) return 0;
        a++; b++;
    }
    return *b == '\0';
}

const char* phprs_http_header(const char* raw, const char* name) {
    if (!raw || !name) return strdup("");
    // Find the header name followed by ": "
    const char* p = raw;
    // Skip to first \r\n (end of request line)
    p = strstr(p, "\r\n");
    if (!p) return strdup("");
    p += 2;

    while (*p) {
        if (phprs_casecmp(p, name) && p[strlen(name)] == ':') {
            p += strlen(name) + 1;
            while (*p == ' ') p++;
            return phprs_copy_until(p, '\r');
        }
        p = strstr(p, "\r\n");
        if (!p) break;
        p += 2;
    }
    return strdup("");
}

const char* phprs_http_body(const char* raw) {
    if (!raw) return strdup("");
    const char* body_start = strstr(raw, "\r\n\r\n");
    if (!body_start) return strdup("");
    body_start += 4;
    return strdup(body_start);
}

const char* phprs_url_decode(const char* encoded) {
    if (!encoded) return strdup("");
    char* r = (char*)malloc(strlen(encoded) + 1);
    char* out = r;
    while (*encoded) {
        if (*encoded == '%' && isxdigit((unsigned char)encoded[1]) && isxdigit((unsigned char)encoded[2])) {
            char hex[3] = { encoded[1], encoded[2], '\0' };
            *out++ = (char)strtol(hex, NULL, 16);
            encoded += 3;
        } else if (*encoded == '+') {
            *out++ = ' ';
            encoded++;
        } else {
            *out++ = *encoded++;
        }
    }
    *out = '\0';
    return r;
}

// Parse a raw HTTP request into a unified flat params string.
// Returns "method=GET&path=/api/user&user_id=42&name=test&body=...&content_type=...&host=..."
// Query params and form-urlencoded POST params are merged into the flat namespace.
// System fields (always present): method, path, body, content_type, host
const char* phprs_request_parse(const char* raw) {
    if (!raw) return strdup("");

    // Use a dynamic buffer for output
    size_t cap = 4096;
    char* out = (char*)malloc(cap);
    if (!out) return strdup("");
    out[0] = '\0';
    size_t len = 0;

    // Helper: append key=val to out
    #define REQ_APPEND(k, v) do { \
        const char* _k = (k); const char* _v = (v) ? (v) : ""; \
        size_t need = len + strlen(_k) + 1 + strlen(_v) + 1; \
        if (need > cap) { cap = need + 1024; char* n = (char*)realloc(out, cap); if (!n) { free(out); return strdup(""); } out = n; } \
        if (len > 0) { out[len++] = '&'; } \
        len += sprintf(out + len, "%s=%s", _k, _v); \
    } while(0)

    // 1. Method
    const char* method = phprs_http_method(raw);
    REQ_APPEND("method", method);

    // 2. Path (full, from request line)
    const char* full_path = phprs_http_path(raw);
    const char* path_only = full_path;
    const char* query_str = "";

    // Split path and query
    const char* qmark = strchr(full_path, '?');
    if (qmark) {
        size_t path_len = (size_t)(qmark - full_path);
        char* p = (char*)malloc(path_len + 1);
        memcpy(p, full_path, path_len);
        p[path_len] = '\0';
        path_only = p;
        query_str = qmark + 1;
    } else {
        path_only = strdup(full_path);
    }
    REQ_APPEND("path", path_only);

    // 3. Parse query string into flat params
    if (query_str && *query_str) {
        char* qs = strdup(query_str);
        char* save = NULL;
        char* tok = strtok_r(qs, "&", &save);
        while (tok) {
            char* eq = strchr(tok, '=');
            if (eq) {
                *eq = '\0';
                const char* val = eq + 1;
                REQ_APPEND(tok, val);
            }
            tok = strtok_r(NULL, "&", &save);
        }
        free(qs);
    }

    // 4. Body
    const char* body = phprs_http_body(raw);
    REQ_APPEND("body", body);

    // 5. Content-Type header
    const char* content_type = phprs_http_header(raw, "Content-Type");
    REQ_APPEND("content_type", content_type);

    // 6. Host header
    const char* host = phprs_http_header(raw, "Host");
    REQ_APPEND("host", host);

    // 7. If body is form-urlencoded, parse and merge params
    if (content_type && strstr(content_type, "x-www-form-urlencoded") && body && *body) {
        char* bs = strdup(body);
        char* save = NULL;
        char* tok = strtok_r(bs, "&", &save);
        while (tok) {
            char* eq = strchr(tok, '=');
            if (eq) {
                *eq = '\0';
                const char* val = eq + 1;
                REQ_APPEND(tok, val);
            }
            tok = strtok_r(NULL, "&", &save);
        }
        free(bs);
    }

    #undef REQ_APPEND
    return out;
}

// Strip CR/LF from a string in-place to prevent HTTP header injection.
static void strip_crlf(char* s) {
    if (!s) return;
    char* dst = s;
    for (char* src = s; *src; src++) {
        if (*src != '\r' && *src != '\n') {
            *dst++ = *src;
        }
    }
    *dst = '\0';
}

const char* phprs_http_response(int64_t status_code, const char* content_type, const char* body) {
    if (!content_type) content_type = "text/html";
    if (!body) body = "";

    // Sanitize content_type to prevent HTTP response splitting
    char* safe_ct = strdup(content_type);
    if (safe_ct) { strip_crlf(safe_ct); content_type = safe_ct; }

    const char* status_text = "OK";
    switch ((int)status_code) {
        case 200: status_text = "OK"; break;
        case 201: status_text = "Created"; break;
        case 204: status_text = "No Content"; break;
        case 301: status_text = "Moved Permanently"; break;
        case 302: status_text = "Found"; break;
        case 400: status_text = "Bad Request"; break;
        case 401: status_text = "Unauthorized"; break;
        case 403: status_text = "Forbidden"; break;
        case 404: status_text = "Not Found"; break;
        case 405: status_text = "Method Not Allowed"; break;
        case 500: status_text = "Internal Server Error"; break;
        default: status_text = "OK"; break;
    }

    size_t body_len = strlen(body);
    // Rough estimate for headers
    size_t buf_size = 256 + body_len + strlen(content_type);
    char* r = (char*)malloc(buf_size + 1);
    int n = snprintf(r, buf_size + 1,
        "HTTP/1.1 %lld %s\r\n"
        "Content-Type: %s\r\n"
        "Content-Length: %zu\r\n"
        "Connection: close\r\n"
        "\r\n"
        "%s",
        (long long)status_code, status_text,
        content_type,
        body_len,
        body);
    // If snprintf truncated, re-allocate (unlikely)
    if ((size_t)n >= buf_size + 1) {
        char* r2 = (char*)malloc((size_t)n + 1);
        snprintf(r2, (size_t)n + 1,
            "HTTP/1.1 %lld %s\r\n"
            "Content-Type: %s\r\n"
            "Content-Length: %zu\r\n"
            "Connection: close\r\n"
            "\r\n"
            "%s",
            (long long)status_code, status_text,
            content_type,
            body_len,
            body);
        free(r);
        free(safe_ct);
        return r2;
    }
    free(safe_ct);
    return r;
}

// ---- Minimal JSON Helpers (flat objects only) ----

const char* phprs_json_get_string(const char* json, const char* key) {
    if (!json || !key) return strdup("");

    // Build search pattern: "key"
    size_t key_len = strlen(key);
    size_t pattern_len = key_len + 4;
    char* pattern = (char*)malloc(pattern_len + 1);
    snprintf(pattern, pattern_len + 1, "\"%s\"", key);
    const char* pos = strstr(json, pattern);
    free(pattern);
    if (!pos) return strdup("");

    // Skip past "key"
    pos += key_len + 2;
    while (*pos == ' ' || *pos == ':') pos++;
    if (*pos != '"') return strdup("");

    pos++; // skip opening quote
    return phprs_copy_until(pos, '"');
}

int64_t phprs_json_get_int(const char* json, const char* key) {
    char* s = phprs_json_get_string(json, key);
    if (!s || !*s) { free(s); return 0; }
    int64_t val = (int64_t)strtoll(s, NULL, 10);
    free(s);
    return val;
}

// ---- Full JSON Encode / Decode ----

static void json_escape_string(const char* s, char* out, size_t* out_len) {
    size_t j = 0;
    size_t len = strlen(s);
    for (size_t i = 0; i < len; i++) {
        unsigned char c = (unsigned char)s[i];
        switch (c) {
            case '"':  out[j++] = '\\'; out[j++] = '"'; break;
            case '\\': out[j++] = '\\'; out[j++] = '\\'; break;
            case '\n': out[j++] = '\\'; out[j++] = 'n'; break;
            case '\r': out[j++] = '\\'; out[j++] = 'r'; break;
            case '\t': out[j++] = '\\'; out[j++] = 't'; break;
            default:
                if (c < 0x20) {
                    j += sprintf(out + j, "\\u%04x", c);
                } else {
                    out[j++] = (char)c;
                }
        }
    }
    out[j] = '\0';
    *out_len = j;
}

// json_encode: Simple JSON encoder
// Input is a string representation. Returns JSON string.
// For strings: wraps in quotes and escapes. For everything else: passes through.
const char* json_encode(const char* value) {
    if (!value) return strdup("null");

    // Check if it's already formatted JSON (starts with { or [)
    if (*value == '{' || *value == '[') return strdup(value);

    // Check for null / bool-like values
    if (strcmp(value, "null") == 0) return strdup("null");
    if (strcmp(value, "true") == 0) return strdup("true");
    if (strcmp(value, "false") == 0) return strdup("false");

    // Try to detect number: all digits, optionally with . and -
    {
        const char* p = value;
        if (*p == '-') p++;
        int has_dot = 0;
        int is_num = (*p >= '0' && *p <= '9');
        while (*p) {
            if (*p >= '0' && *p <= '9') { p++; continue; }
            if (*p == '.' && !has_dot) { has_dot = 1; p++; continue; }
            is_num = 0;
            break;
        }
        if (is_num && *p == '\0') return strdup(value);
    }

    // Default: treat as string
    size_t len = strlen(value);
    char* r = malloc(len * 6 + 3); // worst-case: all chars need \uXXXX
    if (!r) return strdup("\"\"");
    r[0] = '"';
    size_t esc_len;
    json_escape_string(value, r + 1, &esc_len);
    r[1 + esc_len] = '"';
    r[1 + esc_len + 1] = '\0';
    return r;
}

// json_decode: Simple JSON decoder
// Takes a JSON string, returns the parsed value as a string.
// For JSON strings: returns the unescaped content.
// For JSON numbers: returns the number as string.
// For JSON objects/arrays: returns the JSON text as-is (simplified).
const char* json_decode(const char* json) {
    if (!json) return strdup("null");

    // Skip leading whitespace
    while (*json == ' ' || *json == '\t' || *json == '\n' || *json == '\r') json++;

    if (*json == '"') {
        // String value — extract and unescape
        json++;
        size_t len = strlen(json);
        char* r = malloc(len + 1);
        if (!r) return strdup("");
        size_t j = 0;
        for (size_t i = 0; i < len && json[i] != '\0'; i++) {
            if (json[i] == '\\' && i + 1 < len) {
                i++;
                switch (json[i]) {
                    case '"':  r[j++] = '"'; break;
                    case '\\': r[j++] = '\\'; break;
                    case '/':  r[j++] = '/'; break;
                    case 'n':  r[j++] = '\n'; break;
                    case 'r':  r[j++] = '\r'; break;
                    case 't':  r[j++] = '\t'; break;
                    case 'u': {
                        unsigned int cp = 0;
                        for (int k = 1; k <= 4 && i + k < len; k++) {
                            char hc = json[i + k];
                            cp <<= 4;
                            if (hc >= '0' && hc <= '9') cp += hc - '0';
                            else if (hc >= 'a' && hc <= 'f') cp += hc - 'a' + 10;
                            else if (hc >= 'A' && hc <= 'F') cp += hc - 'A' + 10;
                        }
                        i += 4;
                        if (cp <= 0x7F) {
                            r[j++] = (char)cp;
                        } else if (cp <= 0x7FF) {
                            r[j++] = (char)(0xC0 | (cp >> 6));
                            r[j++] = (char)(0x80 | (cp & 0x3F));
                        } else {
                            r[j++] = (char)(0xE0 | (cp >> 12));
                            r[j++] = (char)(0x80 | ((cp >> 6) & 0x3F));
                            r[j++] = (char)(0x80 | (cp & 0x3F));
                        }
                        break;
                    }
                    default: r[j++] = json[i]; break;
                }
            } else if (json[i] == '"') {
                break;
            } else {
                r[j++] = json[i];
            }
        }
        r[j] = '\0';
        return r;
    }

    // For numbers, booleans, null, objects, arrays: return as-is
    return strdup(json);
}

// ---- String Helpers ----

const char* phprs_str_replace(const char* s, const char* from, const char* to) {
    if (!s) return strdup("");
    if (!from || !*from) return strdup(s);
    if (!to) to = "";

    size_t s_len = strlen(s);
    size_t from_len = strlen(from);
    size_t to_len = strlen(to);

    // Count occurrences
    size_t count = 0;
    const char* tmp = s;
    while ((tmp = strstr(tmp, from)) != NULL) {
        count++;
        tmp += from_len;
    }

    size_t result_len = s_len + count * (to_len > from_len ? to_len - from_len : 0) + 1;
    // Allocate generously in case to_len < from_len (we'll over-allocate)
    result_len = result_len > s_len + 1 ? result_len : s_len + 1;
    char* r = (char*)malloc(result_len);
    char* out = r;

    while (*s) {
        if (strncmp(s, from, from_len) == 0) {
            memcpy(out, to, to_len);
            out += to_len;
            s += from_len;
        } else {
            *out++ = *s++;
        }
    }
    *out = '\0';
    return r;
}

int64_t phprs_str_contains(const char* haystack, const char* needle) {
    if (!haystack || !needle) return 0;
    return strstr(haystack, needle) ? 1 : 0;
}

const char* phprs_str_split(const char* s, const char* delim, int64_t index) {
    if (!s || !delim || !*delim) return strdup("");
    size_t delim_len = strlen(delim);
    int64_t current = 0;
    const char* start = s;

    while (*start) {
        if (current == index) {
            const char* end = strstr(start, delim);
            if (end) {
                size_t len = (size_t)(end - start);
                char* r = (char*)malloc(len + 1);
                memcpy(r, start, len);
                r[len] = '\0';
                return r;
            } else {
                return strdup(start);
            }
        }
        start = strstr(start, delim);
        if (!start) break;
        start += delim_len;
        current++;
    }
    return strdup("");
}

int64_t phprs_str_starts_with(const char* s, const char* prefix) {
    if (!s || !prefix) return 0;
    size_t prefix_len = strlen(prefix);
    return strncmp(s, prefix, prefix_len) == 0 ? 1 : 0;
}

int64_t phprs_str_ends_with(const char* s, const char* suffix) {
    if (!s || !suffix) return 0;
    size_t s_len = strlen(s);
    size_t suf_len = strlen(suffix);
    if (suf_len > s_len) return 0;
    return strcmp(s + s_len - suf_len, suffix) == 0 ? 1 : 0;
}

const char* phprs_str_upper(const char* s) {
    if (!s) return strdup("");
    char* r = strdup(s);
    for (char* p = r; *p; p++) {
        if (*p >= 'a' && *p <= 'z') *p -= 32;
    }
    return r;
}

const char* phprs_str_lower(const char* s) {
    if (!s) return strdup("");
    char* r = strdup(s);
    for (char* p = r; *p; p++) {
        if (*p >= 'A' && *p <= 'Z') *p += 32;
    }
    return r;
}

// ---- SHA-1 Hash (FIPS PUB 180-4) ----

struct phprs_sha1_ctx {
    uint32_t state[5];
    uint64_t count;
    unsigned char buffer[64];
    size_t buffer_len;
};

#define ROTL32(x, n) (((x) << (n)) | ((x) >> (32 - (n))))

static void phprs_sha1_transform(struct phprs_sha1_ctx* ctx) {
    uint32_t W[80];
    for (int t = 0; t < 16; t++) {
        W[t] = ((uint32_t)ctx->buffer[t * 4] << 24) |
               ((uint32_t)ctx->buffer[t * 4 + 1] << 16) |
               ((uint32_t)ctx->buffer[t * 4 + 2] << 8) |
               ((uint32_t)ctx->buffer[t * 4 + 3]);
    }
    for (int t = 16; t < 80; t++) {
        W[t] = ROTL32(W[t - 3] ^ W[t - 8] ^ W[t - 14] ^ W[t - 16], 1);
    }

    uint32_t a = ctx->state[0], b = ctx->state[1], c = ctx->state[2],
             d = ctx->state[3], e = ctx->state[4];

    for (int t = 0; t < 80; t++) {
        uint32_t f, k;
        if (t < 20) {
            f = (b & c) | (~b & d);
            k = 0x5A827999;
        } else if (t < 40) {
            f = b ^ c ^ d;
            k = 0x6ED9EBA1;
        } else if (t < 60) {
            f = (b & c) | (b & d) | (c & d);
            k = 0x8F1BBCDC;
        } else {
            f = b ^ c ^ d;
            k = 0xCA62C1D6;
        }
        uint32_t temp = ROTL32(a, 5) + f + e + k + W[t];
        e = d; d = c; c = ROTL32(b, 30); b = a; a = temp;
    }

    ctx->state[0] += a; ctx->state[1] += b; ctx->state[2] += c;
    ctx->state[3] += d; ctx->state[4] += e;
}

static void phprs_sha1_update(struct phprs_sha1_ctx* ctx, const unsigned char* data, size_t len) {
    ctx->count += len;
    if (ctx->buffer_len > 0) {
        size_t fill = 64 - ctx->buffer_len;
        if (len < fill) fill = len;
        memcpy(ctx->buffer + ctx->buffer_len, data, fill);
        ctx->buffer_len += fill;
        data += fill; len -= fill;
        if (ctx->buffer_len == 64) {
            phprs_sha1_transform(ctx);
            ctx->buffer_len = 0;
        }
    }
    while (len >= 64) {
        memcpy(ctx->buffer, data, 64);
        phprs_sha1_transform(ctx);
        data += 64; len -= 64;
    }
    if (len > 0) {
        memcpy(ctx->buffer, data, len);
        ctx->buffer_len = len;
    }
}

static void phprs_sha1_final(struct phprs_sha1_ctx* ctx, unsigned char output[20]) {
    ctx->buffer[ctx->buffer_len++] = 0x80;
    if (ctx->buffer_len > 56) {
        while (ctx->buffer_len < 64) ctx->buffer[ctx->buffer_len++] = 0;
        phprs_sha1_transform(ctx);
        ctx->buffer_len = 0;
    }
    while (ctx->buffer_len < 56) ctx->buffer[ctx->buffer_len++] = 0;
    uint64_t bits = ctx->count * 8;
    ctx->buffer[56] = (unsigned char)(bits >> 56);
    ctx->buffer[57] = (unsigned char)(bits >> 48);
    ctx->buffer[58] = (unsigned char)(bits >> 40);
    ctx->buffer[59] = (unsigned char)(bits >> 32);
    ctx->buffer[60] = (unsigned char)(bits >> 24);
    ctx->buffer[61] = (unsigned char)(bits >> 16);
    ctx->buffer[62] = (unsigned char)(bits >> 8);
    ctx->buffer[63] = (unsigned char)(bits);
    phprs_sha1_transform(ctx);
    for (int i = 0; i < 5; i++) {
        output[i * 4]     = (unsigned char)(ctx->state[i] >> 24);
        output[i * 4 + 1] = (unsigned char)(ctx->state[i] >> 16);
        output[i * 4 + 2] = (unsigned char)(ctx->state[i] >> 8);
        output[i * 4 + 3] = (unsigned char)(ctx->state[i]);
    }
}

static void phprs_sha1(const unsigned char* input, size_t len, unsigned char output[20]) {
    struct phprs_sha1_ctx ctx;
    ctx.state[0] = 0x67452301;
    ctx.state[1] = 0xEFCDAB89;
    ctx.state[2] = 0x98BADCFE;
    ctx.state[3] = 0x10325476;
    ctx.state[4] = 0xC3D2E1F0;
    ctx.count = 0;
    ctx.buffer_len = 0;
    phprs_sha1_update(&ctx, input, len);
    phprs_sha1_final(&ctx, output);
}

// ---- Base64 Encoding ----

static const char* phprs_base64_alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

static char* phprs_base64_encode(const unsigned char* input, size_t len) {
    size_t out_len = ((len + 2) / 3) * 4;
    char* output = (char*)malloc(out_len + 1);
    if (!output) return strdup("");
    size_t out = 0;
    for (size_t i = 0; i < len; i += 3) {
        unsigned char b0 = input[i];
        unsigned char b1 = (i + 1 < len) ? input[i + 1] : 0;
        unsigned char b2 = (i + 2 < len) ? input[i + 2] : 0;
        output[out++] = phprs_base64_alphabet[(b0 >> 2) & 0x3F];
        output[out++] = phprs_base64_alphabet[((b0 << 4) | (b1 >> 4)) & 0x3F];
        output[out++] = (i + 1 < len) ? phprs_base64_alphabet[((b1 << 2) | (b2 >> 6)) & 0x3F] : '=';
        output[out++] = (i + 2 < len) ? phprs_base64_alphabet[b2 & 0x3F] : '=';
    }
    output[out] = '\0';
    return output;
}

// ---- WebSocket Support ----

// Read exactly len bytes from socket, looping until complete or error.
// Returns total bytes read (may be < len on error/disconnect).
static int phprs_ws_read_exact(int64_t fd, unsigned char* buf, int len) {
    int total = 0;
    while (total < len) {
        int n;
#ifdef _WIN32
        n = recv((SOCKET)fd, (char*)buf + total, len - total, 0);
#else
        n = (int)recv((int)fd, buf + total, (size_t)(len - total), 0);
#endif
        if (n <= 0) return total;
        total += n;
    }
    return total;
}

int64_t phprs_is_websocket_upgrade(const char* raw) {
    if (!raw) return 0;
    const char* upgrade = phprs_http_header(raw, "Upgrade");
    int64_t result = 0;
    if (upgrade && *upgrade) {
        size_t len = strlen(upgrade);
        char* lower = (char*)malloc(len + 1);
        for (size_t i = 0; i <= len; i++) {
            char c = upgrade[i];
            if (c >= 'A' && c <= 'Z') c += 32;
            lower[i] = c;
        }
        result = strstr(lower, "websocket") ? 1 : 0;
        free(lower);
    }
    free((void*)upgrade);
    return result;
}

const char* phprs_ws_handshake_response(const char* raw) {
    if (!raw) {
        return phprs_http_response(400, "text/plain", "Bad Request");
    }
    const char* key = phprs_http_header(raw, "Sec-WebSocket-Key");
    if (!key || !*key) {
        free((void*)key);
        return phprs_http_response(400, "text/plain", "Missing Sec-WebSocket-Key");
    }

    const char* magic = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";
    size_t key_len = strlen(key);
    size_t comb_len = key_len + 36;
    unsigned char* combined = (unsigned char*)malloc(comb_len);
    memcpy(combined, key, key_len);
    memcpy(combined + key_len, magic, 36);
    free((void*)key);

    unsigned char sha1_hash[20];
    phprs_sha1(combined, comb_len, sha1_hash);
    free(combined);

    char* accept_key = phprs_base64_encode(sha1_hash, 20);

    char* response = (char*)malloc(256);
    snprintf(response, 256,
        "HTTP/1.1 101 Switching Protocols\r\n"
        "Upgrade: websocket\r\n"
        "Connection: Upgrade\r\n"
        "Sec-WebSocket-Accept: %s\r\n"
        "\r\n",
        accept_key);
    free(accept_key);
    return response;
}

const char* phprs_ws_read_frame(int64_t fd, int64_t timeout_ms) {
    (void)timeout_ms; // blocking I/O for now

    // Read 2-byte header
    unsigned char header[2];
    if (phprs_ws_read_exact(fd, header, 2) < 2)
        return strdup("-1:");

    unsigned char opcode = header[0] & 0x0F;
    unsigned char masked = (header[1] >> 7) & 1;
    uint64_t payload_len = header[1] & 0x7F;

    // Extended payload length
    if (payload_len == 126) {
        unsigned char ext[2];
        if (phprs_ws_read_exact(fd, ext, 2) < 2)
            return strdup("-1:");
        payload_len = ((uint64_t)ext[0] << 8) | ext[1];
    } else if (payload_len == 127) {
        unsigned char ext[8];
        if (phprs_ws_read_exact(fd, ext, 8) < 8)
            return strdup("-1:");
        payload_len = 0;
        for (int i = 0; i < 8; i++)
            payload_len = (payload_len << 8) | ext[i];
    }

    // Safety limit
    if (payload_len > 1024 * 1024) return strdup("-1:");

    // Read mask key
    unsigned char mask_key[4] = {0, 0, 0, 0};
    if (masked) {
        if (phprs_ws_read_exact(fd, mask_key, 4) < 4)
            return strdup("-1:");
    }

    // Read payload
    unsigned char* payload = (unsigned char*)malloc((size_t)payload_len + 1);
    if (!payload) return strdup("-1:");
    if (payload_len > 0) {
        if (phprs_ws_read_exact(fd, payload, (int)payload_len) < (int)payload_len) {
            free(payload);
            return strdup("-1:");
        }
    }
    payload[payload_len] = '\0';

    // Unmask
    if (masked) {
        for (uint64_t i = 0; i < payload_len; i++) {
            payload[i] ^= mask_key[i % 4];
        }
    }

    // Format: "opcode:payload"
    char opcode_str[16];
    snprintf(opcode_str, sizeof(opcode_str), "%d", (int)opcode);
    size_t result_len = strlen(opcode_str) + 1 + payload_len + 1;
    char* result = (char*)malloc(result_len);
    sprintf(result, "%s:", opcode_str);
    memcpy(result + strlen(opcode_str) + 1, payload, payload_len);
    result[result_len - 1] = '\0';
    free(payload);
    return result;
}

int64_t phprs_ws_write_frame(int64_t fd, const char* payload, int64_t opcode) {
    if (!payload) payload = "";
    size_t payload_len = strlen(payload);

    // Determine header size
    unsigned char ext_buf[8];
    int ext_bytes = 0;
    unsigned char len_byte;

    if (payload_len <= 125) {
        len_byte = (unsigned char)payload_len;
    } else if (payload_len <= 65535) {
        len_byte = 126;
        ext_buf[0] = (unsigned char)((payload_len >> 8) & 0xFF);
        ext_buf[1] = (unsigned char)(payload_len & 0xFF);
        ext_bytes = 2;
    } else {
        len_byte = 127;
        for (int i = 7; i >= 0; i--) {
            ext_buf[7 - i] = (unsigned char)((payload_len >> (i * 8)) & 0xFF);
        }
        ext_bytes = 8;
    }

    size_t header_size = 2 + (size_t)ext_bytes;
    size_t frame_size = header_size + payload_len;
    unsigned char* frame = (unsigned char*)malloc(frame_size);
    if (!frame) return -1;

    frame[0] = 0x80 | ((unsigned char)opcode & 0x0F);
    frame[1] = len_byte;
    if (ext_bytes > 0) {
        memcpy(frame + 2, ext_buf, ext_bytes);
    }
    memcpy(frame + header_size, payload, payload_len);

    int64_t sent;
#ifdef _WIN32
    sent = send((SOCKET)fd, (const char*)frame, (int)frame_size, 0);
#else
    sent = (int64_t)send((int)fd, frame, frame_size, 0);
#endif
    free(frame);
    return sent;
}

int64_t phprs_ws_send_pong(int64_t fd, const char* payload) {
    if (!payload) payload = "";
    size_t payload_len = strlen(payload);
    size_t frame_size = 2 + payload_len;
    unsigned char* frame = (unsigned char*)malloc(frame_size);
    if (!frame) return -1;
    frame[0] = 0x8A;
    frame[1] = (unsigned char)payload_len;
    if (payload_len > 0) memcpy(frame + 2, payload, payload_len);
    int64_t sent;
#ifdef _WIN32
    sent = send((SOCKET)fd, (const char*)frame, (int)frame_size, 0);
#else
    sent = (int64_t)send((int)fd, frame, frame_size, 0);
#endif
    free(frame);
    return sent;
}

void phprs_ws_close(int64_t fd) {
    if (fd >= 0) {
        unsigned char close_frame[] = { 0x88, 0x00 };
#ifdef _WIN32
        send((SOCKET)fd, (const char*)close_frame, 2, 0);
        shutdown((SOCKET)fd, SD_BOTH);
#else
        send((int)fd, close_frame, 2, 0);
        shutdown((int)fd, SHUT_RDWR);
#endif
        phprs_socket_close(fd);
    }
}

// ---- DNS Resolution ----

const char* phprs_dns_resolve(const char* hostname) {
    if (!hostname || !*hostname) return strdup("");
#ifdef _WIN32
    phprs_winsock_init();
    struct addrinfo hints, *result;
    ZeroMemory(&hints, sizeof(hints));
    hints.ai_family = AF_INET;
    hints.ai_socktype = SOCK_STREAM;
    if (getaddrinfo(hostname, NULL, &hints, &result) != 0) return strdup("");
    char ip[INET_ADDRSTRLEN];
    struct sockaddr_in* addr = (struct sockaddr_in*)result->ai_addr;
    const char* s = inet_ntop(AF_INET, &addr->sin_addr, ip, sizeof(ip));
    char* r = s ? strdup(s) : strdup("");
    freeaddrinfo(result);
    return r;
#else
    struct addrinfo hints, *result;
    memset(&hints, 0, sizeof(hints));
    hints.ai_family = AF_INET;
    hints.ai_socktype = SOCK_STREAM;
    if (getaddrinfo(hostname, NULL, &hints, &result) != 0) return strdup("");
    char ip[INET_ADDRSTRLEN];
    struct sockaddr_in* addr = (struct sockaddr_in*)result->ai_addr;
    const char* s = inet_ntop(AF_INET, &addr->sin_addr, ip, sizeof(ip));
    char* r = s ? strdup(s) : strdup("");
    freeaddrinfo(result);
    return r;
#endif
}

// ---- TCP Client Connect ----

int64_t phprs_tcp_connect(const char* host, int64_t port) {
    if (!host || !*host) return -1;
    const char* ip = phprs_dns_resolve(host);
    if (!ip || !*ip) return -1;
#ifdef _WIN32
    phprs_winsock_init();
    phprs_socket_t sock = socket(AF_INET, SOCK_STREAM, 0);
    if (sock == PHPRS_INVALID_SOCKET) { free((void*)ip); return -1; }
    struct sockaddr_in addr;
    addr.sin_family = AF_INET;
    addr.sin_port = htons((unsigned short)port);
    inet_pton(AF_INET, ip, &addr.sin_addr);
    free((void*)ip);
    if (connect(sock, (struct sockaddr*)&addr, sizeof(addr)) == SOCKET_ERROR) {
        phprs_closesocket(sock);
        return -1;
    }
    return (int64_t)sock;
#else
    phprs_socket_t sock = socket(AF_INET, SOCK_STREAM, 0);
    if (sock < 0) { free((void*)ip); return -1; }
    struct sockaddr_in addr;
    addr.sin_family = AF_INET;
    addr.sin_port = htons((unsigned short)port);
    inet_pton(AF_INET, ip, &addr.sin_addr);
    free((void*)ip);
    if (connect(sock, (struct sockaddr*)&addr, sizeof(addr)) < 0) {
        close(sock);
        return -1;
    }
    return (int64_t)sock;
#endif
}
// ---- Windows Schannel TLS ----

#ifdef _WIN32
static CredHandle phprs_schannel_cred;
static BOOL phprs_schannel_inited = FALSE;

static int phprs_schannel_init(void) {
    if (phprs_schannel_inited) return 0;
    phprs_winsock_init();
    SCHANNEL_CRED sc;
    ZeroMemory(&sc, sizeof(sc));
    sc.dwVersion = SCHANNEL_CRED_VERSION;
    sc.grbitEnabledProtocols = SP_PROT_TLS1_2_CLIENT;
    sc.dwFlags = SCH_CRED_NO_DEFAULT_CREDS | SCH_CRED_MANUAL_CRED_VALIDATION;
    TimeStamp expiry;
    if (AcquireCredentialsHandleW(NULL, (LPWSTR)UNISP_NAME_W, SECPKG_CRED_OUTBOUND,
            NULL, &sc, NULL, NULL, &phprs_schannel_cred, &expiry) != SEC_E_OK)
        return -1;
    phprs_schannel_inited = TRUE;
    return 0;
}

static int phprs_schannel_handshake(phprs_socket_t sock, const char* hostname,
                                     phprs_tls_ctx* ctx) {
    size_t hostlen = strlen(hostname);
    WCHAR* target = (WCHAR*)malloc((hostlen + 1) * sizeof(WCHAR));
    for (size_t i = 0; i < hostlen; i++)
        target[i] = (WCHAR)(unsigned char)hostname[i];
    target[hostlen] = 0;

    SecBufferDesc out_desc;
    SecBuffer out_buf;
    DWORD flags = ISC_REQ_SEQUENCE_DETECT | ISC_REQ_REPLAY_DETECT |
                  ISC_REQ_CONFIDENTIALITY | ISC_REQ_ALLOCATE_MEMORY |
                  ISC_REQ_STREAM;
    DWORD sspi_flags;
    BOOL done = FALSE;

    out_buf.BufferType = SECBUFFER_TOKEN;
    out_buf.pvBuffer = NULL;
    out_buf.cbBuffer = 0;
    out_desc.ulVersion = SECBUFFER_VERSION;
    out_desc.cBuffers = 1;
    out_desc.pBuffers = &out_buf;

    SECURITY_STATUS status = InitializeSecurityContextW(
        &phprs_schannel_cred, NULL, target, flags, 0,
        SECURITY_NATIVE_DREP, NULL, 0, &ctx->ctxt_handle,
        &out_desc, &sspi_flags, NULL);
    free(target);

    if (status == SEC_I_CONTINUE_NEEDED && out_buf.cbBuffer > 0 && out_buf.pvBuffer) {
        send(sock, (const char*)out_buf.pvBuffer, out_buf.cbBuffer, 0);
        FreeContextBuffer(out_buf.pvBuffer);
        out_buf.pvBuffer = NULL;
    } else if (status != SEC_E_OK && status != SEC_I_CONTINUE_NEEDED) {
        return -1;
    }

    unsigned char recv_buf[16384];
    SecBuffer in_bufs[2];
    SecBufferDesc in_desc;

    while (!done) {
        int n = recv(sock, (char*)recv_buf, sizeof(recv_buf), 0);
        if (n <= 0) return -1;

        in_bufs[0].BufferType = SECBUFFER_TOKEN;
        in_bufs[0].pvBuffer = recv_buf;
        in_bufs[0].cbBuffer = n;
        in_bufs[1].BufferType = SECBUFFER_EMPTY;
        in_bufs[1].pvBuffer = NULL;
        in_bufs[1].cbBuffer = 0;
        in_desc.ulVersion = SECBUFFER_VERSION;
        in_desc.cBuffers = 2;
        in_desc.pBuffers = in_bufs;

        out_buf.BufferType = SECBUFFER_TOKEN;
        out_buf.pvBuffer = NULL;
        out_buf.cbBuffer = 0;
        out_desc.cBuffers = 1;
        out_desc.pBuffers = &out_buf;

        status = InitializeSecurityContextW(
            &phprs_schannel_cred, &ctx->ctxt_handle, NULL, flags, 0,
            SECURITY_NATIVE_DREP, &in_desc, 0, NULL,
            &out_desc, &sspi_flags, NULL);

        if (out_buf.cbBuffer > 0 && out_buf.pvBuffer) {
            send(sock, (const char*)out_buf.pvBuffer, out_buf.cbBuffer, 0);
            FreeContextBuffer(out_buf.pvBuffer);
            out_buf.pvBuffer = NULL;
        }

        if (status == SEC_E_OK) {
            done = TRUE;
        } else if (status != SEC_I_CONTINUE_NEEDED) {
            return -1;
        }
    }

    QueryContextAttributesW(&ctx->ctxt_handle, SECPKG_ATTR_STREAM_SIZES, &ctx->sizes);
    ctx->handshake_done = TRUE;
    ctx->decrypt_buf = NULL;
    ctx->decrypt_buf_len = 0;
    ctx->decrypt_buf_offset = 0;
    return 0;
}

static int phprs_tls_encrypt(phprs_tls_ctx* ctx, const char* data, size_t len,
                              unsigned char** out_ptr, size_t* out_len) {
    size_t msg_size = ctx->sizes.cbHeader + len + ctx->sizes.cbTrailer;
    unsigned char* msg = (unsigned char*)malloc(msg_size);

    SecBuffer bufs[4];
    bufs[0].BufferType = SECBUFFER_STREAM_HEADER;
    bufs[0].pvBuffer = msg;
    bufs[0].cbBuffer = ctx->sizes.cbHeader;
    bufs[1].BufferType = SECBUFFER_DATA;
    bufs[1].pvBuffer = msg + ctx->sizes.cbHeader;
    bufs[1].cbBuffer = (unsigned long)len;
    memcpy(bufs[1].pvBuffer, data, len);
    bufs[2].BufferType = SECBUFFER_STREAM_TRAILER;
    bufs[2].pvBuffer = msg + ctx->sizes.cbHeader + len;
    bufs[2].cbBuffer = ctx->sizes.cbTrailer;
    bufs[3].BufferType = SECBUFFER_EMPTY;
    bufs[3].pvBuffer = NULL;
    bufs[3].cbBuffer = 0;

    SecBufferDesc desc;
    desc.ulVersion = SECBUFFER_VERSION;
    desc.cBuffers = 4;
    desc.pBuffers = bufs;

    if (EncryptMessage(&ctx->ctxt_handle, 0, &desc, 0) != SEC_E_OK) {
        free(msg);
        return -1;
    }

    *out_len = bufs[0].cbBuffer + bufs[1].cbBuffer + bufs[2].cbBuffer;
    *out_ptr = msg;
    return 0;
}

static int phprs_tls_decrypt(phprs_tls_ctx* ctx, unsigned char* enc_data, size_t enc_len,
                              unsigned char** out_ptr, size_t* out_len) {
    unsigned char* work = (unsigned char*)malloc(enc_len);
    memcpy(work, enc_data, enc_len);

    SecBuffer bufs[4];
    bufs[0].BufferType = SECBUFFER_DATA;
    bufs[0].pvBuffer = work;
    bufs[0].cbBuffer = (unsigned long)enc_len;
    bufs[1].BufferType = SECBUFFER_EMPTY;
    bufs[1].pvBuffer = NULL;
    bufs[1].cbBuffer = 0;
    bufs[2].BufferType = SECBUFFER_EMPTY;
    bufs[2].pvBuffer = NULL;
    bufs[2].cbBuffer = 0;
    bufs[3].BufferType = SECBUFFER_EMPTY;
    bufs[3].pvBuffer = NULL;
    bufs[3].cbBuffer = 0;

    SecBufferDesc desc;
    desc.ulVersion = SECBUFFER_VERSION;
    desc.cBuffers = 4;
    desc.pBuffers = bufs;

    SECURITY_STATUS status = DecryptMessage(&ctx->ctxt_handle, &desc, 0, NULL);
    if (status == SEC_E_INCOMPLETE_MESSAGE) {
        free(work);
        return 0; // need more data
    }
    if (status != SEC_E_OK && status != SEC_I_RENEGOTIATE) {
        free(work);
        return -1;
    }

    // Find the data buffer
    size_t plain_len = 0;
    unsigned char* plain_ptr = NULL;
    for (int i = 0; i < 4; i++) {
        if (bufs[i].BufferType == SECBUFFER_DATA && bufs[i].pvBuffer) {
            plain_ptr = (unsigned char*)bufs[i].pvBuffer;
            plain_len = bufs[i].cbBuffer;
            break;
        }
    }

    if (plain_ptr && plain_len > 0) {
        *out_ptr = (unsigned char*)malloc(plain_len);
        memcpy(*out_ptr, plain_ptr, plain_len);
        *out_len = plain_len;
    } else {
        *out_ptr = NULL;
        *out_len = 0;
    }

    free(work);
    return 1;
}
#endif

// ---- POSIX OpenSSL TLS ----

#ifndef _WIN32
static SSL_CTX* phprs_openssl_ctx = NULL;
static int phprs_openssl_inited = 0;

static int phprs_openssl_init(void) {
    if (phprs_openssl_inited) return 0;
    SSL_load_error_strings();
    OpenSSL_add_ssl_algorithms();
    phprs_openssl_ctx = SSL_CTX_new(TLS_client_method());
    if (!phprs_openssl_ctx) return -1;
    SSL_CTX_set_verify(phprs_openssl_ctx, SSL_VERIFY_NONE, NULL);
    phprs_openssl_inited = 1;
    return 0;
}
#endif

int64_t phprs_tls_connect(const char* host, int64_t port) {
    if (!host || !*host) return -1;
    // TCP connect first
    int64_t fd = phprs_tcp_connect(host, port);
    if (fd < 0) return -1;

    phprs_tls_ctx* ctx = (phprs_tls_ctx*)calloc(1, sizeof(phprs_tls_ctx));
    if (!ctx) { phprs_closesocket((phprs_socket_t)fd); return -1; }

#ifdef _WIN32
    if (phprs_schannel_init() != 0 ||
        phprs_schannel_handshake((phprs_socket_t)fd, host, ctx) != 0) {
        free(ctx);
        phprs_closesocket((phprs_socket_t)fd);
        return -1;
    }
#else
    if (phprs_openssl_init() != 0) {
        free(ctx);
        phprs_closesocket((phprs_socket_t)fd);
        return -1;
    }
    ctx->ssl = SSL_new(phprs_openssl_ctx);
    if (!ctx->ssl) {
        free(ctx);
        phprs_closesocket((phprs_socket_t)fd);
        return -1;
    }
    SSL_set_fd(ctx->ssl, (int)fd);
    SSL_set_tlsext_host_name(ctx->ssl, host);
    if (SSL_connect(ctx->ssl) != 1) {
        SSL_free(ctx->ssl);
        free(ctx);
        phprs_closesocket((phprs_socket_t)fd);
        return -1;
    }
#endif

    if (phprs_tls_add(fd, ctx) != 0) {
#ifdef _WIN32
        DeleteSecurityContext(&ctx->ctxt_handle);
#else
        SSL_shutdown(ctx->ssl);
        SSL_free(ctx->ssl);
#endif
        free(ctx);
        phprs_closesocket((phprs_socket_t)fd);
        return -1;
    }
    return fd;
}

// ---- Socket Read All (for HTTP responses) ----

const char* phprs_socket_read_all(int64_t fd) {
    if (fd < 0) return strdup("");

    // Check for TLS context
    phprs_tls_ctx* tls = phprs_tls_find(fd);
    if (tls) {
#ifdef _WIN32
        // Schannel decryption loop — HTTP-aware
        size_t cap = 4096;
        size_t total = 0;
        char* result = (char*)malloc(cap);
        if (!result) return strdup("");
        const size_t max_size = 10 * 1024 * 1024;
        unsigned char enc_buf[16384];
        int headers_complete = 0;
        size_t body_start = 0;
        long content_length = -1;
        int is_chunked = 0;
        while (total < max_size) {
            int n = recv((SOCKET)fd, (char*)enc_buf, sizeof(enc_buf), 0);
            if (n <= 0) break;

            size_t combined_len = tls->decrypt_buf_offset + n;
            unsigned char* combined = (unsigned char*)malloc(combined_len);
            if (tls->decrypt_buf && tls->decrypt_buf_offset > 0)
                memcpy(combined, tls->decrypt_buf, tls->decrypt_buf_offset);
            memcpy(combined + tls->decrypt_buf_offset, enc_buf, n);
            free(tls->decrypt_buf);
            tls->decrypt_buf = NULL;
            tls->decrypt_buf_offset = 0;

            unsigned char* plain = NULL;
            size_t plain_len = 0;
            int rc = phprs_tls_decrypt(tls, combined, combined_len, &plain, &plain_len);
            if (rc < 0) { free(combined); break; }
            if (rc == 0) {
                tls->decrypt_buf = combined;
                tls->decrypt_buf_offset = combined_len;
                continue;
            }
            free(combined);

            if (plain && plain_len > 0) {
                if (total + plain_len + 1 > cap) {
                    size_t new_cap = cap * 2;
                    if (new_cap > max_size) new_cap = max_size;
                    char* new_buf = (char*)realloc(result, new_cap);
                    if (!new_buf) { free(plain); free(result); return strdup(""); }
                    result = new_buf;
                    cap = new_cap;
                }
                memcpy(result + total, plain, plain_len);
                total += plain_len;
                free(plain);
                result[total] = '\0';

                if (!headers_complete) {
                    char* header_end = strstr(result, "\r\n\r\n");
                    if (header_end) {
                        headers_complete = 1;
                        body_start = (size_t)(header_end - result) + 4;
                        char* cl = strstr(result, "Content-Length:");
                        if (!cl) cl = strstr(result, "content-length:");
                        if (cl) content_length = strtol(cl + 15, NULL, 10);
                        if ((strstr(result, "Transfer-Encoding:") && strstr(result, "chunked")) ||
                            (strstr(result, "transfer-encoding:") && strstr(result, "chunked")))
                            is_chunked = 1;
                    }
                }

                if (headers_complete) {
                    if (content_length >= 0 && total >= body_start + (size_t)content_length)
                        break;
                    if (is_chunked) {
                        char* body = result + body_start;
                        if (strstr(body, "0\r\n\r\n")) break;
                    }
                    if (content_length < 0 && !is_chunked) break;
                }
            }
        }
        result[total] = '\0';
        return result;
#else
        // OpenSSL read loop — HTTP-aware
        size_t cap = 4096;
        size_t total = 0;
        char* result = (char*)malloc(cap);
        if (!result) return strdup("");
        const size_t max_size = 10 * 1024 * 1024;
        char buf[4096];
        int headers_complete = 0;
        size_t body_start = 0;
        long content_length = -1;
        int is_chunked = 0;
        while (total < max_size) {
            int n = SSL_read(tls->ssl, buf, sizeof(buf));
            if (n <= 0) break;
            if (total + n + 1 > cap) {
                size_t new_cap = cap * 2;
                if (new_cap > max_size) new_cap = max_size;
                char* new_buf = (char*)realloc(result, new_cap);
                if (!new_buf) { free(result); return strdup(""); }
                result = new_buf;
                cap = new_cap;
            }
            memcpy(result + total, buf, n);
            total += n;
            result[total] = '\0';

            if (!headers_complete) {
                char* header_end = strstr(result, "\r\n\r\n");
                if (header_end) {
                    headers_complete = 1;
                    body_start = (size_t)(header_end - result) + 4;
                    char* cl = strstr(result, "Content-Length:");
                    if (!cl) cl = strstr(result, "content-length:");
                    if (cl) content_length = strtol(cl + 15, NULL, 10);
                    if ((strstr(result, "Transfer-Encoding:") && strstr(result, "chunked")) ||
                        (strstr(result, "transfer-encoding:") && strstr(result, "chunked")))
                        is_chunked = 1;
                }
            }

            if (headers_complete) {
                if (content_length >= 0 && total >= body_start + (size_t)content_length)
                    break;
                if (is_chunked) {
                    char* body = result + body_start;
                    if (strstr(body, "0\r\n\r\n")) break;
                }
                if (content_length < 0 && !is_chunked) break;
            }
        }
        result[total] = '\0';
        return result;
#endif
    }

    // Plain socket read_all — HTTP-aware: reads until response is complete
    // rather than waiting for connection close (which may never come with keep-alive).
    size_t cap = 4096;
    size_t total = 0;
    char* buf = (char*)malloc(cap);
    if (!buf) return strdup("");
    const size_t max_size = 10 * 1024 * 1024; // 10 MB limit
    int headers_complete = 0;
    size_t body_start = 0;
    long content_length = -1;
    int is_chunked = 0;

    // Phase 1: Read headers
    while (total < max_size) {
        if (total + 4096 > cap) {
            size_t new_cap = cap * 2;
            if (new_cap > max_size) new_cap = max_size;
            char* new_buf = (char*)realloc(buf, new_cap);
            if (!new_buf) { free(buf); return strdup(""); }
            buf = new_buf;
            cap = new_cap;
        }
        int n;
#ifdef _WIN32
        n = recv((SOCKET)fd, buf + total, (int)(cap - total - 1), 0);
#else
        n = (int)recv((int)fd, buf + total, cap - total - 1, 0);
#endif
        if (n <= 0) break;
        total += n;
        buf[total] = '\0';

        char* header_end = strstr(buf, "\r\n\r\n");
        if (header_end) {
            headers_complete = 1;
            body_start = (size_t)(header_end - buf) + 4;

            // Parse Content-Length
            char* cl = strstr(buf, "Content-Length:");
            if (!cl) cl = strstr(buf, "content-length:");
            if (cl) {
                content_length = strtol(cl + 15, NULL, 10);
            }

            // Detect chunked
            if (strstr(buf, "Transfer-Encoding:") && strstr(buf, "chunked"))
                is_chunked = 1;
            else if (strstr(buf, "transfer-encoding:") && strstr(buf, "chunked"))
                is_chunked = 1;

            break;
        }
    }

    // Phase 2: Read body based on Content-Length or chunked encoding
    if (headers_complete) {
        if (content_length >= 0) {
            size_t needed = body_start + (size_t)content_length;
            while (total < needed && total < max_size) {
                if (total + 4096 > cap) {
                    size_t new_cap = cap * 2;
                    if (new_cap > max_size) new_cap = max_size;
                    char* new_buf = (char*)realloc(buf, new_cap);
                    if (!new_buf) { free(buf); return strdup(""); }
                    buf = new_buf;
                    cap = new_cap;
                }
                int n;
#ifdef _WIN32
                n = recv((SOCKET)fd, buf + total, (int)(cap - total - 1), 0);
#else
                n = (int)recv((int)fd, buf + total, cap - total - 1, 0);
#endif
                if (n <= 0) break;
                total += n;
                buf[total] = '\0';
            }
        } else if (is_chunked) {
            // Read until the terminating "0\r\n\r\n" chunk
            while (total < max_size) {
                // Check for terminating chunk marker: "0\r\n\r\n"
                char* body = buf + body_start;
                if (strstr(body, "0\r\n\r\n")) break;
                if (total >= body_start && buf[total - 1] == '\n' &&
                    total >= body_start + 5 &&
                    strncmp(buf + total - 5, "0\r\n\r\n", 5) == 0) break;

                if (total + 4096 > cap) {
                    size_t new_cap = cap * 2;
                    if (new_cap > max_size) new_cap = max_size;
                    char* new_buf = (char*)realloc(buf, new_cap);
                    if (!new_buf) { free(buf); return strdup(""); }
                    buf = new_buf;
                    cap = new_cap;
                }
                int n;
#ifdef _WIN32
                n = recv((SOCKET)fd, buf + total, (int)(cap - total - 1), 0);
#else
                n = (int)recv((int)fd, buf + total, cap - total - 1, 0);
#endif
                if (n <= 0) break;
                total += n;
                buf[total] = '\0';
            }
        }
        // If neither CL nor chunked, we already have headers + whatever body came with them
    }
    buf[total] = '\0';
    return buf;
}

// ---- HTTP Request Builder ----

const char* phprs_http_build_request(const char* method, const char* host, const char* path, const char* headers, const char* body) {
    if (!method) method = "GET";
    if (!host) host = "localhost";
    if (!path) path = "/";
    if (!headers) headers = "";
    if (!body) body = "";
    size_t method_len = strlen(method);
    size_t host_len = strlen(host);
    size_t path_len = strlen(path);
    size_t headers_len = strlen(headers);
    size_t body_len = strlen(body);
    // Estimate: method + " " + path + " HTTP/1.1\r\nHost: " + host + "\r\n" + headers + "\r\n" + body
    size_t req_len = method_len + 1 + path_len + 11 + host_len + 8 + headers_len + 2 + body_len + 1;
    // Add extra space for any extra headers we might prepend
    char* req = (char*)malloc(req_len + 256);
    if (!req) return strdup("");
    int offset = 0;
    // Check if Host header already present in custom headers
    int has_host = headers_len > 0 && strstr(headers, "Host:") != NULL;
    offset += snprintf(req + offset, req_len + 256 - offset, "%s %s HTTP/1.1\r\nConnection: close\r\n", method, path);
    if (!has_host) {
        offset += snprintf(req + offset, req_len + 256 - offset, "Host: %s\r\n", host);
    }
    if (headers_len > 0) {
        offset += snprintf(req + offset, req_len + 256 - offset, "%s", headers);
    }
    offset += snprintf(req + offset, req_len + 256 - offset, "\r\n%s", body);
    return req;
}

// ---- HTTP Response Parsing (client-side) ----

int64_t phprs_http_response_status(const char* raw) {
    if (!raw) return 0;
    // Format: "HTTP/1.1 200 OK\r\n..."
    const char* space = strchr(raw, ' ');
    if (!space) return 0;
    space++;
    return (int64_t)strtoll(space, NULL, 10);
}

const char* phprs_http_response_body(const char* raw) {
    if (!raw) return strdup("");
    const char* body_start = strstr(raw, "\r\n\r\n");
    if (!body_start) return strdup("");
    body_start += 4;
    return strdup(body_start);
}

// ---- Threading Support ----

#ifdef _WIN32
#include <process.h>
#define PHPRS_THREAD_RETURN unsigned __stdcall
#define PHPRS_THREAD_RETVAL 0
typedef HANDLE phprs_thread_t;
#else
#include <pthread.h>
#define PHPRS_THREAD_RETURN void*
#define PHPRS_THREAD_RETVAL NULL
typedef pthread_t phprs_thread_t;
#endif

// Function dispatch table for threaded callbacks
#define PHPRS_MAX_DISPATCH_FUNCS 64

typedef const char* (*phprs_handler_fn)(const char*);

static struct {
    const char* name;
    phprs_handler_fn fn;
} phprs_dispatch_table[PHPRS_MAX_DISPATCH_FUNCS];

static int phprs_dispatch_count = 0;

void phprs_register_handler(const char* name, phprs_handler_fn fn) {
    if (phprs_dispatch_count < PHPRS_MAX_DISPATCH_FUNCS && name && fn) {
        phprs_dispatch_table[phprs_dispatch_count].name = name;
        phprs_dispatch_table[phprs_dispatch_count].fn = fn;
        phprs_dispatch_count++;
    }
}

static phprs_handler_fn phprs_lookup_handler(const char* name) {
    for (int i = 0; i < phprs_dispatch_count; i++) {
        if (strcmp(phprs_dispatch_table[i].name, name) == 0) {
            return phprs_dispatch_table[i].fn;
        }
    }
    return NULL;
}

// Thread argument struct
struct phprs_thread_arg {
    const char* func_name;
    int64_t client_fd;
    char* raw_request;  // owned copy for the thread
};

static PHPRS_THREAD_RETURN phprs_thread_worker(void* arg) {
    struct phprs_thread_arg* ta = (struct phprs_thread_arg*)arg;
    phprs_handler_fn handler = phprs_lookup_handler(ta->func_name);
    if (handler) {
        const char* response = handler(ta->raw_request);
        if (response) {
            phprs_socket_write(ta->client_fd, response);
        }
    }
    phprs_socket_close(ta->client_fd);
    free(ta->raw_request);
    free(ta);
    return PHPRS_THREAD_RETVAL;
}

int64_t phprs_thread_spawn(const char* func_name, int64_t client_fd, const char* raw_request) {
    if (!func_name || !raw_request) return 0;
    struct phprs_thread_arg* ta = (struct phprs_thread_arg*)malloc(sizeof(struct phprs_thread_arg));
    if (!ta) return 0;
    ta->func_name = func_name;
    ta->client_fd = client_fd;
    ta->raw_request = strdup(raw_request);
#ifdef _WIN32
    HANDLE thread = (HANDLE)_beginthreadex(NULL, 0, phprs_thread_worker, ta, 0, NULL);
    if (thread) {
        CloseHandle(thread);  // detach
        return 1;
    }
#else
    pthread_t thread;
    if (pthread_create(&thread, NULL, phprs_thread_worker, ta) == 0) {
        pthread_detach(thread);
        return 1;
    }
#endif
    free(ta->raw_request);
    free(ta);
    return 0;
}

// ---- Thread Pool ----

#define PHPRS_TP_MAX_THREADS 64
#define PHPRS_TP_QUEUE_SIZE 256

static struct {
    phprs_thread_t threads[PHPRS_TP_MAX_THREADS];
    int num_threads;
    int running;

    struct phprs_thread_arg* queue[PHPRS_TP_QUEUE_SIZE];
    int head;
    int tail;
    int count;

#ifdef _WIN32
    CRITICAL_SECTION mutex;
    CONDITION_VARIABLE cond;
#else
    pthread_mutex_t mutex;
    pthread_cond_t cond;
#endif
} phprs_tp;

static PHPRS_THREAD_RETURN phprs_tp_worker(void* arg) {
    (void)arg;
    for (;;) {
        struct phprs_thread_arg* ta = NULL;

#ifdef _WIN32
        EnterCriticalSection(&phprs_tp.mutex);
        while (phprs_tp.running && phprs_tp.count == 0) {
            SleepConditionVariableCS(&phprs_tp.cond, &phprs_tp.mutex, INFINITE);
        }
        if (!phprs_tp.running && phprs_tp.count == 0) {
            LeaveCriticalSection(&phprs_tp.mutex);
            break;
        }
        ta = phprs_tp.queue[phprs_tp.head];
        phprs_tp.head = (phprs_tp.head + 1) % PHPRS_TP_QUEUE_SIZE;
        phprs_tp.count--;
        LeaveCriticalSection(&phprs_tp.mutex);
#else
        pthread_mutex_lock(&phprs_tp.mutex);
        while (phprs_tp.running && phprs_tp.count == 0) {
            pthread_cond_wait(&phprs_tp.cond, &phprs_tp.mutex);
        }
        if (!phprs_tp.running && phprs_tp.count == 0) {
            pthread_mutex_unlock(&phprs_tp.mutex);
            break;
        }
        ta = phprs_tp.queue[phprs_tp.head];
        phprs_tp.head = (phprs_tp.head + 1) % PHPRS_TP_QUEUE_SIZE;
        phprs_tp.count--;
        pthread_mutex_unlock(&phprs_tp.mutex);
#endif

        if (ta) {
            phprs_handler_fn handler = phprs_lookup_handler(ta->func_name);
            if (handler) {
                const char* response = handler(ta->raw_request);
                if (response) {
                    phprs_socket_write(ta->client_fd, response);
                }
            }
            phprs_socket_close(ta->client_fd);
            free(ta->raw_request);
            free(ta);
        }
    }
    return PHPRS_THREAD_RETVAL;
}

int64_t phprs_thread_pool_init(int64_t num_threads) {
    if (num_threads < 1) num_threads = 4;
    if (num_threads > PHPRS_TP_MAX_THREADS) num_threads = PHPRS_TP_MAX_THREADS;

    phprs_tp.num_threads = (int)num_threads;
    phprs_tp.running = 1;
    phprs_tp.head = 0;
    phprs_tp.tail = 0;
    phprs_tp.count = 0;

#ifdef _WIN32
    InitializeCriticalSection(&phprs_tp.mutex);
    InitializeConditionVariable(&phprs_tp.cond);
#else
    pthread_mutex_init(&phprs_tp.mutex, NULL);
    pthread_cond_init(&phprs_tp.cond, NULL);
#endif

    for (int i = 0; i < phprs_tp.num_threads; i++) {
#ifdef _WIN32
        phprs_tp.threads[i] = (HANDLE)_beginthreadex(NULL, 0, phprs_tp_worker, NULL, 0, NULL);
#else
        pthread_create(&phprs_tp.threads[i], NULL, phprs_tp_worker, NULL);
#endif
    }
    return 1;
}

int64_t phprs_thread_pool_enqueue(const char* func_name, int64_t client_fd, const char* raw_request) {
    if (!func_name || !raw_request) return 0;

    struct phprs_thread_arg* ta = (struct phprs_thread_arg*)malloc(sizeof(struct phprs_thread_arg));
    if (!ta) return 0;
    ta->func_name = func_name;
    ta->client_fd = client_fd;
    ta->raw_request = strdup(raw_request);

#ifdef _WIN32
    EnterCriticalSection(&phprs_tp.mutex);
#else
    pthread_mutex_lock(&phprs_tp.mutex);
#endif

    if (phprs_tp.count >= PHPRS_TP_QUEUE_SIZE || !phprs_tp.running) {
#ifdef _WIN32
        LeaveCriticalSection(&phprs_tp.mutex);
#else
        pthread_mutex_unlock(&phprs_tp.mutex);
#endif
        free(ta->raw_request);
        free(ta);
        return 0;
    }

    phprs_tp.queue[phprs_tp.tail] = ta;
    phprs_tp.tail = (phprs_tp.tail + 1) % PHPRS_TP_QUEUE_SIZE;
    phprs_tp.count++;

#ifdef _WIN32
    WakeConditionVariable(&phprs_tp.cond);
    LeaveCriticalSection(&phprs_tp.mutex);
#else
    pthread_cond_signal(&phprs_tp.cond);
    pthread_mutex_unlock(&phprs_tp.mutex);
#endif

    return 1;
}

void phprs_thread_pool_shutdown() {
#ifdef _WIN32
    EnterCriticalSection(&phprs_tp.mutex);
#else
    pthread_mutex_lock(&phprs_tp.mutex);
#endif
    phprs_tp.running = 0;
#ifdef _WIN32
    WakeAllConditionVariable(&phprs_tp.cond);
    LeaveCriticalSection(&phprs_tp.mutex);
#else
    pthread_cond_broadcast(&phprs_tp.cond);
    pthread_mutex_unlock(&phprs_tp.mutex);
#endif

    for (int i = 0; i < phprs_tp.num_threads; i++) {
#ifdef _WIN32
        WaitForSingleObject(phprs_tp.threads[i], INFINITE);
        CloseHandle(phprs_tp.threads[i]);
#else
        pthread_join(phprs_tp.threads[i], NULL);
#endif
    }

#ifdef _WIN32
    DeleteCriticalSection(&phprs_tp.mutex);
#else
    pthread_mutex_destroy(&phprs_tp.mutex);
    pthread_cond_destroy(&phprs_tp.cond);
#endif
}

// ---- Mutex (integer handle API) ----

#define PHPRS_MAX_MUTEXES 64

#ifdef _WIN32
typedef CRITICAL_SECTION phprs_mutex_inner_t;
#else
typedef pthread_mutex_t phprs_mutex_inner_t;
#endif

static phprs_mutex_inner_t phprs_mutex_pool[PHPRS_MAX_MUTEXES];
static int phprs_mutex_used[PHPRS_MAX_MUTEXES] = {0};

int64_t phprs_mutex_new() {
    for (int i = 0; i < PHPRS_MAX_MUTEXES; i++) {
        if (!phprs_mutex_used[i]) {
            phprs_mutex_used[i] = 1;
#ifdef _WIN32
            InitializeCriticalSection(&phprs_mutex_pool[i]);
#else
            pthread_mutex_init(&phprs_mutex_pool[i], NULL);
#endif
            return (int64_t)i;
        }
    }
    return -1;
}

void phprs_mutex_lock(int64_t handle) {
    if (handle < 0 || handle >= PHPRS_MAX_MUTEXES || !phprs_mutex_used[handle]) return;
#ifdef _WIN32
    EnterCriticalSection(&phprs_mutex_pool[handle]);
#else
    pthread_mutex_lock(&phprs_mutex_pool[handle]);
#endif
}

void phprs_mutex_unlock(int64_t handle) {
    if (handle < 0 || handle >= PHPRS_MAX_MUTEXES || !phprs_mutex_used[handle]) return;
#ifdef _WIN32
    LeaveCriticalSection(&phprs_mutex_pool[handle]);
#else
    pthread_mutex_unlock(&phprs_mutex_pool[handle]);
#endif
}

// ---- Type Checking (compiled mode stubs) ----
// In compiled C code, types are statically known. These are stubs for compatibility.
// The AST-to-C transpiler may inline or replace these calls with constants.

bool is_null(int64_t val) { return val == 0; }
bool is_int(int64_t val) { (void)val; return true; }
bool is_string(const char* val) { (void)val; return true; }
bool is_bool(bool val) { (void)val; return true; }
bool is_float(double val) { (void)val; return true; }
bool is_array(int64_t val) { (void)val; return false; }
const char* gettype(int64_t val) { (void)val; return "unknown"; }
bool isset(int64_t val) { return val != 0; }
bool empty_(const char* val) {
    if (!val) return true;              // null
    if (val[0] == '\0') return true;    // "" (empty string)
    if (val[0] == '0' && val[1] == '\0') return true;  // "0"
    return false;
}
void unset_(int64_t val) { (void)val; }

// ---- Exception Handling (try/catch/throw) ----

static jmp_buf* phprs_catch_stack[64];
static int phprs_catch_depth = 0;
char* __catch_error = NULL;

void __push_catch(jmp_buf* buf) {
    if (phprs_catch_depth < 64) {
        phprs_catch_stack[phprs_catch_depth++] = buf;
    }
}

void __pop_catch(void) {
    if (phprs_catch_depth > 0) {
        phprs_catch_depth--;
    }
}

void __throw(const char* message) {
    if (__catch_error) free(__catch_error);
    __catch_error = message ? strdup(message) : strdup("Unknown exception");
    if (phprs_catch_depth > 0) {
        longjmp(*phprs_catch_stack[phprs_catch_depth - 1], 1);
    }
    fprintf(stderr, "Uncaught exception: %s\n", __catch_error);
    exit(1);
}

// ---- PHP-compatible String Functions ----

char* substr(const char* s, int64_t start, int64_t length) {
    if (!s) { char* r = malloc(1); r[0] = '\0'; return r; }
    int64_t slen = (int64_t)strlen(s);
    if (start < 0) start = slen + start;
    if (start < 0) start = 0;
    if (start > slen) { char* r = malloc(1); r[0] = '\0'; return r; }
    int64_t maxlen = slen - start;
    if (length < 0 || length > maxlen) length = maxlen;
    char* r = malloc((size_t)length + 1);
    memcpy(r, s + start, (size_t)length);
    r[length] = '\0';
    return r;
}

int64_t strpos(const char* haystack, const char* needle) {
    if (!haystack || !needle) return -1;
    const char* p = strstr(haystack, needle);
    return p ? (int64_t)(p - haystack) : -1;
}

int64_t stripos(const char* haystack, const char* needle) {
    if (!haystack || !needle) return -1;
    size_t hlen = strlen(haystack), nlen = strlen(needle);
    if (nlen > hlen) return -1;
    for (size_t i = 0; i <= hlen - nlen; i++) {
        size_t j;
        for (j = 0; j < nlen; j++) {
            if (tolower((unsigned char)haystack[i+j]) != tolower((unsigned char)needle[j])) break;
        }
        if (j == nlen) return (int64_t)i;
    }
    return -1;
}

// explode returns a serialized array: "count\0elem1\0elem2\0..."
char* explode(const char* delimiter, const char* s) {
    if (!s) { char* r = malloc(10); snprintf(r, 10, "0"); return r; }
    size_t dlen = delimiter ? strlen(delimiter) : 0;
    if (dlen == 0) {
        size_t slen = strlen(s);
        char* r = malloc(slen * 2 + 20);
        int off = snprintf(r, 20, "%zu", slen);
        for (size_t i = 0; i < slen; i++) {
            r[off++] = '\0'; r[off++] = s[i];
        }
        r[off] = '\0';
        return r;
    }
    // Count parts and build
    size_t slen = strlen(s);
    size_t count = 1;
    const char* p = s;
    while ((p = strstr(p, delimiter)) != NULL) { count++; p += dlen; }
    // Estimate size
    char* r = malloc(slen + count * 2 + 20);
    int off = snprintf(r, 20, "%zu", count);
    p = s;
    while (1) {
        const char* next = strstr(p, delimiter);
        size_t len = next ? (size_t)(next - p) : strlen(p);
        r[off++] = '\0';
        memcpy(r + off, p, len); off += (int)len;
        r[off] = '\0';
        if (!next) break;
        p = next + dlen;
    }
    r[off] = '\0';
    return r;
}

// implode: joins array elements with glue. Array format: "count\0elem1\0elem2\0..."
char* implode(const char* glue, const char* serialized_array) {
    if (!serialized_array) { char* r = malloc(1); r[0] = '\0'; return r; }
    const char* g = glue ? glue : "";
    size_t glen = strlen(g);
    // Parse count
    int count = atoi(serialized_array);
    if (count <= 0) { char* r = malloc(1); r[0] = '\0'; return r; }
    const char* p = serialized_array;
    while (*p) p++; p++; // skip count and null
    // Calculate total length
    size_t total = 0;
    const char* parts[1024];
    size_t lens[1024];
    int n = count > 1024 ? 1024 : count;
    for (int i = 0; i < n; i++) {
        parts[i] = p;
        size_t len = strlen(p);
        lens[i] = len;
        total += len;
        p += len + 1;
    }
    total += glen * (size_t)(n > 0 ? n - 1 : 0);
    char* r = malloc(total + 1);
    size_t pos = 0;
    for (int i = 0; i < n; i++) {
        if (i > 0) { memcpy(r + pos, g, glen); pos += glen; }
        memcpy(r + pos, parts[i], lens[i]); pos += lens[i];
    }
    r[pos] = '\0';
    return r;
}

char* str_repeat(const char* s, int64_t count) {
    if (!s || count <= 0) { char* r = malloc(1); r[0] = '\0'; return r; }
    size_t slen = strlen(s);
    size_t total = slen * (size_t)count;
    char* r = malloc(total + 1);
    for (int64_t i = 0; i < count; i++) memcpy(r + i * (int64_t)slen, s, slen);
    r[total] = '\0';
    return r;
}

char* strtolower(const char* s) {
    if (!s) { char* r = malloc(1); r[0] = '\0'; return r; }
    size_t len = strlen(s);
    char* r = malloc(len + 1);
    for (size_t i = 0; i < len; i++) r[i] = (char)tolower((unsigned char)s[i]);
    r[len] = '\0';
    return r;
}

char* strtoupper(const char* s) {
    if (!s) { char* r = malloc(1); r[0] = '\0'; return r; }
    size_t len = strlen(s);
    char* r = malloc(len + 1);
    for (size_t i = 0; i < len; i++) r[i] = (char)toupper((unsigned char)s[i]);
    r[len] = '\0';
    return r;
}

char* htmlspecialchars(const char* s) {
    if (!s) { char* r = malloc(1); r[0] = '\0'; return r; }
    size_t len = strlen(s);
    size_t alloc = len * 6 + 1; // worst-case: all & becomes &amp;
    char* r = malloc(alloc);
    size_t j = 0;
    for (size_t i = 0; i < len; i++) {
        switch (s[i]) {
            case '&': memcpy(r + j, "&amp;", 5); j += 5; break;
            case '<': memcpy(r + j, "&lt;", 4); j += 4; break;
            case '>': memcpy(r + j, "&gt;", 4); j += 4; break;
            case '"': memcpy(r + j, "&quot;", 6); j += 6; break;
            case '\'': memcpy(r + j, "&#039;", 6); j += 6; break;
            default: r[j++] = s[i];
        }
    }
    r[j] = '\0';
    return r;
}

char* strip_tags(const char* s) {
    if (!s) { char* r = malloc(1); r[0] = '\0'; return r; }
    size_t len = strlen(s);
    char* r = malloc(len + 1);
    size_t j = 0;
    int in_tag = 0;
    for (size_t i = 0; i < len; i++) {
        if (s[i] == '<') { in_tag = 1; continue; }
        if (s[i] == '>') { in_tag = 0; continue; }
        if (!in_tag) r[j++] = s[i];
    }
    r[j] = '\0';
    return r;
}

char* nl2br(const char* s) {
    if (!s) { char* r = malloc(1); r[0] = '\0'; return r; }
    size_t len = strlen(s);
    size_t newlines = 0;
    for (size_t i = 0; i < len; i++) if (s[i] == '\n') newlines++;
    size_t alloc = len + newlines * 6 + 1;
    char* r = malloc(alloc);
    size_t j = 0;
    for (size_t i = 0; i < len; i++) {
        if (s[i] == '\n') { memcpy(r + j, "<br />\n", 7); j += 7; }
        else r[j++] = s[i];
    }
    r[j] = '\0';
    return r;
}

// ---- More String Functions ----

const char* str_replace(const char* search, const char* replace, const char* subject) {
    if (!search || !replace || !subject) return strdup("");
    size_t search_len = strlen(search);
    if (search_len == 0) return strdup(subject);
    // Count occurrences
    size_t count = 0;
    const char* p = subject;
    while ((p = strstr(p, search)) != NULL) { count++; p += search_len; }
    size_t replace_len = strlen(replace);
    size_t result_len = strlen(subject) + count * (replace_len - search_len) + 1;
    char* result = malloc(result_len);
    if (!result) return strdup("");
    char* w = result;
    const char* r = subject;
    while (*r) {
        const char* found = strstr(r, search);
        if (found) {
            size_t prefix = found - r;
            memcpy(w, r, prefix); w += prefix;
            memcpy(w, replace, replace_len); w += replace_len;
            r = found + search_len;
        } else {
            size_t rest = strlen(r);
            memcpy(w, r, rest); w += rest;
            break;
        }
    }
    *w = '\0';
    return result;
}

const char* trim(const char* s) {
    if (!s) return strdup("");
    // Trim left
    while (*s == ' ' || *s == '\t' || *s == '\n' || *s == '\r') s++;
    // Trim right
    const char* end = s + strlen(s) - 1;
    while (end >= s && (*end == ' ' || *end == '\t' || *end == '\n' || *end == '\r')) end--;
    size_t len = (size_t)(end - s + 1);
    char* r = malloc(len + 1);
    memcpy(r, s, len); r[len] = '\0';
    return r;
}

const char* ltrim(const char* s) {
    if (!s) return strdup("");
    while (*s == ' ' || *s == '\t' || *s == '\n' || *s == '\r') s++;
    return strdup(s);
}

const char* rtrim(const char* s) {
    if (!s) return strdup("");
    const char* end = s + strlen(s) - 1;
    while (end >= s && (*end == ' ' || *end == '\t' || *end == '\n' || *end == '\r')) end--;
    size_t len = (size_t)(end - s + 1);
    char* r = malloc(len + 1);
    memcpy(r, s, len); r[len] = '\0';
    return r;
}

int64_t strrpos(const char* haystack, const char* needle) {
    if (!haystack || !needle) return -1;
    size_t hlen = strlen(haystack), nlen = strlen(needle);
    if (nlen == 0) return (int64_t)hlen;
    if (nlen > hlen) return -1;
    for (int64_t i = (int64_t)(hlen - nlen); i >= 0; i--) {
        if (strncmp(haystack + i, needle, nlen) == 0) return i;
    }
    return -1;
}

const char* ucfirst(const char* s) {
    if (!s || *s == '\0') return strdup("");
    char* r = strdup(s);
    if (r[0] >= 'a' && r[0] <= 'z') r[0] -= 32;
    return r;
}

const char* phprs_sprintf(const char* fmt, const char* a1, const char* a2, const char* a3, const char* a4) {
    if (!fmt) return strdup("");
    size_t fmt_len = strlen(fmt);
    char* result = malloc(fmt_len + 256);
    if (!result) return strdup("");
    size_t ri = 0;
    int ai = 0;
    const char* args[4] = { a1, a2, a3, a4 };
    for (size_t i = 0; i < fmt_len; i++) {
        if (fmt[i] == '%' && i + 1 < fmt_len) {
            char spec = fmt[i + 1];
            if (spec == '%') { result[ri++] = '%'; i++; continue; }
            if (ai < 4 && args[ai]) {
                const char* arg = args[ai];
                ai++;
                size_t alen = strlen(arg);
                memcpy(result + ri, arg, alen); ri += alen;
                i++;
                continue;
            }
        }
        result[ri++] = fmt[i];
    }
    result[ri] = '\0';
    return result;
}

const char* number_format(double num, int64_t decimals) {
    char fmt[32];
    snprintf(fmt, sizeof(fmt), "%%.%lldf", (long long)decimals);
    char buf[64];
    snprintf(buf, sizeof(buf), fmt, num);
    return strdup(buf);
}

// ---- Math Functions ----

int64_t abs_(int64_t n) { return n < 0 ? -n : n; }
double abs_f(double n) { return n < 0.0 ? -n : n; }

int64_t ceil_(double n) { return (int64_t)n + (n > (double)((int64_t)n) ? 1 : 0); }
int64_t floor_(double n) { return (int64_t)n - (n < (double)((int64_t)n) ? 1 : 0); }

double round_(double n, int64_t precision) {
    double factor = 1.0;
    int64_t p = precision;
    while (p > 0) { factor *= 10.0; p--; }
    while (p < 0) { factor /= 10.0; p++; }
    return (double)((int64_t)(n * factor + 0.5)) / factor;
}

int64_t max_i(int64_t a, int64_t b) { return a > b ? a : b; }
double max_f(double a, double b) { return a > b ? a : b; }
int64_t min_i(int64_t a, int64_t b) { return a < b ? a : b; }
double min_f(double a, double b) { return a < b ? a : b; }

int64_t rand_(int64_t min, int64_t max) {
    if (min > max) return min;
    return min + (rand() % (max - min + 1));
}

int64_t mt_rand_(int64_t min, int64_t max) {
    return rand_(min, max);
}

double pow_(double base, double exponent) {
    double result = 1.0;
    if (exponent == 0.0) return 1.0;
    if (exponent < 0.0) { base = 1.0 / base; exponent = -exponent; }
    for (int64_t i = 0; i < (int64_t)exponent; i++) result *= base;
    double frac = exponent - (int64_t)exponent;
    if (frac > 0.0) result *= exp(frac * log(base));
    return result;
}

double sqrt_(double n) {
    if (n <= 0.0) return 0.0;
    double x = n;
    for (int i = 0; i < 20; i++) { x = (x + n / x) * 0.5; }
    return x;
}

// ---- Date/Time Functions ----

int64_t time_(void) {
    return (int64_t)time(NULL);
}

char* date(const char* format, int64_t timestamp) {
    time_t t = (time_t)timestamp;
    struct tm* tm_info = localtime(&t);
    char buf[256];
    // Simple format replacement
    strftime(buf, sizeof(buf), "%Y-%m-%d %H:%M:%S", tm_info);
    // Handle PHP-style format tokens
    char result[256];
    const char* f = format;
    char* r = result;
    while (*f && (size_t)(r - result) < 250) {
        if (*f == 'Y') { r += snprintf(r, 5, "%04d", tm_info->tm_year + 1900); f++; }
        else if (*f == 'y') { r += snprintf(r, 3, "%02d", tm_info->tm_year % 100); f++; }
        else if (*f == 'm') { r += snprintf(r, 3, "%02d", tm_info->tm_mon + 1); f++; }
        else if (*f == 'd') { r += snprintf(r, 3, "%02d", tm_info->tm_mday); f++; }
        else if (*f == 'H') { r += snprintf(r, 3, "%02d", tm_info->tm_hour); f++; }
        else if (*f == 'i') { r += snprintf(r, 3, "%02d", tm_info->tm_min); f++; }
        else if (*f == 's') { r += snprintf(r, 3, "%02d", tm_info->tm_sec); f++; }
        else { *r++ = *f++; }
    }
    *r = '\0';
    char* out = malloc(strlen(result) + 1);
    strcpy(out, result);
    return out;
}

int64_t strtotime(const char* s) {
    if (!s) return 0;
    if (strcmp(s, "now") == 0) return (int64_t)time(NULL);
    if (strcmp(s, "tomorrow") == 0) return (int64_t)(time(NULL) + 86400);
    if (strcmp(s, "yesterday") == 0) return (int64_t)(time(NULL) - 86400);
    // Try YYYY-MM-DD
    int y = 0, m = 0, d = 0;
    if (sscanf(s, "%d-%d-%d", &y, &m, &d) == 3) {
        struct tm tm_info = {0};
        tm_info.tm_year = y - 1900;
        tm_info.tm_mon = m - 1;
        tm_info.tm_mday = d;
        return (int64_t)mktime(&tm_info);
    }
    return 0;
}

char* microtime(void) {
    char* r = malloc(64);
#ifdef _WIN32
    FILETIME ft;
    GetSystemTimeAsFileTime(&ft);
    ULARGE_INTEGER uli;
    uli.LowPart = ft.dwLowDateTime;
    uli.HighPart = ft.dwHighDateTime;
    // Convert to Unix time (100ns intervals since 1601-01-01)
    uint64_t t = uli.QuadPart / 10 - 11644473600000000ULL;
    snprintf(r, 64, "0.%06llu %llu", (unsigned long long)(t % 1000000), (unsigned long long)(t / 1000000));
#else
    struct timeval tv;
    gettimeofday(&tv, NULL);
    snprintf(r, 64, "0.%06ld %ld", (long)tv.tv_usec, (long)tv.tv_sec);
#endif
    return r;
}

// ---- curl: High-level HTTP client ----

// Helper: parse URL into proto, host, port, path
static void phprs_parse_url(const char* url, char** proto, char** host, int64_t* port, char** path) {
    const char* rest = url;
    const char* u_proto = "http";
    int64_t u_port = 80;

    if (strncmp(rest, "https://", 8) == 0) {
        u_proto = "https";
        rest += 8;
        u_port = 443;
    } else if (strncmp(rest, "http://", 7) == 0) {
        u_proto = "http";
        rest += 7;
    }

    // Find path separator
    const char* slash = strchr(rest, '/');
    char* host_port;
    if (slash) {
        size_t hp_len = slash - rest;
        host_port = (char*)malloc(hp_len + 1);
        memcpy(host_port, rest, hp_len);
        host_port[hp_len] = '\0';
        *path = strdup(slash);
    } else {
        host_port = strdup(rest);
        *path = strdup("/");
    }

    // Check for port in host
    char* colon = strchr(host_port, ':');
    if (colon) {
        *colon = '\0';
        *host = strdup(host_port);
        u_port = (int64_t)strtoll(colon + 1, NULL, 10);
    } else {
        *host = strdup(host_port);
    }
    *port = u_port;
    *proto = strdup(u_proto);
    free(host_port);
}

// Helper: HTTP request builder with User-Agent and custom headers
static char* phprs_build_request_ext(const char* method, const char* host, const char* path,
                                      const char* headers, const char* body) {
    size_t method_len = strlen(method);
    size_t host_len = strlen(host);
    size_t path_len = strlen(path);
    size_t headers_len = headers ? strlen(headers) : 0;
    size_t body_len = body ? strlen(body) : 0;

    const char* ua = "User-Agent: phprs-curl\r\n";
    size_t ua_len = strlen(ua);
    const char* accept = "Accept: */*\r\n";
    size_t accept_len = strlen(accept);

    size_t req_len = method_len + 1 + path_len + 11 + host_len + 10 + ua_len + accept_len + headers_len + body_len + 40;
    char* req = (char*)malloc(req_len + 256);
    if (!req) return strdup("");

    int offset = 0;
    offset += snprintf(req + offset, req_len + 256 - offset, "%s %s HTTP/1.1\r\nHost: %s\r\nConnection: close\r\n",
                       method, path, host);
    offset += snprintf(req + offset, req_len + 256 - offset, "%s%s", ua, accept);
    if (headers_len > 0) {
        offset += snprintf(req + offset, req_len + 256 - offset, "%s", headers);
    }
    if (body_len > 0) {
        offset += snprintf(req + offset, req_len + 256 - offset, "Content-Length: %zu\r\n", body_len);
    }
    offset += snprintf(req + offset, req_len + 256 - offset, "\r\n%s", body ? body : "");
    return req;
}

// Helper: build JSON response from HTTP raw response
static char* phprs_build_response_json(int64_t status, const char* raw) {
    const char* headers_str = "";
    const char* body_str = "";
    char* headers_copy = NULL;

    if (raw) {
        const char* first_nl = strstr(raw, "\r\n");
        const char* body_start = strstr(raw, "\r\n\r\n");
        if (first_nl && body_start && body_start > first_nl) {
            size_t hdr_len = body_start - first_nl - 2;
            if (hdr_len > 0) {
                headers_copy = (char*)malloc(hdr_len + 1);
                memcpy(headers_copy, first_nl + 2, hdr_len);
                headers_copy[hdr_len] = '\0';
                headers_str = headers_copy;
            }
            body_str = body_start + 4;
        } else if (body_start) {
            body_str = body_start + 4;
        }
    }

    // Escape strings for JSON
    char clean_headers[4096] = "";
    if (headers_str && *headers_str) {
        char* p = clean_headers;
        const char* s = headers_str;
        while (*s && (size_t)(p - clean_headers) < sizeof(clean_headers) - 2) {
            if (*s == '"' || *s == '\\') *p++ = '\\';
            else if (*s == '\r') { s++; continue; }
            else if (*s == '\n') { *p++ = '\\'; *p++ = 'n'; s++; continue; }
            *p++ = *s++;
        }
        *p = '\0';
    }

    char clean_body[4096] = "";
    if (body_str && *body_str) {
        char* p = clean_body;
        const char* s = body_str;
        while (*s && (size_t)(p - clean_body) < sizeof(clean_body) - 2) {
            if (*s == '"' || *s == '\\') *p++ = '\\';
            else if (*s == '\r') { s++; continue; }
            else if (*s == '\n') { *p++ = '\\'; *p++ = 'n'; s++; continue; }
            *p++ = *s++;
        }
        *p = '\0';
    }

    size_t result_size = 512 + strlen(clean_headers) + strlen(clean_body);
    char* result = (char*)malloc(result_size);
    if (headers_copy) free(headers_copy);
    if (!result) return strdup("{\"status\":0,\"headers\":\"\",\"body\":\"\"}");

    snprintf(result, result_size, "{\"status\":%lld,\"headers\":\"%s\",\"body\":\"%s\"}",
             (long long)status, clean_headers, clean_body);
    return result;
}

// phprs_curl: Synchronous HTTP request
// url: full URL (http://... or https://...)
// options_json: JSON: {"method":"GET","timeout":10,"body":"...","headers":"..."}
// Returns JSON: {"status":200,"headers":"...","body":"..."}
const char* phprs_curl(const char* url, const char* options_json) {
    if (!url || !*url) {
        return strdup("{\"status\":0,\"headers\":\"\",\"body\":\"\",\"error\":\"No URL\"}");
    }

    // Parse URL
    char* proto = NULL;
    char* host = NULL;
    int64_t port = 0;
    char* path = NULL;
    phprs_parse_url(url, &proto, &host, &port, &path);

    // Parse options
    const char* method = "GET";
    const char* body = "";
    const char* extra_headers = "";
    char* opt_method = NULL;
    char* opt_body = NULL;
    char* opt_headers = NULL;

    if (options_json && *options_json) {
        opt_method = phprs_json_get_string(options_json, "method");
        opt_body = phprs_json_get_string(options_json, "body");
        opt_headers = phprs_json_get_string(options_json, "headers");
        if (opt_method && *opt_method) method = opt_method;
        if (opt_body && *opt_body) body = opt_body;
        if (opt_headers && *opt_headers) extra_headers = opt_headers;
    }

    // Connect
    int64_t fd = -1;
    char* raw_resp = NULL;
    const char* result;

    if (strcmp(proto, "https") == 0) {
        fd = phprs_tls_connect(host, port);
    } else {
        fd = phprs_tcp_connect(host, port);
    }

    if (fd < 0) {
        char err[256];
        snprintf(err, sizeof(err), "{\"status\":0,\"headers\":\"\",\"body\":\"\",\"error\":\"Connection failed to %s:%lld\"}",
                 host, (long long)port);
        result = strdup(err);
        goto cleanup;
    }

    // Build and send request
    {
        char* req = phprs_build_request_ext(method, host, path, extra_headers, body);
        phprs_socket_write(fd, req);
        free(req);
    }

    // Read response
    raw_resp = phprs_socket_read_all(fd);
    phprs_socket_close(fd);

    {
        int64_t status = phprs_http_response_status(raw_resp);
        char* resp_json = phprs_build_response_json(status, raw_resp);
        free(raw_resp);
        result = resp_json;
    }

cleanup:
    free(proto);
    free(host);
    free(path);
    if (opt_method) free(opt_method);
    if (opt_body) free(opt_body);
    if (opt_headers) free(opt_headers);

    return result;
}

// ---- curl_async: Background threading ----

#define PHPRS_MAX_ASYNC_TASKS 64

typedef struct {
    int64_t handle;
    int done;
    char* result;   // owned JSON string
#ifdef _WIN32
    HANDLE thread_handle;
#else
    pthread_t thread_handle;
#endif
} phprs_async_task_t;

static phprs_async_task_t phprs_async_tasks[PHPRS_MAX_ASYNC_TASKS];
static int64_t phprs_async_counter = 0;
static int phprs_async_mutex_ready = 0;
static phprs_mutex_inner_t phprs_async_mutex;

static void phprs_async_init(void) {
    if (!phprs_async_mutex_ready) {
        phprs_async_mutex_ready = 1;
#ifdef _WIN32
        InitializeCriticalSection(&phprs_async_mutex);
#else
        pthread_mutex_init(&phprs_async_mutex, NULL);
#endif
        memset(phprs_async_tasks, 0, sizeof(phprs_async_tasks));
    }
}

struct phprs_curl_async_arg {
    char* url;
    char* options_json;
    int64_t handle;
};

static PHPRS_THREAD_RETURN phprs_curl_async_worker(void* arg) {
    struct phprs_curl_async_arg* ca = (struct phprs_curl_async_arg*)arg;
    const char* result = phprs_curl(ca->url, ca->options_json);

#ifdef _WIN32
    EnterCriticalSection(&phprs_async_mutex);
#else
    pthread_mutex_lock(&phprs_async_mutex);
#endif
    for (int i = 0; i < PHPRS_MAX_ASYNC_TASKS; i++) {
        if (phprs_async_tasks[i].handle == ca->handle) {
            phprs_async_tasks[i].result = strdup(result);
            phprs_async_tasks[i].done = 1;
            break;
        }
    }
#ifdef _WIN32
    LeaveCriticalSection(&phprs_async_mutex);
#else
    pthread_mutex_unlock(&phprs_async_mutex);
#endif

    free(ca->url);
    free(ca->options_json);
    free(ca);
    return PHPRS_THREAD_RETVAL;
}

int64_t phprs_curl_async(const char* url, const char* options_json) {
    phprs_async_init();
    if (!url || !*url) return -1;

#ifdef _WIN32
    EnterCriticalSection(&phprs_async_mutex);
#else
    pthread_mutex_lock(&phprs_async_mutex);
#endif

    int slot = -1;
    for (int i = 0; i < PHPRS_MAX_ASYNC_TASKS; i++) {
        if (phprs_async_tasks[i].handle == 0 || phprs_async_tasks[i].done) {
            if (phprs_async_tasks[i].done && phprs_async_tasks[i].result) {
                free(phprs_async_tasks[i].result);
            }
#ifdef _WIN32
            if (phprs_async_tasks[i].handle != 0 && phprs_async_tasks[i].done
                && phprs_async_tasks[i].thread_handle) {
                CloseHandle(phprs_async_tasks[i].thread_handle);
            }
#endif
            memset(&phprs_async_tasks[i], 0, sizeof(phprs_async_task_t));
            slot = i;
            break;
        }
    }

    if (slot < 0) {
#ifdef _WIN32
        LeaveCriticalSection(&phprs_async_mutex);
#else
        pthread_mutex_unlock(&phprs_async_mutex);
#endif
        return -1;  // All slots busy
    }

    phprs_async_counter++;
    int64_t handle = phprs_async_counter;
    phprs_async_tasks[slot].handle = handle;

    struct phprs_curl_async_arg* ca = (struct phprs_curl_async_arg*)malloc(sizeof(struct phprs_curl_async_arg));
    ca->url = strdup(url);
    ca->options_json = options_json ? strdup(options_json) : strdup("{}");
    ca->handle = handle;

#ifdef _WIN32
    HANDLE thread = (HANDLE)_beginthreadex(NULL, 0, phprs_curl_async_worker, ca, 0, NULL);
    if (thread) {
        phprs_async_tasks[slot].thread_handle = thread;
    } else {
        phprs_async_tasks[slot].handle = 0;
        handle = -1;
        free(ca->url);
        free(ca->options_json);
        free(ca);
    }
#else
    pthread_t thread;
    if (pthread_create(&thread, NULL, phprs_curl_async_worker, ca) == 0) {
        pthread_detach(thread);
    } else {
        phprs_async_tasks[slot].handle = 0;
        handle = -1;
        free(ca->url);
        free(ca->options_json);
        free(ca);
    }
#endif

#ifdef _WIN32
    LeaveCriticalSection(&phprs_async_mutex);
#else
    pthread_mutex_unlock(&phprs_async_mutex);
#endif

    return handle;
}

int64_t phprs_curl_is_done(int64_t handle) {
    phprs_async_init();
    int64_t found = 0;
#ifdef _WIN32
    EnterCriticalSection(&phprs_async_mutex);
#else
    pthread_mutex_lock(&phprs_async_mutex);
#endif
    for (int i = 0; i < PHPRS_MAX_ASYNC_TASKS; i++) {
        if (phprs_async_tasks[i].handle == handle) {
            found = phprs_async_tasks[i].done ? 1 : 0;
            break;
        }
    }
#ifdef _WIN32
    LeaveCriticalSection(&phprs_async_mutex);
#else
    pthread_mutex_unlock(&phprs_async_mutex);
#endif
    return found;
}

const char* phprs_curl_wait(int64_t handle) {
    phprs_async_init();
    while (1) {
        int done = 0;
        char* result = NULL;
#ifdef _WIN32
        EnterCriticalSection(&phprs_async_mutex);
#else
        pthread_mutex_lock(&phprs_async_mutex);
#endif
        for (int i = 0; i < PHPRS_MAX_ASYNC_TASKS; i++) {
            if (phprs_async_tasks[i].handle == handle) {
                done = phprs_async_tasks[i].done;
                if (done) {
                    result = phprs_async_tasks[i].result;
                    phprs_async_tasks[i].result = NULL;
                    phprs_async_tasks[i].handle = 0;
#ifdef _WIN32
                    if (phprs_async_tasks[i].thread_handle) {
                        CloseHandle(phprs_async_tasks[i].thread_handle);
                        phprs_async_tasks[i].thread_handle = NULL;
                    }
#endif
                }
                break;
            }
        }
#ifdef _WIN32
        LeaveCriticalSection(&phprs_async_mutex);
#else
        pthread_mutex_unlock(&phprs_async_mutex);
#endif
        if (done) {
            if (result) return result;
            return strdup("{\"status\":0,\"headers\":\"\",\"body\":\"\",\"error\":\"No result\"}");
        }
#ifdef _WIN32
        Sleep(10);
#else
        usleep(10000);
#endif
    }
}

// ---- Public API wrappers (short names for PHPRS extern declarations) ----

const char* curl(const char* url, const char* options_json) {
    return phprs_curl(url, options_json);
}

int64_t curl_async(const char* url, const char* options_json) {
    return phprs_curl_async(url, options_json);
}

const char* curl_wait(int64_t handle) {
    return phprs_curl_wait(handle);
}

int64_t curl_is_done(int64_t handle) {
    return phprs_curl_is_done(handle);
}

// ---- String helpers ----

const char* chr(int64_t codepoint) {
    if (codepoint < 0 || codepoint > 0x10FFFF) return strdup("");
    char* r = malloc(5);
    if (!r) return strdup("");
    if (codepoint < 0x80) {
        r[0] = (char)codepoint; r[1] = '\0';
    } else if (codepoint < 0x800) {
        r[0] = (char)(0xC0 | (codepoint >> 6));
        r[1] = (char)(0x80 | (codepoint & 0x3F));
        r[2] = '\0';
    } else if (codepoint < 0x10000) {
        r[0] = (char)(0xE0 | (codepoint >> 12));
        r[1] = (char)(0x80 | ((codepoint >> 6) & 0x3F));
        r[2] = (char)(0x80 | (codepoint & 0x3F));
        r[3] = '\0';
    } else {
        r[0] = (char)(0xF0 | (codepoint >> 18));
        r[1] = (char)(0x80 | ((codepoint >> 12) & 0x3F));
        r[2] = (char)(0x80 | ((codepoint >> 6) & 0x3F));
        r[3] = (char)(0x80 | (codepoint & 0x3F));
        r[4] = '\0';
    }
    return r;
}

int64_t ord(const char* s) {
    if (!s || !*s) return 0;
    unsigned char c = (unsigned char)s[0];
    if (c < 0x80) return (int64_t)c;
    int64_t cp = 0; int bytes = 0;
    if ((c & 0xE0) == 0xC0)      { cp = c & 0x1F; bytes = 2; }
    else if ((c & 0xF0) == 0xE0) { cp = c & 0x0F; bytes = 3; }
    else if ((c & 0xF8) == 0xF0) { cp = c & 0x07; bytes = 4; }
    else return (int64_t)c;
    for (int i = 1; i < bytes; i++) {
        if (((unsigned char)s[i] & 0xC0) != 0x80) return (int64_t)c;
        cp = (cp << 6) | ((unsigned char)s[i] & 0x3F);
    }
    return cp;
}

const char* addslashes(const char* s) {
    if (!s) return strdup("");
    size_t len = strlen(s);
    // Count how many chars need escaping
    size_t extra = 0;
    for (size_t i = 0; i < len; i++) {
        if (s[i] == '\'' || s[i] == '"' || s[i] == '\\') extra++;
    }
    char* r = malloc(len + extra + 1);
    if (!r) return strdup("");
    size_t j = 0;
    for (size_t i = 0; i < len; i++) {
        switch (s[i]) {
            case '\'': r[j++] = '\\'; r[j++] = '\''; break;
            case '"':  r[j++] = '\\'; r[j++] = '"'; break;
            case '\\': r[j++] = '\\'; r[j++] = '\\'; break;
            default:   r[j++] = s[i]; break;
        }
    }
    r[j] = '\0';
    return r;
}

const char* stripslashes(const char* s) {
    if (!s) return strdup("");
    size_t len = strlen(s);
    char* r = malloc(len + 1);
    if (!r) return strdup("");
    size_t j = 0;
    for (size_t i = 0; i < len; i++) {
        if (s[i] == '\\' && i + 1 < len) {
            i++;
            switch (s[i]) {
                case '\'': r[j++] = '\''; break;
                case '"':  r[j++] = '"'; break;
                case '\\': r[j++] = '\\'; break;
                case '0':  r[j++] = '\0'; break;
                default:   r[j++] = '\\'; r[j++] = s[i]; break;
            }
        } else {
            r[j++] = s[i];
        }
    }
    r[j] = '\0';
    return r;
}

// ---- Filesystem functions ----

bool copy(const char* source, const char* dest) {
    if (!source || !dest) return false;
    FILE* fsrc = fopen(source, "rb");
    if (!fsrc) return false;
    FILE* fdst = fopen(dest, "wb");
    if (!fdst) { fclose(fsrc); return false; }
    char buf[8192];
    size_t n;
    while ((n = fread(buf, 1, sizeof(buf), fsrc)) > 0) {
        if (fwrite(buf, 1, n, fdst) != n) {
            fclose(fsrc); fclose(fdst);
            return false;
        }
    }
    fclose(fsrc); fclose(fdst);
    return true;
}

bool rename_(const char* old_path, const char* new_path) {
    if (!old_path || !new_path) return false;
    return rename(old_path, new_path) == 0;
}

int64_t filesize(const char* path) {
    if (!path) return -1;
    struct stat st;
    if (stat(path, &st) != 0) return -1;
    if (!S_ISREG(st.st_mode)) return -1;
    return (int64_t)st.st_size;
}

int64_t filemtime(const char* path) {
    if (!path) return -1;
    struct stat st;
    if (stat(path, &st) != 0) return -1;
    return (int64_t)st.st_mtime;
}

const char* pathinfo(const char* path) {
    if (!path) return strdup("{\"dirname\":\"\",\"basename\":\"\",\"extension\":\"\",\"filename\":\"\"}");
    char dirname[1024] = "";
    char basename[256] = "";
    char filename[256] = "";
    char extension[64] = "";

    // Find last separator for dirname
    const char* sep = strrchr(path, '/');
    const char* sep2 = strrchr(path, '\\');
    if (sep2 && (!sep || sep2 > sep)) sep = sep2;

    if (sep) {
        size_t dlen = sep - path;
        if (dlen >= sizeof(dirname)) dlen = sizeof(dirname) - 1;
        memcpy(dirname, path, dlen);
        dirname[dlen] = '\0';
        strncpy(basename, sep + 1, sizeof(basename) - 1);
        basename[sizeof(basename) - 1] = '\0';
    } else {
        dirname[0] = '\0';
        strncpy(basename, path, sizeof(basename) - 1);
        basename[sizeof(basename) - 1] = '\0';
    }

    // Extension: find last '.'
    const char* dot = strrchr(basename, '.');
    if (dot && dot != basename) {
        size_t flen = dot - basename;
        if (flen >= sizeof(filename)) flen = sizeof(filename) - 1;
        memcpy(filename, basename, flen);
        filename[flen] = '\0';
        strncpy(extension, dot + 1, sizeof(extension) - 1);
        extension[sizeof(extension) - 1] = '\0';
    } else {
        strncpy(filename, basename, sizeof(filename) - 1);
        filename[sizeof(filename) - 1] = '\0';
        extension[0] = '\0';
    }

    // JSON-escape each component
    char esc_dirname[2048] = "";
    char esc_basename[512] = "";
    char esc_filename[512] = "";
    char esc_extension[128] = "";
    const char* srcs[] = { dirname, basename, filename, extension };
    char* dsts[] = { esc_dirname, esc_basename, esc_filename, esc_extension };
    size_t dstsz[] = { sizeof(esc_dirname), sizeof(esc_basename), sizeof(esc_filename), sizeof(esc_extension) };
    for (int k = 0; k < 4; k++) {
        const char* s = srcs[k];
        char* d = dsts[k];
        size_t j = 0, mx = dstsz[k] - 1;
        for (size_t i = 0; s[i] && j < mx - 1; i++) {
            switch (s[i]) {
                case '"':  if (j+1<mx) { d[j++]='\\'; d[j++]='"'; } break;
                case '\\': if (j+1<mx) { d[j++]='\\'; d[j++]='\\'; } break;
                case '\n': if (j+1<mx) { d[j++]='\\'; d[j++]='n'; } break;
                case '\r': if (j+1<mx) { d[j++]='\\'; d[j++]='r'; } break;
                case '\t': if (j+1<mx) { d[j++]='\\'; d[j++]='t'; } break;
                default:   d[j++] = s[i]; break;
            }
        }
        d[j] = '\0';
    }

    char* result = malloc(4096);
    if (!result) return strdup("{}");
    snprintf(result, 4096,
        "{\"dirname\":\"%s\",\"basename\":\"%s\",\"extension\":\"%s\",\"filename\":\"%s\"}",
        esc_dirname, esc_basename, esc_extension, esc_filename);
    return result;
}

bool move_uploaded_file(const char* tmp, const char* dest) {
    if (!tmp || !dest) return false;
    // Basic safety: check source exists
    struct stat st;
    if (stat(tmp, &st) != 0) return false;
    return rename(tmp, dest) == 0;
}

// ---- Security functions ----

// Cross-platform secure random bytes helper. Returns 1 on success, 0 on failure.
static int phprs_secure_random(void* buf, size_t len) {
#ifdef _WIN32
    // Use RtlGenRandom (SystemFunction036) available since Windows XP
    typedef BOOLEAN (APIENTRY *RtlGenRandomFn)(PVOID, ULONG);
    HMODULE hLib = LoadLibraryA("advapi32.dll");
    if (hLib) {
        RtlGenRandomFn fn = (RtlGenRandomFn)GetProcAddress(hLib, "SystemFunction036");
        if (fn && fn(buf, (ULONG)len)) { FreeLibrary(hLib); return 1; }
        FreeLibrary(hLib);
    }
    return 0;
#else
    FILE* f = fopen("/dev/urandom", "rb");
    if (f) {
        size_t rd = fread(buf, 1, len, f);
        fclose(f);
        if (rd == len) return 1;
    }
    return 0;
#endif
}

const char* random_bytes(int64_t length) {
    if (length < 1) length = 1;
    if (length > 1024 * 1024) length = 1024 * 1024;
    size_t n = (size_t)length;
    unsigned char* buf = malloc(n);
    if (!buf) return strdup("");
    if (!phprs_secure_random(buf, n)) {
        free(buf);
        return strdup("");
    }
    char* hex = malloc(n * 2 + 1);
    if (!hex) { free(buf); return strdup(""); }
    for (size_t i = 0; i < n; i++) {
        snprintf(hex + i * 2, 3, "%02x", buf[i]);
    }
    hex[n * 2] = '\0';
    free(buf);
    return hex;
}

int64_t random_int(int64_t min, int64_t max) {
    if (min > max) return min;
    uint64_t range = (uint64_t)(max - min);
    if (range == 0) return min;
    uint64_t threshold = UINT64_MAX - (UINT64_MAX % (range + 1));
    uint64_t val = 0;
    int attempts = 0;
    do {
        if (!phprs_secure_random(&val, sizeof(val))) {
            val = (uint64_t)time(NULL) ^ (uint64_t)attempts;
            break;
        }
        attempts++;
    } while (val >= threshold && attempts < 128);
    return min + (int64_t)(val % (range + 1));
}

const char* password_hash(const char* password, const char* algo) {
    if (!password) password = "";
    size_t pwlen = strlen(password);

    // Generate 16-byte random salt
    unsigned char salt_bytes[16];
    if (!phprs_secure_random(salt_bytes, 16)) {
        return strdup("");
    }
    char salt_hex[33];
    for (int i = 0; i < 16; i++) snprintf(salt_hex + i * 2, 3, "%02x", salt_bytes[i]);
    salt_hex[32] = '\0';

    // Determine algorithm string — only sha1 is supported
    const char* algo_str = algo;
    if (!algo_str || algo_str[0] == '\0') algo_str = "sha1";
    if (strcmp(algo_str, "sha1") != 0) {
        algo_str = "sha1";
    }

    // Repeated SHA-1: 10000 iterations
    // First: hash = SHA1(salt_hex + password)
    size_t initial_len = 32 + pwlen;
    unsigned char* initial = malloc(initial_len);
    if (!initial) return strdup("");
    memcpy(initial, salt_hex, 32);
    memcpy(initial + 32, password, pwlen);
    unsigned char hash[20];
    phprs_sha1(initial, initial_len, hash);
    free(initial);

    for (int iter = 0; iter < 9999; iter++) {
        unsigned char* combined = malloc(20 + pwlen);
        if (!combined) return strdup("");
        memcpy(combined, hash, 20);
        memcpy(combined + 20, password, pwlen);
        phprs_sha1(combined, 20 + pwlen, hash);
        free(combined);
    }
    char hash_hex[41];
    for (int i = 0; i < 20; i++) snprintf(hash_hex + i * 2, 3, "%02x", hash[i]);
    hash_hex[40] = '\0';

    size_t result_len = strlen(algo_str) + 1 + 32 + 1 + 40 + 1;
    char* result = malloc(result_len);
    if (!result) return strdup("");
    snprintf(result, result_len, "%s$%s$%s", algo_str, salt_hex, hash_hex);
    return result;
}

bool password_verify(const char* password, const char* stored_hash) {
    if (!password || !stored_hash) return false;

    // Parse: algo$salt$hash
    const char* dollar1 = strchr(stored_hash, '$');
    if (!dollar1) return false;
    const char* dollar2 = strchr(dollar1 + 1, '$');
    if (!dollar2) return false;

    size_t salt_len = dollar2 - dollar1 - 1;
    if (salt_len > 64) return false;
    char salt_hex[65];
    memcpy(salt_hex, dollar1 + 1, salt_len);
    salt_hex[salt_len] = '\0';

    const char* expected_hex = dollar2 + 1;
    size_t pwlen = strlen(password);

    // Recompute hash
    size_t initial_len = salt_len + pwlen;
    unsigned char* initial = malloc(initial_len);
    if (!initial) return false;
    memcpy(initial, salt_hex, salt_len);
    memcpy(initial + salt_len, password, pwlen);
    unsigned char hash[20];
    phprs_sha1(initial, initial_len, hash);
    free(initial);

    for (int iter = 0; iter < 9999; iter++) {
        unsigned char* combined = malloc(20 + pwlen);
        if (!combined) return false;
        memcpy(combined, hash, 20);
        memcpy(combined + 20, password, pwlen);
        phprs_sha1(combined, 20 + pwlen, hash);
        free(combined);
    }
    char computed_hex[41];
    for (int i = 0; i < 20; i++) snprintf(computed_hex + i * 2, 3, "%02x", hash[i]);
    computed_hex[40] = '\0';

    // Constant-time comparison
    size_t explen = strlen(expected_hex);
    if (explen != 40) return false;
    int diff = 0;
    for (size_t i = 0; i < 40; i++) {
        diff |= (computed_hex[i] ^ expected_hex[i]);
    }
    return diff == 0;
}

// ---- Rate Limiting ----

#define RATE_LIMIT_SLOTS 256

typedef struct {
    char ip[46];
    int64_t window_start;
    int count;
} rate_limit_slot;

static rate_limit_slot g_rate_slots[RATE_LIMIT_SLOTS];
static int g_rate_max_requests = 100;
static int g_rate_window_secs = 60;

static unsigned rate_limit_hash(const char* ip) {
    unsigned h = 5381;
    for (const char* p = ip; *p; p++) {
        h = ((h << 5) + h) + (unsigned char)*p;
    }
    return h % RATE_LIMIT_SLOTS;
}

void phprs_rate_limit_init(int64_t max_req, int64_t window_sec) {
    g_rate_max_requests = (int)max_req;
    g_rate_window_secs = (int)window_sec;
    memset(g_rate_slots, 0, sizeof(g_rate_slots));
}

int64_t phprs_rate_limit_check(const char* ip) {
    if (!ip || !*ip) return 0;

    unsigned slot = rate_limit_hash(ip);
    int64_t now = (int64_t)time(NULL);

    // Reset slot if window expired or IP changed
    if (g_rate_slots[slot].window_start == 0 ||
        now - g_rate_slots[slot].window_start >= g_rate_window_secs ||
        strcmp(g_rate_slots[slot].ip, ip) != 0) {
        strncpy(g_rate_slots[slot].ip, ip, 45);
        g_rate_slots[slot].ip[45] = '\0';
        g_rate_slots[slot].window_start = now;
        g_rate_slots[slot].count = 0;
    }

    g_rate_slots[slot].count++;
    return (g_rate_slots[slot].count <= g_rate_max_requests) ? 1 : 0;
}

// ---- CORS ----

static char g_cors_origin[128] = "*";
static char g_cors_methods[256] = "GET,POST,PUT,DELETE,PATCH,OPTIONS";
static char g_cors_headers[512] = "Content-Type,Authorization";

void phprs_cors_set_config(const char* origin, const char* methods, const char* headers) {
    if (origin && origin[0]) {
        strncpy(g_cors_origin, origin, sizeof(g_cors_origin) - 1);
        g_cors_origin[sizeof(g_cors_origin) - 1] = '\0';
    }
    if (methods && methods[0]) {
        strncpy(g_cors_methods, methods, sizeof(g_cors_methods) - 1);
        g_cors_methods[sizeof(g_cors_methods) - 1] = '\0';
    }
    if (headers && headers[0]) {
        strncpy(g_cors_headers, headers, sizeof(g_cors_headers) - 1);
        g_cors_headers[sizeof(g_cors_headers) - 1] = '\0';
    }
}

const char* phprs_cors_get_origin(void)   { return g_cors_origin; }
const char* phprs_cors_get_methods(void)   { return g_cors_methods; }
const char* phprs_cors_get_headers(void)   { return g_cors_headers; }

int64_t phprs_cors_is_preflight(const char* raw) {
    if (!raw) return 0;
    // Check if first line starts with "OPTIONS "
    return (strncmp(raw, "OPTIONS ", 8) == 0) ? 1 : 0;
}

// ---- Batch 2: Type Casting ----

int64_t intval(const char* s, int64_t base) {
    if (!s || !*s) return 0;
    while (*s == ' ' || *s == '\t' || *s == '\n' || *s == '\r') s++;
    if (base == 10) {
        return (int64_t)strtoll(s, NULL, 10);
    }
    const char* p = s;
    if (base == 16 && (p[0] == '0' && (p[1] == 'x' || p[1] == 'X'))) p += 2;
    return (int64_t)strtoll(p, NULL, (int)base);
}

double floatval(const char* s) {
    if (!s || !*s) return 0.0;
    return strtod(s, NULL);
}

const char* strval_fn(int64_t n) {
    char* buf = malloc(32);
    if (!buf) return strdup("");
    snprintf(buf, 32, "%" PRId64, n);
    return buf;
}

bool boolval(const char* s) {
    if (!s) return false;
    return s[0] != '\0' && !(s[0] == '0' && s[1] == '\0');
}

// ---- Batch 2: String Functions ----

const char* str_pad(const char* input, int64_t length, const char* pad, int64_t pad_type) {
    if (!input) input = "";
    if (!pad || !*pad) pad = " ";
    size_t input_len = strlen(input);
    if ((int64_t)input_len >= length) return strdup(input);
    size_t pad_needed = (size_t)(length - (int64_t)input_len);
    size_t pad_str_len = strlen(pad);
    char* padding = malloc(pad_needed + 1);
    if (!padding) return strdup(input);
    for (size_t i = 0; i < pad_needed; i++) padding[i] = pad[i % pad_str_len];
    padding[pad_needed] = '\0';

    size_t total = input_len + pad_needed + 1;
    char* result = malloc(total);
    if (!result) { free(padding); return strdup(input); }

    if (pad_type == 1) { // STR_PAD_LEFT
        memcpy(result, padding, pad_needed);
        memcpy(result + pad_needed, input, input_len);
    } else if (pad_type == 2) { // STR_PAD_BOTH
        size_t left = pad_needed / 2;
        size_t right = pad_needed - left;
        memcpy(result, padding, left);
        memcpy(result + left, input, input_len);
        memcpy(result + left + input_len, padding, right);
    } else { // STR_PAD_RIGHT
        memcpy(result, input, input_len);
        memcpy(result + input_len, padding, pad_needed);
    }
    result[total - 1] = '\0';
    free(padding);
    return result;
}

const char* wordwrap(const char* str, int64_t width, const char* brk, bool cut_long) {
    if (!str) return strdup("");
    if (!brk) brk = "\n";
    if (width <= 0) width = 75;
    size_t slen = strlen(str);
    size_t blen = strlen(brk);
    size_t alloc = slen * 2 + blen * (slen / (size_t)width + 1) + 1;
    char* result = malloc(alloc);
    if (!result) return strdup(str);
    size_t ri = 0, line_len = 0, i = 0;
    while (i < slen) {
        if (str[i] == ' ') {
            // Find next word length
            size_t wstart = i + 1;
            size_t wend = wstart;
            while (wend < slen && str[wend] != ' ') wend++;
            size_t wlen = wend - wstart;
            if (line_len + 1 + wlen > (size_t)width && line_len > 0) {
                memcpy(result + ri, brk, blen); ri += blen;
                line_len = 0;
            } else {
                result[ri++] = ' '; line_len++;
            }
            i++;
        } else {
            if (cut_long && line_len >= (size_t)width) {
                memcpy(result + ri, brk, blen); ri += blen;
                line_len = 0;
            }
            result[ri++] = str[i++]; line_len++;
        }
    }
    result[ri] = '\0';
    return result;
}

int64_t str_word_count(const char* s) {
    if (!s || !*s) return 0;
    int64_t count = 0;
    bool in_word = false;
    for (; *s; s++) {
        if (*s == ' ' || *s == '\t' || *s == '\n' || *s == '\r') {
            in_word = false;
        } else if (!in_word) {
            in_word = true;
            count++;
        }
    }
    return count;
}

const char* chunk_split(const char* body, int64_t chunklen, const char* end) {
    if (!body) return strdup("");
    if (!end) end = "\r\n";
    if (chunklen < 1) chunklen = 76;
    size_t blen = strlen(body);
    size_t elen = strlen(end);
    size_t chunks = (blen + (size_t)chunklen - 1) / (size_t)chunklen;
    char* result = malloc(blen + chunks * elen + 1);
    if (!result) return strdup(body);
    size_t ri = 0, i = 0;
    while (i < blen) {
        size_t take = (size_t)chunklen;
        if (i + take > blen) take = blen - i;
        memcpy(result + ri, body + i, take); ri += take; i += take;
        memcpy(result + ri, end, elen); ri += elen;
    }
    result[ri] = '\0';
    return result;
}

// ---- Batch 2: Array Functions (C stubs — arrays in compiled mode are handled by codegen) ----

// array_splice, array_pad, array_key_first, array_key_last, array_is_list
// These operate on PHPRS's JSON-encoded array representation in compiled mode.

// ---- Batch 2: Math/Date ----

double fmod_(double x, double y) {
    return fmod(x, y);
}

int64_t intdiv(int64_t a, int64_t b) {
    if (b == 0) return 0;
    return a / b;
}

bool checkdate(int64_t month, int64_t day, int64_t year) {
    if (year < 1 || year > 32767 || month < 1 || month > 12 || day < 1) return false;
    int days_in_month[] = {31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31};
    if ((year % 4 == 0 && year % 100 != 0) || year % 400 == 0) days_in_month[1] = 29;
    return day <= days_in_month[month - 1];
}

int64_t mktime_(int64_t hour, int64_t min, int64_t sec, int64_t month, int64_t day, int64_t year) {
    struct tm t;
    memset(&t, 0, sizeof(t));
    t.tm_year = (int)(year - 1900);
    t.tm_mon = (int)(month - 1);
    t.tm_mday = (int)day;
    t.tm_hour = (int)hour;
    t.tm_min = (int)min;
    t.tm_sec = (int)sec;
    t.tm_isdst = -1;
    time_t result = mktime(&t);
    return (int64_t)result;
}

// str_starts_with / str_ends_with
bool str_starts_with(const char* haystack, const char* needle) {
    if (!haystack || !needle) return false;
    size_t nlen = strlen(needle);
    return strncmp(haystack, needle, nlen) == 0;
}

bool str_ends_with(const char* haystack, const char* needle) {
    if (!haystack || !needle) return false;
    size_t hlen = strlen(haystack);
    size_t nlen = strlen(needle);
    if (nlen > hlen) return false;
    return strcmp(haystack + hlen - nlen, needle) == 0;
}
