#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrailingCommas {
    /// Always have trailing commas.
    Always,

    /// Never have trailing commas.
    Never,

    /// Only have trailing commas in multi-line tables.
    #[default]
    OnlyMultiLine,
}
