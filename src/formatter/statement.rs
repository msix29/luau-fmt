use luau_parser::types::Statement;

use crate::types::Format;

impl Format for Statement {
    fn format(&self, indentation: &mut i32) -> String {
        match self {
            Statement::LocalAssignment(local_assignment) => local_assignment.format(indentation),
            Statement::TypeDefinition(type_definition) => type_definition.format(indentation),
            Statement::IfStatement(_) => todo!(),
            Statement::DoBlock(do_block) => do_block.format(indentation),
            Statement::GenericFor(_) => todo!(),
            Statement::NumericalFor(_) => todo!(),
            Statement::RepeatBlock(_) => todo!(),
            Statement::WhileLoop(_) => todo!(),
            Statement::SetExpression(_) => todo!(),
            Statement::CompoundSetExpression(_) => todo!(),
            Statement::FunctionCall(_) => todo!(),
            Statement::LocalFunction(_) => todo!(),
            Statement::GlobalFunction(_) => todo!(),
            Statement::Comment(_) => todo!(),
        }
    }
}
