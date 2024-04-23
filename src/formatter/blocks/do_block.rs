//! Implements formatting traits for do blocks.

use luau_parser::types::DoBlock;

use crate::{types::Format, TAB};

impl Format for DoBlock {
    fn format(&self, indentation: &mut i32) -> String {
        format!(
            "do\n{}{}end",
            self.body.format(&mut (*indentation + 1)),
            TAB.repeat(*indentation as usize),
        )
    }
}
