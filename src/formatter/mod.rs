//! All `impl` blocks for CST-related types.

mod block;
mod bracketed;
mod expression;
mod list;
mod name;
mod value;

use luau_parser::{
    prelude::{Literal, LuauString, Token, TokenType},
    types::Pointer,
};

use crate::{
    config::Config,
    traits::{Format, FormatWithArgs, Indentation},
};

impl Format for LuauString {
    fn format(&self, _indentation: Indentation, config: &Config) -> String {
        //TODO: Length check
        config.quote_style.apply(self)
    }
}

impl FormatWithArgs<bool> for Token {
    fn format_with_args(&self, indentation: Indentation, config: &Config, is_method_name: bool) -> String {
        match &self.token_type {
            TokenType::Literal(Literal::String(luau_string)) => {
                luau_string.format(indentation, config)
            }
            TokenType::Identifier(identifier) => {
                if is_method_name {
                    config.method_casing.apply(&identifier)
                } else {
                    config.variable_casing.apply(&identifier)
                }
            }

            // `unwrap` itself is safe and should never error as this will only be
            // be called by the library, which checks for the CST's correctness
            // before starting any of the formatting.
            token_type => token_type.try_as_string().unwrap_or_default(),
        }
    }
}

impl Format for Token {
    #[inline]
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        self.format_with_args(indentation, config, false)
    }
}

impl<T: Format> Format for Option<T> {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        match self {
            Some(item) => item.format(indentation, config),
            None => "".to_string(),
        }
    }
}
impl<A, T: FormatWithArgs<A>> FormatWithArgs<A> for Option<T> {
    fn format_with_args(&self, indentation: Indentation, config: &Config, args: A) -> String {
        match self {
            Some(item) => item.format_with_args(indentation, config, args),
            None => "".to_string(),
        }
    }
}

impl<T: Format> Format for Pointer<T> {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        (**self).format(indentation, config)
    }
}
impl<A, T: FormatWithArgs<A>> FormatWithArgs<A> for Pointer<T> {
    fn format_with_args(&self, indentation: Indentation, config: &Config, args: A) -> String {
        (**self).format_with_args(indentation, config, args)
    }
}
