//! All `impl` blocks for:
//!
//! * [`TypeValue`]
//! * [`ParameterTypeName`]
//! * [`TypeDefinition`]
//! * [`GenericParameterInfo`]
//! * [`GenericDeclarationParameter`]
//! * [`GenericParameterInfoDefault`]

use luau_parser::types::{GenericDeclarationParameter, GenericParameterInfo, GenericParameterInfoDefault, ParameterTypeName, TypeDefinition, TypeValue};

use crate::{config::Config, formatter::TokenFormatType, traits::{Format, FormatWithArgs, Indentation}};

impl Format for TypeValue {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        todo!()
    }
}

impl Format for ParameterTypeName {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        todo!()
    }
}

impl Format for TypeDefinition {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        let mut string = if self.export_keyword.is_some() {
            "export type ".to_string()
        } else {
            "type ".to_string()
        };

        string.push_str(&self.type_name.format_with_args(indentation, config, TokenFormatType::Type));
        string.push_str(&self.generics.format_with_args(indentation, config, ""));
        string.push_str(" = ");
        string.push_str(&self.type_value.format(indentation, config));

        string
    }
}

impl Format for GenericParameterInfo {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        todo!()
    }
}

impl Format for GenericDeclarationParameter {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        todo!()
    }
}

impl Format for GenericParameterInfoDefault {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        todo!()
    }
}

