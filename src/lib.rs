#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::arc_with_non_send_sync)]
#![warn(missing_docs)]
#![warn(clippy::absolute_paths)]

pub use config::*;
use luau_parser::types::Cst;
use std::{fs, io::Error as IoError, path::Path};
use toml::de::Error as TomlError;
use traits::Format;

mod config;
mod formatter;
mod traits;

/// An error that may happen during formatting.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum FormattingError {
    /// This [`Cst`] had a [`status`](luau_parser::types::AstStatus) of
    /// [`AstStatus::HasErrors`](luau_parser::types::AstStatus::HasErrors)
    ErroneousCst,
}

/// Formats the passed [`Cst`] with the default [`Config`].
#[inline]
pub fn format(cst: &Cst) -> Result<String, FormattingError> {
    format_with_config(cst, &Config::default())
}

/// Formats the passed [`Cst`] with the passed [`Config`].
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

/// Errors that may happen during loading of a [`Config`] from a `.toml` file.
pub enum LoadConfigError {
    /// An [`io error`](IoError).
    Io(IoError),

    /// An error parsing the TOML.
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

/// Load a [`Config`] from the passed path.
#[inline]
pub fn load_config<P: AsRef<Path>>(path: P) -> Result<Config, LoadConfigError> {
    fs::read_to_string(path)
        .map(|content| toml::from_str(&content))?
        .map_err(From::from)
}
