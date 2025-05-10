#![allow(clippy::wrong_self_convention)]

use heck::{ToShoutySnakeCase, ToSnakeCase as _, ToUpperCamelCase};
use indexmap::IndexSet;
use quote::{format_ident, quote};
use smol_str::SmolStr;
use std::{borrow::Cow, sync::Arc};
use syn::{Ident, Visibility, parse_quote};

use super::{Heck, SafeIdent};

pub use grost_proto::reflection::{EnumRepr, EnumVariantValue};

mod reflection;

trait EnumReprExt {
  fn to_encode_fn(&self, path_to_grost: &syn::Path) -> syn::Path;
  fn to_encode_to_fn(&self, path_to_grost: &syn::Path) -> syn::Path;
  fn to_decode_fn(&self, path_to_grost: &syn::Path) -> syn::Path;
  fn to_encode_len_fn(&self, path_to_grost: &syn::Path) -> syn::Path;
  fn to_max_encoded_len(&self) -> proc_macro2::TokenStream;
  fn to_min_encoded_len(&self) -> proc_macro2::TokenStream;
}

impl EnumReprExt for EnumRepr {
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

  fn to_encode_to_fn(&self, path_to_grost: &syn::Path) -> syn::Path {
    match self {
      Self::U8 => parse_quote! { #path_to_grost::__private::varing::encode_u8_varint_to },
      Self::U16 => parse_quote! { #path_to_grost::__private::varing::encode_u16_varint_to },
      Self::U32 => parse_quote! { #path_to_grost::__private::varing::encode_u32_varint_to },
      Self::U64 => parse_quote! { #path_to_grost::__private::varing::encode_u64_varint_to },
      Self::I8 => parse_quote! { #path_to_grost::__private::varing::encode_i8_varint_to },
      Self::I16 => parse_quote! { #path_to_grost::__private::varing::encode_i16_varint_to },
      Self::I32 => parse_quote! { #path_to_grost::__private::varing::encode_i32_varint_to },
      Self::I64 => parse_quote! { #path_to_grost::__private::varing::encode_i64_varint_to },
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

  fn to_encode_len_fn(&self, path_to_grost: &syn::Path) -> syn::Path {
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
}

#[derive(Clone)]
pub struct EnumVariant {
  name: SafeIdent,
  schema_name: SmolStr,
  description: Option<SmolStr>,
  as_str_case: Option<Heck>,
  default: bool,
  value: EnumVariantValue,
}

impl PartialEq for EnumVariant {
  fn eq(&self, other: &Self) -> bool {
    self.value == other.value && self.name == other.name
  }
}

impl Eq for EnumVariant {}

impl core::hash::Hash for EnumVariant {
  fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
    self.name.hash(state);
    self.value.hash(state);
  }
}

impl EnumVariant {
  /// Creates a new enum variant.
  pub fn new(name: SafeIdent, value: EnumVariantValue) -> Self {
    Self {
      schema_name: name.original_str().into(),
      name,
      description: None,
      as_str_case: None,
      default: false,
      value,
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

  /// Sets the schema name of the enum variant.
  pub fn with_schema_name(mut self, schema_name: impl Into<SmolStr>) -> Self {
    self.schema_name = schema_name.into();
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
  pub fn value(&self) -> EnumVariantValue {
    self.value
  }

  /// Returns the name of the enum variant.
  pub fn name(&self) -> &SafeIdent {
    &self.name
  }

  /// Returns the schema name of the enum variant.
  pub fn schema_name(&self) -> &str {
    self.schema_name.as_str()
  }

  /// Returns the const variant name of the enum variant.
  ///
  /// e.g. `Color::Red` will be `RED`
  pub fn const_variant_name(&self) -> Ident {
    format_ident!("{}", self.name.name_str().to_shouty_snake_case())
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
  schema_name: SmolStr,
  description: Option<SmolStr>,
  vis: Visibility,
  repr: EnumRepr,
  variants: Arc<[EnumVariant]>,
  as_str_case: Option<Heck>,
}

impl PartialEq for Enum {
  fn eq(&self, other: &Self) -> bool {
    self.name.name().eq(other.name.name())
  }
}

impl Eq for Enum {}

impl core::hash::Hash for Enum {
  fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
    self.name.name().hash(state);
  }
}

impl Enum {
  /// Creates a new enum.
  pub fn new(name: SafeIdent, repr: EnumRepr, variants: Vec<EnumVariant>) -> Self {
    Self {
      schema_name: name.original_str().into(),
      name,
      description: None,
      vis: parse_quote! { pub },
      repr,
      variants: Arc::from(variants),
      as_str_case: None,
    }
  }

  /// Sets the schema name of the enum.
  pub fn with_schema_name(mut self, schema_name: impl Into<SmolStr>) -> Self {
    self.schema_name = schema_name.into();
    self
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

  /// Returns the name of the enum.
  pub fn name(&self) -> &SafeIdent {
    &self.name
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

  /// Returns the shema name of the enum.
  pub fn schema_name(&self) -> &str {
    self.schema_name.as_str()
  }

  /// Returns the repr of the enum.
  pub fn repr(&self) -> &EnumRepr {
    &self.repr
  }

  /// Returns the parse enum error name.
  pub fn parse_error_name(&self) -> Ident {
    format_ident!(
      "Parse{}Error",
      self.name.original_str().to_upper_camel_case()
    )
  }

  /// Returns the name of the reflection struct of this enum.
  pub fn reflection_name(&self) -> Ident {
    format_ident!(
      "{}Reflection",
      self.name.original_str().to_upper_camel_case()
    )
  }

  /// Returns the variant name of the reflection struct of this enum variants.
  pub fn variant_reflection_name(&self) -> Ident {
    format_ident!(
      "{}VariantReflection",
      self.name.original_str().to_upper_camel_case()
    )
  }

  pub(super) fn enum_definations(&self) -> proc_macro2::TokenStream {
    let name = &self.name;
    let repr = &self.repr;
    let meta_repr = repr.to_attribute();
    let repr_ty = repr.to_full_qualified_type();
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

    let parse_error_name = self.parse_error_name();
    let parse_error_comment = format!(
      "The error type returned when parsing [`{}`]",
      name.name_str()
    );

    #[cfg(any(feature = "alloc", feature = "std"))]
    let def = quote! {
      #[derive(
        ::core::clone::Clone,
        ::core::fmt::Debug,
        ::core::cmp::PartialEq,
        ::core::cmp::Eq,
        ::core::hash::Hash,
      )]
      #[doc = #parse_error_comment]
      pub struct #parse_error_name {
        _priv: ::std::string::String,
      }
    };

    #[cfg(not(any(feature = "alloc", feature = "std")))]
    let def = quote! {
      #[derive(
        ::core::marker::Copy,
        ::core::clone::Clone,
        ::core::fmt::Debug,
        ::core::cmp::PartialEq,
        ::core::cmp::Eq,
        ::core::hash::Hash,
      )]
      #[doc = #parse_error_comment]
      pub struct #parse_error_name {
        _priv: (),
      }
    };

    quote! {
      #def

      #[derive(::core::marker::Copy, ::core::clone::Clone, ::core::fmt::Debug, ::core::default::Default, ::core::cmp::PartialEq, ::core::cmp::Eq, ::core::hash::Hash)]
      #document
      #meta_repr
      #[non_exhaustive]
      #vis enum #name {
        #(#variants,)*
        /// An unknown variant of the enum, used for backwards and forwards compatibility.
        Unknown(#repr_ty),
      }
    }
  }

  pub(super) fn enum_as_str(&self) -> proc_macro2::TokenStream {
    let repr_ty = self.repr.to_full_qualified_type();
    let as_str_branches = self.variants.iter().map(|v| {
      let variant_name_ident = &v.name;
      let variant_name_str = v
        .as_str_case
        .as_ref()
        .map(|s| s.convert(v.name.original_str()))
        .or_else(|| {
          self
            .as_str_case
            .as_ref()
            .map(|s| s.convert(v.name.original_str()))
        })
        .map(Cow::Owned)
        .unwrap_or(Cow::Borrowed(v.name.original_str()));

      quote! {
        Self::#variant_name_ident => #variant_name_str,
      }
    });

    let name_ident = &self.name;
    let unknown_str = self
      .as_str_case
      .unwrap_or(Heck::UpperCamel)
      .convert("Unknown");
    let unknown = format!("{unknown_str}({{}})");

    quote! {
      #[automatically_derived]
      impl ::core::fmt::Display for #name_ident {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
          match self.try_as_str() {
            ::core::result::Result::Ok(variant_str) => {
              ::core::write!(f, "{variant_str}")
            }
            ::core::result::Result::Err(val) => {
              ::core::write!(f, #unknown, val)
            }
          }
        }
      }

      #[automatically_derived]
      impl #name_ident {
        /// Try to return the enum variant as a `str`, if the variant is unknown, it will return the value of the unknown variant.
        #[inline]
        pub const fn try_as_str(&self) -> ::core::result::Result<&'static ::core::primitive::str, #repr_ty> {
          ::core::result::Result::Ok(match self {
            #(#as_str_branches)*
            Self::Unknown(val) => return ::core::result::Result::Err(*val),
          })
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
        self.name.name_str(),
        variant_name_ident.name_str()
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
      #[automatically_derived]
      impl #name_ident {
        #(#is_variant_fns)*
      }
    }
  }

  pub(super) fn enum_from_str(&self) -> proc_macro2::TokenStream {
    let name_ident = &self.name;
    let parse_error_name = self.parse_error_name();

    #[cfg(any(feature = "alloc", feature = "std"))]
    let error_display = {
      let err_display = format!("Fail to parse `{}`, unknown {{}}", name_ident.name_str());
      quote! {
        ::core::write!(f, #err_display, self._priv)
      }
    };
    #[cfg(not(any(feature = "alloc", feature = "std")))]
    let error_display = {
      let err_display = format!(
        "Fail to parse `{}`, unknown variant string",
        name_ident.name_str()
      );
      quote! {
        ::core::write!(f, #err_display)
      }
    };

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

    let repr_ty = self.repr.to_full_qualified_type();

    #[cfg(any(feature = "alloc", feature = "std"))]
    let err_branch = quote! {
      val => {
        if let ::core::option::Option::Some(remaining) = val.strip_prefix("Unknown(").or_else(|| val.strip_prefix("unknown(")) {
          if let ::core::option::Option::Some(remaining) = remaining.strip_suffix(')') {
            if let ::core::result::Result::Ok(val) = <#repr_ty as ::core::str::FromStr>::from_str(remaining) {
              return ::core::result::Result::Ok(::core::convert::From::from(val));
            }
          }
        }

        if let ::core::result::Result::Ok(val) = <#repr_ty as ::core::str::FromStr>::from_str(val) {
          return ::core::result::Result::Ok(::core::convert::From::from(val));
        }

        ::core::result::Result::Err(#parse_error_name {
          _priv: ::std::string::ToString::to_string(src),
        })
      },
    };

    #[cfg(not(any(feature = "alloc", feature = "std")))]
    let err_branch = quote! {
      val => {
        if let ::core::option::Option::Some(remaining) = val.strip_prefix("Unknown(").or_else(|| val.strip_prefix("unknown(")) {
          if let ::core::option::Option::Some(remaining) = remaining.strip_suffix(')') {
            if let ::core::result::Result::Ok(val) = <#repr_ty as ::core::str::FromStr>::from_str(remaining) {
              return ::core::result::Result::Ok(::core::convert::From::from(val));
            }
          }
        }

        if let ::core::result::Result::Ok(val) = <#repr_ty as ::core::str::FromStr>::from_str(val) {
          return ::core::result::Result::Ok(::core::convert::From::from(val));
        }

        ::core::result::Result::Err(#parse_error_name {
          _priv: (),
        })
      },
    };

    quote! {
      #[automatically_derived]
      impl ::core::fmt::Display for #parse_error_name {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
          #error_display
        }
      }

      #[automatically_derived]
      impl ::core::error::Error for #parse_error_name {}

      #[automatically_derived]
      impl ::core::str::FromStr for #name_ident {
        type Err = #parse_error_name;

        fn from_str(src: &::core::primitive::str) -> ::core::result::Result<Self, Self::Err> {
          match src.trim() {
            #(#try_from_branch)*
            #err_branch
          }
        }
      }

      #[automatically_derived]
      impl<'a> ::core::convert::TryFrom<&'a ::core::primitive::str> for #name_ident {
        type Error = #parse_error_name;

        fn try_from(src: &'a ::core::primitive::str) -> ::core::result::Result<Self, Self::Error> {
          ::core::str::FromStr::from_str(src)
        }
      }
    }
  }

  #[cfg(feature = "quickcheck")]
  pub(super) fn enum_quickcheck<F>(
    &self,
    path_to_grost: &syn::Path,
    flavor: &F,
  ) -> proc_macro2::TokenStream
  where
    F: super::DeriveGenerator + ?Sized,
  {
    let name_ident = &self.name;
    let variants = self.variants.iter().map(|v| &v.name);

    let quickcheck_test_mod = format_ident!(
      "__quickcheck_fuzzy_{}_flavor_{}__",
      flavor.name().to_snake_case(),
      name_ident.name_str().to_snake_case()
    );
    let quickcheck_fn = format_ident!("fuzzy_{}", name_ident.name_str().to_snake_case());
    let flavor_ty = flavor.ty();

    quote! {
      const _: () = {
        use #path_to_grost::__private::quickcheck::{Arbitrary, Gen};

        #[automatically_derived]
        impl #name_ident {
          const __QUICKCHECK_VARIANTS: &[Self] = &[
            #(Self::#variants,)*
          ];
        }

        #[automatically_derived]
        impl Arbitrary for #name_ident {
          fn arbitrary(g: &mut Gen) -> Self {
            *g.choose(Self::__QUICKCHECK_VARIANTS).unwrap()
          }
        }
      };

      #[cfg(test)]
      #[allow(non_snake_case)]
      mod #quickcheck_test_mod {
        extern crate std;

        use super::#name_ident;
        use #path_to_grost::__private::{Encode, Decode};

        #path_to_grost::__private::quickcheck::quickcheck! {
          fn #quickcheck_fn(ctx: <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Context, value: #name_ident) -> bool {
            let encoded_len = Encode::encoded_len(&value, &ctx);
            let mut buf = ::std::vec![0u8; encoded_len];
            let ::core::result::Result::Ok(written) = Encode::encode(&value, &ctx, &mut buf) else {
              return false;
            };

            if written != encoded_len {
              return false;
            }

            let ::core::result::Result::Ok((read, decoded)) = <#name_ident as Decode<'_, _, _, #name_ident>>::decode::<()>(&ctx, &buf[..encoded_len]) else {
              return false;
            };

            if decoded != value || read != written {
              return false;
            }

            true
          }
        }
      }
    }
  }

  #[cfg(feature = "arbitrary")]
  pub(super) fn enum_arbitrary(&self, path_to_grost: &syn::Path) -> proc_macro2::TokenStream {
    let name_ident = &self.name;
    let variants = self.variants.iter().map(|v| &v.name);

    quote! {
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
    }
  }

  pub(super) fn enum_repr_conversion(&self, path_to_grost: &syn::Path) -> proc_macro2::TokenStream {
    let name_ident = &self.name;
    let repr = &self.repr;
    let repr_fq_ty = repr.to_full_qualified_type();
    let repr_ty = repr.to_type();

    let repr_encode_fn = repr.to_encode_fn(path_to_grost);
    let repr_encode_to_fn = repr.to_encode_to_fn(path_to_grost);
    let repr_decode_fn = repr.to_decode_fn(path_to_grost);
    let repr_encoded_len_fn = repr.to_encode_len_fn(path_to_grost);

    let repr_max_encoded_len = repr.to_max_encoded_len();
    let repr_min_encoded_len = repr.to_min_encoded_len();

    let from_fn_name = format_ident!("from_{}", repr_ty);
    let to_fn_name = format_ident!("as_{}", repr_ty);

    let variant_reflection_name = self.variant_reflection_name();

    let const_encode_branches = self.variants.iter().map(|v| {
      let variant_name_ident = &v.name;
      let raw_value = v.value().to_value();

      quote! {
        Self::#variant_name_ident => *<#variant_reflection_name<
          #path_to_grost::__private::reflection::encode::EncodeReflection<
            #path_to_grost::__private::reflection::EnumVariantReflection,
          >,
          (),
          #raw_value,
        > as #path_to_grost::__private::reflection::Reflectable<
          (),
        >>::REFLECTION,
      }
    });

    let const_encode_to_branches = self.variants.iter().map(|v| {
      let variant_name_ident = &v.name;
      let raw_value = v.value().to_value();

      quote! {
        Self::#variant_name_ident => {
          const ENCODED_LEN: ::core::primitive::usize = *<#variant_reflection_name<
            #path_to_grost::__private::reflection::Len<
              #path_to_grost::__private::reflection::encode::EncodeReflection<
                #path_to_grost::__private::reflection::EnumVariantReflection,
              >,
            >,
            (),
            #raw_value,
          > as #path_to_grost::__private::reflection::Reflectable<
            (),
          >>::REFLECTION;

          let buf_len = buf.len();
          if buf_len < ENCODED_LEN {
            return ::core::result::Result::Err(::grost::__private::varing::EncodeError::underflow(ENCODED_LEN, buf_len));
          }

          let (b, _) = buf.split_at_mut(ENCODED_LEN);
          b.copy_from_slice(<#variant_reflection_name<
            #path_to_grost::__private::reflection::encode::EncodeReflection<
              #path_to_grost::__private::reflection::EnumVariantReflection,
            >,
            (),
            #raw_value,
          > as #path_to_grost::__private::reflection::Reflectable<
            (),
          >>::REFLECTION.as_slice());

          ::core::result::Result::Ok(ENCODED_LEN)
        },
      }
    });

    let const_decode_branches = self.variants.iter().map(|v| {
      let variant_name_ident = &v.name;
      let encoded = v.value.to_encoded_value_varint();

      quote! {
        #encoded => (#encoded.len(), Self::#variant_name_ident),
      }
    });

    let const_encoded_len_branches = self.variants.iter().map(|v| {
      let variant_name_ident = &v.name;
      let raw_value = v.value().to_value();

      quote! {
        Self::#variant_name_ident => *<#variant_reflection_name<
          #path_to_grost::__private::reflection::Len<
            #path_to_grost::__private::reflection::encode::EncodeReflection<
              #path_to_grost::__private::reflection::EnumVariantReflection,
            >,
          >,
          (),
          #raw_value,
        > as #path_to_grost::__private::reflection::Reflectable<
          (),
        >>::REFLECTION,
      }
    });

    let from_branches = self.variants.iter().map(|v| {
      let variant_name_ident = &v.name;
      let value = v.value().to_value();

      quote! {
        #value => Self::#variant_name_ident,
      }
    });

    let to_branches = self.variants.iter().map(|v| {
      let variant_name_ident = &v.name;
      let value = v.value().to_value();

      quote! {
        Self::#variant_name_ident => #value,
      }
    });

    quote! {
      #[automatically_derived]
      impl ::core::convert::From<#repr_fq_ty> for #name_ident {
        #[inline]
        fn from(repr: #repr_fq_ty) -> Self {
          Self::#from_fn_name(repr)
        }
      }

      #[automatically_derived]
      impl ::core::convert::From<#name_ident> for #repr_fq_ty {
        #[inline]
        fn from(e: #name_ident) -> Self {
          e.#to_fn_name()
        }
      }

      #[automatically_derived]
      impl<'a> ::core::convert::TryFrom<&'a [::core::primitive::u8]> for #name_ident {
        type Error = #path_to_grost::__private::varing::DecodeError;

        #[inline]
        fn try_from(src: &'a [::core::primitive::u8]) -> ::core::result::Result<Self, Self::Error> {
          Self::decode(src).map(|(_, this)| this)
        }
      }

      #[automatically_derived]
      impl #path_to_grost::__private::varing::Varint for #name_ident {
        const MAX_ENCODED_LEN: ::core::primitive::usize = #repr_max_encoded_len;
        const MIN_ENCODED_LEN: ::core::primitive::usize = #repr_min_encoded_len;

        #[inline]
        fn encode(&self, buf: &mut [::core::primitive::u8]) -> ::core::result::Result<::core::primitive::usize, #path_to_grost::__private::varing::EncodeError> {
          Self::encode_to(self, buf)
        }

        #[inline]
        fn encoded_len(&self) -> ::core::primitive::usize {
          Self::encoded_len(self)
        }

        #[inline]
        fn decode(src: &[::core::primitive::u8]) -> ::core::result::Result<(::core::primitive::usize, Self), #path_to_grost::__private::varing::DecodeError> {
          Self::decode(src)
        }
      }

      #[automatically_derived]
      impl #name_ident {
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
        #[inline]
        pub const fn encode(&self) -> #path_to_grost::__private::varing::utils::Buffer<{ #repr_max_encoded_len + 1 }> {
          match self {
            #(#const_encode_branches)*
            Self::Unknown(val) => #repr_encode_fn(*val),
          }
        }

        /// Returns the encoded bytes of the enum variant.
        #[inline]
        pub const fn encode_to(&self, buf: &mut [::core::primitive::u8]) -> ::core::result::Result<::core::primitive::usize, #path_to_grost::__private::varing::EncodeError> {
          match self {
            #(#const_encode_to_branches)*
            Self::Unknown(val) => #repr_encode_to_fn(*val, buf),
          }
        }

        /// Decodes the enum variant from the encoded bytes.
        ///
        /// Returns the number of bytes readed and `Self`.
        #[inline]
        pub const fn decode(src: &[::core::primitive::u8]) -> ::core::result::Result<(::core::primitive::usize, Self), #path_to_grost::__private::varing::DecodeError> {
          ::core::result::Result::Ok(match src {
            #(#const_decode_branches)*
            src => {
              return match #repr_decode_fn(src) {
                ::core::result::Result::Ok((read, val)) => ::core::result::Result::Ok((read, Self::Unknown(val))),
                ::core::result::Result::Err(e) => ::core::result::Result::Err(e),
              };
            },
          })
        }

        /// Returns the encoded length of the enum variant.
        #[inline]
        pub const fn encoded_len(&self) -> ::core::primitive::usize {
          match self {
            #(#const_encoded_len_branches)*
            Self::Unknown(val) => #repr_encoded_len_fn(*val),
          }
        }
      }
    }
  }
}
