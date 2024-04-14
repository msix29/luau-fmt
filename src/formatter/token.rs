use luau_parser::types::Statement;

use crate::types::Format;

impl Format for Statement {
    fn format(&self) -> String {
        match self {
            Statement::LocalAssignment(_) => todo!(),
            Statement::TypeDefinition(_) => todo!(),
            Statement::IfStatement(_) => todo!(),
            Statement::DoBlock(_) => todo!(),
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
