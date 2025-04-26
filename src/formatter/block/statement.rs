use luau_parser::types::{Statement, TerminationStatement};

use crate::{
    config::Config,
    traits::{Format, FormatWithArgs, Indentation},
};

impl Format for Statement {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        match self {
            Statement::ERROR => unreachable!(),
            Statement::LocalFunction(local_function) => local_function.format(indentation, config),
            Statement::LocalAssignment(local_assignment) => {
                local_assignment.format(indentation, config)
            }
            Statement::TypeDefinition(type_definition) => {
                type_definition.format(indentation, config)
            }
            Statement::IfStatement(if_statement) => if_statement.format(indentation, config),
            Statement::DoBlock(do_block) => do_block.format(indentation, config),
            Statement::GenericFor(generic_for) => generic_for.format(indentation, config),
            Statement::NumericalFor(numerical_for) => numerical_for.format(indentation, config),
            Statement::RepeatBlock(repeat_block) => repeat_block.format(indentation, config),
            Statement::WhileLoop(while_loop) => while_loop.format(indentation, config),
            Statement::SetExpression(set_expression) => set_expression.format(indentation, config),
            Statement::CompoundSetExpression(compound_set_expression) => {
                compound_set_expression.format(indentation, config)
            }
            Statement::FunctionCall(function_call) => function_call.format(indentation, config),
            Statement::GlobalFunction(global_function) => {
                global_function.format(indentation, config)
            }
            Statement::TypeFunction(type_function) => type_function.format(indentation, config),
            Statement::EndOfFile(end_of_file) => end_of_file.format(indentation, config),
        }
    }
}

impl Format for TerminationStatement {
    fn format(&self, indentation: Indentation, config: &Config) -> String {
        match self {
            TerminationStatement::Return {
                return_keyword,
                expressions: Some(expressions),
            } => {
                let string = expressions.format_with(indentation, config, ", ");

                if string.len() > config.column_width {
                    return_keyword.format(indentation, config)
                        + " "
                        + &expressions.format_with(
                            indentation,
                            config,
                            &(",".to_string()
                                + config.newline_style.as_str()
                                + &config.indent_style.to_string(indentation, config)),
                        )
                } else {
                    return_keyword.format(indentation, config) + " " + &string
                }
            }
            TerminationStatement::Break(token)
            | TerminationStatement::Continue(token)
            | TerminationStatement::Return {
                return_keyword: token,
                ..
            } => token.format(indentation, config),
        }
    }
}
