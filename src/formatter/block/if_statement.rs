//! All `impl` blocks for:
//!
//! * [`IfStatement`]
//! * [`ElseStatement`]
//! * [`ElseIfStatement`]

use luau_parser::types::{ElseIfStatement, ElseStatement, IfStatement};

use crate::{
    config::Config,
    traits::{Format, Indentation},
};

impl Format for IfStatement {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        let mut string = self.if_keyword.format(indentation, config);
        string.push(' ');
        string.push_str(&self.condition.format(indentation, config));
        string.push(' ');
        string.push_str(&self.then_keyword.format(indentation, config));
        string.push_str(&self.body.format(indentation + 1, config));

        for else_if_statement in self.else_if_statements.iter() {
            string.push_str(&else_if_statement.format(indentation, config));
        }
        string.push_str(&self.else_statement.format(indentation, config));
        string.push_str(&self.end_keyword.format(indentation, config));

        string
    }
}

impl Format for ElseIfStatement {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        let mut string = self.elseif_keyword.format(indentation, config);
        string.push(' ');
        string.push_str(&self.condition.format(indentation, config));
        string.push(' ');
        string.push_str(&self.then_keyword.format(indentation, config));
        string.push_str(&self.body.format(indentation + 1, config));

        string
    }
}

impl Format for ElseStatement {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        let mut string = self.else_keyword.format(indentation, config);
        string.push_str(&self.body.format(indentation + 1, config));

        string
    }
}
