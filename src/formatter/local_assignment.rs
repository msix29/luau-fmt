//! Holds all implementations for local assignments.

use luau_parser::types::LocalAssignment;

use crate::types::{Format, FormatWithArgs};

impl Format for LocalAssignment {
    fn format(&self, indentation: &mut i32) -> String {
        format!(
            "local {} {} {}",
            self.name_list.format_with_args(indentation, " "),
            self.equal_token
                .as_ref()
                .map_or_else(String::new, |equal_token| equal_token.format(indentation)),
            self.expressions.format_with_args(indentation, " ")
        )
        .trim_end()
        .to_string()
    }
}
