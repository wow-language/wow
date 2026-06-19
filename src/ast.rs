/// A span records where in the source file a node came from.
/// Used by ariadne to point at the right place in error messages.
pub type Span = std::ops::Range<usize>;

/// Every piece of a wow program is one of these nodes.
/// The backends each walk this tree and emit their target language.
#[derive(Debug, Clone)]
pub enum Node {
    // ----------------------------------------------------------------
    // Top level
    // ----------------------------------------------------------------

    /// The whole program: a list of top-level statements
    Program(Vec<Spanned<Node>>),

    // ----------------------------------------------------------------
    // Literals
    // ----------------------------------------------------------------

    Number(f64),
    Str(String),
    /// String with {interpolation} holes: list of either raw text or an expression
    Interpolated(Vec<InterpolPart>),
    Bool(bool),    // sahi / ghalat
    Null,          // khali
    Identifier(String),
    List(Vec<Spanned<Node>>),
    /// { naam: "Ahmad", umar: 10 }
    Object { pairs: Vec<(String, Spanned<Node>)> },

    // ----------------------------------------------------------------
    // Statements
    // ----------------------------------------------------------------

    /// x = expr  or  rakho x = expr
    Assign {
        name: String,
        value: Box<Spanned<Node>>,
    },

    /// likho expr
    Likho(Box<Spanned<Node>>),

    /// pucho "..."  — returns what the user typed
    Pucho(Box<Spanned<Node>>),

    /// agar cond { ... } warna agar cond { ... } warna { ... }
    Agar {
        condition: Box<Spanned<Node>>,
        then_body: Vec<Spanned<Node>>,
        else_ifs: Vec<(Spanned<Node>, Vec<Spanned<Node>>)>,
        else_body: Option<Vec<Spanned<Node>>>,
    },

    /// har i 0 se 10 tak { ... }
    HarRange {
        var: String,
        from: Box<Spanned<Node>>,
        to: Box<Spanned<Node>>,
        body: Vec<Spanned<Node>>,
    },

    /// har item mein lista { ... }
    HarList {
        var: String,
        list: Box<Spanned<Node>>,
        body: Vec<Spanned<Node>>,
    },

    /// 3 baar { ... }
    Baar {
        times: Box<Spanned<Node>>,
        body: Vec<Spanned<Node>>,
    },

    /// jabtak condition { ... }
    Jabtak {
        condition: Box<Spanned<Node>>,
        body: Vec<Spanned<Node>>,
    },

    /// roko — break
    Roko,

    /// aage — continue
    Aage,

    /// banao naam(param, param = default) { ... }
    Banao {
        naam: String,
        params: Vec<Param>,
        body: Vec<Spanned<Node>>,
    },

    /// do expr — return
    Bhejo(Box<Spanned<Node>>),

    /// koshish { ... } pakro ghalti { ... }
    Koshish {
        body: Vec<Spanned<Node>>,
        // consumed once error handling lands on a backend (later phase)
        #[allow(dead_code)]
        catch_var: Option<String>,
        catch_body: Vec<Spanned<Node>>,
    },

    /// lao naam from "path"
    // consumed once `lao` imports land on the Node backend (later phase)
    #[allow(dead_code)]
    Lao {
        name: String,
        path: String,
    },

    // ----------------------------------------------------------------
    // Expressions
    // ----------------------------------------------------------------

    /// function call: naam(args)
    Call {
        name: String,
        args: Vec<Spanned<Node>>,
    },

    /// binary operation: left op right
    BinOp {
        op: BinOp,
        left: Box<Spanned<Node>>,
        right: Box<Spanned<Node>>,
    },

    /// unary: nahi expr
    UnaryNot(Box<Spanned<Node>>),

    /// unary minus: -expr
    Negate(Box<Spanned<Node>>),

    /// expr phir fn phir fn ...
    Phir {
        value: Box<Spanned<Node>>,
        calls: Vec<PhirStep>,
    },

    /// "bara" agar x > 5 warna "chota"
    Ternary {
        condition: Box<Spanned<Node>>,
        then_val: Box<Spanned<Node>>,
        else_val: Box<Spanned<Node>>,
    },

    /// shaks.naam  — regular property access, crashes if not an object
    PropAccess { object: Box<Spanned<Node>>, prop: String },

    /// shaks.naam = "Ahmad"  — property mutation
    PropAssign { object: Box<Spanned<Node>>, prop: String, value: Box<Spanned<Node>> },

    /// shaks ka naam / ki naam / kay naam / shaks?.naam — safe access, returns khali if null
    SafePropAccess { object: Box<Spanned<Node>>, prop: String },

    /// safe access: user?.profile?.naam  (legacy form — parser now emits SafePropAccess)
    #[allow(dead_code)]
    SafeAccess {
        parts: Vec<String>,
    },

    // ----------------------------------------------------------------
    // Arduino-specific
    // ----------------------------------------------------------------

    /// banao shuru() { }
    ArduinoShuru(Vec<Spanned<Node>>),
    /// banao chalao() { }
    ArduinoChalao(Vec<Spanned<Node>>),

    // ----------------------------------------------------------------
    // Node/web-specific
    // ----------------------------------------------------------------

    /// server(port)
    Server(Box<Spanned<Node>>),
    /// rasta GET "/path" { }
    Rasta {
        // method + path are consumed once routes land on the Node backend (later phase)
        #[allow(dead_code)]
        method: HttpMethod,
        #[allow(dead_code)]
        path: String,
        body: Vec<Spanned<Node>>,
    },
    /// jawab expr
    Jawab(Box<Spanned<Node>>),
}

// ----------------------------------------------------------------
// Supporting types
// ----------------------------------------------------------------

/// A node paired with its source location
#[derive(Debug, Clone)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn new(node: T, span: Span) -> Self {
        Self { node, span }
    }
}

/// One step in a phir chain
#[derive(Debug, Clone)]
pub struct PhirStep {
    pub name: String,
    pub args: Vec<Spanned<Node>>,
    pub span: Span,
}

/// A function parameter, optionally with a default value
#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub default: Option<Box<Spanned<Node>>>,
}

/// One part of an interpolated string
#[derive(Debug, Clone)]
pub enum InterpolPart {
    /// Raw text: "Salam "
    Text(String),
    /// Expression in braces: {naam}
    Expr(Box<Spanned<Node>>),
}

/// Binary operators
#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    Add,     // +
    Sub,     // -
    Mul,     // *
    Div,     // /
    Mod,     // %
    Eq,      // ==
    NotEq,   // !=
    Lt,      // <
    Lte,     // <=
    Gt,      // >
    Gte,     // >=
    And,     // aur
    Or,      // ya
}

/// HTTP methods for rasta
#[derive(Debug, Clone)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}
