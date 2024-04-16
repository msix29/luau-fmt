//! Holds all implementation for lists.

use luau_parser::types::{HasRawValue, List, ListItem};

use crate::types::{Format, FormatWithArgs};

impl<T: Format> FormatWithArgs<&str> for List<T> {
    fn format_with_args(&self, indentation: &mut i32, join_with: &str) -> String {
        self.items
            .iter()
            .map(|item| match item {
                ListItem::Trailing { item, separator } => {
                    format!("{}{}", item.format(indentation), separator.get_raw_value())
                }
                ListItem::NonTrailing(item) => item.format(indentation),
            })
            .collect::<Vec<String>>()
            .join(join_with)
    }
}

impl<P: Copy, T: FormatWithArgs<P>> FormatWithArgs<(&str, P)> for List<T> {
    fn format_with_args(&self, indentation: &mut i32, (join_with, parameter): (&str, P)) -> String {
        self.items
            .iter()
            .map(|item| match item {
                ListItem::Trailing { item, separator } => {
                    format!(
                        "{}{}",
                        item.format_with_args(indentation, parameter),
                        separator.get_raw_value()
                    )
                }
                ListItem::NonTrailing(item) => {
                    item.format_with_args(indentation, parameter)
                }
            })
            .collect::<Vec<String>>()
            .join(join_with)
    }
}
