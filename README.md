# wow و

> *"Code likho. Wow bolo."*

**wow** is a programming language with Roman Urdu keywords, built to make coding accessible for kids in Pakistan. Write one `.wow` file and compile it to a desktop program, an Arduino sketch, or a Node.js web server.

---

## Installation

### Linux / macOS

```sh
curl -fsSL https://raw.githubusercontent.com/wow-language/wow/main/install.sh | sh
```

### Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/wow-language/wow/main/install.ps1 | iex
```

### With Rust installed

```sh
cargo install --git https://github.com/wow-language/wow
```

After installation, run any `.wow` file:

```sh
wow run myprog.wow
wow build myprog.wow --target node
wow --help
```

---

## What it looks like

```wow
banao salam(naam = "dost") {
    bol "Salam {naam}! Kaise ho?"
}

numbers = [1, 5, 3, 8, 2, 9]

bade = numbers phir chuno(x > 4) phir tarteeb

har n mein bade {
    bol "mila: {n}"
}

salam("Ahmad")
```

Output:
```
mila: 5
mila: 8
mila: 9
Salam Ahmad! Kaise ho?
```

---

## Targets

| Command | Output | Runs on |
|---|---|---|
| `wow build program.wow` | C program | Desktop |
| `wow build program.wow --target arduino` | `.ino` sketch | Arduino board |
| `wow build program.wow --target node` | Node.js app | Web server |

---

## Language in a glance

| wow | What it does |
|---|---|
| `bol "..."` | print |
| `agar x > 5 { }` | if |
| `warna { }` | else |
| `har i 0 se 10 tak { }` | numeric for loop |
| `har item mein lista { }` | list for loop |
| `3 baar { }` | repeat N times |
| `jabtak x < 10 { }` | while |
| `banao naam(params) { }` | define a function |
| `bhejo nateeja` | return a value |
| `pucho "Kya naam hai?"` | read user input |
| `lao express` | import a library |
| `koshish { } pakro ghalti { }` | try / catch |

### Operators

| wow | Means |
|---|---|
| `aur` | && |
| `ya` | \|\| |
| `nahi` | ! |
| `sahi` | true |
| `ghalat` | false |
| `khali` | null |

### Chaining with `phir`

```wow
nateeja = numbers
    phir chuno(x > 5)
    phir tarteeb
    phir pehla
```

---

## Built-in toolbox: `auzaar`

Auto-loaded everywhere, no import needed.

**Collections**

| Function | Does what |
|---|---|
| `badlo(list, fn)` | transform every item |
| `chuno(list, fn)` | keep items that pass a test |
| `joro(list, fn, start)` | reduce to one value |
| `dhundo(list, fn)` | first item that matches |
| `shamil(list, item)` | is item present |
| `ginti(list)` | count items |
| `jama(list)` | sum all numbers |
| `max(list)` | largest value |
| `min(list)` | smallest value |
| `tarteeb(list)` | sort |
| `ulta(list)` | reverse |
| `alag(list)` | remove duplicates |
| `flatten(list)` | flatten nested lists |
| `tukre(list, n)` | split into chunks of n |
| `pehla(list)` | first item |
| `aakhri(list)` | last item |
| `phento(list)` | shuffle |
| `guroh(list, fn)` | group by key |
| `silsila(start, end)` | generate number range |

**Strings**

| Function | Does what |
|---|---|
| `toro(text, sep)` | split into list |
| `milao(list, sep)` | join list into string |
| `saaf(text)` | trim whitespace |
| `tabdeel(text, old, new)` | replace |
| `lambai(text)` | length |
| `bara_likho(text)` | uppercase |
| `chota_likho(text)` | lowercase |

**Math**

| Function | Does what |
|---|---|
| `random()` | random 0 to 1 |
| `random_number(min, max)` | random whole number |
| `round(n)` | round |
| `round_up(n)` | ceiling |
| `round_down(n)` | floor |
| `square_root(n)` | square root |
| `power(n, p)` | n to the power p |
| `absolute(n)` | absolute value |

---

## Arduino example

```wow
banao shuru() {
    pin_set(13, output)
}

banao chalao() {
    pin_likho(13, on)
    intezar(1000)
    pin_likho(13, off)
    intezar(1000)
}
```

Flash to your board:
```bash
wow build blink.wow --target arduino
```

---

## Web server example

```wow
lao express

banao shuru() {
    server(3000)
}

rasta GET "/" {
    jawab "Salam Duniya!"
}
```

Run it:
```bash
wow build examples/server.wow --target node   # -> examples/server.js
cd examples && npm install express            # one-time, for the web target
node server.js                                # open http://localhost:3000
```

`rasta` registers a route, `jawab` sends the reply, and `server(port)` starts
listening — the Express app is wired up for you, so there's no `app = express()`
or `(req, res)` to write. `banao shuru()` holds code that runs at startup.

---

## Catching mistakes with `koshish` / `pakro`

```wow
koshish {
    natija = 10 / 0
    bol natija
} pakro ghalti {
    bol "Ghalti pakdi: {ghalti}"
}
```

Dividing by zero raises a catchable error (`sifr se taqseem nahi ho sakta`) on
both the C and Node targets. An uncaught error prints a friendly Roman Urdu line
instead of a crash dump.

---

## Error messages in Roman Urdu

wow tries to explain mistakes clearly:

```
Ghalti: 'agar' ke baad condition chahiye
  --> mera_code.wow:5:4
   |
 5 |     agar {
   |          ^ yahan condition honi chahiye
   |
   = madad: agar x > 5 { ... } likho
```

---

## Built with

- **Rust** — the compiler toolchain
- **logos** — lexer
- **chumsky** — parser
- **ariadne** — error messages
- **clap** — CLI

---

## Try it

```bash
# build the compiler
cargo build

# run a program (defaults to the C target; needs gcc)
./target/debug/wow run examples/pahara.wow

# same source, JavaScript target (needs node)
./target/debug/wow run examples/pahara.wow --target node

# the Arduino target emits a .ino sketch (flash it with arduino-cli)
./target/debug/wow build examples/blink.wow --target arduino  # -> examples/blink.ino

# just emit the generated code without running it
./target/debug/wow build examples/salam.wow            # -> examples/salam.c
./target/debug/wow build examples/salam.wow --target node  # -> examples/salam.js
```

## Project status

All three targets are working from one source language: the **core language runs
on C and Node.js with identical output**, the **Arduino target** turns a `.wow`
file into a flashable `.ino` sketch, and the **Node web keywords** turn a few
lines of Roman Urdu into a running Express server.

What works today:

- Variables, `bol`, string interpolation, math, comparisons, `aur` / `ya` / `nahi`
- `agar` / `warna agar` / `warna`, the word-ternary (`"bara" agar x > 5 warna "chota"`)
- Loops: `har i 0 se 10 tak`, `har item mein list`, `N baar`, `jabtak`, with `roko` / `aage`
- `banao` / `bhejo` functions with default parameters and recursion
- Lists, `pucho` (input), and the **full `auzaar` toolbox** on C and Node
  (incl. `joro`/reduce, `guroh`/groupBy, `phento`/shuffle)
- `phir` pipelines, including higher-order tools (`numbers phir chuno(x > 4) phir tarteeb`)
- `koshish` / `pakro` error handling on C and Node (e.g. catching divide-by-zero)
- **Arduino**: `banao shuru()` / `banao chalao()`, `pin_set` / `pin_likho` / `pin_parho`,
  `intezar`, and the math `auzaar` helpers — the memory-heavy parts (lists,
  collection tools, `pucho`) give a friendly "Arduino par nahi" error
- **Web (Node)**: `lao` imports, `rasta` routes, `jawab` replies, and `server(port)`
  — a real Express server with no boilerplate
- Misspell a keyword and the compiler suggests the fix (`agr` → "kya aap ka matlab 'agar' tha?")
- Clear, pointed compile errors in Roman Urdu

**Objects** — structured data with dot access and Urdu possessive safe access:

```wow
shaks = { naam: "Ahmad", umar: 14, shahar: "Karachi" }

bol shaks.naam            # Ahmad   — regular access (crashes if khali)
bol shaks ka umar         # 14      — safe: ka / ki / kay all work
bol shaks ki shahar       # Karachi — pick whichever sounds right for the noun
bol shaks ka email        # khali   — missing key, no crash

shaks.umar = 15           # update a property
shaks.email ?= "default"  # only assign if currently khali

# Lists of objects — a very common pattern
log = [
    { naam: "Ahmad", umar: 10 },
    { naam: "Sara", umar: 12 },
]
har p mein log {
    bol "{p.naam}: {p ka umar} saal"
}
```

Object auzaar: `mafta(obj)` (keys), `qeemtain(obj)` (values), `key_hai(obj, key)`, `hata(obj, key)`.
Objects are available on C and Node targets; Arduino gives a helpful "memory kam hai" error.

A couple of design notes for the curious:

- Ranges are **inclusive**: `har i 1 se 5 tak` runs for 1, 2, 3, 4, 5.
- Inside string interpolation, a `{...}` hole can hold any expression, but not a
  string literal using the same quotes — write `umar = "{saal}"` ahead of time.
- The parser is hand-written (not a combinator library) so the Roman Urdu error
  messages stay precise; see the note in `Cargo.toml`.

See [docs/keywords.md](docs/keywords.md) for the full keyword reference and [docs/auzaar.md](docs/auzaar.md) for the toolbox reference.

---

## License

MIT
