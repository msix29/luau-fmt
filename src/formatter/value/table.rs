//! All `impl` blocks for table-related types:
//!
//! * [`TableKey`]
//! * [`TableField`]
//! * [`TableFieldValue`]
//! * [`Table`]

use luau_parser::types::{Table, TableField, TableFieldValue, TableKey};

use crate::{
    config::{CompactTable, Config, TrailingCommas},
    formatter::TokenFormatType,
    traits::{Format, FormatWithArgs, Indentation},
};

impl Format for TableKey {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        match self {
            Self::ERROR => unreachable!(),
            Self::UndefinedNumber(_) | Self::UndefinedString(_) => String::new(),
            Self::Simple(token) => token.format_with(indentation, config, TokenFormatType::Name),
            Self::Expression(bracketed) => bracketed.format(indentation + 1, config),
            Self::Type(bracketed) => bracketed.format(indentation + 1, config),
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
            Self::ERROR => unreachable!(),
            Self::Expression(expression) => expression.format(indentation + 1, config),
            Self::Type(type_value) => type_value.format(indentation + 1, config),
            Self::VariadicValues(token) => token.format(indentation, config),
        }
    }
}

fn get_separator(single_line: bool, indentation: Indentation, config: &Config) -> (String, String) {
    let spaces = if single_line {
        " ".to_string()
    } else {
        config.newline_style.to_string() + &config.indent_style.to_string(indentation + 1, config)
    };

    let separator = ",".to_string() + &spaces;

    (spaces, separator)
}

impl FormatWithArgs<bool> for Table {
    fn format_with(&self, indentation: Indentation, config: &Config, is_type: bool) -> String {
        if self.0.is_empty() {
            return self.0.opening_bracket.format(indentation, config)
                + &self.0.closing_bracket.format(indentation, config);
        }

        let mut single_line = config.compact_table.should_be_single_line(self);
        let (mut spaces, mut separator) = get_separator(single_line, indentation, config);

        let mut string = "{".to_string()
            + &spaces
            + &self
                .0
                .item
                .format_with(indentation, config, (&separator, is_type));

        if single_line
            && config.compact_table != CompactTable::Always
            && string.len() > config.column_width
        {
            single_line = false;

            let (new_spaces, new_separator) = get_separator(single_line, indentation, config);
            string = string.replace(&separator, &new_separator);
            string.replace_range(1..spaces.len() + 1, &new_spaces);

            (spaces, separator) = (new_spaces, new_separator);
        }

        match config.trailing_commas {
            TrailingCommas::Never => string = string.trim_end_matches(&separator).to_string(),
            TrailingCommas::OnlyMultiLine => {
                let ends_with_separator = string.ends_with(&separator);
                if !single_line {
                    if !ends_with_separator {
                        string.push_str(&separator)
                    }
                } else if ends_with_separator {
                    string.replace_range(
                        string.len() - separator.len()..string.len() - separator.len() + 1,
                        "",
                    );
                } else {
                    string.push_str(&spaces);
                }
            }
            TrailingCommas::Always if !string.ends_with(&separator) => string.push_str(&separator),
            TrailingCommas::Always => {}
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
