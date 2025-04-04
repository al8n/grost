use num_complex_0_4::Complex;

#[cfg(feature = "ruint_1")]
const _: () = {
  use ruint_1::aliases::*;

  varint!(
    Complex<U64>,
    Complex<U128>,
    Complex<U256>,
    Complex<U320>,
    Complex<U384>,
    Complex<U448>,
    Complex<U512>,
    Complex<U768>,
    Complex<U1024>,
    Complex<U2048>,
    Complex<U4096>,
  );
};

#[cfg(feature = "bnum_0_13")]
const _: () = {
  use bnum_0_13::*;

  macro_rules! impl_ {
    ($($ty:ident),+$(,)?) => {
      $(
        $crate::varint!(
          Complex<$ty<1>>,
          Complex<$ty<2>>,
          Complex<$ty<4>>,
          Complex<$ty<8>>,
          Complex<$ty<16>>,
          Complex<$ty<32>>,
          Complex<$ty<64>>,
          Complex<$ty<128>>,
          Complex<$ty<256>>,
          Complex<$ty<512>>,
          Complex<$ty<1024>>,
          Complex<$ty<2048>>,
          Complex<$ty<4096>>,
        );
      )*
    };
  }

  impl_!(
    BUintD8, BUintD16, BUintD32, BUint, BIntD8, BIntD16, BIntD32, BInt
  );
};

macro_rules! impl_ {
  ($($ty:ty), +$(,)?) => {
    $(
      varint!(Complex<$ty>);
    )*
  };
}

impl_!(u8, u16, u32, u64, i8, i16, i32, i64);

#[cfg(any(feature = "bnum_0_13", feature = "ruint_1"))]
impl_!(i128, u128);

#[cfg(feature = "half_2")]
const _: () = {
  use half_2::f16;

  fixed!(
    32(Complex<f16> {
      from_bytes: |src: &[u8]| {
        let re = f16::from_le_bytes(src[..2].try_into().unwrap());
        let im = f16::from_le_bytes(src[2..].try_into().unwrap());
        Ok(Complex { re, im })
      },
      to_bytes: |this: &Self, buf: &mut [u8]| {
        let re = this.re.to_le_bytes();
        let im = this.im.to_le_bytes();

        buf[..2].copy_from_slice(&re);
        buf[2..].copy_from_slice(&im);

        Ok(())
      },
    }),
  );
};

fixed!(
  64(Complex<f32> {
    from_bytes: |src: &[u8]| {
      let re = f32::from_le_bytes(src[..4].try_into().unwrap());
      let im = f32::from_le_bytes(src[4..].try_into().unwrap());
      Ok(Complex { re, im })
    },
    to_bytes: |this: &Self, buf: &mut [u8]| {
      let re = this.re.to_le_bytes();
      let im = this.im.to_le_bytes();

      buf[..4].copy_from_slice(&re);
      buf[4..].copy_from_slice(&im);

      Ok(())
    },
  }),
  128(Complex<f64> {
    from_bytes: |src: &[u8]| {
      let re = f64::from_le_bytes(src[..8].try_into().unwrap());
      let im = f64::from_le_bytes(src[8..].try_into().unwrap());
      Ok(Complex { re, im })
    },
    to_bytes: |this: &Self, buf: &mut [u8]| {
      let re = this.re.to_le_bytes();
      let im = this.im.to_le_bytes();

      buf[..8].copy_from_slice(&re);
      buf[8..].copy_from_slice(&im);

      Ok(())
    },
  }),
);
