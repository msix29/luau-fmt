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
            Statement::SetExpression(_) => todo!(),
            Statement::CompoundSetExpression(_) => todo!(),
            Statement::FunctionCall(_) => todo!(),
            Statement::LocalFunction(_) => todo!(),
            Statement::GlobalFunction(_) => todo!(),
            Statement::Comment(_) => todo!(),
        }
    }
}
