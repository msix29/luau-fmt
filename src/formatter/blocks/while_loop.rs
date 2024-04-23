//! Implements formatting traits for while loops.

use luau_parser::types::WhileLoop;

use crate::types::Format;

impl Format for WhileLoop {
    fn format(&self, indentation: &mut i32) -> String {
        format!(
            "while {} {}",
            self.condition.format(indentation),
            self.do_block.format(indentation),
        )
    }
}
