//! [`IndentStyle`] enum.

use crate::traits::Indentation;

use super::Config;

/// Number of spaces that represent a tab
pub type IndentSize = usize;

/// Styles for indentation
#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IndentStyle {
    /// Use spaces where each tab is <n> spaces.
    #[default]
    Spaces,

    /// One tab (`\t`) per indentation.
    Tabs,
}

impl IndentStyle {
    #[inline]
    pub fn to_string(self, indentation: Indentation, config: &Config) -> String {
        match self {
            IndentStyle::Spaces => " ".repeat(config.tab_size * indentation as usize),
            IndentStyle::Tabs => "\t".repeat(indentation as usize),
        }
    }
}
