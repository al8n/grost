use either::Either;
use quote::quote;
use syn::{
  Ident, Meta, Path, Token, Type, WherePredicate,
  ext::IdentExt,
  parse::{Parse, ParseStream},
  punctuated::Punctuated,
  spanned::Spanned,
  token::{Comma, Paren},
};

pub use generic::*;
pub use list_like::*;
pub use map::*;
pub use nullable::*;

mod generic;
mod list_like;
mod map;
mod nullable;

const LIST_TAG: u8 = 0;
const SET_TAG: u8 = 1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefaultWireFormat {
  ty: Type,
  constraints: Punctuated<WherePredicate, Comma>,
}

impl DefaultWireFormat {
  /// Creates a new `DefaultWireFormat` with the given type and constraints.
  fn new(ty: Type) -> Self {
    Self {
      ty,
      constraints: Punctuated::new(),
    }
  }

  fn with_constraints(ty: Type, constraints: Punctuated<WherePredicate, Comma>) -> Self {
    Self { ty, constraints }
  }

  /// Returns the type of the default wire format.
  pub fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the constraints of the default wire format.
  pub fn constraints(&self) -> &Punctuated<WherePredicate, Comma> {
    &self.constraints
  }

  /// Consumes the `DefaultWireFormat`
  pub fn into_components(self) -> (Type, Punctuated<WherePredicate, Comma>) {
    (self.ty, self.constraints)
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
  #[display("generic")]
  Generic(GenericLabelValue),
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
  #[display("nullable")]
  Nullable(Either<NullableLabel, Type>),
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

  /// Returns the default wire format type for this label.
  pub fn default_wire_format(
    &self,
    path_to_grost: &Path,
    flavor_type: &Type,
    ty: &Type,
    tag: u32,
  ) -> syn::Result<DefaultWireFormat> {
    self.default_wire_format_helper(path_to_grost, flavor_type, ty, tag, true)
  }

  fn default_wire_format_helper(
    &self,
    path_to_grost: &Path,
    flavor_type: &Type,
    ty: &Type,
    tag: u32,
    outermost: bool,
  ) -> syn::Result<DefaultWireFormat> {
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
      Self::Scalar(wf) => DefaultWireFormat::new(match wf {
        None => syn::parse2(quote! {
          #path_to_grost::__private::marker::ScalarMarker<#ty>
        })?,
        Some(wf) => syn::parse2(quote! {
          #path_to_grost::__private::marker::WireFormatMarker<#ty, #wf>
        })?,
      }),
      Self::Bytes(wf) => DefaultWireFormat::new(match wf {
        None => syn::parse2(quote! {
          #path_to_grost::__private::marker::BytesMarker<#ty>
        })?,
        Some(wf) => syn::parse2(quote! {
          #path_to_grost::__private::marker::WireFormatMarker<#ty, #wf>
        })?,
      }),
      Self::String(wf) => DefaultWireFormat::new(match wf {
        None => syn::parse2(quote! {
          #path_to_grost::__private::marker::StringMarker<#ty>
        })?,
        Some(wf) => syn::parse2(quote! {
          #path_to_grost::__private::marker::WireFormatMarker<#ty, #wf>
        })?,
      }),
      Self::Object(wf) => DefaultWireFormat::new(match wf {
        None => syn::parse2(quote! {
          #path_to_grost::__private::marker::ObjectMarker<#ty>
        })?,
        Some(wf) => syn::parse2(quote! {
          #path_to_grost::__private::marker::WireFormatMarker<#ty, #wf>
        })?,
      }),
      Self::Enum(wf) => DefaultWireFormat::new(match wf {
        None => syn::parse2(quote! {
          #path_to_grost::__private::marker::EnumMarker<#ty>
        })?,
        Some(wf) => syn::parse2(quote! {
          #path_to_grost::__private::marker::WireFormatMarker<#ty, #wf>
        })?,
      }),
      Self::Union(wf) => DefaultWireFormat::new(match wf {
        None => syn::parse2(quote! {
          #path_to_grost::__private::marker::UnionMarker<#ty>
        })?,
        Some(wf) => syn::parse2(quote! {
          #path_to_grost::__private::marker::WireFormatMarker<#ty, #wf>
        })?,
      }),
      Self::Interface(wf) => DefaultWireFormat::new(match wf {
        None => syn::parse2(quote! {
          #path_to_grost::__private::marker::InterfaceMarker<#ty>
        })?,
        Some(wf) => syn::parse2(quote! {
          #path_to_grost::__private::marker::WireFormatMarker<#ty, #wf>
        })?,
      }),
      Self::Generic(value) => match value {
        GenericLabelValue::Marker(marker) => {
          let mut constraints = Punctuated::new();
          constraints.push(syn::parse2(quote! {
            #marker: #path_to_grost::__private::flavors::DefaultWireFormat<#flavor_type> + #path_to_grost::__private::marker::Marker<Marked = #ty>
          })?);

          DefaultWireFormat::with_constraints(
            syn::parse2(quote! {
              #path_to_grost::__private::marker::GenericMarker<#ty, #marker>
            })?,
            constraints,
          )
        }
        GenericLabelValue::As(wf) => {
          let mut constraints = Punctuated::new();
          let marker: Type = syn::parse2(quote! {
            #path_to_grost::__private::marker::WireFormatMarker<#ty, #wf>
          })?;
          constraints.push(syn::parse2(quote! {
            #marker: #path_to_grost::__private::flavors::DefaultWireFormat<#flavor_type>
          })?);

          DefaultWireFormat::with_constraints(marker, constraints)
        }
      },
      Self::Map(Either::Right(wf)) => DefaultWireFormat::new(syn::parse2(quote! {
        #path_to_grost::__private::marker::WireFormatMarker<#ty, #wf>
      })?),
      Self::Map(Either::Left(MapLabel {
        key,
        value,
        repeated,
      })) => {
        let k: Type = syn::parse2(quote! {
          <#ty as #path_to_grost::__private::convert::State<
            #path_to_grost::__private::convert::Flattened<#path_to_grost::__private::convert::MapKey>,
          >>::Output
        })?;
        let v: Type = syn::parse2(quote! {
          <#ty as #path_to_grost::__private::convert::State<
            #path_to_grost::__private::convert::Flattened<#path_to_grost::__private::convert::MapValue>,
          >>::Output
        })?;
        let (k, kcs) = key
          .default_wire_format_helper(path_to_grost, flavor_type, &k, INNER_TAG, false)?
          .into_components();
        let (v, vcs) = value
          .default_wire_format_helper(path_to_grost, flavor_type, &v, INNER_TAG, false)?
          .into_components();

        let ty = if *repeated {
          syn::parse2(quote! {
            #path_to_grost::__private::marker::RepeatedEntryMarker<#ty, #k, #v, #tag>
          })?
        } else {
          syn::parse2(quote! {
            #path_to_grost::__private::marker::MapMarker<#ty, #k, #v>
          })?
        };

        DefaultWireFormat::with_constraints(ty, kcs.into_iter().chain(vcs).collect())
      }
      Self::Set(Either::Right(wf)) => DefaultWireFormat::new(syn::parse2(quote! {
        #path_to_grost::__private::marker::WireFormatMarker<#ty, #wf>
      })?),
      Self::Set(Either::Left(ListLikeLabel { label, repeated })) => {
        let inner_ty: Type = syn::parse2(quote! {
          <#ty as #path_to_grost::__private::convert::State<
            #path_to_grost::__private::convert::Flattened<#path_to_grost::__private::convert::Inner>,
          >>::Output
        })?;
        let (inner, c) = label
          .default_wire_format_helper(path_to_grost, flavor_type, &inner_ty, INNER_TAG, false)?
          .into_components();
        let ty = if *repeated {
          syn::parse2(quote! {
            #path_to_grost::__private::marker::RepeatedMarker<#ty, #inner, #tag>
          })?
        } else {
          syn::parse2(quote! {
            #path_to_grost::__private::marker::SetMarker<#ty, #inner>
          })?
        };
        DefaultWireFormat::with_constraints(ty, c)
      }
      Self::List(Either::Right(wf)) => DefaultWireFormat::new(syn::parse2(quote! {
        #path_to_grost::__private::marker::WireFormatMarker<#ty, #wf>
      })?),
      Self::List(Either::Left(ListLikeLabel { label, repeated })) => {
        let inner_ty: Type = syn::parse2(quote! {
          <#ty as #path_to_grost::__private::convert::State<
            #path_to_grost::__private::convert::Flattened<#path_to_grost::__private::convert::Inner>,
          >>::Output
        })?;
        let (inner, c) = label
          .default_wire_format_helper(path_to_grost, flavor_type, &inner_ty, INNER_TAG, false)?
          .into_components();

        let ty = if *repeated {
          syn::parse2(quote! {
            #path_to_grost::__private::marker::RepeatedMarker<#ty, #inner, #tag>
          })?
        } else {
          syn::parse2(quote! {
            #path_to_grost::__private::marker::ListMarker<#ty, #inner>
          })?
        };

        DefaultWireFormat::with_constraints(ty, c)
      }
      Self::Nullable(Either::Right(wf)) => DefaultWireFormat::new(syn::parse2(quote! {
        #path_to_grost::__private::marker::WireFormatMarker<#ty, #wf>
      })?),
      Self::Nullable(Either::Left(label)) => {
        let inner_ty: Type = syn::parse2(quote! {
          <#ty as #path_to_grost::__private::convert::State<
            #path_to_grost::__private::convert::Flattened<#path_to_grost::__private::convert::Inner>,
          >>::Output
        })?;
        let (inner, c) = label
          .label()
          .default_wire_format_helper(path_to_grost, flavor_type, &inner_ty, tag, false)?
          .into_components();

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
        let ty = if outermost {
          syn::parse2(quote! {
            #path_to_grost::__private::marker::FlattenMarker<#ty, #inner>
          })?
        } else {
          syn::parse2(quote! {
            #path_to_grost::__private::marker::NullableMarker<#ty, #inner>
          })?
        };

        DefaultWireFormat::with_constraints(ty, c)
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
      let ty: Type = parse_type(&content)?;

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
    () if ident.eq("map") => Err(syn::Error::new(
      ident.span(),
      "`map` requires a key and value, e.g. `map(key(...), value(...))`, `map(key(...), value(...), repeated)` or `map(as = \"...\")`",
    )),
    () if ident.eq("set") => Err(syn::Error::new(
      ident.span(),
      "`set` requires a type, e.g. `set(...)` or `set(..., repeated)`",
    )),
    () if ident.eq("list") => Err(syn::Error::new(
      ident.span(),
      "`list` requires a type, e.g. `list(...)` or `list(..., repeated)`",
    )),
    () if ident.eq("nullable") => Err(syn::Error::new(
      ident.span(),
      "`nullable` requires a type, e.g. `nullable(...)`",
    )),
    () if ident.eq("key") => Err(syn::Error::new(
      ident.span(),
      "`key` can only be used in `map(...)`",
    )),
    () if ident.eq("value") => Err(syn::Error::new(
      ident.span(),
      "`value` can only be used in `map(...)`",
    )),
    _ => Err(syn::Error::new(
      ident.span(),
      "Expected one of [scalar, bytes, string, generic, object, enum, union, interface, map, set, list, nullable]",
    )),
  }
}

fn parse_type(input: ParseStream) -> syn::Result<Type> {
  let expr: syn::ExprLit = input.parse()?;
  match expr.lit {
    syn::Lit::Str(lit_str) => syn::parse_str(lit_str.value().as_str()),
    _ => Err(syn::Error::new(
      expr.span(),
      "Expected a string literal for type",
    )),
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
          () if ident.eq("generic") => {
            GenericLabelParser::parse(&content).map(|v| Self::Generic(v.0))?
          }
          () if ident.eq("nullable") => {
            NullableLabelParser::parse(&content).map(|val| Self::Nullable(val.0))?
          }
          () if ident.eq("set") => {
            ListLikeLabelParser::<SET_TAG>::parse(&content).map(|label| Self::Set(label.0))?
          }
          () if ident.eq("list") => {
            ListLikeLabelParser::<LIST_TAG>::parse(&content).map(|label| Self::List(label.0))?
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
