use core::marker::PhantomData;

phantom!(
  /// Reflection to an encode fn.
  EncodeReflection,
  /// Reflection to an partial encode fn.
  PartialEncodeReflection,
);

impl EncodeReflection<EncodeField> {
  /// Returns the encode function.
  #[inline]
  pub const fn encode_field() -> Self {
    Self::new()
  }
}

impl EncodeReflection<EncodeRefField> {
  /// Returns the encode function.
  #[inline]
  pub const fn encode_ref_field() -> Self {
    Self(PhantomData)
  }
}

/// Encodes a field.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct EncodeField;

/// Encodes the reference type of a field
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct EncodeRefField;

/// Encodes a field.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct PartialEncodeField;

/// Encodes the reference type of a field
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct PartialEncodeRefField;
