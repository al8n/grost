use num_rational_0_4::Ratio;

#[cfg(feature = "ruint_1")]
const _: () = {
  use ruint_1::aliases::*;

  varint!(
    Ratio<U64>,
    Ratio<U128>,
    Ratio<U256>,
    Ratio<U320>,
    Ratio<U384>,
    Ratio<U448>,
    Ratio<U512>,
    Ratio<U768>,
    Ratio<U1024>,
    Ratio<U2048>,
    Ratio<U4096>,
  );
};

#[cfg(feature = "bnum_0_13")]
const _: () = {
  use bnum_0_13::*;

  macro_rules! impl_ {
    ($($ty:ident),+$(,)?) => {
      $(
        $crate::varint!(
          Ratio<$ty<1>>,
          Ratio<$ty<2>>,
          Ratio<$ty<4>>,
          Ratio<$ty<8>>,
          Ratio<$ty<16>>,
          Ratio<$ty<32>>,
          Ratio<$ty<64>>,
          Ratio<$ty<128>>,
          Ratio<$ty<256>>,
          Ratio<$ty<512>>,
          Ratio<$ty<1024>>,
          Ratio<$ty<2048>>,
          Ratio<$ty<4096>>,
        );
      )*
    };
  }

  impl_!(BUintD8, BUintD16, BUintD32, BUint, BIntD8, BIntD16, BIntD32, BInt);
};

macro_rules! impl_ {
  ($($ty:ty), +$(,)?) => {
    $(
      varint!(Ratio<$ty>);
    )*   
  };
}

impl_!(u8, u16, u32, u64, i8, i16, i32, i64);

#[cfg(any(feature = "bnum_0_13", feature = "ruint_1"))]
impl_!(i128, u128);
