//! Module holding all types used by this crate, and optionally by others.

/// A trait which represents that this struct can be formatted.
pub trait Format {
    /// Format this struct into a string.
    fn format(&self, indentation: &mut i32) -> String;
}

/// A trait which represents that this struct can be formatted but accepts args to
/// customize the end result.
pub trait FormatWithArgs<P> {
    /// Format this struct into a string.
    fn format_with_args(&self, indentation: &mut i32, parameter: P) -> String;
}

/// A trait which represents that this struct can be wrapped into multiple lines.
pub trait Wrap {
    /// Wrap this struct into a multi-line string. If possible.
    fn wrap(&self, formatted_string: &str) -> String {
        formatted_string.to_string()
    }
}

/// Struct representing the config fiel.
pub struct Config {
    /// The maximum width of characters per line, excluding comments.
    pub column_width: u32,
}
