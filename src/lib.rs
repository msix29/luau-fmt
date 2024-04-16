//! # Luau Formatter
//!
//! This crate provides a formatter for luau using [`asts`](luau_parser::types::Ast)
//! provided by the [`luau_parser`](https://github.com/msix29/luau-parser) crate.
//!
//! # Usage:
//!
//! ```rust
//! use luau_formatter::formatter::format_luau;
//! use luau_parser::prelude::LuauParser;
//!
//! let mut parser = LuauParser::new();
//! let code = "";
//! let ast = parser.parse(code, "");
//! if let Ok(formatted_code) = format_luau(&ast) {}
//! ```
//!

#![forbid(unsafe_code)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::arc_with_non_send_sync)]
#![warn(missing_docs)]
#![warn(clippy::absolute_paths)]

/// 4 spaces, representing one tab.
pub(crate) const TAB: &str = "    ";

mod config;
pub mod formatter;
pub mod types;
