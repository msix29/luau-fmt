#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
// #![warn(clippy::missing_docs_in_private_items)]
// #![warn(clippy::arc_with_non_send_sync)]
// #![warn(missing_docs)]
#![warn(clippy::absolute_paths)]

pub use config::Config;
use luau_parser::types::Cst;
use std::{fs, io::Error as IoError, path::Path};
use toml::de::Error as TomlError;
use traits::Format;

mod config;
mod formatter;
mod traits;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
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
    } else if cst.block.is_empty() {
        Ok("".to_string())
    } else {
        Ok(cst.block.format(0, config))
    }
}

pub enum LoadConfigError {
    Io(IoError),
    Toml(TomlError),
}

impl From<IoError> for LoadConfigError {
    #[inline]
    fn from(value: IoError) -> Self {
        Self::Io(value)
    }
}
impl From<TomlError> for LoadConfigError {
    #[inline]
    fn from(value: TomlError) -> Self {
        Self::Toml(value)
    }
}

#[inline]
pub fn load_config<P: AsRef<Path>>(path: P) -> Result<Config, LoadConfigError> {
    fs::read_to_string(path)
        .map(|content| toml::from_str(&content))?
        .map_err(From::from)
}
