use crate::object::meta::concrete::PartialRefFieldFromMeta;

use super::{FieldDecodeOptions, FieldEncodeOptions};
use syn::{Attribute, Type, WherePredicate, punctuated::Punctuated, token::Comma};

impl PartialRefFieldFromMeta {
  pub(super) fn finalize(self) -> darling::Result<PartialRefFieldOptions> {
    Ok(PartialRefFieldOptions {
      copy: self.copy,
      attrs: self.attrs,
      ty: self.ty,
      encode: self.encode.finalize()?,
      decode: self.decode.finalize()?,
    })
  }
}

#[derive(Debug, Clone)]
pub(super) struct PartialRefFieldOptions {
  pub(in crate::object) copy: bool,
  pub(in crate::object) attrs: Vec<Attribute>,
  pub(in crate::object) ty: Option<Type>,
  pub(in crate::object) encode: FieldEncodeOptions,
  pub(in crate::object) decode: FieldDecodeOptions,
}

#[derive(Debug, Clone)]
pub struct PartialRefField {
  pub(super) ty: Type,
  pub(super) optional_type: Type,
  pub(super) decoded_state_type: Option<Type>,
  pub(super) decode_trait_type: Type,
  pub(super) attrs: Vec<Attribute>,
  pub(super) constraints: Punctuated<WherePredicate, Comma>,
  pub(super) copy: bool,
  pub(super) encode: FieldEncodeOptions,
  pub(super) decode: FieldDecodeOptions,
}

impl PartialRefField {
  /// Returns the type of the partial ref field.
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the optional type of the partial ref field, which is `Option<_>`.
  #[inline]
  pub const fn optional_type(&self) -> &Type {
    &self.optional_type
  }

  /// Returns the decoded state type of the partial ref field.
  ///
  /// Returns `Some(_)` only if the field use generics,
  #[inline]
  pub const fn decoded_state_type(&self) -> Option<&Type> {
    self.decoded_state_type.as_ref()
  }

  /// Returns the field decode trait type for this partial ref field.
  #[inline]
  pub const fn decode_trait_type(&self) -> &Type {
    &self.decode_trait_type
  }

  /// Returns the constraints of the partial ref field.
  #[inline]
  pub const fn type_constraints(&self) -> &Punctuated<WherePredicate, Comma> {
    &self.constraints
  }

  /// Returns the attributes of the partial ref field.
  #[inline]
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns whether the partial ref field is copyable.
  #[inline]
  pub const fn copy(&self) -> bool {
    self.copy
  }

  /// Returns the encode options about how to encode the partial ref field.
  #[inline]
  pub const fn encode(&self) -> &FieldEncodeOptions {
    &self.encode
  }

  /// Returns the decode options about how to decode the partial ref field.
  #[inline]
  pub const fn decode(&self) -> &FieldDecodeOptions {
    &self.decode
  }
}
