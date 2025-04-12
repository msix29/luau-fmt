//! Module holding all types used by this crate, and optionally by others.

use crate::config::Config;

/// The current indentation (in tabs).
pub type Indentation = u32;

/// A trait which represents that this struct can be formatted.
pub trait Format {
    /// Format this struct into a string.
    fn format(&self, indentation: Indentation, config: &Config) -> String;
}

/// A trait which represents that this struct can be expanded to multiple strings.
pub trait Expand {
    /// Expand this struct into a multi-string.
    fn expand(&self, indentation: Indentation, config: &Config) -> String;
}

/// A trait which represents that this struct can be formatted, the passed arguments
/// customize the end result.
pub trait FormatWithArgs<P> {
    /// Format this struct into a string.
    fn format_with_args(&self, indentation: Indentation, config: &Config, args: P) -> String;
}


/// A trait which represents that this struct can be expanded, the passed arguments
/// customize the end result.
pub trait ExpandWithArgs<P> {
    /// Expand this struct into a string.
    fn expand_with_args(&self, indentation: Indentation, config: &Config, args: P) -> String;
}
