//! All `impl` blocks for [`Name`].

use luau_parser::types::Name;

use crate::{
    config::Config,
    traits::{Format, FormatWithArgs, Indentation},
};

use super::TokenFormatType;

impl Format for Name {
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
