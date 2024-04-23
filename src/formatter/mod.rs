//! Holds all implementations.

mod blocks;
mod comment;
mod expression;
mod functions;
mod list;
mod local_assignment;
mod name;
mod set_expressions;
mod statement;
mod token;
mod type_definition;

use std::sync::Arc;

use lazy_static::lazy_static;
use luau_parser::types::{Ast, AstStatus, Print};

use crate::{
    types::{Config, Format, FormatWithArgs},
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
        for (statement, _) in self.statements.iter() {
            if statement.print().ends_with("\n\n") {
                code.push_str(&(statement.format(indentation).trim_end().to_string() + "\n"));
            } else {
                code.push_str(&statement.format(indentation));
            }
            code.push_str(&statement_separator);
        }

        format!("{}{}\n", spaces, code.trim())
    }
}

impl<T: Format> Format for Arc<T> {
    fn format(&self, indentation: &mut i32) -> String {
        (**self).format(indentation)
    }
}
impl<A: Format, B: Format> Format for (A, B) {
    fn format(&self, indentation: &mut i32) -> String {
        format!(
            "{}{}",
            self.0.format(indentation),
            self.1.format(indentation)
        )
    }
}

impl<P, T: FormatWithArgs<P>> FormatWithArgs<P> for Arc<T> {
    fn format_with_args(&self, indentation: &mut i32, parameter: P) -> String {
        (**self).format_with_args(indentation, parameter)
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
