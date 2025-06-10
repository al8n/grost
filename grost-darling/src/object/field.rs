use std::collections::HashSet;

use darling::{ast::Data, util::Ignored, FromDeriveInput, FromMeta};
use quote::{format_ident, quote, ToTokens};
use syn::{Attribute, Generics, Ident, Path, Visibility};

use super::{Field, Attributes, DarlingAttributes};

#[derive(Debug, FromMeta)]
struct FieldMeta {
  #[darling(rename = "crate", default = "super::super::default_path")]
  path_to_crate: Path,
  #[darling(default, map = "DarlingAttributes::into_inner")]
  attributes: HashSet<Ident>,
  #[darling(default)]
  rename: Option<Ident>,
  #[darling(default, map = "Attributes::into_inner")]
  meta: Vec<Attribute>,
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(grost), forward_attrs, supports(struct_named, struct_unit))]
struct FieldAttributeInput {
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
  attributes: HashSet<Ident>,
  rename: Option<Ident>,
  meta: Vec<Attribute>,
}

impl ObjectField {
  pub fn from_attribute_input(
    args: proc_macro2::TokenStream,
    input: proc_macro2::TokenStream,
  ) -> darling::Result<Self> {
    let input: syn::DeriveInput = syn::parse2(input)?;
    let input = <FieldAttributeInput as FromDeriveInput>::from_derive_input(&input)?;
    let args = darling::ast::NestedMeta::parse_meta_list(args)?;
    let args = <FieldMeta as FromMeta>::from_list(&args)?;

    Ok(Self {
      ident: input.ident,
      vis: input.vis,
      path_to_crate: args.path_to_crate,
      attributes: args.attributes,
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
    let (ig,tg, w) = generics.split_for_impl();

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
    let hidden_input_name = self.name("__", "Input__");
    let custom_meta_name = self.name("", "Meta");
    let vis = &self.vis;
    let path_to_crate = &self.path_to_crate;
    let generics = &self.generics;
    let (ig, tg, w) = generics.split_for_impl();

    let attributes = self.attributes.iter();
    let meta = &self.meta;
    let (
      custom_meta,
      custom_meta_ty,
      custom_meta_getter,
      custom_meta_field,
      custom_meta_field_without_darling,
      into_outer,
    ) = match self.data.as_ref() {
      Data::Enum(_) => unreachable!("FieldDeriveInput should not be used for enums"),
      Data::Struct(fields) => {
        if fields.is_unit() || fields.is_empty() {
          (quote!(), quote!(()), quote!(&()), None, None, None)
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
                    #[darling(#m)]
                  }
                });
                let name = f
                  .ident
                  .as_ref()
                  .expect("Field should only work on structs with named fields");
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

        #custom_meta_field_without_darling

        __meta__: #path_to_crate::__private::object::FieldAttribute,
      }

      const _: () = {
        use #path_to_crate::__private::{darling, syn};

        #[derive(::core::fmt::Debug, ::core::clone::Clone, darling::FromField)]
        #[darling(attributes(#(#attributes),*), forward_attrs)]
        struct #hidden_input_name #tg #w {
          ident: ::core::option::Option<#path_to_crate::__private::syn::Ident>,
          vis: #path_to_crate::__private::syn::Visibility,
          ty: #path_to_crate::__private::syn::Type,
          attrs: ::std::vec::Vec<#path_to_crate::__private::syn::Attribute>,

          #custom_meta_field

          #[darling(flatten)]
          __meta__: #path_to_crate::__private::object::FieldFromMeta,
        }

        darling::uses_type_params!(#input_name #tg, ty);
        darling::uses_lifetimes!(#input_name #tg, ty);

        impl #ig ::core::convert::TryFrom<#hidden_input_name #tg> for #input_name #tg #w {
          type Error = syn::Error;

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

        impl #ig darling::FromField for #input_name #tg #w {
          fn from_field(
            field: &#path_to_crate::__private::syn::Field,
          ) -> darling::Result<Self> {
            <#hidden_input_name #tg as darling::FromField>::from_field(field)
              .and_then(|field| ::core::convert::TryInto::try_into(field).map_err(darling::Error::from))
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

          fn partial(&self) -> &#path_to_crate::__private::object::PartialFieldAttribute {
            self.__meta__.partial()
          }

          fn partial_decoded(&self) -> &#path_to_crate::__private::object::PartialDecodedFieldAttribute {
            self.__meta__.partial_decoded()
          }

          fn copy(&self) -> ::core::primitive::bool {
            self.__meta__.copy()
          }

          fn skip(&self) -> ::core::primitive::bool {
            self.__meta__.skip()
          }

          fn selector(&self) -> &#path_to_crate::__private::object::SelectorFieldAttribute {
            &self.__meta__.selector()
          }

          fn label(&self) -> &#path_to_crate::__private::object::Label {
            self.__meta__.label()
          }

          fn schema(&self) -> &#path_to_crate::__private::utils::SchemaAttribute {
            &self.__meta__.schema()
          }

          fn default(&self) -> ::core::option::Option<&syn::Path> {
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
