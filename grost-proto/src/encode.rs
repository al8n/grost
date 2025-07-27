use crate::{identifier::MaybeEncodedIdentifier, selection::Selector};

use super::{
  error::Error,
  flavors::{Flavor, FlavorError, WireFormat},
  selection::Selectable,
};

/// A trait for collection types like `Vec`, `HashSet`, etc.
pub trait Length {
  /// Returns the number of elements in the collection.
  fn length(&self) -> usize;
}

impl<T: ?Sized + Length> Length for &T {
  fn length(&self) -> usize {
    T::length(self)
  }
}

impl<T> Length for [T] {
  fn length(&self) -> usize {
    <[T]>::len(self)
  }
}

/// A marker trait indicating that two types produce equivalent encoded output
/// despite potentially having different wire formats or internal representations.
///
/// ## Safety
///
/// This trait is unsafe because incorrect implementation can lead to data corruption
/// or incorrect behavior in systems that rely on encoding equivalence. Implementers
/// must ensure that:
///
/// 1. all methods of `Encode` for `Self` and `O` produce the same results for equivalent values
/// 2. The equivalence holds across all possible contexts (both `F::Context` and `<Self::Flavor as Flavor>::Context`)
///
/// # Example
///
/// ```ignore
/// struct MyStr(str);
///
/// unsafe impl EquivalentEncode<MyStr, LengthDelimited, Groto> for str {
///   type Flavor = Groto;
///   type WireFormat = LengthDelimited;
/// }
/// ```
pub unsafe trait EquivalentEncode<Rhs, W, F>
where
  Self: Encode<Self::WireFormat, F>,
  Rhs: Encode<W, F> + ?Sized,
  W: WireFormat<F>,
  F: Flavor + ?Sized,
{
  /// The wire format for Self
  type WireFormat: WireFormat<F>;
}

unsafe impl<W, F, T> EquivalentEncode<T, W, F> for T
where
  T: Encode<W, F> + ?Sized,
  W: WireFormat<F>,
  F: Flavor + ?Sized,
{
  type WireFormat = W;
}

unsafe impl<W, F, T> EquivalentEncode<&T, W, F> for T
where
  T: Encode<W, F> + ?Sized,
  W: WireFormat<F>,
  F: Flavor + ?Sized,
{
  type WireFormat = W;
}

unsafe impl<W, F, T> EquivalentEncode<T, W, F> for &T
where
  T: Encode<W, F> + ?Sized,
  W: WireFormat<F>,
  F: Flavor + ?Sized,
{
  type WireFormat = W;
}

/// A marker trait indicating that two types produce equivalent partial encoded output
/// despite potentially having different wire formats or internal representations.
///
/// ## Safety
///
/// This trait is unsafe because incorrect implementation can lead to data corruption
/// or incorrect behavior in systems that rely on encoding equivalence. Implementers
/// must ensure that:
///
/// 1. all methods of `PartialEncode` for `Self` and `O` produce the same results for equivalent values
/// 2. The equivalence holds across all possible contexts (both `F::Context` and `<Self::Flavor as Flavor>::Context`)
///
/// ## Example
///
/// ```ignore
/// struct MyStr(str);
///
/// unsafe impl EquivalentPartialEncode<MyStr, LengthDelimited, Groto> for str {
///   type Flavor = Groto;
///   type WireFormat = LengthDelimited;
/// }
/// ```
pub unsafe trait EquivalentPartialEncode<Rhs, W, F>
where
  Self: PartialEncode<Self::WireFormat, F> + Selectable<F>,
  Rhs: PartialEncode<W, F> + Selectable<F> + ?Sized,
  W: WireFormat<F>,
  F: Flavor + ?Sized,
{
  /// The wire format for Self
  type WireFormat: WireFormat<F>;
}

unsafe impl<W, F, T> EquivalentPartialEncode<T, W, F> for T
where
  T: PartialEncode<W, F> + Selectable<F> + ?Sized,
  W: WireFormat<F>,
  F: Flavor + ?Sized,
{
  type WireFormat = W;
}

unsafe impl<W, F, T> EquivalentPartialEncode<&T, W, F> for T
where
  T: PartialEncode<W, F> + Selectable<F> + ?Sized,
  W: WireFormat<F>,
  F: Flavor + ?Sized,
{
  type WireFormat = W;
}

unsafe impl<W, F, T> EquivalentPartialEncode<T, W, F> for &T
where
  T: PartialEncode<W, F> + Selectable<F> + ?Sized,
  W: WireFormat<F>,
  F: Flavor + ?Sized,
{
  type WireFormat = W;
}

/// A trait for serializing data into a binary format using a specified [`Flavor`] and [`WireFormat`].
///
/// This trait provides three main encoding strategies:
///
/// 1. **Raw encoding** (`encode_raw`): Encodes data without any metadata or framing
/// 2. **Standard encoding** (`encode`): Encodes data with necessary metadata for self-describing formats - **this is what decoders expect**
/// 3. **Length-delimited encoding** (`encode_length_delimited`): Prepends a length prefix for framed protocols
///
/// ## Encoding Method Comparison
///
/// | Method | Use Case | Length Prefix | Self-Describing | Decoder Compatible |
/// |--------|----------|---------------|-----------------|-------------------|
/// | `encode_raw` | Raw data serialization | No | No | No<sup>1</sup> |
/// | `encode` | Self-contained messages | Depends on type | Yes | **Yes** |
/// | `encode_length_delimited` | Framed protocols | Always | Yes | Special<sup>2</sup> |
///
/// **Notes:**
/// - <sup>1</sup>`encode_raw` output cannot be decoded by standard `Decode` implementations
/// - <sup>2</sup>`encode_length_delimited` requires length-delimited-aware decoders
///
/// Types implementing this trait should ensure consistency between `encoded_len` and `encode`,
/// between `encoded_raw_len` and `encode_raw`, and between `encoded_length_delimited_len` and `encode_length_delimited`.
pub trait Encode<W: WireFormat<F>, F: Flavor + ?Sized> {
  /// Encodes the raw data without any metadata or framing information.
  ///
  /// This method performs the most basic encoding, serializing only the actual data content
  /// without any length prefixes, type information, or other metadata. The result is the
  /// minimal binary representation of the data.
  ///
  /// ## Relationship to other encoding methods
  ///
  /// - For **self-describing encode wire format**: `encode_raw` produces the same output as `encode`
  ///   because these types are self-describing and don't need additional metadata.
  ///
  /// - For **not self-describing encode wire format**: `encode_raw` produces
  ///   different output than `encode` because it omits the self-describing information that `encode` includes.
  ///
  /// ## Examples
  ///
  /// ```rust
  /// // For a string "hello":
  /// // encode_raw()    -> [0x68, 0x65, 0x6c, 0x6c, 0x6f]           (just UTF-8 bytes)
  /// // encode()        -> [0x05, 0x68, 0x65, 0x6c, 0x6c, 0x6f]     (length + UTF-8)
  ///
  /// // For a u32 value 42:
  /// // encode_raw()    -> [0x2a, 0x00, 0x00, 0x00]                 (4 bytes)
  /// // encode()        -> [0x2a, 0x00, 0x00, 0x00]                 (same, self-describing)
  /// ```
  fn encode_raw(&self, context: &F::Context, buf: &mut [u8]) -> Result<usize, F::Error>;

  /// Returns the exact number of bytes that `encode_raw` will write.
  ///
  /// This method calculates the size of the raw encoded data without any metadata
  /// or framing. It must return the exact number of bytes that `encode_raw` will
  /// write for the same context and data.
  ///
  /// ## Consistency guarantee
  ///
  /// For any given value and context, this method must return the same value
  /// that `encode_raw` will actually write:
  ///
  /// ```rust,ignore
  /// let expected_len = value.encoded_raw_len(context);
  /// let mut buffer = vec![0u8; expected_len];
  /// let actual_len = value.encode_raw(context, &mut buffer)?;
  /// assert_eq!(expected_len, actual_len);
  /// ```
  fn encoded_raw_len(&self, context: &F::Context) -> usize;

  /// Encodes the message with appropriate metadata for self-describing formats.
  ///
  /// This method produces a complete, self-contained encoded representation that can
  /// be decoded without external length or type information. For primitive types,
  /// this is identical to `encode_raw`. For variable-length types, this typically
  /// includes a length prefix or other metadata.
  ///
  /// **Important**: This is the method whose output is expected by the corresponding
  /// [`Decode`](super::decode::Decode) trait implementation. Decoders will expect to receive the exact format
  /// produced by this method, not the output of `encode_raw` or `encode_length_delimited`.
  ///
  /// ## Relationship to `encode_raw`
  ///
  /// - For **self-describing encode wire format**: `encode_raw` produces the same output as `encode`
  ///   because these types are self-describing and don't need additional metadata.
  ///
  /// - For **not self-describing encode wire format**: `encode_raw` produces
  ///   different output than `encode` because it omits the self-describing information that `encode` includes.
  fn encode(&self, context: &F::Context, buf: &mut [u8]) -> Result<usize, F::Error>;

  /// Returns the exact number of bytes that `encode` will write.
  ///
  /// This method calculates the size of the complete encoded message including
  /// any necessary metadata. It must return the exact number of bytes that
  /// `encode` will write for the same context and data.
  fn encoded_len(&self, context: &F::Context) -> usize;

  /// Encodes the message with a length prefix for framed protocols.
  ///
  /// This method creates a length-delimited message by:
  /// 1. Encoding the message length as a varint
  /// 2. Encoding the message itself using the `encode` method
  ///
  /// **Important**: This format requires length-delimited-aware decoders. Standard
  /// `Decode` implementations expect the output of `encode`, not `encode_length_delimited`.
  ///
  /// ## Output format
  ///
  /// ```text
  /// [varint_length][encoded_message]
  /// ```
  ///
  /// ## Use cases
  ///
  /// - Network protocols that require message framing
  /// - Streaming formats where message boundaries must be preserved
  /// - Protobuf-style wire formats
  ///
  /// ## Example
  ///
  /// ```rust
  /// // For a message that encodes to 5 bytes:
  /// // encode_length_delimited() -> [0x05, ...5 bytes of encoded message...]
  /// ```
  fn encode_length_delimited(
    &self,
    context: &F::Context,
    buf: &mut [u8],
  ) -> Result<usize, F::Error> {
    let encoded_len = self.encoded_len(context);
    let buf_len = buf.len();
    let offset = varing::encode_u32_varint_to(encoded_len as u32, buf).map_err(|e| {
      Error::from_varint_encode_error(e).update(self.encoded_length_delimited_len(context), buf_len)
    })?;

    let required = encoded_len + offset;
    if offset + encoded_len > buf_len {
      return Err(Error::insufficient_buffer(required, buf_len).into());
    }

    if offset >= buf_len {
      return Err(Error::insufficient_buffer(encoded_len, buf_len).into());
    }

    self
      .encode(context, &mut buf[offset..])
      .map(|v| {
        #[cfg(debug_assertions)]
        {
          crate::debug_assert_write_eq::<Self>(v, encoded_len);
        }

        required
      })
      .map_err(|mut e| {
        e.update_insufficient_buffer(required, buf_len);
        e
      })
  }

  /// Returns the number of bytes needed for length-delimited encoding.
  ///
  /// Length-delimited encoding prepends a varint-encoded length prefix to the
  /// standard encoded message. This format is commonly used in framed transport
  /// protocols like Protobuf wire format.
  ///
  /// ## Format
  ///
  /// ```text
  /// [varint_length][encoded_message]
  /// ```
  ///
  /// Where:
  /// - `varint_length` is the length of `encoded_message` encoded as a varint
  /// - `encoded_message` is the output of the `encode` method
  fn encoded_length_delimited_len(&self, context: &F::Context) -> usize {
    let encoded_len = self.encoded_len(context);
    let len_size = varing::encoded_u32_varint_len(encoded_len as u32);
    len_size + encoded_len
  }

  /// Encodes the raw message into a [`Vec`](std::vec::Vec).
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn encode_raw_to_vec(&self, context: &F::Context) -> Result<std::vec::Vec<u8>, F::Error> {
    let mut buf = std::vec![0; self.encoded_raw_len(context)];
    self.encode_raw(context, &mut buf)?;
    Ok(buf)
  }

  /// Encodes the message into a [`Vec`](std::vec::Vec).
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn encode_to_vec(&self, context: &F::Context) -> Result<std::vec::Vec<u8>, F::Error> {
    let mut buf = std::vec![0; self.encoded_len(context)];
    self.encode(context, &mut buf)?;
    Ok(buf)
  }

  /// Encodes the message into a [`Bytes`](super::bytes::Bytes).
  #[cfg(all(any(feature = "std", feature = "alloc"), feature = "bytes"))]
  fn encode_raw_to_bytes(&self, context: &F::Context) -> Result<super::bytes::Bytes, F::Error> {
    self.encode_raw_to_vec(context).map(Into::into)
  }

  /// Encodes the message into a [`Bytes`](super::bytes::Bytes).
  #[cfg(all(any(feature = "std", feature = "alloc"), feature = "bytes"))]
  fn encode_to_bytes(&self, context: &F::Context) -> Result<super::bytes::Bytes, F::Error> {
    self.encode_to_vec(context).map(Into::into)
  }

  /// Encodes the message with length-delimited format into a [`Vec`](std::vec::Vec).
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn encode_length_delimited_to_vec(
    &self,
    context: &F::Context,
  ) -> Result<std::vec::Vec<u8>, F::Error> {
    let mut buf = std::vec![0; self.encoded_length_delimited_len(context)];
    self.encode_length_delimited(context, &mut buf)?;
    Ok(buf)
  }

  /// Encodes the message with length-delimited format into a [`Bytes`](super::bytes::Bytes).
  #[cfg(all(any(feature = "std", feature = "alloc"), feature = "bytes"))]
  fn partial_encode_length_delimited_to_bytes(
    &self,
    context: &F::Context,
  ) -> Result<super::bytes::Bytes, F::Error> {
    self.encode_length_delimited_to_vec(context).map(Into::into)
  }
}

/// A trait for encoding only selected parts of a message based on a [`Selector`](Selectable::Selector).
///
/// `PartialEncode` extends the concept of encoding by allowing selective serialization of
/// message fields at runtime. This is particularly useful for filtering, patch updates,
/// field projections, or implementing protocols with nullable fields.
///
/// ## Field Selection Guarantees
///
/// **Important**: The level of field selection depends on the implementing type:
///
/// - **The reference types of collections** (e.g., `RepeatedDecoder`, `PackedDecoder`, `PackedSetDecoder`, `PackedMapDecoder`, `RepeatedMapDecoder`): May encode additional fields
///
/// ### Collection Reference Types Behavior
///
/// The reference types of collections are lazy decoders by default that hold encoded collection data and decode
/// elements on-demand during iteration. To encode only selected fields, they must first decode
/// each element and then re-encode with field selection applied. This process has several implications:
///
/// #### Trade-offs
///
/// - **Memory efficiency vs. Field precision**: Lazy decoders avoid allocating full collections in memory,
///   but may include additional fields in the output due to decoding granularity
/// - **Performance vs. Size**: Direct encoding from lazy decoders is faster (zero-allocating) but produces larger output;
///   decoding to owned data first is slower but guarantees minimal output size
///
/// #### Impact and Mitigation
///
/// **Generally not problematic**: On decoding, decoders can skip unselected fields,
/// so additional fields don't affect correctness.
///
/// **When size matters**: If encoded size is critical for your protocol (bandwidth-constrained
/// networks, storage limits, etc.), decode to owned collections first:
///
/// ```rust,ignore
/// // Option 1: Direct encoding (faster, potentially larger output)
/// let decoder: RepeatedDecoder<UserRef<'_>> = // ... from encoded data
/// let data = decoder.partial_encode_to_vec(context, &selector)?; // May include additional fields beyond selection
///
/// // Option 2: Decode-first approach (slower, guaranteed minimal output)
/// let users: Vec<User> = TryFromRef::try_from_ref(decoder)?; // Decode to owned collection
/// let data = users.partial_encode_to_vec(context, &selector)?; // Guaranteed to contain only selected fields
/// ```
///
/// #### Recommendation
///
/// - **Default approach**: Use direct encoding from collection reference types for better performance and memory usage
/// - **Size-critical protocols**: Decode to owned collections first when output size must be minimized
/// - **Hybrid approach**: Profile your specific use case to determine the optimal trade-off
///
/// ## Encoding Method Comparison
///
/// Like [`Encode`], this trait provides three encoding strategies for partial data:
///
/// | Method | Use Case | Length Prefix | Self-Describing | Decoder Compatible |
/// |--------|----------|---------------|-----------------|-------------------|
/// | `partial_encode_raw` | Raw partial data | No | No | No<sup>1</sup> |
/// | `partial_encode` | Self-contained partial messages | Depends on type | Yes | **Yes** |
/// | `partial_encode_length_delimited` | Framed partial protocols | Always | Yes | Special<sup>2</sup> |
///
/// **Notes:**
/// - <sup>1</sup>`partial_encode_raw` output cannot be decoded by standard partial decoders
/// - <sup>2</sup>`partial_encode_length_delimited` requires length-delimited-aware partial decoders
///
/// ## Relationship to [`Encode`]
///
/// `PartialEncode` complements [`Encode`] by offering runtime field selection:
/// - [`Encode`]: Serializes all fields of a message
/// - `PartialEncode`: Serializes only fields specified by the selector
///
/// When a selector includes all fields, the output should be equivalent to the corresponding
/// [`Encode`] method.
///
/// ## Common Use Cases
///
/// - **API filtering**: Return only requested fields to clients
/// - **Patch updates**: Serialize only changed fields for incremental updates  
/// - **Field projections**: Extract specific data subsets for processing
/// - **Conditional serialization**: Include fields based on runtime conditions
/// - **Bandwidth optimization**: Reduce payload size by omitting unnecessary fields
///
/// ## Example
///
/// ```rust,ignore
/// // For a User struct with fields: id, name, email, created_at
/// let user = User { id: 42, name: "Alice", email: "alice@example.com", created_at: now };
///
/// // Selector that includes only id and name
/// let selector = UserSelector::new().select_id().select_name();
///
/// // partial_encode will serialize only the selected fields
/// let partial_data = user.partial_encode_to_vec(context, &selector)?;
/// // Result contains only id and name, not email or created_at
/// ```
///
/// Types implementing this trait should ensure consistency between length calculation
/// and encoding methods, and maintain the same field selection behavior across all methods.
pub trait PartialEncode<W: WireFormat<F>, F: Flavor + ?Sized>: Selectable<F> {
  /// Encodes only the selected fields as raw data without any metadata or framing.
  ///
  /// This method performs selective encoding of the specified fields, serializing only
  /// their actual data content without any length prefixes, type information, or other
  /// metadata. The result is the minimal binary representation of the selected fields.
  ///
  /// ## When to use `partial_encode_raw`
  ///
  /// - When you need raw binary data of selected fields without any framing
  /// - For custom protocols where you handle length/type information separately
  /// - When concatenating multiple partial encoded values where metadata would interfere
  /// - **Note**: The output cannot be decoded by standard partial decoders
  ///
  /// ## Relationship to other partial encoding methods
  ///
  /// - For **self-describing encode wire format**: `partial_encode_raw` produces the same output as `partial_encode`
  ///   because these types are self-describing and don't need additional metadata.
  ///
  /// - For **not self-describing encode wire format**: `partial_encode_raw` produces
  ///   different output than `partial_encode` because it omits the self-describing information that `partial_encode` includes.
  fn partial_encode_raw(
    &self,
    context: &F::Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, F::Error>;

  /// Returns the exact number of bytes that `partial_encode_raw` will write for the selected fields.
  ///
  /// This method calculates the size of the raw encoded data for only the selected fields,
  /// without any metadata or framing. It must return the exact number of bytes that
  /// `partial_encode_raw` will write for the same context, data, and selector.
  ///
  /// ## Consistency guarantee
  ///
  /// For any given value, context, and selector, this method must return the same value
  /// that `partial_encode_raw` will actually write:
  ///
  /// ```rust
  /// let expected_len = value.partial_encoded_raw_len(context, selector);
  /// let mut buffer = vec![0u8; expected_len];
  /// let actual_len = value.partial_encode_raw(context, &mut buffer, selector)?;
  /// assert_eq!(expected_len, actual_len);
  /// ```
  ///
  /// ## Use cases
  ///
  /// - Pre-allocating buffers for partial raw encoding
  /// - Calculating total size when concatenating multiple partial raw-encoded values
  /// - Implementing custom partial framing protocols
  fn partial_encoded_raw_len(&self, context: &F::Context, selector: &Self::Selector) -> usize;

  /// Encodes only the selected fields with appropriate metadata for self-describing formats.
  ///
  /// This method produces a complete, self-contained encoded representation of the selected
  /// fields that can be decoded without external length or type information. For primitive
  /// selected fields, this is identical to `partial_encode_raw`. For variable-length selected
  /// fields, this typically includes length prefixes or other metadata.
  ///
  /// **Important**: This is the method whose output is expected by the corresponding
  /// partial `Decode` trait implementation. Partial decoders will expect to receive the exact
  /// format produced by this method, not the output of `partial_encode_raw` or
  /// `partial_encode_length_delimited`.
  ///
  /// ## Field Selection Behavior
  ///
  /// Only fields specified by the selector are included in the output. The encoding format
  /// and field ordering depend on the specific implementation, but typically:
  /// - Fields are encoded in their declaration order
  /// - Unselected fields are completely omitted (zero bytes)
  /// - Selected fields maintain their normal encoding format
  ///
  /// ## Relationship to `partial_encode_raw`
  ///
  /// - For **self-describing encode wire format**: `partial_encode_raw` produces the same output as `partial_encode`
  ///   because these types are self-describing and don't need additional metadata.
  ///
  /// - For **not self-describing encode wire format**: `partial_encode_raw` produces
  ///   different output than `partial_encode` because it omits the self-describing information that `partial_encode` includes.
  fn partial_encode(
    &self,
    context: &F::Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, F::Error>;

  /// Returns the exact number of bytes that `partial_encode` will write for the selected fields.
  ///
  /// This method calculates the size of the complete encoded message including any necessary
  /// metadata for only the selected fields. It must return the exact number of bytes that
  /// `partial_encode` will write for the same context, data, and selector.
  fn partial_encoded_len(&self, context: &F::Context, selector: &Self::Selector) -> usize;

  /// Encodes selected fields with a length prefix for framed protocols.
  ///
  /// This method creates a length-delimited partial message by:
  /// 1. Encoding the selected fields' total length as a varint
  /// 2. Encoding only the selected fields using the `partial_encode` method
  ///
  /// **Important**: This format requires length-delimited-aware partial decoders. Standard
  /// partial `Decode` implementations expect the output of `partial_encode`, not
  /// `partial_encode_length_delimited`.
  ///
  /// ## Output format
  ///
  /// ```text
  /// [varint_length][partial_encoded_selected_fields]
  /// ```
  ///
  /// ## Use cases
  ///
  /// - Network protocols that require partial message framing
  /// - Streaming formats where partial message boundaries must be preserved
  /// - Protobuf-style wire formats with field filtering
  /// - Patch/update protocols that send only changed fields
  ///
  /// ## Example
  ///
  /// ```rust
  /// // For selected fields that encode to 8 bytes total:
  /// // partial_encode_length_delimited() -> [0x08, ...8 bytes of selected fields...]
  /// ```
  fn partial_encode_length_delimited(
    &self,
    context: &F::Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, F::Error> {
    let encoded_len = self.partial_encoded_len(context, selector);
    let buf_len = buf.len();
    let offset = varing::encode_u32_varint_to(encoded_len as u32, buf).map_err(|e| {
      Error::from_varint_encode_error(e).update(
        self.partial_encoded_length_delimited_len(context, selector),
        buf_len,
      )
    })?;

    let required = encoded_len + offset;
    if offset + encoded_len > buf_len {
      return Err(Error::insufficient_buffer(required, buf_len).into());
    }

    if offset >= buf_len {
      return Err(Error::insufficient_buffer(encoded_len, buf_len).into());
    }

    self
      .partial_encode(context, &mut buf[offset..], selector)
      .map(|v| {
        #[cfg(debug_assertions)]
        {
          crate::debug_assert_write_eq::<Self>(v, encoded_len);
        }

        required
      })
      .map_err(|mut e| {
        e.update_insufficient_buffer(required, buf_len);
        e
      })
  }

  /// Returns the number of bytes needed for length-delimited encoding of selected fields.
  ///
  /// Length-delimited partial encoding prepends a varint-encoded length prefix to the
  /// partial encoded message. This format is commonly used in framed transport protocols
  /// where you need to transmit only selected fields with clear message boundaries.
  ///
  /// ## Format
  ///
  /// ```text
  /// [varint_length][partial_encoded_message]
  /// ```
  ///
  /// Where:
  /// - `varint_length` is the length of `partial_encoded_message` encoded as a varint
  /// - `partial_encoded_message` is the output of the `partial_encode` method for the selected fields
  fn partial_encoded_length_delimited_len(
    &self,
    context: &F::Context,
    selector: &Self::Selector,
  ) -> usize {
    let encoded_len = self.partial_encoded_len(context, selector);
    let len_size = varing::encoded_u32_varint_len(encoded_len as u32);
    len_size + encoded_len
  }

  /// Encodes selected fields in raw binary into [`Vec`](std::vec::Vec).
  ///
  /// This is a convenience method that allocates a buffer of the appropriate size
  /// and encodes the selected fields using `partial_encode_raw`.
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn partial_encode_raw_to_vec(
    &self,
    context: &F::Context,
    selector: &Self::Selector,
  ) -> Result<std::vec::Vec<u8>, F::Error> {
    let mut buf = std::vec![0; self.partial_encoded_raw_len(context, selector)];
    self.partial_encode_raw(context, &mut buf, selector)?;
    Ok(buf)
  }

  /// Encodes selected fields raw binary into [`Bytes`](super::bytes::Bytes).
  ///
  /// This is a convenience method that encodes to a `Vec` and then converts to `Bytes`.
  #[cfg(all(any(feature = "std", feature = "alloc"), feature = "bytes"))]
  fn partial_encode_raw_to_bytes(
    &self,
    context: &F::Context,
    selector: &Self::Selector,
  ) -> Result<super::bytes::Bytes, F::Error> {
    self
      .partial_encode_raw_to_vec(context, selector)
      .map(Into::into)
  }

  /// Encodes selected fields into a [`Vec`](std::vec::Vec).
  ///
  /// This is a convenience method that allocates a buffer of the appropriate size
  /// and encodes the selected fields using `partial_encode`.
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn partial_encode_to_vec(
    &self,
    context: &F::Context,
    selector: &Self::Selector,
  ) -> Result<std::vec::Vec<u8>, F::Error> {
    let mut buf = std::vec![0; self.partial_encoded_len(context, selector)];
    self.partial_encode(context, &mut buf, selector)?;
    Ok(buf)
  }

  /// Encodes selected fields into a [`Bytes`](super::bytes::Bytes).
  ///
  /// This is a convenience method that encodes to a `Vec` and then converts to `Bytes`.
  #[cfg(all(any(feature = "std", feature = "alloc"), feature = "bytes"))]
  fn partial_encode_to_bytes(
    &self,
    context: &F::Context,
    selector: &Self::Selector,
  ) -> Result<super::bytes::Bytes, F::Error> {
    self
      .partial_encode_to_vec(context, selector)
      .map(Into::into)
  }

  /// Encodes selected fields with length-delimited format into a [`Vec`](std::vec::Vec).
  ///
  /// This is a convenience method that allocates a buffer of the appropriate size
  /// and encodes the selected fields using `partial_encode_length_delimited`.
  #[cfg(any(feature = "std", feature = "alloc"))]
  fn partial_encode_length_delimited_to_vec(
    &self,
    context: &F::Context,
    selector: &Self::Selector,
  ) -> Result<std::vec::Vec<u8>, F::Error> {
    let mut buf = std::vec![0; self.partial_encoded_length_delimited_len(context, selector)];
    self.partial_encode_length_delimited(context, &mut buf, selector)?;
    Ok(buf)
  }

  /// Encodes selected fields with length-delimited format into a [`Bytes`](super::bytes::Bytes).
  ///
  /// This is a convenience method that encodes to a length-delimited `Vec` and then converts to `Bytes`.
  #[cfg(all(any(feature = "std", feature = "alloc"), feature = "bytes"))]
  fn partial_encode_length_delimited_to_bytes(
    &self,
    context: &F::Context,
    selector: &Self::Selector,
  ) -> Result<super::bytes::Bytes, F::Error> {
    self
      .partial_encode_length_delimited_to_vec(context, selector)
      .map(Into::into)
  }
}

/// A trait for encoding selected fields with their field identifiers in protobuf-style formats.
///
/// `PartialEncodeField` extends [`PartialEncode`] by adding field identifier encoding,
/// making it suitable for encoding fields within parent structures in protobuf-like wire formats.
/// This trait handles the complete field encoding process: identifier + selected data.
///
/// ## Key Differences from [`PartialEncode`]
///
/// | Aspect | [`PartialEncode`] | `PartialEncodeField` |
/// |--------|-------------------|---------------------|
/// | **Purpose** | Direct encoding of selected data | Field encoding with identifier |
/// | **Output** | `[selected_data]` | `[identifier][selected_data]` |
/// | **Use case** | Root-level encoding | Field within parent structure |
/// | **Identifier** | None | Required via `MaybeEncodedIdentifier` |
///
/// ## Field Encoding Process
///
/// Each method in this trait follows the same pattern:
/// 1. **Selection check**: If selector is empty, return 0 (no encoding)
/// 2. **Size calculation**: Calculate total size (identifier + selected data)
/// 3. **Buffer validation**: Ensure sufficient buffer space
/// 4. **Identifier encoding**: Write the field identifier first
/// 5. **Data encoding**: Write the selected data using corresponding `PartialEncode` method
///
/// ## Empty Selection Optimization
///
/// All methods include an important optimization:
/// ```rust
/// if selector.is_empty() {
///     return Ok(0);
/// }
/// ```
///
/// This ensures that:
/// - **No field identifier is written** when no data is selected
/// - **Zero bytes are produced** for completely unselected fields
/// - **Performance is optimized** by avoiding unnecessary encoding work
///
/// ## Method Variants
///
/// This trait provides field encoding variants for all three `PartialEncode` methods:
///
/// ### Raw Field Encoding
/// - `partial_encode_raw_field()` / `partial_encoded_raw_field_len()`
/// - **Format**: `[identifier][raw_selected_data]`
/// - **Use case**: Custom protocols, concatenation scenarios
///
/// ### Standard Field Encoding  
/// - `partial_encode_field()` / `partial_encoded_field_len()`
/// - **Format**: `[identifier][self_describing_selected_data]`
/// - **Use case**: Standard protobuf-style field encoding
/// - **Decoder compatible**: Output can be decoded by corresponding partial decoders
///
/// ### Length-Delimited Field Encoding
/// - `partial_encode_length_delimited_field()` / `partial_encoded_length_delimited_field_len()`
/// - **Format**: `[identifier][length_prefix][selected_data]`
/// - **Use case**: Streaming protocols, framed messages
/// - **Special requirement**: Needs length-delimited-aware decoders
///
/// ## Buffer Management
///
/// All encoding methods include comprehensive buffer management:
/// - **Pre-flight size checks**: Validate buffer capacity before encoding
/// - **Incremental validation**: Check buffer space at each encoding step
/// - **Detailed error context**: Provide expected vs. actual buffer sizes in errors
/// - **Atomic operations**: Either complete encoding succeeds or no bytes are written
///
/// ## Performance Characteristics
///
/// - **Zero allocation**: All methods work with provided buffers
/// - **Single pass**: Each method makes exactly one pass through the data
/// - **Early termination**: Empty selectors return immediately without processing
/// - **Compile-time optimization**: Field identifiers can be pre-encoded at compile time
/// - **Minimal overhead**: Identifier encoding is typically 1-5 bytes per field
///
/// ## Design Rationale
///
/// This trait addresses the fundamental challenge in protobuf-style encoding:
/// **field selection must be determined before the field identifier is encoded**.
///
/// By providing a separate trait for field-level encoding, we achieve:
/// - **Clean separation**: Direct encoding vs. field encoding are distinct operations
/// - **Optimization opportunities**: Empty selections can skip all encoding work
/// - **Flexibility**: Different identifier types and encoding strategies
/// - **Consistency**: All three encoding variants follow the same pattern
///
/// ## Implementation Notes
///
/// This trait is automatically implemented for all types that implement `PartialEncode`:
/// ```rust,ignore
/// impl<F, W, T> PartialEncodeField<W, F> for T
/// where
///     T: PartialEncode<W, F> + ?Sized,
///     F: Flavor + ?Sized,
///     W: WireFormat<F>,
/// {}
/// ```
///
/// This blanket implementation ensures that any type supporting partial encoding
/// automatically supports field encoding without additional implementation work.
pub trait PartialEncodeField<W: WireFormat<F>, F: Flavor + ?Sized>: PartialEncode<W, F> {
  /// Encodes selected fields as a raw field with identifier.
  ///
  /// This method combines field identifier encoding with raw partial data encoding,
  /// producing the format: `[identifier][raw_selected_data]`.
  ///
  /// ## Format
  /// ```text
  /// [field_identifier][raw_partial_data]
  /// ```
  ///
  /// ## Empty Selection Behavior
  /// If the selector is empty (no fields selected), this method returns `Ok(0)`
  /// without writing any bytes, including the field identifier.
  ///
  /// ## Parameters
  /// - `context`: Encoding context for the flavor
  /// - `identifier`: Field identifier to encode before the data
  /// - `buf`: Buffer to write the encoded field to
  /// - `selector`: Specifies which fields to include in the encoding
  ///
  /// ## Returns
  /// - `Ok(usize)`: Number of bytes written (identifier + selected data)
  /// - `Err(F::Error)`: Buffer insufficient or encoding error
  ///
  /// ## Buffer Requirements
  /// The buffer must be large enough to hold both the identifier and the selected data.
  /// Use `partial_encoded_raw_field_len()` to determine the required buffer size.
  fn partial_encode_raw_field(
    &self,
    context: &F::Context,
    identifier: MaybeEncodedIdentifier<'_, F>,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, F::Error> {
    if selector.is_empty() {
      return Ok(0);
    }

    let buf_len = buf.len();
    let encoded_data_len = self.partial_encoded_raw_len(context, selector);
    let encoded_len = identifier.encoded_len() + encoded_data_len;

    if encoded_len > buf_len {
      return Err(Error::insufficient_buffer(encoded_len, buf_len).into());
    }

    let mut offset = identifier.encode_to(buf).map_err(|mut e| {
      e.update_insufficient_buffer(encoded_len, buf_len);
      e
    })?;

    if offset + encoded_data_len > buf_len {
      return Err(Error::insufficient_buffer(encoded_len, buf_len).into());
    }

    let written = self
      .partial_encode_raw(context, &mut buf[offset..], selector)
      .map_err(|mut e| {
        e.update_insufficient_buffer(encoded_len, buf_len);
        e
      })?;
    offset += written;

    #[cfg(debug_assertions)]
    {
      crate::debug_assert_write_eq::<Self>(written, encoded_len);
    }

    Ok(offset)
  }

  /// Returns the total encoded length for raw field encoding.
  ///
  /// Calculates the total number of bytes that `partial_encode_raw_field` will write,
  /// including both the field identifier and the selected data.
  ///
  /// ## Returns
  /// - `0` if the selector is empty (no encoding will occur)
  /// - `identifier_len + selected_data_len` otherwise
  fn partial_encoded_raw_field_len(
    &self,
    context: &F::Context,
    identifier: MaybeEncodedIdentifier<'_, F>,
    selector: &Self::Selector,
  ) -> usize {
    if selector.is_empty() {
      return 0;
    }

    let encoded_data_len = self.partial_encoded_raw_len(context, selector);
    let identifier_len = identifier.encoded_len();
    identifier_len + encoded_data_len
  }

  /// Encodes selected fields as a complete field with identifier.
  ///
  /// This method combines field identifier encoding with standard partial data encoding,
  /// producing the format: `[identifier][self_describing_selected_data]`.
  ///
  /// **Important**: This is the primary method for field encoding in protobuf-style formats.
  /// The output is compatible with corresponding partial decoders.
  ///
  /// ## Format
  /// ```text
  /// [field_identifier][self_describing_partial_data]
  /// ```
  ///
  /// ## Empty Selection Behavior
  /// If the selector is empty, returns `Ok(0)` without writing any bytes.
  ///
  /// ## Decoder Compatibility
  /// The output of this method can be decoded by partial decoders that expect
  /// the standard field encoding format.
  fn partial_encode_field(
    &self,
    context: &F::Context,
    identifier: MaybeEncodedIdentifier<'_, F>,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, F::Error> {
    if selector.is_empty() {
      return Ok(0);
    }

    let buf_len = buf.len();
    let encoded_data_len = self.partial_encoded_len(context, selector);
    let encoded_len = identifier.encoded_len() + encoded_data_len;

    if encoded_len > buf_len {
      return Err(Error::insufficient_buffer(encoded_len, buf_len).into());
    }

    let mut offset = identifier.encode_to(buf).map_err(|mut e| {
      e.update_insufficient_buffer(encoded_len, buf_len);
      e
    })?;

    if offset + encoded_data_len > buf_len {
      return Err(Error::insufficient_buffer(encoded_len, buf_len).into());
    }

    let written = self
      .partial_encode(context, &mut buf[offset..], selector)
      .map_err(|mut e| {
        e.update_insufficient_buffer(encoded_len, buf_len);
        e
      })?;
    offset += written;

    #[cfg(debug_assertions)]
    {
      crate::debug_assert_write_eq::<Self>(written, encoded_len);
    }

    Ok(offset)
  }

  /// Returns the total encoded length for standard field encoding.
  ///
  /// Calculates the total bytes for identifier + self-describing selected data.
  fn partial_encoded_field_len(
    &self,
    context: &F::Context,
    identifier: MaybeEncodedIdentifier<'_, F>,
    selector: &Self::Selector,
  ) -> usize {
    if selector.is_empty() {
      return 0;
    }

    let encoded_data_len = self.partial_encoded_len(context, selector);
    let identifier_len = identifier.encoded_len();
    identifier_len + encoded_data_len
  }

  /// Encodes selected fields as a length-delimited field with identifier.
  ///
  /// This method produces a length-delimited field format suitable for streaming
  /// protocols: `[identifier][length_prefix][selected_data]`.
  ///
  /// ## Format
  /// ```text
  /// [field_identifier][varint_length][selected_data]
  /// ```
  ///
  /// ## Use Cases
  /// - Streaming protocols that need message boundaries
  /// - Framed transport protocols
  /// - Protocols where field length must be known before processing data
  ///
  /// ## Decoder Requirements
  /// Requires length-delimited-aware partial decoders. Standard partial decoders
  /// expect the output of `partial_encode_field`, not this method.
  ///
  /// ## Empty Selection Behavior
  /// Returns `Ok(0)` without writing any bytes if the selector is empty.
  fn partial_encode_length_delimited_field(
    &self,
    context: &F::Context,
    identifier: MaybeEncodedIdentifier<'_, F>,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, F::Error> {
    if selector.is_empty() {
      return Ok(0);
    }

    let buf_len = buf.len();
    let encoded_data_len = self.partial_encoded_length_delimited_len(context, selector);
    let encoded_len = identifier.encoded_len() + encoded_data_len;

    if encoded_len > buf_len {
      return Err(Error::insufficient_buffer(encoded_len, buf_len).into());
    }

    let mut offset = identifier.encode_to(buf).map_err(|mut e| {
      e.update_insufficient_buffer(encoded_len, buf_len);
      e
    })?;

    if offset + encoded_data_len > buf_len {
      return Err(Error::insufficient_buffer(encoded_len, buf_len).into());
    }

    let written = self
      .partial_encode_length_delimited(context, &mut buf[offset..], selector)
      .map_err(|mut e| {
        e.update_insufficient_buffer(encoded_len, buf_len);
        e
      })?;
    offset += written;

    #[cfg(debug_assertions)]
    {
      crate::debug_assert_write_eq::<Self>(written, encoded_len);
    }

    Ok(offset)
  }

  /// Returns the total encoded length for length-delimited field encoding.
  ///
  /// Calculates the total bytes for identifier + length prefix + selected data.
  fn partial_encoded_length_delimited_field_len(
    &self,
    context: &F::Context,
    identifier: MaybeEncodedIdentifier<'_, F>,
    selector: &Self::Selector,
  ) -> usize {
    if selector.is_empty() {
      return 0;
    }

    let encoded_data_len = self.partial_encoded_length_delimited_len(context, selector);
    let identifier_len = identifier.encoded_len();
    identifier_len + encoded_data_len
  }
}

impl<F, W, T> PartialEncodeField<W, F> for T
where
  T: PartialEncode<W, F> + ?Sized,
  F: Flavor + ?Sized,
  W: WireFormat<F>,
{
}

/// A trait for encoding complete fields with their field identifiers in protobuf-style formats.
///
/// `EncodeField` extends [`Encode`] by adding field identifier encoding,
/// making it suitable for encoding complete fields within parent structures in protobuf-like wire formats.
/// This trait handles the complete field encoding process: identifier + complete data.
///
/// ## Key Differences from [`Encode`]
///
/// | Aspect | [`Encode`] | `EncodeField` |
/// |--------|------------|---------------|
/// | **Purpose** | Direct encoding of complete data | Field encoding with identifier |
/// | **Output** | `[complete_data]` | `[identifier][complete_data]` |
/// | **Use case** | Root-level encoding | Field within parent structure |
/// | **Identifier** | None | Required via `MaybeEncodedIdentifier` |
///
/// ## Relationship to `PartialEncodeField`
///
/// | Aspect | `PartialEncodeField` | `EncodeField` |
/// |--------|---------------------|---------------|
/// | **Data scope** | Selected fields only | Complete structure |
/// | **Selector** | Required for field selection | Not applicable |
/// | **Empty optimization** | Skips encoding if selector empty | Always encodes (no selector) |
/// | **Use case** | Selective field encoding | Complete field encoding |
///
/// ## Field Encoding Process
///
/// Each method in this trait follows the same pattern:
/// 1. **Size calculation**: Calculate total size (identifier + complete data)
/// 2. **Buffer validation**: Ensure sufficient buffer space
/// 3. **Identifier encoding**: Write the field identifier first
/// 4. **Data encoding**: Write the complete data using corresponding `Encode` method
///
/// ## Method Variants
///
/// This trait provides field encoding variants for all three `Encode` methods:
///
/// ### Raw Field Encoding
/// - `encode_raw_field()` / `encoded_raw_field_len()`
/// - **Format**: `[identifier][raw_complete_data]`
/// - **Use case**: Custom protocols, concatenation scenarios
///
/// ### Standard Field Encoding  
/// - `encode_field()` / `encoded_field_len()`
/// - **Format**: `[identifier][self_describing_complete_data]`
/// - **Use case**: Standard protobuf-style field encoding
/// - **Decoder compatible**: Output can be decoded by corresponding decoders
///
/// ### Length-Delimited Field Encoding
/// - `encode_length_delimited_field()` / `encoded_length_delimited_field_len()`
/// - **Format**: `[identifier][length_prefix][complete_data]`
/// - **Use case**: Streaming protocols, framed messages
/// - **Special requirement**: Needs length-delimited-aware decoders
///
/// ## Buffer Management
///
/// All encoding methods include comprehensive buffer management:
/// - **Pre-flight size checks**: Validate buffer capacity before encoding
/// - **Incremental validation**: Check buffer space at each encoding step
/// - **Detailed error context**: Provide expected vs. actual buffer sizes in errors
/// - **Atomic operations**: Either complete encoding succeeds or no bytes are written
///
/// ## Performance Characteristics
///
/// - **Zero allocation**: All methods work with provided buffers
/// - **Single pass**: Each method makes exactly one pass through the data
/// - **Compile-time optimization**: Field identifiers can be pre-encoded at compile time
/// - **Minimal overhead**: Identifier encoding is typically 1-5 bytes per field
/// - **No selection overhead**: Unlike `PartialEncodeField`, no selector processing required
///
/// ## Design Rationale
///
/// This trait addresses complete field encoding in protobuf-style formats where:
/// **the entire structure needs to be encoded with a field identifier**.
///
/// By providing a separate trait for complete field-level encoding, we achieve:
/// - **Clean separation**: Complete encoding vs. partial encoding are distinct operations
/// - **Performance**: No selector processing overhead for complete encoding
/// - **Flexibility**: Different identifier types and encoding strategies
/// - **Consistency**: All three encoding variants follow the same pattern as `PartialEncodeField`
///
/// ## Implementation Notes
///
/// This trait is automatically implemented for all types that implement `Encode`:
/// ```rust,ignore
/// impl<F, W, T> EncodeField<W, F> for T
/// where
///     T: Encode<W, F> + ?Sized,
///     F: Flavor + ?Sized,
///     W: WireFormat<F>,
/// {}
/// ```
///
/// This blanket implementation ensures that any type supporting complete encoding
/// automatically supports field encoding without additional implementation work.
pub trait EncodeField<W: WireFormat<F>, F: Flavor + ?Sized>: Encode<W, F> {
  /// Encodes the complete structure as a raw field with identifier.
  ///
  /// This method combines field identifier encoding with raw complete data encoding,
  /// producing the format: `[identifier][raw_complete_data]`.
  ///
  /// ## Format
  /// ```text
  /// [field_identifier][raw_complete_data]
  /// ```
  ///
  /// ## Purpose
  /// Raw field encoding is used when you need direct byte-level concatenation
  /// without any self-describing metadata. The output contains the field identifier
  /// followed immediately by the raw encoded data.
  ///
  /// ## Parameters
  /// - `context`: Encoding context for the flavor, providing protocol-specific settings
  /// - `identifier`: Field identifier to encode before the data. This identifies the field
  ///   in the parent structure and is typically 1-5 bytes for most protocols
  /// - `buf`: Mutable byte buffer to write the encoded field to. Must have sufficient capacity
  /// - `selector`: Specifies which fields to include in the encoding (unused in this trait)
  ///
  /// ## Returns
  /// - `Ok(usize)`: Number of bytes written to the buffer (identifier + complete raw data)
  /// - `Err(F::Error)`: Encoding failed due to insufficient buffer space or encoding error
  ///
  /// ## Buffer Requirements
  /// The buffer must be large enough to hold both the field identifier and the complete
  /// raw encoded data. Use `encoded_raw_field_len()` to determine the exact buffer size needed.
  ///
  /// ## Error Conditions
  /// - **Insufficient buffer**: Buffer too small for identifier + data
  /// - **Identifier encoding failure**: Field identifier cannot be encoded
  /// - **Data encoding failure**: Raw data encoding fails
  ///
  /// ## Use Cases
  /// - **Custom protocols**: When you need control over the exact byte layout
  /// - **Performance-critical paths**: Minimal encoding overhead
  /// - **Binary concatenation**: Direct byte-level composition
  /// - **Legacy protocol support**: Protocols without self-describing formats
  ///
  /// ## Example
  /// ```rust,ignore
  /// let mut buffer = [0u8; 256];
  /// let identifier = MaybeEncodedIdentifier::from_field_number(42);
  /// let bytes_written = my_struct.encode_raw_field(&context, identifier, &mut buffer)?;
  /// // buffer now contains: [field_42_id][raw_struct_bytes]
  /// ```
  fn encode_raw_field(
    &self,
    context: &F::Context,
    identifier: MaybeEncodedIdentifier<'_, F>,
    buf: &mut [u8],
  ) -> Result<usize, F::Error> {
    let buf_len = buf.len();
    let encoded_data_len = self.encoded_raw_len(context);
    let encoded_len = identifier.encoded_len() + encoded_data_len;

    if encoded_len > buf_len {
      return Err(Error::insufficient_buffer(encoded_len, buf_len).into());
    }

    let mut offset = identifier.encode_to(buf).map_err(|mut e| {
      e.update_insufficient_buffer(encoded_len, buf_len);
      e
    })?;

    if offset + encoded_data_len > buf_len {
      return Err(Error::insufficient_buffer(encoded_len, buf_len).into());
    }

    let written = self
      .encode_raw(context, &mut buf[offset..])
      .map_err(|mut e| {
        e.update_insufficient_buffer(encoded_len, buf_len);
        e
      })?;
    offset += written;

    #[cfg(debug_assertions)]
    {
      crate::debug_assert_write_eq::<Self>(written, encoded_len);
    }

    Ok(offset)
  }

  /// Returns the total encoded length for raw field encoding.
  ///
  /// Calculates the total number of bytes that `partial_encode_raw_field` will write,
  /// including both the field identifier and the selected data.
  ///
  /// ## Returns
  /// - `identifier_len + selected_data_len` otherwise
  fn encoded_raw_field_len(
    &self,
    context: &F::Context,
    identifier: MaybeEncodedIdentifier<'_, F>,
  ) -> usize {
    let encoded_data_len = self.encoded_raw_len(context);
    let identifier_len = identifier.encoded_len();
    identifier_len + encoded_data_len
  }

  /// Encodes field with identifier.
  ///
  /// This method combines field identifier encoding with standard partial data encoding,
  /// producing the format: `[identifier][self_describing_complete_data]`.
  ///
  /// **Important**: This is the primary method for field encoding in protobuf-style formats.
  /// The output is compatible with corresponding partial decoders.
  ///
  /// ## Format
  /// ```text
  /// [field_identifier][self_describing_complete_data]
  /// ```
  /// 
  /// ## Decoder Compatibility
  /// The output of this method can be decoded by partial decoders that expect
  /// the standard field encoding format.
  fn encode_field(
    &self,
    context: &F::Context,
    identifier: MaybeEncodedIdentifier<'_, F>,
    buf: &mut [u8],
  ) -> Result<usize, F::Error> {
    let buf_len = buf.len();
    let encoded_data_len = self.encoded_len(context);
    let encoded_len = identifier.encoded_len() + encoded_data_len;

    if encoded_len > buf_len {
      return Err(Error::insufficient_buffer(encoded_len, buf_len).into());
    }

    let mut offset = identifier.encode_to(buf).map_err(|mut e| {
      e.update_insufficient_buffer(encoded_len, buf_len);
      e
    })?;

    if offset + encoded_data_len > buf_len {
      return Err(Error::insufficient_buffer(encoded_len, buf_len).into());
    }

    let written = self
      .encode(context, &mut buf[offset..])
      .map_err(|mut e| {
        e.update_insufficient_buffer(encoded_len, buf_len);
        e
      })?;
    offset += written;

    #[cfg(debug_assertions)]
    {
      crate::debug_assert_write_eq::<Self>(written, encoded_len);
    }

    Ok(offset)
  }

  /// Returns the total encoded length for standard field encoding.
  ///
  /// Calculates the total bytes for identifier + self-describing selected data.
  fn encoded_field_len(
    &self,
    context: &F::Context,
    identifier: MaybeEncodedIdentifier<'_, F>,
  ) -> usize {
    let encoded_data_len = self.encoded_len(context);
    let identifier_len = identifier.encoded_len();
    identifier_len + encoded_data_len
  }

  /// Encodes selected fields as a length-delimited field with identifier.
  ///
  /// This method produces a length-delimited field format suitable for streaming
  /// protocols: `[identifier][length_prefix][complete_data]`.
  ///
  /// ## Format
  /// ```text
  /// [field_identifier][varint_length][complete_data]
  /// ```
  ///
  /// ## Use Cases
  /// - Streaming protocols that need message boundaries
  /// - Framed transport protocols
  /// - Protocols where field length must be known before processing data
  ///
  /// ## Decoder Requirements
  /// Requires length-delimited-aware partial decoders. Standard partial decoders
  /// expect the output of `partial_encode_field`, not this method.
  fn encode_length_delimited_field(
    &self,
    context: &F::Context,
    identifier: MaybeEncodedIdentifier<'_, F>,
    buf: &mut [u8],
  ) -> Result<usize, F::Error> {
    let buf_len = buf.len();
    let encoded_data_len = self.encoded_length_delimited_len(context);
    let encoded_len = identifier.encoded_len() + encoded_data_len;

    if encoded_len > buf_len {
      return Err(Error::insufficient_buffer(encoded_len, buf_len).into());
    }

    let mut offset = identifier.encode_to(buf).map_err(|mut e| {
      e.update_insufficient_buffer(encoded_len, buf_len);
      e
    })?;

    if offset + encoded_data_len > buf_len {
      return Err(Error::insufficient_buffer(encoded_len, buf_len).into());
    }

    let written = self
      .encode_length_delimited(context, &mut buf[offset..])
      .map_err(|mut e| {
        e.update_insufficient_buffer(encoded_len, buf_len);
        e
      })?;
    offset += written;

    #[cfg(debug_assertions)]
    {
      crate::debug_assert_write_eq::<Self>(written, encoded_len);
    }

    Ok(offset)
  }

  /// Returns the total encoded length for length-delimited field encoding.
  ///
  /// Calculates the total bytes for identifier + length prefix + selected data.
  fn encoded_length_delimited_field_len(
    &self,
    context: &F::Context,
    identifier: MaybeEncodedIdentifier<'_, F>,
  ) -> usize {
    let encoded_data_len = self.encoded_length_delimited_len(context);
    let identifier_len = identifier.encoded_len();
    identifier_len + encoded_data_len
  }
}

impl<F, W, T> EncodeField<W, F> for T
where
  T: Encode<W, F> + ?Sized,
  F: Flavor + ?Sized,
  W: WireFormat<F>,
{
}

impl<F, W, T> Encode<W, F> for &T
where
  T: Encode<W, F> + ?Sized,
  F: Flavor + ?Sized,
  W: WireFormat<F>,
{
  fn encode_raw(
    &self,
    context: &<F as Flavor>::Context,
    buf: &mut [u8],
  ) -> Result<usize, <F as Flavor>::Error> {
    (*self).encode_raw(context, buf)
  }

  fn encoded_raw_len(&self, context: &<F as Flavor>::Context) -> usize {
    (*self).encoded_raw_len(context)
  }

  fn encode(&self, context: &F::Context, buf: &mut [u8]) -> Result<usize, F::Error> {
    (*self).encode(context, buf)
  }

  fn encoded_len(&self, context: &F::Context) -> usize {
    (*self).encoded_len(context)
  }

  fn encode_length_delimited(
    &self,
    context: &F::Context,
    buf: &mut [u8],
  ) -> Result<usize, F::Error> {
    (*self).encode_length_delimited(context, buf)
  }

  fn encoded_length_delimited_len(&self, context: &F::Context) -> usize {
    (*self).encoded_length_delimited_len(context)
  }
}

impl<F, W, T> PartialEncode<W, F> for &T
where
  T: PartialEncode<W, F> + ?Sized,
  F: Flavor + ?Sized,
  W: WireFormat<F>,
{
  fn partial_encode_raw(
    &self,
    context: &<F as Flavor>::Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, <F as Flavor>::Error> {
    (*self).partial_encode_raw(context, buf, selector)
  }

  fn partial_encoded_raw_len(
    &self,
    context: &<F as Flavor>::Context,
    selector: &Self::Selector,
  ) -> usize {
    (*self).partial_encoded_raw_len(context, selector)
  }

  fn partial_encode(
    &self,
    context: &F::Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, F::Error> {
    (*self).partial_encode(context, buf, selector)
  }

  fn partial_encoded_len(&self, context: &F::Context, selector: &Self::Selector) -> usize {
    (*self).partial_encoded_len(context, selector)
  }

  fn partial_encode_length_delimited(
    &self,
    context: &F::Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, F::Error> {
    (*self).partial_encode_length_delimited(context, buf, selector)
  }

  fn partial_encoded_length_delimited_len(
    &self,
    context: &F::Context,
    selector: &Self::Selector,
  ) -> usize {
    (*self).partial_encoded_length_delimited_len(context, selector)
  }
}

/// A trait for types that can be encoded into a wire format.
///
/// `Encodable` provides a high-level abstraction for types that support encoding
/// by creating specialized encoder instances. This trait serves as the entry point
/// for encoding complex structured data in grost's type system.
pub trait Encodable<W: WireFormat<F>, F: Flavor + ?Sized> {
  /// The type of encoder that can be used to encode this type.
  ///
  /// This associated type defines the specific encoder implementation that will
  /// handle encoding operations for this type.
  type Encoder<B>;

  /// Returns the encoder for this type.
  ///
  /// Creates and initializes an encoder instance that can be used to encode
  /// values of this type into the specified wire format.
  fn new<B>(buf: B) -> Self::Encoder<B>;

  /// Returns an encoder with buf and the start position of the buffer.
  fn with_position<B>(buf: B, current_position: usize) -> Self::Encoder<B>;
}

#[cfg(any(feature = "std", feature = "alloc", feature = "triomphe_0_1"))]
macro_rules! deref_encode_impl {
  ($($ty:ty),+$(,)?) => {
    $(
      impl<F, W, T> Encode<W, F> for $ty
      where
        T: Encode<W, F> + ?Sized,
        F: Flavor + ?Sized,
        W: WireFormat<F>,
      {
        fn encode_raw(&self, context: &<F as Flavor>::Context, buf: &mut [u8]) -> Result<usize, <F as Flavor>::Error> {
          (**self).encode_raw(context, buf)
        }

        fn encoded_raw_len(&self, context: &<F as Flavor>::Context) -> usize {
          (**self).encoded_raw_len(context)
        }

        fn encode(&self, context: &F::Context, buf: &mut [u8]) -> Result<usize, F::Error> {
          (**self).encode(context, buf)
        }

        fn encoded_len(&self, context: &F::Context) -> usize {
          (**self).encoded_len(context)
        }

        fn encode_length_delimited(
          &self,
          context: &F::Context,
          buf: &mut [u8],
        ) -> Result<usize, F::Error> {
          (**self).encode_length_delimited(context, buf)
        }

        fn encoded_length_delimited_len(&self, context: &F::Context) -> usize {
          (**self).encoded_length_delimited_len(context)
        }
      }
    )*
  };
}

#[cfg(any(feature = "std", feature = "alloc", feature = "triomphe_0_1"))]
macro_rules! deref_partial_encode_impl {
  ($($ty:ty),+$(,)?) => {{
    $(
      impl<T, F> Selectable<F> for $ty
      where
        T: ?Sized + Selectable<F>,
        F: Flavor + ?Sized,
      {
        type Selector = T::Selector;
      }

      impl<F, W, T> PartialEncode<W, F> for $ty
      where
        T: PartialEncode<W, F> + Selectable<F> + ?Sized,
        F: Flavor + ?Sized,
        W: WireFormat<F>,
      {
        fn partial_encode_raw(
          &self,
          context: &<F as Flavor>::Context,
          buf: &mut [u8],
          selector: &Self::Selector,
        ) -> Result<usize, <F as Flavor>::Error> {
          (**self).partial_encode_raw(context, buf, selector)
        }

        fn partial_encoded_raw_len(
          &self,
          context: &<F as Flavor>::Context,
          selector: &Self::Selector,
        ) -> usize {
          (**self).partial_encoded_raw_len(context, selector)
        }

        fn partial_encode(
          &self,
          context: &F::Context,
          buf: &mut [u8],
          selector: &Self::Selector,
        ) -> Result<usize, F::Error> {
          (**self).partial_encode(context, buf, selector)
        }

        fn partial_encoded_len(&self, context: &F::Context, selector: &Self::Selector) -> usize {
          (**self).partial_encoded_len(context, selector)
        }

        fn partial_encode_length_delimited(
          &self,
          context: &F::Context,
          buf: &mut [u8],
          selector: &Self::Selector,
        ) -> Result<usize, F::Error> {
          (**self).partial_encode_length_delimited(context, buf, selector)
        }

        fn partial_encoded_length_delimited_len(
          &self,
          context: &F::Context,
          selector: &Self::Selector,
        ) -> usize {
          (**self).partial_encoded_length_delimited_len(context, selector)
        }
      }
    )*
  }};
}

#[cfg(any(feature = "std", feature = "alloc"))]
const _: () = {
  use std::{boxed::Box, rc::Rc, sync::Arc};

  deref_encode_impl!(Box<T>, Rc<T>, Arc<T>);
  deref_partial_encode_impl!(Box<T>, Rc<T>, Arc<T>);
};

#[cfg(feature = "triomphe_0_1")]
const _: () = {
  use triomphe_0_1::Arc;

  deref_encode_impl!(Arc<T>);
  deref_partial_encode_impl!(Arc<T>);
};
