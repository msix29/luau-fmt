//! Implements formatting traits for single tokens.

use luau_parser::types::{Comment, Token};

use crate::types::Format;

impl Format for Token {
    fn format(&self, _: &mut i32) -> String {
        self.word.to_string()
    }
}
impl Format for Comment {
    fn format(&self, _: &mut i32) -> String {
        self.0.to_string()
    }
}
