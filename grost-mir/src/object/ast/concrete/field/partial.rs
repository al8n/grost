use quote::quote;
use syn::{Attribute, Type};

use crate::{
  object::{Label, meta::concrete::PartialFieldFromMeta},
  utils::MissingOperation,
};

use super::PartialFieldConvertOptions;

impl PartialFieldFromMeta {
  /// Finalizes the partial field meta and returns the attribute
  pub(super) fn finalize(self) -> darling::Result<PartialFieldOptions> {
    Ok(PartialFieldOptions {
      attrs: self.attrs,
      partial_transform_ref: self.partial_transform_ref.finalize()?,
      transform_ref: self.transform_ref.finalize()?,
      partial_transform: self.partial_transform.finalize()?,
    })
  }
}

#[derive(Debug, Clone)]
pub(super) struct PartialFieldOptions {
  pub(in crate::object) attrs: Vec<Attribute>,
  pub(in crate::object) transform_ref: PartialFieldConvertOptions,
  pub(in crate::object) partial_transform_ref: PartialFieldConvertOptions,
  pub(in crate::object) partial_transform: PartialFieldConvertOptions,
}

#[derive(Debug, Clone)]
pub struct PartialField {
  pub(super) ty: Type,
  pub(super) optional_type: Type,
  pub(super) attrs: Vec<Attribute>,
  pub(super) transform_ref: PartialFieldConvertOptions,
  pub(super) partial_transform_ref: PartialFieldConvertOptions,
  pub(super) partial_transform: PartialFieldConvertOptions,
}

impl PartialField {
  /// Returns the specified type of the partial field, if any.
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the optional type of the partial field, which is `Option<_>`.
  #[inline]
  pub const fn optional_type(&self) -> &Type {
    &self.optional_type
  }

  /// Returns the attributes of the partial field.
  #[inline]
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns the transformation options for the partial field.
  #[inline]
  pub const fn transform_ref(&self) -> &PartialFieldConvertOptions {
    &self.transform_ref
  }

  /// Returns the partial transformation options for the partial field.
  #[inline]
  pub const fn partial_transform_ref(&self) -> &PartialFieldConvertOptions {
    &self.partial_transform_ref
  }

  /// Returns the partial transformation options for the partial field.
  #[inline]
  pub const fn partial_transform(&self) -> &PartialFieldConvertOptions {
    &self.partial_transform
  }

  pub(super) fn from_options(
    object: &super::RawObject,
    ty: &Type,
    mut opts: PartialFieldOptions,
    label: &Label,
  ) -> darling::Result<Self> {
    let optional_type = syn::parse2(quote! {
      ::core::option::Option<#ty>
    })?;

    let transform_ref_missing_operation = opts.transform_ref.missing_operation.or_else(|| {
      object
        .partial
        .transform
        .or_default_by_label(label)
        .then_some(MissingOperation::OrDefault)
    });
    let partial_transform_ref_missing_operation =
      opts.partial_transform_ref.missing_operation.or_else(|| {
        object
          .partial
          .partial_transform
          .or_default_by_label(label)
          .then_some(MissingOperation::OrDefault)
      });
    let partial_transform_missing_operation =
      opts.partial_transform.missing_operation.or_else(|| {
        object
          .partial
          .partial_transform
          .or_default_by_label(label)
          .then_some(MissingOperation::OrDefault)
      });

    Ok(Self {
      ty: ty.clone(),
      optional_type,
      attrs: opts.attrs,
      transform_ref: {
        opts.transform_ref.missing_operation = transform_ref_missing_operation;
        opts.transform_ref
      },
      partial_transform_ref: {
        opts.partial_transform_ref.missing_operation = partial_transform_ref_missing_operation;
        opts.partial_transform_ref
      },
      partial_transform: {
        opts.partial_transform.missing_operation = partial_transform_missing_operation;
        opts.partial_transform
      },
    })
  }
}
