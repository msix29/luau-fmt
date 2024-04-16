use luau_parser::types::{
    GenericDeclaration, GenericDeclarationParameter, GenericParameterInfo,
    GenericParameterInfoDefault, TypeDefinition, TypeValue,
};

use crate::types::{Format, FormatWithArgs};

impl Format for TypeDefinition {
    fn format(&self, indentation: &mut i32) -> String {
        if let Some(type_keyword) = &self.type_keyword {
            let export = self
                .export_keyword
                .as_ref()
                .map_or_else(|| "".to_string(), |export| export.format(indentation));

            format!(
                "{} {} {}{} = {}",
                export,
                type_keyword.format(indentation),
                self.type_name.format(indentation),
                self.generics.format_with_args(indentation, ""),
                self.type_value.format(indentation)
            )
        } else {
            self.type_value.format(indentation)
        }
    }
}
impl Format for TypeValue {
    fn format(&self, indentation: &mut i32) -> String {
        match self {
            TypeValue::Basic(value) | TypeValue::String(value) | TypeValue::Boolean(value) => {
                value.format(indentation)
            }
            TypeValue::Wrap { r#type, .. } => format!("({})", r#type.format(indentation)),
            TypeValue::Function {
                generics,
                parameters,
                return_type,
                ..
            } => format!(
                "{}({}) -> {}",
                generics.format_with_args(indentation, ""),
                parameters.format_with_args(indentation, " "),
                return_type.format(indentation),
            ),
            TypeValue::Generic { base, generics, .. } => format!(
                "{}<{}>",
                base.format(indentation),
                generics.format_with_args(indentation, " ")
            ),
            TypeValue::GenericPack { name, .. } => format!("{}...", name.format(indentation)),
            TypeValue::Intersection { left, right, .. } => format!(
                "{} & {}",
                left.format(indentation),
                right.format(indentation)
            ),
            TypeValue::Union { left, right, .. } => format!(
                "{} | {}",
                left.format(indentation),
                right.format(indentation)
            ),
            TypeValue::Module {
                module, type_info, ..
            } => format!(
                "{}.{}",
                module.format(indentation),
                type_info.format(indentation)
            ),
            TypeValue::Optional { base, .. } => format!("{}?", base.format(indentation)),
            TypeValue::Table(table) => table.format_with_args(&mut (*indentation + 1), (": ", ",")),
            TypeValue::Typeof { inner, .. } => format!("typeof({})", inner.format(indentation)),
            TypeValue::Tuple { types, .. } => {
                format!("({})", types.format_with_args(indentation, " "))
            }
            TypeValue::Variadic { type_info, .. } => {
                format!("...{}", type_info.format(indentation))
            }
            TypeValue::VariadicPack { name, .. } => format!("...{}", name.format(indentation)),
        }
    }
}

impl FormatWithArgs<&str> for Option<GenericDeclaration> {
    fn format_with_args(&self, indentation: &mut i32, suffix: &str) -> String {
        self.as_ref().map_or(String::new(), |item| {
            item.format_with_args(indentation, suffix)
        })
    }
}
impl FormatWithArgs<&str> for GenericDeclaration {
    fn format_with_args(&self, indentation: &mut i32, suffix: &str) -> String {
        format!(
            "{}<{}>",
            suffix,
            self.generics.format_with_args(indentation, " ")
        )
    }
}

impl Format for GenericDeclarationParameter {
    fn format(&self, indentation: &mut i32) -> String {
        if let Some(default) = &self.default {
            format!(
                "{}{}",
                self.parameter.format(indentation),
                default.format(indentation)
            )
        } else {
            self.parameter.format(indentation)
        }
    }
}
impl Format for GenericParameterInfo {
    fn format(&self, indentation: &mut i32) -> String {
        match self {
            GenericParameterInfo::Name(name) => name.format(indentation),
            GenericParameterInfo::Pack { name, .. } => format!("{}...", name.format(indentation)),
        }
    }
}
impl Format for GenericParameterInfoDefault {
    fn format(&self, indentation: &mut i32) -> String {
        match self {
            GenericParameterInfoDefault::Name { name, .. } => {
                format!(" = {}", name.format(indentation))
            }
            GenericParameterInfoDefault::Pack { r#type, .. } => {
                format!(" = {}", r#type.format(indentation))
            }
        }
    }
}
