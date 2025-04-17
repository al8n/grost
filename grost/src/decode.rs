use super::{Buffer, Context, DecodeError, UnknownBuffer, Wirable, WireType, debug_assert_read_eq};

/// A trait for types that can be decoded from bytes with a lifetime.
///
/// This trait provides methods to decode data from byte slices,
/// with support for both direct and length-prefixed decoding.
///
/// * `'de` - The lifetime of the input data
pub trait Decode<'de, O>: Wirable {
  /// Decodes an instance of this type from a byte buffer.
  ///
  /// The function consumes the entire buffer and returns both the
  /// number of bytes consumed and the decoded instance.
  fn decode<UB>(context: &Context, src: &'de [u8]) -> Result<(usize, O), DecodeError>
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
pub trait DecodeOwned<O>: Decode<'static, O> + 'static {
  /// Decodes an instance of this type from a byte buffer.
  ///
  /// The function consumes the entire buffer and returns both the
  /// number of bytes consumed and the decoded instance.
  fn decode_owned<B, UB>(context: &Context, src: B) -> Result<(usize, Self), DecodeError>
  where
    Self: Sized + 'static,
    B: Buffer + 'static,
    UB: UnknownBuffer<B> + 'static;
}
