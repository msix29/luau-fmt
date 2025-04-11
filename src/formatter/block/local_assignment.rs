//! All `impl` blocks for [`LocalAssignment`].

use luau_parser::types::LocalAssignment;

use crate::{
    config::Config,
    traits::{Format, FormatWithArgs, Indentation},
};

impl Format for LocalAssignment {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        let mut string = self.local_token.format(indentation, config);
        string.push(' ');
        string.push_str(&self.name_list.format_with_args(indentation, config, ", "));

        if self.equal_token.is_some() {
            string.push(' ');
            string.push_str(&self.equal_token.format(indentation, config));
            string.push(' ');
            string.push_str(&self.expressions.format_with_args(indentation, config, ", "));
        }

        string
    }
}
