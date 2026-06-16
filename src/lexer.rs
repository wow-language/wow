use logos::Logos;

/// Every token the wow lexer can produce.
/// logos reads the #[token] and #[regex] annotations and builds
/// a fast DFA from them. We never write a hand-rolled match loop.
#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\r]+")] // skip spaces and tabs (not newlines)
#[logos(skip r"#[^\n]*")]  // skip # comments to end of line
pub enum Token {
    // ----------------------------------------------------------------
    // Keywords — core
    // ----------------------------------------------------------------
    #[token("bol")]     Bol,
    #[token("rakho")]   Rakho,
    #[token("agar")]    Agar,
    #[token("warna")]   Warna,
    #[token("har")]     Har,
    #[token("mein")]    Mein,
    #[token("se")]      Se,
    #[token("tak")]     Tak,
    #[token("baar")]    Baar,
    #[token("jabtak")]  Jabtak,
    #[token("roko")]    Roko,
    #[token("aage")]    Aage,
    #[token("banao")]   Banao,
    #[token("bhejo")]    Bhejo,
    #[token("sahi")]    Sahi,
    #[token("ghalat")]  Ghalat,
    #[token("khali")]   Khali,
    #[token("aur")]     Aur,
    #[token("ya")]      Ya,
    #[token("nahi")]    Nahi,
    #[token("lao")]     Lao,
    #[token("phir")]    Phir,
    #[token("koshish")] Koshish,
    #[token("pakro")]   Pakro,
    #[token("pucho")]   Pucho,

    // ----------------------------------------------------------------
    // Keywords — Arduino target
    // ----------------------------------------------------------------
    #[token("shuru")]       Shuru,
    #[token("chalao")]      Chalao,
    #[token("pin_set")]     PinSet,
    #[token("pin_likho")]   PinLikho,
    #[token("pin_parho")]   PinParho,
    #[token("intezar")]     Intezar,

    // ----------------------------------------------------------------
    // Keywords — Node target
    // ----------------------------------------------------------------
    #[token("server")]  Server,
    #[token("rasta")]   Rasta,
    #[token("jawab")]   Jawab,
    #[token("GET")]     Get,
    #[token("POST")]    Post,
    #[token("PUT")]     Put,
    #[token("DELETE")]  Delete,

    // ----------------------------------------------------------------
    // Literals
    // ----------------------------------------------------------------

    // No leading minus here: "5-3" must lex as 5, -, 3 — not 5, -3.
    // Unary negation is handled in the parser instead.
    #[regex(r"[0-9]+(\.[0-9]+)?", |lex| lex.slice().parse::<f64>().ok())]
    Number(f64),

    /// Quoted string: "hello {naam}"
    #[regex(r#""([^"\\]|\\.)*""#, |lex| {
        let s = lex.slice();
        // Strip the surrounding quotes
        Some(s[1..s.len()-1].to_string())
    })]
    Str(String),

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Ident(String),

    // ----------------------------------------------------------------
    // Symbols
    // ----------------------------------------------------------------
    #[token("{")]   LBrace,
    #[token("}")]   RBrace,
    #[token("(")]   LParen,
    #[token(")")]   RParen,
    #[token("[")]   LBracket,
    #[token("]")]   RBracket,
    #[token(",")]   Comma,
    #[token(".")]   Dot,
    #[token("?.")]  SafeDot,
    #[token("=")]   Assign,
    #[token("==")]  Eq,
    #[token("!=")]  NotEq,
    #[token("<")]   Lt,
    #[token("<=")]  Lte,
    #[token(">")]   Gt,
    #[token(">=")]  Gte,
    #[token("+")]   Plus,
    #[token("-")]   Minus,
    #[token("*")]   Star,
    #[token("/")]   Slash,
    #[token("%")]   Percent,
    #[token("=>")]  Arrow,
    #[token("?=")]  NullAssign,

    /// Newlines are meaningful: they end statements
    #[token("\n")]  Newline,
}

/// A token with its source span attached
#[derive(Debug, Clone)]
pub struct Spanned {
    pub token: Token,
    pub span: std::ops::Range<usize>,
}

/// Lex a wow source string into a flat list of spanned tokens.
/// Any lex errors are collected and reported via ariadne before exiting.
pub fn lex(source: &str, filename: &str) -> Vec<Spanned> {
    use ariadne::{Color, Label, Report, ReportKind, Source};

    let mut tokens = Vec::new();
    let mut had_error = false;

    for (result, span) in Token::lexer(source).spanned() {
        match result {
            Ok(token) => tokens.push(Spanned { token, span }),
            Err(_) => {
                had_error = true;
                let bad_char = &source[span.clone()];
                Report::build(ReportKind::Error, filename, span.start)
                    .with_message("Pehchana nahi gaya character")
                    .with_label(
                        Label::new((filename, span))
                            .with_message(format!("'{bad_char}' wow mein nahi hai"))
                            .with_color(Color::Red),
                    )
                    .finish()
                    .eprint((filename, Source::from(source)))
                    .ok();
            }
        }
    }

    if had_error {
        std::process::exit(1);
    }

    tokens
}
