//! All `impl` blocks for the main statements in a [`Cst`](luau_parser::types::Cst).

mod do_block;
mod function;
mod generic_for;
mod get_block_type;
mod get_trailing_trivia;
mod if_statement;
mod local_assignment;
mod numerical_for;
mod repeat_block;
mod set_expressions;
mod statement;
mod type_definition;
mod while_loop;

use get_block_type::{BlockType, get_block_type, get_name_from_token, get_name_from_var};
use get_trailing_trivia::{
    get_trailing_trivia_expr, get_trailing_trivia_function_call, get_trailing_trivia_token,
    get_trailing_trivia_type,
};
use luau_parser::{
    prelude::{Block, Statement, TerminationStatement, Token, Trivia},
    types::{Pointer, Print},
};

use crate::{
    config::{Config, Semicolon},
    traits::{Format, FormatWithArgs, Indentation},
};

use super::trivia::TriviaFormattingType;

/// Get the trailing trivia of a [`Statement`].
#[inline]
fn get_trailing_trivia_statement(statement: &Statement) -> &[Trivia] {
    match statement {
        Statement::ERROR => unreachable!(),
        Statement::LocalFunction(local_function) => {
            get_trailing_trivia_token(&local_function.end_keyword)
        }
        Statement::LocalAssignment(local_assignment) => local_assignment
            .expressions
            .last()
            .map(|expression| get_trailing_trivia_expr(expression))
            .unwrap_or_else(|| {
                let name = local_assignment.name_list.last().unwrap();

                name.r#type
                    .as_ref()
                    .map(|type_value| get_trailing_trivia_type(type_value))
                    .unwrap_or_else(|| get_trailing_trivia_token(&name.name))
            }),
        Statement::TypeDefinition(type_definition) => {
            get_trailing_trivia_type(&type_definition.type_value)
        }
        Statement::IfStatement(if_statement) => {
            get_trailing_trivia_token(&if_statement.end_keyword)
        }
        Statement::DoBlock(do_block) => get_trailing_trivia_token(&do_block.end_keyword),
        Statement::GenericFor(generic_for) => {
            get_trailing_trivia_token(&generic_for.do_block.end_keyword)
        }
        Statement::NumericalFor(numerical_for) => {
            get_trailing_trivia_token(&numerical_for.do_block.end_keyword)
        }
        Statement::RepeatBlock(repeat_block) => get_trailing_trivia_expr(&repeat_block.condition),
        Statement::WhileLoop(while_loop) => {
            get_trailing_trivia_token(&while_loop.do_block.end_keyword)
        }
        Statement::SetExpression(set_expression) => {
            get_trailing_trivia_expr(set_expression.values.last().unwrap())
        }
        Statement::CompoundSetExpression(compound_set_expression) => {
            get_trailing_trivia_expr(&compound_set_expression.value)
        }
        Statement::FunctionCall(function_call) => get_trailing_trivia_function_call(function_call),
        Statement::GlobalFunction(global_function) => {
            get_trailing_trivia_token(&global_function.end_keyword)
        }
        Statement::TypeFunction(type_function) => {
            get_trailing_trivia_token(&type_function.end_keyword)
        }
        Statement::EndOfFile(token) => get_trailing_trivia_token(token),
    }
}

/// Get the trailing trivia of a [`TerminationStatement`].
#[inline]
fn get_trailing_trivia_last_statement(last_statement: &TerminationStatement) -> &[Trivia] {
    match last_statement {
        TerminationStatement::Return {
            expressions: Some(expressions),
            ..
        } => get_trailing_trivia_expr(expressions.last().unwrap()),
        TerminationStatement::Break(token)
        | TerminationStatement::Continue(token)
        | TerminationStatement::Return {
            return_keyword: token,
            ..
        } => get_trailing_trivia_token(token),
    }
}

/// Get the trailing comments of a [`Statement`] or a [`TerminationStatement`].
#[inline]
fn get_trailing_comments<T, F>(
    (statement, semicolon): &(T, Option<Token>),
    indentation: Indentation,
    config: &Config,
    get_trailing_trivia: F,
) -> String
where
    F: FnOnce(&T) -> &[Trivia],
{
    if let Some(semicolon) = semicolon {
        get_trailing_trivia_token(semicolon)
    } else {
        get_trailing_trivia(statement)
    }
    .format_with(indentation, config, TriviaFormattingType::CommentsOnly)
}

/// Handles the semicolon character that may or may not be after a [`Statement`] or
/// a [`TerminationStatement`].
fn handle_semicolon<F>(
    formatted_code: &mut String,
    semicolon: &Option<Token>,
    indentation: Indentation,
    config: &Config,
    get_trailing_spaces: F,
) -> String
where
    F: FnOnce() -> String,
{
    let spaces = semicolon
        .as_ref()
        .map(|semicolon| {
            get_trailing_trivia_token(semicolon).format_with(
                indentation,
                config,
                TriviaFormattingType::SpacesOnly,
            )
        })
        .unwrap_or_else(get_trailing_spaces);

    let new_lines = spaces.matches('\n').count();

    let spaces = if config.keep_statements_spacing {
        spaces
    } else if new_lines >= 2 {
        // Maximum of 2 new lines (1 empty line) if we
        // don't preserve user spacing.
        config.newline_style.to_string().repeat(2)
    } else {
        config.newline_style.to_string()
    };

    match config.semicolon {
        Semicolon::Keep => {
            formatted_code.push_str(&semicolon.format(indentation, config));
        }
        Semicolon::Always if semicolon.is_some() => {
            formatted_code.push_str(&semicolon.format(indentation, config));
        }
        Semicolon::Always => {
            formatted_code.push(';');
        }
        _ => (),
    }

    formatted_code.push_str(&spaces);

    spaces
}

/// Gets the name (identefier) from a [`Statement`]
fn get_name_from_statement(statement: &Statement) -> String {
    match statement {
        Statement::LocalAssignment(local_assignment) => {
            get_name_from_token(&local_assignment.name_list[0].name).unwrap()
        }
        Statement::SetExpression(set_expression) => {
            get_name_from_var(&set_expression.variables[0]).unwrap()
        }
        _ => unreachable!(),
    }
}

/// Arranges the passed statements in alphabetical order (by first variable name)
/// and appends them to `formatted_code`.
fn arrange_statements(
    formatted_code: &mut String,
    statements: &[(Pointer<Statement>, Option<Token>)],
    indentation: Indentation,
    config: &Config,
    spacing: &str,
) {
    let mut statements_sorted = statements.to_vec();

    statements_sorted
        .sort_by(|a, b| get_name_from_statement(&a.0).cmp(&get_name_from_statement(&b.0)));

    for (i, (statement, _)) in statements_sorted.iter().enumerate() {
        formatted_code.push_str(&statement.format(indentation, config));

        handle_semicolon(
            &mut *formatted_code,
            &statements[i].1,
            indentation,
            config,
            || {
                get_trailing_trivia_statement(&statements[i].0).format_with(
                    indentation,
                    config,
                    TriviaFormattingType::SpacesOnly,
                )
            },
        );

        formatted_code.push_str(spacing);
    }
}

impl Format for Block {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        let indentation_spacing = config.indent_style.to_string(indentation, config);

        if self.is_empty() {
            return if indentation == 0 {
                String::new()
            } else {
                config.newline_style.to_string()
                    + &config.indent_style.to_string(indentation - 1, config)
            };
        }

        let mut formatted_code = if indentation == 0 {
            String::new()
        } else {
            config.newline_style.to_string() + &config.indent_style.to_string(indentation, config)
        };

        let mut previous_block_type = BlockType::None;
        let mut block_start_index = 0;
        let last_index = self.statements.len() - 1;
        let mut last_spaces = String::new();
        let mut is_formatting = true;
        let mut single_statement_skip = false;

        for (i, (statement, semicolon)) in self.statements.iter().enumerate() {
            let mut semicolon_string = String::new();
            let mut should_format = true;
            let spaces = handle_semicolon(
                &mut semicolon_string,
                semicolon,
                indentation,
                config,
                || {
                    get_trailing_trivia_statement(statement).format_with(
                        indentation,
                        config,
                        TriviaFormattingType::SpacesOnly,
                    )
                },
            );

            let statement_string = statement.print_without_final_trivia();
            let trimmed_statement = statement_string.trim_start();
            let mut offset = statement_string.len() - trimmed_statement.len();

            for line in trimmed_statement.lines() {
                if !line.starts_with("--") {
                    offset += line.len();
                    continue;
                }

                let trimmed = line.trim();
                if trimmed == "--@luau-fmt skip" {
                    single_statement_skip = true;
                } else if trimmed == "--@luau-fmt skip-start" {
                    should_format = false;
                    is_formatting = false;
                } else if trimmed == "--@luau-fmt skip-end" {
                    is_formatting = true;

                    if i > 0 {
                        let leading_trivia = if let Some(semicolon) = &self.statements[i - 1].1 {
                            semicolon.print_final_trivia()
                        } else {
                            self.statements[i - 1].0.print_final_trivia()
                        };

                        formatted_code.push_str(&leading_trivia[..offset]);
                    }
                    offset += line.len();
                }
            }

            let block_type = if single_statement_skip {
                BlockType::None
            } else {
                get_block_type(statement, config)
            };
            if block_type != previous_block_type
                || (last_spaces.find('\n') != last_spaces.rfind('\n')
                    && matches!(
                        previous_block_type,
                        BlockType::GetService | BlockType::Require
                    ))
            {
                last_spaces = spaces;
                match previous_block_type {
                    BlockType::GetService | BlockType::Require => arrange_statements(
                        &mut formatted_code,
                        &self.statements[block_start_index..i],
                        indentation,
                        config,
                        &indentation_spacing,
                    ),
                    BlockType::None => {}
                }

                previous_block_type = block_type;

                match block_type {
                    BlockType::Require | BlockType::GetService => {
                        if i == last_index {
                            arrange_statements(
                                &mut formatted_code,
                                &self.statements[block_start_index..],
                                indentation,
                                config,
                                &indentation_spacing,
                            )
                        } else {
                            block_start_index = i;
                            continue;
                        }
                    }
                    _ => (),
                }

                block_start_index = i;
            } else if block_type == BlockType::GetService || block_type == BlockType::Require {
                last_spaces = spaces;
                continue;
            }

            if single_statement_skip {
                formatted_code = formatted_code.trim_end().to_string();
                formatted_code.push_str(&statement_string);
                formatted_code.push_str(&semicolon_string);

                single_statement_skip = false;

                continue;
            }

            if !is_formatting || !should_format {
                if !should_format {
                    if single_statement_skip {
                        handle_semicolon(
                            &mut formatted_code,
                            &self.statements[i - 1].1,
                            indentation,
                            config,
                            || {
                                get_trailing_trivia_statement(&self.statements[i - 1].0)
                                    .format_with(
                                        indentation,
                                        config,
                                        TriviaFormattingType::SpacesOnly,
                                    )
                            },
                        );
                    }

                    formatted_code.push_str(trimmed_statement);
                } else {
                    formatted_code.push_str(&statement_string);
                }

                formatted_code.push_str(&semicolon.print_without_final_trivia());

                continue;
            }

            formatted_code.push_str(&statement.format(indentation, config));
            formatted_code.push_str(&semicolon_string);
            formatted_code.push_str(&indentation_spacing);
        }

        if is_formatting {
            if let Some(last_statement) = &self.last_statement {
                formatted_code.push_str(&last_statement.0.format(indentation, config));

                handle_semicolon(
                    &mut formatted_code,
                    &last_statement.1,
                    indentation,
                    config,
                    || {
                        get_trailing_trivia_last_statement(&last_statement.0).format_with(
                            indentation,
                            config,
                            TriviaFormattingType::SpacesOnly,
                        )
                    },
                );

                formatted_code.push_str(&indentation_spacing);
                formatted_code.push_str(&get_trailing_comments(
                    last_statement,
                    indentation,
                    config,
                    |last_statement| get_trailing_trivia_last_statement(last_statement),
                ))
            } else {
                formatted_code.push_str(&get_trailing_comments(
                    self.statements.last().unwrap(),
                    indentation,
                    config,
                    |statement| get_trailing_trivia_statement(statement),
                ))
            }
        }

        formatted_code = formatted_code.trim_end().to_string();

        if indentation == 0 {
            if config.add_final_newline {
                formatted_code.push_str(config.newline_style.as_str());
            }
        } else {
            formatted_code.push_str(config.newline_style.as_str());
            formatted_code.push_str(&config.indent_style.to_string(indentation - 1, config));
        }

        formatted_code
    }
}
