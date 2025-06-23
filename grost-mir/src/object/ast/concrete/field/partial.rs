use quote::quote;
use syn::{Attribute, Type};

use crate::{object::Label, utils::MissingOperation};

use super::{FieldConvertOptions, PartialFieldOptions};

#[derive(Debug, Clone)]
pub struct PartialField {
  pub(super) ty: Type,
  pub(super) optional_type: Type,
  pub(super) attrs: Vec<Attribute>,
  pub(super) transform: FieldConvertOptions,
  pub(super) partial_transform: FieldConvertOptions,
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

  pub(super) fn from_options<T, S, M>(
    object: &super::RawObject<T, S, M>,
    ty: &Type,
    mut opts: PartialFieldOptions,
    label: &Label,
  ) -> darling::Result<Self> {
    let optional_type = syn::parse2(quote! {
      ::core::option::Option<#ty>
    })?;

    let transform_missing_operation = opts.transform.missing_operation.or_else(|| {
      object
        .partial
        .transform
        .or_default_by_label(label)
        .then_some(MissingOperation::OrDefault)
    });
    let partial_transform_operation = opts.partial_transform.missing_operation.or_else(|| {
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
      transform: {
        opts.transform.missing_operation = transform_missing_operation;
        opts.transform
      },
      partial_transform: {
        opts.partial_transform.missing_operation = partial_transform_operation;
        opts.partial_transform
      },
    })
  }
}
