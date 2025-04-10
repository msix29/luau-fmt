//! All `impl` blocks for:
//!
//! * [`LocalFunction`]
//! * [`GlobalFunction`]
//! * [`GlobalFunctionName`]

use luau_parser::types::{GlobalFunction, GlobalFunctionName, LocalFunction};

use crate::{
    config::Config,
    traits::{Format, FormatWithArgs, Indentation},
};

impl Format for LocalFunction {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        let mut string = "local function ".to_string();
        string.push_str(&self.function_name.format(indentation, config));
        string.push_str(&self.parameters.format_with(indentation, config, " "));
        string.push_str(&self.colon.format(indentation, config));
        string.push(' ');
        string.push_str(&self.return_type.format(indentation, config));
        string.push_str(&self.body.format(indentation + 1, config));
        string.push_str("end");

        string
    }
}

impl Format for GlobalFunctionName {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        match self {
            GlobalFunctionName::SimpleName(token) => token.format(indentation, config),
            GlobalFunctionName::Table { table, keys, method } => {
                let mut string = table.format(indentation, config);

                for key in keys.iter() {
                    string.push_str(key.format(indentation, config));
                }

                if let Some(method) = method {
                    string.push(':');
                    string.push_str(&method.1.format(indentation, config));
                }

                string
            },
        }
    }
}

impl Format for GlobalFunction {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        let mut string = "function ".to_string();
        string.push_str(&self.function_name.format(indentation, config));
        string.push_str(&self.parameters.format_with(indentation, config, " "));
        string.push_str(&self.colon.format(indentation, config));
        string.push(' ');
        string.push_str(&self.return_type.format(indentation, config));
        string.push_str(&self.body.format(indentation + 1, config));
        string.push_str("end");

        string
    }
}
