//! [`NewLineStyle`] enum.

///
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum NewLineStyle {
    /// `\r`
    CR,

    /// `\n` - default for Unix systems.
    #[default]
    LF,

    /// `\r\n` - default for Posix systems.
    CRLF,
}
