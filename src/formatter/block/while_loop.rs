//! All `impl` blocks for [`WhileLoop`].

use luau_parser::types::WhileLoop;

use crate::traits::Format;

impl Format for WhileLoop {
    fn format(&self, indentation: crate::traits::Indentation, config: &crate::config::Config) -> String {
        let string = "while ".to_string();
        string.push_str(&self.condition.format(indentation, config));
        string.push(' ');
        string.push_str(&self.do_block.format(indentation, config));

        string
    }
}
