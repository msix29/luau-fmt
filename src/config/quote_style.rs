//! [`QuoteStyle`] enum

use luau_parser::prelude::LuauString;

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

const ESCAPED_QUOTE: &str = r"\'";
const QUOTE: &str = r"'";
const ESCAPED_DOUBLE_QUOTE: &str = r#"\""#;
const DOUBLE_QUOTE: &str = r#"""#;

macro_rules! convert {
    ($str: ident, $quotes: ident, $(($from:ident => $to:ident)),* $(,)?) => {{
        let mut string = $str $(.replace($from, $to))*;
        string.replace_range(..1, $quotes);
        string.replace_range(string.len() - 1.., $quotes);
        string
    }};
}

fn count_escapes(string: &str) -> usize {
    let bytes = string.as_bytes();

    let mut i = 1; // ignore first quote
    let end = bytes.len() - 2; // ignore final quote

    let mut escapes = 0;

    while i < end {
        if bytes[i - 1] == b'\\' && (bytes[i] == b'\'' || bytes[i] == b'"') {
            escapes += 1;
        }

        i += 1;
    }

    escapes
}

impl QuoteStyle {
    /// Convert the quote style of a [`LuauString`]
    ///
    /// # Panics
    ///
    /// * If [`self`] isn't [`QuoteStyle::Single`] or [`QuoteStyle::Double`].
    /// * If `luau_string` isn't [`LuauString::SingleQuotes`] or [`LuauString::DoubleQuotes`].
    fn convert(&self, luau_string: &LuauString) -> String {
        match self {
            QuoteStyle::Single => match luau_string {
                LuauString::SingleQuotes(smol_str) => smol_str.to_string(),
                LuauString::DoubleQuotes(smol_str) => convert!(
                    smol_str,
                    QUOTE,
                    (ESCAPED_DOUBLE_QUOTE => DOUBLE_QUOTE),
                    (QUOTE => ESCAPED_QUOTE)
                ),
                _ => unreachable!(),
            },
            QuoteStyle::Double => match luau_string {
                LuauString::SingleQuotes(smol_str) => convert!(
                    smol_str,
                    DOUBLE_QUOTE,
                    (ESCAPED_QUOTE => QUOTE),
                    (DOUBLE_QUOTE => ESCAPED_DOUBLE_QUOTE)
                ),
                LuauString::DoubleQuotes(smol_str) => smol_str.to_string(),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    /// Convert a [`LuauString`] to a [`String`].
    fn luau_string_to_string(luau_string: &LuauString) -> String {
        match luau_string {
            LuauString::SingleQuotes(smol_str)
            | LuauString::DoubleQuotes(smol_str)
            | LuauString::Backticks(smol_str)
            | LuauString::MultiLine(smol_str) => smol_str.to_string(),
        }
    }

    /// Pick the best string depending on the number of escaped characters they
    /// contain. If they are the same, the `preferred` (first argument) one is
    /// chosen.
    #[inline]
    fn pick_best(preferred: String, other: String) -> String {
        println!("preferred - {preferred}");
        println!("  other   - {other}");
        println!("{},{}", count_escapes(&preferred), count_escapes(&other));
        if count_escapes(&preferred) > count_escapes(&other) {
            other
        } else {
            preferred
        }
    }

    pub fn apply(&self, luau_string: &LuauString) -> String {
        match luau_string {
            LuauString::Backticks(smol_str) | LuauString::MultiLine(smol_str) => {
                smol_str.to_string()
            }
            _ => match self {
                QuoteStyle::PreferDouble if matches!(luau_string, LuauString::DoubleQuotes(_)) => {
                    Self::luau_string_to_string(luau_string)
                }
                QuoteStyle::PreferSingle if matches!(luau_string, LuauString::SingleQuotes(_)) => {
                    Self::luau_string_to_string(luau_string)
                }
                QuoteStyle::PreferSingle => Self::pick_best(
                    Self::convert(&Self::Single, luau_string),
                    Self::luau_string_to_string(luau_string),
                ),
                QuoteStyle::PreferDouble => Self::pick_best(
                    Self::convert(&Self::Double, luau_string),
                    Self::luau_string_to_string(luau_string),
                ),
                _ => self.convert(luau_string),
            },
        }
    }
}
