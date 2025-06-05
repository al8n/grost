use crate::ast::object::GenericObject as GenericObjectAst;

#[derive(Debug, Clone)]
pub struct GenericObject {}

impl GenericObject {
  pub(super) fn from_ast(object: GenericObjectAst) -> darling::Result<Self> {
    todo!()
  }
}
