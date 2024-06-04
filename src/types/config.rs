//! Module holding all types related to the config file.

/// Possible styles of quotes.
///
/// # Note
///
/// This doesn't affect backticks (interpolated strings).
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum QuoteStyle {
    /// Always use single quotes.
    Single,

    /// Only use single quotes if the string will have less or the same amount of escape
    /// sequences.
    PreferSingle,

    /// Always use double quotes.
    Double,

    /// Only use double quotes if the string will have less or the same amount of escape
    /// sequences.
    PreferDouble,
}

/// Enum representing when tables should be wrapped
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TableWrap {
    /// Always wrap the table to one line.
    Always,

    /// Only wrap the table to one line if it's made of literals such as strings, numbers.
    /// and booleans.
    OnlyLiterals,

    /// Only wrap the table to one line if it's a single element.
    SingleElement,

    /// Never wrap a table to one line. This won't affect empty tables.
    Never,
}

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

    /// When a table should be wrapped.
    pub table_wrap: TableWrap,
}
