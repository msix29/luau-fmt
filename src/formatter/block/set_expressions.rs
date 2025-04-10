//! All `impl` blocks for:
//!
//! * [`SetExpression`]
//! * [`CompoundSetExpression`]

use luau_parser::types::SetExpression;

use crate::{
    config::Config,
    traits::{Format, Indentation, FormatWithArgs},
};

impl Format for SetExpression {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        let mut string = self.variables.format_with_args(indentation, config, " ");
        string.push_str(" = ");
        string.push_str(self.variables.format_with_args(indentation, config, " "));

        string
    }
}
