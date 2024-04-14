mod list;
mod local_assignment;
mod token;

use luau_parser::types::Ast;

use crate::types::Format;

pub fn format_luau(ast: &Ast) -> String {
    let len = ast.statements.len();
    if len == 0 {
        return '\n'.to_string();
    }

    let mut code = String::new();
    let last_index = len - 1;
    for (i, token) in ast.statements.iter().enumerate() {
        if i == last_index {
            code.push_str(&token.format());
        } else {
            code.push_str(token.format().trim_end());
        }
    }

    format!("{}\n", code.trim())
}
