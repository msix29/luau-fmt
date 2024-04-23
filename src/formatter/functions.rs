//! Implements formatting traits for local and global functions.

use luau_parser::types::{GlobalFunction, GlobalFunctionName, LocalFunction};

use crate::{
    types::{Format, FormatWithArgs},
    TAB,
};

impl Format for LocalFunction {
    fn format(&self, indentation: &mut i32) -> String {
        let return_type = if let Some(return_type) = &self.returns {
            format!(": {}", return_type.format(indentation))
        } else {
            String::new()
        };

        format!(
            "local function {}{}({}){}\n{}{}end",
            self.function_name.format(indentation),
            self.generics.format_with_args(indentation, ""),
            self.parameters.format_with_args(indentation, " "),
            return_type,
            self.body.format(indentation),
            TAB.repeat(*indentation as usize),
        )
    }
}

impl Format for GlobalFunction {
    fn format(&self, indentation: &mut i32) -> String {
        let return_type = if let Some(return_type) = &self.returns {
            format!(": {}", return_type.format(indentation))
        } else {
            String::new()
        };

        format!(
            "function {}{}({}){}\n{}{}end",
            self.function_name.format(indentation),
            self.generics.format_with_args(indentation, ""),
            self.parameters.format_with_args(indentation, " "),
            return_type,
            self.body.format(indentation),
            TAB.repeat(*indentation as usize),
        )
    }
}
impl Format for GlobalFunctionName {
    fn format(&self, indentation: &mut i32) -> String {
        match self {
            GlobalFunctionName::SimpleName(name) => name.format(indentation),
            GlobalFunctionName::Table {
                table,
                keys,
                method,
            } => {
                let method = if let Some(method) = method {
                    format!(":{}", method.format(indentation))
                } else {
                    String::new()
                };

                format!(
                    "{}{}{}",
                    table.format(indentation),
                    keys.format_with_args(indentation, ""),
                    method,
                )
            }
        }
    }
}
