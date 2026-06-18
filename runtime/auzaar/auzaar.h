/* auzaar.h — the wow runtime + standard toolbox, C edition.
 *
 * This single header is written next to every .c file the wow compiler emits
 * and is #included by it. It is header-only (everything is `static`) so a wow
 * program compiles as one translation unit with no separate link step:
 *
 *     gcc program.c -o program -lm
 *
 * Two jobs:
 *   1. The runtime — a dynamically typed value (`WowValue`) so a kid can write
 *      `x = 5` then `x = "salam"` and have it work, even though C is static.
 *   2. auzaar — the built-in toolbox (math, strings, collections), auto-loaded.
 *
 * Memory: this Phase-1 runtime never frees. wow programs for beginners are
 * short-lived, so leaking is a fine trade for simplicity. A later phase can add
 * reference counting or an arena.
 */
#ifndef AUZAAR_H
#define AUZAAR_H

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <math.h>
#include <time.h>
#include <setjmp.h>

/* ================================================================
 * Error handling (koshish / pakdo)
 *
 * C has no exceptions, so we keep a stack of jump targets. wow_raise jumps to
 * the innermost active koshish; with none active it prints the message and
 * exits. The compiler manages the stack so early `do`/`roko`/`aage` out of a
 * koshish body still unwind correctly.
 * ================================================================ */

#define WOW_MAX_HANDLERS 64
static jmp_buf wow_handlers[WOW_MAX_HANDLERS];
static int wow_handler_top = 0;
static char wow_error_msg[256];

static void wow_raise(const char *msg) {
    size_t n = strlen(msg);
    if (n >= sizeof(wow_error_msg)) n = sizeof(wow_error_msg) - 1;
    memcpy(wow_error_msg, msg, n);
    wow_error_msg[n] = '\0';
    if (wow_handler_top > 0) {
        longjmp(wow_handlers[wow_handler_top - 1], 1);
    }
    fprintf(stderr, "Ghalti: %s\n", wow_error_msg);
    exit(1);
}

/* ================================================================
 * The value type
 * ================================================================ */

typedef enum { WOW_NULL, WOW_NUM, WOW_BOOL, WOW_STR, WOW_LIST, WOW_OBJ } WowType;

struct WowValue;
typedef struct WowValue WowValue;

typedef struct {
    WowValue *items;
    int len;
    int cap;
} WowList;

struct WowObj;
typedef struct WowObj WowObj;

struct WowValue {
    WowType type;
    union {
        double   num;     /* WOW_NUM  */
        int      boolean; /* WOW_BOOL */
        char    *str;     /* WOW_STR  — heap owned */
        WowList *list;    /* WOW_LIST — heap owned */
        WowObj  *obj;     /* WOW_OBJ  — heap owned */
    } as;
};

typedef struct { char *key; WowValue val; } WowEntry;
struct WowObj {
    WowEntry *entries;
    int len;
    int cap;
};

/* ================================================================
 * Tiny helpers
 * ================================================================ */

static char *wow_strdup(const char *s) {
    size_t n = strlen(s);
    char *p = (char *)malloc(n + 1);
    memcpy(p, s, n + 1);
    return p;
}

/* ================================================================
 * Constructors
 * ================================================================ */

static WowValue wow_null(void) {
    WowValue v; v.type = WOW_NULL; v.as.num = 0; return v;
}
static WowValue wow_num(double n) {
    WowValue v; v.type = WOW_NUM; v.as.num = n; return v;
}
static WowValue wow_bool(int b) {
    WowValue v; v.type = WOW_BOOL; v.as.boolean = b ? 1 : 0; return v;
}
static WowValue wow_str(const char *s) {
    WowValue v; v.type = WOW_STR; v.as.str = wow_strdup(s); return v;
}
/* takes ownership of an already-heap-allocated buffer */
static WowValue wow_str_owned(char *s) {
    WowValue v; v.type = WOW_STR; v.as.str = s; return v;
}

static WowValue wow_list_new(void) {
    WowList *l = (WowList *)malloc(sizeof(WowList));
    l->len = 0; l->cap = 4;
    l->items = (WowValue *)malloc(sizeof(WowValue) * l->cap);
    WowValue v; v.type = WOW_LIST; v.as.list = l; return v;
}
static void wow_list_push(WowValue list, WowValue item) {
    WowList *l = list.as.list;
    if (l->len == l->cap) {
        l->cap *= 2;
        l->items = (WowValue *)realloc(l->items, sizeof(WowValue) * l->cap);
    }
    l->items[l->len++] = item;
}
/* build a list literal: wow_list_lit(3, (WowValue[]){a, b, c}) */
static WowValue wow_list_lit(int n, WowValue *items) {
    WowValue list = wow_list_new();
    for (int i = 0; i < n; i++) wow_list_push(list, items[i]);
    return list;
}

/* ================================================================
 * Object constructors and access
 * ================================================================ */

static WowValue wow_obj_new(void) {
    WowObj *o = (WowObj *)malloc(sizeof(WowObj));
    o->len = 0; o->cap = 4;
    o->entries = (WowEntry *)malloc(sizeof(WowEntry) * o->cap);
    WowValue v; v.type = WOW_OBJ; v.as.obj = o; return v;
}

/* upsert: add new key or overwrite existing */
static void wow_obj_set(WowValue obj, const char *key, WowValue val) {
    if (obj.type != WOW_OBJ) return;
    WowObj *o = obj.as.obj;
    for (int i = 0; i < o->len; i++) {
        if (strcmp(o->entries[i].key, key) == 0) { o->entries[i].val = val; return; }
    }
    if (o->len == o->cap) {
        o->cap *= 2;
        o->entries = (WowEntry *)realloc(o->entries, sizeof(WowEntry) * o->cap);
    }
    o->entries[o->len].key = wow_strdup(key);
    o->entries[o->len].val = val;
    o->len++;
}

/* get — crashes with wow_raise if obj is not an object or key is missing */
static WowValue wow_obj_get(WowValue obj, const char *key) {
    if (obj.type == WOW_OBJ) {
        WowObj *o = obj.as.obj;
        for (int i = 0; i < o->len; i++)
            if (strcmp(o->entries[i].key, key) == 0) return o->entries[i].val;
    }
    char msg[128];
    snprintf(msg, sizeof(msg), "'%s' key nahi mila", key);
    wow_raise(msg);
    return wow_null();
}

/* safe get — returns khali if obj is not an object or key is missing */
static WowValue wow_safe_get(WowValue obj, const char *key) {
    if (obj.type != WOW_OBJ) return wow_null();
    WowObj *o = obj.as.obj;
    for (int i = 0; i < o->len; i++)
        if (strcmp(o->entries[i].key, key) == 0) return o->entries[i].val;
    return wow_null();
}

/* build an object literal from parallel key/value arrays */
static WowValue wow_obj_lit(int n, const char **keys, WowValue *vals) {
    WowValue obj = wow_obj_new();
    for (int i = 0; i < n; i++) wow_obj_set(obj, keys[i], vals[i]);
    return obj;
}

/* ================================================================
 * Coercions
 * ================================================================ */

static double wow_as_num(WowValue v) {
    switch (v.type) {
        case WOW_NUM:  return v.as.num;
        case WOW_BOOL: return v.as.boolean ? 1.0 : 0.0;
        case WOW_STR:  return atof(v.as.str);
        default:       return 0.0;
    }
}

static int wow_truthy(WowValue v) {
    switch (v.type) {
        case WOW_NULL: return 0;
        case WOW_BOOL: return v.as.boolean;
        case WOW_NUM:  return v.as.num != 0.0;
        case WOW_STR:  return v.as.str[0] != '\0';
        case WOW_LIST: return v.as.list->len != 0;
        case WOW_OBJ:  return v.as.obj->len != 0;
    }
    return 0;
}

/* a clean number string: 5 not 5.000000, 5.5 stays 5.5 */
static char *wow_num_to_cstr(double n) {
    char *buf = (char *)malloc(32);
    if (isfinite(n) && n == (double)(long long)n && fabs(n) < 1e15) {
        snprintf(buf, 32, "%lld", (long long)n);
    } else {
        snprintf(buf, 32, "%.10g", n);
    }
    return buf;
}

static WowValue wow_to_str(WowValue v) {
    switch (v.type) {
        case WOW_STR:  return v;
        case WOW_NUM:  return wow_str_owned(wow_num_to_cstr(v.as.num));
        case WOW_BOOL: return wow_str(v.as.boolean ? "sahi" : "ghalat");
        case WOW_NULL: return wow_str("khali");
        case WOW_LIST: {
            WowList *l = v.as.list;
            /* start with "[" and grow */
            size_t cap = 2, len = 0;
            char *buf = (char *)malloc(cap);
            buf[len++] = '[';
            for (int i = 0; i < l->len; i++) {
                if (i > 0) {
                    char *grown = (char *)realloc(buf, len + 3);
                    buf = grown; buf[len++] = ','; buf[len++] = ' ';
                }
                WowValue s = wow_to_str(l->items[i]);
                size_t sl = strlen(s.as.str);
                buf = (char *)realloc(buf, len + sl + 2);
                memcpy(buf + len, s.as.str, sl); len += sl;
            }
            buf = (char *)realloc(buf, len + 2);
            buf[len++] = ']'; buf[len] = '\0';
            return wow_str_owned(buf);
        }
        case WOW_OBJ: {
            WowObj *o = v.as.obj;
            size_t cap = 4, len = 0;
            char *buf = (char *)malloc(cap);
            buf[len++] = '{';
            for (int i = 0; i < o->len; i++) {
                const char *sep = i > 0 ? ", " : " ";
                size_t sepl = strlen(sep);
                buf = (char *)realloc(buf, len + sepl + 1);
                memcpy(buf + len, sep, sepl); len += sepl;
                /* key */
                size_t kl = strlen(o->entries[i].key);
                buf = (char *)realloc(buf, len + kl + 3);
                memcpy(buf + len, o->entries[i].key, kl); len += kl;
                buf[len++] = ':'; buf[len++] = ' ';
                /* value — strings get quotes, others print as-is */
                WowValue sv = wow_to_str(o->entries[i].val);
                size_t vl = strlen(sv.as.str);
                if (o->entries[i].val.type == WOW_STR) {
                    buf = (char *)realloc(buf, len + vl + 3);
                    buf[len++] = '"';
                    memcpy(buf + len, sv.as.str, vl); len += vl;
                    buf[len++] = '"';
                } else {
                    buf = (char *)realloc(buf, len + vl + 1);
                    memcpy(buf + len, sv.as.str, vl); len += vl;
                }
            }
            buf = (char *)realloc(buf, len + 3);
            if (o->len > 0) buf[len++] = ' ';
            buf[len++] = '}'; buf[len] = '\0';
            return wow_str_owned(buf);
        }
    }
    return wow_str("khali");
}

/* ================================================================
 * Output and input
 * ================================================================ */

/* bol — print a value and a newline */
static WowValue wow_print(WowValue v) {
    WowValue s = wow_to_str(v);
    printf("%s\n", s.as.str);
    return wow_null();
}

/* pucho — print a prompt (no newline) and read a line of input */
static WowValue wow_pucho(WowValue prompt) {
    WowValue s = wow_to_str(prompt);
    printf("%s", s.as.str);
    fflush(stdout);

    size_t cap = 64, len = 0;
    char *buf = (char *)malloc(cap);
    int c;
    while ((c = getchar()) != EOF && c != '\n') {
        if (len + 1 >= cap) { cap *= 2; buf = (char *)realloc(buf, cap); }
        buf[len++] = (char)c;
    }
    buf[len] = '\0';
    return wow_str_owned(buf);
}

/* ================================================================
 * Arithmetic, comparison, logic
 * ================================================================ */

/* + concatenates if either side is a string, otherwise adds numbers */
static WowValue wow_add(WowValue a, WowValue b) {
    if (a.type == WOW_STR || b.type == WOW_STR) {
        WowValue sa = wow_to_str(a), sb = wow_to_str(b);
        size_t la = strlen(sa.as.str), lb = strlen(sb.as.str);
        char *buf = (char *)malloc(la + lb + 1);
        memcpy(buf, sa.as.str, la);
        memcpy(buf + la, sb.as.str, lb);
        buf[la + lb] = '\0';
        return wow_str_owned(buf);
    }
    return wow_num(wow_as_num(a) + wow_as_num(b));
}
static WowValue wow_sub(WowValue a, WowValue b) { return wow_num(wow_as_num(a) - wow_as_num(b)); }
static WowValue wow_mul(WowValue a, WowValue b) { return wow_num(wow_as_num(a) * wow_as_num(b)); }
static WowValue wow_div(WowValue a, WowValue b) {
    double d = wow_as_num(b);
    if (d == 0.0) wow_raise("sifr se taqseem nahi ho sakta");
    return wow_num(wow_as_num(a) / d);
}
static WowValue wow_mod(WowValue a, WowValue b) {
    double d = wow_as_num(b);
    if (d == 0.0) wow_raise("sifr se taqseem nahi ho sakta");
    return wow_num(fmod(wow_as_num(a), d));
}
static WowValue wow_neg(WowValue a)             { return wow_num(-wow_as_num(a)); }

static int wow_equal(WowValue a, WowValue b) {
    if (a.type != b.type) {
        /* numbers and bools compare across the line */
        if ((a.type == WOW_NUM || a.type == WOW_BOOL) &&
            (b.type == WOW_NUM || b.type == WOW_BOOL)) {
            return wow_as_num(a) == wow_as_num(b);
        }
        return 0;
    }
    switch (a.type) {
        case WOW_NULL: return 1;
        case WOW_NUM:  return a.as.num == b.as.num;
        case WOW_BOOL: return a.as.boolean == b.as.boolean;
        case WOW_STR:  return strcmp(a.as.str, b.as.str) == 0;
        case WOW_LIST: return a.as.list == b.as.list;
        case WOW_OBJ:  return a.as.obj  == b.as.obj;
    }
    return 0;
}

static WowValue wow_eq(WowValue a, WowValue b)  { return wow_bool(wow_equal(a, b)); }
static WowValue wow_neq(WowValue a, WowValue b) { return wow_bool(!wow_equal(a, b)); }
static WowValue wow_lt(WowValue a, WowValue b)  { return wow_bool(wow_as_num(a) <  wow_as_num(b)); }
static WowValue wow_lte(WowValue a, WowValue b) { return wow_bool(wow_as_num(a) <= wow_as_num(b)); }
static WowValue wow_gt(WowValue a, WowValue b)  { return wow_bool(wow_as_num(a) >  wow_as_num(b)); }
static WowValue wow_gte(WowValue a, WowValue b) { return wow_bool(wow_as_num(a) >= wow_as_num(b)); }

static WowValue wow_and(WowValue a, WowValue b) { return wow_bool(wow_truthy(a) && wow_truthy(b)); }
static WowValue wow_or(WowValue a, WowValue b)  { return wow_truthy(a) ? a : b; }
static WowValue wow_not(WowValue a)             { return wow_bool(!wow_truthy(a)); }

/* ================================================================
 * List access (used by loops and indexing)
 * ================================================================ */

static int wow_count(WowValue v) {
    if (v.type == WOW_LIST) return v.as.list->len;
    if (v.type == WOW_STR)  return (int)strlen(v.as.str);
    return 0;
}
static WowValue wow_at(WowValue list, int i) {
    if (list.type == WOW_LIST && i >= 0 && i < list.as.list->len)
        return list.as.list->items[i];
    return wow_null();
}

/* ================================================================
 * auzaar — sleep (shared keyword `intezar`)
 * ================================================================ */

static WowValue wow_intezar(WowValue ms) {
    double m = wow_as_num(ms);
    struct timespec ts;
    ts.tv_sec  = (time_t)(m / 1000.0);
    ts.tv_nsec = (long)((m - ts.tv_sec * 1000.0) * 1.0e6);
    nanosleep(&ts, NULL);
    return wow_null();
}

/* ================================================================
 * auzaar — math (kids meet these names in math class)
 * ================================================================ */

static WowValue wow_round(WowValue n)       { return wow_num(round(wow_as_num(n))); }
static WowValue wow_round_up(WowValue n)    { return wow_num(ceil(wow_as_num(n))); }
static WowValue wow_round_down(WowValue n)  { return wow_num(floor(wow_as_num(n))); }
static WowValue wow_square_root(WowValue n) { return wow_num(sqrt(wow_as_num(n))); }
static WowValue wow_power(WowValue n, WowValue p) { return wow_num(pow(wow_as_num(n), wow_as_num(p))); }
static WowValue wow_absolute(WowValue n)    { return wow_num(fabs(wow_as_num(n))); }

static WowValue wow_random(void) { return wow_num((double)rand() / (double)RAND_MAX); }
static WowValue wow_random_number(WowValue lo, WowValue hi) {
    int a = (int)wow_as_num(lo), b = (int)wow_as_num(hi);
    if (b < a) { int t = a; a = b; b = t; }
    return wow_num(a + rand() % (b - a + 1));
}

/* ================================================================
 * auzaar — strings
 * ================================================================ */

static WowValue wow_lambai(WowValue s)      { return wow_num(strlen(wow_to_str(s).as.str)); }
static WowValue wow_bara_likho(WowValue s)  {
    char *p = wow_strdup(wow_to_str(s).as.str);
    for (char *q = p; *q; q++) *q = (char)toupper((unsigned char)*q);
    return wow_str_owned(p);
}
static WowValue wow_chota_likho(WowValue s) {
    char *p = wow_strdup(wow_to_str(s).as.str);
    for (char *q = p; *q; q++) *q = (char)tolower((unsigned char)*q);
    return wow_str_owned(p);
}
static WowValue wow_saaf(WowValue s) {
    const char *p = wow_to_str(s).as.str;
    while (*p == ' ' || *p == '\t' || *p == '\n' || *p == '\r') p++;
    const char *end = p + strlen(p);
    while (end > p && (end[-1] == ' ' || end[-1] == '\t' || end[-1] == '\n' || end[-1] == '\r')) end--;
    size_t n = (size_t)(end - p);
    char *buf = (char *)malloc(n + 1);
    memcpy(buf, p, n); buf[n] = '\0';
    return wow_str_owned(buf);
}
/* split text on a separator into a list of strings */
static WowValue wow_toro(WowValue text, WowValue sep) {
    const char *s   = wow_to_str(text).as.str;
    const char *sp  = wow_to_str(sep).as.str;
    WowValue out = wow_list_new();
    size_t spl = strlen(sp);
    if (spl == 0) { wow_list_push(out, wow_str(s)); return out; }
    const char *hit;
    while ((hit = strstr(s, sp)) != NULL) {
        size_t n = (size_t)(hit - s);
        char *piece = (char *)malloc(n + 1);
        memcpy(piece, s, n); piece[n] = '\0';
        wow_list_push(out, wow_str_owned(piece));
        s = hit + spl;
    }
    wow_list_push(out, wow_str(s));
    return out;
}
/* join a list into a string with a separator */
static WowValue wow_milao(WowValue list, WowValue sep) {
    if (list.type != WOW_LIST) return wow_to_str(list);
    WowList *l = list.as.list;
    const char *sp = wow_to_str(sep).as.str;
    WowValue acc = wow_str("");
    for (int i = 0; i < l->len; i++) {
        if (i > 0) acc = wow_add(acc, wow_str(sp));
        acc = wow_add(acc, l->items[i]);
    }
    return acc;
}
/* replace every occurrence of `old` with `naya` */
static WowValue wow_tabdeel(WowValue text, WowValue old, WowValue naya) {
    const char *s = wow_to_str(text).as.str;
    WowValue parts = wow_toro(wow_str(s), old);
    return wow_milao(parts, naya);
}

/* ================================================================
 * auzaar — collections (return new lists; never mutate the input)
 * ================================================================ */

static WowValue wow_ginti(WowValue list)  { return wow_num(wow_count(list)); }
static WowValue wow_jama(WowValue list) {
    double total = 0;
    if (list.type == WOW_LIST)
        for (int i = 0; i < list.as.list->len; i++) total += wow_as_num(list.as.list->items[i]);
    return wow_num(total);
}
static WowValue wow_max(WowValue list) {
    if (list.type != WOW_LIST || list.as.list->len == 0) return wow_null();
    WowValue best = list.as.list->items[0];
    for (int i = 1; i < list.as.list->len; i++)
        if (wow_as_num(list.as.list->items[i]) > wow_as_num(best)) best = list.as.list->items[i];
    return best;
}
static WowValue wow_min(WowValue list) {
    if (list.type != WOW_LIST || list.as.list->len == 0) return wow_null();
    WowValue best = list.as.list->items[0];
    for (int i = 1; i < list.as.list->len; i++)
        if (wow_as_num(list.as.list->items[i]) < wow_as_num(best)) best = list.as.list->items[i];
    return best;
}
static WowValue wow_pehla(WowValue list)  { return wow_at(list, 0); }
static WowValue wow_aakhri(WowValue list) { return wow_at(list, wow_count(list) - 1); }
static WowValue wow_ulta(WowValue list) {
    WowValue out = wow_list_new();
    if (list.type == WOW_LIST)
        for (int i = list.as.list->len - 1; i >= 0; i--) wow_list_push(out, list.as.list->items[i]);
    return out;
}
static WowValue wow_shamil(WowValue list, WowValue item) {
    if (list.type == WOW_LIST)
        for (int i = 0; i < list.as.list->len; i++)
            if (wow_equal(list.as.list->items[i], item)) return wow_bool(1);
    return wow_bool(0);
}
static WowValue wow_alag(WowValue list) {
    WowValue out = wow_list_new();
    if (list.type == WOW_LIST)
        for (int i = 0; i < list.as.list->len; i++)
            if (!wow_truthy(wow_shamil(out, list.as.list->items[i])))
                wow_list_push(out, list.as.list->items[i]);
    return out;
}
static WowValue wow_silsila(WowValue start, WowValue end) {
    WowValue out = wow_list_new();
    long a = (long)wow_as_num(start), b = (long)wow_as_num(end);
    for (long i = a; i < b; i++) wow_list_push(out, wow_num((double)i));
    return out;
}
static WowValue wow_tukre(WowValue list, WowValue size) {
    WowValue out = wow_list_new();
    int n = (int)wow_as_num(size);
    if (n < 1 || list.type != WOW_LIST) return out;
    WowList *l = list.as.list;
    for (int i = 0; i < l->len; i += n) {
        WowValue chunk = wow_list_new();
        for (int j = i; j < i + n && j < l->len; j++) wow_list_push(chunk, l->items[j]);
        wow_list_push(out, chunk);
    }
    return out;
}
static void wow_flatten_into(WowValue out, WowValue list) {
    if (list.type != WOW_LIST) { wow_list_push(out, list); return; }
    for (int i = 0; i < list.as.list->len; i++) {
        WowValue it = list.as.list->items[i];
        if (it.type == WOW_LIST) wow_flatten_into(out, it);
        else wow_list_push(out, it);
    }
}
static WowValue wow_flatten(WowValue list) {
    WowValue out = wow_list_new();
    wow_flatten_into(out, list);
    return out;
}

static int wow_cmp_num(const void *a, const void *b) {
    double x = wow_as_num(*(const WowValue *)a);
    double y = wow_as_num(*(const WowValue *)b);
    return (x > y) - (x < y);
}
/* tarteeb — sort ascending into a new list */
static WowValue wow_tarteeb(WowValue list) {
    WowValue out = wow_list_new();
    if (list.type == WOW_LIST) {
        for (int i = 0; i < list.as.list->len; i++) wow_list_push(out, list.as.list->items[i]);
        qsort(out.as.list->items, out.as.list->len, sizeof(WowValue), wow_cmp_num);
    }
    return out;
}

/* Higher-order tools. The wow compiler lifts the kid's `x`-expression into a
 * top-level function and passes it here as a function pointer. */
static WowValue wow_badlo(WowValue list, WowValue (*fn)(WowValue)) {
    WowValue out = wow_list_new();
    if (list.type == WOW_LIST)
        for (int i = 0; i < list.as.list->len; i++) wow_list_push(out, fn(list.as.list->items[i]));
    return out;
}
static WowValue wow_chuno(WowValue list, WowValue (*fn)(WowValue)) {
    WowValue out = wow_list_new();
    if (list.type == WOW_LIST)
        for (int i = 0; i < list.as.list->len; i++)
            if (wow_truthy(fn(list.as.list->items[i]))) wow_list_push(out, list.as.list->items[i]);
    return out;
}
static WowValue wow_dhundo(WowValue list, WowValue (*fn)(WowValue)) {
    if (list.type == WOW_LIST)
        for (int i = 0; i < list.as.list->len; i++)
            if (wow_truthy(fn(list.as.list->items[i]))) return list.as.list->items[i];
    return wow_null();
}

/* joro — reduce. The kid's expression uses two implicit names, `acc` and `x`,
 * so the compiler lifts it into a two-argument function. */
static WowValue wow_joro(WowValue list, WowValue (*fn)(WowValue, WowValue), WowValue start) {
    WowValue acc = start;
    if (list.type == WOW_LIST)
        for (int i = 0; i < list.as.list->len; i++)
            acc = fn(acc, list.as.list->items[i]);
    return acc;
}

/* phento — shuffle into a new list (Fisher-Yates) */
static WowValue wow_phento(WowValue list) {
    WowValue out = wow_list_new();
    if (list.type == WOW_LIST) {
        for (int i = 0; i < list.as.list->len; i++) wow_list_push(out, list.as.list->items[i]);
        for (int i = out.as.list->len - 1; i > 0; i--) {
            int j = rand() % (i + 1);
            WowValue t = out.as.list->items[i];
            out.as.list->items[i] = out.as.list->items[j];
            out.as.list->items[j] = t;
        }
    }
    return out;
}

/* guroh — group by a key. There is no map type in the C runtime, so the result
 * is a list of [key, [members...]] pairs, in first-seen key order. The same
 * shape is produced on the node target so output matches across platforms. */
static WowValue wow_guroh(WowValue list, WowValue (*fn)(WowValue)) {
    WowValue out = wow_list_new();
    if (list.type != WOW_LIST) return out;
    for (int i = 0; i < list.as.list->len; i++) {
        WowValue item = list.as.list->items[i];
        WowValue key = fn(item);
        WowValue members = wow_null();
        for (int j = 0; j < out.as.list->len; j++) {
            WowValue pair = out.as.list->items[j];
            if (wow_equal(wow_at(pair, 0), key)) { members = wow_at(pair, 1); break; }
        }
        if (members.type == WOW_LIST) {
            wow_list_push(members, item);
        } else {
            WowValue group = wow_list_new();
            wow_list_push(group, item);
            WowValue pair = wow_list_new();
            wow_list_push(pair, key);
            wow_list_push(pair, group);
            wow_list_push(out, pair);
        }
    }
    return out;
}

/* ================================================================
 * auzaar — objects
 * ================================================================ */

/* mafta — list of keys */
static WowValue wow_mafta(WowValue obj) {
    WowValue out = wow_list_new();
    if (obj.type == WOW_OBJ)
        for (int i = 0; i < obj.as.obj->len; i++) wow_list_push(out, wow_str(obj.as.obj->entries[i].key));
    return out;
}

/* qeemtain — list of values */
static WowValue wow_qeemtain(WowValue obj) {
    WowValue out = wow_list_new();
    if (obj.type == WOW_OBJ)
        for (int i = 0; i < obj.as.obj->len; i++) wow_list_push(out, obj.as.obj->entries[i].val);
    return out;
}

/* key_hai — bool: does key exist? */
static WowValue wow_key_hai(WowValue obj, WowValue key) {
    if (obj.type != WOW_OBJ) return wow_bool(0);
    const char *k = wow_to_str(key).as.str;
    for (int i = 0; i < obj.as.obj->len; i++)
        if (strcmp(obj.as.obj->entries[i].key, k) == 0) return wow_bool(1);
    return wow_bool(0);
}

/* hata — new object without the given key */
static WowValue wow_hata(WowValue obj, WowValue key) {
    WowValue out = wow_obj_new();
    if (obj.type != WOW_OBJ) return out;
    const char *k = wow_to_str(key).as.str;
    for (int i = 0; i < obj.as.obj->len; i++)
        if (strcmp(obj.as.obj->entries[i].key, k) != 0)
            wow_obj_set(out, obj.as.obj->entries[i].key, obj.as.obj->entries[i].val);
    return out;
}

#endif /* AUZAAR_H */
