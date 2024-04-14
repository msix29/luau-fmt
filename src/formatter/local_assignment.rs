use luau_parser::types::{LocalAssignment, Print};

use crate::types::Format;

impl Format for LocalAssignment {
    fn format(&self) -> String {
        format!("local {}", self.name_list.print())
    }
}

