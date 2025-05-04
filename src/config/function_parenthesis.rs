//! [`FunctionParenthesis`] enum.

/// Different naming conventions
#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FunctionParenthesis {
    // Always include parenthesis.
    #[default]
    Always,

    // Keep it as it is; don't add nor remove parenthesis.
    Keep,

    /// Remove them only for strings (one argument max).
    RemoveForStrings,

    /// Remove them only for tables (one argument max).
    RemoveForTables,

    /// Remove when possible (strings or tables [one argument max]).
    RemoveWhenPossible,
}
