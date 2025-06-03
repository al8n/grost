use std::collections::HashSet;

use darling::{FromDeriveInput, FromField, ast::Data, util::Ignored};
use quote::{ToTokens, format_ident, quote};
use syn::{Ident, Type, Visibility};

use super::{Attributes, DarlingAttributes};

#[derive(Debug, FromField)]
#[darling(attributes(grost), forward_attrs)]
struct Field {
  ident: Option<Ident>,
  vis: Visibility,
  ty: syn::Type,
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

    let (custom_meta_field, custom_meta_field_without_darling, into_outer) =
      match self.data.as_ref() {
        Data::Enum(_) => unreachable!("FieldDeriveInput should not be used for enums"),
        Data::Struct(fields) => {
          if fields.is_unit() || fields.is_empty() {
            (None, None, None)
          } else {
            (
              Some(quote! {
                #[darling(flatten)]
                #[doc(hidden)]
                __custom_meta__: #name,
              }),
              Some(quote! {
                #[doc(hidden)]
                __custom_meta__: #name,
              }),
              Some(quote! {
                __custom_meta__: input.__custom_meta__,
              }),
            )
          }
        }
      };

    let fields = self.data.as_ref().take_struct().unwrap();
    let accessors = if fields.is_unit() || fields.is_empty() {
      quote! {}
    } else {
      let iter = fields.iter().map(|f| {
        let field_name = f.ident.as_ref().unwrap();
        let field_ty = &f.ty;
        let field_vis = &f.vis;
        let fn_name = format_ident!("{}_ref", field_name);
        let fn_mut_name = format_ident!("{}_mut", field_name);
        let doc = format!(" Returns a reference to the field `{}`.", field_name);
        let doc_mut = format!(
          " Returns a mutable reference to the field `{}`.",
          field_name
        );
        quote! {
          #[doc = #doc]
          #[inline]
          #field_vis fn #fn_name(&self) -> &#field_ty {
            &self.__custom_meta__.#field_name
          }

          #[doc = #doc_mut]
          #[inline]
          #field_vis fn #fn_mut_name(&mut self) -> &mut #field_ty {
            &mut self.__custom_meta__.#field_name
          }
        }
      });
      quote! {
        #(#iter)*
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

        __meta__: #path_to_crate::__private::ast::object::FieldAttribute,
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
          __meta__: #path_to_crate::__private::ast::object::FieldFromMeta,
        }

        darling::uses_type_params!(#derive_input_name #tg, ty);
        darling::uses_lifetimes!(#derive_input_name #tg, ty);

        impl #ig ::core::convert::TryFrom<#hidden_derive_input_name #tg> for #derive_input_name #tg #w {
          type Error = syn::Error;

          #[inline]
          fn try_from(input: #hidden_derive_input_name #tg) -> ::core::result::Result<Self, Self::Error> {
            Ok(Self {
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

        impl #ig #derive_input_name #tg #w {
          #accessors
        }

        impl #ig #path_to_crate::__private::ast::object::Field for #derive_input_name #tg #w {
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

          fn flavors(&self) -> &[#path_to_crate::__private::ast::object::FieldFlavorAttribute] {
            self.__meta__.flavors()
          }

          fn convert(&self) -> &#path_to_crate::__private::ast::object::ConvertAttribute {
            &self.__meta__.convert()
          }

          fn flavor(&self) -> &[#path_to_crate::__private::ast::object::FieldFlavorAttribute] {
            self.__meta__.flavor()
          }

          fn partial(&self) -> &#path_to_crate::__private::ast::object::PartialFieldAttribute {
            self.__meta__.partial()
          }

          fn partial_decoded(&self) -> &#path_to_crate::__private::ast::object::PartialDecodedFieldAttribute {
            self.__meta__.partial_decoded()
          }

          fn copy(&self) -> ::core::primitive::bool {
            self.__meta__.copy()
          }

          fn skip(&self) -> ::core::primitive::bool {
            self.__meta__.skip()
          }

          fn selector(&self) -> &#path_to_crate::__private::ast::object::SelectorFieldAttribute {
            &self.__meta__.selector()
          }

          fn label(&self) -> ::core::option::Option<&#path_to_crate::__private::ast::object::Label> {
            self.__meta__.label()
          }

          fn schema(&self) -> &#path_to_crate::__private::ast::SchemaAttribute {
            &self.__meta__.schema()
          }

          fn default(&self) -> ::core::option::Option<&syn::Path> {
            self.__meta__.default()
          }
        }
      };
    });
  }
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(grost), forward_attrs, supports(struct_named, struct_unit))]
pub struct ObjectDeriveInput {
  ident: Ident,
  vis: Visibility,
  generics: syn::Generics,
  data: Data<Ignored, Field>,
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

impl ToTokens for ObjectDeriveInput {
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
      .unwrap_or_else(|| format_ident!("{}Input", self.ident));
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

    let (meta_field, meta_field_without_darling, convert) = match self.data.as_ref() {
      Data::Enum(_) => unreachable!("ObjectDeriveInput should not be used for enums"),
      Data::Struct(fields) => {
        if fields.is_unit() || fields.is_empty() {
          (None, None, None)
        } else {
          (
            Some(quote! {
              #[darling(flatten)]
              #[doc(hidden)]
              __custom_meta__: #name,
            }),
            Some(quote! {
              #[doc(hidden)]
              __custom_meta__: #name,
            }),
            Some(quote! {
              __custom_meta__: args.__custom_meta__,
            }),
          )
        }
      }
    };

    let fields = self.data.as_ref().take_struct().unwrap();
    let accessors = if fields.is_unit() || fields.is_empty() {
      quote! {}
    } else {
      let iter = fields.iter().map(|f| {
        let field_name = f.ident.as_ref().unwrap();
        let field_ty = &f.ty;
        let field_vis = &f.vis;
        let fn_name = format_ident!("{}_ref", field_name);
        let fn_mut_name = format_ident!("{}_mut", field_name);
        let doc = format!(" Returns a reference to the field `{}`.", field_name);
        let doc_mut = format!(
          " Returns a mutable reference to the field `{}`.",
          field_name
        );
        quote! {
          #[doc = #doc]
          #field_vis fn #fn_name(&self) -> &#field_ty {
            &self.__args__.__custom_meta__.#field_name
          }

          #[doc = #doc_mut]
          #field_vis fn #fn_mut_name(&mut self) -> &mut #field_ty {
            &mut self.__args__.__custom_meta__.#field_name
          }
        }
      });
      quote! {
        #(#iter)*
      }
    };

    let object_name_ident = format_ident!("ident");
    let object_ty_ident = format_ident!("ty");
    let object_tg_ident = format_ident!("tg");
    let path_to_grost_ident = format_ident!("path_to_grost");

    tokens.extend(quote! {
      #[allow(warnings)]
      #[doc(hidden)]
      #[derive(::core::fmt::Debug, ::core::clone::Clone)]
      struct #meta_name #generics #where_clause {
        #meta_field_without_darling

        #[doc(hidden)]
        __meta__: #path_to_crate::__private::ast::object::ObjectAttribute,
      }

      #(#meta)*
      #[derive(::core::fmt::Debug, ::core::clone::Clone)]
      #vis struct #input_name #generics #where_clause {
        ident: #path_to_crate::__private::syn::Ident,
        vis: #path_to_crate::__private::syn::Visibility,
        ty: #path_to_crate::__private::syn::Type,
        reflectable: #path_to_crate::__private::syn::Type,
        generics: #path_to_crate::__private::syn::Generics,
        attrs: ::std::vec::Vec<#path_to_crate::__private::syn::Attribute>,
        data: #path_to_crate::__private::darling::ast::Data<#path_to_crate::__private::darling::util::Ignored, #field>,
        output: ::core::option::Option<#path_to_crate::__private::ast::Output>,
        derived: ::core::primitive::bool,

        #[doc(hidden)]
        __args__: #meta_name,
      }

      const _: () = {
        use #path_to_crate::__private::{darling, syn, quote::{quote, ToTokens}};

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
          __meta__: #path_to_crate::__private::ast::object::ObjectFromMeta,
        }

        #[allow(warnings)]
        #[doc(hidden)]
        #[derive(::core::fmt::Debug, ::core::clone::Clone, #path_to_crate::__private::darling::FromMeta)]
        struct #darling_attribute_meta_name #generics #where_clause {
          #meta_field

          #[darling(rename = "output", default)]
          __output__: ::core::option::Option<#path_to_crate::__private::ast::Output>,

          #[darling(rename = "crate", default = #path)]
          #[doc(hidden)]
          __path_to_crate__: #path_to_crate::__private::syn::Path,
          #[darling(flatten)]
          #[doc(hidden)]
          __meta__: #path_to_crate::__private::ast::object::ObjectFromMeta,
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
          ) -> #path_to_crate::__private::darling::Result<Self> {
            let input: #path_to_crate::__private::syn::DeriveInput = #path_to_crate::__private::syn::parse2(input)?;
            let input = <#derive_input_name #tg as #path_to_crate::__private::darling::FromDeriveInput>::from_derive_input(&input)?;
            let args = input.__args__;
            let ident = input.ident;
            let (_, tg, _) = input.generics.split_for_impl();
            let ty = syn::parse2(quote::quote! {
              # #object_name_ident # #object_tg_ident
            })?;
            let path_to_grost = &args.__path_to_crate__;
            let reflectable = syn::parse2(quote::quote! {
              # #path_to_grost_ident::__private::reflection::Reflectable<# #object_ty_ident>
            })?;
            ::core::result::Result::Ok(Self {
              ident,
              ty,
              reflectable,
              vis: input.vis,
              generics: input.generics,
              attrs: input.attrs,
              data: input.data,
              output: ::core::option::Option::None,
              derived: true,
              __args__: #meta_name {
                __meta__: args.__meta__.finalize(args.__path_to_crate__)?,
                #convert
              },
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
            let input: #path_to_crate::__private::syn::DeriveInput = #path_to_crate::__private::syn::parse2(input)?;
            let input = <#attribute_input_name #tg as #path_to_crate::__private::darling::FromDeriveInput>::from_derive_input(&input)?;
            let args = #path_to_crate::__private::darling::ast::NestedMeta::parse_meta_list(args)?;
            let args = <#darling_attribute_meta_name #tg as #path_to_crate::__private::darling::FromMeta>::from_list(&args)?;
            let ident = input.ident;
            let (_, tg, _) = input.generics.split_for_impl();
            let ty = syn::parse2(quote::quote! {
              # #object_name_ident # #object_tg_ident
            })?;
            let path_to_grost = &args.__path_to_crate__;
            let reflectable = syn::parse2(quote::quote! {
              # #path_to_grost_ident::__private::reflection::Reflectable<# #object_ty_ident>
            })?;

            ::core::result::Result::Ok(Self {
              ident,
              vis: input.vis,
              ty,
              reflectable,
              generics: input.generics,
              attrs: input.attrs,
              data: input.data,
              output: args.__output__,
              derived: false,
              __args__: #meta_name {
                __meta__: args.__meta__.finalize(args.__path_to_crate__)?,
                #convert
              },
            })
          }

          /// Returns the output configuration of the generated code for the object.
          /// 
          /// - If the instance is created from [`from_derive_input`], this will always be `None`.
          #[inline]
          pub const fn output(&self) -> ::core::option::Option<&#path_to_crate::__private::ast::Output> {
            self.output.as_ref()
          }

          /// Returns `true` if the object is created from derive macro.
          #[inline]
          pub const fn derived(&self) -> ::core::primitive::bool {
            self.derived
          }

          #accessors
        }

        impl #ig #path_to_crate::__private::ast::object::Object for #input_name #tg #w {
          type Field = #field;

          fn name(&self) -> &#path_to_crate::__private::syn::Ident {
            &self.ident
          }

          fn ty(&self) -> &#path_to_crate::__private::syn::Type {
            &self.ty
          }

          fn reflectable(&self) -> &#path_to_crate::__private::syn::Type {
            &self.reflectable
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

          fn schema(&self) -> &#path_to_crate::__private::ast::SchemaAttribute {
            self.__args__.__meta__.schema()
          }

          fn partial(&self) -> &#path_to_crate::__private::ast::object::PartialObjectAttribute {
            self.__args__.__meta__.partial()
          }

          fn partial_decoded(&self) -> &#path_to_crate::__private::ast::object::PartialDecodedObjectAttribute {
            self.__args__.__meta__.partial_decoded()
          }

          fn selector(&self) -> &#path_to_crate::__private::ast::object::SelectorAttribute {
            self.__args__.__meta__.selector()
          }

          fn selector_iter(&self) -> &#path_to_crate::__private::ast::object::SelectorIterAttribute {
            self.__args__.__meta__.selector_iter()
          }

          fn indexer(&self) -> &#path_to_crate::__private::ast::object::IndexerAttribute {
            self.__args__.__meta__.indexer()
          }

          fn copy(&self) -> ::core::primitive::bool {
            self.__args__.__meta__.copy()
          }

          fn flavors(&self,) -> &[#path_to_crate::__private::ast::FlavorAttribute] {
            self.__args__.__meta__.flavors()
          }
        }
      };
    });
  }
}
