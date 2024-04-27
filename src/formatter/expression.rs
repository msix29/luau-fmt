//! Holds all implementations for expressions.

// Clippy is drunk.
#![allow(clippy::invalid_regex)]

use lazy_static::lazy_static;
use luau_parser::types::{
    ElseIfExpression, Expression, FunctionArguments, FunctionCall, FunctionCallInvoked, Number,
    PrefixExp, SingleToken, StringLiteral, Table, TableAccess, TableAccessKey, TableAccessPrefix,
    TableField, TableFieldValue, TableKey, Var,
};
use regex::Regex;

use crate::{
    types::{Format, FormatWithArgs, QuoteStyle},
    TAB,
};

use super::CONFIG;

lazy_static! {
    static ref ESCAPED_QUOTE: Regex = Regex::new(r#"([^\\]((\\{2})+)?)\\'"#).unwrap();
    static ref QUOTE: Regex = Regex::new(r#"((\\{2})+)?'"#).unwrap();
    static ref ESCAPED_DOUBLE_QUOTE: Regex = Regex::new(r#"([^\\]((\\{2})+)?)\\""#).unwrap();
    static ref DOUBLE_QUOTE: Regex = Regex::new(r#"((\\{2})+)?""#).unwrap();
}

/// Formats a string and changes quote style if needed.
pub(crate) fn format_string(string: &SingleToken, indentation: &mut i32) -> String {
    let formatted = string.format(indentation);
    if formatted.starts_with('`') || (formatted.starts_with('[') && formatted.contains('\n')) {
        // If it's an interpolated strings, don't do anything to it.
        // And if it's a multi-line string, also don't do anything to it.
        return formatted;
    }
    let stripped_formatted = StringLiteral::strip_delimiters(&formatted);

    match CONFIG.read().unwrap().quote_style {
        QuoteStyle::Single => {
            return format!("'{}'", QUOTE.replace_all(&stripped_formatted, "\\'"))
        }
        QuoteStyle::PreferSingle => {
            if !QUOTE.is_match(&stripped_formatted) {
                return format!(
                    "'{}'",
                    ESCAPED_DOUBLE_QUOTE.replace_all(&stripped_formatted, "$1\"")
                );
            }
        }
        QuoteStyle::Double => {
            return format!(r#""{}""#, QUOTE.replace_all(&stripped_formatted, r#"\\""#))
        }
        QuoteStyle::PreferDouble => {
            if !DOUBLE_QUOTE.is_match(&stripped_formatted) {
                return format!(
                    r#""{}""#,
                    ESCAPED_QUOTE.replace_all(&stripped_formatted, "$1'")
                );
            }
        }
    };

    formatted
}

impl Format for Expression {
    fn format(&self, indentation: &mut i32) -> String {
        match self {
            Expression::Nil(value)
            | Expression::Boolean(value)
            | Expression::Number(Number(value)) => value.format(indentation),
            Expression::String(string) => format_string(string, indentation),
            Expression::Function {
                generics,
                parameters,
                colon,
                returns,
                body,
                ..
            } => format!(
                "function{}({}){}{}\n{}end",
                generics.format_with_args(indentation, ""),
                parameters.format_with_args(indentation, " "),
                colon.as_ref().map_or_else(|| "", |_| ": "),
                returns
                    .as_ref()
                    .map_or_else(String::new, |returns| returns.format(indentation)),
                body.format(&mut (*indentation + 1)),
            ),
            Expression::FunctionCall(funcion_call) => funcion_call.format(indentation),
            Expression::ExpressionWrap(wrap) => {
                format!("({})", wrap.expression.format(indentation))
            }
            Expression::Var(var) => var.format(indentation),
            Expression::Table(table) => {
                table.format_with_args(&mut (*indentation + 1), (" = ", ","))
            }
            Expression::UnaryExpression {
                operator,
                expression,
            } => {
                let operator = operator.format(indentation);
                if operator == "not" {
                    format!("not {}", expression.format(indentation))
                } else {
                    format!("{}{}", operator, expression.format(indentation))
                }
            }
            Expression::BinaryExpression {
                left,
                operator,
                right,
            } => {
                let operator = operator.format(indentation);
                if operator == "^" {
                    format!("{}^{}", left.format(indentation), right.format(indentation))
                } else {
                    format!(
                        "{} {} {}",
                        left.format(indentation),
                        operator,
                        right.format(indentation)
                    )
                }
            }
            Expression::Cast {
                expression,
                cast_to,
                ..
            } => format!(
                "{} :: {}",
                expression.format(indentation),
                cast_to.format(indentation)
            ),
            Expression::IfExpression {
                condition,
                if_expression,
                else_if_expressions,
                else_expression,
                ..
            } => {
                if else_if_expressions.len() > 0 {
                    let separator = format!("\n{}", TAB.repeat((*indentation + 1) as usize));
                    format!(
                        "if {} then {}\
                        {separator}{}\
                        {separator}else {}",
                        condition.format(indentation),
                        if_expression.format(indentation),
                        else_if_expressions
                            .iter()
                            .map(|else_if| else_if.format(indentation))
                            .collect::<Vec<String>>()
                            .join(&separator),
                        else_expression.format(indentation)
                    )
                } else {
                    format!(
                        "if {} then {} else {}",
                        condition.format(indentation),
                        if_expression.format(indentation),
                        else_expression.format(indentation)
                    )
                }
            }
        }
    }
}

impl Format for PrefixExp {
    fn format(&self, indentation: &mut i32) -> String {
        match self {
            PrefixExp::Var(var) => var.format(indentation),
            PrefixExp::FunctionCall(function_call) => function_call.format(indentation),
            PrefixExp::ExpressionWrap(wrap) => format!("({})", wrap.expression.format(indentation)),
        }
    }
}
impl Format for Var {
    fn format(&self, indentation: &mut i32) -> String {
        match self {
            Var::Name(name) => name.format(indentation),
            Var::TableAccess(table_access) => table_access.format(indentation),
        }
    }
}

impl Format for TableAccess {
    fn format(&self, indentation: &mut i32) -> String {
        format!(
            "{}{}",
            self.prefix.format(indentation),
            self.accessed_keys
                .iter()
                .map(|key| key.format(indentation))
                .collect::<Vec<String>>()
                .join("")
        )
    }
}
impl Format for TableAccessPrefix {
    fn format(&self, indentation: &mut i32) -> String {
        match self {
            TableAccessPrefix::Name(name) => name.format(indentation),
            TableAccessPrefix::FunctionCall(function_call) => function_call.format(indentation),
            TableAccessPrefix::ExpressionWrap(wrap) => {
                format!("({})", wrap.expression.format(indentation))
            }
        }
    }
}
impl Format for TableAccessKey {
    fn format(&self, indentation: &mut i32) -> String {
        match self {
            TableAccessKey::Expression(expression) => expression.format_with_args(indentation, ""),
            TableAccessKey::Name { name, .. } => format!(".{}", name.format(indentation)),
        }
    }
}

impl FormatWithArgs<(&str, &str)> for Table {
    fn format_with_args(
        &self,
        indentation: &mut i32,
        (key_value_separator, fields_separator): (&str, &str),
    ) -> String {
        let len = self.fields.len();
        if len == 0 {
            String::from("{}")
        } else if len == 1 {
            format!(
                "{{ {} }}",
                self.fields[0].format_with_args(indentation, key_value_separator)
            )
        } else {
            let spaces = TAB.repeat(*indentation as usize);
            format!(
                "{{\n{}{}{}\n{}}}",
                spaces,
                self.fields
                    .iter()
                    .map(|field| field.format_with_args(indentation, key_value_separator))
                    .collect::<Vec<String>>()
                    .join(&format!("{}\n{}", fields_separator, spaces)),
                fields_separator, // trailing
                TAB.repeat((indentation.saturating_sub(1)) as usize),
            )
        }
    }
}
impl FormatWithArgs<&str> for TableField {
    fn format_with_args(&self, indentation: &mut i32, key_value_separator: &str) -> String {
        format!(
            "{}{}",
            self.key.format_with_args(indentation, key_value_separator),
            self.value.format(indentation)
        )
    }
}
impl FormatWithArgs<&str> for TableKey {
    fn format_with_args(&self, indentation: &mut i32, separator: &str) -> String {
        match self {
            TableKey::UndefinedNumber(_) | TableKey::UndefinedString(_) => String::new(),
            TableKey::String(string) => format!("{}{}", string.format(indentation), separator),
            TableKey::Expression { expression, .. } => {
                format!("[{}]{}", expression.format(indentation), separator)
            }
            TableKey::Type { r#type, .. } => {
                format!("[{}]{}", r#type.format(indentation), separator)
            }
        }
    }
}
impl Format for TableFieldValue {
    fn format(&self, indentation: &mut i32) -> String {
        match self {
            TableFieldValue::Expression(expression) => expression.format(indentation),
            TableFieldValue::Type(r#type) => r#type.format(indentation),
        }
    }
}

impl Format for FunctionCall {
    fn format(&self, indentation: &mut i32) -> String {
        format!(
            "{}{}",
            self.invoked.format(indentation),
            self.arguments.format(indentation)
        )
    }
}
impl Format for FunctionCallInvoked {
    fn format(&self, indentation: &mut i32) -> String {
        match self {
            FunctionCallInvoked::Function(name) => name.format(indentation),
            FunctionCallInvoked::TableMethod { table, method, .. } => format!(
                "{}:{}",
                table.format(indentation),
                method.format(indentation)
            ),
        }
    }
}
impl Format for FunctionArguments {
    fn format(&self, indentation: &mut i32) -> String {
        match self {
            FunctionArguments::String(string) => string.format(indentation),
            FunctionArguments::Table(table) => {
                table.format_with_args(&mut (*indentation + 1), (" = ", ","))
            }
            FunctionArguments::List { arguments, .. } => {
                format!("({})", arguments.format_with_args(indentation, " "))
            }
        }
    }
}

impl Format for ElseIfExpression {
    fn format(&self, indentation: &mut i32) -> String {
        format!(
            "elseif {} then {}",
            self.condition.format(indentation),
            self.expression.format(indentation)
        )
    }
}

#[cfg(test)]
mod test {
    use crate::formatter::expression::format_string;
    use luau_parser::types::SingleToken;

    macro_rules! test_strings {
        ($input:literal, $output:literal) => {
            assert_eq!(format_string(&SingleToken::new($input), &mut 0), $output)
        };
    }

    /*
    local _ = `hi, it's me!`
    local _ = "hi, it's me!"
    local _ = "hi, it's me!"
    local _ = [[
        hi, it's me!
    ]]
     */

    #[test]
    fn string_formatting_1() {
        test_strings!(r#""hi""#, r#""hi""#)
    }

    #[test]
    fn string_formatting_2() {
        test_strings!(
            r#""Escaped quotes are like \"this!\"""#,
            r#""Escaped quotes are like \"this!\"""#
        )
    }

    #[test]
    fn string_formatting_3() {
        test_strings!(
            r#"'Escaped quotes are like "this!"'"#,
            r#"'Escaped quotes are like "this!"'"#
        )
    }

    #[test]
    fn string_formatting_4() {
        test_strings!("`backticks don't change`", "`backticks don't change`")
    }

    #[test]
    fn string_formatting_5() {
        test_strings!(r#""It's lovely""#, r#""It's lovely""#)
    }

    #[test]
    fn string_formatting_6() {
        test_strings!("'It\'s lovely'", r#""It's lovely""#)
    }
}
