use crate::ast::{BinOp, HttpMethod, InterpolPart, Node, Spanned};

/// Walk the AST and emit valid C source code.
/// The output is a single .c file that is compiled by gcc.
/// The auzaar runtime (runtime/auzaar/auzaar.h) is included automatically.
pub fn generate(ast: &Spanned<Node>) -> String {
    let mut out = String::new();

    out.push_str("#include <stdio.h>\n");
    out.push_str("#include <stdlib.h>\n");
    out.push_str("#include <string.h>\n");
    out.push_str("#include \"auzaar.h\"\n\n");

    if let Node::Program(stmts) = &ast.node {
        // Forward-declare user functions so call order doesn't matter
        // TODO: implement in Phase 1

        out.push_str("int main() {\n");
        for stmt in stmts {
            out.push_str(&gen_node(stmt, 1));
        }
        out.push_str("    return 0;\n}\n");
    }

    out
}

fn gen_node(node: &Spanned<Node>, depth: usize) -> String {
    let indent = "    ".repeat(depth);
    match &node.node {
        // TODO: implement each node kind in Phase 1
        _ => format!("{indent}/* TODO */\n"),
    }
}
