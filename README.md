# wow و

> *"Code likho. Wow bolo."*

**wow** is a programming language with Roman Urdu keywords, built to make coding accessible for kids in Pakistan. Write one `.wow` file and compile it to a desktop program, an Arduino sketch, or a Node.js web server.

---

## What it looks like

```wow
kaam salam(naam = "dost") {
    bol "Salam {naam}! Kaise ho?"
}

numbers = [1, 5, 3, 8, 2, 9]

bade = numbers phir chuno(x > 4) phir tarteeb

har n mein bade {
    bol "mila: {n}"
}

salam("Ali")
```

Output:
```
mila: 5
mila: 8
mila: 9
Salam Ali! Kaise ho?
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
| `kaam naam(params) { }` | define a function |
| `do nateeja` | return a value |
| `pucho "Kya naam hai?"` | read user input |
| `lao express` | import a library |
| `koshish { } pakdo ghalti { }` | try / catch |

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
kaam shuru() {
    pin_set(13, output)
}

kaam chalao() {
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

kaam shuru() {
    server(3000)
}

rasta GET "/" {
    jawab "Salam Duniya!"
}
```

Run it:
```bash
wow build server.wow --target node
```

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

# just emit the generated code without running it
./target/debug/wow build examples/salam.wow            # -> examples/salam.c
./target/debug/wow build examples/salam.wow --target node  # -> examples/salam.js
```

## Project status

Phase 1 is working: the **core language runs on both the C and Node.js targets
from the same source file**, with identical output.

What works today:

- Variables, `bol`, string interpolation, math, comparisons, `aur` / `ya` / `nahi`
- `agar` / `warna agar` / `warna`, the word-ternary (`"bara" agar x > 5 warna "chota"`)
- Loops: `har i 0 se 10 tak`, `har item mein list`, `N baar`, `jabtak`, with `roko` / `aage`
- `kaam` / `do` functions with default parameters and recursion
- Lists, `pucho` (input), and a good slice of the `auzaar` toolbox
- `phir` pipelines, including higher-order tools (`numbers phir chuno(x > 4) phir tarteeb`)
- Clear, pointed compile errors in Roman Urdu

Coming next, per [the plan](docs): the Arduino target (`shuru` / `chalao` / pins),
the web keywords (`server` / `rasta` / `jawab` / `lao`), and `koshish` / `pakdo`.
Using one of those today produces a friendly "not on this target yet" error.

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
