use quote::{format_ident, quote};
use syn::{Expr, Type, WherePredicate, punctuated::Punctuated, token::Comma};

use crate::object::mir::{
  encoded_identifier_len_reflection, encoded_identifier_reflection, encoded_tag_len_reflection,
  encoded_tag_reflection, field_reflection, identifier_reflection, tag_reflection,
  wire_format_reflection, wire_type_reflection,
};

#[derive(Debug, Clone)]
pub struct ConcreteFieldReflection {
  field_reflection: Type,
  wire_format_reflection: Type,
  wire_type_reflection: Type,
  identifier_reflection: Type,
  encoded_identifier_reflection: Type,
  encoded_identifier_len_reflection: Type,
  tag_reflection: Type,
  encoded_tag_reflection: Type,
  encoded_tag_len_reflection: Type,
  value: Expr,
  constraints: Punctuated<WherePredicate, Comma>,
}

impl ConcreteFieldReflection {
  /// Returns the type of the field reflection.
  #[inline]
  pub const fn field_reflection(&self) -> &Type {
    &self.field_reflection
  }

  /// Returns the value of the field reflection.
  #[inline]
  pub const fn field_reflection_value(&self) -> &Expr {
    &self.value
  }

  /// Returns the type of the wire format reflection.
  #[inline]
  pub const fn wire_format_reflection(&self) -> &Type {
    &self.wire_format_reflection
  }

  /// Returns the type of the wire type reflection.
  #[inline]
  pub const fn wire_type_reflection(&self) -> &Type {
    &self.wire_type_reflection
  }

  /// Returns the type of the identifier reflection.
  #[inline]
  pub const fn identifier_reflection(&self) -> &Type {
    &self.identifier_reflection
  }

  /// Returns the type of the encoded identifier reflection.
  #[inline]
  pub const fn encoded_identifier_reflection(&self) -> &Type {
    &self.encoded_identifier_reflection
  }

  /// Returns the type of the encoded identifier length reflection.
  #[inline]
  pub const fn encoded_identifier_len_reflection(&self) -> &Type {
    &self.encoded_identifier_len_reflection
  }

  /// Returns the type of the tag reflection.
  #[inline]
  pub const fn tag_reflection(&self) -> &Type {
    &self.tag_reflection
  }

  /// Returns the type of the encoded tag reflection.
  #[inline]
  pub const fn encoded_tag_reflection(&self) -> &Type {
    &self.encoded_tag_reflection
  }

  /// Returns the type of the encoded tag length reflection.
  #[inline]
  pub const fn encoded_tag_len_reflection(&self) -> &Type {
    &self.encoded_tag_len_reflection
  }

  /// Returns the constraints for this field reflection.
  #[inline]
  pub const fn constraints(&self) -> &Punctuated<WherePredicate, Comma> {
    &self.constraints
  }

  pub(super) fn try_new<M, F>(
    object: &super::ConcreteObjectAst<M, F>,
    field: &super::ConcreteTaggedFieldAst<F>,
    flavor_ty: &Type,
    tag: u32,
  ) -> darling::Result<Self>
  where
    F: Clone,
  {
    let path_to_grost = object.path_to_grost();
    let object_type = object.ty();
    let mut constraints = Punctuated::new();
    let field_ty = field.ty();
    constraints.push(syn::parse2(quote! {
      #path_to_grost::__private::reflection::TypeReflection<#field_ty>: #path_to_grost::__private::reflection::Reflectable<#field_ty, Reflection = #path_to_grost::__private::reflection::Type>
    })?);
    let field_reflection = field_reflection(path_to_grost, object_type, flavor_ty, tag)?;
    let wire_format_reflection =
      wire_format_reflection(path_to_grost, object_type, flavor_ty, tag)?;
    let wire_type_reflection =
      wire_type_reflection(path_to_grost, object_type, flavor_ty, tag)?;
    let identifier_reflection =
      identifier_reflection(path_to_grost, object_type, flavor_ty, tag)?;
    let encoded_identifier_reflection =
      encoded_identifier_reflection(path_to_grost, object_type, flavor_ty, tag)?;
    let encoded_identifier_len_reflection =
      encoded_identifier_len_reflection(path_to_grost, object_type, flavor_ty, tag)?;
    let tag_reflection = tag_reflection(path_to_grost, object_type, flavor_ty, tag)?;
    let encoded_tag_reflection =
      encoded_tag_reflection(path_to_grost, object_type, flavor_ty, tag)?;
    let encoded_tag_len_reflection =
      encoded_tag_len_reflection(path_to_grost, object_type, flavor_ty, tag)?;

    let schema_name = field.schema_name();
    let schema_description = field.schema_description();

    Ok(Self {
      field_reflection,
      value: syn::parse2(quote! {
        #path_to_grost::__private::reflection::ObjectFieldBuilder {
          name: #schema_name,
          description: #schema_description,
          ty: <#path_to_grost::__private::reflection::TypeReflection<#field_ty> as #path_to_grost::__private::reflection::Reflectable<#field_ty>>::REFLECTION,
        }.build()
      })?,
      constraints,
      wire_format_reflection,
      wire_type_reflection,
      identifier_reflection,
      encoded_identifier_reflection,
      encoded_identifier_len_reflection,
      tag_reflection,
      encoded_tag_reflection,
      encoded_tag_len_reflection,
    })
  }
}
