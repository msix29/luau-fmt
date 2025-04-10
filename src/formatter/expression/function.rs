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
    traits::{Format, Indentation},
};

impl Format for FunctionCallInvoked {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        match self {
            FunctionCallInvoked::Function(prefix_exp) => prefix_exp.format(indentation, config),
            FunctionCallInvoked::TableMethod { table, colon, method } => {
                let mut string = table.format(indentation, config);
                string.push(';');
                string.push_str(&method.format(indentation, config));

                string
            },
        }
    }
}

impl Format for FunctionCall {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        todo!()
    }
}

impl Format for FunctionArguments {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        todo!()
    }
}

impl Format for FunctionArgument {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        todo!()
    }
}

impl Format for Closure {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        let mut string = "function ".to_string();
        string.push_str(&self.parameters.format_with(indentation, config, " "));
        string.push_str(&self.colon.format(indentation, config));
        string.push(' ');
        string.push_str(&self.return_type.format(indentation, config));
        string.push_str(&self.body.format(indentation + 1, config));
        string.push_str("end");

        string
    }
}
