//! All `impl` blocks for [`DoBlock`].

use luau_parser::types::DoBlock;

use crate::{
    config::Config,
    traits::{Format, Indentation},
};

impl Format for DoBlock {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        let mut string = self.do_keyword.format(indentation, config);
        string.push_str(&self.body.format(indentation + 1, config));
        string.push_str(&self.end_keyword.format(indentation, config));

        string
    }
}
