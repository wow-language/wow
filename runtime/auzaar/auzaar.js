// auzaar.js — the wow standard toolbox, Node.js edition
// Auto-required by every .js file the wow compiler emits.
// Full implementation: the Node target gets the complete toolbox.

'use strict';

// ----------------------------------------------------------------
// Runtime core — formatting, output, input, truthiness.
// These mirror the C runtime so a program prints the same on every target.
// ----------------------------------------------------------------

// wow-style string form: 5 not 5.0, sahi/ghalat, khali, [a, b, c]
const fmt = (v) => {
    if (v === null || v === undefined) return 'khali';
    if (typeof v === 'boolean') return v ? 'sahi' : 'ghalat';
    if (Array.isArray(v)) return '[' + v.map(fmt).join(', ') + ']';
    return String(v);
};

// bol — print a value
const bol = (v) => { console.log(fmt(v)); };

// pucho — print a prompt and read one line from stdin (synchronously)
const pucho = (prompt) => {
    process.stdout.write(fmt(prompt));
    const fs = require('fs');
    let s = '';
    const buf = Buffer.alloc(1);
    for (;;) {
        let n;
        try {
            n = fs.readSync(0, buf, 0, 1, null);
        } catch (e) {
            if (e.code === 'EAGAIN') continue;
            break;
        }
        if (n === 0) break;
        const c = buf.toString('utf8');
        if (c === '\n') break;
        if (c === '\r') continue;
        s += c;
    }
    return s;
};

// wow truthiness: khali and empty things are false
const truthy = (v) => {
    if (v === null || v === undefined) return false;
    if (typeof v === 'boolean') return v;
    if (typeof v === 'number') return v !== 0;
    if (typeof v === 'string') return v.length > 0;
    if (Array.isArray(v)) return v.length > 0;
    return true;
};

// intezar — wait (busy-free async is impossible synchronously; we block briefly)
const intezar = (ms) => {
    const end = Date.now() + Number(ms);
    while (Date.now() < end) { /* spin */ }
};

// ----------------------------------------------------------------
// Collections
// ----------------------------------------------------------------

const badlo      = (list, fn) => list.map(fn);
const chuno      = (list, fn) => list.filter(fn);
const joro       = (list, fn, start) => list.reduce(fn, start);
const dhundo     = (list, fn) => list.find(fn) ?? null;
const shamil     = (list, item) => list.includes(item);
const ginti      = (list) => list.length;
const jama       = (list) => list.reduce((a, b) => a + b, 0);
const max        = (list) => Math.max(...list);
const min        = (list) => Math.min(...list);
const tarteeb    = (list, fn) =>
    fn ? [...list].sort(fn) : [...list].sort((a, b) => (a > b ? 1 : a < b ? -1 : 0));
const ulta       = (list) => [...list].reverse();
const alag       = (list) => [...new Set(list)];
const flatten    = (list) => list.flat(Infinity);
const tukre      = (list, n) => {
    const chunks = [];
    for (let i = 0; i < list.length; i += n) chunks.push(list.slice(i, i + n));
    return chunks;
};
const pehla      = (list) => list[0] ?? null;
const aakhri     = (list) => list[list.length - 1] ?? null;
const phento     = (list) => {
    const a = [...list];
    for (let i = a.length - 1; i > 0; i--) {
        const j = Math.floor(Math.random() * (i + 1));
        [a[i], a[j]] = [a[j], a[i]];
    }
    return a;
};
// guroh — group by a key. Returns a list of [key, [members...]] pairs in
// first-seen key order, matching the C runtime (which has no map type).
const guroh      = (list, fn) => {
    const out = [];
    for (const item of list) {
        const key = fn(item);
        let pair = out.find((p) => p[0] === key);
        if (!pair) { pair = [key, []]; out.push(pair); }
        pair[1].push(item);
    }
    return out;
};
const silsila    = (start, end) => {
    const out = [];
    for (let i = start; i < end; i++) out.push(i);
    return out;
};

// ----------------------------------------------------------------
// Strings
// ----------------------------------------------------------------

const toro       = (text, sep) => text.split(sep);
const milao      = (list, sep) => list.join(sep);
const saaf       = (text) => text.trim();
const tabdeel    = (text, old, naya) => text.replaceAll(old, naya);
const lambai     = (text) => text.length;
const bara_likho = (text) => text.toUpperCase();
const chota_likho= (text) => text.toLowerCase();

// ----------------------------------------------------------------
// Math
// ----------------------------------------------------------------

const random        = () => Math.random();
const random_number = (min, max) => Math.floor(Math.random() * (max - min + 1)) + min;
const round         = (n) => Math.round(n);
const round_up      = (n) => Math.ceil(n);
const round_down    = (n) => Math.floor(n);
const square_root   = (n) => Math.sqrt(n);
const power         = (n, p) => Math.pow(n, p);
const absolute      = (n) => Math.abs(n);

// ----------------------------------------------------------------
// Export
// ----------------------------------------------------------------

module.exports = {
    // runtime core
    fmt, bol, pucho, truthy, intezar,
    // collections
    badlo, chuno, joro, dhundo, shamil, ginti, jama, max, min,
    tarteeb, ulta, alag, flatten, tukre, pehla, aakhri, phento,
    guroh, silsila,
    // strings
    toro, milao, saaf, tabdeel, lambai, bara_likho, chota_likho,
    // math
    random, random_number, round, round_up, round_down,
    square_root, power, absolute,
};
