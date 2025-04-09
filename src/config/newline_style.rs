//! [`NewLineStyle`] enum.

/// New line styles.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum NewLineStyle {
    /// `\r`
    CR,

    /// `\n` - default for Unix systems.
    #[default]
    LF,

    /// `\r\n` - default for Posix systems.
    #[allow(clippy::upper_case_acronyms)]
    CRLF,
}

impl NewLineStyle {
    #[inline]
    pub fn as_str(&self) -> &str {
        match self {
            NewLineStyle::CR => "\r",
            NewLineStyle::LF => "\n",
            NewLineStyle::CRLF => "\n\r",
        }
    }

    #[inline]
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(self) -> String {
        self.as_str().to_string()
    }
}
