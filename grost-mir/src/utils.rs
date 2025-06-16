use std::path::PathBuf;

use darling::FromMeta;
use quote::quote;
use syn::{Attribute, parse::Parser};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Invokable {
  expr: syn::Expr,
}

impl AsRef<syn::Expr> for Invokable {
  fn as_ref(&self) -> &syn::Expr {
    &self.expr
  }
}

impl From<syn::ExprPath> for Invokable {
  fn from(value: syn::ExprPath) -> Self {
    Self {
      expr: syn::Expr::Path(value),
    }
  }
}

impl From<syn::ExprClosure> for Invokable {
  fn from(value: syn::ExprClosure) -> Self {
    Self {
      expr: syn::Expr::Closure(value),
    }
  }
}

impl From<syn::Path> for Invokable {
  fn from(path: syn::Path) -> Self {
    Self::from(syn::ExprPath {
      attrs: vec![],
      qself: None,
      path,
    })
  }
}

impl From<Invokable> for syn::Expr {
  fn from(value: Invokable) -> Self {
    value.expr
  }
}

impl TryFrom<Invokable> for syn::Path {
  type Error = darling::Error;

  fn try_from(value: Invokable) -> Result<Self, Self::Error> {
    match value.expr {
      syn::Expr::Path(expr_path) => Ok(expr_path.path),
      syn::Expr::Closure(_) => Err(darling::Error::custom(
        "Invokable is a closure, cannot be converted to a path",
      )),
      _ => unreachable!("Invokable should only contain ExprPath or ExprClosure"),
    }
  }
}

impl TryFrom<syn::Expr> for Invokable {
  type Error = darling::Error;

  fn try_from(expr: syn::Expr) -> Result<Self, Self::Error> {
    match expr {
      syn::Expr::Path(_) | syn::Expr::Closure(_) => Ok(Self { expr }),
      syn::Expr::Lit(syn::ExprLit {
        lit: syn::Lit::Str(s),
        ..
      }) => syn::parse_str::<syn::Path>(&s.value())
        .map_err(|e| {
          darling::Error::custom(format!("expect a path in string literal: {}", e)).with_span(&s)
        })
        .map(Self::from),
      _ => Err(darling::Error::unexpected_expr_type(&expr)),
    }
  }
}

impl TryFrom<&syn::Expr> for Invokable {
  type Error = darling::Error;

  fn try_from(expr: &syn::Expr) -> Result<Self, Self::Error> {
    Self::try_from(expr.clone())
  }
}

impl FromMeta for Invokable {
  fn from_expr(expr: &syn::Expr) -> darling::Result<Self> {
    Self::try_from(expr.clone())
  }
}

impl quote::ToTokens for Invokable {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    self.expr.to_tokens(tokens);
  }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct BoolOption(
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
  Or(Invokable),
  OrDefault(Option<Invokable>),
  OkOr(Invokable),
}

impl MissingOperation {
  /// Returns the name of the operation, which is used to generate the method name in the code.
  pub const fn name(&self) -> &'static str {
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
  Into(Invokable),
  TryInto(Invokable),
}

impl TransformOperation {
  /// Returns the name of the operation, which is used to generate the method name in the code.
  pub const fn name(&self) -> &'static str {
    match self {
      Self::Into(_) => "transform",
      Self::TryInto(_) => "try_transform",
    }
  }
}

pub(crate) enum NestedMetaWithTypeField {
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

/// A wrapper over `Vec<syn::Attribute>`, which implements `FromMeta` to parse
/// a list of attributes from the meta data.
#[derive(Debug, Default, Clone)]
pub struct Attributes(Vec<Attribute>);

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
pub struct SchemaFromMeta {
  #[darling(default)]
  pub(crate) name: Option<String>,
  #[darling(default)]
  pub(crate) description: Option<String>,
}

impl From<SchemaFromMeta> for SchemaAttribute {
  fn from(meta: SchemaFromMeta) -> Self {
    Self {
      name: meta.name,
      description: meta.description,
    }
  }
}

/// The meta for the schema
#[derive(Default, Debug, Clone)]
pub struct SchemaAttribute {
  name: Option<String>,
  description: Option<String>,
}

impl SchemaAttribute {
  /// Returns the name of the schema
  pub const fn name(&self) -> Option<&str> {
    match self.name.as_ref() {
      Some(name) => Some(name.as_str()),
      None => None,
    }
  }

  /// Returns the description of the schema
  pub const fn description(&self) -> Option<&str> {
    match self.description.as_ref() {
      Some(description) => Some(description.as_str()),
      None => None,
    }
  }
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

/// Returns a generic parameter `__GROST_READ_BUFFER__`, which is used to represent
/// the read buffer generic parameter in the generated code.
pub fn grost_read_buffer_param() -> syn::TypeParam {
  quote::format_ident!("__GROST_READ_BUFFER__").into()
}

/// Returns a generic parameter `__GROST_WRITE_BUFFER__`, which is used to represent
/// the write buffer generic parameter in the generated code
pub fn grost_write_buffer_param() -> syn::TypeParam {
  quote::format_ident!("__GROST_WRITE_BUFFER__").into()
}
