//! Holds wrapping implementations for expressions.
#![allow(unused)] //TODO: Remove.

use luau_parser::types::{Expression, FunctionArguments};

use crate::{
    types::{Expand, Format},
    TAB,
};

impl Expand for Expression {
    fn expand(&self, indentation: &mut i32) -> Option<String> {
        match self {
            Expression::Function {
                function_keyword,
                generics,
                opening_parenthesis,
                parameters,
                closing_parenthesis,
                colon,
                returns,
                body,
                end_keyword,
            } => todo!(),
            Expression::FunctionCall(function_call) => {
                let spaces = TAB.repeat(*indentation as usize);

                Some(format!(
                    "{}(\n{}{}\n{})",
                    function_call.invoked.format(indentation),
                    spaces,
                    function_call.arguments.expand(indentation).unwrap(),
                    spaces
                ))
            }
            Expression::ExpressionWrap(_) => todo!(),
            Expression::Var(_) => todo!(),
            Expression::Table(_) => todo!(),
            Expression::UnaryExpression {
                operator,
                expression,
            } => todo!(),
            Expression::BinaryExpression {
                left,
                operator,
                right,
            } => todo!(),
            Expression::Cast {
                expression,
                operator,
                cast_to,
            } => todo!(),
            Expression::IfExpression {
                if_token,
                condition,
                then_token,
                if_expression,
                else_if_expressions,
                else_token,
                else_expression,
            } => todo!(),
            _ => None,
        }
    }
}

impl Expand for FunctionArguments {}
