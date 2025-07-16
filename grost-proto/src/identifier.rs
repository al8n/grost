use super::{buffer::ReadBuf, flavors::Flavor};

/// The identifier
pub trait Identifier<F: Flavor + ?Sized>: Copy + core::fmt::Debug + core::fmt::Display {
  /// Returns the wire type of the identifier.
  fn wire_type(&self) -> F::WireType;

  /// Returns the tag of the identifier.
  fn tag(&self) -> F::Tag;

  /// Encode the identifier into a buffer.
  fn encode(&self, dst: &mut [u8]) -> Result<usize, F::Error>;

  /// Return the length of the encoded identifier.
  fn encoded_len(&self) -> usize;

  /// Decode the identifier from a buffer.
  fn decode<'de, B>(buf: B) -> Result<(usize, Self), F::Error>
  where
    B: ReadBuf + Sized + 'de,
    Self: Sized;
}

/// An encoded identifier for a flavor.
///
/// The struct contains the identifier and its encoded representation.
pub struct EncodedIdentifier<'a, F: Flavor + ?Sized> {
  identifier: F::Identifier,
  encoded: &'a [u8],
}

impl<'a, F: Flavor + ?Sized> core::fmt::Debug for EncodedIdentifier<'a, F> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.debug_struct("EncodedIdentifier")
      .field("identifier", &self.identifier)
      .field("encoded", &self.encoded)
      .finish()
  }
}

impl<'a, F: Flavor + ?Sized> core::clone::Clone for EncodedIdentifier<'a, F> {
  fn clone(&self) -> Self {
    *self
  }
}

impl<'a, F: Flavor + ?Sized> core::marker::Copy for EncodedIdentifier<'a, F> {}

impl<'a, F: Flavor + ?Sized> EncodedIdentifier<'a, F> {
  /// Creates a new compiled identifier.
  #[inline]
  pub const fn new(identifier: F::Identifier, encoded: &'a [u8]) -> Self {
    Self {
      identifier,
      encoded,
    }
  }

  /// Returns the identifier.
  pub const fn identifier(&self) -> F::Identifier {
    self.identifier
  }

  /// Returns the encoded representation of the identifier.
  #[inline]
  pub const fn encoded(&self) -> &'a [u8] {
    self.encoded
  }
}

/// A maybe encoded identifier that can either be an encoded identifier or just an identifier.
pub enum MaybeEncodedIdentifier<'a, F: Flavor + ?Sized> {
  /// The identifier already contains the encoded representation.
  Encoded(EncodedIdentifier<'a, F>),
  /// The identifier only contains the identifier.
  Identifier(F::Identifier),
}

impl<'a, F: Flavor + ?Sized> core::fmt::Debug for MaybeEncodedIdentifier<'a, F> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match self {
      Self::Encoded(encoded) => f
        .debug_tuple("MaybeEncodedIdentifier::Encoded")
        .field(encoded)
        .finish(),
      Self::Identifier(identifier) => f
        .debug_tuple("MaybeEncodedIdentifier::Identifier")
        .field(identifier)
        .finish(),
    }
  }
}

impl<'a, F: Flavor + ?Sized> From<EncodedIdentifier<'a, F>> for MaybeEncodedIdentifier<'a, F> {
  fn from(encoded: EncodedIdentifier<'a, F>) -> Self {
    Self::Encoded(encoded)
  }
}

impl<'a, F: Flavor + ?Sized> MaybeEncodedIdentifier<'a, F> {
  /// Returns the identifier.
  #[inline]
  pub const fn identifier(&self) -> F::Identifier {
    match self {
      Self::Encoded(encoded) => encoded.identifier(),
      Self::Identifier(identifier) => *identifier,
    }
  }

  /// Returns the encoded representation of the identifier.
  #[inline]
  pub const fn encoded_identifier(&self) -> Option<&'a [u8]> {
    match self {
      Self::Encoded(encoded) => Some(encoded.encoded()),
      Self::Identifier(_) => None,
    }
  }

  /// Returns the encoded length of the identifier.
  #[inline]
  pub fn encoded_len(&self) -> usize {
    match self {
      Self::Encoded(encoded) => encoded.encoded().len(),
      Self::Identifier(identifier) => identifier.encoded_len(),
    }
  }

  /// Encodes the identifier into the given buffer.
  pub fn encode_to(&self, buf: &mut [u8]) -> Result<usize, F::Error> {
    match self {
      Self::Encoded(encoded) => {
        let encoded_len = encoded.encoded().len();
        let buf_len = buf.len();
        if encoded_len > buf_len {
          return Err(crate::error::Error::insufficient_buffer(encoded_len, buf_len).into());
        }
        buf[..encoded_len].copy_from_slice(encoded.encoded());
        Ok(encoded_len)
      }
      Self::Identifier(identifier) => identifier.encode(buf),
    }
  }
}

impl<'a, F: Flavor + ?Sized> core::clone::Clone for MaybeEncodedIdentifier<'a, F> {
  fn clone(&self) -> Self {
    *self
  }
}

impl<'a, F: Flavor + ?Sized> core::marker::Copy for MaybeEncodedIdentifier<'a, F> {}
