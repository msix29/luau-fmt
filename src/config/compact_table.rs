//! [`CompactTable`] enum.

/// Enum representing when tables should use the compact form (be on one line).
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum CompactTable {
    /// Always wrap the table to one line.
    Always,

    /// Only wrap the table to one line if it's made of literals such as strings, numbers.
    /// and booleans.
    #[default]
    OnlyLiterals,

    /// Only wrap the table to one line if it's a single element.
    SingleElement,

    /// Never wrap a table to one line. This won't affect empty tables.
    Never,
}
