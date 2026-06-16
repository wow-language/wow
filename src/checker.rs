use crate::ast::{InterpolPart, Node, Span, Spanned};
use crate::Target;
use ariadne::{Color, Label, Report, ReportKind, Source};

/// Walk the AST and verify that every construct is legal for the chosen target,
/// reporting clear Roman Urdu errors (via ariadne) that point at the exact spot.
///
/// A keyword that only makes sense on one platform — `pin_likho` (Arduino),
/// `server` (Node) — is a friendly compile error on the others, instead of
/// producing broken output. Features not yet implemented on a target are
/// reported the same way.
pub fn check(ast: &Spanned<Node>, target: &Target, source: &str, filename: &str) {
    let mut errors: Vec<Violation> = Vec::new();
    walk(ast, target, &mut errors);
    if *target == Target::Arduino {
        check_arduino_toplevel(ast, &mut errors);
    }

    if errors.is_empty() {
        return;
    }

    for v in &errors {
        let mut builder = Report::build(ReportKind::Error, filename, v.span.start)
            .with_message(&v.message)
            .with_label(
                Label::new((filename, v.span.clone()))
                    .with_message(&v.label)
                    .with_color(Color::Red),
            );
        if let Some(help) = &v.help {
            builder = builder.with_help(help);
        }
        builder.finish().eprint((filename, Source::from(source))).ok();
    }

    std::process::exit(1);
}

struct Violation {
    span: Span,
    message: String,
    label: String,
    help: Option<String>,
}

fn flag(errors: &mut Vec<Violation>, span: &Span, message: &str, label: &str, help: &str) {
    errors.push(Violation {
        span: span.clone(),
        message: message.to_string(),
        label: label.to_string(),
        help: Some(help.to_string()),
    });
}

fn walk(node: &Spanned<Node>, target: &Target, errors: &mut Vec<Violation>) {
    check_node(node, target, errors);

    // Recurse into every child so nested misuse is caught too.
    match &node.node {
        Node::Program(stmts) => walk_all(stmts, target, errors),

        Node::Assign { value, .. } => walk(value, target, errors),
        Node::Bol(e)
        | Node::Bhejo(e)
        | Node::Jawab(e)
        | Node::Pucho(e)
        | Node::Negate(e)
        | Node::UnaryNot(e)
        | Node::Server(e) => walk(e, target, errors),

        Node::Agar { condition, then_body, else_ifs, else_body } => {
            walk(condition, target, errors);
            walk_all(then_body, target, errors);
            for (cond, body) in else_ifs {
                walk(cond, target, errors);
                walk_all(body, target, errors);
            }
            if let Some(body) = else_body {
                walk_all(body, target, errors);
            }
        }
        Node::HarRange { from, to, body, .. } => {
            walk(from, target, errors);
            walk(to, target, errors);
            walk_all(body, target, errors);
        }
        Node::HarList { list, body, .. } => {
            walk(list, target, errors);
            walk_all(body, target, errors);
        }
        Node::Baar { times, body } => {
            walk(times, target, errors);
            walk_all(body, target, errors);
        }
        Node::Jabtak { condition, body } => {
            walk(condition, target, errors);
            walk_all(body, target, errors);
        }
        Node::Banao { params, body, .. } => {
            for p in params {
                if let Some(def) = &p.default {
                    walk(def, target, errors);
                }
            }
            walk_all(body, target, errors);
        }
        Node::Koshish { body, catch_body, .. } => {
            walk_all(body, target, errors);
            walk_all(catch_body, target, errors);
        }
        Node::Call { args, .. } => walk_all(args, target, errors),
        Node::BinOp { left, right, .. } => {
            walk(left, target, errors);
            walk(right, target, errors);
        }
        Node::Phir { value, calls } => {
            walk(value, target, errors);
            for step in calls {
                walk_all(&step.args, target, errors);
            }
        }
        Node::Ternary { condition, then_val, else_val } => {
            walk(condition, target, errors);
            walk(then_val, target, errors);
            walk(else_val, target, errors);
        }
        Node::List(items) => walk_all(items, target, errors),
        Node::Interpolated(parts) => {
            for part in parts {
                if let InterpolPart::Expr(e) = part {
                    walk(e, target, errors);
                }
            }
        }
        Node::Rasta { body, .. }
        | Node::ArduinoShuru(body)
        | Node::ArduinoChalao(body) => walk_all(body, target, errors),

        // Leaves
        _ => {}
    }
}

fn walk_all(nodes: &[Spanned<Node>], target: &Target, errors: &mut Vec<Violation>) {
    for n in nodes {
        walk(n, target, errors);
    }
}

/// The per-node legality rules for the current target.
fn check_node(node: &Spanned<Node>, target: &Target, errors: &mut Vec<Violation>) {
    let span = &node.span;
    match target {
        Target::C => match &node.node {
            Node::Server(_) | Node::Rasta { .. } | Node::Jawab(_) => flag(
                errors, span,
                "Ye sirf web (node) ke liye hai",
                "server / rasta / jawab desktop (C) par nahi chalte",
                "is file ko --target node ke saath chalayein",
            ),
            Node::ArduinoShuru(_) | Node::ArduinoChalao(_) => flag(
                errors, span,
                "Ye sirf Arduino ke liye hai",
                "banao shuru() / chalao() sirf Arduino par chalte hain",
                "is file ko --target arduino ke saath chalayein",
            ),
            Node::Call { name, .. } if is_arduino_call(name) => flag(
                errors, span,
                "Ye sirf Arduino ke liye hai",
                "pin wale kaam sirf Arduino par chalte hain",
                "is file ko --target arduino ke saath chalayein",
            ),
            Node::Lao { .. } => flag(
                errors, span,
                "'lao' abhi sirf Node par hai",
                "library import abhi desktop (C) par nahi",
                "abhi --target node istemaal karein",
            ),
            Node::SafeAccess { .. } => flag(
                errors, span,
                "'?.' abhi C par nahi",
                "safe access abhi desktop (C) par nahi aaya",
                "agle phase mein aayega",
            ),
            _ => {}
        },

        Target::Arduino => match &node.node {
            Node::Server(_) | Node::Rasta { .. } | Node::Jawab(_) => flag(
                errors, span,
                "Ye sirf web (node) ke liye hai",
                "server / rasta / jawab Arduino par nahi chalte",
                "is file ko --target node ke saath chalayein",
            ),
            Node::Lao { .. } => flag(
                errors, span,
                "'lao' Arduino par nahi",
                "library import Arduino par nahi",
                "Arduino ke liye built-in pin wale kaam istemaal karein",
            ),
            // The board's couple of KB of RAM can't hold lists / collections.
            Node::List(_) => flag(
                errors, span,
                "Arduino par list nahi",
                "board ki memory itni kam hai ke list nahi rakh sakte",
                "list wala kaam C ya node target par karein",
            ),
            Node::HarList { .. } => flag(
                errors, span,
                "Arduino par list loop nahi",
                "'har ... mein ...' ke liye list chahiye, jo Arduino par nahi",
                "'har i 0 se 10 tak' wala loop istemaal karein",
            ),
            Node::Call { name, .. } if is_collection_tool(name) => flag(
                errors, span,
                "Arduino par ye auzaar nahi",
                "collection/string auzaar board ki memory mein nahi aate",
                "Arduino par sirf math wale auzaar (round, power, ...) chalte hain",
            ),
            Node::Pucho(_) => flag(
                errors, span,
                "Arduino par 'pucho' nahi",
                "board par keyboard input nahi hota",
                "sensor parhne ke liye pin_parho istemaal karein",
            ),
            Node::Koshish { .. } => flag(
                errors, span,
                "Arduino par 'koshish/pakro' nahi",
                "error handling abhi Arduino par nahi",
                "agle phase mein aayega",
            ),
            Node::SafeAccess { .. } => flag(
                errors, span,
                "Arduino par '?.' nahi",
                "safe access abhi Arduino par nahi",
                "agle phase mein aayega",
            ),
            _ => {}
        },

        // On Node, `banao shuru()` is allowed — it's just startup code. But
        // `chalao()` (an Arduino loop) has no meaning here.
        Target::Node => match &node.node {
            Node::ArduinoChalao(_) => flag(
                errors, span,
                "Node par 'chalao' nahi",
                "chalao() (loop) sirf Arduino par chalta hai",
                "Node par startup ke liye 'banao shuru()' istemaal karein",
            ),
            Node::Call { name, .. } if is_arduino_call(name) => flag(
                errors, span,
                "Ye sirf Arduino ke liye hai",
                "pin wale kaam sirf Arduino par chalte hain",
                "is file ko --target arduino ke saath chalayein",
            ),
            _ => {}
        },
    }
}

fn is_arduino_call(name: &str) -> bool {
    matches!(name, "pin_set" | "pin_likho" | "pin_parho")
}

/// auzaar tools too heavy for a microcontroller (everything except the math
/// helpers, which the Arduino runtime does provide).
fn is_collection_tool(name: &str) -> bool {
    matches!(
        name,
        "badlo" | "chuno" | "joro" | "dhundo" | "shamil" | "ginti" | "jama"
            | "max" | "min" | "tarteeb" | "ulta" | "alag" | "flatten" | "tukre"
            | "pehla" | "aakhri" | "phento" | "guroh" | "silsila"
            | "toro" | "milao" | "saaf" | "tabdeel" | "lambai"
            | "bara_likho" | "chota_likho"
    )
}

/// On Arduino, executable code must live inside `banao shuru()` / `banao chalao()`
/// (or a helper `banao`). Only variable declarations and function definitions are
/// allowed at the top level, mirroring how an .ino sketch is structured.
fn check_arduino_toplevel(ast: &Spanned<Node>, errors: &mut Vec<Violation>) {
    let Node::Program(stmts) = &ast.node else { return };
    for stmt in stmts {
        match &stmt.node {
            Node::Assign { .. }
            | Node::Banao { .. }
            | Node::ArduinoShuru(_)
            | Node::ArduinoChalao(_) => {}
            _ => flag(
                errors,
                &stmt.span,
                "Arduino par code yahan nahi chal sakta",
                "ye line kisi kaam ke bahar hai",
                "isay 'banao shuru() { ... }' ya 'banao chalao() { ... }' ke andar likhein",
            ),
        }
    }
}
