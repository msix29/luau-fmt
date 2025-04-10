//! All `impl` blocks for table access-related types:
//!
//! * [`TableAccessPrefix`]
//! * [`TableAccessKey`]
//! * [`TableAccess`]

use luau_parser::types::{TableAccess, TableAccessKey, TableAccessPrefix};

use crate::{
    config::Config,
    formatter::TokenFormatType,
    traits::{Format, FormatWithArgs, Indentation},
};

impl Format for TableAccessPrefix {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        match self {
            TableAccessPrefix::Name(token) => token.format(indentation, config),
            TableAccessPrefix::FunctionCall(function_call) => {
                function_call.format(indentation, config)
            }
            TableAccessPrefix::ExpressionWrap(bracketed) => {
                bracketed.format_with_args(indentation, config, " ")
            }
        }
    }
}

impl Format for TableAccessKey {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        match self {
            TableAccessKey::Expression(table_key) => table_key.format(indentation, config),
            TableAccessKey::Name { name, .. } => {
                //FIXME: What if this is a method?
                ".".to_string() + &name.format_with_args(indentation, config, TokenFormatType::Name)
            }
        }
    }
}

impl Format for TableAccess {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        let mut string = self.prefix.format(indentation, config);

        for key in self.accessed_keys.iter() {
            string.push_str(&key.format(indentation, config));
        }

        string
    }
}
