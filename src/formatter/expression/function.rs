//! All `impl` blocks for function call-related types:
//!
//! * [`FunctionCallInvoked`]
//! * [`FunctionCall`]
//! * [`FunctionArguments`]
//! * [`FunctionArgument`]
//! * [`Closure`]

use luau_parser::types::{
    Closure, Expression, FunctionArgument, FunctionArguments, FunctionCall, FunctionCallInvoked,
};

use crate::{
    config::{Config, FunctionParenthesis},
    formatter::TokenFormatType,
    traits::{Expand, ExpandWithArgs, Format, FormatWithArgs, Indentation},
};

impl Format for FunctionCallInvoked {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        match self {
            Self::Function(prefix_exp) => prefix_exp.format(indentation, config),
            Self::TableMethod {
                table,
                colon,
                method,
            } => {
                let mut string = table.format(indentation, config);
                string.push_str(&colon.format(indentation, config));
                string.push_str(&method.format_with(indentation, config, TokenFormatType::Method));

                string
            }
        }
    }
}

impl Format for FunctionCall {
    #[inline]
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        self.invoked.format(indentation, config) + &self.arguments.format(indentation, config)
    }
}

/// Whether or not the passed [`FunctionArgument`] is a table or a string.
#[inline]
fn is_table_or_string(function_argument: &FunctionArgument) -> bool {
    matches!(
        function_argument,
        FunctionArgument::Expression(Expression::Table(_) | Expression::String(_))
    )
}

impl Format for FunctionArguments {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        let (is_string, is_table, string) = match self {
            Self::String(token) => (true, false, token.format(indentation, config)),
            Self::Table(table) => (false, true, table.format_with(indentation, config, false)),
            Self::List(bracketed)
                if config.function_parenthesis == FunctionParenthesis::RemoveWhenPossible
                    && bracketed.len() == 1
                    && is_table_or_string(&bracketed[0]) =>
            {
                (
                    // The booleans aren't used if this match arm is reached,
                    // so we don't have to worry about their values.
                    false,
                    false,
                    " ".to_string() + &bracketed.item.format_with(indentation, config, ", "),
                )
            }
            Self::List(bracketed) => (
                false,
                false,
                bracketed.format_with(indentation, config, ", "),
            ),
        };

        match config.function_parenthesis {
            FunctionParenthesis::Always if is_string || is_table => "(".to_string() + &string + ")",
            FunctionParenthesis::Keep if is_string || is_table => " ".to_string() + &string,
            FunctionParenthesis::RemoveForStrings if is_table => "(".to_string() + &string + ")",
            FunctionParenthesis::RemoveForTables if is_string => "(".to_string() + &string + ")",
            FunctionParenthesis::RemoveForStrings | FunctionParenthesis::RemoveForTables => {
                " ".to_string() + &string
            }
            FunctionParenthesis::RemoveWhenPossible if is_string || is_table => {
                " ".to_string() + &string
            }
            _ => string,
        }
    }
}

impl Format for FunctionArgument {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        match self {
            Self::Expression(expression) => expression.format(indentation, config),
            Self::VariadicValues(token) => token.format(indentation, config),
        }
    }
}

impl Format for Closure {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        format_function!(self, indentation, config,)
    }
}

impl Expand for FunctionCallInvoked {
    fn expand(&self, indentation: Indentation, config: &Config) -> String {
        match self {
            Self::Function(prefix_exp) => prefix_exp.expand(indentation, config),
            Self::TableMethod {
                table,
                colon,
                method,
            } => {
                let mut string = table.expand(indentation, config);
                string.push_str(
                    &(config.newline_style.to_string()
                        + &config.indent_style.to_string(indentation + 1, config)),
                );
                string.push_str(&colon.format(indentation, config));
                string.push_str(&method.format_with(indentation, config, TokenFormatType::Method));

                string
            }
        }
    }
}

impl Expand for FunctionCall {
    fn expand(&self, indentation: Indentation, config: &Config) -> String {
        let string = self.invoked.format(indentation, config);
        let arguments = self.arguments.format(indentation, config);

        if string.len() > config.column_width {
            self.invoked.expand(indentation, config) + &arguments
        } else if arguments.find('\n') != arguments.rfind('\n') {
            // This is most likely a closure, no need to expand.
            string + &arguments
        } else {
            string + &self.arguments.expand(indentation, config)
        }
    }
}

impl Expand for FunctionArguments {
    fn expand(&self, indentation: Indentation, config: &Config) -> String {
        match self {
            Self::List(bracketed) => bracketed.expand_with(
                indentation + 1,
                config,
                &(",".to_string()
                    + &config.newline_style.to_string()
                    + &config.indent_style.to_string(indentation + 1, config)),
            ),
            _ => self.format(indentation, config),
        }
    }
}
