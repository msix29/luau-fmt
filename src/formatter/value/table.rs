//! All `impl` blocks for table-related types:
//!
//! * [`TableKey`]
//! * [`TableField`]
//! * [`TableFieldValue`]
//! * [`Table`]

use luau_parser::types::{Table, TableField, TableFieldValue, TableKey};

use crate::{
    config::Config,
    formatter::TokenFormatType,
    traits::{Format, FormatWithArgs, Indentation},
};

impl Format for TableKey {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        match self {
            TableKey::ERROR => unreachable!(),
            TableKey::UndefinedNumber(_) | TableKey::UndefinedString(_) => "".to_string(),
            TableKey::Simple(token) => {
                token.format_with_args(indentation, config, TokenFormatType::Name)
            }
            TableKey::Expression(bracketed) => bracketed.format_with_args(indentation, config, " "),
            TableKey::Type(bracketed) => bracketed.format_with_args(indentation, config, " "),
        }
    }
}

impl FormatWithArgs<bool> for TableField {
    fn format_with_args(&self, indentation: Indentation, config: &Config, is_type: bool) -> String {
        if self.equal_or_colon.is_none() {
            self.value.format(indentation, config)
        } else if is_type {
            self.key.format(indentation, config) + ": " + &self.value.format(indentation, config)
        } else {
            self.key.format(indentation, config) + " = " + &self.value.format(indentation, config)
        }
    }
}

impl Format for TableFieldValue {
    #[inline]
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        match self {
            TableFieldValue::ERROR => unreachable!(),
            TableFieldValue::Expression(expression) => expression.format(indentation, config),
            TableFieldValue::Type(type_value) => type_value.format(indentation, config),
            TableFieldValue::VariadicValues(token) => token.format(indentation, config),
        }
    }
}

impl FormatWithArgs<bool> for Table {
    fn format_with_args(&self, indentation: Indentation, config: &Config, is_type: bool) -> String {
        todo!()
    }
}
