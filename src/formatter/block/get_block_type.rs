//! Gets the type of a block - used for grouping blocks together to arrange the
//! statements in them, like for `require` calls.

use luau_parser::{
    prelude::{Expression, Statement, Token, TokenType},
    types::{FunctionCall, FunctionCallInvoked, PrefixExp, TableAccess, TableAccessPrefix, Var},
};

use crate::Config;

/// The [`BlockType`] enum.
#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum BlockType {
    /// A `game:GetService` call, or a `game.<IDENT>`.
    GetService,

    /// A `require` call.
    Require,

    /// Neither of the above.
    None,
}

/// Tries getting the name (identifier) from the passed [`Token`].
#[inline]
pub fn get_name_from_token(token: &Token) -> Option<String> {
    matches!(token.token_type, TokenType::Identifier(_))
        .then(|| token.token_type.try_as_string().unwrap())
}

/// Tries getting the name (identifier) from the passed [`Var`]
#[inline]
pub fn get_name_from_var(var: &Var) -> Option<String> {
    match var {
        Var::Name(name)
        | Var::TableAccess(TableAccess {
            prefix: TableAccessPrefix::Name(name),
            ..
        }) => get_name_from_token(name),
        _ => None,
    }
}

/// Tries getting the name (identifier) from the passed [`PrefixExp`]
#[inline]
fn get_name_from_prefix_exp(prefix_exp: &PrefixExp) -> Option<String> {
    match prefix_exp {
        PrefixExp::Var(var) => get_name_from_var(var),
        _ => None,
    }
}

/// Gets the [`BlockType`] depending on the passed [`Expression`].
fn get_block_type_from_expr(expression: &Expression, config: &Config) -> BlockType {
    match expression {
        Expression::FunctionCall(FunctionCall { invoked, .. }) => match invoked {
            FunctionCallInvoked::Function(prefix_exp)
                if config.sort_requires
                    && get_name_from_prefix_exp(prefix_exp)
                        .is_some_and(|name| name == "require") =>
            {
                BlockType::Require
            }
            FunctionCallInvoked::TableMethod { table, method, .. }
                if config.sort_services
                    && get_name_from_prefix_exp(table).is_some_and(|name| name == "game")
                    && get_name_from_token(method)
                        .is_some_and(|name| name == "GetService" || name == "getService") =>
            {
                BlockType::GetService
            }
            _ => BlockType::None,
        },

        Expression::Var(Var::TableAccess(TableAccess {
            prefix: TableAccessPrefix::Name(name),
            accessed_keys,
        })) if config.sort_services
            && get_name_from_token(name).is_some_and(|name| name == "require")
            && accessed_keys.len() == 1 =>
        {
            BlockType::GetService
        }

        Expression::ExpressionWrap(expression) => get_block_type_from_expr(expression, config),
        Expression::TypeCast { expression, .. } => get_block_type_from_expr(expression, config),
        _ => BlockType::None,
    }
}

/// Gets the [`BlockType`] for the passed [`Statement`].
pub fn get_block_type(statement: &Statement, config: &Config) -> BlockType {
    if !config.sort_requires && !config.sort_services {
        return BlockType::None;
    }

    match statement {
        Statement::LocalAssignment(local_assignment)
            if local_assignment.name_list.len() == 1 && local_assignment.expressions.len() == 1 =>
        {
            get_block_type_from_expr(local_assignment.expressions.first().unwrap(), config)
        }
        Statement::SetExpression(set_expression)
            if set_expression.variables.len() == 1 && set_expression.values.len() == 1 =>
        {
            get_block_type_from_expr(set_expression.values.first().unwrap(), config)
        }
        _ => BlockType::None,
    }
}
