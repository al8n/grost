use crate::ast::object::ConcretePartialDecodedObject as ConcretePartialDecodedObjectAst;

#[derive(Debug, Clone)]
pub struct ConcretePartialDecodedObject {}

impl ConcretePartialDecodedObject {
  pub(super) fn from_ast(object: ConcretePartialDecodedObjectAst) -> darling::Result<Self> {
    todo!()
  }
}
