use super::Flavor;

use ghost::phantom;

pub use default_wire_format::*;
pub use merged_wire_format::*;

mod default_wire_format;
mod merged_wire_format;

/// The wire format used for encoding and decoding.
pub trait WireFormat<F: Flavor + ?Sized>:
  Copy + Eq + core::hash::Hash + core::fmt::Debug + core::fmt::Display + Into<F::WireType>
{
  /// The cooresponding value to the wire type.
  const WIRE_TYPE: F::WireType;
  /// The self.
  const SELF: Self;
}

/// A static lifetime wire format, this is used to indicate that the wire format can be
/// used as a default wire format for a type.
pub trait StaticWireFormat<F: Flavor + ?Sized>: WireFormat<F> + 'static {
  const REFERENCE: &'static Self;
}

/// A wire format for packed repeated fields.
///
/// This is used to encode repeated fields in a more compact form,
/// a length prefix is used to indicate the number of bytes that the total packed data occupies.
///
/// For example, if you have a repeated field of integers, the packed format will encode them as:
///
/// ```text
/// | identifier | total_length | elem1 | elem2 | elem3 | ...
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
#[phantom]
pub struct Packed<W: ?Sized>;

impl<W: ?Sized> core::fmt::Display for Packed<W> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_str("packed")
  }
}

/// A wire format for packed entry repeated fields.
///
/// This is used to encode repeated fields in a more compact form,
/// a length prefix is used to indicate the number of bytes that the total packed data occupies.
///
/// For example, if you have a repeated field of integers, the packed format will encode them as:
///
/// ```text
/// | identifier | total_length | ent1 | ent2 | ent3 | ...
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
#[phantom]
pub struct PackedEntry<KW: ?Sized, VW: ?Sized>;

impl<KW: ?Sized, VW: ?Sized> core::fmt::Display for PackedEntry<KW, VW> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_str("packed_entry")
  }
}

/// The repeated wire format for element encoding within repeated fields.
///
/// When used as `Repeated<Repeated>`, this changes the encoding strategy
/// to a more stream-friendly format where each element is individually tagged.
/// This is similar to how Protocol Buffers encodes repeated fields when the `packed=false`
/// option is used.
///
/// Instead of the default length-prefixed encoding:
///
/// ```text
/// | identifier | total_length | elem1 | elem2 | elem3 | ... |
/// ```
///
/// `Repeated` encoding repeats the field identifier for each element:
///
/// ```text
/// | identifier | elem1 | identifier | elem2 | identifier | elem3 | ... |
/// ```
///
/// This format is useful for:
/// - Streaming scenarios where you need to process elements before receiving the entire collection
/// - Compatibility with Protocol Buffer's default repeated field encoding
/// - Cases where elements may be added incrementally

// TODO(al8n): change const `TAG: u32` to `TAG: Tag` wihen `feature(adt_const_params)` is stable
#[phantom]
pub struct Repeated<W: ?Sized, const TAG: u32>;

impl<W: ?Sized, const TAG: u32> Clone for Repeated<W, TAG> {
  fn clone(&self) -> Self {
    *self
  }
}

impl<W: ?Sized, const TAG: u32> Copy for Repeated<W, TAG> {}

impl<W: ?Sized, const TAG: u32> core::hash::Hash for Repeated<W, TAG> {
  fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
    TAG.hash(state);
  }
}

impl<W: ?Sized, const TAG: u32> PartialEq for Repeated<W, TAG> {
  fn eq(&self, _: &Self) -> bool {
    true
  }
}

impl<W: ?Sized, const TAG: u32> Eq for Repeated<W, TAG> {}

impl<W: ?Sized, const TAG: u32> core::fmt::Display for Repeated<W, TAG> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_str("repeated")
  }
}

impl<W: ?Sized, const TAG: u32> core::fmt::Debug for Repeated<W, TAG> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_str("Repeated")
  }
}

/// Similar to [`Repeated`], but used for repeated entries, like map entries and etc.
#[phantom]
pub struct RepeatedEntry<K: ?Sized, V: ?Sized, const TAG: u32>;

impl<K: ?Sized, V: ?Sized, const TAG: u32> Clone for RepeatedEntry<K, V, TAG> {
  fn clone(&self) -> Self {
    *self
  }
}

impl<K: ?Sized, V: ?Sized, const TAG: u32> Copy for RepeatedEntry<K, V, TAG> {}

impl<K: ?Sized, V: ?Sized, const TAG: u32> core::hash::Hash for RepeatedEntry<K, V, TAG> {
  fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
    TAG.hash(state);
  }
}

impl<K: ?Sized, V: ?Sized, const TAG: u32> PartialEq for RepeatedEntry<K, V, TAG> {
  fn eq(&self, _: &Self) -> bool {
    true
  }
}

impl<K: ?Sized, V: ?Sized, const TAG: u32> Eq for RepeatedEntry<K, V, TAG> {}

impl<K: ?Sized, V: ?Sized, const TAG: u32> core::fmt::Display for RepeatedEntry<K, V, TAG> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_str("repeated-entry")
  }
}

impl<K: ?Sized, V: ?Sized, const TAG: u32> core::fmt::Debug for RepeatedEntry<K, V, TAG> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_str("RepeatedEntry")
  }
}

/// A wire format for borrowed data.
///
/// ## Why we need `Borrowed` wire format wrapper?
///
/// e.g. when implementing `Encode` for `[T]`, we know that if `[T]` is encodable,
/// then `[&T]` must be encodable. But we cannot write the following code,
/// as `T` must be `Sized`, requried by `[T]`:
///
/// ```ignore
/// // We cannot write this code, as `T` must be `Sized`
/// impl<T: ?Sized> Encode<LengthDelimited, Groto> for [T] { ... }
/// ```
///
/// We cannot write another impl for `[&T]`, it will reports conflicting implementations.
///
/// ```ignore
/// // This implementation will conflict with the above one.
/// impl<T: ?Sized> Encode<LengthDelimited, Groto> for [&T] { ... }
/// ```
///
/// Hence, we need this wrapper type to indicate that the data is in borrowed state,
/// and we can implement `Encode` for `[&T]` like this:
///
/// ```ignore
/// impl<'a, T: ?Sized> Encode<Groto, Borrowed<'a, LengthDelimited>> for [&'a T] { ... }
/// ```
///
/// This is quite useful, when you have the following struct:
///
/// ```ignore
/// struct Friends(Vec<String>);
/// ```
///
/// And somehow, in a gossip groto, you receive three friends names from different peers.
///
/// ```ignore
/// let peer1_subscriber = subscribe_peer(1);
/// let peer2_subscriber = subscribe_peer(2);
/// let peer3_subscriber = subscribe_peer(3);
///
/// let alice_from_peer1 = peer1_subscriber.next().await;
/// let bob_from_peer2 = peer2_subscriber.next().await;
/// let charlie_from_peer3 = peer3_subscriber.next().await;
///
/// let borrowed_friends: &[&[&str]] = &[&alice_from_peer1, &bob_from_peer2, &charlie_from_peer3];
/// ```
///
/// Tranditional serialization frameworks would require you to create a new `Vec<String>`
/// and encode `&[String]`. With the `Borrowed` wrapper, you do not need to create a new `&[String]`
/// to encode it, you can just encode the borrowed slice directly as there is a blanket
/// implementation in this crate.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[phantom]
pub struct Borrowed<'a, W: ?Sized>;

impl<'a, W: ?Sized> core::fmt::Display for Borrowed<'a, W> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_str("borrowed")
  }
}

/// A wire format for nested data to encode it as a flattened structure.
///
/// ## Why we need `Flatten` wire format wrapper?
///
/// e.g. when implementing `Encode` for `Vec<Vec<T>>`, in some case, we want it to be encoded as `Vec<T>`:
/// So wrapping the inner wire format with `Flatten` will make it encoded as a flattened structure.
///
/// The first `W` is the actual wire format of the inner data,
/// and the second `I` is the wire format will be flattened to.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[phantom]
pub struct Flatten<W: ?Sized, I: ?Sized>;

impl<W: ?Sized, I: ?Sized> core::fmt::Display for Flatten<W, I> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_str("flatten")
  }
}

/// A wire format for nullable fields, in Rust it is used to represent `Option<T>` fields,
/// in schema, it corresponds to fields without `!`.
///
/// This wire format will make it have different encoding behavior for repeated fields.
/// For non-repeated fields, its encoding behavior just the same as inner wire format.
///
/// But, for `Packed<Nullable<W>>`, or `Repeated<Nullable<W>>`, it will be encoded differently.
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
#[phantom]
pub struct Nullable<W: ?Sized>;

impl<W: ?Sized> core::fmt::Display for Nullable<W> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_str("nullable")
  }
}

pub(super) mod sealed {
  pub trait JoinableAscii<const ASCII: u8> {}

  seq_macro::seq!(ASCII in 0..=128 {
    impl JoinableAscii<ASCII> for () {}
  });
}

seq_macro::seq!(N in 0..=63 {
  /// A wire format for joining collections with ASCII byte separators.
  ///
  /// `JoinAscii` allows you to encode collections (like `&[&[u8]]`) by joining their elements
  /// with up to 64 ASCII bytes as separators. This is useful for creating delimited formats
  /// where you need specific ASCII characters between elements.
  ///
  /// ## Type Parameters
  /// - `W`: The underlying wire format used to encode individual elements
  /// - `A~N`: Up to 64 ASCII byte constants (0-127) used as separators
  ///
  /// ## Separator Behavior
  /// - ASCII bytes with values 0-127 are included as separators
  /// - The special value `128` (default) is ignored and produces no separator
  /// - Separators are concatenated in the order they appear in the type parameters
  ///
  /// ## Examples
  ///
  /// ```rust
  /// // No separator - equivalent to directly encoding with the underlying format
  /// type NoSeparator = JoinAscii<LengthDelimited>; // Uses default 128 (ignored)
  ///
  /// // Single comma separator
  /// type CommaSeparated = JoinAscii<LengthDelimited, b','>;
  ///
  /// // Multiple separators: ", " (comma + space)
  /// type CommaSpaceSeparated = JoinAscii<LengthDelimited, b',', b' '>;
  ///
  /// // Custom delimiter sequence
  /// type PipeSeparated = JoinAscii<LengthDelimited, b'|'>;
  /// ```
  ///
  /// ## Use Cases
  /// - CSV-like formats with custom delimiters
  /// - Creating human-readable serialized data
  /// - Protocol formats that require specific ASCII separators
  /// - Converting nested collections to flat, delimited representations
  #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
  #[phantom]
  pub struct JoinAscii<W: ?Sized, #(const A~N: u8 = 128,)*>;

  impl<W: ?Sized, #(const A~N: u8,)*> core::fmt::Display for JoinAscii<W, #(A~N,)*> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
      f.write_str("join-ascii")
    }
  }

  impl<W: ?Sized, #(const A~N: u8,)*> JoinAscii<W, #(A~N,)*> {
    const __BYTES__: crate::utils::InlinedBytes::<64> = {
      let mut buf = [0; 64];

      let mut i = 0;
      #(
        if A~N < 128 {
          buf[i] = A~N;
          i += 1;
        }
      )*

      crate::utils::InlinedBytes::<64>::from_slice(buf.split_at(i).0).unwrap()
    };

    /// The ASCII bytes used for joining elements.
    ///
    /// This contains only the valid ASCII bytes (0-127) from the type parameters,
    /// in the order they were specified, with sentinel values (128) filtered out.
    pub const BYTES: &[u8] = Self::__BYTES__.as_bytes();

    /// The ASCII string used for joining elements.
    ///
    /// This is the UTF-8 representation of [`BYTES`](Self::BYTES). Since all bytes
    /// are guaranteed to be valid ASCII (0-127), this conversion is always safe.
    pub const STR: &str = unsafe {
      // SAFETY: The bytes are guaranteed to be valid UTF-8 as they are ASCII characters.
      core::str::from_utf8_unchecked(Self::BYTES)
    };
  }
});

const SENTINEL_CHAR: char = '\u{FFFF}';

seq_macro::seq!(N in 0..=63 {
  /// A wire format for joining collections with Unicode character separators.
  ///
  /// `JoinChar` allows you to encode collections (like `&[&[u8]]`) by joining their elements
  /// with up to 64 Unicode characters as separators. This provides more flexibility than
  /// `JoinAscii` by supporting the full Unicode character set.
  ///
  /// ## Type Parameters
  /// - `W`: The underlying wire format used to encode individual elements
  /// - `C~N`: Up to 64 Unicode character constants used as separators
  ///
  /// ## Separator Behavior
  /// - Any valid Unicode character can be used as a separator
  /// - The sentinel value `'\u{FFFF}'` (default) is ignored and produces no separator
  /// - Separators are concatenated in the order they appear in the type parameters
  /// - Characters are encoded as UTF-8 bytes in the final output
  ///
  /// ## Examples
  ///
  /// ```rust
  /// // No separator - equivalent to directly encoding with the underlying format
  /// type NoSeparator = JoinChar<LengthDelimited>; // Uses default sentinel (ignored)
  ///
  /// // Single comma separator
  /// type CommaSeparated = JoinChar<LengthDelimited, ','>;
  ///
  /// // Unicode separators
  /// type UnicodeArrow = JoinChar<LengthDelimited, '→'>;
  /// type EmojiBullet = JoinChar<LengthDelimited, '•'>;
  ///
  /// // Multiple characters: " -> " (space + arrow + space)
  /// type ArrowSeparated = JoinChar<LengthDelimited, ' ', '→', ' '>;
  ///
  /// // Mixed ASCII and Unicode
  /// type FancySeparated = JoinChar<LengthDelimited, ' ', '|', ' ', '★', ' '>;
  /// ```
  ///
  /// ## Use Cases
  /// - Internationalized data formats
  /// - Human-readable output with Unicode symbols
  /// - Creating visually appealing delimited data
  /// - Protocols that need to support non-ASCII separators
  /// - Converting nested collections with custom Unicode delimiters
  ///
  /// ## Performance Notes
  /// - Characters are encoded to UTF-8 at compile time
  /// - Multi-byte Unicode characters will use more space than ASCII equivalents
  #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
  #[phantom]
  // TODO(al8n): change const `C: char` to `C: Option<char>` when `feature(adt_const_params)` is stable
  pub struct JoinChar<W: ?Sized, #(const C~N: char = SENTINEL_CHAR,)*>;

  impl<W: ?Sized, #(const C~N: char,)*> core::fmt::Display for JoinChar<W, #(C~N,)*> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
      f.write_str("join-char")
    }
  }

  impl<W: ?Sized, #(const C~N: char,)*> JoinChar<W, #(C~N,)*> {
    const __BYTES__: crate::utils::InlinedBytes::<{ 64 * 4 }> = {
      let mut buf = [0; { 64 * 4 }];

      let mut i = 0;
      #(
        if C~N != SENTINEL_CHAR {
          i += C~N.encode_utf8(buf.split_at_mut(i).1).len();
        }
      )*

      crate::utils::InlinedBytes::<{ 64 * 4 }>::from_slice(buf.split_at(i).0).unwrap()
    };

    /// The UTF-8 encoded bytes used for joining elements.
    ///
    /// This contains the UTF-8 representation of all non-sentinel characters
    /// from the type parameters, concatenated in the order they were specified.
    pub const BYTES: &[u8] = Self::__BYTES__.as_bytes();

    /// The Unicode string used for joining elements.
    ///
    /// This is the UTF-8 string representation of all the separator characters.
    /// Characters are concatenated in the order they appear in the type parameters,
    /// with sentinel values filtered out.
    pub const STR: &str = unsafe {
      // SAFETY: The bytes are guaranteed to be valid UTF-8 since they come from
      // char::encode_utf8, which always produces valid UTF-8 sequences.
      core::str::from_utf8_unchecked(Self::BYTES)
    };
  }
});
