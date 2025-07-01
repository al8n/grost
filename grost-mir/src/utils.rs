use std::path::PathBuf;

use darling::FromMeta;
use proc_macro2::{Delimiter, TokenTree};
use quote::{ToTokens, quote};
use syn::{
  Attribute, ConstParam, Expr, ExprLit, Ident, Lit, MacroDelimiter, Meta, MetaList, MetaNameValue,
  Path, PathSegment, Token, TypeParam,
  ext::IdentExt,
  parse::{ParseStream, Parser, discouraged::Speculative as _},
  punctuated::Punctuated,
  token::{self, Brace, Bracket, Paren},
};

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

  pub const fn is_some(&self) -> bool {
    self.0.is_some()
  }
}

/// Specifies the behavior of how to handle the missing field during decoding.
#[derive(Debug, Clone, PartialEq, Eq, derive_more::Display, darling::FromMeta)]
pub enum MissingOperation {
  #[display("or_else")]
  #[darling(rename = "or_else")]
  OrElse(Invokable),
  #[display("or_default")]
  #[darling(rename = "or_default")]
  OrDefault,
  #[display("ok_or_else")]
  #[darling(rename = "ok_or_else")]
  OkOrElse(Invokable),
}

impl MissingOperation {
  /// Returns the name of the operation, which is used to generate the method name in the code.
  pub const fn name(&self) -> &'static str {
    match self {
      MissingOperation::OrElse(_) => "or_else",
      MissingOperation::OrDefault => "or_default",
      MissingOperation::OkOrElse(_) => "ok_or_else",
    }
  }

  /// Returns the possible names.
  #[inline]
  pub const fn possible_names() -> &'static [&'static str] {
    &["or_else", "or_default", "ok_or_else"]
  }

  /// Parsing an optional missing operation from a meta list
  #[inline]
  pub fn parse_from_meta_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Option<Self>> {
    fn duplicate_error(old_op: &MissingOperation, new_op: &str) -> darling::Error {
      darling::Error::custom(format!(
        "Cannot specify both `{old_op}` and `{new_op}` at the same time.",
      ))
    }

    if items.is_empty() {
      return Ok(None);
    }

    let mut missing_operation = None;
    for item in items {
      match item {
        darling::ast::NestedMeta::Lit(lit) => {
          return Err(darling::Error::unexpected_lit_type(lit));
        }
        darling::ast::NestedMeta::Meta(meta) => {
          if meta.path().is_ident("or_else") {
            if let Some(ref old_op) = missing_operation {
              return Err(duplicate_error(old_op, "or_else"));
            }

            let nv = meta.require_name_value()?;
            let invokable = Invokable::try_from(&nv.value)?;
            missing_operation = Some(MissingOperation::OrElse(invokable));
          } else if meta.path().is_ident("or_default") {
            if let Some(ref old_op) = missing_operation {
              return Err(duplicate_error(old_op, "or_default"));
            }

            let _ = meta.require_path_only()?;
            missing_operation = Some(MissingOperation::OrDefault);
          } else if meta.path().is_ident("ok_or_else") {
            if let Some(ref old_op) = missing_operation {
              return Err(duplicate_error(old_op, "ok_or_else"));
            }

            let nv = meta.require_name_value()?;
            let invokable = Invokable::try_from(&nv.value)?;
            missing_operation = Some(MissingOperation::OkOrElse(invokable));
          }
        }
      }
    }

    Ok(missing_operation)
  }

  pub fn call(&self) -> proc_macro2::TokenStream {
    match self {
      MissingOperation::OrElse(invokable) => {
        quote! { (#invokable)() }
      }
      MissingOperation::OrDefault => {
        quote! { (::core::default::Default::default)() }
      }
      MissingOperation::OkOrElse(invokable) => {
        quote! { (#invokable)()? }
      }
    }
  }
}

/// Specifies the behavior of how to do the convert operation.
#[derive(Debug, Clone, PartialEq, Eq, darling::FromMeta)]
pub enum ConvertOperation {
  From(Invokable),
  TryFrom(Invokable),
}

impl ConvertOperation {
  /// Returns the name of the operation, which is used to generate the method name in the code.
  pub const fn name(&self) -> &'static str {
    match self {
      Self::From(_) => "from",
      Self::TryFrom(_) => "try_from",
    }
  }

  /// Returns the possible names.
  #[inline]
  pub const fn possible_names() -> &'static [&'static str] {
    &["from", "try_from"]
  }

  /// Parses an optional convert operation from a meta list
  #[inline]
  pub fn parse_from_meta_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Option<Self>> {
    fn duplicate_error(old_op: &ConvertOperation, new_op: &str) -> darling::Error {
      darling::Error::custom(format!(
        "Cannot specify both `{}` and `{}` at the same time.",
        old_op.name(),
        new_op,
      ))
    }

    if items.is_empty() {
      return Ok(None);
    }

    let mut convert_operation = None;

    for item in items {
      match item {
        darling::ast::NestedMeta::Lit(lit) => {
          return Err(darling::Error::unexpected_lit_type(lit));
        }
        darling::ast::NestedMeta::Meta(meta) => {
          if meta.path().is_ident("from") {
            if let Some(ref old_op) = convert_operation {
              return Err(duplicate_error(old_op, "from"));
            }

            let nv = meta.require_name_value()?;
            let invokable = Invokable::try_from(&nv.value)?;
            convert_operation = Some(ConvertOperation::From(invokable));
          } else if meta.path().is_ident("try_from") {
            if let Some(ref old_op) = convert_operation {
              return Err(duplicate_error(old_op, "try_from"));
            }

            let nv = meta.require_name_value()?;
            let invokable = Invokable::try_from(&nv.value)?;
            convert_operation = Some(ConvertOperation::TryFrom(invokable));
          }
        }
      }
    }
    Ok(convert_operation)
  }

  pub fn call(&self, args: &[impl ToTokens]) -> proc_macro2::TokenStream {
    match self {
      ConvertOperation::From(invokable) => {
        quote! { (#invokable)(#(#args),*) }
      }
      ConvertOperation::TryFrom(invokable) => {
        quote! { (#invokable)(#(#args),*)? }
      }
    }
  }
}

/// A no-op meta type, which is used when the meta data is not needed or
/// should be ignored.
#[derive(Debug, Clone, Default)]
pub struct NoopFromMeta;

impl FromMeta for NoopFromMeta {
  fn from_word() -> darling::Result<Self> {
    Ok(Self)
  }

  fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
    let mut errors = darling::Error::accumulator();
    for item in items {
      match item {
        darling::ast::NestedMeta::Lit(lit) => {
          errors.push(darling::Error::unexpected_lit_type(lit));
        }
        darling::ast::NestedMeta::Meta(meta) => {
          errors.push(darling::Error::unknown_field_path(meta.path()))
        }
      }
    }

    errors.finish().map(|_| Self)
  }
}

#[derive(Debug, Clone)]
pub enum NestedMeta {
  Lit(syn::Lit),
  Meta(syn::Meta),
}

fn parse_meta_path(input: ParseStream) -> syn::Result<Path> {
  Ok(Path {
    leading_colon: input.parse()?,
    segments: {
      let mut segments = Punctuated::new();
      loop {
        if !input.peek(Ident::peek_any) {
          break;
        }

        let ident = Ident::parse_any(input)?;
        segments.push_value(PathSegment::from(ident));
        if !input.peek(Token![::]) {
          break;
        }
        let punct = input.parse()?;
        segments.push_punct(punct);
      }
      if segments.is_empty() {
        return Err(input.parse::<Ident>().unwrap_err());
      } else if segments.trailing_punct() {
        return Err(input.error("expected path segment after `::`"));
      }
      segments
    },
  })
}

fn parse_meta_after_path(path: Path, input: ParseStream) -> syn::Result<Meta> {
  if input.peek(token::Paren) || input.peek(token::Bracket) || input.peek(token::Brace) {
    parse_meta_list_after_path(path, input).map(Meta::List)
  } else if input.peek(Token![=]) {
    parse_meta_name_value_after_path(path, input).map(Meta::NameValue)
  } else {
    Ok(Meta::Path(path))
  }
}

fn parse_meta_list_after_path(path: Path, input: ParseStream) -> syn::Result<MetaList> {
  let (delimiter, tokens) = input.step(|cursor| {
    if let Some((TokenTree::Group(g), rest)) = cursor.token_tree() {
      let span = g.delim_span();
      let delimiter = match g.delimiter() {
        Delimiter::Parenthesis => MacroDelimiter::Paren(Paren(span)),
        Delimiter::Brace => MacroDelimiter::Brace(Brace(span)),
        Delimiter::Bracket => MacroDelimiter::Bracket(Bracket(span)),
        Delimiter::None => {
          return Err(cursor.error("expected delimiter"));
        }
      };
      Ok(((delimiter, g.stream()), rest))
    } else {
      Err(cursor.error("expected delimiter"))
    }
  })?;
  Ok(MetaList {
    path,
    delimiter,
    tokens,
  })
}

fn parse_meta_name_value_after_path(path: Path, input: ParseStream) -> syn::Result<MetaNameValue> {
  let eq_token: Token![=] = input.parse()?;
  let ahead = input.fork();
  let lit: Option<Lit> = ahead.parse()?;
  let value = if let (Some(lit), true) = (lit, ahead.is_empty()) {
    input.advance_to(&ahead);
    Expr::Lit(ExprLit {
      attrs: Vec::new(),
      lit,
    })
  } else if input.peek(Token![#]) && input.peek2(token::Bracket) {
    return Err(input.error("unexpected attribute inside of attribute"));
  } else {
    input.parse()?
  };
  Ok(MetaNameValue {
    path,
    eq_token,
    value,
  })
}

impl syn::parse::Parse for NestedMeta {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    if input.peek(syn::Lit) && !(input.peek(syn::LitBool) && input.peek2(syn::Token![=])) {
      input.parse().map(Self::Lit)
    } else if input.peek(syn::Ident::peek_any) {
      let path = parse_meta_path(input)?;
      parse_meta_after_path(path, input).map(Self::Meta)
    } else {
      Err(input.error("expected identifier or literal"))
    }
  }
}

impl NestedMeta {
  pub fn parse_meta_list(
    tokens: proc_macro2::TokenStream,
  ) -> syn::Result<Vec<darling::ast::NestedMeta>> {
    syn::punctuated::Punctuated::<NestedMeta, Token![,]>::parse_terminated
      .parse2(tokens)
      .map(|punctuated| punctuated.into_iter().map(Into::into).collect())
  }
}

impl From<NestedMeta> for darling::ast::NestedMeta {
  fn from(value: NestedMeta) -> Self {
    match value {
      NestedMeta::Lit(lit) => darling::ast::NestedMeta::Lit(lit),
      NestedMeta::Meta(meta) => darling::ast::NestedMeta::Meta(meta),
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

impl From<SchemaFromMeta> for SchemaOptions {
  fn from(meta: SchemaFromMeta) -> Self {
    Self {
      name: meta.name,
      description: meta.description,
    }
  }
}

/// The meta for the schema
#[derive(Default, Debug, Clone)]
pub struct SchemaOptions {
  pub(crate) name: Option<String>,
  pub(crate) description: Option<String>,
}

impl SchemaOptions {
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
pub fn grost_flavor_param() -> TypeParam {
  syn::parse_quote!(__GROST_FLAVOR__: ?::core::marker::Sized)
}

/// Returns a generic parameter `__GROST_WIRE_FORMAT__: ?::core::marker::Sized`, which is used to represent
/// the a wire format generic parameter in the generated code. This is used to avoid
/// conflicts with other generic parameters in the code.
pub fn grost_wire_format_param() -> TypeParam {
  syn::parse_quote!(__GROST_WIRE_FORMAT__: ?::core::marker::Sized)
}

/// Returns a generic parameter `__GROST_BUFFER__`, which is used to represent
/// the unknown buffer generic parameter in the generated code, which is used to store unknown data.
/// This is used to avoid conflicts with other generic parameters in the code.
pub fn grost_buffer_param() -> TypeParam {
  quote::format_ident!("__GROST_BUFFER__").into()
}

/// Returns a generic parameter `__GROST_READ_BUFFER__`, which is used to represent
/// the read buffer generic parameter in the generated code.
pub fn grost_read_buffer_param() -> TypeParam {
  quote::format_ident!("__GROST_READ_BUFFER__").into()
}

/// Returns a generic parameter `__GROST_WRITE_BUFFER__`, which is used to represent
/// the write buffer generic parameter in the generated code
pub fn grost_write_buffer_param() -> TypeParam {
  quote::format_ident!("__GROST_WRITE_BUFFER__").into()
}

pub(crate) fn grost_decode_trait_lifetime() -> syn::LifetimeParam {
  syn::parse_quote!('__grost_decode_lifetime__)
}

pub(crate) fn grost_selected_param() -> ConstParam {
  syn::parse_quote!(
    const __GROST_SELECTED__: ::core::primitive::bool = true
  )
}
