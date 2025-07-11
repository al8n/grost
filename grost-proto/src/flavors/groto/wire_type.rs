use super::Groto;
use crate::flavors::{
  Borrowed, Flatten, JoinAscii, JoinChar, MergeableWireFormat, MergedWireFormat, Nullable, Packed,
  PackedEntry, Repeated, RepeatedEntry, WireFormat, sealed::JoinableAscii,
};

wire_format!(
  WireType<Groto>
  /// The varint encoding/decoding wire format
  "varint",
  /// The length-delimited encoding/decoding wire format
  "length-delimited",
  /// The fixed 8-bit length encoding/decoding wire format
  "fixed8",
  /// The fixed 16-bit length encoding/decoding wire format
  "fixed16",
  /// The fixed 32-bit length encoding/decoding wire format
  "fixed32",
  /// The fixed 64-bit length encoding/decoding wire format
  "fixed64",
  /// The fixed 128-bit length encoding/decoding wire format
  "fixed128",
);

wire_type!(
  enum WireType<Groto> {
    /// The nullable encoding/decoding wire format
    "nullable" = 0,
    /// The varint encoding/decoding wire format
    "varint" = 1,
    /// The length-delimited encoding/decoding wire format
    "length-delimited" = 2,
    /// The fixed 8-bit length encoding/decoding wire format
    "fixed8" = 3,
    /// The fixed 16-bit length encoding/decoding wire format
    "fixed16" = 4,
    /// The fixed 32-bit length encoding/decoding wire format
    "fixed32" = 5,
    /// The fixed 64-bit length encoding/decoding wire format
    "fixed64" = 6,
    /// The fixed 128-bit length encoding/decoding wire format
    "fixed128" = 7,
  }
);

macro_rules! fixed_size {
  ($($ty:ty:$size:literal),+$(,)?) => {
    $(
      impl $ty {
        /// The size of the corresponding fixed-size type.
        pub const SIZE: usize = $size;
      }
    )*
  };
}

fixed_size!(
  Fixed8: 1,
  Fixed16: 2,
  Fixed32: 4,
  Fixed64: 8,
  Fixed128: 16,
);

// join ascii
const _: () = {
  seq_macro::seq!(N in 0..=63 {
    impl<W: WireFormat<Groto>, #(const A~N: u8,)*> From<JoinAscii<W, #(A~N,)*>> for WireType
    where
      #((): JoinableAscii<A~N>,)*
    {
      fn from(_: JoinAscii<W, #(A~N,)*>) -> Self {
        W::WIRE_TYPE
      }
    }

    impl<W: WireFormat<Groto>, #(const A~N: u8,)*> WireFormat<Groto> for JoinAscii<W, #(A~N,)*>
    where
      #((): JoinableAscii<A~N>,)*
    {
      const WIRE_TYPE: WireType = W::WIRE_TYPE;
      const INSTANCE: Self = JoinAscii;
    }

    impl<W: WireFormat<Groto>, #(const A~N: char,)*> From<JoinChar<W, #(A~N,)*>> for WireType
    {
      fn from(_: JoinChar<W, #(A~N,)*>) -> Self {
        W::WIRE_TYPE
      }
    }

    impl<W: WireFormat<Groto>, #(const A~N: char,)*> WireFormat<Groto> for JoinChar<W, #(A~N,)*>
    {
      const WIRE_TYPE: WireType = W::WIRE_TYPE;
      const INSTANCE: Self = JoinChar;
    }
  });
};

// borrowed
const _: () = {
  impl<'a, W: WireFormat<Groto>> From<Borrowed<'a, Packed<W>>> for WireType {
    fn from(_: Borrowed<'a, Packed<W>>) -> Self {
      WireType::LengthDelimited
    }
  }

  impl<'a, W: WireFormat<Groto>> WireFormat<Groto> for Borrowed<'a, Packed<W>> {
    const WIRE_TYPE: WireType = Packed::<W>::WIRE_TYPE;
    const INSTANCE: Self = Borrowed;
  }

  impl<'a, K: WireFormat<Groto>, V: WireFormat<Groto>> From<Borrowed<'a, PackedEntry<K, V>>>
    for WireType
  {
    fn from(_: Borrowed<'a, PackedEntry<K, V>>) -> Self {
      WireType::LengthDelimited
    }
  }

  impl<'a, K: WireFormat<Groto>, V: WireFormat<Groto>> WireFormat<Groto>
    for Borrowed<'a, PackedEntry<K, V>>
  {
    const WIRE_TYPE: WireType = PackedEntry::<K, V>::WIRE_TYPE;
    const INSTANCE: Self = Borrowed;
  }
};

// Flatten borrowed
const _: () = {
  impl<'a, W: WireFormat<Groto>, I: WireFormat<Groto>> From<Flatten<Borrowed<'a, Packed<W>>, I>>
    for WireType
  {
    fn from(_: Flatten<Borrowed<'a, Packed<W>>, I>) -> Self {
      I::WIRE_TYPE
    }
  }

  impl<'a, W: WireFormat<Groto>, I: WireFormat<Groto>> WireFormat<Groto>
    for Flatten<Borrowed<'a, Packed<W>>, I>
  {
    const WIRE_TYPE: WireType = I::WIRE_TYPE;
    const INSTANCE: Self = Flatten;
  }

  impl<'a, K: WireFormat<Groto>, V: WireFormat<Groto>, I: WireFormat<Groto>>
    From<Flatten<Borrowed<'a, PackedEntry<K, V>>, I>> for WireType
  {
    fn from(_: Flatten<Borrowed<'a, PackedEntry<K, V>>, I>) -> Self {
      I::WIRE_TYPE
    }
  }

  impl<'a, K: WireFormat<Groto>, V: WireFormat<Groto>, I: WireFormat<Groto>> WireFormat<Groto>
    for Flatten<Borrowed<'a, PackedEntry<K, V>>, I>
  {
    const WIRE_TYPE: WireType = I::WIRE_TYPE;
    const INSTANCE: Self = Flatten;
  }
};

// Flatten
const _: () = {
  impl<W: WireFormat<Groto>, I: WireFormat<Groto>> From<Flatten<Packed<W>, I>> for WireType {
    fn from(_: Flatten<Packed<W>, I>) -> Self {
      I::WIRE_TYPE
    }
  }

  impl<W: WireFormat<Groto>, I: WireFormat<Groto>> WireFormat<Groto> for Flatten<Packed<W>, I> {
    const WIRE_TYPE: WireType = I::WIRE_TYPE;
    const INSTANCE: Self = Flatten;
  }

  impl<K: WireFormat<Groto>, V: WireFormat<Groto>, I: WireFormat<Groto>>
    From<Flatten<PackedEntry<K, V>, I>> for WireType
  {
    fn from(_: Flatten<PackedEntry<K, V>, I>) -> Self {
      I::WIRE_TYPE
    }
  }

  impl<K: WireFormat<Groto>, V: WireFormat<Groto>, I: WireFormat<Groto>> WireFormat<Groto>
    for Flatten<PackedEntry<K, V>, I>
  {
    const WIRE_TYPE: WireType = I::WIRE_TYPE;
    const INSTANCE: Self = Flatten;
  }
};

// Flatten nullable
const _: () = {
  impl<W: WireFormat<Groto>, I: WireFormat<Groto>> From<Flatten<Nullable<W>, I>> for WireType {
    fn from(_: Flatten<Nullable<W>, I>) -> Self {
      I::WIRE_TYPE
    }
  }

  impl<W: WireFormat<Groto>, I: WireFormat<Groto>> WireFormat<Groto> for Flatten<Nullable<W>, I> {
    const WIRE_TYPE: WireType = I::WIRE_TYPE;
    const INSTANCE: Self = Flatten;
  }
};

// packed
const _: () = {
  impl<W: WireFormat<Groto>> From<Packed<W>> for WireType {
    fn from(_: Packed<W>) -> Self {
      WireType::LengthDelimited
    }
  }

  impl<W: WireFormat<Groto>> WireFormat<Groto> for Packed<W> {
    const WIRE_TYPE: WireType = WireType::LengthDelimited;
    const INSTANCE: Self = Packed;
  }
};

// packed entry
const _: () = {
  impl<KW: WireFormat<Groto>, VW: WireFormat<Groto>> From<PackedEntry<KW, VW>> for WireType {
    fn from(_: PackedEntry<KW, VW>) -> Self {
      WireType::LengthDelimited
    }
  }

  impl<KW: WireFormat<Groto>, VW: WireFormat<Groto>> WireFormat<Groto> for PackedEntry<KW, VW> {
    const WIRE_TYPE: WireType = WireType::LengthDelimited;
    const INSTANCE: Self = PackedEntry;
  }
};

// nullable
const _: () = {
  impl<W: WireFormat<Groto>> From<Nullable<W>> for WireType {
    fn from(_: Nullable<W>) -> Self {
      WireType::Nullable
    }
  }

  impl<W: WireFormat<Groto>> WireFormat<Groto> for Nullable<W> {
    const WIRE_TYPE: WireType = WireType::Nullable;
    const INSTANCE: Self = Nullable;
  }
};

// Repeated
const _: () = {
  impl<W, const I: u32> WireFormat<Groto> for Repeated<W, I>
  where
    W: WireFormat<Groto>,
  {
    const WIRE_TYPE: WireType = W::WIRE_TYPE;
    const INSTANCE: Self = Repeated;
  }

  impl<const I: u32, W> From<Repeated<W, I>> for WireType
  where
    W: WireFormat<Groto>,
  {
    fn from(_: Repeated<W, I>) -> Self {
      W::WIRE_TYPE
    }
  }
};

// RepeatedEntry
const _: () = {
  impl<KW, VW, const I: u32> WireFormat<Groto> for RepeatedEntry<KW, VW, I>
  where
    KW: WireFormat<Groto>,
    VW: WireFormat<Groto>,
    MergedWireFormat<KW, VW>: WireFormat<Groto>,
  {
    const WIRE_TYPE: WireType = MergedWireFormat::<KW, VW>::WIRE_TYPE;
    const INSTANCE: Self = RepeatedEntry;
  }

  impl<KW, VW, const I: u32> From<RepeatedEntry<KW, VW, I>> for WireType
  where
    KW: WireFormat<Groto>,
    VW: WireFormat<Groto>,
    MergedWireFormat<KW, VW>: WireFormat<Groto>,
  {
    fn from(_: RepeatedEntry<KW, VW, I>) -> Self {
      MergedWireFormat::<KW, VW>::WIRE_TYPE
    }
  }
};

const _: () = {
  macro_rules! merge_self {
    ($($ty: ty => $output:ty),+$(,)?) => {
      $(
        impl MergeableWireFormat<Groto> for $ty {
          type Merged = $output;
        }
      )*
    };
  }

  macro_rules! merge_rhs {
    ($(($ty: ty, $rhs:ty) => $output:ty),+$(,)?) => {
      $(
        impl MergeableWireFormat<Groto, $rhs> for $ty {
          type Merged = $output;
        }
      )*
    };
  }

  merge_self!(
    Fixed8 => Fixed16,
    Fixed16 => Fixed32,
    Fixed32 => Fixed64,
    Fixed64 => Fixed128,
    Varint => Varint,
  );

  impl<W2> MergeableWireFormat<Groto, W2> for LengthDelimited
  where
    W2: WireFormat<Groto>,
  {
    type Merged = LengthDelimited;
  }

  impl<W1, W2> WireFormat<Groto> for MergedWireFormat<W1, W2>
  where
    W1: WireFormat<Groto>,
    W2: WireFormat<Groto>,
    W1: MergeableWireFormat<Groto, W2>,
  {
    const WIRE_TYPE: WireType = <W1 as MergeableWireFormat<Groto, W2>>::Merged::WIRE_TYPE;
    const INSTANCE: Self = MergedWireFormat;
  }

  impl<W1, W2> From<MergedWireFormat<W1, W2>> for WireType
  where
    W1: WireFormat<Groto>,
    W2: WireFormat<Groto>,
    W1: MergeableWireFormat<Groto, W2>,
  {
    fn from(_: MergedWireFormat<W1, W2>) -> Self {
      <W1 as MergeableWireFormat<Groto, W2>>::Merged::WIRE_TYPE
    }
  }
};
