//! Module holding all types used by this crate, and optionally by others.

/// A trait which represents that this struct can be formatted.
pub trait Format {
    /// Format this struct into a string.
    fn format(&self, indentation: &mut i32) -> String;
}

/// A trait which represents that this struct can be formatted, the passed arguments
/// customize the end result.
pub trait FormatWithArgs<P> {
    /// Format this struct into a string.
    fn format_with_args(&self, indentation: &mut i32, parameter: P) -> String;
}
