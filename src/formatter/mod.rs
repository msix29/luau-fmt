//! Holds all implementations.

mod blocks;
mod expression;
mod list;
mod local_assignment;
mod name;
mod statement;
mod token;
mod type_definition;

use lazy_static::lazy_static;
use luau_parser::types::{Ast, AstStatus};

use crate::{
    types::{Config, Format},
    TAB,
};

lazy_static! {
    /// The global config specified by the user.
    pub static ref CONFIG: Config = Config::default();
}

/// An enum representing formatting errors that stopped [`format_luau`] from working.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum FormattingError {
    /// The [`ast`](Ast) is incomplete, meaning it had syntax errors.
    IncompleteAst,
}

impl Format for Ast {
    fn format(&self, indentation: &mut i32) -> String {
        let len = self.statements.len();
        if len == 0 {
            return String::new();
        }

        let spaces = TAB.repeat(*indentation as usize);
        let statement_separator = format!("\n{}", spaces);
        let mut code = String::new();
        let last_index = len - 1;
        for (i, token) in self.statements.iter().enumerate() {
            if i == last_index {
                code.push_str(&token.0.format(indentation));
            } else {
                code.push_str(token.0.format(indentation).trim_end());
            }
            code.push_str(&statement_separator);
        }

        format!("{}{}\n", spaces, code.trim())
    }
}

/// A "safer" version for [ast::format](Ast::format). This will return `Err`
/// if the [`ast's status`](Ast::status) isn't [`complete`](AstStatus::Complete).
/// This function is more "safe" as it ensures the resulted output will always
/// match the input code in terms of functionality.
#[inline]
pub fn format_luau(ast: &Ast) -> Result<String, FormattingError> {
    if ast.status == AstStatus::Complete {
        Ok(ast.format(&mut 0))
    } else {
        Err(FormattingError::IncompleteAst)
    }
}
