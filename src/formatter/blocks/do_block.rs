use luau_parser::types::DoBlock;

use crate::types::Format;

impl Format for DoBlock {
    fn format(&self, indentation: &mut i32) -> String {
        format!(
            "do\n{}end",
            self.body.format(&mut (*indentation + 1))
        )
    }
}
