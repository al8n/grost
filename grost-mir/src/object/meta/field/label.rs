use quote::quote;
use std::sync::Arc;
use syn::{Meta, ext::IdentExt, parse::ParseStream};

/// A type specification for an object field.
#[derive(Debug, Clone, PartialEq, Eq, derive_more::IsVariant, derive_more::Display)]
#[non_exhaustive]
pub enum Label {
  /// A scalar type label, e.g. `i32`, `f64`, etc.
  #[display("scalar")]
  Scalar,
  /// A byte array type label, e.g. `Vec<u8>`, `bytes`, etc.
  #[display("bytes")]
  Bytes,
  /// A string type label, e.g. `String`, `&str`, etc.
  #[display("string")]
  String,
  /// An object type label
  #[display("object")]
  Object,
  /// An enum type label
  #[display("enum")]
  Enum,
  /// A union type label
  #[display("union")]
  Union,
  /// An interface type label
  #[display("interface")]
  Interface,
  /// A map type label
  #[display("map(key({key}), value({value}))")]
  Map {
    /// The key type label of the map
    key: Arc<Label>,
    /// The value type label of the map
    value: Arc<Label>,
  },
  /// A set type label
  #[display("set({_0})")]
  Set(Arc<Label>),
  /// A list type label
  #[display("list({_0})")]
  List(Arc<Label>),
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
        _ => false,
      });
    }

    Ok(false)
  }

  /// Construct a mark type for the type based on the label.
  pub fn mark(&self, path_to_grost: &syn::Path, ty: &syn::Type) -> syn::Result<syn::Type> {
    Ok(match self {
      Self::Scalar => syn::parse2(quote! {
        #path_to_grost::__private::marker::ScalarMarker<#ty>
      })?,
      Self::Bytes => syn::parse2(quote! {
        #path_to_grost::__private::marker::BytesMarker<#ty>
      })?,
      Self::String => syn::parse2(quote! {
        #path_to_grost::__private::marker::StringMarker<#ty>
      })?,
      Self::Object => syn::parse2(quote! {
        #path_to_grost::__private::marker::ObjectMarker<#ty>
      })?,
      Self::Enum => syn::parse2(quote! {
        #path_to_grost::__private::marker::EnumMarker<#ty>
      })?,
      Self::Union => syn::parse2(quote! {
        #path_to_grost::__private::marker::UnionMarker<#ty>
      })?,
      Self::Interface => syn::parse2(quote! {
        #path_to_grost::__private::marker::InterfaceMarker<#ty>
      })?,
      Self::Map { key, value } => {
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
        let km = key.mark(path_to_grost, &k)?;
        let vm = value.mark(path_to_grost, &v)?;

        syn::parse2(quote! {
          #path_to_grost::__private::marker::MapMarker<#ty, #km, #vm>
        })?
      }
      Self::Set(label) => {
        let inner_ty: syn::Type = syn::parse2(quote! {
          <#ty as #path_to_grost::__private::convert::State<
            #path_to_grost::__private::convert::Flattened<#path_to_grost::__private::convert::Inner>,
          >>::Output
        })?;
        let inner = label.mark(path_to_grost, &inner_ty)?;
        syn::parse2(quote! {
          #path_to_grost::__private::marker::SetMarker<#ty, #inner>
        })?
      }
      Self::List(label) => {
        let inner_ty: syn::Type = syn::parse2(quote! {
          <#ty as #path_to_grost::__private::convert::State<
            #path_to_grost::__private::convert::Flattened<#path_to_grost::__private::convert::Inner>,
          >>::Output
        })?;
        let inner = label.mark(path_to_grost, &inner_ty)?;
        syn::parse2(quote! {
          #path_to_grost::__private::marker::ListMarker<#ty, #inner>
        })?
      }
      Self::Nullable(label) => {
        let inner_ty: syn::Type = syn::parse2(quote! {
          <#ty as #path_to_grost::__private::convert::State<
            #path_to_grost::__private::convert::Flattened<#path_to_grost::__private::convert::Inner>,
          >>::Output
        })?;
        let inner = label.mark(path_to_grost, &inner_ty)?;
        syn::parse2(quote! {
          #path_to_grost::__private::marker::NullableMarker<#ty, #inner>
        })?
      }
    })
  }
}

impl syn::parse::Parse for Label {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    if input.peek(syn::Token![enum]) {
      let _: syn::Token![enum] = input.parse()?;
      return Ok(Self::Enum);
    }

    if input.peek(syn::Ident::peek_any) && !input.peek2(syn::token::Paren) {
      let ident: syn::Ident = input.parse()?;
      return Ok(match () {
        () if ident.eq("scalar") => Self::Scalar,
        () if ident.eq("bytes") => Self::Bytes,
        () if ident.eq("string") => Self::String,
        () if ident.eq("object") => Self::Object,
        () if ident.eq("enum") => Self::Enum,
        () if ident.eq("union") => Self::Union,
        () if ident.eq("interface") => Self::Interface,
        () if ident.eq("map") => {
          return Err(syn::Error::new(
            ident.span(),
            "`map` requires a key and value, e.g. `map(key(...), value(...))`",
          ));
        }
        () if ident.eq("set") => {
          return Err(syn::Error::new(
            ident.span(),
            "`set` requires a type, e.g. `set(...)`",
          ));
        }
        () if ident.eq("list") => {
          return Err(syn::Error::new(
            ident.span(),
            "`list` requires a type, e.g. `list(...)`",
          ));
        }
        () if ident.eq("nullable") => {
          return Err(syn::Error::new(
            ident.span(),
            "`nullable` requires a type, e.g. `nullable(...)`",
          ));
        }
        _ => {
          return Err(syn::Error::new(
            ident.span(),
            "Expected one of [scalar, bytes, string, object, enum, union, interface, map, set, list, nullable]",
          ));
        }
      });
    }

    if input.peek(syn::Ident::peek_any) && input.peek2(syn::token::Paren) {
      let ident: syn::Ident = input.parse()?;
      let content;
      syn::parenthesized!(content in input);

      if content.is_empty() {
        return Err(syn::Error::new(
          content.span(),
          "Unexpected format `map()`, expected `map(key(...), value(...))`",
        ));
      }

      return Ok(match () {
        () if ident.eq("map") => {
          let mut key_ty = None;
          let mut value_ty = None;

          while !content.is_empty() {
            let require_comma = key_ty.is_none() && value_ty.is_none();

            let param_name: syn::Ident = content.parse()?;

            if !(param_name.eq("key") || param_name.eq("value")) {
              return Err(syn::Error::new(
                param_name.span(),
                format!("Unknown `{param_name}`, possible attributes in map are: `key` or `value`"),
              ));
            }

            if !content.peek(syn::token::Paren) {
              return Err(syn::Error::new(
                param_name.span(),
                format!(
                  "Unexpected format `map({param_name}{content})`, expected `map(key(...), value(...))`"
                ),
              ));
            }

            let param_content;
            syn::parenthesized!(param_content in content);
            match () {
              () if param_name.eq("key") => {
                if key_ty.is_some() {
                  return Err(syn::Error::new(
                    param_name.span(),
                    "Duplicate `key` found in `map(...)`",
                  ));
                }

                if param_content.peek(syn::Ident::peek_any) {
                  let next_ident: syn::Ident = param_content.fork().parse()?;
                  if next_ident.eq("map") {
                    return Err(syn::Error::new(
                      param_name.span(),
                      "`map(key(map(...)), value(...))` is not allowed, because the `key` of a `map` cannot be another `map`",
                    ));
                  }
                }

                key_ty = Some(Arc::new(Label::parse(&param_content)?));
              }
              () if param_name.eq("value") => {
                if value_ty.is_some() {
                  return Err(syn::Error::new(
                    param_name.span(),
                    "Duplicate `value` found in `map(...)`",
                  ));
                }
                value_ty = Some(Arc::new(Label::parse(&param_content)?));
              }
              _ => {
                return Err(syn::Error::new(
                  param_name.span(),
                  format!(
                    "Unexpected `{param_name}` in `map(...)`, expected `key(...)` or `value(...)`"
                  ),
                ));
              }
            }

            if require_comma || content.peek(syn::Token![,]) {
              let _: syn::Token![,] = content.parse()?;
            }
          }

          // Ensure both key and value were provided
          let key = key_ty
            .ok_or_else(|| syn::Error::new(content.span(), "Missing `key(...)` in `map(...)`"))?;

          let value = value_ty
            .ok_or_else(|| syn::Error::new(content.span(), "Missing `value(...)` in `map(...)`"))?;

          Self::Map { key, value }
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
        () if ident.eq("set") => {
          let ty = Label::parse(&content)?;
          if ty.is_set() {
            return Err(syn::Error::new(
              content.span(),
              "`set(set(...))` is not allowed",
            ));
          }

          if ty.is_map() {
            return Err(syn::Error::new(
              content.span(),
              "`set(map(...))` is not allowed",
            ));
          }

          Self::Set(Arc::new(ty))
        }
        () if ident.eq("list") => Self::List(Arc::new(Label::parse(&content)?)),
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
        _ => {
          return Err(syn::Error::new(
            content.span(),
            "Expected one of [map, set, list, nullable]",
          ));
        }
      });
    }
    Err(syn::Error::new(
      input.span(),
      "Expected one of [scalar, bytes, string, object, enum, union, interface, map, set, list, nullable]",
    ))
  }
}

#[cfg(test)]
mod tests {
  use quote::quote;

  use super::*;

  #[test]
  fn test_scalar() {
    let scalar = quote! {
      scalar
    };

    let ty = syn::parse2::<Label>(scalar).unwrap();
    assert_eq!(ty, Label::Scalar);
  }

  #[test]
  fn test_bytes() {
    let bytes = quote! {
      bytes
    };

    let ty = syn::parse2::<Label>(bytes).unwrap();
    assert_eq!(ty, Label::Bytes);
  }

  #[test]
  fn test_string() {
    let string = quote! {
      string
    };

    let ty = syn::parse2::<Label>(string).unwrap();
    assert_eq!(ty, Label::String);
  }

  #[test]
  fn test_object() {
    let object = quote! {
      object
    };

    let ty = syn::parse2::<Label>(object).unwrap();
    assert_eq!(ty, Label::Object);
  }

  #[test]
  fn test_enum() {
    let enum_ty = quote! {
      enum
    };

    let ty = syn::parse2::<Label>(enum_ty).unwrap();
    assert_eq!(ty, Label::Enum);
  }

  #[test]
  fn test_union() {
    let union = quote! {
      union
    };

    let ty = syn::parse2::<Label>(union).unwrap();
    assert_eq!(ty, Label::Union);
  }

  #[test]
  fn test_interface() {
    let interface = quote! {
      interface
    };

    let ty = syn::parse2::<Label>(interface).unwrap();
    assert_eq!(ty, Label::Interface);
  }

  #[test]
  fn test_map() {
    let map = quote! {
      map(key(scalar), value(string))
    };

    let ty = syn::parse2::<Label>(map).unwrap();
    assert!(matches!(ty, Label::Map { key, value } if key.is_scalar() && value.is_string()));
  }

  #[test]
  fn test_map_nested_key() {
    let map = quote! {
      map(key(list(scalar)), value(string))
    };

    let ty = syn::parse2::<Label>(map).unwrap();
    assert!(matches!(ty, Label::Map { key, value } if key.is_list() && value.is_string()));
  }

  #[test]
  fn test_map_nested_value() {
    let map = quote! {
      map(key(list(scalar)), value(map(key(string), value(object))))
    };

    let ty = syn::parse2::<Label>(map).unwrap();
    assert!(matches!(ty, Label::Map { key, value } if key.is_list() && value.is_map()));
  }

  #[test]
  fn test_set() {
    let set = quote! {
      set(string)
    };

    let ty = syn::parse2::<Label>(set).unwrap();
    assert!(matches!(ty, Label::Set(inner) if inner.is_string()));
  }

  #[test]
  fn test_list() {
    let list = quote! {
      list(object)
    };

    let ty = syn::parse2::<Label>(list).unwrap();
    assert!(matches!(ty, Label::List(inner) if inner.is_object()));
  }

  #[test]
  fn test_nullable() {
    let nullable = quote! {
      nullable(string)
    };

    let ty = syn::parse2::<Label>(nullable).unwrap();
    assert!(matches!(ty, Label::Nullable(inner) if inner.is_string()));
  }

  #[test]
  fn test_invalid_set() {
    let invalid_set = quote! {
      set(map(key(scalar), value(string)))
    };

    let result = syn::parse2::<Label>(invalid_set);
    assert!(result.is_err());
    assert!(
      result
        .unwrap_err()
        .to_string()
        .contains("`set(map(...))` is not allowed")
    );
  }

  #[test]
  fn test_invalid_set2() {
    let invalid_list = quote! {
      set(set(string))
    };

    let result = syn::parse2::<Label>(invalid_list);
    assert!(result.is_err());
    assert!(
      result
        .unwrap_err()
        .to_string()
        .contains("`set(set(...))` is not allowed")
    );
  }

  #[test]
  fn test_invalid_map() {
    let invalid_map = quote! {
      map(scalar)
    };

    let result = syn::parse2::<Label>(invalid_map);
    assert!(result.is_err());
    assert!(
      result
        .unwrap_err()
        .to_string()
        .contains("Unknown `scalar`, possible attributes in map are: `key` or `value`")
    );
  }

  #[test]
  fn test_invalid_map2() {
    let invalid_map = quote! {
      map()
    };

    let result = syn::parse2::<Label>(invalid_map);
    assert!(result.is_err());
    assert!(
      result
        .unwrap_err()
        .to_string()
        .contains("Unexpected format `map()`, expected `map(key(...), value(...))`")
    );
  }

  #[test]
  fn test_invalid_map3() {
    let invalid_map = quote! {
      map(key)
    };

    let result = syn::parse2::<Label>(invalid_map);
    assert!(result.is_err());
    assert!(
      result
        .unwrap_err()
        .to_string()
        .contains("Unexpected format `map(key)`, expected `map(key(...), value(...))`")
    );
  }

  #[test]
  fn test_invalid_map_key() {
    let invalid_map = quote! {
      map(key(map(key(scalar))))
    };

    let result = syn::parse2::<Label>(invalid_map);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("`map(key(map(...)), value(...))` is not allowed, because the `key` of a `map` cannot be another `map`"));
  }

  #[test]
  fn test_invalid_nullable() {
    let invalid_set = quote! {
      nullable(nullable(string))
    };

    let result = syn::parse2::<Label>(invalid_set);
    assert!(result.is_err());
    assert!(
      result
        .unwrap_err()
        .to_string()
        .contains("`nullable(nullable(...))` is not allowed")
    );
  }
}
