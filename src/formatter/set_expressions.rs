//! Implements formatting traits for both set expressions and compount set expressions

use luau_parser::types::{CompoundSetExpression, SetExpression};

use crate::types::{Format, FormatWithArgs};

impl Format for SetExpression {
    fn format(&self, indentation: &mut i32) -> String {
        format!(
            "{} = {}",
            self.variables.format_with_args(indentation, " "),
            self.values.format_with_args(indentation, " "),
        )
    }
}
impl Format for CompoundSetExpression {
    fn format(&self, indentation: &mut i32) -> String {
        format!(
            "{} {} {}",
            self.variable.format(indentation),
            self.operation.format(indentation),
            self.value.format(indentation),
        )
    }
}
