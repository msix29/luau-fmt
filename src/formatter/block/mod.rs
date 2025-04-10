//! All `impl` blocks for the main statements in a [`Cst`](luau_parser::types::Cst).

mod comment;
mod do_block;
mod function;
mod generic_for;
mod if_statement;
mod local_assignment;
mod numerical_for;
mod repeat_block;
mod set_expressions;
mod statement;
mod type_definition;
mod while_loop;

use luau_parser::prelude::Block;

use crate::{
    config::{Config, Semicolon},
    traits::{Format, Indentation},
};

impl Format for Block {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        let mut formatted_code =
            config.newline_style.to_string() + &config.indent_style.to_string(indentation, config);

        if self.is_empty() {
            // We add a newline with the correct indentation if this isn't the
            // main block.
            if indentation != 0 {
                return formatted_code;
            } else {
                return "".to_string();
            }
        }

        for (statement, semicolon) in self.statements.iter() {
            formatted_code.push_str(&statement.format(indentation, config));

            let trimmed = formatted_code.trim_end();
            let spaces = formatted_code[trimmed.len()..].to_string();

            let new_lines = spaces.matches('\n').count();

            match config.semicolon {
                Semicolon::Keep => {
                    formatted_code.push_str(&semicolon.format(indentation, config));
                }
                Semicolon::Always if semicolon.is_some() => {
                    formatted_code.push_str(&semicolon.format(indentation, config));
                }
                Semicolon::Always => {
                    let ending_spaces = if config.keep_statements_spacing {
                        formatted_code = trimmed.to_string();

                        spaces
                    } else {
                        formatted_code = trimmed.to_string();

                        if new_lines > 2 {
                            // Maximum of 2 new lines (1 empty line) if we
                            // don't preserve user spacing.
                            config.newline_style.to_string().repeat(2)
                        } else {
                            config.newline_style.to_string()
                        }
                    };

                    formatted_code.push(';');
                    formatted_code.push_str(&ending_spaces);
                }
                _ => (),
            }
        }

        if config.add_final_newline {
            formatted_code.push_str(config.newline_style.as_str());
        }

        formatted_code
    }
}
