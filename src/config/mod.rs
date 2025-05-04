//! Module holding all types related to the config file.

macro_rules! reexport {
    ($($name: ident),* $(,)?) => {
        $( mod $name; )*

        $( pub use $name::*; )*
    };
}

reexport!(
    quote_style,
    compact_table,
    indent_style,
    newline_style,
    trailing_commas,
    semicolon,
    naming_convention,
    function_parenthesis,
);

/// Struct representing the config file.
#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Config {
    /// The maximum width of characters per line. This configuration isn't very strict
    /// as it doesn't actually limit the width of characters on the whole line but
    /// rather parts of line. For example, an expression can never pass this width yet
    /// a full local assignment can pass it. Also indentations aren't counted as
    /// characters.
    #[serde(default)]
    pub column_width: usize,

    /// The maximum width of a string per line. This fully overrides (and ignores)
    /// `column_width`.
    #[serde(default)]
    pub string_width: usize,

    /// The maximum width of characters in a comment per line.
    #[serde(default)]
    pub comments_width: usize,

    /// Quote style to use.
    #[serde(default)]
    pub quote_style: QuoteStyle,

    /// When to use compact table format (be one line).
    #[serde(default)]
    pub compact_table: CompactTable,

    /// Whether to use spaces or tabs.
    #[serde(default)]
    pub indent_style: IndentStyle,

    /// Number of spaces per tab (if [`Config::indent_style`] is
    /// [`IndentStyle::Spaces`]).
    #[serde(default)]
    pub tab_size: IndentSize,

    /// Whether to use CRLF, LF, or CR line endings.
    #[serde(default)]
    pub newline_style: NewLineStyle,

    /// When to have trailing commas in tables
    #[serde(default)]
    pub trailing_commas: TrailingCommas,

    /// Whether or not to keep the spacing between statements and not decrease it
    /// to a maximum of 2 lines (1 empty line).
    #[serde(default)]
    pub keep_statements_spacing: bool,

    /// When to have `;` after statements.
    #[serde(default)]
    pub semicolon: Semicolon,

    /// Whether or not to have a newline at the end of the file.
    #[serde(default)]
    pub add_final_newline: bool,

    /// Naming convention to use for variables.
    #[serde(skip)]
    #[serde(default)]
    pub variable_casing: NamingConvention,

    /// Naming convention to use for methods.
    #[serde(skip)]
    #[serde(default)]
    pub method_casing: NamingConvention,

    /// Naming convention to use for types.
    #[serde(skip)]
    #[serde(default)]
    pub type_casing: NamingConvention,

    /// Whether or not to sort `require(...)` that are in the same block.
    #[serde(default)]
    pub sort_requires: bool,

    /// Whether or not to sort `game:GetService(...)` and `game.<IDENT>` that
    /// are in the same block.
    #[serde(default)]
    pub sort_services: bool,

    /// When to include parenthesis around function arguments.
    #[serde(default)]
    pub function_parenthesis: FunctionParenthesis,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            column_width: 100,
            string_width: 60,
            comments_width: 80,

            quote_style: Default::default(),

            compact_table: Default::default(),

            indent_style: Default::default(),
            tab_size: 4,

            newline_style: Default::default(),

            trailing_commas: Default::default(),

            keep_statements_spacing: false,
            semicolon: Default::default(),

            add_final_newline: true,

            variable_casing: NamingConvention::None,
            method_casing: NamingConvention::None,
            type_casing: NamingConvention::None,

            sort_services: true,
            sort_requires: true,

            function_parenthesis: Default::default(),
        }
    }
}
