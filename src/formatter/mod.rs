//! All `impl` blocks for CST-related types.

#[macro_use]
mod macros;

mod block;
mod bracketed;
mod expression;
mod list;
mod name;
mod trivia;
mod value;

use luau_parser::prelude::{Literal, LuauString, Pointer, Token, TokenType};
use trivia::TriviaFormattingType;

use crate::{
    config::{Config, NamingConvention},
    traits::{Expand, Format, FormatWithArgs, Indentation},
};

impl Format for LuauString {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        let string = config.quote_style.apply(self);

        if string.len() > config.string_width {
            let separator = r"\z".to_string()
                + &config.newline_style.to_string()
                + &config.indent_style.to_string(indentation + 1, config);

            let words = string.split_whitespace().collect::<Vec<&str>>();
            let mut current_line = String::new();
            let mut result = String::new();

            for word in words {
                if current_line.len() + word.len() + 1 > config.string_width {
                    if !current_line.is_empty() {
                        result.push_str(&current_line);
                        result.push_str(&separator);
                    }

                    current_line = word.to_string();
                } else {
                    current_line.push_str(word);
                }

                current_line.push(' ');
            }

            if !current_line.is_empty() {
                result.push_str(&current_line);
            }

            result.pop(); // remove the final space
            result
        } else {
            config.quote_style.apply(self)
        }
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
    fn format_with(
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
                TokenFormatType::None => NamingConvention::None.apply(identifier),
            },

            // `unwrap` itself is safe and should never error as this will only be
            // be called by the library, which checks for the CST's correctness
            // before starting any of the formatting.
            token_type => token_type.try_as_string().unwrap_or_default(),
        };

        self.leading_trivia
            .format_with(indentation, config, TriviaFormattingType::CommentsOnly)
            + &token_type
    }
}

impl Format for Token {
    #[inline]
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        self.format_with(indentation, config, TokenFormatType::None)
    }
}

impl<T: Format> Format for Option<T> {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        match self {
            Some(item) => item.format(indentation, config),
            None => String::new(),
        }
    }
}
impl<A, T: FormatWithArgs<A>> FormatWithArgs<A> for Option<T> {
    fn format_with(&self, indentation: Indentation, config: &Config, args: A) -> String {
        match self {
            Some(item) => item.format_with(indentation, config, args),
            None => String::new(),
        }
    }
}

impl<T: Expand> Expand for Option<T> {
    fn expand(&self, indentation: Indentation, config: &Config) -> String {
        match self {
            Some(item) => item.expand(indentation, config),
            None => String::new(),
        }
    }
}

impl<T: Format> Format for Pointer<T> {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        (**self).format(indentation, config)
    }
}
impl<A, T: FormatWithArgs<A>> FormatWithArgs<A> for Pointer<T> {
    fn format_with(&self, indentation: Indentation, config: &Config, args: A) -> String {
        (**self).format_with(indentation, config, args)
    }
}

impl<T: Expand> Expand for Pointer<T> {
    fn expand(&self, indentation: Indentation, config: &Config) -> String {
        (**self).expand(indentation, config)
    }
}
