use either::Either;
use quote::quote;
use std::sync::Arc;
use syn::{
  Ident, Meta, Token, Type,
  ext::IdentExt,
  parse::{Parse, ParseStream},
  token::Paren,
};

const WIRE_FORMAT: &str = "as";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListLikeLabel {
  pub(crate) label: Arc<Label>,
  pub(crate) repeated: bool,
}

impl ListLikeLabel {
  /// Returns the label of the list-like type.
  pub fn label(&self) -> &Label {
    &self.label
  }

  /// Returns whether the list-like type is marked as repeated.
  pub fn is_repeated(&self) -> bool {
    self.repeated
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapLabel {
  pub(crate) key: Arc<Label>,
  pub(crate) value: Arc<Label>,
  pub(crate) repeated: bool,
}

impl MapLabel {
  /// Returns the key label of the map.
  pub fn key(&self) -> &Label {
    &self.key
  }

  /// Returns the value label of the map.
  pub fn value(&self) -> &Label {
    &self.value
  }

  /// Returns whether the map is marked as repeated.
  pub fn is_repeated(&self) -> bool {
    self.repeated
  }
}

/// A type specification for an object field.
#[derive(Debug, Clone, PartialEq, Eq, derive_more::IsVariant, derive_more::Display)]
#[non_exhaustive]
pub enum Label {
  /// A scalar type label, e.g. `i32`, `f64`, etc.
  #[display("scalar")]
  Scalar(Option<Type>),
  /// A byte array type label, e.g. `Vec<u8>`, `bytes`, etc.
  #[display("bytes")]
  Bytes(Option<Type>),
  /// A string type label, e.g. `String`, `&str`, etc.
  #[display("string")]
  String(Option<Type>),
  /// An object type label
  #[display("object")]
  Object(Option<Type>),
  /// An enum type label
  #[display("enum")]
  Enum(Option<Type>),
  /// A union type label
  #[display("union")]
  Union(Option<Type>),
  /// An interface type label
  #[display("interface")]
  Interface(Option<Type>),
  /// A generic type label, which means the type of this field is generic param
  #[display("union")]
  Generic(Option<Type>),
  /// A map type label
  #[display("map")]
  Map(Either<MapLabel, Type>),
  /// A set type label
  #[display("set")]
  Set(Either<ListLikeLabel, Type>),
  /// A list type label
  #[display("list")]
  List(Either<ListLikeLabel, Type>),
  /// An nullable type label
  #[display("nullable({_0})")]
  Nullable(Arc<Label>),
}

impl Label {
  /// Returns the possible identifiers for labels.
  #[inline]
  pub const fn possible_idents() -> &'static [&'static str] {
    &[
      "scalar",
      "bytes",
      "string",
      "object",
      "enum",
      "union",
      "interface",
      "generic",
      "map",
      "set",
      "list",
      "nullable",
    ]
  }
}

impl darling::FromMeta for Label {
  fn from_meta(item: &Meta) -> darling::Result<Self> {
    (syn::parse2(quote!(#item))).map_err(|e| darling::Error::from(e).with_span(item))
  }

  fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
    let ts = quote! (
      #(#items)*
    );

    (syn::parse2(ts)).map_err(darling::Error::from)
  }
}

impl Label {
  /// Check if the parse stream starts with a label that can be parsed as a `Label`.
  pub fn peek(input: &ParseStream) -> syn::Result<bool> {
    if input.peek(syn::Token![enum]) {
      return Ok(true);
    }

    if input.peek(syn::Ident::peek_any) {
      let ident: syn::Ident = input.fork().parse()?;
      return Ok(match () {
        () if ident.eq("scalar") => true,
        () if ident.eq("bytes") => true,
        () if ident.eq("string") => true,
        () if ident.eq("object") => true,
        () if ident.eq("enum") => true,
        () if ident.eq("union") => true,
        () if ident.eq("interface") => true,
        () if ident.eq("map") => true,
        () if ident.eq("set") => true,
        () if ident.eq("list") => true,
        () if ident.eq("nullable") => true,
        () if ident.eq("generic") => true,
        _ => false,
      });
    }

    Ok(false)
  }

  /// Construct a mark type for the type based on the label.
  pub fn mark(&self, path_to_grost: &syn::Path, ty: &syn::Type, tag: u32) -> syn::Result<syn::Type> {
    self.mark_helper(path_to_grost, ty, tag, true)
  }

  fn mark_helper(
    &self,
    path_to_grost: &syn::Path,
    ty: &syn::Type,
    tag: u32,
    outermost: bool,
  ) -> syn::Result<syn::Type> {
    /// The inner tag for repeating fields should always be 1.
    /// 
    /// e.g.
    /// 
    /// ```rust,ignore
    /// struct User {
    ///   #[grost(tag = 3, map(key(string), value(list(string, repeated)), repeated))]
    ///   media: HashMap<String, Vec<String>>,
    /// }
    /// ```
    /// 
    /// For the first repeated field, the tag will be 3,
    /// the inner tag for the repeated field will be 1, because it just like
    /// 
    /// ```rust,ignore
    /// struct Anonymous {
    ///   #[grost(tag = 1, list(string, repeated))]
    ///   links: Vec<String>
    /// }
    /// 
    /// struct User {
    ///   #[grost(tag = 3, map(key(string), value(object), repeated))]
    ///   media: HashMap<String, Anonymous>,
    /// }
    /// ```
    const INNER_TAG: u32 = 1;

    Ok(match self {
      Self::Scalar(ty) => match ty {
        None => syn::parse2(quote! {
          #path_to_grost::__private::marker::ScalarMarker<#ty>
        })?,
        Some(ty) => ty.clone(),
      },
      Self::Bytes(ty) => match ty {
        None => syn::parse2(quote! {
          #path_to_grost::__private::marker::BytesMarker<#ty>
        })?,
        Some(ty) => ty.clone(),
      },
      Self::String(ty) => match ty {
        None => syn::parse2(quote! {
          #path_to_grost::__private::marker::StringMarker<#ty>
        })?,
        Some(ty) => ty.clone(),
      },
      Self::Object(ty) => match ty {
        None => syn::parse2(quote! {
          #path_to_grost::__private::marker::ObjectMarker<#ty>
        })?,
        Some(ty) => ty.clone(),
      },
      Self::Enum(ty) => match ty {
        None => syn::parse2(quote! {
          #path_to_grost::__private::marker::EnumMarker<#ty>
        })?,
        Some(ty) => ty.clone(),
      },
      Self::Union(ty) => match ty {
        None => syn::parse2(quote! {
          #path_to_grost::__private::marker::UnionMarker<#ty>
        })?,
        Some(ty) => ty.clone(),
      },
      Self::Interface(ty) => match ty {
        None => syn::parse2(quote! {
          #path_to_grost::__private::marker::InterfaceMarker<#ty>
        })?,
        Some(ty) => ty.clone(),
      },
      Self::Generic(ty) => match ty {
        None => syn::parse2(quote! {
          #path_to_grost::__private::marker::GenericMarker<#ty>
        })?,
        Some(ty) => ty.clone(),
      },
      Self::Map(Either::Right(ty)) => ty.clone(),
      Self::Map(Either::Left(MapLabel {
        key,
        value,
        repeated,
      })) => {
        let k: syn::Type = syn::parse2(quote! {
          <#ty as #path_to_grost::__private::convert::State<
            #path_to_grost::__private::convert::Flattened<#path_to_grost::__private::convert::MapKey>,
          >>::Output
        })?;
        let v: syn::Type = syn::parse2(quote! {
          <#ty as #path_to_grost::__private::convert::State<
            #path_to_grost::__private::convert::Flattened<#path_to_grost::__private::convert::MapValue>,
          >>::Output
        })?;
        let km = key.mark_helper(path_to_grost, &k, INNER_TAG, false)?;
        let vm = value.mark_helper(path_to_grost, &v, INNER_TAG, false)?;

        if *repeated {
          syn::parse2(quote! {
            #path_to_grost::__private::marker::RepeatedEntryMarker<#ty, #km, #vm, #path_to_grost::__private::buffer::DefaultBuffer<>, #tag>
          })?
        } else {
          syn::parse2(quote! {
            #path_to_grost::__private::marker::MapMarker<#ty, #km, #vm>
          })?
        }
      }
      Self::Set(Either::Right(ty)) => ty.clone(),
      Self::Set(Either::Left(ListLikeLabel { label, repeated })) => {
        let inner_ty: syn::Type = syn::parse2(quote! {
          <#ty as #path_to_grost::__private::convert::State<
            #path_to_grost::__private::convert::Flattened<#path_to_grost::__private::convert::Inner>,
          >>::Output
        })?;
        let inner = label.mark_helper(path_to_grost, &inner_ty, INNER_TAG, false)?;
        if *repeated {
          syn::parse2(quote! {
            #path_to_grost::__private::marker::RepeatedMarker<#ty, #inner, #path_to_grost::__private::buffer::DefaultBuffer<>, #tag>
          })?
        } else {
          syn::parse2(quote! {
            #path_to_grost::__private::marker::SetMarker<#ty, #inner>
          })?
        }
      }
      Self::List(Either::Right(ty)) => ty.clone(),
      Self::List(Either::Left(ListLikeLabel { label, repeated })) => {
        let inner_ty: syn::Type = syn::parse2(quote! {
          <#ty as #path_to_grost::__private::convert::State<
            #path_to_grost::__private::convert::Flattened<#path_to_grost::__private::convert::Inner>,
          >>::Output
        })?;
        let inner = label.mark_helper(path_to_grost, &inner_ty, INNER_TAG, false)?;

        if *repeated {
          syn::parse2(quote! {
            #path_to_grost::__private::marker::RepeatedMarker<#ty, #inner, #path_to_grost::__private::buffer::DefaultBuffer<>, #tag>
          })?
        } else {
          syn::parse2(quote! {
            #path_to_grost::__private::marker::ListMarker<#ty, #inner>
          })?
        }
      }
      Self::Nullable(label) => {
        let inner_ty: syn::Type = syn::parse2(quote! {
          <#ty as #path_to_grost::__private::convert::State<
            #path_to_grost::__private::convert::Flattened<#path_to_grost::__private::convert::Inner>,
          >>::Output
        })?;
        let inner = label.mark_helper(path_to_grost, &inner_ty, tag, false)?;

        // if the option is the outermost, we should use the inner type wire format.
        //
        // e.g.
        //
        // ```rust
        // struct User {
        //   name: Option<String>,
        // }
        // ```
        //
        // In this case, there is no outer wrapper over the `Option<String>`,
        // the default should be the default wire format of String, encode name as nullable
        // will waste at least 2 bytes space.
        if outermost {
          syn::parse2(quote! {
            #path_to_grost::__private::marker::FlattenMarker<#ty, #inner>
          })?
        } else {
          syn::parse2(quote! {
            #path_to_grost::__private::marker::NullableMarker<#ty, #inner>
          })?
        }
      }
    })
  }
}

fn parse_maybe_as(input: ParseStream, name: &str) -> syn::Result<Option<Type>> {
  if input.is_empty() {
    return Ok(None);
  }

  if input.peek(Paren) {
    let content;
    syn::parenthesized!(content in input);

    if content.is_empty() {
      return Ok(None);
    }

    if content.peek(Token![as]) && content.peek2(Token![=]) {
      let _: Token![as] = content.parse()?;
      let _: Token![=] = content.parse()?;
      let ty: Type = content.parse()?;

      return Ok(Some(ty));
    }
  }

  Err(syn::Error::new(
    input.span(),
    format!("Expected `{name}`, `{name}()`, or `{name}(as = \"...\")`"),
  ))
}

fn unexpected_eos_error(ident: Ident) -> syn::Result<Label> {
  match () {
    () if ident.eq("map") => {
      return Err(syn::Error::new(
        ident.span(),
        "`map` requires a key and value, e.g. `map(key(...), value(...))`, `map(key(...), value(...), repeated)` or `map(as = \"...\")`",
      ));
    }
    () if ident.eq("set") => {
      return Err(syn::Error::new(
        ident.span(),
        "`set` requires a type, e.g. `set(...)` or `set(..., repeated)`",
      ));
    }
    () if ident.eq("list") => {
      return Err(syn::Error::new(
        ident.span(),
        "`list` requires a type, e.g. `list(...)` or `list(..., repeated)`",
      ));
    }
    () if ident.eq("nullable") => {
      return Err(syn::Error::new(
        ident.span(),
        "`nullable` requires a type, e.g. `nullable(...)`",
      ));
    }
    () if ident.eq("key") => {
      return Err(syn::Error::new(
        ident.span(),
        "`key` can only be used in `map(...)`",
      ));
    }
    () if ident.eq("value") => {
      return Err(syn::Error::new(
        ident.span(),
        "`value` can only be used in `map(...)`",
      ));
    }
    _ => {
      return Err(syn::Error::new(
        ident.span(),
        "Expected one of [scalar, bytes, string, generic, object, enum, union, interface, map, set, list, nullable]",
      ));
    }
  }
}

struct ListLikeLabelParser<const TAG: u8>(Either<ListLikeLabel, Type>);

impl<const TAG: u8> ListLikeLabelParser<TAG> {
  fn is_set(&self) -> bool {
    matches!(&self.0, Either::Left(ListLikeLabel { label, .. }) if label.is_set())
  }

  fn is_map(&self) -> bool {
    matches!(&self.0, Either::Left(ListLikeLabel { label, .. }) if label.is_map())
  }
}

enum ListLikeLabelParserHelper<const TAG: u8> {
  Label(Label),
  Repeated,
  As(Type),
}

impl<const TAG: u8> Parse for ListLikeLabelParserHelper<TAG> {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    if input.peek(Token![enum]) {
      return Ok(Self::Label(Label::parse(input)?));
    }

    if input.peek(Token![as]) {
      let _: Token![as] = input.parse()?;
      if !input.peek(Token![=]) {
        return Err(syn::Error::new(input.span(), "Expected `=` after `as`"));
      }
      let _: Token![=] = input.parse()?;
      let ty: Type = input.parse()?;
      return Ok(Self::As(ty));
    }

    if !input.peek(Ident) {
      return Err(syn::Error::new(
        input.span(),
        "Expected one of [scalar, bytes, string, generic, object, enum, union, interface, map, set, list, nullable, repeated, as]",
      ));
    }

    let ident: Ident = input.fork().parse()?;
    match () {
      () if Label::possible_idents().iter().any(|label| ident.eq(label)) => {
        Label::parse(input).map(Self::Label)
      }
      () if ident.eq("repeated") => Ok(Self::Repeated),
      _ => Err(syn::Error::new(
        ident.span(),
        format!(
          "Expected one of [scalar, bytes, string, generic, object, enum, union, interface, map, set, list, nullable, repeated, as], found `{}`",
          ident
        ),
      )),
    }
  }
}

impl<const TAG: u8> Parse for ListLikeLabelParser<TAG> {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let mut label = None;
    let mut repeated = None;
    let mut as_type = None;
    let items = input.parse_terminated(ListLikeLabelParserHelper::<TAG>::parse, Token![,])?;

    for item in items {
      match item {
        ListLikeLabelParserHelper::Label(l) => {
          if let Some(ref old) = label {
            return Err(syn::Error::new(
              input.span(),
              format!(
                "Duplicate label found, expected only one label, found `{}` and `{}`",
                old, l
              ),
            ));
          }
          label = Some(l);
        }
        ListLikeLabelParserHelper::Repeated => {
          if repeated.is_some() {
            return Err(syn::Error::new(
              input.span(),
              "Duplicate `repeated` found, only one `repeated` is allowed",
            ));
          }
          repeated = Some(true);
        }
        ListLikeLabelParserHelper::As(ty) => {
          if as_type.is_some() {
            return Err(syn::Error::new(
              input.span(),
              format!("Duplicate `as` found, expected only one `as`"),
            ));
          }
          as_type = Some(ty);
        }
      }
    }
    let repeated = repeated.unwrap_or(false);

    let key = if TAG == 0 { "list" } else { "set" };

    Ok(match (label, repeated, as_type) {
      (Some(label), true, None) => Self(Either::Left(ListLikeLabel {
        label: Arc::new(label),
        repeated: true,
      })),
      (Some(label), false, None) => Self(Either::Left(ListLikeLabel {
        label: Arc::new(label),
        repeated: false,
      })),
      (None, false, Some(ty)) => Self(Either::Right(ty)),
      _ => {
        return Err(syn::Error::new(
          input.span(),
          format!("Expected `{key}(...)`, or `{key}(..., repeated)` or `{key}(as = \"...\")`",),
        ));
      }
    })
  }
}

enum MapLabelParserHelper {
  Key(Label),
  Value(Label),
  Repeated,
  As(Type),
}

impl Parse for MapLabelParserHelper {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    if input.peek(Token![as]) {
      let _: Token![as] = input.parse()?;
      if !input.peek(Token![=]) {
        return Err(syn::Error::new(input.span(), "Expected `as = \"...\"`"));
      }
      let _: Token![=] = input.parse()?;
      let ty: Type = input.parse()?;
      return Ok(Self::As(ty));
    }

    if !input.peek(Ident) {
      return Err(syn::Error::new(
        input.span(),
        "Expected one of [key, value, repeated, as]",
      ));
    }

    let ident: Ident = input.parse()?;
    match () {
      () if ident.eq("key") || ident.eq("value") => {
        if !input.peek(Paren) {
          return Err(syn::Error::new(
            ident.span(),
            format!("Expected `(...)` after `{ident}`"),
          ));
        }

        let content;
        syn::parenthesized!(content in input);
        if content.is_empty() {
          return Err(syn::Error::new(
            ident.span(),
            format!("Expected `{ident}(...)`, `{ident}(..., repeated)` or `{ident}(as = \"...\")`"),
          ));
        }

        let label = Label::parse(&content)?;
        if ident.eq("key") {
          Ok(Self::Key(label))
        } else {
          Ok(Self::Value(label))
        }
      }
      () if ident.eq("repeated") => Ok(Self::Repeated),
      _ => Err(syn::Error::new(
        ident.span(),
        format!(
          "Expected one of [scalar, bytes, string, generic, object, enum, union, interface, map, set, list, nullable, repeated, as], found `{}`",
          ident
        ),
      )),
    }
  }
}

struct MapLabelParser(Either<MapLabel, Type>);

impl Parse for MapLabelParser {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let mut key = None;
    let mut value = None;
    let mut repeated = None;
    let mut as_type = None;
    let items = input.parse_terminated(MapLabelParserHelper::parse, Token![,])?;

    for item in items {
      match item {
        MapLabelParserHelper::Key(l) => {
          if let Some(ref old) = key {
            return Err(syn::Error::new(
              input.span(),
              format!(
                "Duplicate key found, expected only one key, found `key({})` and `key({})`",
                old, l
              ),
            ));
          }
          key = Some(l);
        }
        MapLabelParserHelper::Value(l) => {
          if let Some(ref old) = value {
            return Err(syn::Error::new(
              input.span(),
              format!(
                "Duplicate value found, expected only one value, found `value({})` and `value({})`",
                old, l
              ),
            ));
          }
          value = Some(l);
        }
        MapLabelParserHelper::Repeated => {
          if repeated.is_some() {
            return Err(syn::Error::new(
              input.span(),
              "Duplicate `repeated` found, only one `repeated` is allowed",
            ));
          }
          repeated = Some(true);
        }
        MapLabelParserHelper::As(ty) => {
          if as_type.is_some() {
            return Err(syn::Error::new(
              input.span(),
              format!("Duplicate `as` found, expected only one `as`"),
            ));
          }
          as_type = Some(ty);
        }
      }
    }
    let repeated = repeated.unwrap_or(false);

    Ok(match (key, value, repeated, as_type) {
      (Some(key), Some(value), true, None) => Self(Either::Left(MapLabel {
        key: Arc::new(key),
        value: Arc::new(value),
        repeated: true,
      })),
      (Some(key), Some(value), false, None) => Self(Either::Left(MapLabel {
        key: Arc::new(key),
        value: Arc::new(value),
        repeated: false,
      })),
      (None, None, false, Some(ty)) => Self(Either::Right(ty)),
      _ => {
        return Err(syn::Error::new(
          input.span(),
          "Expected `map(key(...), value(...))`, or `map(key(...), value(...), repeated)` or `map(as = \"...\")`",
        ));
      }
    })
  }
}

impl Parse for Label {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    if input.peek(syn::Token![enum]) {
      let _: syn::Token![enum] = input.parse()?;

      return parse_maybe_as(input, "enum").map(Self::Enum);
    }

    if !input.peek(syn::Ident::peek_any) {
      return Err(syn::Error::new(
        input.span(),
        "Expected one of [scalar, bytes, string, generic, object, enum, union, interface, map, set, list, nullable]",
      ));
    }

    let ident: syn::Ident = input.parse()?;
    Ok(match () {
      () if ident.eq("scalar") => parse_maybe_as(input, "scalar").map(Self::Scalar)?,
      () if ident.eq("bytes") => parse_maybe_as(input, "bytes").map(Self::Bytes)?,
      () if ident.eq("string") => parse_maybe_as(input, "string").map(Self::String)?,
      () if ident.eq("generic") => parse_maybe_as(input, "generic").map(Self::Generic)?,
      () if ident.eq("object") => parse_maybe_as(input, "object").map(Self::Object)?,
      () if ident.eq("enum") => parse_maybe_as(input, "enum").map(Self::Enum)?,
      () if ident.eq("union") => parse_maybe_as(input, "union").map(Self::Union)?,
      () if ident.eq("interface") => parse_maybe_as(input, "interface").map(Self::Interface)?,
      _ => {
        if input.is_empty() {
          return unexpected_eos_error(ident);
        }

        if !input.peek(Paren) {
          return Err(syn::Error::new(
            ident.span(),
            format!("Expected `(...)` after the {ident}"),
          ));
        }

        let content;
        syn::parenthesized!(content in input);

        if content.is_empty() {
          return unexpected_eos_error(ident);
        }

        return Ok(match () {
          () if ident.eq("nullable") => {
            let ty = Label::parse(&content)?;
            if ty.is_nullable() {
              return Err(syn::Error::new(
                content.span(),
                "`nullable(nullable(...))` is not allowed",
              ));
            }

            Self::Nullable(Arc::new(ty))
          }
          () if ident.eq("set") => {
            ListLikeLabelParser::<1>::parse(&content).map(|label| Self::Set(label.0))?
          }
          () if ident.eq("list") => {
            ListLikeLabelParser::<0>::parse(&content).map(|label| Self::List(label.0))?
          }
          () if ident.eq("key") => {
            return Err(syn::Error::new(
              content.span(),
              "`key` can only be used in `map(...)`",
            ));
          }
          () if ident.eq("value") => {
            return Err(syn::Error::new(
              content.span(),
              "`value` can only be used in `map(...)`",
            ));
          }
          () if ident.eq("map") => {
            MapLabelParser::parse(&content).map(|label| Self::Map(label.0))?
          }
          _ => {
            return Err(syn::Error::new(
              content.span(),
              "Expected one of [map, set, list, nullable]",
            ));
          }
        });
      }
    })
  }
}

// #[cfg(test)]
// mod tests {
//   use quote::quote;

//   use super::*;

//   #[test]
//   fn test_scalar() {
//     let scalar = quote! {
//       scalar
//     };

//     let ty = syn::parse2::<Label>(scalar).unwrap();
//     assert_eq!(ty, Label::Scalar(None));
//   }

//   #[test]
//   fn test_bytes() {
//     let bytes = quote! {
//       bytes
//     };

//     let ty = syn::parse2::<Label>(bytes).unwrap();
//     assert_eq!(ty, Label::Bytes(None));
//   }

//   #[test]
//   fn test_string() {
//     let string = quote! {
//       string
//     };

//     let ty = syn::parse2::<Label>(string).unwrap();
//     assert_eq!(ty, Label::String(None));
//   }

//   #[test]
//   fn test_object() {
//     let object = quote! {
//       object
//     };

//     let ty = syn::parse2::<Label>(object).unwrap();
//     assert_eq!(ty, Label::Object(None));
//   }

//   #[test]
//   fn test_enum() {
//     let enum_ty = quote! {
//       enum
//     };

//     let ty = syn::parse2::<Label>(enum_ty).unwrap();
//     assert_eq!(ty, Label::Enum(None));
//   }

//   #[test]
//   fn test_union() {
//     let union = quote! {
//       union
//     };

//     let ty = syn::parse2::<Label>(union).unwrap();
//     assert_eq!(ty, Label::Union(None));
//   }

//   #[test]
//   fn test_interface() {
//     let interface = quote! {
//       interface
//     };

//     let ty = syn::parse2::<Label>(interface).unwrap();
//     assert_eq!(ty, Label::Interface(None));
//   }

//   #[test]
//   fn test_map() {
//     let map = quote! {
//       map(key(scalar), value(string))
//     };

//     let ty = syn::parse2::<Label>(map).unwrap();
//     assert!(
//       matches!(ty, Label::Map { key, value, repeated: false, } if key.is_scalar() && value.is_string())
//     );
//   }

//   #[test]
//   fn test_map_nested_key() {
//     let map = quote! {
//       map(key(list(scalar)), value(string))
//     };

//     let ty = syn::parse2::<Label>(map).unwrap();
//     assert!(
//       matches!(ty, Label::Map { key, value, repeated: false, } if key.is_list() && value.is_string())
//     );
//   }

//   #[test]
//   fn test_map_nested_value() {
//     let map = quote! {
//       map(key(list(scalar)), value(map(key(string), value(object))))
//     };

//     let ty = syn::parse2::<Label>(map).unwrap();
//     assert!(
//       matches!(ty, Label::Map { key, value, repeated: false } if key.is_list() && value.is_map())
//     );
//   }

//   #[test]
//   fn test_set() {
//     let set = quote! {
//       set(string)
//     };

//     let ty = syn::parse2::<Label>(set).unwrap();
//     assert!(matches!(ty, Label::Set { key: inner, repeated: false } if inner.is_string()));
//   }

//   #[test]
//   fn test_list() {
//     let list = quote! {
//       list(object)
//     };

//     let ty = syn::parse2::<Label>(list).unwrap();
//     assert!(matches!(ty, Label::List { item: inner, repeated: false} if inner.is_object()));
//   }

//   #[test]
//   fn test_nullable() {
//     let nullable = quote! {
//       nullable(string)
//     };

//     let ty = syn::parse2::<Label>(nullable).unwrap();
//     assert!(matches!(ty, Label::Nullable(inner) if inner.is_string()));
//   }

//   #[test]
//   fn test_invalid_set() {
//     let invalid_set = quote! {
//       set(map(key(scalar), value(string)))
//     };

//     let result = syn::parse2::<Label>(invalid_set);
//     assert!(result.is_err());
//     assert!(
//       result
//         .unwrap_err()
//         .to_string()
//         .contains("`set(map(...))` is not allowed")
//     );
//   }

//   #[test]
//   fn test_invalid_set2() {
//     let invalid_list = quote! {
//       set(set(string))
//     };

//     let result = syn::parse2::<Label>(invalid_list);
//     assert!(result.is_err());
//     assert!(
//       result
//         .unwrap_err()
//         .to_string()
//         .contains("`set(set(...))` is not allowed")
//     );
//   }

//   #[test]
//   fn test_invalid_map() {
//     let invalid_map = quote! {
//       map(scalar)
//     };

//     let result = syn::parse2::<Label>(invalid_map);
//     assert!(result.is_err());
//     assert!(
//       result
//         .unwrap_err()
//         .to_string()
//         .contains("Unknown `scalar`, possible attributes in map are: `key` or `value`")
//     );
//   }

//   #[test]
//   fn test_invalid_map2() {
//     let invalid_map = quote! {
//       map()
//     };

//     let result = syn::parse2::<Label>(invalid_map);
//     assert!(result.is_err());
//     assert!(
//       result
//         .unwrap_err()
//         .to_string()
//         .contains("Unexpected format `map()`, expected `map(key(...), value(...))`")
//     );
//   }

//   #[test]
//   fn test_invalid_map3() {
//     let invalid_map = quote! {
//       map(key)
//     };

//     let result = syn::parse2::<Label>(invalid_map);
//     assert!(result.is_err());
//     assert!(
//       result
//         .unwrap_err()
//         .to_string()
//         .contains("Unexpected format `map(key)`, expected `map(key(...), value(...))`")
//     );
//   }

//   #[test]
//   fn test_invalid_map_key() {
//     let invalid_map = quote! {
//       map(key(map(key(scalar))))
//     };

//     let result = syn::parse2::<Label>(invalid_map);
//     assert!(result.is_err());
//     assert!(result.unwrap_err().to_string().contains("`map(key(map(...)), value(...))` is not allowed, because the `key` of a `map` cannot be another `map`"));
//   }

//   #[test]
//   fn test_invalid_nullable() {
//     let invalid_set = quote! {
//       nullable(nullable(string))
//     };

//     let result = syn::parse2::<Label>(invalid_set);
//     assert!(result.is_err());
//     assert!(
//       result
//         .unwrap_err()
//         .to_string()
//         .contains("`nullable(nullable(...))` is not allowed")
//     );
//   }
// }
