//! All `impl` blocks for [`Var`].

use luau_parser::types::Var;

use crate::{
    config::Config,
    formatter::TokenFormatType,
    traits::{Expand, Format, FormatWithArgs, Indentation},
};

impl Format for Var {
    #[inline]
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        match self {
            Self::ERROR => unreachable!(),
            Self::Name(token) => token.format_with(indentation, config, TokenFormatType::Name),
            Self::TableAccess(table_access) => table_access.format(indentation, config),
        }
    }
}

impl Expand for Var {
    fn expand(&self, indentation: Indentation, config: &Config) -> String {
        match self {
            Self::TableAccess(table_access) => table_access.expand(indentation, config),
            _ => self.format(indentation, config),
        }
    }
}
