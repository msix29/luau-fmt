//! All `impl` blocks for [`GenericFor`].

use luau_parser::types::GenericFor;

use crate::{
    config::Config,
    traits::{Format, FormatWithArgs, Indentation},
};

impl Format for GenericFor {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        let mut string = "for ".to_string();
        string.push_str(&self.names.format_with_args(indentation, config, ", "));
        string.push_str(" in");
        string.push_str(&self.expressions.format_with_args(indentation, config, ", "));
        string.push(' ');
        string.push_str(&self.do_block.format(indentation, config));

        string
    }
}
