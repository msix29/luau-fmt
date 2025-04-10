//! All `impl` blocks for [`RepeatBlock`].

use luau_parser::types::RepeatBlock;

use crate::{
    config::Config,
    traits::{Format, Indentation},
};

impl Format for RepeatBlock {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        let mut string = "repeat".to_string();
        string.push_str(&self.body.format(indentation + 1, config));
        string.push_str("until ");
        string.push_str(&self.condition.format(indentation, config));

        string
    }
}
