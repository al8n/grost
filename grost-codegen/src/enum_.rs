#![allow(clippy::wrong_self_convention)]

use std::{borrow::Cow, num::NonZeroI128, sync::Arc};

use heck::ToSnakeCase as _;
use indexmap::IndexSet;
use quote::{format_ident, quote, ToTokens};
use smol_str::SmolStr;
use syn::{parse_quote, Visibility};

use super::{SafeIdent, Heck};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnumRepr {
  U8,
  U16,
  U32,
  U64,
  I8,
  I16,
  I32,
  I64,
}

impl EnumRepr {
  fn to_ty(&self) -> syn::Ident {
    match self {
      Self::U8 => parse_quote! { u8 },
      Self::U16 => parse_quote! { u16 },
      Self::U32 => parse_quote! { u32 },
      Self::U64 => parse_quote! { u64 },
      Self::I8 => parse_quote! { i8 },
      Self::I16 => parse_quote! { i16 },
      Self::I32 => parse_quote! { i32 },
      Self::I64 => parse_quote! { i64 },
    }
  }

  fn to_full_qualified_ty(&self) -> syn::Type {
    match self {
      Self::U8 => parse_quote! { ::core::primitive::u8 },
      Self::U16 => parse_quote! { ::core::primitive::u16 },
      Self::U32 => parse_quote! { ::core::primitive::u32 },
      Self::U64 => parse_quote! { ::core::primitive::u64 },
      Self::I8 => parse_quote! { ::core::primitive::i8 },
      Self::I16 => parse_quote! { ::core::primitive::i16 },
      Self::I32 => parse_quote! { ::core::primitive::i32 },
      Self::I64 => parse_quote! { ::core::primitive::i64 },
    }
  }

  fn to_encode_fn(&self, path_to_grost: &syn::Path) -> syn::Path {
    match self {
      Self::U8 => parse_quote! { #path_to_grost::__private::varing::encode_u8_varint },
      Self::U16 => parse_quote! { #path_to_grost::__private::varing::encode_u16_varint },
      Self::U32 => parse_quote! { #path_to_grost::__private::varing::encode_u32_varint },
      Self::U64 => parse_quote! { #path_to_grost::__private::varing::encode_u64_varint },
      Self::I8 => parse_quote! { #path_to_grost::__private::varing::encode_i8_varint },
      Self::I16 => parse_quote! { #path_to_grost::__private::varing::encode_i16_varint },
      Self::I32 => parse_quote! { #path_to_grost::__private::varing::encode_i32_varint },
      Self::I64 => parse_quote! { #path_to_grost::__private::varing::encode_i64_varint },
    }
  }

  fn to_decode_fn(&self, path_to_grost: &syn::Path) -> syn::Path {
    match self {
      Self::U8 => parse_quote! { #path_to_grost::__private::varing::decode_u8_varint },
      Self::U16 => parse_quote! { #path_to_grost::__private::varing::decode_u16_varint },
      Self::U32 => parse_quote! { #path_to_grost::__private::varing::decode_u32_varint },
      Self::U64 => parse_quote! { #path_to_grost::__private::varing::decode_u64_varint },
      Self::I8 => parse_quote! { #path_to_grost::__private::varing::decode_i8_varint },
      Self::I16 => parse_quote! { #path_to_grost::__private::varing::decode_i16_varint },
      Self::I32 => parse_quote! { #path_to_grost::__private::varing::decode_i32_varint },
      Self::I64 => parse_quote! { #path_to_grost::__private::varing::decode_i64_varint },
    }
  }

  fn to_encoded_len_fn(&self, path_to_grost: &syn::Path) -> syn::Path {
    match self {
      Self::U8 => parse_quote! { #path_to_grost::__private::varing::encoded_u8_varint_len },
      Self::U16 => parse_quote! { #path_to_grost::__private::varing::encoded_u16_varint_len },
      Self::U32 => parse_quote! { #path_to_grost::__private::varing::encoded_u32_varint_len },
      Self::U64 => parse_quote! { #path_to_grost::__private::varing::encoded_u64_varint_len },
      Self::I8 => parse_quote! { #path_to_grost::__private::varing::encoded_i8_varint_len },
      Self::I16 => parse_quote! { #path_to_grost::__private::varing::encoded_i16_varint_len },
      Self::I32 => parse_quote! { #path_to_grost::__private::varing::encoded_i32_varint_len },
      Self::I64 => parse_quote! { #path_to_grost::__private::varing::encoded_i64_varint_len },
    }
  }

  fn to_max_encoded_len(&self) -> proc_macro2::TokenStream {
    match self {
      Self::U8 => {
        let max = <u8 as varing::Varint>::MAX_ENCODED_LEN;
        quote! { #max }
      }
      Self::U16 => {
        let max = <u16 as varing::Varint>::MAX_ENCODED_LEN;
        quote! { #max }
      }
      Self::U32 => {
        let max = <u32 as varing::Varint>::MAX_ENCODED_LEN;
        quote! { #max }
      }
      Self::U64 => {
        let max = <u64 as varing::Varint>::MAX_ENCODED_LEN;
        quote! { #max }
      }
      Self::I8 => {
        let max = <i8 as varing::Varint>::MAX_ENCODED_LEN;
        quote! { #max }
      }
      Self::I16 => {
        let max = <i16 as varing::Varint>::MAX_ENCODED_LEN;
        quote! { #max }
      }
      Self::I32 => {
        let max = <i32 as varing::Varint>::MAX_ENCODED_LEN;
        quote! { #max }
      }
      Self::I64 => {
        let max = <i64 as varing::Varint>::MAX_ENCODED_LEN;
        quote! { #max }
      }
    }
  }

  fn to_min_encoded_len(&self) -> proc_macro2::TokenStream {
    quote! { 1usize }
  }

  fn to_encoded(&self, tag: i128) -> proc_macro2::TokenStream {
    match self {
      Self::U8 => {
        let tag = tag as u8;
        let encoded = varing::encode_u8_varint(tag);
        let buf = encoded.as_ref();
        quote! { [#(#buf),*] }
      }
      Self::U16 => {
        let tag = tag as u16;
        let encoded = varing::encode_u16_varint(tag);
        let buf = encoded.as_ref();
        quote! { [#(#buf),*] }
      }
      Self::U32 => {
        let tag = tag as u32;
        let encoded = varing::encode_u32_varint(tag);
        let buf = encoded.as_ref();
        quote! { [#(#buf),*] }
      }
      Self::U64 => {
        let tag = tag as u64;
        let encoded = varing::encode_u64_varint(tag);
        let buf = encoded.as_ref();
        quote! { [#(#buf),*] }
      }
      Self::I8 => {
        let tag = tag as i8;
        let encoded = varing::encode_i8_varint(tag);
        let buf = encoded.as_ref();
        quote! { [#(#buf),*] }
      }
      Self::I16 => {
        let tag = tag as i16;
        let encoded = varing::encode_i16_varint(tag);
        let buf = encoded.as_ref();
        quote! { [#(#buf),*] }
      }
      Self::I32 => {
        let tag = tag as i32;
        let encoded = varing::encode_i32_varint(tag);
        let buf = encoded.as_ref();
        quote! { [#(#buf),*] }
      }
      Self::I64 => {
        let tag = tag as i64;
        let encoded = varing::encode_i64_varint(tag);
        let buf = encoded.as_ref();
        quote! { [#(#buf),*] }
      }
    }
  }

  fn to_encoded_len(&self, tag: i128) -> proc_macro2::TokenStream {
    match self {
      Self::U8 => {
        let tag = tag as u8;
        let len = varing::encoded_u8_varint_len(tag);
        quote! { #len }
      }
      Self::U16 => {
        let tag = tag as u16;
        let len = varing::encoded_u16_varint_len(tag);
        quote! { #len }
      }
      Self::U32 => {
        let tag = tag as u32;
        let len = varing::encoded_u32_varint_len(tag);
        quote! { #len }
      }
      Self::U64 => {
        let tag = tag as u64;
        let len = varing::encoded_u64_varint_len(tag);
        quote! { #len }
      }
      Self::I8 => {
        let tag = tag as i8;
        let len = varing::encoded_i8_varint_len(tag);
        quote! { #len }
      }
      Self::I16 => {
        let tag = tag as i16;
        let len = varing::encoded_i16_varint_len(tag);
        quote! { #len }
      }
      Self::I32 => {
        let tag = tag as i32;
        let len = varing::encoded_i32_varint_len(tag);
        quote! { #len }
      }
      Self::I64 => {
        let tag = tag as i64;
        let len = varing::encoded_i64_varint_len(tag);
        quote! { #len }
      }
    }
  }

  fn to_value(&self, tag: i128) -> proc_macro2::TokenStream {
    match self {
      Self::U8 => {
        let tag = tag as u8;
        quote! { #tag }
      }
      Self::U16 => {
        let tag = tag as u16;
        quote! { #tag }
      }
      Self::U32 => {
        let tag = tag as u32;
        quote! { #tag }
      }
      Self::U64 => {
        let tag = tag as u64;
        quote! { #tag }
      }
      Self::I8 => {
        let tag = tag as i8;
        quote! { #tag }
      }
      Self::I16 => {
        let tag = tag as i16;
        quote! { #tag }
      }
      Self::I32 => {
        let tag = tag as i32;
        quote! { #tag }
      }
      Self::I64 => {
        let tag = tag as i64;
        quote! { #tag }
      }
    }
  }
}

impl ToTokens for EnumRepr {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    match self {
      Self::U8 => tokens.extend(quote! { #[repr(u8)] }),
      Self::U16 => tokens.extend(quote! { #[repr(u16)] }),
      Self::U32 => tokens.extend(quote! { #[repr(u32)] }),
      Self::U64 => tokens.extend(quote! { #[repr(u64)] }),
      Self::I8 => tokens.extend(quote! { #[repr(i8)] }),
      Self::I16 => tokens.extend(quote! { #[repr(i16)] }),
      Self::I32 => tokens.extend(quote! { #[repr(i32)] }),
      Self::I64 => tokens.extend(quote! { #[repr(i64)] }),
    }
  }
}

#[derive(Clone)]
pub struct EnumVariant {
  name: SafeIdent,
  description: Option<SmolStr>,
  as_str_case: Option<Heck>,
  default: bool,
  tag: NonZeroI128,
}

impl PartialEq for EnumVariant {
  fn eq(&self, other: &Self) -> bool {
    self.tag == other.tag && self.name == other.name
  }
}

impl Eq for EnumVariant {}

impl core::hash::Hash for EnumVariant {
  fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
    self.name.hash(state);
    self.tag.hash(state);
  }
}

impl EnumVariant {
  /// Creates a new enum variant.
  pub fn new(name: SafeIdent, tag: NonZeroI128) -> Self {
    Self {
      name,
      description: None,
      as_str_case: None,
      default: false,
      tag,
    }
  }

  /// Marks the enum variant is default or not.
  pub fn with_default(mut self, default: bool) -> Self {
    self.default = default;
    self
  }

  /// Sets the description of the enum variant.
  pub fn with_description(mut self, description: Option<SmolStr>) -> Self {
    self.description = description;
    self
  }

  /// Sets the as_str_case of the enum variant.
  pub fn with_as_str_case(mut self, as_str_case: Option<Heck>) -> Self {
    self.as_str_case = as_str_case;
    self
  }

  /// Returns the description of the enum.
  pub fn description(&self) -> Option<&str> {
    self.description.as_deref()
  }

  /// Returns the tag of the enum variant.
  pub fn tag(&self) -> i128 {
    self.tag.get()
  }

  /// Returns the name of the enum variant.
  pub fn name(&self) -> &SafeIdent {
    &self.name
  }

  /// Returns the case of the enum variant.
  pub fn as_str_case(&self) -> Option<&Heck> {
    self.as_str_case.as_ref()
  }

  /// Returns the default of the enum variant.
  pub fn default(&self) -> bool {
    self.default
  }
}

#[derive(Clone)]
pub struct Enum {
  name: SafeIdent,
  description: Option<SmolStr>,
  vis: Visibility,
  repr: EnumRepr,
  variants: Arc<[EnumVariant]>,
  as_str_case: Option<Heck>,
}

impl Enum {
  /// Creates a new enum.
  pub fn new(name: SafeIdent, repr: EnumRepr, variants: Vec<EnumVariant>) -> Self {
    Self {
      name,
      description: None,
      vis: parse_quote! { pub },
      repr,
      variants: Arc::from(variants),
      as_str_case: None,
    }
  }

  /// Sets the visibility of the enum.
  pub fn with_visibility(mut self, vis: Visibility) -> Self {
    self.vis = vis;
    self
  }

  /// Sets the description of the enum.
  pub fn with_description(mut self, description: Option<SmolStr>) -> Self {
    self.description = description;
    self
  }

  /// Sets the as_str_case of the enum.
  pub fn with_as_str_case(mut self, as_str_case: Option<Heck>) -> Self {
    self.as_str_case = as_str_case;
    self
  }

  /// Returns the variants of the enum.
  pub fn variants(&self) -> &[EnumVariant] {
    &self.variants
  }

  /// Returns the visibility of the enum.
  pub fn visibility(&self) -> &Visibility {
    &self.vis
  }

  /// Returns the `as_str_case` of the enum.
  pub fn as_str_case(&self) -> Option<&Heck> {
    self.as_str_case.as_ref()
  }

  /// Returns the description of the enum.
  pub fn description(&self) -> Option<&str> {
    self.description.as_deref()
  }

  pub(crate) fn enum_defination(&self) -> proc_macro2::TokenStream {
    let name = &self.name;
    let repr = &self.repr;
    let repr_ty = repr.to_full_qualified_ty();
    let document = self.description.as_deref();
    let vis = &self.vis;

    let variants = self.variants.iter().map(|v| {
      let variant_name_ident = &v.name;
      let doc = v.description();

      let default = if v.default {
        quote! { #[default] }
      } else {
        quote! {}
      };

      quote! {
        #doc
        #default
        #variant_name_ident
      }
    });

    quote! {
      #[derive(::core::marker::Copy, ::core::clone::Clone, ::core::fmt::Debug, ::core::default::Default, ::core::cmp::PartialEq, ::core::cmp::Eq, ::core::hash::Hash)]
      #document
      #repr
      #[non_exhaustive]
      #vis enum #name {
        #(#variants,)*
        /// An unknown variant of the enum, used for backwards and forwards compatibility.
        Unknown(#repr_ty),
      }
    }
  }

  pub(super) fn enum_as_str(&self) -> proc_macro2::TokenStream {
    let as_str_branches = self.variants.iter().map(|v| {
      let variant_name_ident = &v.name;
      let variant_name_str = v
        .as_str_case
        .as_ref()
        .map(|s| s.convert(v.name.original_str()))
        .or_else(|| self.as_str_case.as_ref().map(|s| s.convert(v.name.original_str())))
        .map(Cow::Owned)
        .unwrap_or(Cow::Borrowed(v.name.original_str()));

      quote! {
        Self::#variant_name_ident => #variant_name_str
      }
    });

    let name_ident = &self.name;

    quote! {
      impl ::core::fmt::Display for #name_ident {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
          write!(f, "{}", self.as_str())
        }
      }

      impl #name_ident {
        /// Returns the string representation of the enum variant.
        #[inline]
        pub const fn as_str(&self) -> &'static ::core::primitive::str {
          match self {
            #(#as_str_branches),*
          }
        }
      }
    }
  }

  pub(super) fn enum_is_variant(&self) -> proc_macro2::TokenStream {
    let is_variant_fns = self.variants.iter().map(|v| {
      let variant_name_ident = &v.name;
      let fn_name = format_ident!("is_{}", v.name.name_str().to_snake_case());
      let doc = format!(
        "Returns `true` if the enum variant is [`{}::{}`]",
        self.name.name_str(), variant_name_ident.name_str()
      );
      quote! {
        #[doc = #doc]
        #[inline]
        pub const fn #fn_name(&self) -> ::core::primitive::bool {
          ::core::matches!(self, Self::#variant_name_ident)
        }
      }
    });

    let name_ident = &self.name;

    quote! {
      impl #name_ident {
        #(#is_variant_fns)*
      }
    }
  }

  pub(super) fn enum_from_str(&self, path_to_grost: &syn::Path) -> proc_macro2::TokenStream {
    let name_ident = &self.name;
    let parse_error_name = format_ident!("Parse{}Error", name_ident.name_str());
    let parse_error_comment = format!("The error type returned when parsing [`{}`]", name_ident.name_str());
    #[cfg(not(feature = "no-alloc"))]
    let parse_error_display = format!("Fail to parse `{}`, unknown {{}}", name_ident.name_str());
    let no_string_parse_error_display =
      format!("Fail to parse `{}`, unknown variant string", name_ident.name_str());

    let try_from_branch = self.variants.iter().map(|v| {
      let name_strs = Heck::all_cases(v.name.original_str())
        .into_iter()
        .chain(Heck::all_cases(v.name.name_str()))
        .collect::<IndexSet<_>>()
        .into_iter();
      let variant_name_ident = &v.name;

      quote! {
        #(#name_strs)|* => ::core::result::Result::Ok(Self::#variant_name_ident),
      }
    });

    #[cfg(not(feature = "no-alloc"))]
    let err_branch = quote! {
      _ => ::core::result::Result::Err(#parse_error_name {
        _priv: ::std::string::ToString::to_string(src),
      }),
    };

    #[cfg(feature = "no-alloc")]
    let err_branch = quote! {
      _ => ::core::result::Result::Err(#parse_error_name {
        _priv: (),
      }),
    };

    #[cfg(not(feature = "no-alloc"))]
    let def = quote! {
      #[derive(
        ::core::clone::Clone,
        ::core::fmt::Debug,
        ::core::cmp::PartialEq,
        ::core::cmp::Eq,
        ::core::hash::Hash,
        #path_to_grost::__private::thiserror::Error
      )]
      #[error(#parse_error_display, _priv)]
      #[doc = #parse_error_comment]
      pub struct #parse_error_name {
        _priv: ::std::string::String,
      }
    };

    #[cfg(feature = "no-alloc")]
    let def = quote! {
      #[derive(
        ::core::marker::Copy,
        ::core::clone::Clone,
        ::core::fmt::Debug,
        ::core::cmp::PartialEq,
        ::core::cmp::Eq,
        ::core::hash::Hash,
        #path_to_grost::__private::thiserror::Error
      )]
      #[error(#no_string_parse_error_display)]
      #[doc = #parse_error_comment]
      pub struct #parse_error_name {
        _priv: (),
      }
    };

    quote! {
      #def

      impl ::core::str::FromStr for #name_ident {
        type Err = #parse_error_name;

        fn from_str(src: &::core::primitive::str) -> ::core::result::Result<Self, Self::Err> {
          match src.trim() {
            #(#try_from_branch)*
            #err_branch
          }
        }
      }

      impl<'a> ::core::convert::TryFrom<&'a ::core::primitive::str> for #name_ident {
        type Error = #parse_error_name;

        fn try_from(src: &'a ::core::primitive::str) -> ::core::result::Result<Self, Self::Error> {
          ::core::str::FromStr::from_str(src)
        }
      }
    }
  }

  pub(super) fn enum_quickcheck(&self, path_to_grost: &syn::Path) -> proc_macro2::TokenStream {
    let name_ident = &self.name;
    let variants = self.variants.iter().map(|v| &v.name);

    #[cfg(feature = "quickcheck")]
    let output = quote! {
      #[cfg(feature = "quickcheck")]
      const _: () = {
        use #path_to_grost::__private::quickcheck::{Arbitrary, Gen};

        impl #name_ident {
          const __QUICKCHECK_VARIANTS: &[Self] = &[
            #(Self::#variants,)*
          ];
        }

        impl Arbitrary for #name_ident {
          fn arbitrary(g: &mut Gen) -> Self {
            *g.choose(Self::__QUICKCHECK_VARIANTS).unwrap()
          }
        }
      };
    };

    #[cfg(not(feature = "quickcheck"))]
    let output = quote! {};

    output
  }

  pub(super) fn enum_arbitrary(&self, path_to_grost: &syn::Path) -> proc_macro2::TokenStream {
    let name_ident = &self.name;
    let variants = self.variants.iter().map(|v| &v.name);

    #[cfg(feature = "arbitrary")]
    let output = quote! {
      #[cfg(feature = "arbitrary")]
      const _: () = {
        use #path_to_grost::__private::arbitrary::{Arbitrary, Unstructured, Error};

        impl #name_ident {
          const __ARBITRARY_VARIANTS: &[Self] = &[
            #(Self::#variants,)*
          ];
        }

        impl Arbitrary for #name_ident {
          fn arbitrary(u: &mut Unstructured) -> ::core::result::Result<Self, Error> {
            u.choose(Self::__ARBITRARY_VARIANTS).map(|v| *v)
          }
        }
      };
    };

    #[cfg(not(feature = "arbitrary"))]
    let output = quote! {};

    output
  }

  pub(super) fn enum_repr_conversion(&self, path_to_grost: &syn::Path) -> proc_macro2::TokenStream {
    let name_ident = &self.name;
    let repr = &self.repr;
    let repr_fq_ty = repr.to_full_qualified_ty();
    let repr_ty = repr.to_ty();

    let repr_encode_fn = repr.to_encode_fn(path_to_grost);
    let repr_decode_fn = repr.to_decode_fn(path_to_grost);
    let repr_encoded_len_fn = repr.to_encoded_len_fn(path_to_grost);

    let repr_max_encoded_len = repr.to_max_encoded_len();
    let repr_min_encoded_len = repr.to_min_encoded_len();

    let from_fn_name = format_ident!("from_{}", repr_ty);
    let to_fn_name = format_ident!("to_{}", repr_ty);

    let const_encode_branches = self.variants.iter().map(|v| {
      let variant_name_ident = &v.name;
      let encoded = repr.to_encoded(v.tag());

      quote! {
        Self::#variant_name_ident => &#encoded,
      }
    });

    let const_decode_branches = self.variants.iter().map(|v| {
      let variant_name_ident = &v.name;
      let encoded = repr.to_encoded(v.tag());

      quote! {
        #encoded => (#encoded.len(), Self::#variant_name_ident),
      }
    });

    let const_encoded_len_branches = self.variants.iter().map(|v| {
      let variant_name_ident = &v.name;
      let encoded_len = repr.to_encoded_len(v.tag());

      quote! {
        Self::#variant_name_ident => #encoded_len,
      }
    });

    let from_branches = self.variants.iter().map(|v| {
      let variant_name_ident = &v.name;
      let value = repr.to_value(v.tag());

      quote! {
        #value => Self::#variant_name_ident,
      }
    });

    let to_branches = self.variants.iter().map(|v| {
      let variant_name_ident = &v.name;
      let value = repr.to_value(v.tag());

      quote! {
        Self::#variant_name_ident => #value,
      }
    });

    quote! {
      impl ::core::convert::From<#repr_fq_ty> for #name_ident {
        #[inline]
        fn from(repr: #repr_fq_ty) -> Self {
          Self::#from_fn_name(repr)
        }
      }

      impl ::core::convert::From<#name_ident> for #repr_fq_ty {
        #[inline]
        fn from(e: #name_ident) -> Self {
          e.#to_fn_name()
        }
      }

      impl<'a> ::core::convert::TryFrom<&'a [::core::primitive::u8]> for #name_ident {
        type Error = #path_to_grost::__private::DecodeError;

        fn try_from(src: &'a [::core::primitive::u8]) -> ::core::result::Result<Self, Self::Error> {
          Self::const_decode(src).map(|(_, this)| this)
        }
      }

      impl #name_ident {
        /// The maximum encoded length in bytes.
        pub const MAX_ENCODED_LEN: usize = #repr_max_encoded_len;
        /// The minimum encoded length in bytes.
        pub const MIN_ENCODED_LEN: usize = #repr_min_encoded_len;

        /// Returns the enum variant from the raw representation.
        #[inline]
        pub const fn #from_fn_name(repr: #repr_fq_ty) -> Self {
          match repr {
            #(#from_branches)*
            val => Self::Unknown(val),
          }
        }

        /// Returns the raw representation of the enum variant.
        #[inline]
        pub const fn #to_fn_name(&self) -> #repr_fq_ty {
          match self {
            #(#to_branches)*
            Self::Unknown(val) => *val,
          }
        }

        /// Returns the encoded bytes of the enum variant.
        ///
        /// If self is not `Unknown`, then `Either::Left` will be returned,
        /// Otherwise, `Either::Right` will be returned.
        #[inline]
        pub const fn const_encode(&self) -> #path_to_grost::__private::either::Either<&'static [::core::primitive::u8], #path_to_grost::__private::varing::utils::Buffer<{ #repr_max_encoded_len + 1 }>> {
          ::either::Either::Left(match self {
            #(#const_encode_branches)*
            Self::Unknown(val) => {
              return ::either::Either::Right(#repr_encode_fn(*val));
            },
          })
        }

        /// Decodes the enum variant from the encoded bytes.
        ///
        /// Returns the number of bytes readed and `Self`.
        #[inline]
        pub const fn const_decode(src: &[::core::primitive::u8]) -> ::core::result::Result<(::core::primitive::usize, Self), #path_to_grost::__private::DecodeError> {
          ::core::result::Result::Ok(match src {
            #(#const_decode_branches)*
            src => {
              return match #repr_decode_fn(src) {
                ::core::result::Result::Ok((read, val)) => ::core::result::Result::Ok((read, Self::Unknown(val))),
                ::core::result::Result::Err(e) => ::core::result::Result::Err(::grost::__private::DecodeError::from_varint_error(e)),
              };
            },
          })
        }

        /// Returns the encoded length of the enum variant.
        #[inline]
        pub const fn const_encoded_len(&self) -> ::core::primitive::usize {
          match self {
            #(#const_encoded_len_branches)*
            Self::Unknown(val) => #repr_encoded_len_fn(*val),
          }
        }
      }
    }
  }

  pub(super) fn enum_codec(&self, path_to_grost: &syn::Path) -> proc_macro2::TokenStream {
    let name_ident = &self.name;

    #[cfg(not(feature = "no-alloc"))]
    let decode_owned = quote! {
      impl #path_to_grost::__private::DecodeOwned for #name_ident
      {
        #[inline]
        fn decode_from_bytes<U>(
          src: #path_to_grost::__private::bytes::Bytes,
          _: &mut U,
        ) -> ::core::result::Result<(::core::primitive::usize, Self), #path_to_grost::__private::DecodeError>
        where
          Self: ::core::marker::Sized + 'static,
          U: #path_to_grost::__private::UnknownBuffer<#path_to_grost::__private::bytes::Bytes>,
        {
          Self::const_decode(&src)
        }
      }
    };

    #[cfg(feature = "no-alloc")]
    let decode_owned = quote! {
      impl #path_to_grost::__private::DecodeOwned for #name_ident {}
    };

    quote! {
      impl #path_to_grost::__private::Wirable for #name_ident {
        const WIRE_TYPE: #path_to_grost::__private::WireType = #path_to_grost::__private::WireType::Varint;
      }

      impl #path_to_grost::__private::Encode for #name_ident {
        fn encode(&self, buf: &mut [::core::primitive::u8]) -> ::core::result::Result<::core::primitive::usize, #path_to_grost::__private::EncodeError> {
          let encoded = self.const_encode();
          let len = encoded.len();
          let buf_len = buf.len();
          if buf_len < len {
            return ::core::result::Result::Err(::grost::__private::EncodeError::insufficient_buffer(len, buf_len));
          }
          buf[..len].copy_from_slice(encoded);
          ::core::result::Result::Ok(len)
        }

        #[inline]
        fn encoded_len(&self) -> ::core::primitive::usize {
          self.const_encoded_len()
        }
      }

      impl #path_to_grost::__private::PartialEncode for #name_ident {
        type Selection = ();

        #[inline]
        fn partial_encode(&self, buf: &mut [::core::primitive::u8], _: &Self::Selection) -> ::core::result::Result<::core::primitive::usize, #path_to_grost::__private::EncodeError> {
          #path_to_grost::__private::Encode::encode(self, buf)
        }

        #[inline]
        fn partial_encoded_len(&self, _: &Self::Selection,) -> ::core::primitive::usize {
          #path_to_grost::__private::Encode::encoded_len(self)
        }
      }

      impl<'de> #path_to_grost::__private::Decode<'de> for #name_ident {
        #[inline]
        fn decode(src: &'de [::core::primitive::u8]) -> ::core::result::Result<(::core::primitive::usize, Self), #path_to_grost::__private::DecodeError> {
          Self::const_decode(src)
        }
      }

      #decode_owned 

      impl #path_to_grost::__private::IntoTarget<Self> for #name_ident {
        fn into_target(self) -> ::core::result::Result<Self, #path_to_grost::__private::DecodeError> {
          ::core::result::Result::Ok(self)
        }
      }

      impl #path_to_grost::__private::TypeRef<Self> for #name_ident {
        fn to(&self) -> ::core::result::Result<Self, #path_to_grost::__private::DecodeError> {
          ::core::result::Result::Ok(*self)
        }
      }

      impl #path_to_grost::__private::TypeOwned<Self> for #name_ident {
        fn to(&self) -> ::core::result::Result<Self, #path_to_grost::__private::DecodeError> {
          ::core::result::Result::Ok(*self)
        }
      }

      impl<'a> ::core::convert::From<&'a Self> for #name_ident {
        fn from(e: &'a Self) -> Self {
          *e
        }
      }

      impl #path_to_grost::__private::Message for #name_ident {
        type Serialized<'a> = Self where Self: Sized + 'a;
        type Borrowed<'a> = Self where Self: 'a;
        type SerializedOwned = Self where Self: Sized;
      }
    }
  }
}

