//! [`CompactTable`] enum.

use luau_parser::types::{Expression, Table, TableFieldValue, TypeValue};

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

impl CompactTable {
    pub fn should_be_single_line(&self, table: &Table) -> bool {
        match self {
            CompactTable::Always => true,
            CompactTable::OnlyLiterals => !table.0.iter().any(|item| match &*item.value {
                // Should we handle wraps?
                TableFieldValue::Expression(
                    Expression::Nil(_)
                    | Expression::Boolean(_)
                    | Expression::Number(_)
                    | Expression::String(_),
                    // Should we include `Expression::Var(_)`?
                )
                | TableFieldValue::Type(
                    TypeValue::String(_)
                    | TypeValue::Boolean(_)
                    | TypeValue::Basic { .. }
                    | TypeValue::Module { .. }
                    | TypeValue::Nil(_),
                )
                | TableFieldValue::VariadicValues(_) => false,
                _ => true,
            }),
            CompactTable::SingleElement => table.0.len() == 1,
            CompactTable::Never => false,
        }
    }
}
