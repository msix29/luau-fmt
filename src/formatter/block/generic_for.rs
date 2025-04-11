//! All `impl` blocks for [`GenericFor`].

use luau_parser::types::GenericFor;

use crate::{
    config::Config,
    traits::{Format, FormatWithArgs, Indentation},
};

impl Format for GenericFor {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        let mut string = self.for_keyword.format(indentation, config);
        string.push(' ');
        string.push_str(&self.names.format_with_args(indentation, config, ", "));
        string.push(' ');
        string.push_str(&self.in_keyword.format(indentation, config));
        string.push(' ');
        string.push_str(&self.expressions.format_with_args(indentation, config, ", "));
        string.push(' ');
        string.push_str(&self.do_block.format(indentation, config));

        string
    }
}
