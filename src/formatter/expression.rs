//! Holds all implementations for expressions.

use luau_parser::types::{
    ElseIfExpression, Expression, FunctionArguments, FunctionCall, FunctionCallInvoked, PrefixExp,
    Table, TableAccess, TableAccessKey, TableAccessPrefix, TableField, TableFieldValue, TableKey,
    Var,
};

use crate::{
    types::{Format, FormatWithArgs},
    TAB,
};

impl Format for Expression {
    fn format(&self, indentation: &mut i32) -> String {
        match self {
            Expression::Nil(value) | Expression::Boolean(value) | Expression::Number(value) => {
                value.format(indentation)
            }
            // Strings will have more formatting to them.
            Expression::String(value) => value.format(indentation),
            Expression::Function {
                generics,
                parameters,
                colon,
                returns,
                body,
                ..
            } => format!(
                "function{}({}){}{}\n{}end",
                generics.format_with_args(indentation, " "),
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
                            .join(&format!("\n{separator}")),
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
        let len = self.fields.items.len();
        if len == 0 {
            String::from("{}")
        } else if len == 1 {
            format!(
                "{{ {} }}",
                self.fields.items[0].format_with_args(indentation, key_value_separator)
            )
        } else {
            let spaces = TAB.repeat(*indentation as usize);
            format!(
                "{{\n{}{}{}\n{}}}",
                spaces,
                self.fields
                    .items
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
                format!("({})", arguments.format_with_args(indentation, ", "))
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
