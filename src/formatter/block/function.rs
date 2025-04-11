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
    traits::{Format, FormatWithArgs, Indentation},
};

impl Format for LocalFunction {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        let mut string = self.attributes.format(indentation, config);
        string.push_str(&self.local_keyword.format(indentation, config));
        string.push(' ');
        string.push_str(&self.function_keyword.format(indentation, config));
        string.push(' ');
        string.push_str(&self.function_name.format(indentation, config));
        string.push_str(&self.generics.format_with_args(indentation, config, ", "));
        string.push_str(&self.parameters.format_with_args(indentation, config, ", "));
        string.push_str(&self.colon.format(indentation, config));
        string.push(' ');
        string.push_str(&self.return_type.format(indentation, config));
        string.push_str(&self.body.format(indentation + 1, config));
        string.push_str(&self.end_keyword.format(indentation, config));

        string
    }
}

impl Format for GlobalFunctionName {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        match self {
            GlobalFunctionName::SimpleName(token) => token.format(indentation, config),
            GlobalFunctionName::Table {
                table,
                keys,
                method,
            } => {
                let mut string = table.format(indentation, config);

                for key in keys.iter() {
                    string.push_str(&key.format(indentation, config));
                }

                if let Some(method) = method {
                    string.push_str(&method.0.format(indentation, config));
                    string.push_str(&method.1.format_with_args(
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
        let mut string = self.attributes.format(indentation, config);
        string.push_str(&self.function_keyword.format(indentation, config));
        string.push(' ');
        string.push_str(&self.function_name.format(indentation, config));
        string.push_str(&self.generics.format_with_args(indentation, config, ", "));
        string.push_str(&self.parameters.format_with_args(indentation, config, ", "));
        string.push_str(&self.colon.format(indentation, config));
        string.push(' ');
        string.push_str(&self.return_type.format(indentation, config));
        string.push_str(&self.body.format(indentation + 1, config));
        string.push_str(&self.end_keyword.format(indentation, config));

        string
    }
}

impl Format for Attribute {
    #[inline]
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        "@".to_string()
            + &self
                .attribute
                .format_with_args(indentation, config, TokenFormatType::Name)
    }
}
impl Format for Vec<Attribute> {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        self.iter().fold("".to_string(), |str, item| {
            str + &item.format(indentation, config) + "\n"
        })
    }
}

impl Format for Parameter {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        if let Some(r#type) = self.r#type.as_ref() {
            let mut string = self
                .name
                .format_with_args(indentation, config, TokenFormatType::Name);
            string.push_str(&self.colon.format(indentation, config));
            string.push_str(&r#type.format(indentation, config));

            string
        } else {
            self.name
                .format_with_args(indentation, config, TokenFormatType::Name)
        }
    }
}

impl Format for TypeFunction {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        let mut string = if self.export_keyword.is_some() {
            let mut string = self.export_keyword.format(indentation, config);
            string.push(' ');
            string
        } else {
            "".to_string()
        };

        string.push_str(&self.type_keyword.format(indentation, config));
        string.push(' ');
        string.push_str(&self.function_keyword.format(indentation, config));
        string.push(' ');
        string.push_str(&self.function_name.format(indentation, config));
        string.push_str(&self.generics.format_with_args(indentation, config, ", "));
        string.push_str(&self.parameters.format_with_args(indentation, config, ", "));
        string.push_str(&self.colon.format(indentation, config));
        string.push(' ');
        string.push_str(&self.return_type.format(indentation, config));
        string.push_str(&self.body.format(indentation + 1, config));
        string.push_str(&self.end_keyword.format(indentation, config));

        string
    }
}
