use luau_parser::types::{
    GenericDeclaration, GenericDeclarationParameter, TypeDefinition, TypeValue,
};

use crate::types::{Format, FormatWithArgs};

impl Format for TypeDefinition {
    fn format(&self, indentation: &mut i32) -> String {
        todo!()
    }
}
impl Format for TypeValue {
    fn format(&self, indentation: &mut i32) -> String {
        todo!()
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
        todo!()
    }
}
