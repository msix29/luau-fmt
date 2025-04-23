//! All `impl` blocks for function call-related types:
//!
//! * [`FunctionCallInvoked`]
//! * [`FunctionCall`]
//! * [`FunctionArguments`]
//! * [`FunctionArgument`]
//! * [`Closure`]

use luau_parser::types::{
    Closure, FunctionArgument, FunctionArguments, FunctionCall, FunctionCallInvoked,
};

use crate::{
    config::Config,
    formatter::TokenFormatType,
    traits::{Expand, ExpandWithArgs, Format, FormatWithArgs, Indentation},
};

impl Format for FunctionCallInvoked {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        match self {
            FunctionCallInvoked::Function(prefix_exp) => prefix_exp.format(indentation, config),
            FunctionCallInvoked::TableMethod {
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

impl Format for FunctionArguments {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        match self {
            FunctionArguments::String(token) => {
                " ".to_string() + &token.format(indentation, config)
            }
            FunctionArguments::Table(table) => {
                " ".to_string() + &table.format_with(indentation, config, false)
            }
            FunctionArguments::List(bracketed) => bracketed.format_with(indentation, config, ", "),
        }
    }
}

impl Format for FunctionArgument {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        match self {
            FunctionArgument::Expression(expression) => expression.format(indentation, config),
            FunctionArgument::VariadicValues(token) => token.format(indentation, config),
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
            FunctionCallInvoked::Function(prefix_exp) => prefix_exp.expand(indentation, config),
            FunctionCallInvoked::TableMethod {
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
        self.invoked.expand(indentation, config) + &self.arguments.expand(indentation, config)
    }
}

impl Expand for FunctionArguments {
    fn expand(&self, indentation: Indentation, config: &Config) -> String {
        match self {
            FunctionArguments::List(bracketed) => bracketed.expand_with(
                indentation,
                config,
                &(",".to_string()
                    + &config.newline_style.to_string()
                    + &config.indent_style.to_string(indentation + 1, config)),
            ),
            _ => self.format(indentation, config),
        }
    }
}
