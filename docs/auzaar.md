# auzaar — the wow built-in toolbox

`auzaar` is wow's standard utility library, inspired by lodash. It is auto-loaded
in every wow program. You never write `lao auzaar`. These functions are simply
always available.

## Naming rule

A function name should plainly say what it does in Urdu. We do not pick a word
for a clever sound or a pun. When there is no clear everyday Urdu word, we use
the plain English name, because most kids already learn these terms in math class.

## Target support

| Target | Support level |
|---|---|
| Node | Full — the complete set |
| C | Full — backed by a bundled C runtime; collection tools use a dynamic-array type |
| Arduino | Light — math helpers only; lists and collection tools are too heavy for board memory |

Using an unavailable function on a target produces a clear Roman Urdu compile error.

---

## Collections

All collection tools take the list as their first argument so they work cleanly with `phir`.

| Function | Does what |
|---|---|
| `badlo(list, fn)` | transform every item (map) |
| `chuno(list, fn)` | keep items that pass a test (filter) |
| `joro(list, fn, start)` | combine all items into one value (reduce) |
| `dhundo(list, fn)` | first item that matches a test (find) |
| `shamil(list, item)` | is the item present (includes) |
| `ginti(list)` | count of items |
| `jama(list)` | sum of all numbers |
| `max(list)` | largest value |
| `min(list)` | smallest value |
| `tarteeb(list)` | sort in ascending order |
| `tarteeb(list, fn)` | sort by a custom function |
| `ulta(list)` | reverse order |
| `alag(list)` | remove duplicate values |
| `flatten(list)` | flatten nested lists |
| `tukre(list, n)` | split into chunks of size n |
| `pehla(list)` | first item |
| `aakhri(list)` | last item |
| `phento(list)` | shuffle into random order |
| `guroh(list, fn)` | group items by a key |
| `silsila(start, end)` | generate a list of numbers from start to end |

`joro` and `guroh` use implicit names inside their expression: `joro` reads `acc`
(the running result) and `x` (the current item), e.g. `numbers phir joro(acc + x, 0)`;
the others use just `x`, e.g. `numbers phir chuno(x > 5)`.

`guroh` returns a list of `[key, [members...]]` pairs (in first-seen key order),
the same shape on every target — the C runtime has no map type, so this keeps
output identical across platforms. Read a pair with `pehla` (key) and `aakhri`
(members).

## Strings

| Function | Does what |
|---|---|
| `toro(text, sep)` | split a string into a list |
| `milao(list, sep)` | join a list of strings into one string |
| `saaf(text)` | trim whitespace from both ends |
| `tabdeel(text, old, new)` | replace all occurrences |
| `lambai(text)` | length of a string |
| `bara_likho(text)` | convert to uppercase |
| `chota_likho(text)` | convert to lowercase |

## Objects

These functions work with object (structured data) values. They are available on C and Node
targets. On Arduino, objects are not supported (board memory is too tight).

| Function | Does what | C | Node | Arduino |
|---|---|---|---|---|
| `mafta(obj)` | list of all keys | ✓ | ✓ | ✗ |
| `qeemtain(obj)` | list of all values | ✓ | ✓ | ✗ |
| `key_hai(obj, key)` | `sahi` if key exists in object | ✓ | ✓ | ✗ |
| `hata(obj, key)` | new object without the given key | ✓ | ✓ | ✗ |

```
shaks = { naam: "Ahmad", umar: 14, shahar: "Karachi" }

bol mafta(shaks)           # → ["naam", "umar", "shahar"]
bol qeemtain(shaks)        # → ["Ahmad", 14, "Karachi"]
bol key_hai(shaks, "naam") # → sahi
bol key_hai(shaks, "adres")# → ghalat

naya = hata(shaks, "umar")
bol mafta(naya)            # → ["naam", "shahar"]
```

## Math

Plain English names are used here because kids already know these from math class.

| Function | Does what |
|---|---|
| `random()` | random decimal number between 0 and 1 |
| `random_number(min, max)` | random whole number between min and max |
| `round(n)` | round to the nearest whole number |
| `round_up(n)` | round up to next whole number (ceiling) |
| `round_down(n)` | round down to previous whole number (floor) |
| `square_root(n)` | square root of n |
| `power(n, p)` | n raised to the power of p |
| `absolute(n)` | absolute value (remove the negative sign) |
