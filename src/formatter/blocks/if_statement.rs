//! Implements formatting traits for do blocks.

use luau_parser::types::{ElseIfStatement, ElseStatement, IfStatement};

use crate::{types::Format, TAB};

impl Format for IfStatement {
    fn format(&self, indentation: &mut i32) -> String {
        format!(
            "if {} then\n{}{}{}{}end",
            self.condition.format(indentation),
            self.body.format(&mut (*indentation + 1)),
            self.else_if_statements
                .iter()
                .map(|else_if_expression| else_if_expression.format(indentation))
                .collect::<String>(),
            self.else_statement
                .as_ref()
                .map_or_else(String::new, |else_expression| else_expression
                    .format(indentation)),
            TAB.repeat(*indentation as usize)
        )
    }
}
impl Format for ElseIfStatement {
    fn format(&self, indentation: &mut i32) -> String {
        format!(
            "{}elseif {} then\n{}",
            TAB.repeat(*indentation as usize),
            self.condition.format(indentation),
            self.body.format(&mut (*indentation + 1))
        )
    }
}
impl Format for ElseStatement {
    fn format(&self, indentation: &mut i32) -> String {
        format!(
            "{}else\n{}",
            TAB.repeat(*indentation as usize),
            self.body.format(&mut (*indentation + 1))
        )
    }
}
