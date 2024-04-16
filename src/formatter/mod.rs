mod expression;
mod list;
mod local_assignment;
mod name;
mod statement;
mod token;
mod type_definition;

use lazy_static::lazy_static;
use luau_parser::types::Ast;

use crate::types::{Config, Format};

lazy_static! {
    pub static ref CONFIG: Config = Config::default();
}

impl Format for Ast {
    fn format(&self, indentation: &mut i32) -> String {
        let len = self.statements.len();
        if len == 0 {
            return '\n'.to_string();
        }

        let mut code = String::new();
        let last_index = len - 1;
        for (i, token) in self.statements.iter().enumerate() {
            if i == last_index {
                code.push_str(&token.0.format(indentation));
            } else {
                code.push_str(token.0.format(indentation).trim_end());
            }
            code.push('\n');
        }

        format!("{}\n", code.trim())
    }
}

#[inline]
pub fn format_luau(ast: &Ast) -> String {
    ast.format(&mut 0)
}
