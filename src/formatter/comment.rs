//! Implements formatting traits for comments.

use luau_parser::types::{Comment, Print};

use crate::types::Format;

impl Format for Comment {
    fn format(&self, _: &mut i32) -> String {
        //TODO?
        self.print()
    }
}
