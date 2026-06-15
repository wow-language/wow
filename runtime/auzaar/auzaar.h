#ifndef AUZAAR_H
#define AUZAAR_H

#include <math.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

// ----------------------------------------------------------------
// Math — thin wrappers over the C standard library
// ----------------------------------------------------------------

static inline double wow_round(double n)       { return round(n); }
static inline double wow_round_up(double n)    { return ceil(n); }
static inline double wow_round_down(double n)  { return floor(n); }
static inline double wow_square_root(double n) { return sqrt(n); }
static inline double wow_power(double n, double p) { return pow(n, p); }
static inline double wow_absolute(double n)    { return fabs(n); }

static inline double wow_random() {
    return (double)rand() / (double)RAND_MAX;
}

static inline int wow_random_number(int min, int max) {
    return min + rand() % (max - min + 1);
}

// ----------------------------------------------------------------
// String — thin wrappers
// ----------------------------------------------------------------

static inline int wow_lambai(const char *s) { return (int)strlen(s); }

// TODO: expand with collection types in Phase 3

#endif // AUZAAR_H
