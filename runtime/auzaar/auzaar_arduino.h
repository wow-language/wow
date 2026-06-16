#ifndef AUZAAR_ARDUINO_H
#define AUZAAR_ARDUINO_H

// auzaar (Arduino edition) — the small subset that fits in board memory.
//
// Boards like the Uno have only a couple of KB of RAM, so the lodash-style
// collection tools are left out. The wow compiler reports a friendly Roman Urdu
// error if you use a collection tool, a list, or `pucho` on the Arduino target.
// What remains here: bol (print over Serial), the math helpers, and a small
// String builder for interpolation. `intezar` maps straight to delay().

#include <Arduino.h>

// ---- bol: print a value. Whole numbers print without a trailing ".00". ----
static inline void wow_print(double v) {
    if (v == (long)v) Serial.println((long)v);
    else              Serial.println(v);
}
static inline void wow_print(long v)          { Serial.println(v); }
static inline void wow_print(int v)           { Serial.println(v); }
static inline void wow_print(bool b)          { Serial.println(b ? "sahi" : "ghalat"); }
static inline void wow_print(const char *s)   { Serial.println(s); }
static inline void wow_print(const String &s) { Serial.println(s); }

// ---- wow_str: turn the pieces of an interpolated string into text ----
static inline String wow_str(const char *s)   { return String(s); }
static inline String wow_str(const String &s) { return s; }
static inline String wow_str(bool b)          { return String(b ? "sahi" : "ghalat"); }
static inline String wow_str(double v) {
    if (v == (long)v) return String((long)v);
    return String(v);
}

// ---- math: kids meet these names in math class ----
static inline double wow_round(double n)        { return round(n); }
static inline double wow_round_up(double n)     { return ceil(n); }
static inline double wow_round_down(double n)   { return floor(n); }
static inline double wow_square_root(double n)  { return sqrt(n); }
static inline double wow_power(double n, double p) { return pow(n, p); }
static inline double wow_absolute(double n)     { return fabs(n); }
static inline double wow_random()               { return (double)random(0, 10001) / 10000.0; }
static inline double wow_random_number(double lo, double hi) {
    return (double)random((long)lo, (long)hi + 1);
}

#endif // AUZAAR_ARDUINO_H
