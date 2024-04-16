//! Implements formatting traits for single tokens.

use luau_parser::types::SingleToken;

use crate::types::Format;

impl Format for SingleToken {
    fn format(&self, _: &mut i32) -> String {
        self.word.to_string()
    }
}
