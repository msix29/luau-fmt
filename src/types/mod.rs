//! Module holding all types used by this crate, and optionally by others.

mod config;

pub use config::*;

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

/// A trait which represents that this struct can be expanded into multiple lines.
pub trait Expand {
    /// Expand this struct into a multi-line string, if possible. Default implementation
    /// just returns the string as-is.
    fn expand(&self, formatted_string: &str) -> String {
        formatted_string.to_string()
    }
}
