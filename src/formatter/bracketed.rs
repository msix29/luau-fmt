//! All `impl` blocks for [`Bracketed`].

use luau_parser::types::Bracketed;

use crate::{
    config::Config,
    traits::{Expand, ExpandWithArgs, Format, FormatWithArgs, Indentation},
};

/// Actual implementation for [`Bracketed<T>`] formatting.
macro_rules! format {
    (
        $self: ident,
        $indentation: ident,
        $config: ident,
        $item: expr;
    ) => {{
        let mut string = $self.opening_bracket.format($indentation, $config);
        string.push_str(&$item);
        string.push_str(&$self.closing_bracket.format($indentation, $config));

        string
    }};
}

impl<T: Format> Format for Bracketed<T> {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        format!(
            self,
            indentation,
            config,
            self.item.format(indentation, config);
        )
    }
}

impl<A, T: FormatWithArgs<A>> FormatWithArgs<A> for Bracketed<T> {
    fn format_with(&self, indentation: Indentation, config: &Config, args: A) -> String {
        format!(
            self,
            indentation,
            config,
            self.item.format_with(indentation, config, args);
        )
    }
}

/// Actual implementation for [`Bracketed<T>`] expanding.
macro_rules! expand {
    (
        $self: ident,
        $indentation: ident,
        $config: ident,
        let $item_name:ident = $item:expr;
    ) => {{
        let $item_name = $item;

        if $item_name.is_empty() {
            let mut string = $self.opening_bracket.format($indentation, $config);
            string.push_str(&$item_name);
            string.push_str(&$self.closing_bracket.format($indentation, $config));

            string
        } else {
            let mut string = $self.opening_bracket.format($indentation, $config);

            string.push_str(
                &($config.newline_style.to_string()
                    + &$config.indent_style.to_string($indentation, $config)),
            );
            string.push_str(&$item_name);
            string.push_str(
                &($config.newline_style.to_string()
                    + &$config.indent_style.to_string($indentation.saturating_sub(1), $config)),
            );
            string.push_str(&$self.closing_bracket.format($indentation, $config));

            string
        }
    }};
}

impl<A, T: FormatWithArgs<A>> ExpandWithArgs<A> for Bracketed<T> {
    fn expand_with(&self, indentation: Indentation, config: &Config, args: A) -> String {
        expand!(
            self,
            indentation,
            config,
            let item = self.item.format_with(indentation, config, args);
        )
    }
}

impl<T: Expand> Expand for Bracketed<T> {
    fn expand(&self, indentation: Indentation, config: &Config) -> String {
        expand!(
            self,
            indentation,
            config,
            let item = self.item.expand(indentation, config);
        )
    }
}
