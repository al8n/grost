use super::{
  error::Error,
  flavors::{Flavor, FlavorError, WireFormat},
  selection::Selectable,
};

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
pub unsafe trait EquivalentEncode<O, W, F>
where
  Self: Encode<Self::WireFormat, Self::Flavor>,
  O: Encode<W, F> + ?Sized,
  W: WireFormat<F>,
  F: Flavor + ?Sized,
{
  /// The wire format for Self
  type WireFormat: WireFormat<Self::Flavor>;

  /// The flavor for Self
  type Flavor: Flavor + ?Sized;
}

unsafe impl<W, F, T> EquivalentEncode<T, W, F> for T
where
  T: Encode<W, F> + ?Sized,
  W: WireFormat<F>,
  F: Flavor + ?Sized,
{
  type WireFormat = W;
  type Flavor = F;
}

unsafe impl<W, F, T> EquivalentEncode<&T, W, F> for T
where
  T: Encode<W, F> + ?Sized,
  W: WireFormat<F>,
  F: Flavor + ?Sized,
{
  type WireFormat = W;
  type Flavor = F;
}

unsafe impl<W, F, T> EquivalentEncode<T, W, F> for &T
where
  T: Encode<W, F> + ?Sized,
  W: WireFormat<F>,
  F: Flavor + ?Sized,
{
  type WireFormat = W;
  type Flavor = F;
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
pub unsafe trait EquivalentPartialEncode<O, W, F>
where
  Self: PartialEncode<Self::WireFormat, Self::Flavor> + Selectable<Self::Flavor>,
  O: PartialEncode<W, F> + Selectable<F> + ?Sized,
  W: WireFormat<F>,
  F: Flavor + ?Sized,
{
  /// The wire format for Self
  type WireFormat: WireFormat<Self::Flavor>;

  /// The flavor for Self
  type Flavor: Flavor + ?Sized;
}

unsafe impl<W, F, T> EquivalentPartialEncode<T, W, F> for T
where
  T: PartialEncode<W, F> + Selectable<F> + ?Sized,
  W: WireFormat<F>,
  F: Flavor + ?Sized,
{
  type WireFormat = W;
  type Flavor = F;
}

unsafe impl<W, F, T> EquivalentPartialEncode<&T, W, F> for T
where
  T: PartialEncode<W, F> + Selectable<F> + ?Sized,
  W: WireFormat<F>,
  F: Flavor + ?Sized,
{
  type WireFormat = W;
  type Flavor = F;
}

unsafe impl<W, F, T> EquivalentPartialEncode<T, W, F> for &T
where
  T: PartialEncode<W, F> + Selectable<F> + ?Sized,
  W: WireFormat<F>,
  F: Flavor + ?Sized,
{
  type WireFormat = W;
  type Flavor = F;
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
/// field projections, or implementing protocols with optional fields.
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

impl<F, W, T> Encode<W, F> for &T
where
  T: Encode<W, F> + ?Sized,
  F: Flavor + ?Sized,
  W: WireFormat<F>,
{
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

impl<F, W, T> Encode<W, F> for Option<T>
where
  T: Encode<W, F>,
  F: Flavor + ?Sized,
  W: WireFormat<F>,
{
  fn encode(&self, context: &F::Context, buf: &mut [u8]) -> Result<usize, F::Error> {
    if let Some(value) = self {
      value.encode(context, buf)
    } else {
      Ok(0)
    }
  }

  fn encoded_len(&self, context: &F::Context) -> usize {
    if let Some(value) = self {
      value.encoded_len(context)
    } else {
      0
    }
  }

  fn encode_length_delimited(
    &self,
    context: &F::Context,
    buf: &mut [u8],
  ) -> Result<usize, F::Error> {
    if let Some(value) = self {
      value.encode_length_delimited(context, buf)
    } else {
      Ok(0)
    }
  }

  fn encoded_length_delimited_len(&self, context: &F::Context) -> usize {
    if let Some(value) = self {
      value.encoded_length_delimited_len(context)
    } else {
      0
    }
  }
}

impl<F, W, T> PartialEncode<W, F> for &T
where
  T: PartialEncode<W, F> + ?Sized,
  F: Flavor + ?Sized,
  W: WireFormat<F>,
{
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

impl<F, W, T> PartialEncode<W, F> for Option<T>
where
  T: PartialEncode<W, F>,
  F: Flavor + ?Sized,
  W: WireFormat<F>,
{
  fn partial_encode(
    &self,
    context: &F::Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, F::Error> {
    if let Some(value) = self {
      value.partial_encode(context, buf, selector)
    } else {
      Ok(0)
    }
  }

  fn partial_encoded_len(&self, context: &F::Context, selector: &Self::Selector) -> usize {
    if let Some(value) = self {
      value.partial_encoded_len(context, selector)
    } else {
      0
    }
  }

  fn partial_encode_length_delimited(
    &self,
    context: &F::Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, F::Error> {
    if let Some(value) = self {
      value.partial_encode_length_delimited(context, buf, selector)
    } else {
      Ok(0)
    }
  }

  fn partial_encoded_length_delimited_len(
    &self,
    context: &F::Context,
    selector: &Self::Selector,
  ) -> usize {
    if let Some(value) = self {
      value.partial_encoded_length_delimited_len(context, selector)
    } else {
      0
    }
  }
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

        fn is_empty(&self) -> bool {
          (**self).is_empty()
        }
      }

      impl<F, W, T> PartialEncode<W, F> for $ty
      where
        T: PartialEncode<W, F> + Selectable<F> + ?Sized,
        F: Flavor + ?Sized,
        W: WireFormat<F>,
      {
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
