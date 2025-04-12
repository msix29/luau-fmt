//! All `impl` blocks for table access-related types:
//!
//! * [`TableAccessPrefix`]
//! * [`TableAccessKey`]
//! * [`TableAccess`]

use luau_parser::types::{TableAccess, TableAccessKey, TableAccessPrefix};

use crate::{
    config::Config,
    formatter::TokenFormatType,
    traits::{Expand, Format, FormatWithArgs, Indentation},
};

impl Format for TableAccessPrefix {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        match self {
            TableAccessPrefix::Name(token) => token.format(indentation, config),
            TableAccessPrefix::FunctionCall(function_call) => {
                function_call.format(indentation, config)
            }
            TableAccessPrefix::ExpressionWrap(bracketed) => bracketed.format(indentation, config),
        }
    }
}

impl Format for TableAccessKey {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        match self {
            TableAccessKey::Expression(table_key) => table_key.format(indentation, config),
            TableAccessKey::Name { name, .. } => {
                //FIXME: What if this is a method?
                ".".to_string() + &name.format_with(indentation, config, TokenFormatType::Name)
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

impl Expand for TableAccessPrefix {
    fn expand(&self, indentation: Indentation, config: &Config) -> String {
        match self {
            TableAccessPrefix::FunctionCall(function_call) => {
                function_call.expand(indentation, config)
            }
            _ => self.format(indentation, config),
        }
    }
}

impl Expand for TableAccess {
    fn expand(&self, indentation: Indentation, config: &Config) -> String {
        let mut string = {
            // We check if we have to expand the prefix or not. Sometimes only
            // the accesses is what needs folding.
            let string = self.prefix.format(indentation, config);

            if string.len() > config.column_width {
                self.prefix.expand(indentation, config)
            } else {
                string
            }
        };

        let spaces = config.newline_style.to_string()
            + &config.indent_style.to_string(indentation + 1, config);

        for key in self.accessed_keys.iter() {
            string.push_str(&spaces);
            string.push_str(&key.format(indentation, config));
        }

        string
    }
}
