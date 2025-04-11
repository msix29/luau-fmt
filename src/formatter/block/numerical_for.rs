//! All `impl` blocks for [`NumericalFor`].

use luau_parser::types::NumericalFor;

use crate::{
    config::Config,
    traits::{Format, Indentation},
};

impl Format for NumericalFor {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        let mut string = self.for_keyword.format(indentation, config);
        string.push(' ');
        string.push_str(&self.start.format(indentation, config));
        string.push_str(&self.start_comma.format(indentation, config));
        string.push(' ');
        string.push_str(&self.end.format(indentation, config));

        if let Some(step) = self.step.as_ref() {
            string.push_str(&self.end_comma.format(indentation, config));
            string.push(' ');
            string.push_str(&step.format(indentation, config));
        }

        string.push(' ');
        string.push_str(&self.do_block.format(indentation, config));

        string
    }
}
