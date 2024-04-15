use luau_parser::types::SingleToken;

use crate::types::Format;

impl Format for SingleToken {
    fn format(&self, indentation: &mut i32) -> String {
        self.word.to_string()
    }
}
