//! All `impl` blocks for [`Bracketed`].

use luau_parser::types::Bracketed;

use crate::{
    config::Config,
    traits::{ExpandWithArgs, Format, FormatWithArgs, Indentation},
};

impl<T: Format> Format for Bracketed<T> {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        let mut string = self.opening_bracket.format(indentation, config);
        string.push_str(&self.item.format(indentation, config));
        string.push_str(&self.closing_bracket.format(indentation, config));

        string
    }
}

impl<A, T: FormatWithArgs<A>> FormatWithArgs<A> for Bracketed<T> {
    fn format_with_args(&self, indentation: Indentation, config: &Config, args: A) -> String {
        let mut string = self.opening_bracket.format(indentation, config);
        string.push_str(&self.item.format_with_args(indentation, config, args));
        string.push_str(&self.closing_bracket.format(indentation, config));

        string
    }
}

impl<A, T: FormatWithArgs<A>> ExpandWithArgs<A> for Bracketed<T> {
    fn expand_with_args(&self, indentation: Indentation, config: &Config, args: A) -> String {
        let mut string = self.opening_bracket.format(indentation, config);
        string.push_str(
            &(config.newline_style.to_string()
                + &config.indent_style.to_string(indentation + 1, config)),
        );
        string.push_str(&self.item.format_with_args(indentation, config, args));
        string.push_str(
            &(config.newline_style.to_string()
                + &config.indent_style.to_string(indentation, config)),
        );
        string.push_str(&self.closing_bracket.format(indentation, config));

        string
    }
}
