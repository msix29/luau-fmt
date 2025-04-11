//! All `impl` blocks for CST-related types.

mod block;
mod bracketed;
mod expression;
mod list;
mod name;
mod value;

use luau_parser::{
    prelude::{Literal, LuauString, Token, TokenType, Trivia},
    types::{Pointer, Print},
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

/// Whether the formatted token is one of the passed ones.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenFormatType {
    /// A type name.
    Type,

    /// A method name.
    Method,

    /// A variable name.
    Name,

    /// Not even a name.
    #[default]
    None,
}

impl FormatWithArgs<TokenFormatType> for Token {
    fn format_with_args(
        &self,
        indentation: Indentation,
        config: &Config,
        token_format_type: TokenFormatType,
    ) -> String {
        let token_type = match &self.token_type {
            TokenType::Literal(Literal::String(luau_string)) => {
                luau_string.format(indentation, config)
            }
            TokenType::Identifier(identifier) => match token_format_type {
                TokenFormatType::Type => config.type_casing.apply(identifier),
                TokenFormatType::Method => config.method_casing.apply(identifier),
                TokenFormatType::Name => config.variable_casing.apply(identifier),
                TokenFormatType::None => unreachable!(),
            },

            // `unwrap` itself is safe and should never error as this will only be
            // be called by the library, which checks for the CST's correctness
            // before starting any of the formatting.
            token_type => token_type.try_as_string().unwrap_or_default(),
        };

        self.leading_trivia
            .iter()
            .fold("".to_string(), |str, trivia| match trivia {
                Trivia::Spaces(_) => str,
                Trivia::Comment(comment) => str + &comment.print(),
            })
            + &token_type
    }
}

impl Format for Token {
    #[inline]
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        self.format_with_args(indentation, config, TokenFormatType::None)
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
