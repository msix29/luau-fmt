#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
// #![warn(clippy::missing_docs_in_private_items)]
// #![warn(clippy::arc_with_non_send_sync)]
// #![warn(missing_docs)]
#![warn(clippy::absolute_paths)]

use config::Config;
use luau_parser::types::Cst;
use traits::Format;

mod config;
mod formatter;
mod traits;

pub enum FormattingError {
    ErroneousCst,
}

#[inline]
pub fn format(cst: &Cst) -> Result<String, FormattingError> {
    format_with_config(cst, &Config::default())
}

#[inline]
pub fn format_with_config(cst: &Cst, config: &Config) -> Result<String, FormattingError> {
    if cst.has_errors() {
        Err(FormattingError::ErroneousCst)
    } else {
        Ok(cst.block.format(&mut 0, config))
    }
}
