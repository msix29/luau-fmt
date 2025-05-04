/// When to include a semicolon after statements
#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Semicolon {
    /// Never have semicolons.
    #[default]
    Never,

    /// Always add semicolons.
    Always,

    /// Keep them as they are, don't add nor remove.
    Keep,
}
