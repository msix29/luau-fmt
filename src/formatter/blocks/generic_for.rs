//! Implements formatting traits for generic for loops.

use luau_parser::types::GenericFor;

use crate::types::{Format, FormatWithArgs};

impl Format for GenericFor {
    fn format(&self, indentation: &mut i32) -> String {
        format!(
            "for {} in {} {}",
            self.names.format_with_args(indentation, " "),
            self.expressions.format_with_args(indentation, " "),
            self.do_block.format(indentation),
        )
    }
}
