//! All `impl` blocks for [`List`].

use luau_parser::types::{List, ListItem};

use crate::{
    config::Config,
    traits::{Format, FormatWithArgs, Indentation},
};

use super::block::filter_trivia_for_comments;

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
                item.format_with(indentation, config, args)
                    + &filter_trivia_for_comments(&original_separator.leading_trivia)
                    + separator
            }
            ListItem::NonTrailing(item) => item.format_with(indentation, config, args),
        }
    }
}
