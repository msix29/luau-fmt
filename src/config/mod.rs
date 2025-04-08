//! Module holding all types related to the config file.

macro_rules! reexport {
    ($($name: ident),* $(,)?) => {
        $( mod $name; )*

        $( pub use $name::*; )*
    };
}

reexport!(quote_style, compact_table);

/// Struct representing the config file.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Config {
    /// The maximum width of characters per line.
    pub column_width: u32,

    /// The maximum width of a string per line.
    pub string_width: u32,

    /// The maximum width of characters in a comment per line.
    pub comments_width: u32,

    /// Quote style to use.
    pub quote_style: QuoteStyle,

    /// When to use compact table format (be one line).
    pub compact_table: CompactTable,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            column_width: 100,
            string_width: 60,
            comments_width: 80,
            quote_style: QuoteStyle::PreferDouble,
            compact_table: CompactTable::SingleElement,
        }
    }
}
