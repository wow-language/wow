# wow keywords

This is the single source of truth for keyword spellings in the wow language.
One official spelling per keyword. The lexer accepts exactly these strings.
Do not change a spelling once the language is in use — it breaks existing code.

## Core keywords

| Keyword | Meaning | Notes |
|---|---|---|
| `bol` | say / print | 3 letters, used constantly, kept short on purpose |
| `rakho` | put / keep | optional variable declaration; plain `x = 5` also works |
| `agar` | if | |
| `warna` | otherwise / else | pairs with agar |
| `warna agar` | else if | two words, treated as one construct |
| `har` | each | loop keyword |
| `mein` | in | `har item mein list` |
| `se` | from | `har i 0 se 10 tak` |
| `tak` | to / until | `har i 0 se 10 tak` |
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

- **Ranges are inclusive.** `har i 1 se 5 tak { }` runs the body for i = 1, 2, 3, 4, 5.
  This reads the way a beginner says it out loud ("ek se panch tak").
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

## Shared but different

These keywords exist on all targets but produce different output.

| Keyword | C | Arduino | Node |
|---|---|---|---|
| `bol` | `printf` | `Serial.println` | `console.log` |
| `intezar` | `sleep` (POSIX) | `delay` | `setTimeout` |
