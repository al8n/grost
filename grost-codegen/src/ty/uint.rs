use cfg_if::cfg_if;
use syn::{Type, parse_quote};

use super::{Dependency, Feature};

/// Unsupported signed integer width
#[derive(Clone, Debug, thiserror::Error)]
pub enum InvalidUintWidth {
  #[error("unsigned integer width {0} is not supported")]
  Unsupported(usize),
  #[error(
    "unsigned integer width {bits} is supported, but require {feature} feature to be enabled"
  )]
  RequireFeature { bits: usize, feature: Feature },
}

impl InvalidUintWidth {
  /// Returns the width of the integer
  pub fn width(&self) -> usize {
    match self {
      Self::Unsupported(width) => *width,
      Self::RequireFeature { bits, .. } => *bits,
    }
  }

  /// Creates a new `InvalidUintWidth::Unsupported` error
  pub fn unsupported(width: usize) -> Self {
    Self::Unsupported(width)
  }

  /// Creates a new `InvalidUintWidth::RequireFeature` error
  pub fn require_feature(bits: usize, feature: Feature) -> Self {
    Self::RequireFeature { bits, feature }
  }
}

#[derive(Clone)]
pub struct Uint {
  ty: Type,
  bits: usize,
  copy: bool,
  dependency: Dependency,
}

impl Uint {
  pub fn ty(&self) -> &Type {
    &self.ty
  }

  pub fn bits(&self) -> usize {
    self.bits
  }

  pub fn copy(&self) -> bool {
    self.copy
  }

  pub fn dependency(&self) -> &Dependency {
    &self.dependency
  }

  #[inline]
  pub const fn new(ty: Type, bits: usize, copy: bool, dependency: Dependency) -> Self {
    Self {
      ty,
      bits,
      copy,
      dependency,
    }
  }

  #[inline]
  pub const fn new_copy(ty: Type, bits: usize, dependency: Dependency) -> Self {
    Self::new(ty, bits, true, dependency)
  }

  /// Returns an `u8`
  #[inline]
  pub fn u8() -> Self {
    Self::new_copy(parse_quote!("::core::primitive::u8"), 8, Dependency::None)
  }

  /// Returns an `u16`
  #[inline]
  pub fn u16() -> Self {
    Self::new_copy(parse_quote!("::core::primitive::u16"), 16, Dependency::None)
  }

  /// Returns an `u32`
  #[inline]
  pub fn u32() -> Self {
    Self::new_copy(parse_quote!("::core::primitive::u32"), 32, Dependency::None)
  }

  /// Returns an `u64`
  #[inline]
  pub fn u64() -> Self {
    Self::new_copy(parse_quote!("::core::primitive::u64"), 64, Dependency::None)
  }

  /// Returns an `u128`
  #[inline]
  pub fn u128() -> Self {
    Self::new_copy(
      parse_quote!("::core::primitive::u128"),
      128,
      Dependency::None,
    )
  }

  /// Returns an `uN` which will use the builtin support of `grost`
  #[allow(unused_variables)]
  #[inline]
  pub fn u_n(bits: usize, path_to_grost: &syn::Path) -> Result<Self, InvalidUintWidth> {
    Ok(match bits {
      0 => {
        cfg_if! {
          if #[cfg(feature = "bnum_0_13")] {
            Self::bnum_0_13(parse_quote!(#path_to_grost::__private::bnum_0_13::BUintD8<0>), bits)
          } else if #[cfg(feature = "ruint_1")] {
            Self::ruint_1(bits, path_to_grost)
          } else {
            return Err(InvalidUintWidth::require_feature(bits, Feature::new("ruint_1")));
          }
        }
      }
      1..=7 => {
        cfg_if! {
          if #[cfg(feature = "arbitrary-int_1")] {
            Self::arbitrary_1(bits, path_to_grost)
          } else if #[cfg(feature = "ruint_1")] {
            Self::ruint_1(bits, path_to_grost)
          } else {
            return Err(InvalidUintWidth::require_feature(bits, Feature::new("ruint_1")));
          }
        }
      }
      8 => Self::u8(),
      9..=15 => {
        cfg_if! {
          if #[cfg(feature = "arbitrary-int_1")] {
            Self::arbitrary_1(bits, path_to_grost)
          } else if #[cfg(feature = "ruint_1")] {
            Self::ruint_1(bits, path_to_grost)
          } else {
            return Err(InvalidUintWidth::require_feature(bits, Feature::new("ruint_1")));
          }
        }
      }
      16 => Self::u16(),
      17..=31 => match () {
        #[cfg(feature = "bnum_0_13")]
        () if bits % 8 == 0 => {
          let limbs = bits / 8;
          Self::bnum_0_13(
            parse_quote!(#path_to_grost::__private::bnum_0_13::BUintD8<#limbs>),
            bits,
          )
        }
        _ => {
          cfg_if! {
            if #[cfg(feature = "arbitrary-int_1")] {
              Self::arbitrary_1(bits, path_to_grost)
            } else if #[cfg(feature = "ruint_1")] {
              Self::ruint_1(bits, path_to_grost)
            } else {
              return Err(InvalidUintWidth::require_feature(bits, Feature::new("ruint_1")));
            }
          }
        }
      },
      32 => Self::u32(),
      33..=63 => match () {
        #[cfg(feature = "bnum_0_13")]
        () if bits % 16 == 0 => {
          let limbs = bits / 16;
          Self::bnum_0_13(
            parse_quote!(#path_to_grost::__private::bnum_0_13::BUintD16<#limbs>),
            bits,
          )
        }
        #[cfg(feature = "bnum_0_13")]
        () if bits % 8 == 0 => {
          let limbs = bits / 8;
          Self::bnum_0_13(
            parse_quote!(#path_to_grost::__private::bnum_0_13::BUintD8<#limbs>),
            bits,
          )
        }
        _ => {
          cfg_if! {
            if #[cfg(feature = "arbitrary-int_1")] {
              Self::arbitrary_1(bits, path_to_grost)
            } else if #[cfg(feature = "ruint_1")] {
              Self::ruint_1(bits, path_to_grost)
            } else {
              return Err(InvalidUintWidth::require_feature(bits, Feature::new("ruint_1")));
            }
          }
        }
      },
      64 => Self::u64(),
      65..=127 => match () {
        #[cfg(feature = "bnum_0_13")]
        () if bits % 32 == 0 => {
          let limbs = bits / 32;
          Self::bnum_0_13(
            parse_quote!(#path_to_grost::__private::bnum_0_13::BUintD32<#limbs>),
            bits,
          )
        }
        #[cfg(feature = "bnum_0_13")]
        () if bits % 16 == 0 => {
          let limbs = bits / 16;
          Self::bnum_0_13(
            parse_quote!(#path_to_grost::__private::bnum_0_13::BUintD16<#limbs>),
            bits,
          )
        }
        #[cfg(feature = "bnum_0_13")]
        () if bits % 8 == 0 => {
          let limbs = bits / 8;
          Self::bnum_0_13(
            parse_quote!(#path_to_grost::__private::bnum_0_13::BUintD8<#limbs>),
            bits,
          )
        }
        _ => {
          cfg_if! {
            if #[cfg(feature = "arbitrary-int_1")] {
              Self::arbitrary_1(bits, path_to_grost)
            } else if #[cfg(feature = "ruint_1")] {
              Self::ruint_1(bits, path_to_grost)
            } else {
              return Err(InvalidUintWidth::require_feature(bits, Feature::new("ruint_1")));
            }
          }
        }
      },
      128 => Self::u128(),
      val => match () {
        #[cfg(feature = "bnum_0_13")]
        () if bits % 64 == 0 => {
          let limbs = bits / 64;
          Self::bnum_0_13(
            parse_quote!(#path_to_grost::__private::bnum_0_13::BUint<#limbs>),
            bits,
          )
        }
        #[cfg(feature = "bnum_0_13")]
        () if bits % 32 == 0 => {
          let limbs = bits / 32;
          Self::bnum_0_13(
            parse_quote!(#path_to_grost::__private::bnum_0_13::BUintD32<#limbs>),
            bits,
          )
        }
        #[cfg(feature = "bnum_0_13")]
        () if bits % 16 == 0 => {
          let limbs = bits / 16;
          Self::bnum_0_13(
            parse_quote!(#path_to_grost::__private::bnum_0_13::BUintD16<#limbs>),
            bits,
          )
        }
        #[cfg(feature = "bnum_0_13")]
        () if bits % 8 == 0 => {
          let limbs = bits / 8;
          Self::bnum_0_13(
            parse_quote!(#path_to_grost::__private::bnum_0_13::BUintD8<#limbs>),
            bits,
          )
        }
        _ => {
          cfg_if! {
            if #[cfg(feature = "ruint_1")] {
              Self::ruint_1(val, path_to_grost)
            } else {
              return Err(InvalidUintWidth::require_feature(bits, Feature::new("ruint_1")));
            }
          }
        }
      },
    })
  }

  #[cfg(feature = "arbitrary-int_1")]
  #[inline]
  fn arbitrary_1(bits: usize, path_to_grost: &syn::Path) -> Self {
    let ident = quote::format_ident!("u{}", bits);
    let ty: Type = parse_quote!(#path_to_grost::__private::arbitrary_int_1::#ident);
    Self {
      ty,
      bits,
      copy: true,
      dependency: Dependency::External(Feature::new("arbitrary-int_1")),
    }
  }

  #[cfg(feature = "bnum_0_13")]
  #[inline]
  fn bnum_0_13(ty: Type, bits: usize) -> Self {
    Self {
      ty,
      bits,
      copy: true,
      dependency: Dependency::External(Feature::new("bnum_0_13")),
    }
  }

  #[cfg(feature = "ruint_1")]
  #[inline]
  fn ruint_1(bits: usize, path_to_grost: &syn::Path) -> Self {
    let limbs = if bits < 64 {
      1usize
    } else if bits % 64 == 0 {
      bits / 64
    } else {
      bits / 64 + 1
    };
    let ident = quote::format_ident!("Uint<{bits}, {limbs}>");
    let ty: Type = parse_quote!(#path_to_grost::__private::ruint_1::#ident);
    Self {
      ty,
      bits,
      copy: true,
      dependency: Dependency::External(Feature::new("ruint_1")),
    }
  }
}
