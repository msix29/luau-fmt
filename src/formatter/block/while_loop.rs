//! All `impl` blocks for [`WhileLoop`].

use luau_parser::types::WhileLoop;

use crate::{
    config::Config,
    traits::{Format, Indentation},
};

impl Format for WhileLoop {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        let mut string = self.while_keyword.format(indentation, config);
        string.push(' ');
        string.push_str(&self.condition.format(indentation, config));
        string.push(' ');
        string.push_str(&self.do_block.format(indentation, config));

        string
    }
}
