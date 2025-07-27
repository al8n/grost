use core::num::*;

macro_rules! impl_reflectable {
  ($(
    $constructor:ident($($ty:ty:$size:expr),+$(,)?)
  ),+$(,)?) => {
    $(
      $(
        impl $crate::reflection::Reflectable<$ty> for $crate::reflection::SchemaTypeReflection<$ty> {
          type Reflection = $crate::reflection::SchemaType;

          const REFLECTION: &Self::Reflection = &$crate::reflection::SchemaType::Scalar($crate::reflection::Scalar::$constructor($size));
        }
      )*
    )*
  };
}

impl_reflectable!(
  uint(
    u8:8,
    u16:16,
    u32:32,
    u64:64,
    u128:128,
  ),
  non_zero_uint(
    NonZeroU8:8,
    NonZeroU16:16,
    NonZeroU32:32,
    NonZeroU64:64,
    NonZeroU128:128,
  ),
  int(
    i8:8,
    i16:16,
    i32:32,
    i64:64,
    i128:128,
  ),
  non_zero_int(
    NonZeroI8:8,
    NonZeroI16:16,
    NonZeroI32:32,
    NonZeroI64:64,
    NonZeroI128:128,
  ),
  float(
    f32:32,
    f64:64,
  ),
);

#[cfg(feature = "half_2")]
const _: () = {
  use half_2::f16;

  impl_reflectable!(
    float(
      f16:16,
    ),
  );
};

#[cfg(feature = "decimal_1")]
const _: () = {
  use rust_decimal_1::Decimal;

  impl_reflectable!(
    float(
      Decimal:128,
    ),
  );
};

#[cfg(feature = "num-rational_0_4")]
const _: () = {
  use num_rational_0_4::Ratio;

  impl_reflectable!(
    unsigned_rational(
      Ratio<u8>:8,
      Ratio<u16>:16,
      Ratio<u32>:32,
      Ratio<u64>:64,
      Ratio<u128>:128,
    ),
    signed_rational(
      Ratio<i8>:8,
      Ratio<i16>:16,
      Ratio<i32>:32,
      Ratio<i64>:64,
      Ratio<i128>:128,
    ),
  );
};

#[cfg(feature = "num-complex_0_4")]
const _: () = {
  use num_complex_0_4::Complex;

  impl_reflectable!(
    unsigned_complex(
      Complex<u8>:8,
      Complex<u16>:16,
      Complex<u32>:32,
      Complex<u64>:64,
      Complex<u128>:128,
    ),
    signed_complex(
      Complex<i8>:8,
      Complex<i16>:16,
      Complex<i32>:32,
      Complex<i64>:64,
      Complex<i128>:128,
    ),
    float_complex(
      Complex<f32>:32,
      Complex<f64>:64,
    ),
  );

  #[cfg(feature = "half_2")]
  impl_reflectable!(
    float_complex(
      Complex<half_2::f16>:16,
    ),
  );

  #[cfg(feature = "decimal_1")]
  impl_reflectable!(
    float_complex(
      Complex<rust_decimal_1::Decimal>:128,
    ),
  );
};

#[cfg(feature = "arbitrary-int_1")]
const _: () = {
  use arbitrary_int_1::*;

  macro_rules! impl_arbitrary_int {
    ($($start:literal..=$end:literal), +$(,)?) => {
      $(
        seq_macro::seq!(N in $start..=$end {
          impl_reflectable!(uint(
            #(u~N:N,)*
          ));
        });
      )*
    };
  }

  impl_arbitrary_int!(1..=7, 9..=15, 17..=31, 33..=63, 65..=127,);
};

#[cfg(feature = "bnum_0_13")]
const _: () = {
  use bnum_0_13::*;

  use crate::reflection::{Reflectable, Scalar, SchemaType, SchemaTypeReflection};

  macro_rules! impl_bnum {
    ($(
      $constructor:ident($($ty:ty:$size:literal),+$(,)?)
    ),+$(,)?) => {
      $(
        $(
          impl<const N: usize> Reflectable<$ty> for SchemaTypeReflection<$ty> {
            type Reflection = SchemaType;

            const REFLECTION: &Self::Reflection = &SchemaType::Scalar(Scalar::$constructor(N * $size));
          }
        )*
      )*
    };
  }

  impl_bnum!(
    uint(
      BUint<N>:64,
      BUintD8<N>:8,
      BUintD16<N>:16,
      BUintD32<N>:32,
    ),
    int(
      BInt<N>:64,
      BIntD8<N>:8,
      BIntD16<N>:16,
      BIntD32<N>:32,
    )
  );
};

#[cfg(feature = "ruint_1")]
const _: () = {
  use ruint_1::*;

  use crate::reflection::{Reflectable, Scalar, SchemaType, SchemaTypeReflection};

  impl<const BITS: usize, const LBITS: usize> Reflectable<Uint<BITS, LBITS>>
    for SchemaTypeReflection<Uint<BITS, LBITS>>
  {
    type Reflection = SchemaType;

    const REFLECTION: &Self::Reflection = &SchemaType::Scalar(Scalar::uint(BITS));
  }
};
