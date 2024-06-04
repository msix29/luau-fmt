//! Module holding everything related to the config file.

use crate::types::{Config, QuoteStyle, TableWrap};

impl Default for Config {
    fn default() -> Self {
        Self {
            column_width: 100,
            string_width: 80,
            comments_width: 80,
            quote_style: QuoteStyle::PreferDouble,
            table_wrap: TableWrap::SingleElement,
        }
    }
}
