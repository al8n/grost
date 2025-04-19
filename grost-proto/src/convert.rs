use super::{flavors::Flavor, unknown, encode::Encode, decode::Decode};

pub trait PartialMessage<F: Flavor> {
  type UnknownBuffer<B: ?Sized>: unknown::UnknownBuffer<F, B>;

  /// A encoded representation of this type with lifetime 'a.
  ///
  /// This type can be converted back to the original type and decoded from raw bytes.
  type Encoded<'a>: Copy + TypeRef<F, Self> + Encode<F> + Decode<'a, F, Self::Encoded<'a>>
  where
    Self: Sized + 'a;

  /// A borrowed view of this type with lifetime 'a.
  ///
  /// This type provides a non-owned view that can be created from a reference
  /// and encoded when needed.
  type Borrowed<'a>: Copy + TypeBorrowed<'a, F, Self> + Encode<F>
  where
    Self: 'a;

  /// An owned encoded representation of this type.
  type EncodedOwned: Clone + TypeOwned<F, Self> + Encode<F> + Decode<'static, F, Self::EncodedOwned>
  where
    Self: Sized + 'static;
}

/// A message type that can be encoded and decoded.
///
/// This trait defines how output types can be encoded, decoded,
/// borrowed, and converted between different representations.
///
/// * `Encoded<'a>` - A encoded representation with lifetime 'a
/// * `Borrowed<'a>` - A borrowed view with lifetime 'a
/// * `EncodedOwned` - An owned encoded representation
pub trait Message<F: Flavor>: Encode<F> {
  /// The partial type of this message.
  type Partial: PartialMessage<F>;

  /// A encoded representation of this type with lifetime 'a.
  ///
  /// This type can be converted back to the original type and decoded from raw bytes.
  type Encoded<'a>: Copy + TypeRef<F, Self> + Encode<F> + Decode<'a, F, Self::Encoded<'a>>
  where
    Self: Sized + 'a;

  /// A borrowed view of this type with lifetime 'a.
  ///
  /// This type provides a non-owned view that can be created from a reference
  /// and encoded when needed.
  type Borrowed<'a>: Copy + TypeBorrowed<'a, F, Self> + Encode<F>
  where
    Self: 'a;

  /// An owned encoded representation of this type.
  type EncodedOwned: Clone + TypeOwned<F, Self> + Encode<F> + Decode<'static, F, Self::EncodedOwned>
  where
    Self: Sized + 'static;
}

/// A trait for consuming `Self` and converting it to `T`.
pub trait IntoTarget<F: Flavor, T> {
  /// Consumes this type and converts it to the target type.
  ///
  /// # Errors
  ///
  /// Returns a [`DecodeError`] if the conversion fails.
  fn into_target(self) -> Result<T, F::DecodeError>;
}

/// A trait for types that can be converted to another type.
///
/// This trait enables bidirectional conversion between encoded
/// representations and their corresponding decoded types.
///
/// * `T` - The target type to convert to
pub trait TypeRef<F: Flavor, T>: IntoTarget<F, T> {
  /// Converts a reference of this type to the target type.
  ///
  /// # Errors
  ///
  /// Returns a [`DecodeError`] if the conversion fails.
  fn to(&self) -> Result<T, F::DecodeError>;
}

/// A trait for types that can be converted to another type.
///
/// This trait enables bidirectional conversion between encoded
/// representations and their corresponding decoded types.
///
/// * `T` - The target type to convert to
pub trait TypeBorrowed<'a, F: Flavor, T: ?Sized> {
  /// Converts a reference of this type to the target type.
  ///
  /// # Errors
  ///
  /// Returns a [`DecodeError`] if the conversion fails.
  fn from_borrow(val: &'a T) -> Self;
}

impl<'a, F: Flavor, T: ?Sized> TypeBorrowed<'a, F, T> for &'a T {
  fn from_borrow(val: &'a T) -> Self {
    val
  }
}

/// A trait for types that can be converted to another type.
///
/// This trait enables bidirectional conversion between encoded
/// representations and their corresponding decoded types.
///
/// * `T` - The target type to convert to
pub trait TypeOwned<F: Flavor, T>: IntoTarget<F, T> {
  /// Converts a reference of this type to the target type.
  ///
  /// # Errors
  ///
  /// Returns a [`DecodeError`] if the conversion fails.
  fn to(&self) -> Result<T, F::DecodeError>;
}
