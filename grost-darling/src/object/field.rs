use darling::{FromDeriveInput, FromMeta, ast::Data, util::Ignored};
use quote::{ToTokens, format_ident, quote};
use syn::{Attribute, Generics, Ident, Path, Visibility};

use super::{Attributes, Field};

const BUILTIN_NAMES: &[&str] = &[
  "schema",
  "default",
  "tag",
  "flavor",
  "convert",
  "partial",
  "partial_decoded",
  "selector",
  "copy",
  "skip",
];

#[derive(Debug, FromMeta)]
struct FieldMeta {
  #[darling(rename = "crate", default = "super::super::default_path")]
  path_to_crate: Path,
  #[darling(default)]
  attribute: Option<Ident>,
  #[darling(default)]
  rename: Option<Ident>,
  #[darling(default, map = "Attributes::into_inner")]
  meta: Vec<Attribute>,
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(grost), forward_attrs, supports(struct_named, struct_unit))]
struct FieldOptionsInput {
  ident: Ident,
  vis: Visibility,
  generics: Generics,
  data: Data<Ignored, Field>,
}

pub struct ObjectField {
  ident: Ident,
  vis: Visibility,
  generics: Generics,
  data: Data<Ignored, Field>,
  path_to_crate: Path,
  attribute: Ident,
  rename: Option<Ident>,
  meta: Vec<Attribute>,
}

impl ObjectField {
  pub fn from_attribute_input(
    args: proc_macro2::TokenStream,
    input: proc_macro2::TokenStream,
  ) -> darling::Result<Self> {
    let input: syn::DeriveInput = syn::parse2(input)?;
    let input = <FieldOptionsInput as FromDeriveInput>::from_derive_input(&input)?;
    let args = darling::ast::NestedMeta::parse_meta_list(args)?;
    let args = <FieldMeta as FromMeta>::from_list(&args)?;

    Ok(Self {
      ident: input.ident,
      vis: input.vis,
      path_to_crate: args.path_to_crate,
      attribute: args.attribute.unwrap_or_else(|| format_ident!("grost")),
      rename: args.rename,
      meta: args.meta,
      generics: input.generics,
      data: input.data,
    })
  }

  fn derive_custom_meta(&self, fields: &darling::ast::Fields<&Field>) -> proc_macro2::TokenStream {
    let meta_name = match self.rename.as_ref() {
      Some(name) => format_ident!("{}Meta", name),
      None => format_ident!("{}Meta", self.ident),
    };

    let generics = &self.generics;
    let (ig, tg, w) = generics.split_for_impl();

    let vis = &self.vis;
    let fields_declare = fields.iter().map(|f| {
      let ty = &f.ty;
      let vis = &f.vis;
      let ident = f
        .ident
        .as_ref()
        .expect("Object should only have named fields");
      quote! {
        #vis #ident: #ty,
      }
    });
    let accessors = fields.iter().map(|f| {
      let name = f.ident.as_ref().unwrap();
      let ty = &f.ty;
      let vis = &f.vis;
      quote! {
        #[inline]
        #vis const fn #name(&self) -> &#ty {
          &self.#name
        }
      }
    });

    quote! {
      #[derive(::core::fmt::Debug, ::core::clone::Clone)]
      #vis struct #meta_name #generics #w {
        #(#fields_declare),*
      }

      impl #ig #meta_name #tg #w {
        #(#accessors)*
      }
    }
  }

  fn name(&self, prefix: &str, suffix: &str) -> Ident {
    if let Some(rename) = &self.rename {
      format_ident!("{}{}{}", prefix, rename, suffix)
    } else {
      format_ident!("{}{}{}", prefix, self.ident, suffix)
    }
  }
}

impl ToTokens for ObjectField {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let input_name = self.name("", "");
    let nested_meta = self.name("__", "NestedMeta__");

    let hidden_input_name = self.name("__", "Input__");
    let field_meta_without_label_name = self.name("__", "FieldMetaWithoutLabel__");
    let field_meta_with_label_name = self.name("__", "FieldMeta__");
    let custom_meta_name = self.name("", "Meta");
    let vis = &self.vis;
    let path_to_crate = &self.path_to_crate;
    let generics = &self.generics;
    let (ig, tg, w) = generics.split_for_impl();

    let attribute = &self.attribute;
    let attribute_str = attribute.to_string();
    let builtin_meta_fields_names = {
      let idents = BUILTIN_NAMES.iter().map(|name| {
        let ident = format_ident!("{name}");
        quote!(#ident,)
      });

      quote! {
        #(#idents)*
      }
    };
    let meta = &self.meta;
    let (
      custom_meta,
      custom_meta_ty,
      custom_meta_getter,
      custom_meta_fields,
      custom_meta_fields_without_darling,
      custom_meta_fields_deconstructor,
      into_outer,
    ) = match self.data.as_ref() {
      Data::Enum(_) => unreachable!("`field` should not be used for enums"),
      Data::Struct(fields) => {
        if fields.is_unit() || fields.is_empty() {
          (quote!(), quote!(()), quote!(&()), None, None, None, None)
        } else {
          (
            self.derive_custom_meta(&fields),
            quote!(#custom_meta_name #tg),
            quote!(&self.__custom_meta__),
            Some({
              let fields = fields.iter().map(|f| {
                let ty = &f.ty;
                let vis = &f.vis;
                let meta = f.darling.as_ref().map(|m| {
                  quote! {
                    #[#m]
                  }
                });
                let name = f
                  .ident
                  .as_ref()
                  .expect("`field` should only work on structs with named fields");
                quote! {
                  #meta
                  #vis #name: #ty,
                }
              });
              quote! {
                #(#fields)*
              }
            }),
            Some({
              quote! {
                #[doc(hidden)]
                __custom_meta__: #custom_meta_name #tg,
              }
            }),
            Some({
              let fields = fields.iter().map(|f| {
                let field_name = f.ident.as_ref().unwrap();
                quote! { #field_name, }
              });

              quote! {
                #(#fields)*
              }
            }),
            Some({
              let fields = fields.iter().map(|f| {
                let field_name = f.ident.as_ref().unwrap();
                quote! { #field_name: input.#field_name, }
              });

              quote! {
                __custom_meta__: #custom_meta_name #tg {
                  #(#fields)*
                },
              }
            }),
          )
        }
      }
    };

    tokens.extend(quote! {
      #custom_meta

      #(#meta)*
      #[derive(::core::fmt::Debug, ::core::clone::Clone)]
      #vis struct #input_name #tg #w {
        ident: ::core::option::Option<#path_to_crate::__private::syn::Ident>,
        vis: #path_to_crate::__private::syn::Visibility,
        ty: #path_to_crate::__private::syn::Type,
        attrs: ::std::vec::Vec<#path_to_crate::__private::syn::Attribute>,

        #custom_meta_fields_without_darling

        __meta__: #path_to_crate::__private::object::FieldOptions,
      }

      const _: () = {
        use #path_to_crate::__private::{darling, syn};

        #[allow(clippy::large_enum_variant)]
        enum #nested_meta {
          Label(#path_to_crate::__private::object::Label),
          Nested(#path_to_crate::__private::darling::ast::NestedMeta),
        }

        impl #path_to_crate::__private::syn::parse::Parse for #nested_meta {
          fn parse(input: #path_to_crate::__private::syn::parse::ParseStream) -> #path_to_crate::__private::syn::Result<Self> {
            if #path_to_crate::__private::object::Label::peek(&input)? {
              let label: #path_to_crate::__private::object::Label = input.parse()?;
              ::core::result::Result::Ok(Self::Label(label))
            } else {
              #path_to_crate::__private::darling::ast::NestedMeta::parse(input).map(Self::Nested)
            }
          }
        }

        #[derive(::core::fmt::Debug, ::core::clone::Clone, #path_to_crate::__private::darling::FromMeta)]
        struct #field_meta_without_label_name #generics #w {
          #[darling(default)]
          schema: #path_to_crate::__private::utils::SchemaFromMeta,
          #[darling(default)]
          default: ::core::option::Option<#path_to_crate::__private::utils::Invokable>,
          #[darling(default)]
          tag: ::core::option::Option<::core::num::NonZeroU32>,
          #[darling(default)]
          flavor: #path_to_crate::__private::object::meta::GenericFieldFlavorFromMeta,
          #[darling(default)]
          convert: #path_to_crate::__private::object::meta::ConvertFromMeta,
          #[darling(default)]
          partial: #path_to_crate::__private::object::meta::PartialFieldFromMeta,
          #[darling(default)]
          partial_decoded: #path_to_crate::__private::object::meta::PartialDecodedFieldFromMeta,
          #[darling(default)]
          selector: #path_to_crate::__private::object::meta::SelectorFieldFromMeta,
          #[darling(default)]
          copy: ::core::primitive::bool,
          #[darling(default)]
          skip: ::core::primitive::bool,
          #custom_meta_fields
        }

        struct #field_meta_with_label_name #generics #w {
          label: ::core::option::Option<#path_to_crate::__private::object::Label>,
          schema: #path_to_crate::__private::utils::SchemaFromMeta,
          default: ::core::option::Option<#path_to_crate::__private::utils::Invokable>,
          tag: ::core::option::Option<::core::num::NonZeroU32>,
          flavor: #path_to_crate::__private::object::meta::GenericFieldFlavorFromMeta,
          convert: #path_to_crate::__private::object::meta::ConvertFromMeta,
          partial: #path_to_crate::__private::object::meta::PartialFieldFromMeta,
          partial_decoded: #path_to_crate::__private::object::meta::PartialDecodedFieldFromMeta,
          selector: #path_to_crate::__private::object::meta::SelectorFieldFromMeta,
          copy: ::core::primitive::bool,
          skip: ::core::primitive::bool,
          #custom_meta_fields_without_darling
        }

        impl #ig #path_to_crate::__private::darling::FromMeta for #field_meta_with_label_name #tg #w {
          fn from_meta(
            item: &#path_to_crate::__private::syn::Meta,
          ) -> #path_to_crate::__private::darling::Result<Self> {
            (match item {
              #path_to_crate::__private::syn::Meta::Path(_) => Self::from_word(),
              #path_to_crate::__private::syn::Meta::NameValue(value) => Self::from_expr(&value.value),
              #path_to_crate::__private::syn::Meta::List(value) => {
                use #path_to_crate::__private::syn::parse::Parser;

                let punctuated =
                  #path_to_crate::__private::syn::punctuated::Punctuated::<#nested_meta, #path_to_crate::__private::syn::Token![,]>::parse_terminated
                    .parse2(value.tokens.clone())?;

                let mut nested_meta = ::std::vec::Vec::new();
                let mut label: ::core::option::Option<#path_to_crate::__private::object::Label> = ::core::option::Option::None;
                for item in punctuated {
                  match item {
                    #nested_meta::Label(l) => {
                      if let ::core::option::Option::Some(ref label) = label {
                        return ::core::result::Result::Err(#path_to_crate::__private::darling::Error::custom(
                          ::std::format!(
                            "Cannot specify both `{label}` and `{l}` at the same time.",
                          )
                        ));
                      }
                      label = ::core::option::Option::Some(l);
                    }
                    #nested_meta::Nested(value) => {
                      nested_meta.push(value);
                    }
                  }
                }

                let #field_meta_without_label_name #tg {
                  #builtin_meta_fields_names
                  #custom_meta_fields_deconstructor
                } = <#field_meta_without_label_name #tg as #path_to_crate::__private::darling::FromMeta>::from_list(&nested_meta)?;
                ::core::result::Result::Ok(Self {
                  label: if skip {
                    ::core::option::Option::None
                  } else {
                    ::core::option::Option::Some(label.ok_or_else(|| #path_to_crate::__private::darling::Error::custom("Expected one of [scalar, bytes, string, object, enum, union, interface, map, set, list, optional] to be specified for a field"))?)
                  },
                  #builtin_meta_fields_names
                  #custom_meta_fields_deconstructor
                })
              }
            })
            .map_err(|e| e.with_span(item))
          }
        }

        #[derive(::core::fmt::Debug, ::core::clone::Clone)]
        struct #hidden_input_name #generics #w {
          ident: ::core::option::Option<#path_to_crate::__private::syn::Ident>,
          vis: #path_to_crate::__private::syn::Visibility,
          ty: #path_to_crate::__private::syn::Type,
          attrs: ::std::vec::Vec<#path_to_crate::__private::syn::Attribute>,

          #custom_meta_fields

          __meta__: #path_to_crate::__private::object::meta::FieldFromMeta,
        }

        impl #ig #path_to_crate::__private::darling::FromField for #hidden_input_name #tg #w {
          fn from_field(
            field: &#path_to_crate::__private::syn::Field,
          ) -> #path_to_crate::__private::darling::Result<Self> {
            use #path_to_crate::__private::quote::ToTokens;

            let mut errors = #path_to_crate::__private::darling::Error::accumulator();
            let mut fwd_attrs: ::std::vec::Vec<
              #path_to_crate::__private::syn::Attribute,
            > = ::std::vec::Vec::new();
            let mut meta: ::core::option::Option<#field_meta_with_label_name #tg> = ::core::option::Option::None;

            for attr in &field.attrs {
              match ::darling::export::ToString::to_string(
                &attr.path().clone().into_token_stream(),
              )
                .as_str()
              {
                #attribute_str => {
                  if meta.is_some() {
                    errors.push(
                      #path_to_crate::__private::darling::Error::custom(
                        ::std::format!(
                          "Cannot specify `{}` attribute multiple times on the same field.",
                          #attribute_str
                        )
                      )
                      .with_span(attr),
                    );
                    continue;
                  }

                  match <#field_meta_with_label_name #tg as #path_to_crate::__private::darling::FromMeta>::from_meta(&attr.meta) {
                    ::core::result::Result::Ok(val) => {
                      meta = ::core::option::Option::Some(val);
                    },
                    ::core::result::Result::Err(e) => {
                      errors.push(e);
                    }
                  }
                }
                _ => fwd_attrs.push(attr.clone()),
              }
            }

            if meta.is_none() {
              errors.push(
                #path_to_crate::__private::darling::Error::missing_field(#attribute_str),
              );
            }

            errors.finish()?;

            let ::core::option::Option::Some(meta) = meta else {
              ::core::panic!("Uninitialized fields without defaults were already checked")
            };

            let #field_meta_with_label_name #tg {
              label,
              #builtin_meta_fields_names
              #custom_meta_fields_deconstructor
            } = meta;

            ::core::result::Result::Ok(Self {
              ident: field.ident.clone(),
              ty: field.ty.clone(),
              vis: field.vis.clone(),
              attrs: fwd_attrs,
              #custom_meta_fields_deconstructor
              __meta__: #path_to_crate::__private::object::meta::FieldFromMeta {
                label,
                #builtin_meta_fields_names
              },
            })
          }
        }

        #path_to_crate::__private::darling::uses_type_params!(#input_name #tg, ty);
        #path_to_crate::__private::darling::uses_lifetimes!(#input_name #tg, ty);

        impl #ig ::core::convert::TryFrom<#hidden_input_name #tg> for #input_name #tg #w {
          type Error = #path_to_crate::__private::syn::Error;

          #[inline]
          fn try_from(input: #hidden_input_name #tg) -> ::core::result::Result<Self, Self::Error> {
            ::core::result::Result::Ok(Self {
              ident: input.ident,
              vis: input.vis,
              ty: input.ty,
              attrs: input.attrs,
              __meta__: input.__meta__.finalize()?,
              #into_outer
            })
          }
        }

        impl #ig #path_to_crate::__private::darling::FromField for #input_name #tg #w {
          fn from_field(
            field: &#path_to_crate::__private::syn::Field,
          ) -> #path_to_crate::__private::darling::Result<Self> {
            <#hidden_input_name #tg as #path_to_crate::__private::darling::FromField>::from_field(field)
              .and_then(|field| ::core::convert::TryInto::try_into(field).map_err(#path_to_crate::__private::darling::Error::from))
          }
        }

        impl #ig #path_to_crate::__private::object::RawField for #input_name #tg #w {
          type Meta = #custom_meta_ty;

          #[inline]
          fn name(&self) -> &#path_to_crate::__private::syn::Ident {
            self.ident.as_ref().expect("the field of the named struct must have a name")
          }

          #[inline]
          fn ty(&self) -> &#path_to_crate::__private::syn::Type {
            &self.ty
          }

          #[inline]
          fn vis(&self) -> &#path_to_crate::__private::syn::Visibility {
            &self.vis
          }

          #[inline]
          fn attrs(&self) -> &[#path_to_crate::__private::syn::Attribute] {
            &self.attrs
          }

          fn tag(&self) -> ::core::option::Option<::core::num::NonZeroU32> {
            self.__meta__.tag()
          }

          fn flavors(&self) -> &[#path_to_crate::__private::object::FieldFlavorAttribute] {
            self.__meta__.flavors()
          }

          fn convert(&self) -> &#path_to_crate::__private::object::ConvertAttribute {
            &self.__meta__.convert()
          }

          fn partial(&self) -> &#path_to_crate::__private::object::PartialFieldOptions {
            self.__meta__.partial()
          }

          fn partial_decoded(&self) -> &#path_to_crate::__private::object::PartialDecodedFieldOptions {
            self.__meta__.partial_decoded()
          }

          fn copy(&self) -> ::core::primitive::bool {
            self.__meta__.copy()
          }

          fn skip(&self) -> ::core::primitive::bool {
            self.__meta__.skip()
          }

          fn selector(&self) -> &#path_to_crate::__private::object::SelectorFieldOptions {
            &self.__meta__.selector()
          }

          fn label(&self) -> ::core::option::Option<&#path_to_crate::__private::object::Label> {
            self.__meta__.label()
          }

          fn schema(&self) -> &#path_to_crate::__private::utils::SchemaAttribute {
            &self.__meta__.schema()
          }

          fn default(&self) -> ::core::option::Option<&#path_to_crate::__private::utils::Invokable> {
            self.__meta__.default()
          }

          fn meta(&self) -> &Self::Meta {
            #custom_meta_getter
          }
        }
      };
    });
  }
}
