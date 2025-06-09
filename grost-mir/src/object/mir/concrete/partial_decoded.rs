use quote::format_ident;
use syn::{Attribute, GenericParam, Generics, Ident, LifetimeParam, Type, TypeParam};

use super::{ConcreteField, ConcreteObjectAst};

#[derive(Debug, Clone)]
pub struct ConcretePartialDecodedObject {
  name: Ident,
  attrs: Vec<Attribute>,
  generics: Generics,
  copy: bool,
  flavor_type: Type,
  unknown_buffer_field_name: Ident,
  unknown_buffer_param: TypeParam,
  lifetime_param: LifetimeParam,
}

impl ConcretePartialDecodedObject {
  /// Returns the name of the partial decoded object.
  #[inline]
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  /// Returns the attributes of the partial decoded object.
  #[inline]
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns the generics of the partial decoded object.
  #[inline]
  pub const fn generics(&self) -> &Generics {
    &self.generics
  }

  /// Returns the flavor type of the partial decoded object.
  #[inline]
  pub const fn flavor_type(&self) -> &Type {
    &self.flavor_type
  }

  /// Returns the generic unknown buffer type parameter of the partial decoded object.
  #[inline]
  pub const fn unknown_buffer(&self) -> &TypeParam {
    &self.unknown_buffer_param
  }

  /// Returns the lifetime parameter of the partial decoded object.
  #[inline]
  pub const fn lifetime(&self) -> &LifetimeParam {
    &self.lifetime_param
  }

  /// Returns `true` if the partial decoded object is copyable, `false` otherwise.
  #[inline]
  pub const fn copy(&self) -> bool {
    self.copy
  }

  pub(super) fn from_ast<M, F>(
    object: &ConcreteObjectAst<M, F>,
    fields: &[ConcreteField<F>],
  ) -> darling::Result<Self> {
    let partial_decoded_object = object.partial_decoded();
    let unknown_buffer_param = partial_decoded_object.unknown_buffer().clone();
    let lifetime_param = partial_decoded_object.lifetime().clone();

    let object_generics = object.generics();
    let mut generics = Generics::default();

    for lt in object_generics.lifetimes() {
      generics.params.push(GenericParam::Lifetime(lt.clone()));
    }
    generics
      .params
      .push(GenericParam::Lifetime(lifetime_param.clone()));

    for tp in object_generics.type_params() {
      generics.params.push(GenericParam::Type(tp.clone()));
    }
    generics
      .params
      .push(GenericParam::Type(unknown_buffer_param.clone()));

    for const_param in object_generics.const_params() {
      generics
        .params
        .push(GenericParam::Const(const_param.clone()));
    }

    if let Some(ref wc) = object_generics.where_clause {
      if !wc.predicates.is_empty() {
        generics
          .make_where_clause()
          .predicates
          .extend(wc.predicates.iter().cloned());
      }
    }

    for field in fields.iter().filter_map(|f| f.try_unwrap_tagged_ref().ok()) {
      let type_constraints = field.partial_decoded().type_constraints();
      if type_constraints.is_empty() {
        generics
          .make_where_clause()
          .predicates
          .extend(type_constraints.iter().cloned());
      }
    }

    Ok(Self {
      name: partial_decoded_object.name().clone(),
      attrs: partial_decoded_object.attrs().to_vec(),
      copy: partial_decoded_object.copy(),
      generics,
      flavor_type: object.flavor().ty().clone(),
      unknown_buffer_param,
      unknown_buffer_field_name: format_ident!("__grost_unknown_buffer__"),
      lifetime_param,
    })
  }
}
