use std::collections::HashSet;

use darling::{FromDeriveInput, FromField, FromMeta, ast::Data, util::Ignored};
use quote::{ToTokens, format_ident, quote};
use syn::{Ident, Type, Visibility};

use super::{Attributes, DarlingAttributes};

#[derive(Debug, FromField)]
#[darling(attributes(grost), forward_attrs)]
struct Field {
  ident: Option<Ident>,
  vis: Visibility,
  ty: syn::Type,
  #[darling(default, map = "super::map_option_meta")]
  darling: Option<syn::Meta>,
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(grost), forward_attrs, supports(struct_named, struct_unit))]
pub struct FieldDeriveInput {
  ident: Ident,
  vis: Visibility,
  generics: syn::Generics,
  data: Data<Ignored, Field>,
  #[darling(rename = "crate", default = "super::default_path")]
  path_to_crate: syn::Path,
  #[darling(default, map = "DarlingAttributes::into_inner")]
  attributes: HashSet<Ident>,
  #[darling(default)]
  rename: Option<Ident>,
  #[darling(default, map = "Attributes::into_inner")]
  meta: Vec<syn::Attribute>,
}

impl ToTokens for FieldDeriveInput {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let name = &self.ident;
    let derive_input_name = self
      .rename
      .clone()
      .unwrap_or_else(|| format_ident!("{}DeriveInput", self.ident));
    let hidden_derive_input_name = self
      .rename
      .clone()
      .unwrap_or_else(|| format_ident!("__{}DeriveInput__", self.ident));
    let vis = &self.vis;
    let path_to_crate = &self.path_to_crate;
    let (ig, tg, w) = self.generics.split_for_impl();

    let attributes = self.attributes.iter();
    let meta = &self.meta;

    let (
      custom_meta_ty,
      custom_meta_getter,
      custom_meta_field,
      custom_meta_field_without_darling,
      into_outer,
    ) = match self.data.as_ref() {
      Data::Enum(_) => unreachable!("FieldDeriveInput should not be used for enums"),
      Data::Struct(fields) => {
        if fields.is_unit() || fields.is_empty() {
          (quote!(()), quote!(&()), None, None, None)
        } else {
          (
            quote!(#name #tg),
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
                  .expect("FieldDeriveInput should only work on structs with named fields");
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
                __custom_meta__: #name #tg,
              }
            }),
            Some({
              let fields = fields.iter().map(|f| {
                let field_name = f.ident.as_ref().unwrap();
                quote! { #field_name: input.#field_name, }
              });

              quote! {
                __custom_meta__: #name #tg {
                  #(#fields)*
                },
              }
            }),
          )
        }
      }
    };

    tokens.extend(quote! {
      #(#meta)*
      #[derive(::core::fmt::Debug, ::core::clone::Clone)]
      #vis struct #derive_input_name #tg #w {
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
        struct #hidden_derive_input_name #tg #w {
          ident: ::core::option::Option<#path_to_crate::__private::syn::Ident>,
          vis: #path_to_crate::__private::syn::Visibility,
          ty: #path_to_crate::__private::syn::Type,
          attrs: ::std::vec::Vec<#path_to_crate::__private::syn::Attribute>,

          #custom_meta_field

          #[darling(flatten)]
          __meta__: #path_to_crate::__private::object::FieldFromMeta,
        }

        darling::uses_type_params!(#derive_input_name #tg, ty);
        darling::uses_lifetimes!(#derive_input_name #tg, ty);

        impl #ig ::core::convert::TryFrom<#hidden_derive_input_name #tg> for #derive_input_name #tg #w {
          type Error = syn::Error;

          #[inline]
          fn try_from(input: #hidden_derive_input_name #tg) -> ::core::result::Result<Self, Self::Error> {
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

        impl #ig darling::FromField for #derive_input_name #tg #w {
          fn from_field(
            field: &#path_to_crate::__private::syn::Field,
          ) -> darling::Result<Self> {
            <#hidden_derive_input_name #tg as darling::FromField>::from_field(field)
              .and_then(|field| ::core::convert::TryInto::try_into(field).map_err(darling::Error::from))
          }
        }

        impl #ig #path_to_crate::__private::object::RawField for #derive_input_name #tg #w {
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

#[derive(Debug, FromMeta)]
pub struct ObjectMeta {
  #[darling(rename = "crate", default = "super::default_path")]
  path_to_crate: syn::Path,
  #[darling(default)]
  attribute: Option<Ident>,
  #[darling(default)]
  rename: Option<Ident>,
  field: Type,
  #[darling(default, map = "Attributes::into_inner")]
  meta: Vec<syn::Attribute>,
  #[darling(default = "super::default_grost_path")]
  grost: syn::Path,
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(grost), forward_attrs, supports(struct_named, struct_unit))]
pub struct ObjectAttributeInput {
  ident: Ident,
  vis: Visibility,
  generics: syn::Generics,
  data: Data<Ignored, Field>,
}

// #[derive(Debug, FromDeriveInput)]
// #[darling(attributes(grost), forward_attrs, supports(struct_named, struct_unit))]
// pub struct ObjectDeriveInput {
//   ident: Ident,
//   vis: Visibility,
//   generics: syn::Generics,
//   data: Data<Ignored, Field>,
//   #[darling(flatten)]
//   meta: ObjectMeta,
// }

// impl core::ops::Deref for ObjectDeriveInput {
//   type Target = ObjectMeta;

//   fn deref(&self) -> &Self::Target {
//     &self.meta
//   }
// }

pub(super) struct Object {
  ident: Ident,
  vis: Visibility,
  generics: syn::Generics,
  data: Data<Ignored, Field>,
  path_to_crate: syn::Path,
  attribute: Option<Ident>,
  rename: Option<Ident>,
  field: Type,
  meta: Vec<syn::Attribute>,
  grost: syn::Path,
}

impl Object {
  pub fn from_attribute_input(
    args: proc_macro2::TokenStream,
    input: proc_macro2::TokenStream,
  ) -> darling::Result<Self> {
    let input: syn::DeriveInput = syn::parse2(input)?;
    let input = <ObjectAttributeInput as FromDeriveInput>::from_derive_input(&input)?;
    let args = darling::ast::NestedMeta::parse_meta_list(args)?;
    let args = <ObjectMeta as FromMeta>::from_list(&args)?;

    Ok(Self {
      ident: input.ident,
      vis: input.vis,
      generics: input.generics,
      data: input.data,
      path_to_crate: args.path_to_crate,
      attribute: args.attribute,
      rename: args.rename,
      field: args.field,
      meta: args.meta,
      grost: args.grost,
    })
  }

  fn derive_custom_meta(&self, fields: &darling::ast::Fields<&Field>) -> proc_macro2::TokenStream {
    let meta_name = self
      .rename
      .clone()
      .unwrap_or_else(|| format_ident!("{}Meta", self.ident));

    let generics = &self.generics;
    let (_, _, w) = generics.split_for_impl();

    let vis = &self.vis;
    let fields = fields.iter().map(|f| {
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

    quote! {
      #[derive(::core::fmt::Debug, ::core::clone::Clone)]
      #vis struct #meta_name #generics #w {
        #(#fields),*
      }
    }
  }
}

impl ToTokens for Object {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let derive_input_name = self
      .rename
      .clone()
      .unwrap_or_else(|| format_ident!("__{}DeriveInput__", self.ident));
    let attribute_input_name = self
      .rename
      .clone()
      .unwrap_or_else(|| format_ident!("__{}AttributeInput__", self.ident));
    let darling_derive_meta_name = self
      .rename
      .clone()
      .unwrap_or_else(|| format_ident!("__{}DarlingDeriveMeta__", self.ident));
    let darling_attribute_meta_name = self
      .rename
      .clone()
      .unwrap_or_else(|| format_ident!("__{}DarlingAttributeMeta__", self.ident));
    let meta_name = self
      .rename
      .clone()
      .unwrap_or_else(|| format_ident!("__{}Meta__", self.ident));
    let input_name = self
      .rename
      .clone()
      .unwrap_or_else(|| format_ident!("{}DeriveInput", self.ident));
    let custom_meta_name = self
      .rename
      .clone()
      .unwrap_or_else(|| format_ident!("{}Meta", self.ident));

    let vis = &self.vis;
    let path_to_crate = &self.path_to_crate;
    let generics = &self.generics;
    let where_clause = generics.where_clause.as_ref();
    let (ig, tg, w) = self.generics.split_for_impl();

    let field = &self.field;
    let attribute = self.attribute.as_ref();
    let meta = &self.meta;
    let expect_msg = format!("{} only supports named structs", derive_input_name);
    let path = &self.grost.to_token_stream().to_string();
    let name = &self.ident;

    let turbofish = if !generics.params.is_empty() {
      quote!(::#tg::)
    } else {
      quote!(::)
    };

    let (custom_meta, meta_ty, meta_getter, meta_field, meta_field_without_darling, convert) =
      match self.data.as_ref() {
        Data::Enum(_) => unreachable!("`object` should not be used for enums"),
        Data::Struct(fields) => {
          if fields.is_unit() || fields.is_empty() {
            (quote!(), quote!(()), quote!(&()), None, None, None)
          } else {
            (
              self.derive_custom_meta(&fields),
              quote!(#custom_meta_name #tg),
              quote!(&self.__args__.__custom_meta__),
              Some({
                let fields = fields.iter().map(|f| {
                  let ty = &f.ty;
                  let vis = &f.vis;
                  let meta = f.darling.as_ref().map(|m| {
                    quote! {
                      #[darling(#m)]
                    }
                  });
                  match &f.ident {
                    Some(name) => quote! {
                      #meta
                      #vis #name: #ty,
                    },
                    None => {
                      quote! {
                        #meta #vis #ty,
                      }
                    }
                  }
                });
                quote! {
                  #(#fields)*
                }
              }),
              Some(quote! {
                #[doc(hidden)]
                __custom_meta__: #custom_meta_name #tg,
              }),
              Some({
                let fields = fields.iter().map(|f| {
                  let field_name = f.ident.as_ref().unwrap();
                  quote! { #field_name: args.#field_name, }
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

      #vis struct #name #generics #where_clause {
        object: #path_to_crate::__private::object::Object<
          #custom_meta_name #tg,
          <#field as #path_to_crate::__private::object::RawField>::Meta,
        >,
        derived: ::core::primitive::bool,
      }

      const _: () = {
        use #path_to_crate::__private::{darling, syn, quote::{quote, ToTokens}};

        #(#meta)*
        #[derive(::core::fmt::Debug, ::core::clone::Clone)]
        struct #input_name #generics #where_clause {
          ident: #path_to_crate::__private::syn::Ident,
          vis: #path_to_crate::__private::syn::Visibility,
          generics: #path_to_crate::__private::syn::Generics,
          attrs: ::std::vec::Vec<#path_to_crate::__private::syn::Attribute>,
          data: #path_to_crate::__private::darling::ast::Data<#path_to_crate::__private::darling::util::Ignored, #field>,

          #[doc(hidden)]
          __args__: #meta_name,
        }

        #[allow(warnings)]
        #[doc(hidden)]
        #[derive(::core::fmt::Debug, ::core::clone::Clone)]
        struct #meta_name #generics #where_clause {
          #meta_field_without_darling

          #[doc(hidden)]
          __meta__: #path_to_crate::__private::object::ObjectAttribute,
        }

        #[allow(warnings)]
        #[doc(hidden)]
        #[derive(::core::fmt::Debug, ::core::clone::Clone, #path_to_crate::__private::darling::FromMeta)]
        struct #darling_derive_meta_name #generics #where_clause {
          #meta_field

          #[darling(rename = "crate", default = #path)]
          #[doc(hidden)]
          __path_to_crate__: #path_to_crate::__private::syn::Path,
          #[darling(flatten)]
          #[doc(hidden)]
          __meta__: #path_to_crate::__private::object::ObjectFromMeta,
        }

        #[allow(warnings)]
        #[doc(hidden)]
        #[derive(::core::fmt::Debug, ::core::clone::Clone, #path_to_crate::__private::darling::FromMeta)]
        struct #darling_attribute_meta_name #generics #where_clause {
          #meta_field

          #[darling(rename = "crate", default = #path)]
          #[doc(hidden)]
          __path_to_crate__: #path_to_crate::__private::syn::Path,
          #[darling(flatten)]
          #[doc(hidden)]
          __meta__: #path_to_crate::__private::object::ObjectFromMeta,
        }

        #[allow(warnings)]
        #[doc(hidden)]
        #[derive(::core::fmt::Debug, ::core::clone::Clone, #path_to_crate::__private::darling::FromDeriveInput)]
        #[darling(attributes(#attribute), forward_attrs, supports(struct_named))]
        struct #derive_input_name #generics #where_clause {
          ident: #path_to_crate::__private::syn::Ident,
          vis: #path_to_crate::__private::syn::Visibility,
          generics: #path_to_crate::__private::syn::Generics,
          attrs: ::std::vec::Vec<#path_to_crate::__private::syn::Attribute>,
          data: #path_to_crate::__private::darling::ast::Data<#path_to_crate::__private::darling::util::Ignored, #field>,

          #[darling(flatten)]
          #[doc(hidden)]
          __args__: #darling_derive_meta_name,
        }

        #[allow(warnings)]
        #[doc(hidden)]
        #[derive(::core::fmt::Debug, ::core::clone::Clone, #path_to_crate::__private::darling::FromDeriveInput)]
        #[darling(forward_attrs, supports(struct_named))]
        struct #attribute_input_name #generics #where_clause {
          ident: #path_to_crate::__private::syn::Ident,
          vis: #path_to_crate::__private::syn::Visibility,
          generics: #path_to_crate::__private::syn::Generics,
          attrs: ::std::vec::Vec<#path_to_crate::__private::syn::Attribute>,
          data: #path_to_crate::__private::darling::ast::Data<#path_to_crate::__private::darling::util::Ignored, #field>,
        }

        impl #ig #input_name #tg #w {
          /// Parse the input from the derive macro.
          /// 
          /// **Note:** This function is only used for the derive macro input, and it will not
          /// work correctly if you use it for the attribute macro. For the attribute macro,
          /// use [`from_attribute_input`](Self::from_attribute_input) instead.
          pub fn from_derive_input(
            input: #path_to_crate::__private::proc_macro2::TokenStream,
          ) -> #path_to_crate::__private::darling::Result<#path_to_crate::__private::object::Object<
            #meta_ty,
            <<Self as #path_to_crate::__private::object::RawObject>::Field as #path_to_crate::__private::object::RawField>::Meta,
          >> {
            let input: #path_to_crate::__private::syn::DeriveInput = #path_to_crate::__private::syn::parse2(input)?;
            let input = <#derive_input_name #tg as #path_to_crate::__private::darling::FromDeriveInput>::from_derive_input(&input)?;
            let args = input.__args__;

            let this = Self {
              ident: input.ident,
              vis: input.vis,
              generics: input.generics,
              attrs: input.attrs,
              data: input.data,
              __args__: #meta_name {
                __meta__: args.__meta__.finalize(args.__path_to_crate__)?,
                #convert
              },
            };

            #path_to_crate::__private::object::Object::from_raw(this)
          }

          /// Parse the input from the attribute macro input.
          /// 
          /// **Note:** This function is only used for the attribute macro input, and it will not
          /// work correctly if you use it for the derive macro. For the derive macro,
          /// use [`from_derive_input`](Self::from_derive_input) instead.
          pub fn from_attribute_input(
            args: #path_to_crate::__private::proc_macro2::TokenStream,
            input: #path_to_crate::__private::proc_macro2::TokenStream,
          ) -> #path_to_crate::__private::darling::Result<#path_to_crate::__private::object::Object<
            #meta_ty,
            <<Self as #path_to_crate::__private::object::RawObject>::Field as #path_to_crate::__private::object::RawField>::Meta,
          >>
          {
            let input: #path_to_crate::__private::syn::DeriveInput = #path_to_crate::__private::syn::parse2(input)?;
            let input = <#attribute_input_name #tg as #path_to_crate::__private::darling::FromDeriveInput>::from_derive_input(&input)?;
            let args = #path_to_crate::__private::darling::ast::NestedMeta::parse_meta_list(args)?;
            let args = <#darling_attribute_meta_name #tg as #path_to_crate::__private::darling::FromMeta>::from_list(&args)?;

            let this = Self {
              ident: input.ident,
              vis: input.vis,
              generics: input.generics,
              attrs: input.attrs,
              data: input.data,
              __args__: #meta_name {
                __meta__: args.__meta__.finalize(args.__path_to_crate__)?,
                #convert
              },
            };

            #path_to_crate::__private::object::Object::from_raw(this)
          }
        }

        impl #ig #path_to_crate::__private::object::RawObject for #input_name #tg #w {
          type Field = #field;
          type Meta = #meta_ty;

          fn name(&self) -> &#path_to_crate::__private::syn::Ident {
            &self.ident
          }

          fn vis(&self) -> &#path_to_crate::__private::syn::Visibility {
            &self.vis
          }

          fn generics(&self) -> &#path_to_crate::__private::syn::Generics {
            &self.generics
          }

          fn attrs(&self) -> &[#path_to_crate::__private::syn::Attribute] {
            &self.attrs
          }

          fn fields(&self) -> ::std::vec::Vec<&Self::Field> {
            self.data.as_ref().take_struct().expect(#expect_msg).fields
          }

          fn path_to_grost(&self) -> &syn::Path {
            self.__args__.__meta__.path_to_grost()
          }

          fn default(&self) -> ::core::option::Option<&syn::Path> {
            self.__args__.__meta__.default()
          }

          fn schema(&self) -> &#path_to_crate::__private::utils::SchemaAttribute {
            self.__args__.__meta__.schema()
          }

          fn partial(&self) -> &#path_to_crate::__private::object::PartialObjectAttribute {
            self.__args__.__meta__.partial()
          }

          fn partial_decoded(&self) -> &#path_to_crate::__private::object::PartialDecodedObjectAttribute {
            self.__args__.__meta__.partial_decoded()
          }

          fn selector(&self) -> &#path_to_crate::__private::object::SelectorAttribute {
            self.__args__.__meta__.selector()
          }

          fn selector_iter(&self) -> &#path_to_crate::__private::object::SelectorIterAttribute {
            self.__args__.__meta__.selector_iter()
          }

          fn indexer(&self) -> &#path_to_crate::__private::object::IndexerAttribute {
            self.__args__.__meta__.indexer()
          }

          fn copy(&self) -> ::core::primitive::bool {
            self.__args__.__meta__.copy()
          }

          fn flavors(&self,) -> &[#path_to_crate::__private::flavor::FlavorAttribute] {
            self.__args__.__meta__.flavors()
          }

          fn meta(&self) -> &Self::Meta {
            #meta_getter
          }
        }
      
        impl #ig #name #tg #w {
          /// Parse the input from the derive macro.
          /// 
          /// **Note:** This function is only used for the derive macro input, and it will not
          /// work correctly if you use it for the attribute macro. For the attribute macro,
          /// use [`from_attribute_input`](Self::from_attribute_input) instead.
          pub fn from_derive_input(
            input: #path_to_crate::__private::proc_macro2::TokenStream,
          ) -> #path_to_crate::__private::darling::Result<Self> {
            #input_name #turbofish from_derive_input(input).map(|object| Self {
              object,
              derived: true,
            })
          }
  
          /// Parse the input from the attribute macro input.
          /// 
          /// **Note:** This function is only used for the attribute macro input, and it will not
          /// work correctly if you use it for the derive macro. For the derive macro,
          /// use [`from_derive_input`](Self::from_derive_input) instead.
          pub fn from_attribute_input(
            args: #path_to_crate::__private::proc_macro2::TokenStream,
            input: #path_to_crate::__private::proc_macro2::TokenStream,
          ) -> #path_to_crate::__private::darling::Result<Self> {
            #input_name #turbofish from_attribute_input(args, input).map(|object| Self {
              object,
              derived: false,
            })
          }

          /// Returns the MIR representation of the object.
          #[inline]
          pub const fn mir(&self) -> &#path_to_crate::__private::object::Object<
            #custom_meta_name #tg,
            <<#input_name #tg as #path_to_crate::__private::object::RawObject>::Field as #path_to_crate::__private::object::RawField>::Meta,
          > {
            &self.object
          }
        }
      };
    });
  }
}
