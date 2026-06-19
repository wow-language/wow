# wow keywords

This is the single source of truth for keyword spellings in the wow language.
One official spelling per keyword. The lexer accepts exactly these strings.
Do not change a spelling once the language is in use — it breaks existing code.

## Core keywords

| Keyword | Meaning | Notes |
|---|---|---|
| `likho` | write / print | "write" — we are writing output to the screen, not speaking |
| `rakho` | put / keep | optional variable declaration; plain `x = 5` also works |
| `agar` | if | |
| `warna` | otherwise / else | pairs with agar |
| `warna agar` | else if | two words, treated as one construct |
| `har` | each | loop keyword — `har list mein item { }` |
| `mein` | in | `har phal mein p { }` — collection comes first |
| `se` | from | `1 se 5 tak har i { }` — range loops start with the range |
| `tak` | to / until | `1 se 5 tak har i { }` |
| `baar` | times | `3 baar { }` |
| `jabtak` | as long as / while | |
| `roko` | stop | break |
| `aage` | forward / next | continue |
| `banao` | make | define a function |
| `bhejo` | send | return a value: `bhejo nateeja` |
| `sahi` | correct | true |
| `ghalat` | wrong | false |
| `khali` | empty | null |
| `aur` | and | logical AND — replaces `&&` |
| `ya` | or | logical OR — replaces `\|\|` |
| `nahi` | not | logical NOT — replaces `!` |
| `lao` | bring | import: `lao express` |
| `phir` | then | chaining: `list phir chuno(x > 5) phir tarteeb` |
| `koshish` | try | error handling |
| `pakro` | catch | pairs with koshish |
| `pucho` | ask | read user input |

## Semantics notes

- **Ranges are inclusive.** `1 se 5 tak har i { }` runs the body for i = 1, 2, 3, 4, 5.
  The range comes first — "1 se 5 tak" — then `har i` names the counter. This mirrors how
  you say it out loud: "ek se paanch tak, har i ke liye".
- **Newlines end statements**, except a `phir` chain may continue on the next line.
- **Variables are function-scoped.** A variable first assigned inside an `agar` or a
  loop is still visible after it, like in everyday scripting languages.
- **`koshish` / `pakro`** catch runtime errors (on C and Node). Dividing by zero
  raises a catchable error; on Node, errors thrown by imported libraries are
  caught too. An uncaught error prints `Ghalti: ...` and stops the program.

## Arduino-only keywords

Using these with `--target c` or `--target node` is a compile error.

| Keyword | Meaning | Maps to |
|---|---|---|
| `shuru` | start | `setup()` |
| `chalao` | run | `loop()` |
| `pin_set` | configure a pin | `pinMode()` |
| `pin_likho` | write to a pin | `digitalWrite()` |
| `pin_parho` | read a pin | `digitalRead()` |
| `intezar` | wait | `delay()` |

## Node-only keywords

Using these with `--target c` or `--target arduino` is a compile error.

| Keyword | Meaning | Maps to |
|---|---|---|
| `server` | start web server | `app.listen()` |
| `rasta` | define a route | `app.get()` / `app.post()` etc |
| `jawab` | send a response | `res.send()` |
| `lao` | import a library | `require()` (e.g. `lao express`) |

On the Node target, `banao shuru()` is also allowed: it holds startup code that
runs once when the program begins (on Arduino the same `shuru` maps to `setup()`).
The Express app behind `server` / `rasta` / `jawab` is created for you.

## Object access keywords

These three keywords are safe property access operators (return `khali` instead of crashing
if the object or key doesn't exist). They are grammatically equivalent — pick whichever
sounds right for the noun.

| Keyword | Grammatical use | Example |
|---|---|---|
| `ka` | masculine singular | `shaks ka naam` ("person's name") |
| `ki` | feminine | `kitaab ki title` ("book's title") |
| `kay` | masculine plural / general | `bachay kay naam` ("child's name") |

All three map to `?.` in the Node backend and to `wow_safe_get()` in C.
Objects are not supported on the Arduino target (memory too tight).

### Object literal syntax

```
shaks = { naam: "Ahmad", umar: 14, shahar: "Karachi" }
```

Keys are plain identifiers. Values are any wow expression. Objects are not supported on Arduino.

### Property access

| Style | Syntax | Behaviour |
|---|---|---|
| Regular dot | `shaks.naam` | Returns value; crashes if `shaks` is `khali` |
| Safe possessive | `shaks ka naam` | Returns `khali` if `shaks` is `khali` or key missing |
| Safe dot | `shaks?.naam` | Same as `ka/ki/kay` |

### Property assignment

```
shaks.umar = 15          # regular assignment
shaks.email ?= "default" # only assigns if email is khali
```

## Shared but different

These keywords exist on all targets but produce different output.

| Keyword | C | Arduino | Node |
|---|---|---|---|
| `likho` | `printf` | `Serial.println` | `console.log` |
| `intezar` | `sleep` (POSIX) | `delay` | `setTimeout` |
