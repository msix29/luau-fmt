use luau_parser::types::SingleToken;

use crate::types::Format;

impl Format for SingleToken {
    fn format(&self) -> String {
        self.word.to_string()
    }
}
