//! Holds all implementations for statements.

use luau_parser::types::Statement;

use crate::types::Format;

impl Format for Statement {
    fn format(&self, indentation: &mut i32) -> String {
        match self {
            Statement::LocalAssignment(local_assignment) => local_assignment.format(indentation),
            Statement::TypeDefinition(type_definition) => type_definition.format(indentation),
            Statement::IfStatement(if_statement) => if_statement.format(indentation),
            Statement::DoBlock(do_block) => do_block.format(indentation),
            Statement::GenericFor(generic_for) => generic_for.format(indentation),
            Statement::NumericalFor(numerical_for) => numerical_for.format(indentation),
            Statement::RepeatBlock(repeat_block) => repeat_block.format(indentation),
            Statement::WhileLoop(while_loop) => while_loop.format(indentation),
            Statement::SetExpression(set_expression) => set_expression.format(indentation),
            Statement::CompoundSetExpression(compound_set_expression) => {
                compound_set_expression.format(indentation)
            }
            Statement::FunctionCall(function_call) => function_call.format(indentation),
            Statement::LocalFunction(local_function) => local_function.format(indentation),
            Statement::GlobalFunction(global_function) => global_function.format(indentation),
        }
    }
}
