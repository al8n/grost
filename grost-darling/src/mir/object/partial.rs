use std::num::NonZeroU32;

use quote::quote;
use syn::{Attribute, Generics, Ident, Type, Visibility};

use crate::meta::object::{ObjectExt as _, TypeSpecification};

#[derive(Debug, Clone)]
pub struct PartialField {
  name: Ident,
  ty: Type,
  vis: Visibility,
  tag: NonZeroU32,
  specification: Option<TypeSpecification>,
  attrs: Vec<Attribute>,
  copy: bool,
}

impl PartialField {
  /// Returns the field name.
  #[inline]
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  /// Returns the field type.
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the field visibility.
  #[inline]
  pub const fn vis(&self) -> &Visibility {
    &self.vis
  }

  /// Returns the field tag.
  #[inline]
  pub const fn tag(&self) -> NonZeroU32 {
    self.tag
  }

  /// Returns the type specification.
  #[inline]
  pub const fn specification(&self) -> Option<&TypeSpecification> {
    self.specification.as_ref()
  }

  /// Returns whether the field is copyable.
  #[inline]
  pub const fn copy(&self) -> bool {
    self.copy
  }

  /// Returns the field attributes.
  #[inline]
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  pub(super) fn from_input<I>(input: &I) -> darling::Result<Self>
  where
    I: crate::meta::object::Field,
  {
    let meta = input.meta();
    let ty = input.ty();

    Ok(Self {
      name: input.name().clone(),
      ty: syn::parse2(quote! { ::core::option::Option<#ty> })?,
      vis: input.vis().clone(),
      tag: meta.tag(),
      specification: meta.type_specification().cloned(),
      attrs: meta.partial().attrs().to_vec(),
      copy: meta.copy(),
    })
  }
}

#[derive(Debug, Clone)]
pub struct PartialObject {
  path_to_grost: syn::Path,
  name: Ident,
  vis: Visibility,
  generics: Generics,
  fields: Vec<PartialField>,
  attrs: Vec<Attribute>,
  copy: bool,
}

impl PartialObject {
  /// Returns the name of the partial object.
  #[inline]
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  /// Returns the path to grost.
  #[inline]
  pub const fn path_to_grost(&self) -> &syn::Path {
    &self.path_to_grost
  }

  /// Returns the visibility of the partial object.
  #[inline]
  pub const fn vis(&self) -> &Visibility {
    &self.vis
  }

  /// Returns the generics of the partial object.
  #[inline]
  pub const fn generics(&self) -> &Generics {
    &self.generics
  }

  /// Returns the fields of the partial object.
  #[inline]
  pub fn fields(&self) -> &[PartialField] {
    self.fields.as_slice()
  }

  /// Returns the attributes of the partial object.
  #[inline]
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns whether the partial object is copyable.
  #[inline]
  pub const fn copy(&self) -> bool {
    self.copy
  }

  pub(super) fn from_input<O>(path_to_grost: &syn::Path, input: &O) -> darling::Result<Self>
  where
    O: crate::meta::object::Object,
  {
    let fields = input
      .fields()
      .into_iter()
      .map(PartialField::from_input)
      .collect::<Result<Vec<_>, darling::Error>>()?;
    let meta = input.meta();
    Ok(Self {
      path_to_grost: path_to_grost.clone(),
      name: input.partial_name(),
      vis: input.vis().clone(),
      generics: input.generics().clone(),
      fields,
      attrs: meta.partial().attrs().to_vec(),
      copy: meta.copy(),
    })
  }
}
