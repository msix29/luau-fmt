use luau_parser::types::LocalAssignment;

use crate::types::{Format, FormatWithArgs};

impl Format for LocalAssignment {
    fn format(&self, indentation: &mut i32) -> String {
        format!(
            "local {} = {}",
            self.name_list.format_with_args(indentation, " "),
            self.expressions.format_with_args(indentation, " ")
        )
    }
}
