//! [`NewLineStyle`] enum.

/// New line styles.
#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum NewLineStyle {
    /// `\n` - default for Unix systems.
    #[default]
    LF,

    /// `\r\n` - default for Posix systems.
    #[allow(clippy::upper_case_acronyms)]
    CRLF,
}

impl NewLineStyle {
    /// Get self as a [`&str`](str)
    #[inline]
    pub fn as_str(&self) -> &str {
        match self {
            NewLineStyle::LF => "\n",
            NewLineStyle::CRLF => "\n\r",
        }
    }

    /// Get self as a [`String`]
    #[inline]
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(self) -> String {
        self.as_str().to_string()
    }
}
