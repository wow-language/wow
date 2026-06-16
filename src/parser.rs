use crate::ast::{
    BinOp, HttpMethod, InterpolPart, Node, Param, PhirStep, Span, Spanned,
};
use crate::lexer::{Spanned as TokSpanned, Token};
use ariadne::{Color, Label, Report, ReportKind, Source};

/// Parse a flat token stream into an AST.
///
/// This is a hand-written recursive-descent parser. wow statements are ended by
/// newlines (no semicolons) and a few keywords do double duty (`agar`/`warna`
/// are both the if-statement and the word ternary), which a combinator parser
/// handles awkwardly. Writing it by hand keeps the Roman Urdu error messages —
/// the point of the language — under our control.
pub fn parse(tokens: Vec<TokSpanned>, source: &str, filename: &str) -> Spanned<Node> {
    let mut p = Parser::new(tokens, source.to_string(), filename.to_string());
    p.program()
}

struct Parser {
    tokens: Vec<TokSpanned>,
    pos: usize,
    source: String,
    filename: String,
    end: usize,
}

impl Parser {
    fn new(tokens: Vec<TokSpanned>, source: String, filename: String) -> Self {
        let end = source.len();
        Parser { tokens, pos: 0, source, filename, end }
    }

    // ----------------------------------------------------------------
    // Cursor helpers
    // ----------------------------------------------------------------

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos).map(|t| &t.token)
    }

    fn at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    fn cur_span(&self) -> Span {
        self.tokens
            .get(self.pos)
            .map(|t| t.span.clone())
            .unwrap_or(self.end..self.end)
    }

    fn prev_span(&self) -> Span {
        if self.pos == 0 {
            0..0
        } else {
            self.tokens[self.pos - 1].span.clone()
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn skip_newlines(&mut self) {
        while matches!(self.peek(), Some(Token::Newline)) {
            self.pos += 1;
        }
    }

    fn is(&self, t: &Token) -> bool {
        self.peek() == Some(t)
    }

    // ----------------------------------------------------------------
    // Errors (fail fast with a pointed Roman Urdu message)
    // ----------------------------------------------------------------

    fn err(&self, span: Span, msg: &str, label: &str, help: Option<&str>) -> ! {
        let mut builder = Report::build(ReportKind::Error, self.filename.clone(), span.start)
            .with_message(msg)
            .with_label(
                Label::new((self.filename.clone(), span))
                    .with_message(label)
                    .with_color(Color::Red),
            );
        if let Some(h) = help {
            builder = builder.with_help(h);
        }
        builder
            .finish()
            .eprint((self.filename.clone(), Source::from(&self.source)))
            .ok();
        std::process::exit(1);
    }

    fn expect_simple(&mut self, want: &Token, msg: &str, label: &str, help: Option<&str>) {
        if self.is(want) {
            self.advance();
        } else {
            self.err(self.cur_span(), msg, label, help);
        }
    }

    fn expect_lparen(&mut self) {
        self.expect_simple(&Token::LParen, "'(' chahiye", "yahan '(' honi chahiye", None);
    }
    fn expect_rparen(&mut self) {
        self.expect_simple(&Token::RParen, "')' chahiye", "yahan ')' band karni chahiye", None);
    }
    fn expect_rbracket(&mut self) {
        self.expect_simple(&Token::RBracket, "']' chahiye", "list ko ']' se band karein", None);
    }
    fn expect_assign(&mut self) {
        self.expect_simple(&Token::Assign, "'=' chahiye", "yahan '=' hona chahiye", None);
    }

    fn ident(&mut self) -> String {
        match self.peek() {
            Some(Token::Ident(s)) => {
                let s = s.clone();
                self.advance();
                s
            }
            _ => self.err(
                self.cur_span(),
                "Naam chahiye",
                "yahan ek naam (identifier) hona chahiye",
                None,
            ),
        }
    }

    fn string_lit(&mut self) -> String {
        match self.peek() {
            Some(Token::Str(s)) => {
                let s = s.clone();
                self.advance();
                s
            }
            _ => self.err(self.cur_span(), "Text chahiye", "yahan \"...\" likhein", None),
        }
    }

    // ----------------------------------------------------------------
    // Program and blocks
    // ----------------------------------------------------------------

    fn program(&mut self) -> Spanned<Node> {
        let mut stmts = Vec::new();
        self.skip_newlines();
        while !self.at_end() {
            stmts.push(self.statement());
            self.skip_newlines();
        }
        Spanned::new(Node::Program(stmts), 0..self.end)
    }

    /// `{ ... }` — a brace block. Newlines inside are insignificant.
    fn block(&mut self) -> Vec<Spanned<Node>> {
        self.skip_newlines();
        self.expect_simple(
            &Token::LBrace,
            "'{' chahiye",
            "yahan block shuru karne ke liye '{' likhein",
            Some("misaal: agar x > 5 { bol \"bara\" }"),
        );
        self.skip_newlines();
        let mut stmts = Vec::new();
        loop {
            if self.is(&Token::RBrace) {
                self.advance();
                break;
            }
            if self.at_end() {
                self.err(
                    self.cur_span(),
                    "Block band nahi hua",
                    "yahan '}' chahiye",
                    Some("har '{' ke liye ek '}' zaroori hai"),
                );
            }
            stmts.push(self.statement());
            self.skip_newlines();
        }
        stmts
    }

    // ----------------------------------------------------------------
    // Statements
    // ----------------------------------------------------------------

    fn statement(&mut self) -> Spanned<Node> {
        let start = self.cur_span().start;
        match self.peek() {
            Some(Token::Bol) => {
                self.advance();
                let e = self.expr();
                let span = start..e.span.end;
                Spanned::new(Node::Bol(Box::new(e)), span)
            }
            Some(Token::Rakho) => {
                self.advance();
                let name = self.ident();
                self.expect_assign();
                let value = self.expr();
                let span = start..value.span.end;
                Spanned::new(Node::Assign { name, value: Box::new(value) }, span)
            }
            Some(Token::Agar) => self.if_stmt(),
            Some(Token::Har) => self.har_stmt(),
            Some(Token::Jabtak) => {
                self.advance();
                let condition = self.expr();
                let body = self.block();
                let span = start..self.prev_span().end;
                Spanned::new(Node::Jabtak { condition: Box::new(condition), body }, span)
            }
            Some(Token::Kaam) => self.kaam_stmt(),
            Some(Token::Do) => {
                self.advance();
                let e = self.expr();
                let span = start..e.span.end;
                Spanned::new(Node::Do(Box::new(e)), span)
            }
            Some(Token::Roko) => {
                let span = self.cur_span();
                self.advance();
                Spanned::new(Node::Roko, span)
            }
            Some(Token::Aage) => {
                let span = self.cur_span();
                self.advance();
                Spanned::new(Node::Aage, span)
            }
            Some(Token::Koshish) => self.koshish_stmt(),
            Some(Token::Lao) => self.lao_stmt(),
            Some(Token::Rasta) => self.rasta_stmt(),
            Some(Token::Jawab) => {
                self.advance();
                let e = self.expr();
                let span = start..e.span.end;
                Spanned::new(Node::Jawab(Box::new(e)), span)
            }
            Some(Token::Server) => {
                self.advance();
                self.expect_lparen();
                let port = self.expr();
                self.expect_rparen();
                let span = start..self.prev_span().end;
                Spanned::new(Node::Server(Box::new(port)), span)
            }
            _ => self.expr_or_assign_stmt(),
        }
    }

    fn expr_or_assign_stmt(&mut self) -> Spanned<Node> {
        let start = self.cur_span().start;
        let e = self.expr();

        // `N baar { ... }` — repeat
        if self.is(&Token::Baar) {
            self.advance();
            let body = self.block();
            let span = start..self.prev_span().end;
            return Spanned::new(Node::Baar { times: Box::new(e), body }, span);
        }

        // `x = ...` — assignment to a bare identifier
        if let Node::Identifier(name) = &e.node {
            if self.is(&Token::Assign) {
                let name = name.clone();
                self.advance();
                let value = self.expr();
                let span = start..value.span.end;
                return Spanned::new(Node::Assign { name, value: Box::new(value) }, span);
            }
        }

        // Otherwise it's an expression evaluated for its effect (e.g. a call).
        e
    }

    fn if_stmt(&mut self) -> Spanned<Node> {
        let start = self.cur_span().start;
        self.advance(); // agar
        let condition = self.expr();
        let then_body = self.block();

        let mut else_ifs = Vec::new();
        let mut else_body = None;
        loop {
            self.skip_newlines();
            if !self.is(&Token::Warna) {
                break;
            }
            self.advance(); // warna
            if self.is(&Token::Agar) {
                self.advance(); // agar
                let cond = self.expr();
                let body = self.block();
                else_ifs.push((cond, body));
            } else {
                else_body = Some(self.block());
                break;
            }
        }

        let span = start..self.prev_span().end;
        Spanned::new(
            Node::Agar { condition: Box::new(condition), then_body, else_ifs, else_body },
            span,
        )
    }

    fn har_stmt(&mut self) -> Spanned<Node> {
        let start = self.cur_span().start;
        self.advance(); // har
        let var = self.ident();

        if self.is(&Token::Mein) {
            // har item mein list { ... }
            self.advance();
            let list = self.expr();
            let body = self.block();
            let span = start..self.prev_span().end;
            Spanned::new(Node::HarList { var, list: Box::new(list), body }, span)
        } else {
            // har i 0 se 10 tak { ... }
            let from = self.expr();
            self.expect_simple(&Token::Se, "'se' chahiye", "range mein 'se' likhein", Some("misaal: har i 0 se 10 tak { }"));
            let to = self.expr();
            self.expect_simple(&Token::Tak, "'tak' chahiye", "range mein 'tak' likhein", Some("misaal: har i 0 se 10 tak { }"));
            let body = self.block();
            let span = start..self.prev_span().end;
            Spanned::new(Node::HarRange { var, from: Box::new(from), to: Box::new(to), body }, span)
        }
    }

    fn kaam_stmt(&mut self) -> Spanned<Node> {
        let start = self.cur_span().start;
        self.advance(); // kaam

        // `kaam shuru()` / `kaam chalao()` are the Arduino entry points.
        if self.is(&Token::Shuru) {
            self.advance();
            self.expect_lparen();
            self.expect_rparen();
            let body = self.block();
            let span = start..self.prev_span().end;
            return Spanned::new(Node::ArduinoShuru(body), span);
        }
        if self.is(&Token::Chalao) {
            self.advance();
            self.expect_lparen();
            self.expect_rparen();
            let body = self.block();
            let span = start..self.prev_span().end;
            return Spanned::new(Node::ArduinoChalao(body), span);
        }

        let naam = self.ident();
        self.expect_lparen();
        let params = self.parse_params();
        self.expect_rparen();
        let body = self.block();
        let span = start..self.prev_span().end;
        Spanned::new(Node::Kaam { naam, params, body }, span)
    }

    fn parse_params(&mut self) -> Vec<Param> {
        let mut params = Vec::new();
        self.skip_newlines();
        if self.is(&Token::RParen) {
            return params;
        }
        loop {
            let name = self.ident();
            let default = if self.is(&Token::Assign) {
                self.advance();
                Some(Box::new(self.expr()))
            } else {
                None
            };
            params.push(Param { name, default });
            if self.is(&Token::Comma) {
                self.advance();
                self.skip_newlines();
            } else {
                break;
            }
        }
        params
    }

    fn koshish_stmt(&mut self) -> Spanned<Node> {
        let start = self.cur_span().start;
        self.advance(); // koshish
        let body = self.block();
        self.skip_newlines();
        self.expect_simple(
            &Token::Pakdo,
            "'pakdo' chahiye",
            "koshish ke baad 'pakdo' likhein",
            Some("misaal: koshish { ... } pakdo ghalti { ... }"),
        );
        let catch_var = if let Some(Token::Ident(_)) = self.peek() {
            Some(self.ident())
        } else {
            None
        };
        let catch_body = self.block();
        let span = start..self.prev_span().end;
        Spanned::new(Node::Koshish { body, catch_var, catch_body }, span)
    }

    fn lao_stmt(&mut self) -> Spanned<Node> {
        let start = self.cur_span().start;
        self.advance(); // lao
        let name = self.ident();
        // optional `from "path"`
        let path = if matches!(self.peek(), Some(Token::Ident(s)) if s == "from") {
            self.advance();
            self.string_lit()
        } else {
            name.clone()
        };
        let span = start..self.prev_span().end;
        Spanned::new(Node::Lao { name, path }, span)
    }

    fn rasta_stmt(&mut self) -> Spanned<Node> {
        let start = self.cur_span().start;
        self.advance(); // rasta
        let method = match self.peek() {
            Some(Token::Get) => HttpMethod::Get,
            Some(Token::Post) => HttpMethod::Post,
            Some(Token::Put) => HttpMethod::Put,
            Some(Token::Delete) => HttpMethod::Delete,
            _ => self.err(
                self.cur_span(),
                "HTTP method chahiye",
                "yahan GET, POST, PUT ya DELETE likhein",
                None,
            ),
        };
        self.advance();
        let path = self.string_lit();
        let body = self.block();
        let span = start..self.prev_span().end;
        Spanned::new(Node::Rasta { method, path, body }, span)
    }

    // ----------------------------------------------------------------
    // Expressions (precedence climbing)
    // ----------------------------------------------------------------

    fn expr(&mut self) -> Spanned<Node> {
        self.phir_expr()
    }

    /// `value phir f(args) phir g` — pipeline. May span multiple lines.
    fn phir_expr(&mut self) -> Spanned<Node> {
        let base = self.ternary();
        // Look past newlines: a `phir` on the next line continues the chain.
        let save = self.pos;
        self.skip_newlines();
        if !self.is(&Token::Phir) {
            self.pos = save;
            return base;
        }

        let start = base.span.start;
        let mut calls = Vec::new();
        loop {
            self.skip_newlines();
            if !self.is(&Token::Phir) {
                break;
            }
            let step_start = self.cur_span().start;
            self.advance(); // phir
            let name = self.ident();
            let mut args = Vec::new();
            if self.is(&Token::LParen) {
                self.advance();
                args = self.arg_list();
                self.expect_rparen();
            }
            let span = step_start..self.prev_span().end;
            calls.push(PhirStep { name, args, span });
        }
        let span = start..self.prev_span().end;
        Spanned::new(Node::Phir { value: Box::new(base), calls }, span)
    }

    /// `then_val agar condition warna else_val`
    fn ternary(&mut self) -> Spanned<Node> {
        let value = self.or_expr();
        if self.is(&Token::Agar) {
            self.advance();
            let condition = self.or_expr();
            self.expect_simple(&Token::Warna, "'warna' chahiye", "ternary mein 'warna' likhein", Some("misaal: \"bara\" agar x > 5 warna \"chota\""));
            let else_val = self.ternary();
            let span = value.span.start..else_val.span.end;
            return Spanned::new(
                Node::Ternary {
                    condition: Box::new(condition),
                    then_val: Box::new(value),
                    else_val: Box::new(else_val),
                },
                span,
            );
        }
        value
    }

    fn or_expr(&mut self) -> Spanned<Node> {
        let mut left = self.and_expr();
        while self.is(&Token::Ya) {
            self.advance();
            let right = self.and_expr();
            left = self.binop(BinOp::Or, left, right);
        }
        left
    }

    fn and_expr(&mut self) -> Spanned<Node> {
        let mut left = self.equality();
        while self.is(&Token::Aur) {
            self.advance();
            let right = self.equality();
            left = self.binop(BinOp::And, left, right);
        }
        left
    }

    fn equality(&mut self) -> Spanned<Node> {
        let mut left = self.comparison();
        loop {
            let op = match self.peek() {
                Some(Token::Eq) => BinOp::Eq,
                Some(Token::NotEq) => BinOp::NotEq,
                _ => break,
            };
            self.advance();
            let right = self.comparison();
            left = self.binop(op, left, right);
        }
        left
    }

    fn comparison(&mut self) -> Spanned<Node> {
        let mut left = self.term();
        loop {
            let op = match self.peek() {
                Some(Token::Lt) => BinOp::Lt,
                Some(Token::Lte) => BinOp::Lte,
                Some(Token::Gt) => BinOp::Gt,
                Some(Token::Gte) => BinOp::Gte,
                _ => break,
            };
            self.advance();
            let right = self.term();
            left = self.binop(op, left, right);
        }
        left
    }

    fn term(&mut self) -> Spanned<Node> {
        let mut left = self.factor();
        loop {
            let op = match self.peek() {
                Some(Token::Plus) => BinOp::Add,
                Some(Token::Minus) => BinOp::Sub,
                _ => break,
            };
            self.advance();
            let right = self.factor();
            left = self.binop(op, left, right);
        }
        left
    }

    fn factor(&mut self) -> Spanned<Node> {
        let mut left = self.unary();
        loop {
            let op = match self.peek() {
                Some(Token::Star) => BinOp::Mul,
                Some(Token::Slash) => BinOp::Div,
                Some(Token::Percent) => BinOp::Mod,
                _ => break,
            };
            self.advance();
            let right = self.unary();
            left = self.binop(op, left, right);
        }
        left
    }

    fn unary(&mut self) -> Spanned<Node> {
        let start = self.cur_span().start;
        if self.is(&Token::Nahi) {
            self.advance();
            let e = self.unary();
            let span = start..e.span.end;
            return Spanned::new(Node::UnaryNot(Box::new(e)), span);
        }
        if self.is(&Token::Minus) {
            self.advance();
            let e = self.unary();
            let span = start..e.span.end;
            return Spanned::new(Node::Negate(Box::new(e)), span);
        }
        self.postfix()
    }

    fn postfix(&mut self) -> Spanned<Node> {
        let mut e = self.primary();
        loop {
            // call: identifier ( args )
            if self.is(&Token::LParen) {
                if let Node::Identifier(name) = &e.node {
                    let name = name.clone();
                    let start = e.span.start;
                    self.advance();
                    let args = self.arg_list();
                    self.expect_rparen();
                    let span = start..self.prev_span().end;
                    e = Spanned::new(Node::Call { name, args }, span);
                    continue;
                }
                break;
            }
            // safe access: user?.profile?.naam
            if self.is(&Token::SafeDot) {
                let start = e.span.start;
                let mut parts = match &e.node {
                    Node::Identifier(name) => vec![name.clone()],
                    _ => self.err(e.span.clone(), "?. sirf naam ke baad", "yahan ek naam hona chahiye", None),
                };
                while self.is(&Token::SafeDot) {
                    self.advance();
                    parts.push(self.ident());
                }
                let span = start..self.prev_span().end;
                e = Spanned::new(Node::SafeAccess { parts }, span);
                continue;
            }
            break;
        }
        e
    }

    fn primary(&mut self) -> Spanned<Node> {
        let span = self.cur_span();
        match self.peek() {
            Some(Token::Number(n)) => {
                let n = *n;
                self.advance();
                Spanned::new(Node::Number(n), span)
            }
            Some(Token::Str(s)) => {
                let s = s.clone();
                self.advance();
                self.string_node(&s, span)
            }
            Some(Token::Sahi) => {
                self.advance();
                Spanned::new(Node::Bool(true), span)
            }
            Some(Token::Ghalat) => {
                self.advance();
                Spanned::new(Node::Bool(false), span)
            }
            Some(Token::Khali) => {
                self.advance();
                Spanned::new(Node::Null, span)
            }
            Some(Token::Ident(name)) => {
                let name = name.clone();
                self.advance();
                Spanned::new(Node::Identifier(name), span)
            }
            Some(Token::LParen) => {
                self.advance();
                let e = self.expr();
                self.expect_rparen();
                e
            }
            Some(Token::LBracket) => self.list_literal(),
            Some(Token::Pucho) => {
                self.advance();
                let prompt = self.unary();
                let full = span.start..prompt.span.end;
                Spanned::new(Node::Pucho(Box::new(prompt)), full)
            }
            // Arduino calls reach us as dedicated tokens; treat them as calls.
            Some(Token::PinSet) => self.builtin_call("pin_set"),
            Some(Token::PinLikho) => self.builtin_call("pin_likho"),
            Some(Token::PinParho) => self.builtin_call("pin_parho"),
            Some(Token::Intezar) => self.builtin_call("intezar"),
            _ => self.err(
                self.cur_span(),
                "Yahan ek value chahiye",
                "samajh nahi aaya — yahan number, text, naam ya ( ) hona chahiye",
                None,
            ),
        }
    }

    fn builtin_call(&mut self, name: &str) -> Spanned<Node> {
        let start = self.cur_span().start;
        self.advance(); // the keyword token
        self.expect_lparen();
        let args = self.arg_list();
        self.expect_rparen();
        let span = start..self.prev_span().end;
        Spanned::new(Node::Call { name: name.to_string(), args }, span)
    }

    fn list_literal(&mut self) -> Spanned<Node> {
        let start = self.cur_span().start;
        self.advance(); // [
        self.skip_newlines();
        let mut items = Vec::new();
        if !self.is(&Token::RBracket) {
            loop {
                self.skip_newlines();
                items.push(self.expr());
                self.skip_newlines();
                if self.is(&Token::Comma) {
                    self.advance();
                    self.skip_newlines();
                    if self.is(&Token::RBracket) {
                        break; // trailing comma
                    }
                } else {
                    break;
                }
            }
        }
        self.expect_rbracket();
        let span = start..self.prev_span().end;
        Spanned::new(Node::List(items), span)
    }

    /// Comma-separated argument expressions; caller consumes the closing ')'.
    fn arg_list(&mut self) -> Vec<Spanned<Node>> {
        let mut args = Vec::new();
        self.skip_newlines();
        if self.is(&Token::RParen) {
            return args;
        }
        loop {
            self.skip_newlines();
            args.push(self.expr());
            self.skip_newlines();
            if self.is(&Token::Comma) {
                self.advance();
            } else {
                break;
            }
        }
        args
    }

    fn binop(&self, op: BinOp, left: Spanned<Node>, right: Spanned<Node>) -> Spanned<Node> {
        let span = left.span.start..right.span.end;
        Spanned::new(Node::BinOp { op, left: Box::new(left), right: Box::new(right) }, span)
    }

    // ----------------------------------------------------------------
    // String interpolation: "Salam {naam}, umar {umar}"
    // ----------------------------------------------------------------

    fn string_node(&self, raw: &str, span: Span) -> Spanned<Node> {
        let chars: Vec<char> = raw.chars().collect();
        let mut parts: Vec<InterpolPart> = Vec::new();
        let mut text = String::new();
        let mut has_interp = false;
        let mut i = 0;

        while i < chars.len() {
            let c = chars[i];
            if c == '\\' && i + 1 < chars.len() {
                // process escape sequences the lexer left untouched
                let e = match chars[i + 1] {
                    'n' => '\n',
                    't' => '\t',
                    'r' => '\r',
                    '"' => '"',
                    '\\' => '\\',
                    '{' => '{',
                    '}' => '}',
                    other => other,
                };
                text.push(e);
                i += 2;
                continue;
            }
            if c == '{' {
                has_interp = true;
                if !text.is_empty() {
                    parts.push(InterpolPart::Text(std::mem::take(&mut text)));
                }
                i += 1;
                let mut inner = String::new();
                while i < chars.len() && chars[i] != '}' {
                    inner.push(chars[i]);
                    i += 1;
                }
                if i < chars.len() {
                    i += 1; // skip '}'
                }
                let node = self.parse_embedded(&inner);
                parts.push(InterpolPart::Expr(Box::new(node)));
                continue;
            }
            text.push(c);
            i += 1;
        }

        if !has_interp {
            return Spanned::new(Node::Str(text), span);
        }
        if !text.is_empty() {
            parts.push(InterpolPart::Text(text));
        }
        Spanned::new(Node::Interpolated(parts), span)
    }

    /// Parse the expression inside a `{...}` interpolation hole.
    fn parse_embedded(&self, text: &str) -> Spanned<Node> {
        use logos::Logos;
        let mut tokens = Vec::new();
        for (res, sp) in Token::lexer(text).spanned() {
            if let Ok(t) = res {
                tokens.push(TokSpanned { token: t, span: sp });
            }
        }
        if tokens.is_empty() {
            self.err(0..0, "Khali interpolation", "{ } ke andar kuch likhein", None);
        }
        let mut p = Parser::new(tokens, text.to_string(), self.filename.clone());
        p.expr()
    }
}
