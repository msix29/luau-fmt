use luau_parser::types::Statement;

use crate::{config::Config, traits::{Format, Indentation}};

impl Format for Statement {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        todo!()
    }
}
