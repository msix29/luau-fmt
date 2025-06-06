//! Helper functions to get the trailing trivia from various [`luau_parser`] types.

use luau_parser::{
    prelude::{Expression, FunctionCall, TableAccessKey, TableKey, Token, Trivia, TypeValue, Var},
    types::FunctionArguments,
};

/// Gets the trailing trivia of a [`Token`]
#[inline]
pub fn get_trailing_trivia_token(token: &Token) -> &[Trivia] {
    &token.trailing_trivia
}

/// Gets the trailing trivia of an [`Expression`]
#[inline]
pub fn get_trailing_trivia_expr(expression: &Expression) -> &[Trivia] {
    match expression {
        Expression::ERROR => unreachable!(),
        Expression::Nil(token)
        | Expression::Boolean(token)
        | Expression::Number(token)
        | Expression::String(token) => get_trailing_trivia_token(token),
        Expression::Closure(closure) => get_trailing_trivia_token(&closure.end_keyword),
        Expression::FunctionCall(function_call) => get_trailing_trivia_function_call(function_call),
        Expression::ExpressionWrap(bracketed) => {
            get_trailing_trivia_token(&bracketed.closing_bracket)
        }
        Expression::Var(var) => get_trailing_trivia_var(var),
        Expression::Table(table) => get_trailing_trivia_token(&table.0.closing_bracket),
        Expression::UnaryExpression { expression, .. }
        | Expression::BinaryExpression {
            right: expression, ..
        } => get_trailing_trivia_expr(expression),
        Expression::TypeCast { cast_to, .. } => get_trailing_trivia_type(cast_to),
        Expression::IfExpression(if_expression) => {
            get_trailing_trivia_expr(&if_expression.else_expression)
        }
    }
}

/// Gets the trailing trivia of a [`TypeValue`]
#[inline]
pub fn get_trailing_trivia_type(type_value: &TypeValue) -> &[Trivia] {
    match type_value {
        TypeValue::ERROR => unreachable!(),
        TypeValue::String(token) | TypeValue::Boolean(token) | TypeValue::Nil(token) => {
            &token.trailing_trivia
        }
        TypeValue::Wrap(bracketed) => get_trailing_trivia_token(&bracketed.closing_bracket),
        TypeValue::Function { return_type, .. } => get_trailing_trivia_type(return_type),
        TypeValue::GenericPack { ellipsis, .. } => &ellipsis.trailing_trivia,
        TypeValue::Intersection { right, .. } | TypeValue::Union { right, .. } => {
            get_trailing_trivia_type(right)
        }
        TypeValue::Basic {
            base: name,
            generics,
        }
        | TypeValue::Module { name, generics, .. } => generics
            .as_ref()
            .map(|generics| &generics.closing_bracket.trailing_trivia)
            .unwrap_or(&name.trailing_trivia),
        TypeValue::Optional { question_mark, .. } => &question_mark.trailing_trivia,
        TypeValue::Table(table) => &table.0.closing_bracket.trailing_trivia,
        TypeValue::Typeof { inner, .. } => &inner.closing_bracket.trailing_trivia,
        TypeValue::Tuple(bracketed) => get_trailing_trivia_token(&bracketed.closing_bracket),
        TypeValue::Variadic { type_value, .. } => get_trailing_trivia_type(type_value),
        TypeValue::VariadicPack { name, .. } => &name.trailing_trivia,
    }
}

/// Gets the trailing trivia of a [`Var`]
#[inline]
fn get_trailing_trivia_var(var: &Var) -> &[Trivia] {
    match var {
        Var::ERROR => unreachable!(),
        Var::Name(token) => &token.trailing_trivia,
        Var::TableAccess(table_access) => match table_access.accessed_keys.last().unwrap() {
            TableAccessKey::Expression(table_key) => match &**table_key {
                TableKey::Simple(token) => &token.trailing_trivia,
                TableKey::Expression(bracketed) => {
                    get_trailing_trivia_token(&bracketed.closing_bracket)
                }
                TableKey::Type(bracketed) => get_trailing_trivia_token(&bracketed.closing_bracket),
                _ => unreachable!(),
            },
            TableAccessKey::Name { name, .. } => get_trailing_trivia_token(name),
        },
    }
}

/// Gets the trailing trivia of a [`FunctionCall`]
#[inline]
pub fn get_trailing_trivia_function_call(function_call: &FunctionCall) -> &[Trivia] {
    match &function_call.arguments {
        FunctionArguments::String(token) => get_trailing_trivia_token(token),
        FunctionArguments::Table(table) => get_trailing_trivia_token(&table.0.closing_bracket),
        FunctionArguments::List(bracketed) => get_trailing_trivia_token(&bracketed.closing_bracket),
    }
}
