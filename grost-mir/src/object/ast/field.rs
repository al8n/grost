use darling::usage::{IdentSet, LifetimeSet, Purpose, UsesLifetimes, UsesTypeParams};
use quote::quote;
use std::num::NonZeroU32;
use syn::{Attribute, Ident, Type, Visibility};

pub(in crate::object) use concrete::{ConcreteField, ConcreteTaggedField};
pub use convert::ConvertAttribute;
pub use flavor::{
  FieldDecodeAttribute, FieldDecodeFlavor, FieldEncodeAttribute, FieldEncodeFlavor, FieldFlavor,
  FieldFlavorAttribute,
};
pub(in crate::object) use generic::{GenericField, GenericTaggedField};
pub use partial::PartialFieldAttribute;
pub use partial_decoded::PartialDecodedFieldAttribute;
pub use selector::SelectorFieldAttribute;

use crate::{
  object::meta::{FieldFromMeta, Label},
  utils::{Invokable, SchemaAttribute},
};

mod concrete;
mod convert;
mod flavor;
mod generic;
mod partial;
mod partial_decoded;
mod selector;

#[derive(Debug, Clone)]
pub struct SkippedField<M = ()> {
  attrs: Vec<Attribute>,
  vis: Visibility,
  name: Ident,
  ty: Type,
  default: Invokable,
  type_params_usages: IdentSet,
  lifetime_params_usages: LifetimeSet,
  meta: M,
}

impl<M> SkippedField<M> {
  /// Returns the name of the skipped field
  #[inline]
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  /// Returns the visibility of the skipped field
  #[inline]
  pub const fn vis(&self) -> &Visibility {
    &self.vis
  }

  /// Returns the type of the skipped field
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the attributes of the skipped field
  #[inline]
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns the path to the default value function for the skipped field
  #[inline]
  pub const fn default(&self) -> &Invokable {
    &self.default
  }

  /// Returns the metadata associated with the skipped field
  #[inline]
  pub const fn meta(&self) -> &M {
    &self.meta
  }

  /// Returns the type parameters used in the skipped field
  #[inline]
  pub const fn type_params_usages(&self) -> &IdentSet {
    &self.type_params_usages
  }

  /// Returns the lifetime parameters used in the skipped field
  #[inline]
  pub const fn lifetime_params_usages(&self) -> &LifetimeSet {
    &self.lifetime_params_usages
  }

  pub(in crate::object) fn try_new<F: RawField<Meta = M>>(
    f: &F,
    type_params: &IdentSet,
    lifetime_params: &LifetimeSet,
  ) -> darling::Result<Self>
  where
    M: Clone,
  {
    let attrs = f.attrs().to_vec();
    let vis = f.vis().clone();
    let name = f.name().clone();
    let ty = f.ty().clone();
    let default = match f.default().cloned() {
      Some(path) => path,
      None => {
        syn::parse2::<syn::Path>(quote! { ::core::default::Default::default }).map(Into::into)?
      }
    };

    let purpose: darling::usage::Options = Purpose::Declare.into();
    let type_params_usages = ty.uses_type_params_cloned(&purpose, type_params);
    let lifetime_params_usages = ty.uses_lifetimes_cloned(&purpose, lifetime_params);

    Ok(Self {
      attrs,
      vis,
      name,
      type_params_usages,
      lifetime_params_usages,
      ty,
      default,
      meta: f.meta().clone(),
    })
  }
}

#[derive(Debug, Clone)]
pub struct FieldAttribute {
  convert: ConvertAttribute,
  default: Option<Invokable>,
  schema: SchemaAttribute,
  tag: Option<NonZeroU32>,
  label: Option<Label>,
  flavor: Vec<FieldFlavorAttribute>,
  partial: PartialFieldAttribute,
  partial_decoded: PartialDecodedFieldAttribute,
  selector: SelectorFieldAttribute,
  copy: bool,
  skip: bool,
}

impl FieldAttribute {
  /// Returns the tag of the field
  pub const fn tag(&self) -> Option<NonZeroU32> {
    self.tag
  }

  /// Returns the flavor specified settings for the field
  pub const fn flavors(&self) -> &[FieldFlavorAttribute] {
    self.flavor.as_slice()
  }

  /// Returns the convert attribute for the field
  pub const fn convert(&self) -> &ConvertAttribute {
    &self.convert
  }

  /// Returns the information about the partial object field
  pub const fn partial(&self) -> &PartialFieldAttribute {
    &self.partial
  }

  /// Returns the information about the partial reference object field
  pub const fn partial_decoded(&self) -> &PartialDecodedFieldAttribute {
    &self.partial_decoded
  }

  /// Returns whether the field type is copyable
  pub const fn copy(&self) -> bool {
    self.copy
  }

  /// Returns whether the field should be skipped
  pub const fn skip(&self) -> bool {
    self.skip
  }

  /// Returns the default selection for the field
  pub const fn selector(&self) -> &SelectorFieldAttribute {
    &self.selector
  }

  /// Returns the type label of the field
  pub const fn label(&self) -> Option<&Label> {
    self.label.as_ref()
  }

  /// Returns the schema information of the field
  pub const fn schema(&self) -> &SchemaAttribute {
    &self.schema
  }

  /// Returns the fn which will be used to generate the default value for the field
  pub const fn default(&self) -> Option<&Invokable> {
    self.default.as_ref()
  }
}

impl FieldFromMeta {
  pub fn finalize(self) -> darling::Result<FieldAttribute> {
    Ok(FieldAttribute {
      default: self.default,
      schema: self.schema.into(),
      tag: self.tag,
      label: self.label,
      skip: self.skip,
      convert: self.convert.finalize()?,
      flavor: self.flavor.finalize()?,
      partial: self.partial.finalize(),
      partial_decoded: self.partial_decoded.finalize()?,
      selector: self.selector.finalize(),
      copy: self.copy,
    })
  }
}

/// The trait for the field derive input
pub trait RawField: Clone {
  /// The custom metadata type for the field
  type Meta: Clone;

  /// Returns the name of the field
  fn name(&self) -> &Ident;

  /// Returns the type of the field
  fn ty(&self) -> &Type;

  /// Returns the visibility of the field
  fn vis(&self) -> &Visibility;

  /// Returns the attributes of the field
  fn attrs(&self) -> &[Attribute];

  /// Returns the tag of the field
  fn tag(&self) -> Option<NonZeroU32>;

  /// Returns the convert attribute for the field
  fn convert(&self) -> &ConvertAttribute;

  /// Returns the information about the partial object field
  fn partial(&self) -> &PartialFieldAttribute;

  /// Returns the information about the partial reference object field
  fn partial_decoded(&self) -> &PartialDecodedFieldAttribute;

  /// Returns whether the field type is copyable
  fn copy(&self) -> bool;

  /// Returns whether the field should be skipped
  fn skip(&self) -> bool;

  /// Returns the default selection for the field
  fn selector(&self) -> &SelectorFieldAttribute;

  /// Returns the type label of the field
  fn label(&self) -> Option<&Label>;

  /// Returns the schema information of the field
  fn schema(&self) -> &SchemaAttribute;

  /// Returns the fn which will be used to generate the default value for the field
  fn default(&self) -> Option<&Invokable>;

  /// Returns the field flavor attribute for the field
  fn flavors(&self) -> &[FieldFlavorAttribute];

  /// Returns the custom metadata of the field
  fn meta(&self) -> &Self::Meta;
}
