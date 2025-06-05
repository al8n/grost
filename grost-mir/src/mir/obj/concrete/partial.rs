use syn::{Attribute, GenericParam, Generics, Ident, Type, TypeParam, Visibility};

use quote::{format_ident, quote};

use super::{ConcreteField, ConcreteObjectAst};
use crate::ast::object::PartialObject;

#[derive(Debug, Clone)]
pub struct ConcretePartialObject {
  name: Ident,
  ty: Type,
  generics: Generics,
  attrs: Vec<Attribute>,
  unknown_buffer_param: TypeParam,
  unknown_buffer_field_name: Ident,
  copy: bool,
}

impl ConcretePartialObject {
  pub(super) fn from_ast(
    object: &ConcreteObjectAst,
    partial_object: &PartialObject,
    fields: &[ConcreteField],
  ) -> darling::Result<Self> {
    let unknown_buffer_param = partial_object.unknown_buffer().clone();

    let mut generics = object.generics().clone();
    generics
      .params
      .push(GenericParam::Type(unknown_buffer_param.clone()));

    fields
      .iter()
      .filter_map(|f| f.try_unwrap_tagged_ref().ok())
      .try_for_each(|f| {
        if !f.partial().type_constraints().is_empty() {
          generics
            .make_where_clause()
            .predicates
            .extend(f.partial().type_constraints().iter().cloned());
        }

        darling::Result::Ok(())
      })?;

    let (_, tg, _) = generics.split_for_impl();
    let name = partial_object.name().clone();
    let ty = syn::parse2(quote! {
      #name #tg
    })?;
    let copy = object.copy();

    Ok(Self {
      name,
      ty,
      generics,
      attrs: partial_object.attrs().to_vec(),
      unknown_buffer_param: partial_object.unknown_buffer().clone(),
      unknown_buffer_field_name: format_ident!("__grost_unknown_buffer__"),
      copy,
    })
  }
}
