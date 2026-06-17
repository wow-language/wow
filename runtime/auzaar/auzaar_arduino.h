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

// ---- ESP32 WiFi + WebServer ----
// These helpers are only compiled when building for an ESP32 board.
// arduino-cli picks the right board automatically when you set board_fqbn.
#if defined(AUZAAR_ESP32) || defined(ESP32)
#include <WiFi.h>
#include <WebServer.h>

static WebServer _wow_server(80);

inline void wow_wifi_jodo(const char* ssid, const char* pass) {
    WiFi.begin(ssid, pass);
    while (WiFi.status() != WL_CONNECTED) delay(500);
}

inline void wow_server_shuru(double /* port */) {
    _wow_server.begin();
}

inline void wow_server_parho() {
    _wow_server.handleClient();
}

inline void wow_jawab_bhejo(double code, const char* content_type, const char* body) {
    _wow_server.send((int)code, content_type, body);
}

inline void wow_jawab_bhejo(double code, const String& content_type, const String& body) {
    _wow_server.send((int)code, content_type, body);
}

inline String wow_wifi_ip() {
    return WiFi.localIP().toString();
}
#endif // AUZAAR_ESP32 / ESP32

#endif // AUZAAR_ARDUINO_H
