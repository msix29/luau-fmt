//! All `impl` blocks for [`Comment`].

use luau_parser::types::Comment;

use crate::{
    config::Config,
    traits::{Format, Indentation},
};

impl Format for Comment {
    #[inline]
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        self.0.format(indentation, config)
    }
}
