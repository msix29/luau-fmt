//! All `impl` blocks for expression-related types

mod function;
mod table;
mod var;

use luau_parser::{
    prelude::{Operator, TokenType},
    types::{ElseIfExpression, Expression, IfExpression, PrefixExp},
};

use crate::{
    config::Config,
    traits::{Expand, Format, FormatWithArgs, Indentation},
};

impl Format for PrefixExp {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        match self {
            Self::Var(var) => var.format(indentation, config),
            Self::FunctionCall(function_call) => function_call.format(indentation, config),
            Self::ExpressionWrap(bracketed) => bracketed.format(indentation + 1, config),
        }
    }
}

impl Expand for PrefixExp {
    fn expand(&self, indentation: Indentation, config: &Config) -> String {
        match self {
            Self::Var(var) => var.expand(indentation, config),
            Self::FunctionCall(function_call) => function_call.expand(indentation, config),
            _ => self.format(indentation, config),
        }
    }
}

impl Format for Expression {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        let string = match self {
            Self::ERROR => unreachable!(),
            Self::Nil(token) | Self::Boolean(token) | Self::Number(token) | Self::String(token) => {
                token.format(indentation + 1, config)
            }
            Self::Closure(closure) => closure.format(indentation, config),
            Self::FunctionCall(function_call) => function_call.format(indentation, config),
            Self::ExpressionWrap(bracketed) => bracketed.format(indentation + 1, config),
            Self::Var(var) => var.format(indentation, config),
            Self::Table(table) => table.format_with(indentation, config, false),
            Self::UnaryExpression {
                operator,
                expression,
            } => {
                if matches!(operator.token_type, TokenType::Operator(Operator::Not)) {
                    operator.format(indentation, config)
                        + " "
                        + &expression.format(indentation, config)
                } else {
                    operator.format(indentation, config) + &expression.format(indentation, config)
                }
            }
            Self::BinaryExpression {
                left,
                operator,
                right,
            } => {
                if matches!(
                    operator.token_type,
                    TokenType::Operator(Operator::Exponentiation)
                ) {
                    left.format(indentation, config)
                        + &operator.format(indentation, config)
                        + &right.format(indentation, config)
                } else {
                    left.format(indentation, config)
                        + " "
                        + &operator.format(indentation, config)
                        + " "
                        + &right.format(indentation, config)
                }
            }
            Self::TypeCast {
                expression,
                operator,
                cast_to,
            } => {
                expression.format(indentation, config)
                    + " "
                    + &operator.format(indentation, config)
                    + " "
                    + &cast_to.format(indentation, config)
            }
            Self::IfExpression(if_expression) => if_expression.format(indentation, config),
        };

        if string.len() > config.column_width {
            // This simple gives priority when expanding. If we have
            // expr ~= expr and expr ~= expr
            // without this priority thing, it'll become something like:
            //
            // expr
            //     ~= expr and expr ~= expr
            //
            // but with it, it becomes
            //
            // expr ~= expr
            //     and expr ~= expr
            //
            // This also handles cases of super long strings and makes them not look
            // awkward.

            match self {
                Self::BinaryExpression {
                    left,
                    operator,
                    right,
                } => {
                    if matches!(&**right, Self::String(_)) | matches!(&**left, Self::String(_)) {
                        // This means it's just a super long string.
                        return string;
                    }

                    match &**right {
                        Self::BinaryExpression {
                            operator: right_operator,
                            ..
                        } => match right_operator.token_type {
                            TokenType::Operator(Operator::And | Operator::Or) => {
                                left.format(indentation, config)
                                    + " "
                                    + &operator.format(indentation, config)
                                    + " "
                                    + &right.expand(indentation, config)
                            }
                            _ => self.expand(indentation, config),
                        },
                        _ => self.expand(indentation, config),
                    }
                }
                _ => self.expand(indentation, config),
            }
        } else {
            string
        }
    }
}

impl Expand for Expression {
    fn expand(&self, indentation: Indentation, config: &Config) -> String {
        match self {
            Self::ERROR => unreachable!(),
            Self::Nil(token) | Self::Boolean(token) | Self::Number(token) | Self::String(token) => {
                token.format(indentation + 1, config)
            }
            Self::Closure(closure) => closure.format(indentation, config),
            Self::FunctionCall(function_call) => function_call.expand(indentation, config),
            Self::ExpressionWrap(bracketed) => bracketed.expand(indentation + 1, config),
            Self::Var(var) => var.expand(indentation, config),
            Self::Table(table) => table.format_with(indentation, config, false),
            Self::UnaryExpression {
                operator,
                expression,
            } => {
                if matches!(operator.token_type, TokenType::Operator(Operator::Not)) {
                    operator.format(indentation, config)
                        + " "
                        + &expression.format(indentation, config)
                } else {
                    operator.format(indentation, config) + &expression.format(indentation, config)
                }
            }
            Self::BinaryExpression {
                left,
                operator,
                right,
            } => {
                if matches!(
                    operator.token_type,
                    TokenType::Operator(Operator::Exponentiation)
                ) {
                    left.format(indentation, config)
                        + &operator.format(indentation, config)
                        + &right.format(indentation, config)
                } else {
                    left.format(indentation, config)
                        + config.newline_style.as_str()
                        + &config.indent_style.to_string(indentation + 1, config)
                        + &operator.format(indentation, config)
                        + " "
                        + &right.format(indentation, config)
                }
            }
            Self::TypeCast {
                expression,
                operator,
                cast_to,
            } => {
                expression.expand(indentation, config)
                    + " "
                    + &operator.format(indentation, config)
                    + " "
                    + &cast_to.expand(indentation, config)
            }
            Self::IfExpression(if_expression) => if_expression.expand(indentation, config),
        }
    }
}

impl Format for IfExpression {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        let mut string = self.if_keyword.format(indentation, config);
        string.push(' ');
        string.push_str(&self.condition.format(indentation, config));
        string.push(' ');
        string.push_str(&self.then_keyword.format(indentation, config));
        string.push(' ');
        string.push_str(&self.if_expression.format(indentation, config));

        for else_if in self.else_if_expressions.iter() {
            string.push_str(&else_if.format(indentation, config));
        }

        string.push(' ');
        string.push_str(&self.else_keyword.format(indentation, config));
        string.push(' ');
        string.push_str(&self.else_expression.format(indentation, config));

        string
    }
}
impl Expand for IfExpression {
    fn expand(&self, indentation: Indentation, config: &Config) -> String {
        let newline_separator = config.newline_style.to_string()
            + &config.indent_style.to_string(indentation + 1, config);

        let mut string = self.if_keyword.format(indentation, config);
        string.push(' ');
        string.push_str(&self.condition.format(indentation, config));
        string.push_str(&newline_separator);
        string.push_str(&self.then_keyword.format(indentation, config));
        string.push(' ');
        string.push_str(&self.if_expression.format(indentation, config));

        for else_if in self.else_if_expressions.iter() {
            string.push_str(&else_if.expand(indentation, config));
        }

        string.push_str(&newline_separator);
        string.push_str(&self.else_keyword.format(indentation, config));
        string.push(' ');
        string.push_str(&self.else_expression.format(indentation, config));

        string
    }
}

impl Format for ElseIfExpression {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        let mut string = " ".to_string();
        string.push_str(&self.else_if_keyword.format(indentation, config));
        string.push(' ');
        string.push_str(&self.condition.format(indentation, config));
        string.push(' ');
        string.push_str(&self.then_keyword.format(indentation, config));
        string.push(' ');
        string.push_str(&self.expression.format(indentation, config));

        string
    }
}
impl Expand for ElseIfExpression {
    fn expand(&self, indentation: Indentation, config: &Config) -> String {
        let newline_separator = config.newline_style.to_string()
            + &config.indent_style.to_string(indentation + 1, config);

        let mut string = " ".to_string();
        string.push_str(&self.else_if_keyword.format(indentation, config));
        string.push(' ');
        string.push_str(&self.condition.format(indentation, config));
        string.push_str(&newline_separator);
        string.push_str(&self.then_keyword.format(indentation, config));
        string.push(' ');
        string.push_str(&self.expression.format(indentation, config));

        string
    }
}
