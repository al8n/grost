use super::{
  buffer::{Buffer, BytesBuffer},
  flavors::Flavor,
};

/// A trait for types that can be decoded from bytes with a lifetime.
///
/// This trait provides methods to decode data from byte slices,
/// with support for both direct and length-prefixed decoding.
///
/// * `'de` - The lifetime of the input data
pub trait Decode<'de, F, O>
where
  F: Flavor + ?Sized,
{
  /// Decodes an instance of this type from a byte buffer.
  ///
  /// The function consumes the entire buffer and returns both the
  /// number of bytes consumed and the decoded instance.
  fn decode<UB>(
    context: &F::Context,
    wire_type: F::WireType,
    src: &'de [u8],
  ) -> Result<(usize, O), F::DecodeError>
  where
    O: Sized + 'de,
    UB: Buffer<F::Unknown<&'de [u8]>> + 'de;

  /// Decodes an instance of this type from a length-delimited byte buffer.
  ///
  /// The function consumes the entire buffer and returns both the
  /// number of bytes consumed and the decoded instance.
  fn decode_length_delimited<UB>(
    context: &F::Context,
    wire_type: F::WireType,
    src: &'de [u8],
  ) -> Result<(usize, O), F::DecodeError>
  where
    O: Sized + 'de,
    UB: Buffer<F::Unknown<&'de [u8]>> + 'de;
}

/// A trait for types that can be decoded without borrowing data.
///
/// Types implementing this trait can be decoded into owned values
/// without maintaining a borrow of the original data.
///
/// This is useful for deserialization scenarios where the input data
/// may not outlive the decoded value.
pub trait DecodeOwned<F, O>: Decode<'static, F, O> + 'static
where
  F: Flavor + ?Sized,
{
  /// Decodes an instance of this type from a byte buffer.
  ///
  /// The function consumes the entire buffer and returns both the
  /// number of bytes consumed and the decoded instance.
  fn decode_owned<B, UB>(
    context: &F::Context,
    wire_type: F::WireType,
    src: B,
  ) -> Result<(usize, O), F::DecodeError>
  where
    O: Sized + 'static,
    B: BytesBuffer + 'static,
    UB: Buffer<F::Unknown<B>> + 'static;

  /// Decodes an instance of this type from a length-delimited byte buffer.
  ///
  /// The function consumes the entire buffer and returns both the
  /// number of bytes consumed and the decoded instance.
  fn decode_length_delimited_owned<B, UB>(
    context: &F::Context,
    wire_type: F::WireType,
    src: B,
  ) -> Result<(usize, O), F::DecodeError>
  where
    O: Sized + 'static,
    B: BytesBuffer + 'static,
    UB: Buffer<F::Unknown<B>> + 'static;
}
