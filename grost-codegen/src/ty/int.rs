use cfg_if::cfg_if;
use syn::{Type, parse_quote};

use super::{Dependency, Feature};

/// Invalid signed integer width
#[derive(Clone, Debug, thiserror::Error)]
pub enum InvalidIntWidth {
  #[error("signed integer width {0} is not supported")]
  Unsupported(usize),
  #[error("signed integer width {bits} is supported, but required feature: {feature}")]
  RequireFeature { bits: usize, feature: Feature },
}

impl InvalidIntWidth {
  /// Returns the width of the integer
  pub fn width(&self) -> usize {
    match self {
      Self::Unsupported(width) => *width,
      Self::RequireFeature { bits, .. } => *bits,
    }
  }

  /// Creates a new `InvalidIntWidth::Unsupported` error
  pub fn unsupported(width: usize) -> Self {
    Self::Unsupported(width)
  }

  /// Creates a new `InvalidIntWidth::RequireFeature` error
  pub fn require_feature(bits: usize, feature: Feature) -> Self {
    Self::RequireFeature { bits, feature }
  }
}

pub struct Int {
  ty: Type,
  bits: usize,
  copy: bool,
  dependency: Dependency,
}

impl Int {
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

  /// Returns an `i8`
  #[inline]
  pub fn i8() -> Self {
    Self::new_copy(parse_quote!("::core::primitive::i8"), 8, Dependency::None)
  }

  /// Returns an `i16`
  #[inline]
  pub fn i16() -> Self {
    Self::new_copy(parse_quote!("::core::primitive::i16"), 16, Dependency::None)
  }

  /// Returns an `i32`
  #[inline]
  pub fn i32() -> Self {
    Self::new_copy(parse_quote!("::core::primitive::i32"), 32, Dependency::None)
  }

  /// Returns an `i64`
  #[inline]
  pub fn i64() -> Self {
    Self::new_copy(parse_quote!("::core::primitive::i64"), 64, Dependency::None)
  }

  /// Returns an `i128`
  #[inline]
  pub fn i128() -> Self {
    Self::new_copy(
      parse_quote!("::core::primitive::i128"),
      128,
      Dependency::None,
    )
  }

  /// Returns an `iN` which will use the builtin support of `grost`
  #[allow(unused_variables)]
  #[inline]
  pub fn i_n(bits: usize, path_to_grost: &syn::Path) -> Result<Self, InvalidIntWidth> {
    Ok(match bits {
      #[cfg(feature = "bnum_0_13")]
      0 => Self::bnum_0_13(
        parse_quote!(#path_to_grost::__private::bnum_0_13::BIntD8<0>),
        0,
      ),
      8 => Self::i8(),
      16 => Self::i16(),
      #[cfg(feature = "bnum_0_13")]
      24 => Self::bnum_0_13(
        parse_quote!(#path_to_grost::__private::bnum_0_13::BIntD8<3>),
        24,
      ),
      32 => Self::i32(),
      33..=63 => match () {
        () if bits % 16 == 0 => {
          cfg_if! {
            if #[cfg(feature = "bnum_0_13")] {
              let limbs = bits / 16;
              Self::bnum_0_13(parse_quote!(#path_to_grost::__private::bnum_0_13::BIntD16<#limbs>), bits)
            } else {
              return Err(InvalidIntWidth::require_feature(bits, Feature::new("bnum_0_13")));
            }
          }
        }
        () if bits % 8 == 0 => {
          cfg_if! {
            if #[cfg(feature = "bnum_0_13")] {
              let limbs = bits / 8;
              Self::bnum_0_13(parse_quote!(#path_to_grost::__private::bnum_0_13::BIntD8<#limbs>), bits)
            } else {
              return Err(InvalidIntWidth::require_feature(bits, Feature::new("bnum_0_13")));
            }
          }
        }
        _ => {
          return Err(InvalidIntWidth::unsupported(bits));
        }
      },
      64 => Self::i64(),
      65..=127 => match () {
        () if bits % 32 == 0 => {
          cfg_if! {
            if #[cfg(feature = "bnum_0_13")] {
              let limbs = bits / 32;
              Self::bnum_0_13(parse_quote!(#path_to_grost::__private::bnum_0_13::BIntD32<#limbs>), bits)
            } else {
              return Err(InvalidIntWidth::require_feature(bits, Feature::new("bnum_0_13")));
            }
          }
        }
        () if bits % 16 == 0 => {
          cfg_if! {
            if #[cfg(feature = "bnum_0_13")] {
              let limbs = bits / 16;
              Self::bnum_0_13(parse_quote!(#path_to_grost::__private::bnum_0_13::BIntD16<#limbs>), bits)
            } else {
              return Err(InvalidIntWidth::require_feature(bits, Feature::new("bnum_0_13")));
            }
          }
        }
        () if bits % 8 == 0 => {
          cfg_if! {
            if #[cfg(feature = "bnum_0_13")] {
              let limbs = bits / 8;
              Self::bnum_0_13(parse_quote!(#path_to_grost::__private::bnum_0_13::BIntD8<#limbs>), bits)
            } else {
              return Err(InvalidIntWidth::require_feature(bits, Feature::new("bnum_0_13")));
            }
          }
        }
        _ => {
          return Err(InvalidIntWidth::unsupported(bits));
        }
      },
      128 => Self::i128(),
      _ => match () {
        () if bits % 64 == 0 => {
          cfg_if! {
            if #[cfg(feature = "bnum_0_13")] {
              let limbs = bits / 64;
              Self::bnum_0_13(parse_quote!(#path_to_grost::__private::bnum_0_13::BIntD64<#limbs>), bits)
            } else {
              return Err(InvalidIntWidth::require_feature(bits, Feature::new("bnum_0_13")));
            }
          }
        }
        () if bits % 32 == 0 => {
          cfg_if! {
            if #[cfg(feature = "bnum_0_13")] {
              let limbs = bits / 32;
              Self::bnum_0_13(parse_quote!(#path_to_grost::__private::bnum_0_13::BIntD32<#limbs>), bits)
            } else {
              return Err(InvalidIntWidth::require_feature(bits, Feature::new("bnum_0_13")));
            }
          }
        }
        () if bits % 16 == 0 => {
          cfg_if! {
            if #[cfg(feature = "bnum_0_13")] {
              let limbs = bits / 16;
              Self::bnum_0_13(parse_quote!(#path_to_grost::__private::bnum_0_13::BIntD16<#limbs>), bits)
            } else {
              return Err(InvalidIntWidth::require_feature(bits, Feature::new("bnum_0_13")));
            }
          }
        }
        () if bits % 8 == 0 => {
          cfg_if! {
            if #[cfg(feature = "bnum_0_13")] {
              let limbs = bits / 8;
              Self::bnum_0_13(parse_quote!(#path_to_grost::__private::bnum_0_13::BIntD8<#limbs>), bits)
            } else {
              return Err(InvalidIntWidth::require_feature(bits, Feature::new("bnum_0_13")));
            }
          }
        }
        _ => {
          return Err(InvalidIntWidth::unsupported(bits));
        }
      },
    })
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
}
