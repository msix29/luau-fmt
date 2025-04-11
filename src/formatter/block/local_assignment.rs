//! All `impl` blocks for [`LocalAssignment`].

use luau_parser::types::LocalAssignment;

use crate::{
    config::Config,
    traits::{Format, FormatWithArgs, Indentation},
};

impl Format for LocalAssignment {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        let mut string = "local ".to_string();
        string.push_str(&self.name_list.format_with_args(indentation, config, " "));

        if self.equal_token.is_some() {
            string.push_str(" = ");
            string.push_str(&self.expressions.format_with_args(indentation, config, " "));
        }

        string
    }
}
