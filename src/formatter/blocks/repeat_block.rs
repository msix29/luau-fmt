//! Implements formatting traits for repeat blocks.

use luau_parser::types::RepeatBlock;

use crate::{types::Format, TAB};

impl Format for RepeatBlock {
    fn format(&self, indentation: &mut i32) -> String {
        format!(
            "repeat\n{}{}until{}",
            self.body.format(&mut (*indentation + 1)),
            TAB.repeat(*indentation as usize),
            self.condition.format(indentation),
        )
    }
}
