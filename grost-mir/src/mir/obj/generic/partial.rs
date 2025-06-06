use syn::{Attribute, GenericParam, Generics, Ident, Type, TypeParam};

use quote::{format_ident, quote};

use crate::{ast::object::GenericObject as GenericObjectAst, obj::GenericField};

#[derive(Debug, Clone)]
pub struct GenericPartialObject {
  name: Ident,
  ty: Type,
  generics: Generics,
  attrs: Vec<Attribute>,
  unknown_buffer_param: TypeParam,
  unknown_buffer_field_name: Ident,
  copy: bool,
}

impl GenericPartialObject {
  /// Returns the name of the partial object
  #[inline]
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  /// Returns the attributes of the partial object
  #[inline]
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns the type of the partial object
  ///
  /// e.g. if the name is `PartialUserObject`, and the `unknown_buffer` returns the `UB`  this will return `PartialUserObject<UB>`
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the generics of the partial object.
  #[inline]
  pub const fn generics(&self) -> &Generics {
    &self.generics
  }

  /// Returns `true` if the partial object is copyable, `false` otherwise.
  #[inline]
  pub const fn copy(&self) -> bool {
    self.copy
  }

  /// Returns the generic unknown buffer type parameter of the partial object.
  #[inline]
  pub const fn unknown_buffer(&self) -> &TypeParam {
    &self.unknown_buffer_param
  }

  pub(super) fn from_ast(
    object: &GenericObjectAst,
    fields: &[GenericField],
  ) -> darling::Result<Self> {
    let partial_object = object.partial();
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
