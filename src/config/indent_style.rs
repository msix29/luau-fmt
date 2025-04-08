//! [`IndentStyle`] enum.

/// Number of spaces that represent a tab
pub type IndentSize = usize;

/// Styles for indentation
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum IndentStyle {
    /// Use spaces where each tab is <n> spaces.
    #[default]
    Spaces,

    /// One tab (`\t`) per indentation.
    Tabs,
}
