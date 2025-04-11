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
);

/// Struct representing the config file.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Config {
    /// The maximum width of characters per line.
    pub column_width: u32,

    /// The maximum width of a string per line.
    pub string_width: u32,

    /// The maximum width of characters in a comment per line.
    pub comments_width: u32,

    /// Quote style to use.
    pub quote_style: QuoteStyle,

    /// When to use compact table format (be one line).
    pub compact_table: CompactTable,

    /// Whether to use spaces or tabs.
    pub indent_style: IndentStyle,

    /// Number of spaces per tab (if [`Config::indent_style`] is
    /// [`IndentStyle::Spaces`]).
    pub indent_size: IndentSize,

    /// Whether to use CRLF, LF, or CR line endings.
    pub newline_style: NewLineStyle,

    /// When to have trailing commas in tables
    pub trailing_commas: TrailingCommas,

    /// Whether or not to keep the spacing between statements and not decrease it
    /// to a maximum of 2 lines (1 empty line).
    pub keep_statements_spacing: bool,

    /// When to have `;` after statements.
    pub semicolon: Semicolon,

    /// Whether or not to trim lines.
    pub trim_lines: bool,

    /// Whether or not to have a newline at the end of the file.
    pub add_final_newline: bool,

    pub variable_casing: NamingConvention,
    pub method_casing: NamingConvention,
    pub type_casing: NamingConvention,
    //TODO: Sorting `require` and `game:GetService` calls.
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
            indent_size: 4,

            newline_style: Default::default(),

            trailing_commas: Default::default(),

            keep_statements_spacing: false,
            semicolon: Default::default(),

            trim_lines: true,
            add_final_newline: true,

            variable_casing: NamingConvention::Camel,
            method_casing: NamingConvention::Pascal,
            type_casing: NamingConvention::Pascal,
        }
    }
}
