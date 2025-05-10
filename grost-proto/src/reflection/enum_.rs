use core::num::{
  NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64,
};

/// The `repr` for unit enum
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, derive_more::IsVariant)]
pub enum EnumRepr {
  /// `repr(u8)` representation
  U8,
  /// `repr(u16)` representation
  U16,
  /// `repr(u32)` representation
  U32,
  /// `repr(u64)` representation
  U64,
  /// `repr(i8)` representation
  I8,
  /// `repr(i16)` representation
  I16,
  /// `repr(i32)` representation
  I32,
  /// `repr(i64)` representation
  I64,
}

impl EnumRepr {
  /// Returns the meta repr
  #[inline]
  pub const fn to_attribute_str(&self) -> &'static str {
    match self {
      Self::U8 => "#[repr(u8)]",
      Self::U16 => "#[repr(u16)]",
      Self::U32 => "#[repr(u32)]",
      Self::U64 => "#[repr(u64)]",
      Self::I8 => "#[repr(i8)]",
      Self::I16 => "#[repr(i16)]",
      Self::I32 => "#[repr(i32)]",
      Self::I64 => "#[repr(i64)]",
    }
  }

  /// Returns the type of the corresponding repr
  #[inline]
  pub const fn to_type_str(&self) -> &'static str {
    match self {
      Self::U8 => "u8",
      Self::U16 => "u16",
      Self::U32 => "u32",
      Self::U64 => "u64",
      Self::I8 => "i8",
      Self::I16 => "i16",
      Self::I32 => "i32",
      Self::I64 => "i64",
    }
  }

  /// Returns the full qualified type of the corresponding repr
  #[inline]
  pub const fn to_full_qualified_type_str(&self) -> &'static str {
    match self {
      Self::U8 => "::core::primitive::u8",
      Self::U16 => "::core::primitive::u16",
      Self::U32 => "::core::primitive::u32",
      Self::U64 => "::core::primitive::u64",
      Self::I8 => "::core::primitive::i8",
      Self::I16 => "::core::primitive::i16",
      Self::I32 => "::core::primitive::i32",
      Self::I64 => "::core::primitive::i64",
    }
  }
}

#[cfg(feature = "codegen")]
const _: () = {
  use quote::format_ident;

  impl EnumRepr {
    /// Returns the attribute of the corresponding repr
    #[inline]
    pub fn to_attribute(&self) -> syn::Attribute {
      match self {
        Self::U8 => syn::parse_quote!(#[repr(u8)]),
        Self::U16 => syn::parse_quote!(#[repr(u16)]),
        Self::U32 => syn::parse_quote!(#[repr(u32)]),
        Self::U64 => syn::parse_quote!(#[repr(u64)]),
        Self::I8 => syn::parse_quote!(#[repr(i8)]),
        Self::I16 => syn::parse_quote!(#[repr(i16)]),
        Self::I32 => syn::parse_quote!(#[repr(i32)]),
        Self::I64 => syn::parse_quote!(#[repr(i64)]),
      }
    }

    /// Returns the variant ident of the corresponding repr
    ///
    /// e.g.
    /// If the value is `EnumRepr::U8`, the ident will be `U8`.
    #[inline]
    pub fn to_variant_ident(&self) -> syn::Ident {
      match self {
        Self::U8 => format_ident!("U8"),
        Self::U16 => format_ident!("U16"),
        Self::U32 => format_ident!("U32"),
        Self::U64 => format_ident!("U64"),
        Self::I8 => format_ident!("I8"),
        Self::I16 => format_ident!("I16"),
        Self::I32 => format_ident!("I32"),
        Self::I64 => format_ident!("I64"),
      }
    }

    /// Returns the full qualified type of the corresponding repr
    #[inline]
    pub fn to_full_qualified_type(&self) -> syn::Type {
      syn::parse_str(self.to_full_qualified_type_str()).unwrap()
    }

    /// Returns the type of the corresponding repr
    #[inline]
    pub fn to_type(&self) -> syn::Ident {
      format_ident!("{}", self.to_type_str())
    }
  }

  impl EnumVariantValue {
    /// Returns the variant ident of the corresponding repr
    ///
    /// e.g.
    ///
    /// If the value is `EnumVariantValue::U8(NonZeroU8(1))`, the ident will be `U8`.
    pub fn to_variant_ident(&self) -> syn::Ident {
      match self {
        Self::U8(_) => format_ident!("U8"),
        Self::U16(_) => format_ident!("U16"),
        Self::U32(_) => format_ident!("U32"),
        Self::U64(_) => format_ident!("U64"),
        Self::I8(_) => format_ident!("I8"),
        Self::I16(_) => format_ident!("I16"),
        Self::I32(_) => format_ident!("I32"),
        Self::I64(_) => format_ident!("I64"),
      }
    }

    /// Returns the varint encoded value of the corresponding repr
    ///
    /// e.g. If the value is `EnumVariantValue::U8(NonZeroU8(1))`, the expr will be the output `::grost::__private::flavors::varint::encode_u8_varint(1)` (`[1u8]` in this example)
    pub fn to_encoded_value_varint(&self) -> syn::Expr {
      macro_rules! to_expr {
        ($ty:ident::$value:ident) => {{
          paste::paste! {
            let tag = $value.get();
            let encoded = varing::[< encode_ $ty _varint >](tag);
            let buf = encoded.as_ref();
            syn::parse_quote! { [#(#buf),*] }
          }
        }};
      }

      match self {
        Self::U8(value) => to_expr!(u8::value),
        Self::U16(value) => to_expr!(u16::value),
        Self::U32(value) => to_expr!(u32::value),
        Self::U64(value) => to_expr!(u64::value),
        Self::I8(value) => to_expr!(i8::value),
        Self::I16(value) => to_expr!(i16::value),
        Self::I32(value) => to_expr!(i32::value),
        Self::I64(value) => to_expr!(i64::value),
      }
    }

    /// Returns the literal to the value.
    ///
    /// e.g.
    ///
    /// If the value is `EnumVariantValue::U8(NonZeroU8(1))`, the expr will be `1u8`
    #[inline]
    pub fn to_value(&self) -> syn::Lit {
      macro_rules! to_expr {
        ($value:ident) => {{
          let value = $value.get();
          syn::parse_quote! {
            #value
          }
        }};
      }

      match self {
        Self::U8(value) => to_expr!(value),
        Self::U16(value) => to_expr!(value),
        Self::U32(value) => to_expr!(value),
        Self::U64(value) => to_expr!(value),
        Self::I8(value) => to_expr!(value),
        Self::I16(value) => to_expr!(value),
        Self::I32(value) => to_expr!(value),
        Self::I64(value) => to_expr!(value),
      }
    }

    /// Returns the expr to the value.
    ///
    /// e.g.
    ///
    /// If the value is `EnumVariantValue::U8(NonZeroU8(1))`, the expr will be `::core::num::NonZeroU8::new(1).unwrap()`
    #[inline]
    pub fn to_non_zero_value(&self) -> syn::Expr {
      macro_rules! to_expr {
        ($ty:ident, $value:ident) => {{
          let value = $value.get();
          syn::parse_quote! {
            ::core::num::$ty::new(#value).unwrap()
          }
        }};
      }

      match self {
        Self::U8(value) => to_expr!(NonZeroU8, value),
        Self::U16(value) => to_expr!(NonZeroU16, value),
        Self::U32(value) => to_expr!(NonZeroU32, value),
        Self::U64(value) => to_expr!(NonZeroU64, value),
        Self::I8(value) => to_expr!(NonZeroI8, value),
        Self::I16(value) => to_expr!(NonZeroI16, value),
        Self::I32(value) => to_expr!(NonZeroI32, value),
        Self::I64(value) => to_expr!(NonZeroI64, value),
      }
    }
  }
};

/// The value of an unit enum variant
#[derive(
  Clone,
  Copy,
  Debug,
  PartialEq,
  Eq,
  Hash,
  derive_more::IsVariant,
  derive_more::Unwrap,
  derive_more::TryUnwrap,
)]
pub enum EnumVariantValue {
  /// `u8` value, `repr(u8)`
  U8(NonZeroU8),
  /// `u16` value, `repr(u16)`
  U16(NonZeroU16),
  /// `u32` value, `repr(u32)`
  U32(NonZeroU32),
  /// `u64` value, `repr(u64)`
  U64(NonZeroU64),
  /// `i8` value, `repr(i8)`
  I8(NonZeroI8),
  /// `i16` value, `repr(i16)`
  I16(NonZeroI16),
  /// `i32` value, `repr(i32)`
  I32(NonZeroI32),
  /// `i64` value, `repr(i64)`
  I64(NonZeroI64),
}

#[doc(hidden)]
pub struct EnumVariantReflectionBuilder {
  pub name: &'static str,
  pub schema_name: &'static str,
  pub description: &'static str,
  pub value: EnumVariantValue,
}

impl EnumVariantReflectionBuilder {
  #[inline]
  pub const fn build(self) -> EnumVariantReflection {
    EnumVariantReflection {
      name: self.name,
      schema_name: self.schema_name,
      description: self.description,
      value: self.value,
    }
  }
}

/// The information of a variant of enum in the Graph protocol buffer
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EnumVariantReflection {
  name: &'static str,
  schema_name: &'static str,
  description: &'static str,
  value: EnumVariantValue,
}

impl EnumVariantReflection {
  /// Get the name of the variant
  #[inline]
  pub const fn name(&self) -> &'static str {
    self.name
  }

  /// Get the schema name of the variant
  #[inline]
  pub const fn schema_name(&self) -> &'static str {
    self.schema_name
  }

  /// Get the value of the variant
  #[inline]
  pub const fn value(&self) -> EnumVariantValue {
    self.value
  }

  /// Get the description of the variant
  #[inline]
  pub const fn description(&self) -> &'static str {
    self.description
  }
}

#[doc(hidden)]
pub struct EnumReflectionBuilder {
  pub name: &'static str,
  pub schema_name: &'static str,
  pub variants: &'static [&'static EnumVariantReflection],
  pub description: &'static str,
  pub repr: EnumRepr,
}

impl EnumReflectionBuilder {
  #[inline]
  pub const fn build(self) -> EnumReflection {
    EnumReflection {
      name: self.name,
      schema_name: self.schema_name,
      variants: self.variants,
      description: self.description,
      repr: self.repr,
    }
  }
}

/// The information of an enum in the Graph protocol buffer
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EnumReflection {
  name: &'static str,
  schema_name: &'static str,
  description: &'static str,
  variants: &'static [&'static EnumVariantReflection],
  repr: EnumRepr,
}

impl EnumReflection {
  /// Get the name of the enum
  #[inline]
  pub const fn name(&self) -> &'static str {
    self.name
  }

  /// Get the schema name of the enum
  #[inline]
  pub const fn schema_name(&self) -> &'static str {
    self.schema_name
  }

  /// Get the variants of the enum
  #[inline]
  pub const fn variants(&self) -> &'static [&'static EnumVariantReflection] {
    self.variants
  }

  /// Get the description of the enum
  #[inline]
  pub const fn description(&self) -> &'static str {
    self.description
  }

  /// Get the representation of the enum
  #[inline]
  pub const fn repr(&self) -> EnumRepr {
    self.repr
  }
}
