//! All `impl` blocks for:
//!
//! * [`SetExpression`]
//! * [`CompoundSetExpression`]

use luau_parser::types::{CompoundSetExpression, SetExpression};

use crate::{
    config::Config,
    traits::{Format, FormatWithArgs, Indentation},
};

impl Format for SetExpression {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        let mut string = self.variables.format_with(indentation, config, ", ");
        string.push(' ');
        string.push_str(&self.equal.format(indentation, config));
        string.push(' ');
        string.push_str(&self.values.format_with(indentation, config, ", "));

        string
    }
}

impl Format for CompoundSetExpression {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        let mut string = self.variable.format(indentation, config);
        string.push(' ');
        string.push_str(&self.operation.format(indentation, config));
        string.push(' ');
        string.push_str(&self.value.format(indentation, config));

        string
    }
}
