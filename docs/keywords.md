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
| `kaam` | task | define a function |
| `do` | give | return a value: `do nateeja` |
| `sahi` | correct | true |
| `ghalat` | wrong | false |
| `khali` | empty | null |
| `aur` | and | logical AND — replaces `&&` |
| `ya` | or | logical OR — replaces `\|\|` |
| `nahi` | not | logical NOT — replaces `!` |
| `lao` | bring | import: `lao express` |
| `phir` | then | chaining: `list phir chuno(x > 5) phir tarteeb` |
| `koshish` | try | error handling |
| `pakdo` | catch | pairs with koshish |
| `pucho` | ask | read user input |

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

## Shared but different

These keywords exist on all targets but produce different output.

| Keyword | C | Arduino | Node |
|---|---|---|---|
| `bol` | `printf` | `Serial.println` | `console.log` |
| `intezar` | `sleep` (POSIX) | `delay` | `setTimeout` |
