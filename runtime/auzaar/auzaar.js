// auzaar.js — the wow standard toolbox, Node.js edition
// Auto-required by every .js file the wow compiler emits.
// Full implementation: the Node target gets the complete toolbox.

'use strict';

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
const tarteeb    = (list, fn) => [...list].sort(fn);
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
const guroh      = (list, fn) => {
    return list.reduce((acc, item) => {
        const key = fn(item);
        (acc[key] = acc[key] || []).push(item);
        return acc;
    }, {});
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
    badlo, chuno, joro, dhundo, shamil, ginti, jama, max, min,
    tarteeb, ulta, alag, flatten, tukre, pehla, aakhri, phento,
    guroh, silsila,
    toro, milao, saaf, tabdeel, lambai, bara_likho, chota_likho,
    random, random_number, round, round_up, round_down,
    square_root, power, absolute,
};
