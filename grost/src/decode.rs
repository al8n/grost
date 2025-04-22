use super::{BytesBuffer, Flavor, UnknownBuffer, Wirable};

/// A trait for types that can be decoded from bytes with a lifetime.
///
/// This trait provides methods to decode data from byte slices,
/// with support for both direct and length-prefixed decoding.
///
/// * `'de` - The lifetime of the input data
pub trait Decode<'de, F, O>: Wirable<F>
where
  F: Flavor,
{
  /// Decodes an instance of this type from a byte buffer.
  ///
  /// The function consumes the entire buffer and returns both the
  /// number of bytes consumed and the decoded instance.
  fn decode<UB>(context: &F::Context, src: &'de [u8]) -> Result<(usize, O), F::DecodeError>
  where
    Self: Sized + 'de,
    UB: UnknownBuffer<&'de [u8]> + 'de;
}

/// A marker trait for types that can be decoded without borrowing data.
///
/// Types implementing this trait can be decoded into owned values
/// without maintaining a borrow of the original data.
///
/// This is useful for deserialization scenarios where the input data
/// may not outlive the decoded value.
pub trait DecodeOwned<F, O>: Decode<'static, F, O> + 'static
where
  F: Flavor,
{
  /// Decodes an instance of this type from a byte buffer.
  ///
  /// The function consumes the entire buffer and returns both the
  /// number of bytes consumed and the decoded instance.
  fn decode_owned<B, UB>(context: &F::Context, src: B) -> Result<(usize, Self), F::DecodeError>
  where
    Self: Sized + 'static,
    B: BytesBuffer + 'static,
    UB: UnknownBuffer<B> + 'static;
}
