//! All `impl` blocks for expression-related types

mod function;
mod table;
mod var;

use luau_parser::types::{ElseIfExpression, Expression, IfExpression, PrefixExp};

use crate::{
    config::Config,
    traits::{Format, FormatWithArgs, Indentation},
};

impl Format for PrefixExp {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        match self {
            PrefixExp::Var(var) => var.format(indentation, config),
            PrefixExp::FunctionCall(function_call) => function_call.format(indentation, config),
            PrefixExp::ExpressionWrap(bracketed) => bracketed.format(indentation, config),
        }
    }
}

impl Format for Expression {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        match self {
            Expression::ERROR => unreachable!(),
            Expression::Nil(token)
            | Expression::Boolean(token)
            | Expression::Number(token)
            | Expression::String(token) => token.format(indentation, config),
            Expression::Closure(closure) => closure.format(indentation, config),
            Expression::FunctionCall(function_call) => function_call.format(indentation, config),
            Expression::ExpressionWrap(bracketed) => bracketed.format(indentation, config),
            Expression::Var(var) => var.format(indentation, config),
            Expression::Table(table) => table.format_with_args(indentation, config, false),
            Expression::UnaryExpression {
                operator,
                expression,
            } => operator.format(indentation, config) + &expression.format(indentation, config),
            Expression::BinaryExpression {
                left,
                operator,
                right,
            } => {
                left.format(indentation, config)
                    + &operator.format(indentation, config)
                    + &right.format(indentation, config)
            }
            Expression::TypeCast {
                expression,
                operator,
                cast_to,
            } => {
                expression.format(indentation, config)
                    + " :: "
                    + &cast_to.format(indentation, config)
            }
            Expression::IfExpression(if_expression) => if_expression.format(indentation, config),
        }
    }
}

impl Format for IfExpression {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        let mut string = "if ".to_string();
        string.push_str(&self.condition.format(indentation, config));
        string.push_str(" then");
        string.push_str(&self.if_expression.format(indentation, config));

        for else_if in self.else_if_expressions.iter() {
            string.push_str(&else_if.format(indentation, config));
        }

        string.push_str(" else ");
        string.push_str(&self.else_expression.format(indentation, config));

        string
    }
}

impl Format for ElseIfExpression {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        let mut string = " elseif ".to_string();
        string.push_str(&self.condition.format(indentation, config));
        string.push_str(" then ");
        string.push_str(&self.expression.format(indentation, config));

        string
    }
}
