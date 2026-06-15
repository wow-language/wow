use crate::ast::{Node, Spanned};
use crate::lexer::Spanned as TokSpanned;

/// Parse a flat token stream into an AST.
/// Phase 1 implements the core language: variables, bol, agar/warna,
/// har loops, kaam/do, and expressions.
/// Later phases add phir, koshish, lao, and the platform-specific keywords.
pub fn parse(tokens: Vec<TokSpanned>, source: &str, filename: &str) -> Spanned<Node> {
    // TODO: implement with chumsky combinators
    // The parser will be written here in Phase 1.
    // Returning a placeholder program node so the project compiles today.
    Spanned::new(Node::Program(vec![]), 0..source.len())
}
