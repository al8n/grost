use std::path::PathBuf;

use darling::FromMeta;
use quote::quote;
use syn::{Attribute, parse::Parser};

pub(crate) mod flavor;
pub(crate) mod generic;
pub(crate) mod object;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub(crate) struct BoolOption(
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))] Option<bool>,
);

impl From<Option<bool>> for BoolOption {
  fn from(value: Option<bool>) -> Self {
    Self(value)
  }
}

impl From<bool> for BoolOption {
  fn from(value: bool) -> Self {
    Self(Some(value))
  }
}

impl From<BoolOption> for Option<bool> {
  fn from(value: BoolOption) -> Self {
    value.0
  }
}

impl FromMeta for BoolOption {
  fn from_word() -> darling::Result<Self> {
    Ok(Self(Some(true)))
  }

  fn from_bool(value: bool) -> darling::Result<Self> {
    if value {
      Ok(Self(Some(true)))
    } else {
      Ok(Self(Some(false)))
    }
  }
}

impl BoolOption {
  pub const fn yes() -> Self {
    Self(Some(true))
  }

  pub const fn no() -> Self {
    Self(Some(false))
  }

  pub const fn is_yes(&self) -> bool {
    matches!(self.0, Some(true))
  }

  pub const fn is_no(&self) -> bool {
    matches!(self.0, Some(false))
  }

  pub const fn is_none(&self) -> bool {
    self.0.is_none()
  }
}

/// Specifies the behavior of how to handle the missing field during decoding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MissingOperation {
  Or(syn::Path),
  OrDefault(Option<syn::Path>),
  OkOr(syn::Path),
}

impl MissingOperation {
  fn name(&self) -> &'static str {
    match self {
      MissingOperation::Or(_) => "or_else",
      MissingOperation::OrDefault(_) => "or_else_default",
      MissingOperation::OkOr(_) => "ok_or_else",
    }
  }
}

/// Specifies the behavior of how to transform from decoded state to base state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransformOperation {
  Into(syn::Path),
  TryInto(syn::Path),
}

impl TransformOperation {
  fn name(&self) -> &'static str {
    match self {
      Self::Into(_) => "transform",
      Self::TryInto(_) => "try_transform",
    }
  }
}

enum NestedMetaWithTypeField {
  Type(syn::Type),
  Nested(darling::ast::NestedMeta),
}

impl syn::parse::Parse for NestedMetaWithTypeField {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    if input.peek(syn::Token![type]) {
      let _: syn::Token![type] = input.parse()?;
      let _: syn::Token![=] = input.parse()?;
      let ty_str: syn::LitStr = input.parse()?;
      Ok(Self::Type(syn::parse_str(&ty_str.value())?))
    } else {
      darling::ast::NestedMeta::parse(input).map(Self::Nested)
    }
  }
}

#[derive(Debug, Default, Clone)]
struct Attributes(Vec<Attribute>);

impl From<Attributes> for Vec<Attribute> {
  fn from(attrs: Attributes) -> Self {
    attrs.0
  }
}

impl Attributes {
  /// Consumes the `Attributes` and returns the inner vector of attributes
  pub fn into_inner(self) -> Vec<Attribute> {
    self.0
  }
}

impl FromMeta for Attributes {
  fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
    let mut attributes = Vec::new();
    for item in items {
      match item {
        darling::ast::NestedMeta::Lit(lit) => {
          return Err(darling::Error::unexpected_lit_type(lit));
        }
        darling::ast::NestedMeta::Meta(meta) => {
          let attr = Attribute::parse_outer
            .parse2(quote! { #[#meta] })
            .map_err(|e| darling::Error::from(e).with_span(meta))?;
          attributes.extend(attr);
        }
      }
    }

    Ok(Self(attributes))
  }
}

/// The meta for the schema
#[derive(Default, Debug, Clone, FromMeta)]
pub(crate) struct SchemaFromMeta {
  #[darling(default)]
  pub(crate) name: Option<String>,
  #[darling(default)]
  pub(crate) description: Option<String>,
}

/// Configures the output of the generated code, this is useful when you want to
/// debug the generated code.
#[derive(Debug, Default, Clone)]
pub struct Output {
  path: PathBuf,
  format: bool,
}

impl Output {
  /// Returns the path to the output file
  pub const fn path(&self) -> &PathBuf {
    &self.path
  }

  /// Returns `true` if the output should be formatted
  pub const fn format(&self) -> bool {
    self.format
  }
}

impl FromMeta for Output {
  fn from_string(value: &str) -> darling::Result<Self> {
    Ok(Self {
      path: PathBuf::from(value),
      format: false,
    })
  }

  fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
    let mut path = None;
    let mut format = false;
    println!("items: {:?}", items);
    for item in items {
      match item {
        darling::ast::NestedMeta::Lit(lit) => {
          return Err(darling::Error::unexpected_lit_type(lit));
        }
        darling::ast::NestedMeta::Meta(meta) => {
          if meta.path().is_ident("path") {
            path = Some(<PathBuf as FromMeta>::from_meta(meta)?);
          } else if meta.path().is_ident("format") {
            format = <bool as FromMeta>::from_meta(meta)?;
          }
        }
      }
    }

    Ok(Self {
      path: path.ok_or_else(|| darling::Error::missing_field("path"))?,
      format,
    })
  }
}

/// Returns a `'__grost_lifetime__` lifetime, which is the lifetime name used
/// in the generated code. This is used to avoid conflicts with other
/// lifetimes in the code.
pub fn grost_lifetime() -> syn::LifetimeParam {
  syn::parse_quote!('__grost_lifetime__)
}

/// Returns a generic parameter `__GROST_FLAVOR__: ?::core::marker::Sized`, which is used to represent
/// the a flavor generic parameter in the generated code. This is used to avoid
/// conflicts with other generic parameters in the code.
pub fn grost_flavor_param() -> syn::TypeParam {
  syn::parse_quote!(__GROST_FLAVOR__: ?::core::marker::Sized)
}

/// Returns a generic parameter `__GROST_WIRE_FORMAT__: ?::core::marker::Sized`, which is used to represent
/// the a wire format generic parameter in the generated code. This is used to avoid
/// conflicts with other generic parameters in the code.
pub fn grost_wire_format_param() -> syn::TypeParam {
  syn::parse_quote!(__GROST_WIRE_FORMAT__: ?::core::marker::Sized)
}

/// Returns a generic parameter `__GROST_UNKNOWN_BUFFER__`, which is used to represent
/// the unknown buffer generic parameter in the generated code, which is used to store unknown data.
/// This is used to avoid conflicts with other generic parameters in the code.
pub fn grost_unknown_buffer_param() -> syn::TypeParam {
  quote::format_ident!("__GROST_UNKNOWN_BUFFER__").into()
}
