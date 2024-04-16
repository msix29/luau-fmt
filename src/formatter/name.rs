//! Holds all implementations for names.

use luau_parser::types::NormalizedName;

use crate::types::Format;

impl Format for NormalizedName {
    fn format(&self, indentation: &mut i32) -> String {
        if let Some(r#type) = &self.r#type {
            format!(
                "{}{}{}",
                self.name.format(indentation),
                self.colon
                    .as_ref()
                    .map_or(String::new(), |colon| format!("{} ", colon.format(indentation))),
                r#type.format(indentation)
            )
        } else {
            self.name.format(indentation)
        }
    }
}
