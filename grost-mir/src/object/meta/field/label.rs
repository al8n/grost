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
pub struct LabelInformation {
  pub(in crate::object) wire_format: Type,
  pub(in crate::object) wire_format_constraints: Punctuated<WherePredicate, Comma>,
}

impl LabelInformation {
  /// Creates a new `LabelInformation` with the given type and wire_format_constraints.
  fn new(wire_format: Type) -> Self {
    Self {
      wire_format,
      wire_format_constraints: Punctuated::new(),
    }
  }

  fn with_wire_format_constraints(
    wire_format: Type,
    wire_format_constraints: Punctuated<WherePredicate, Comma>,
  ) -> Self {
    Self {
      wire_format,
      wire_format_constraints,
    }
  }

  /// Returns the type of the default wire format.
  pub fn wire_format(&self) -> &Type {
    &self.wire_format
  }

  /// Returns the constraints of the default wire format.
  pub fn wire_format_constraints(&self) -> &Punctuated<WherePredicate, Comma> {
    &self.wire_format_constraints
  }

  /// Consumes the `LabelInformation`
  pub fn into_wire_format_components(self) -> (Type, Punctuated<WherePredicate, Comma>) {
    (self.wire_format, self.wire_format_constraints)
  }
}

/// A type specification for an object field.
#[derive(Debug, Clone, derive_more::IsVariant, derive_more::Display)]
#[non_exhaustive]
#[allow(clippy::large_enum_variant)]
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
  /// Returns `true` if the inner type is generic.
  /// 
  /// e.g.
  /// - `map(key(generic(...)), value(string)): true``
  /// - `set(generic(...)): true`
  /// - `list(generic(...)): true`
  /// - `nullable(generic(...)): true`
  /// - `generic(...): false`
  pub fn is_inner_generic(&self) -> bool {
    match self {
      Label::Map(Either::Left(map_label)) => map_label.key().is_generic() || map_label.value().is_generic(),
      Label::Set(Either::Left(label)) => label.label().is_generic(),
      Label::List(Either::Left(label)) => label.label().is_generic(),
      Label::Nullable(Either::Left(label)) => label.label().is_generic(),
      _ => false,
    }
  }

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
  pub fn label_info(
    &self,
    path_to_grost: &Path,
    flavor_type: &Type,
    ty: &Type,
    tag: u32,
  ) -> syn::Result<LabelInformation> {
    self.label_info_helper(path_to_grost, flavor_type, ty, tag, true)
  }

  fn label_info_helper(
    &self,
    path_to_grost: &Path,
    flavor_type: &Type,
    ty: &Type,
    tag: u32,
    outermost: bool,
  ) -> syn::Result<LabelInformation> {
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
      Self::Scalar(wf) => LabelInformation::new(match wf {
        None => syn::parse2(quote! {
          #path_to_grost::__private::marker::ScalarMarker<#ty>
        })?,
        Some(wf) => syn::parse2(quote! {
          #path_to_grost::__private::marker::WireFormatMarker<#ty, #wf>
        })?,
      }),
      Self::Bytes(wf) => LabelInformation::new(match wf {
        None => syn::parse2(quote! {
          #path_to_grost::__private::marker::BytesMarker<#ty>
        })?,
        Some(wf) => syn::parse2(quote! {
          #path_to_grost::__private::marker::WireFormatMarker<#ty, #wf>
        })?,
      }),
      Self::String(wf) => LabelInformation::new(match wf {
        None => syn::parse2(quote! {
          #path_to_grost::__private::marker::StringMarker<#ty>
        })?,
        Some(wf) => syn::parse2(quote! {
          #path_to_grost::__private::marker::WireFormatMarker<#ty, #wf>
        })?,
      }),
      Self::Object(wf) => LabelInformation::new(match wf {
        None => syn::parse2(quote! {
          #path_to_grost::__private::marker::ObjectMarker<#ty>
        })?,
        Some(wf) => syn::parse2(quote! {
          #path_to_grost::__private::marker::WireFormatMarker<#ty, #wf>
        })?,
      }),
      Self::Enum(wf) => LabelInformation::new(match wf {
        None => syn::parse2(quote! {
          #path_to_grost::__private::marker::EnumMarker<#ty>
        })?,
        Some(wf) => syn::parse2(quote! {
          #path_to_grost::__private::marker::WireFormatMarker<#ty, #wf>
        })?,
      }),
      Self::Union(wf) => LabelInformation::new(match wf {
        None => syn::parse2(quote! {
          #path_to_grost::__private::marker::UnionMarker<#ty>
        })?,
        Some(wf) => syn::parse2(quote! {
          #path_to_grost::__private::marker::WireFormatMarker<#ty, #wf>
        })?,
      }),
      Self::Interface(wf) => LabelInformation::new(match wf {
        None => syn::parse2(quote! {
          #path_to_grost::__private::marker::InterfaceMarker<#ty>
        })?,
        Some(wf) => syn::parse2(quote! {
          #path_to_grost::__private::marker::WireFormatMarker<#ty, #wf>
        })?,
      }),
      Self::Generic(GenericLabelValue { value, .. }) => match value {
        GenericLabelValueVariant::Marker { marker, marked } => {
          let mut wire_format_constraints = Punctuated::new();

          let marked = marked.as_ref().unwrap_or(ty);
          wire_format_constraints.push(syn::parse2(quote! {
            #marker: #path_to_grost::__private::flavors::DefaultWireFormat<#flavor_type> + #path_to_grost::__private::marker::Marker<Marked = #marked>
          })?);

          LabelInformation::with_wire_format_constraints(
            syn::parse2(quote! {
              #path_to_grost::__private::marker::GenericMarker<#marked, #marker>
            })?,
            wire_format_constraints,
          )
        }
        GenericLabelValueVariant::As {
          wire_format: wf,
          marked,
        } => {
          let mut wire_format_constraints = Punctuated::new();

          let marked = marked.as_ref().unwrap_or(ty);
          let marker: Type = syn::parse2(quote! {
            #path_to_grost::__private::marker::WireFormatMarker<#marked, #wf>
          })?;
          wire_format_constraints.push(syn::parse2(quote! {
            #marker: #path_to_grost::__private::flavors::DefaultWireFormat<#flavor_type>
          })?);

          LabelInformation::with_wire_format_constraints(marker, wire_format_constraints)
        }
      },
      Self::Map(Either::Right(wf)) => LabelInformation::new(syn::parse2(quote! {
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

        let LabelInformation {
          wire_format: kwf,
          wire_format_constraints: kwfc,
        } = key.label_info_helper(path_to_grost, flavor_type, &k, INNER_TAG, false)?;
        let LabelInformation {
          wire_format: vwf,
          wire_format_constraints: vwfc,
        } = value.label_info_helper(path_to_grost, flavor_type, &v, INNER_TAG, false)?;

        let ty = if *repeated {
          syn::parse2(quote! {
            #path_to_grost::__private::marker::RepeatedEntryMarker<#ty, #kwf, #vwf, #tag>
          })?
        } else {
          syn::parse2(quote! {
            #path_to_grost::__private::marker::MapMarker<#ty, #kwf, #vwf>
          })?
        };

        LabelInformation::with_wire_format_constraints(
          ty,
          kwfc.into_iter().chain(vwfc).collect(),
        )
      }
      Self::Set(Either::Right(wf)) => LabelInformation::new(syn::parse2(quote! {
        #path_to_grost::__private::marker::WireFormatMarker<#ty, #wf>
      })?),
      Self::Set(Either::Left(ListLikeLabel { label, repeated })) => {
        let inner_ty: Type = syn::parse2(quote! {
          <#ty as #path_to_grost::__private::convert::State<
            #path_to_grost::__private::convert::Flattened<#path_to_grost::__private::convert::Inner>,
          >>::Output
        })?;

        let LabelInformation {
          wire_format: inner_wf,
          wire_format_constraints,
        } = label.label_info_helper(path_to_grost, flavor_type, &inner_ty, INNER_TAG, false)?;
        let ty = if *repeated {
          syn::parse2(quote! {
            #path_to_grost::__private::marker::RepeatedMarker<#ty, #inner_wf, #tag>
          })?
        } else {
          syn::parse2(quote! {
            #path_to_grost::__private::marker::SetMarker<#ty, #inner_wf>
          })?
        };
        LabelInformation::with_wire_format_constraints(
          ty,
          wire_format_constraints,
        )
      }
      Self::List(Either::Right(wf)) => LabelInformation::new(syn::parse2(quote! {
        #path_to_grost::__private::marker::WireFormatMarker<#ty, #wf>
      })?),
      Self::List(Either::Left(ListLikeLabel { label, repeated })) => {
        let inner_ty: Type = syn::parse2(quote! {
          <#ty as #path_to_grost::__private::convert::State<
            #path_to_grost::__private::convert::Flattened<#path_to_grost::__private::convert::Inner>,
          >>::Output
        })?;

        let LabelInformation {
          wire_format: inner,
          wire_format_constraints: c,
        } = label.label_info_helper(path_to_grost, flavor_type, &inner_ty, INNER_TAG, false)?;

        let ty = if *repeated {
          syn::parse2(quote! {
            #path_to_grost::__private::marker::RepeatedMarker<#ty, #inner, #tag>
          })?
        } else {
          syn::parse2(quote! {
            #path_to_grost::__private::marker::ListMarker<#ty, #inner>
          })?
        };

        LabelInformation::with_wire_format_constraints(
          ty,
          c,
        )
      }
      Self::Nullable(Either::Right(wf)) => LabelInformation::new(syn::parse2(quote! {
        #path_to_grost::__private::marker::WireFormatMarker<#ty, #wf>
      })?),
      Self::Nullable(Either::Left(label)) => {
        let inner_ty: Type = syn::parse2(quote! {
          <#ty as #path_to_grost::__private::convert::State<
            #path_to_grost::__private::convert::Flattened<#path_to_grost::__private::convert::Inner>,
          >>::Output
        })?;

        let LabelInformation {
          wire_format: inner,
          wire_format_constraints: c,
        } = label
          .label()
          .label_info_helper(path_to_grost, flavor_type, &inner_ty, tag, false)?;

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

        LabelInformation::with_wire_format_constraints(
          ty,
          c,
        )
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
          () if ident.eq("generic") => GenericLabelValue::parse(&content).map(Self::Generic)?,
          () if ident.eq("nullable") => {
            NullableLabelParser::parse(&content).and_then(|NullableLabelParser(val)| match val {
              Either::Left(label) => {
                if label.label().is_nullable() {
                  return Err(syn::Error::new(
                    content.span(),
                    "`nullable(nullable(...))` is not supported",
                  ));
                }

                if label.label().is_generic() {
                  return Err(syn::Error::new(
                    content.span(),
                    "`nullable(generic(...))` is not supported",
                  ));
                }

                Ok(Self::Nullable(Either::Left(label)))
              }
              Either::Right(val) => Ok(Self::Nullable(Either::Right(val))),
            })?
          }
          () if ident.eq("set") => {
            ListLikeLabelParser::<SET_TAG>::parse(&content).and_then(|label| {
              if let Either::Left(label) = &label.0 {
                if label.label().is_generic() {
                  return Err(syn::Error::new(
                    content.span(),
                    "`set(generic(...))` is not supported",
                  ));
                }
              }
              Ok(Self::Set(label.0))
            })?
          }
          () if ident.eq("list") => {
            ListLikeLabelParser::<LIST_TAG>::parse(&content).and_then(|label| {
              if let Either::Left(label) = &label.0 {
                if label.label().is_generic() {
                  return Err(syn::Error::new(
                    content.span(),
                    "`list(generic(...))` is not supported",
                  ));
                }
              }
              Ok(Self::List(label.0))
            })?
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
            MapLabelParser::parse(&content).and_then(|label| {
              if let Either::Left(label) = &label.0 {
                if label.key().is_generic() {
                  return Err(syn::Error::new(
                    content.span(),
                    "`key(generic(...))` is not supported",
                  ));
                }

                if label.value().is_generic() {
                  return Err(syn::Error::new(
                    content.span(),
                    "`value(generic(...))` is not supported",
                  ));
                }
              }
              Ok(Self::Map(label.0))
            })?
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
