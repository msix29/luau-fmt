//! [`QuoteStyle`] enum

/// Possible styles of quotes.
///
/// # Note
///
/// This doesn't affect backticks (interpolated strings).
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
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
    #[default]
    PreferDouble,
}
