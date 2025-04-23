//! All `impl` blocks for:
//!
//! * [`IfStatement`]
//! * [`ElseStatement`]
//! * [`ElseIfStatement`]

use std::rc::Rc;

use luau_parser::types::{ElseIfStatement, ElseStatement, Expression, IfStatement};

use crate::{
    config::Config,
    traits::{Format, Indentation},
};

fn handle_condition(
    string: &mut String,
    condition: &Rc<Expression>,
    indentation: Indentation,
    config: &Config,
) {
    let condition = condition.format(indentation, config);
    let is_condition_multiline = condition.contains(config.newline_style.as_str());

    if is_condition_multiline {
        string.push_str(
            &(config.newline_style.to_string()
                + &config.indent_style.to_string(indentation + 1, config)),
        );
    }

    string.push_str(&condition);

    if is_condition_multiline {
        string.push_str(
            &(config.newline_style.to_string()
                + &config.indent_style.to_string(indentation, config)),
        );
    } else {
        string.push(' ');
    }
}

impl Format for IfStatement {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        let mut string = self.if_keyword.format(indentation, config);
        string.push(' ');
        handle_condition(&mut string, &self.condition, indentation, config);
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
        handle_condition(&mut string, &self.condition, indentation, config);
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
