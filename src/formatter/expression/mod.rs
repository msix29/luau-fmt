//! All `impl` blocks for expression-related types

mod function;
mod table;
mod var;

use luau_parser::types::{ElseIfExpression, Expression, IfExpression, PrefixExp};

use crate::{
    config::Config,
    formatter::TokenFormatType,
    traits::{Format, FormatWithArgs, Indentation},
};

impl Format for PrefixExp {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        match self {
            PrefixExp::Var(var) => var.format(indentation, config),
            PrefixExp::FunctionCall(function_call) => function_call.format(indentation, config),
            PrefixExp::ExpressionWrap(bracketed) => {
                bracketed.format_with_args(indentation, config, "")
            }
        }
    }
}

impl Format for Expression {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        todo!()
    }
}

impl Format for IfExpression {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        todo!()
    }
}

impl Format for ElseIfExpression {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        todo!()
    }
}
