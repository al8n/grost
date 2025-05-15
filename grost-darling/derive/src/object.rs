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
pub struct ObjectFieldDeriveInput {
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

impl ToTokens for ObjectFieldDeriveInput {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let name = &self.ident;
    let derive_input_name = self
      .rename
      .clone()
      .unwrap_or_else(|| format_ident!("{}DeriveInput", self.ident));
    let vis = &self.vis;
    let path_to_crate = &self.path_to_crate;
    let (ig, tg, w) = self.generics.split_for_impl();

    let attributes = self.attributes.iter();
    let meta = &self.meta;

    let custom_meta_field = match self.data.as_ref() {
      Data::Enum(_) => unreachable!("ObjectFieldDeriveInput should not be used for enums"),
      Data::Struct(fields) => {
        if fields.is_unit() || fields.is_empty() {
          None
        } else {
          Some(quote! {
            #[darling(flatten)]
            #[doc(hidden)]
            __custom_meta__: #name,
          })
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
            &self.__custom_meta__.#field_name
          }

          #[doc = #doc_mut]
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
      #[derive(::core::fmt::Debug, ::core::clone::Clone, #path_to_crate::__private::darling::FromField)]
      #[darling(attributes(#(#attributes),*), forward_attrs)]
      #vis struct #derive_input_name #tg #w {
        ident: ::core::option::Option<#path_to_crate::__private::syn::Ident>,
        vis: #path_to_crate::__private::syn::Visibility,
        ty: #path_to_crate::__private::syn::Type,
        attrs: ::std::vec::Vec<#path_to_crate::__private::syn::Attribute>,

        #custom_meta_field

        #[darling(flatten)]
        __meta__: #path_to_crate::__private::meta::object::FieldMeta,
      }

      impl #ig #derive_input_name #tg #w {
        #accessors
      }

      impl #ig #path_to_crate::__private::meta::object::Field for #derive_input_name #tg #w {
        fn name(&self) -> &#path_to_crate::__private::syn::Ident {
          self.ident.as_ref().expect("the field of the named struct must have a name")
        }

        fn ty(&self) -> &#path_to_crate::__private::syn::Type {
          &self.ty
        }

        fn vis(&self) -> &#path_to_crate::__private::syn::Visibility {
          &self.vis
        }

        fn attrs(&self) -> &[#path_to_crate::__private::syn::Attribute] {
          &self.attrs
        }

        fn meta(&self) -> &#path_to_crate::__private::meta::object::FieldMeta {
          &self.__meta__
        }
      }
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
  #[darling(default, map = "DarlingAttributes::into_inner")]
  attributes: HashSet<Ident>,
  #[darling(default)]
  rename: Option<Ident>,
  field: Type,
  #[darling(default, map = "Attributes::into_inner")]
  meta: Vec<syn::Attribute>,
  #[darling(default = "super::default_grost_path")]
  path: syn::Path,
}

impl ToTokens for ObjectDeriveInput {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let derive_input_name = self
      .rename
      .clone()
      .unwrap_or_else(|| format_ident!("{}DeriveInput", self.ident));
    let vis = &self.vis;
    let path_to_crate = &self.path_to_crate;
    let (ig, tg, w) = self.generics.split_for_impl();

    let field = &self.field;
    let attributes = self.attributes.iter();
    let meta = &self.meta;
    let expect_msg = format!("{} only supports named structs", derive_input_name);
    let path = &self.path.to_token_stream().to_string();
    let name = &self.ident;

    let meta_field = match self.data.as_ref() {
      Data::Enum(_) => unreachable!("ObjectDeriveInput should not be used for enums"),
      Data::Struct(fields) => {
        if fields.is_unit() || fields.is_empty() {
          None
        } else {
          Some(quote! {
            #[darling(flatten)]
            #[doc(hidden)]
            __custom_meta__: #name,
          })
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
            &self.__custom_meta__.#field_name
          }

          #[doc = #doc_mut]
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
      #[derive(::core::fmt::Debug, ::core::clone::Clone, #path_to_crate::__private::darling::FromDeriveInput)]
      #[darling(attributes(#(#attributes),*), forward_attrs, supports(struct_named))]
      #vis struct #derive_input_name #tg #w {
        ident: #path_to_crate::__private::syn::Ident,
        vis: #path_to_crate::__private::syn::Visibility,
        generics: #path_to_crate::__private::syn::Generics,
        attrs: ::std::vec::Vec<#path_to_crate::__private::syn::Attribute>,
        data: #path_to_crate::__private::darling::ast::Data<#path_to_crate::__private::darling::util::Ignored, #field>,

        #meta_field

        #[darling(rename = "crate", default = #path)]
        #[doc(hidden)]
        __path_to_crate__: #path_to_crate::__private::syn::Path,
        #[darling(flatten)]
        #[doc(hidden)]
        __meta__: #path_to_crate::__private::meta::object::ObjectMeta,
      }

      impl #ig #derive_input_name #tg #w {
        #accessors
      }

      impl #ig #path_to_crate::__private::meta::object::Object for #derive_input_name #tg #w {
        type Field = #field;

        fn path(&self) -> &#path_to_crate::__private::syn::Path {
          &self.__path_to_crate__
        }

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

        fn meta(&self) -> &#path_to_crate::__private::meta::object::ObjectMeta {
          &self.__meta__
        }
      }
    });
  }
}
