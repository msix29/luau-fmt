use luau_parser::types::{HasRawValue, List};

use crate::types::{Format, FormatWithArgs};

impl<T: Format> FormatWithArgs<&str> for List<T> {
    fn format_with_args(&self, join_with: &str) -> String {
        self.items
            .iter()
            .map(|item| match item {
                luau_parser::types::ListItem::Trailing { item, separator } => {
                    format!("{}{}", item.format(), separator.get_raw_value())
                }
                luau_parser::types::ListItem::NonTrailing(item) => item.format(),
            })
            .collect::<Vec<String>>()
            .join(join_with)
    }
}
