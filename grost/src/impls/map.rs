#[cfg(any(feature = "std", feature = "alloc"))]
mod btreemap;
#[cfg(any(feature = "std", feature = "alloc"))]
mod hashmap;
#[cfg(feature = "heapless")]
mod heapless;
#[cfg(feature = "indexmap")]
mod indexmap;

use varing::utils::*;

/// A trait for types that can be packed into a single value, which
/// can be used for varint encoding.
pub trait Packable<Rhs> {
  type Packed;

  fn pack(&self, rhs: &Rhs) -> Self::Packed;

  fn unpack(packed: Self::Packed) -> (Self, Rhs) where Self: Sized, Rhs: Sized;
}

macro_rules! impl_packable_for_primitives {
  (@self $($ty:ty => $packed:ty), +$(,)?) => {
    paste::paste! {
      $(
        impl Packable<$ty> for $ty {
          type Packed = $packed;
  
          fn pack(&self, rhs: &$ty) -> Self::Packed {
            [< pack_ $ty >](*self, *rhs)
          }
          
          fn unpack(packed: Self::Packed) -> (Self, $ty) where Self: Sized, $ty: Sized {
            [< unpack_ $ty >](packed)
          }
        }
      )*
    }
  };
  (@self_mix $($a:literal => $packed:literal), +$(,)?) => {
    paste::paste! {
      $(
        impl Packable<[< i $a >]> for [< u $a >] {
          type Packed = [< u $packed>];
        
          fn pack(&self, rhs: &[< i $a >]) -> Self::Packed {
            let b = [< zigzag_encode_i $a >](*rhs);
            self.pack(&b)
          }

          fn unpack(packed: Self::Packed) -> (Self, [< i $a >]) where Self: Sized, [< i $a >]: Sized {
            let (a, b) = <[< u $a >] as Packable<[< u $a >]>>::unpack(packed);
            (a, [< zigzag_decode_i $a >](b))
          }
        }

        impl Packable<[< u $a >]> for [< i $a >] {
          type Packed = [< u $packed>];

          #[inline]
          fn pack(&self, rhs: &[< u $a >]) -> Self::Packed {
            <[< u $a >] as Packable<[< i $a >]>>::pack(rhs, self)
          }

          #[inline]
          fn unpack(packed: Self::Packed) -> (Self, [< u $a >]) where Self: Sized, [< u $a >]: Sized {
            let (a, b) = <[< u $a >] as Packable<[< i $a >]>>::unpack(packed);
            (b, a)
          }
        }
      )*
    }
  };
  ($(($a:literal, $b:literal) => $packed:literal), +$(,)?) => {
    paste::paste! {
      impl_packable_for_primitives!(@signed $(($a, $b) => $packed),+);
      impl_packable_for_primitives!(@unsigned $(($a, $b) => $packed),+);
      impl_packable_for_primitives!(@mix $(($a, $b) => $packed),+);
    }
  };
  (@unsigned $(($a:literal, $b:literal) => $packed:literal), +$(,)?) => {
    paste::paste! {
      $(
        impl Packable<[< u $b >]> for [< u $a >] {
          type Packed = [< u $packed>];
        
          fn pack(&self, rhs: &[< u $b >]) -> Self::Packed {
            let small = *self as [< u $packed >];
            let large = *rhs as [< u $packed >];
        
            (large << [< u $a >]::BITS) | small
          }

          fn unpack(packed: Self::Packed) -> (Self, [< u $b >]) where Self: Sized, [< u $b >]: Sized {
            let small_mask: [< u $packed >] = (1 << [< u $a >]::BITS) - 1;

            let small: [< u $packed >] = packed & small_mask;
            let large: [< u $packed >] = packed >> [< u $a >]::BITS;
        
            (small as _, large as _)
          }
        }

        impl Packable<[< u $a >]> for [< u $b >] {
          type Packed = [< u $packed>];

          #[inline]
          fn pack(&self, rhs: &[< u $a >]) -> Self::Packed {
            <[< u $a >] as Packable<[< u $b >]>>::pack(rhs, self)
          }

          #[inline]
          fn unpack(packed: Self::Packed) -> (Self, [< u $a >]) where Self: Sized, [< u $a >]: Sized {
            let (a, b) = <[< u $a >] as Packable<[< u $b >]>>::unpack(packed);
            (b, a)
          }
        }
      )*
    }
  };
  (@signed $(($a:literal, $b:literal) => $packed:literal), +$(,)?) => {
    paste::paste! {
      $(
        impl Packable<[< i $b >]> for [< i $a >] {
          type Packed = [< u $packed>];
        
          fn pack(&self, rhs: &[< i $b >]) -> Self::Packed {
            let a = [< zigzag_encode_i $a >](*self);
            let b = [< zigzag_encode_i $b >](*rhs);
            a.pack(&b)
          }

          fn unpack(packed: Self::Packed) -> (Self, [< i $b >]) where Self: Sized, [< i $b >]: Sized {
            let (a, b) = <[< u $a >] as Packable<[< u $b >]>>::unpack(packed);
            ([< zigzag_decode_i $a >](a), [< zigzag_decode_i $b >](b))
          }
        }

        impl Packable<[< i $a >]> for [< i $b >] {
          type Packed = [< u $packed>];

          #[inline]
          fn pack(&self, rhs: &[< i $a >]) -> Self::Packed {
            <[< i $a >] as Packable<[< i $b >]>>::pack(rhs, self)
          }

          #[inline]
          fn unpack(packed: Self::Packed) -> (Self, [< i $a >]) where Self: Sized, [< u $a >]: Sized {
            let (a, b) = <[< i $a >] as Packable<[< i $b >]>>::unpack(packed);
            (b, a)
          }
        }
      )*
    }
  };
  (@mix $(($a:literal, $b:literal) => $packed:literal), +$(,)?) => {
    paste::paste! {
      $(
        impl Packable<[< i $b >]> for [< u $a >] {
          type Packed = [< u $packed>];
        
          fn pack(&self, rhs: &[< i $b >]) -> Self::Packed {
            let b = [< zigzag_encode_i $b >](*rhs);
            self.pack(&b)
          }

          fn unpack(packed: Self::Packed) -> (Self, [< i $b >]) where Self: Sized, [< i $b >]: Sized {
            let (a, b) = <[< u $a >] as Packable<[< u $b >]>>::unpack(packed);
            (a, [< zigzag_decode_i $b >](b))
          }
        }

        impl Packable<[< u $a >]> for [< i $b >] {
          type Packed = [< u $packed>];

          #[inline]
          fn pack(&self, rhs: &[< u $a >]) -> Self::Packed {
            <[< u $a >] as Packable<[< i $b >]>>::pack(rhs, self)
          }

          #[inline]
          fn unpack(packed: Self::Packed) -> (Self, [< u $a >]) where Self: Sized, [< u $a >]: Sized {
            let (a, b) = <[< u $a >] as Packable<[< i $b >]>>::unpack(packed);
            (b, a)
          }
        }

        impl Packable<[< u $b >]> for [< i $a >] {
          type Packed = [< u $packed>];
        
          fn pack(&self, rhs: &[< u $b >]) -> Self::Packed {
            let a = [< zigzag_encode_i $a >](*self);
            a.pack(rhs)
          }

          fn unpack(packed: Self::Packed) -> (Self, [< u $b >]) where Self: Sized, [< u $b >]: Sized {
            let (a, b) = <[< u $a >] as Packable<[< u $b >]>>::unpack(packed);
            ([< zigzag_decode_i $a >](a), b)
          }
        }

        impl Packable<[< i $a >]> for [< u $b >] {
          type Packed = [< u $packed>];

          #[inline]
          fn pack(&self, rhs: &[< i $a >]) -> Self::Packed {
            <[< i $a >] as Packable<[< u $b >]>>::pack(rhs, self)
          }

          #[inline]
          fn unpack(packed: Self::Packed) -> (Self, [< i $a >]) where Self: Sized, [< i $a >]: Sized {
            let (a, b) = <[< i $a >] as Packable<[< u $b >]>>::unpack(packed);
            (b, a)
          }
        }
      )*
    }
  }
}

impl_packable_for_primitives!(@self
  u8 => u16,
  u16 => u32,
  u32 => u64,
  u64 => u128,
  i8 => u16,
  i16 => u32,
  i32 => u64,
  i64 => u128,
);

impl_packable_for_primitives!(
  @self_mix
  8 => 16,
  16 => 32,
  32 => 64,
  64 => 128, 
);

impl_packable_for_primitives!(
  (8, 16) => 32,
  (8, 32) => 64,
  (8, 64) => 128,
  (16, 32) => 64,
  (16, 64) => 128,
  (32, 64) => 128,
);

#[cfg(feature = "arbitrary-int_1")]
const _: () = {
  use arbitrary_int_1::*;

  // impl Packable<u1> for u1 {
  //   type Packed = u2;

  //   fn pack(&self, rhs: &u1) -> Self::Packed {
  //     let l = u2::new(self.value());
  //     let h = u2::new(rhs.value());
  //     l << 1 | h
  //   }

  //   fn unpack(packed: Self::Packed) -> (Self, u1) where Self: Sized, u1: Sized {
  //     let l = (packed & u2::MAX).value();
  //     let h = (packed >> 1).value();
  //     let l = u1::new(l);
  //     let h = u1::new(h);
  //     (l, h)
  //   }
  // }

  // impl Packable<u2> for u1 {
  //   type Packed = u3;
    
  //   fn pack(&self, rhs: &u2) -> Self::Packed {
  //     let small = u3::new(self.value());
  //     let large = u3::new(rhs.value());
  //     (large << u1::BITS) | small
  //   }

  //   fn unpack(packed: Self::Packed) -> (Self, u2) where Self: Sized, u1: Sized {
  //     let small = (packed & u3::new(u1::MASK)).value();
  //     let large = (packed >> u1::BITS).value();
  //     let u1_val = u1::new(small);
  //     let u2_val = u2::new(large);
  //     (u1_val, u2_val)
  //   }
  // }
};
