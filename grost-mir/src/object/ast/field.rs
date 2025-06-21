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
pub use partial::PartialFieldOptions;
pub use partial_decoded::PartialDecodedFieldOptions;
pub use selector::SelectorFieldOptions;

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
  extra: M,
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

  /// Returns the extra metadata associated with the skipped field
  #[inline]
  pub const fn extra(&self) -> &M {
    &self.extra
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

  pub(in crate::object) fn try_new(
    f: RawSkippedField<M>,
    type_params: &IdentSet,
    lifetime_params: &LifetimeSet,
  ) -> darling::Result<Self>
  where
    M: Clone,
  {
    let attrs = f.attrs;
    let vis = f.vis;
    let name = f.name;
    let ty = f.ty;
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
      extra: f.extra,
    })
  }
}

#[derive(Debug, Clone)]
pub enum FieldOptions<SO, TO> {
  Skipped {
    default: Option<Invokable>,
    extra: SO,
  },
  Tagged {
    label: Label,
    schema: SchemaAttribute,
    default: Option<Invokable>,
    tag: NonZeroU32,
    flavor: Vec<FieldFlavorAttribute>,
    convert: ConvertAttribute,
    partial: PartialFieldOptions,
    partial_decoded: PartialDecodedFieldOptions,
    selector: SelectorFieldOptions,
    copy: bool,
    extra: TO,
  },
}

impl<SO, TO> FieldOptions<SO, TO> {
  /// Returns the fn which will be used to generate the default value for the field
  pub const fn default(&self) -> Option<&Invokable> {
    match self {
      Self::Skipped { default, .. } => default.as_ref(),
      Self::Tagged { default, .. } => default.as_ref(),
    }
  }
}

impl<SO, TO> FieldFromMeta<SO, TO> {
  pub fn finalize(self) -> darling::Result<FieldOptions<SO, TO>> {
    Ok(match self {
      Self::Skipped { default, extra } => FieldOptions::Skipped { default, extra },
      Self::Tagged {
        label,
        schema,
        default,
        tag,
        flavor,
        convert,
        partial,
        partial_decoded,
        selector,
        copy,
        extra,
      } => FieldOptions::Tagged {
        label,
        schema: schema.into(),
        default,
        tag,
        flavor: flavor.finalize()?,
        convert: convert.finalize()?,
        partial: partial.finalize(),
        partial_decoded: partial_decoded.finalize()?,
        selector: selector.finalize(),
        copy,
        extra,
      },
    })
  }
}

#[derive(Debug, Clone)]
pub struct RawSkippedField<E> {
  pub name: Ident,
  pub ty: Type,
  pub vis: Visibility,
  pub attrs: Vec<Attribute>,
  pub default: Invokable,
  pub extra: E,
}

#[derive(Debug, Clone)]
pub struct RawTaggedField<E> {
  pub name: Ident,
  pub ty: Type,
  pub vis: Visibility,
  pub attrs: Vec<Attribute>,
  pub label: Label,
  pub schema: SchemaAttribute,
  pub default: Option<Invokable>,
  pub tag: NonZeroU32,
  pub flavor: Vec<FieldFlavorAttribute>,
  pub convert: ConvertAttribute,
  pub partial: PartialFieldOptions,
  pub partial_decoded: PartialDecodedFieldOptions,
  pub selector: SelectorFieldOptions,
  pub copy: bool,
  pub extra: E,
}

#[derive(Debug, Clone)]
pub enum RawField<TM = (), SM = ()> {
  Skipped(RawSkippedField<SM>),
  Tagged(RawTaggedField<TM>),
}

impl<TM, SM> RawField<TM, SM> {
  /// Creates a new raw field
  pub fn new(
    name: Ident,
    ty: Type,
    vis: Visibility,
    attrs: Vec<Attribute>,
    meta: FieldFromMeta<TM, SM>,
  ) -> darling::Result<Self> {
    match meta {
      FieldFromMeta::Skipped { default, extra } => Ok(Self::Skipped(RawSkippedField {
        name,
        ty,
        vis,
        attrs,
        default,
        extra,
      })),
      FieldFromMeta::Tagged {
        label,
        schema,
        default,
        tag,
        flavor,
        convert,
        partial,
        partial_decoded,
        selector,
        copy,
        extra,
      } => Ok(Self::Tagged(RawTaggedField {
        name,
        ty,
        vis,
        attrs,
        label,
        schema: schema.into(),
        default,
        tag,
        flavor: flavor.finalize()?,
        convert: convert.finalize()?,
        partial: partial.finalize(),
        partial_decoded: partial_decoded.finalize()?,
        selector: selector.finalize(),
        copy,
        extra,
      })),
    }
  }
}
