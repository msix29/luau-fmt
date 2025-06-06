//! All `impl` blocks for:
//!
//! * [`LocalFunction`]
//! * [`GlobalFunction`]
//! * [`GlobalFunctionName`]
//! * [`Parameter`]
//! * [`TypeFunction`]

use luau_parser::types::{
    Attribute, GlobalFunction, GlobalFunctionName, LocalFunction, Parameter, TypeFunction,
};

use crate::{
    config::Config,
    formatter::TokenFormatType,
    traits::{Expand, ExpandWithArgs, Format, FormatWithArgs, Indentation},
};

impl Format for LocalFunction {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        format_function!(
            self,
            indentation,
            config,
            let keyword = local_keyword;
            let name = function_name;
        )
    }
}

impl Format for GlobalFunctionName {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        match self {
            GlobalFunctionName::SimpleName(token) => {
                token.format_with(indentation, config, TokenFormatType::Name)
            }
            GlobalFunctionName::Table {
                table,
                keys,
                method,
            } => {
                let mut string = table.format_with(indentation, config, TokenFormatType::Name);

                for key in keys.iter() {
                    string.push_str(&key.format(indentation, config));
                }

                if let Some(method) = method {
                    string.push_str(&method.0.format(indentation, config));
                    string.push_str(&method.1.format_with(
                        indentation,
                        config,
                        TokenFormatType::Method,
                    ));
                }

                string
            }
        }
    }
}

impl Format for GlobalFunction {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        format_function!(
            self,
            indentation,
            config,
            let name = function_name;
        )
    }
}

impl Format for Attribute {
    #[inline]
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        self.at.format(indentation, config)
            + &self
                .attribute
                .format_with(indentation, config, TokenFormatType::Name)
    }
}
impl Format for Vec<Attribute> {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        self.iter().fold(String::new(), |str, item| {
            str + &item.format(indentation, config) + config.newline_style.as_str()
        })
    }
}

impl Format for Parameter {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        if let Some(r#type) = self.r#type.as_ref() {
            let mut string = self
                .name
                .format_with(indentation, config, TokenFormatType::Name);
            string.push_str(&self.colon.format(indentation, config));
            string.push(' ');
            string.push_str(&r#type.format(indentation, config));

            string
        } else {
            self.name
                .format_with(indentation, config, TokenFormatType::Name)
        }
    }
}

impl Format for TypeFunction {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        format_function!(
            self,
            indentation,
            config,
            let export = export_keyword;
            let keyword = type_keyword;
            let name = function_name;
        )
    }
}
