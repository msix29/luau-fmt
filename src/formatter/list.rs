//! All `impl` blocks for [`List`].

use luau_parser::types::{List, ListItem};

use crate::{
    config::Config,
    traits::{Format, FormatWithArgs, Indentation},
};

use super::block::{filter_trivia_for_comments, filter_trivia_for_spaces};

impl<T: Format> FormatWithArgs<&str> for List<T> {
    fn format_with(&self, indentation: Indentation, config: &Config, separator: &str) -> String {
        let mut string = String::new();

        for item in self.iter() {
            string.push_str(&item.format_with(indentation, config, separator));
        }

        string
    }
}
impl<A: Clone, T: FormatWithArgs<A>> FormatWithArgs<(&str, A)> for List<T> {
    fn format_with(
        &self,
        indentation: Indentation,
        config: &Config,
        (separator, args): (&str, A),
    ) -> String {
        let mut string = String::new();

        for item in self.iter() {
            string.push_str(&item.format_with(indentation, config, (separator, args.clone())));
        }

        string
    }
}

impl<T: Format> FormatWithArgs<&str> for ListItem<T> {
    fn format_with(&self, indentation: Indentation, config: &Config, separator: &str) -> String {
        match self {
            ListItem::Trailing {
                item,
                separator: original_separator,
            } => {
                item.format(indentation, config)
                    + &filter_trivia_for_comments(&original_separator.leading_trivia)
                    + separator
            }
            ListItem::NonTrailing(item) => item.format(indentation, config),
        }
    }
}
impl<A, T: FormatWithArgs<A>> FormatWithArgs<(&str, A)> for ListItem<T> {
    fn format_with(
        &self,
        indentation: Indentation,
        config: &Config,
        (separator, args): (&str, A),
    ) -> String {
        match self {
            ListItem::Trailing {
                item,
                separator: original_separator,
            } => {
                let final_spaces = filter_trivia_for_spaces(&original_separator.trailing_trivia);

                // We check for newlines instead of the config's newline_style since the user
                // may not be using that style by default. \n is guaranteed o exist in any
                // new line.
                let string = item.format_with(indentation, config, args)
                    + &filter_trivia_for_comments(&original_separator.leading_trivia);

                if final_spaces.matches('\n').nth(1).is_some() {
                    // At least 2 spaces exist, so we limit to 2
                    // remove trailing spaces from the first one, then add the
                    // second one without including the separator itself
                    // (which is just the first character).
                    string
                        + separator.trim_end_matches(&config.indent_style.to_string(1, config))
                        + &separator[1..]
                } else {
                    string + separator
                }
            }
            ListItem::NonTrailing(item) => item.format_with(indentation, config, args),
        }
    }
}
