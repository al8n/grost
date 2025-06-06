use crate::ast::object::{
  ConcreteObject as ConcreteObjectAst, GenericObject as GenericObjectAst, Object as ObjectAst,
};

use syn::{ConstParam, parse_quote};

pub use concrete::*;
pub use generic::*;

mod concrete;
mod generic;

/// The Mid-level Intermediate Representation for objects in grost schema.
#[derive(Debug, Clone)]
pub enum Object {
  /// Represents a generic object with generic flavor types.
  Generic(GenericObject),
  /// Represents a concrete object with a specific flavor type.
  Concrete(ConcreteObject),
}

impl Object {
  /// Converts an AST representation of an object into its MIR representation.
  pub fn from_ast(object: ObjectAst) -> darling::Result<Self> {
    match object {
      ObjectAst::Concrete(concrete_object) => {
        ConcreteObject::from_ast(concrete_object).map(Self::Concrete)
      }
      ObjectAst::Generic(generic_object) => {
        GenericObject::from_ast(generic_object).map(Self::Generic)
      }
    }
  }

  /// Generates the final code for the object.
  pub fn derive(&self) -> darling::Result<proc_macro2::TokenStream> {
    match self {
      Self::Generic(generic) => generic.derive(),
      Self::Concrete(concrete) => concrete.derive(),
    }
  }
}

fn grost_selected_param() -> ConstParam {
  parse_quote!(
    const __GROST_SELECTED__: ::core::primitive::bool = true
  )
}
