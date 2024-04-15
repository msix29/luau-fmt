use luau_parser::types::{ElseIfExpression, Expression};

use crate::{
    types::{Format, FormatWithArgs},
    TAB,
};

impl Format for Expression {
    fn format(&self, indentation: &mut i32) -> String {
        match self {
            Expression::Nil(value)
            | Expression::Boolean(value)
            | Expression::Number(value)
            | Expression::String(value) => value.format(indentation),
            Expression::Function {
                generics,
                parameters,
                colon,
                returns,
                body,
                ..
            } => format!(
                "function{}({}){}{}\n{}\nend",
                generics.format_with_args(indentation, " "),
                parameters.format_with_args(indentation, " "),
                colon.as_ref().map_or_else(|| "", |_| ": "),
                returns
                    .as_ref()
                    .map_or_else(String::new, |returns| returns.format(indentation)),
                body.format(indentation),
            ),
            Expression::FunctionCall(_) => todo!(),
            Expression::ExpressionWrap(_) => todo!(),
            Expression::Var(_) => todo!(),
            Expression::Table(_) => todo!(),
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

impl Format for ElseIfExpression {
    fn format(&self, indentation: &mut i32) -> String {
        format!(
            "elseif {} then {}",
            self.condition.format(indentation),
            self.expression.format(indentation)
        )
    }
}
