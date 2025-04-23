//! All `impl` blocks for table-related types:
//!
//! * [`TableKey`]
//! * [`TableField`]
//! * [`TableFieldValue`]
//! * [`Table`]

use luau_parser::types::{Table, TableField, TableFieldValue, TableKey};

use crate::{
    config::{Config, TrailingCommas},
    formatter::TokenFormatType,
    traits::{Format, FormatWithArgs, Indentation},
};

impl Format for TableKey {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        match self {
            TableKey::ERROR => unreachable!(),
            TableKey::UndefinedNumber(_) | TableKey::UndefinedString(_) => String::new(),
            TableKey::Simple(token) => {
                token.format_with(indentation, config, TokenFormatType::Name)
            }
            TableKey::Expression(bracketed) => bracketed.format(indentation + 1, config),
            TableKey::Type(bracketed) => bracketed.format(indentation + 1, config),
        }
    }
}

impl FormatWithArgs<bool> for TableField {
    fn format_with(&self, indentation: Indentation, config: &Config, is_type: bool) -> String {
        if self.equal_or_colon.is_none() {
            self.value.format(indentation, config)
        } else if is_type {
            self.key.format(indentation, config)
                + &self.equal_or_colon.format(indentation, config)
                + " "
                + &self.value.format(indentation, config)
        } else {
            self.key.format(indentation, config)
                + " "
                + &self.equal_or_colon.format(indentation, config)
                + " "
                + &self.value.format(indentation, config)
        }
    }
}

impl Format for TableFieldValue {
    #[inline]
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        match self {
            TableFieldValue::ERROR => unreachable!(),
            TableFieldValue::Expression(expression) => expression.format(indentation + 1, config),
            TableFieldValue::Type(type_value) => type_value.format(indentation + 1, config),
            TableFieldValue::VariadicValues(token) => token.format(indentation, config),
        }
    }
}

impl FormatWithArgs<bool> for Table {
    fn format_with(&self, indentation: Indentation, config: &Config, is_type: bool) -> String {
        if self.0.is_empty() {
            return self.0.opening_bracket.format(indentation, config)
                + &self.0.closing_bracket.format(indentation, config);
        }

        let single_line = config.compact_table.should_be_single_line(self);
        let spaces = if single_line {
            " ".to_string()
        } else {
            config.newline_style.to_string()
                + &config.indent_style.to_string(indentation + 1, config)
        };
        let separator = ",".to_string() + &spaces;

        let mut string = "{".to_string()
            + &spaces
            + &self
                .0
                .item
                .format_with(indentation, config, (&separator, is_type));

        match config.trailing_commas {
            TrailingCommas::Never => string = string.trim_end_matches(&separator).to_string(),
            TrailingCommas::OnlyMultiLine if !single_line && !string.ends_with(&separator) => {
                string.push_str(&separator)
            }
            TrailingCommas::Always if !string.ends_with(&separator) => string.push_str(&separator),
            _ => {
                if !string.ends_with(&separator) {
                    string.push_str(&spaces);
                }
            }
        }

        if !single_line {
            // Remove the last indentation.
            for _ in 0..config.indent_style.to_string(1, config).len() {
                string.pop();
            }
        }

        string + "}"
    }
}
