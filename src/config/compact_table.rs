//! [`CompactTable`] enum.

use luau_parser::types::{Expression, Table, TableFieldValue, TypeValue, Var};

/// Enum representing when tables should use the compact form (be on one line).
#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompactTable {
    /// Always wrap the table to one line.
    Always,

    /// Only wrap the table to one line if it's made of literals such as strings, numbers.
    /// and booleans. If they get too big, they'll become multiline.
    #[default]
    OnlyLiterals,

    /// Only wrap the table to one line if it's a single element.
    SingleElement,

    /// Never wrap a table to one line. This won't affect empty tables.
    Never,
}

impl CompactTable {
    /// Whether or not the passed [`Expression`] counts as a literal.
    fn is_literal_expression(&self, expression: &Expression) -> bool {
        match expression {
            Expression::Nil(_)
            | Expression::Boolean(_)
            | Expression::UnaryExpression { .. }
            | Expression::BinaryExpression { .. }
            | Expression::TypeCast { .. }
            | Expression::Number(_)
            | Expression::Var(Var::Name(_))
            | Expression::String(_) => true,
            Expression::ExpressionWrap(wrap) => self.is_literal_expression(&wrap.item),
            Expression::Table(table) => self.should_be_single_line(table),
            _ => false,
        }
    }

    /// Whether or not the passed [`TypeValue`] counts as a literal.
    fn is_literal_type_value(&self, type_value: &TypeValue) -> bool {
        match type_value {
            TypeValue::String(_)
            | TypeValue::Boolean(_)
            | TypeValue::Basic { .. }
            | TypeValue::Module { .. }
            | TypeValue::GenericPack { .. }
            | TypeValue::VariadicPack { .. }
            | TypeValue::Nil(_) => true,
            TypeValue::Wrap(wrap) => self.is_literal_type_value(&wrap.item),
            TypeValue::Optional { base, .. } => self.is_literal_type_value(base),
            TypeValue::Table(table) => self.should_be_single_line(table),
            TypeValue::Typeof { inner, .. } => self.is_literal_expression(&inner.item),
            TypeValue::Union { left, right, .. } | TypeValue::Intersection { left, right, .. } => {
                self.is_literal_type_value(left) && self.is_literal_type_value(right)
            }
            // No need to check for tuples as tables can't have that type.
            _ => false,
        }
    }

    /// Whether or not the passed [`Table`] should be single line.
    pub fn should_be_single_line(&self, table: &Table) -> bool {
        match self {
            CompactTable::Always => true,
            CompactTable::OnlyLiterals => !table.0.iter().any(|item| match &*item.value {
                TableFieldValue::Expression(expression)
                    if self.is_literal_expression(expression) =>
                {
                    false
                }
                TableFieldValue::Type(type_value) if self.is_literal_type_value(type_value) => {
                    false
                }
                TableFieldValue::VariadicValues(_) => false,
                _ => true,
            }),
            CompactTable::SingleElement => table.0.len() == 1,
            CompactTable::Never => false,
        }
    }
}
