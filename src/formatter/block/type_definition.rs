//! All `impl` blocks for:
//!
//! * [`TypeValue`]
//! * [`ParameterTypeName`]
//! * [`TypeDefinition`]
//! * [`GenericParameterInfo`]
//! * [`GenericDeclarationParameter`]
//! * [`GenericParameterInfoDefault`]

use luau_parser::types::{
    GenericDeclarationParameter, GenericParameterInfo, GenericParameterInfoDefault,
    ParameterTypeName, TypeDefinition, TypeValue,
};

use crate::{
    config::Config,
    formatter::TokenFormatType,
    traits::{Format, FormatWithArgs, Indentation},
};

impl Format for TypeValue {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        match self {
            Self::ERROR => unreachable!(),
            Self::String(token) | Self::Boolean(token) | Self::Nil(token) => {
                token.format(indentation, config)
            }
            Self::Wrap(bracketed) => bracketed.format(indentation, config),
            Self::Function {
                generics,
                parameters,
                arrow,
                return_type,
            } => {
                let mut string = generics.format_with(indentation, config, ", ");
                string.push_str(&parameters.format_with(indentation, config, ", "));
                string.push(' ');
                string.push_str(&arrow.format(indentation, config));
                string.push(' ');
                string.push_str(&return_type.format(indentation, config));

                string
            }
            Self::Basic { base, generics } => {
                base.format_with(indentation, config, TokenFormatType::Type)
                    + &generics.format_with(indentation, config, ", ")
            }
            Self::GenericPack { name, ellipsis } => {
                name.format_with(indentation, config, TokenFormatType::Type)
                    + &ellipsis.format(indentation, config)
            }
            Self::Intersection {
                left,
                ampersand: token,
                right,
            }
            | Self::Union {
                left,
                pipe: token,
                right,
            } => {
                left.format(indentation, config)
                    + " "
                    + &token.format(indentation, config)
                    + " "
                    + &right.format(indentation, config)
            }
            Self::Module {
                module,
                dot,
                name,
                generics,
            } => {
                module.format_with(indentation, config, TokenFormatType::Name)
                    + &dot.format(indentation, config)
                    + &name.format_with(indentation, config, TokenFormatType::Type)
                    + &generics.format_with(indentation, config, ", ")
            }
            Self::Optional {
                base,
                question_mark,
            } => base.format(indentation, config) + &question_mark.format(indentation, config),
            Self::Table(table) => table.format_with(indentation, config, true),
            Self::Typeof {
                typeof_token,
                inner,
            } => typeof_token.format(indentation, config) + &inner.format(indentation, config),
            Self::Tuple(bracketed) => bracketed.format_with(indentation, config, ", "),
            Self::Variadic {
                ellipsis,
                type_value,
            } => ellipsis.format(indentation, config) + &type_value.format(indentation, config),
            Self::VariadicPack { ellipsis, name } => {
                ellipsis.format(indentation, config)
                    + &name.format_with(indentation, config, TokenFormatType::Type)
            }
        }
    }
}

impl Format for ParameterTypeName {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        match self {
            Self::Normal(name) => name.format(indentation, config),
            Self::Type(type_value) => type_value.format(indentation, config),
        }
    }
}

impl Format for TypeDefinition {
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
        string.push_str(
            &self
                .type_name
                .format_with(indentation, config, TokenFormatType::Type),
        );
        string.push_str(&self.generics.format_with(indentation, config, ""));
        string.push(' ');
        string.push_str(&self.equal_sign.format(indentation, config));
        string.push(' ');
        string.push_str(&self.type_value.format(indentation, config));

        string
    }
}

impl Format for GenericParameterInfo {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        match self {
            Self::ERROR => unreachable!(),
            Self::Name(token) => token.format_with(indentation, config, TokenFormatType::Type),
            Self::Pack { name, ellipsis } => {
                name.format_with(indentation, config, TokenFormatType::Type)
                    + &ellipsis.format(indentation, config)
            }
        }
    }
}

impl Format for GenericDeclarationParameter {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        if let Some(default) = self.default.as_ref() {
            self.parameter.format(indentation, config)
                + " "
                + &self.equal.format(indentation, config)
                + " "
                + &default.format(indentation, config)
        } else {
            self.parameter.format(indentation, config)
        }
    }
}

impl Format for GenericParameterInfoDefault {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        match self {
            Self::ERROR => unreachable!(),
            Self::Name(token) => token.format_with(indentation, config, TokenFormatType::Type),
            Self::Pack(type_value) => type_value.format(indentation, config),
        }
    }
}
