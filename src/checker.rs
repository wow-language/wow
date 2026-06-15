use crate::ast::{Node, Spanned};
use crate::main::Target;

/// Walk the AST and verify that:
/// - No Arduino-only keywords appear in a C or Node build
/// - No Node-only keywords appear in a C or Arduino build
/// - All functions called exist (later phase)
/// - All types are consistent (later phase)
///
/// Uses ariadne to emit Roman Urdu error messages pointing at the
/// exact source location of each problem.
pub fn check(ast: &Spanned<Node>, target: &Target, source: &str, filename: &str) {
    // TODO: implement in Phase 2
}
