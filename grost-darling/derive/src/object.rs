use darling::{ast::Data, util::Ignored, FromDeriveInput, FromField};
use quote::{format_ident, quote, ToTokens};
use syn::{Ident, Type, Visibility};

use super::Attributes;

#[derive(Debug, FromField)]
#[darling(attributes(grost), forward_attrs)]
struct Field {
  ident: Option<Ident>,
  vis: Visibility,
  ty: syn::Type,

  #[darling(default, flatten)]
  attributes: Attributes,
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(grost), forward_attrs, supports(struct_named))]
pub struct ObjectFieldDeriveInput {
  ident: Ident,
  vis: Visibility,
  generics: syn::Generics,
  data: Data<Ignored, Field>,
  #[darling(rename = "crate", default = "super::default_path")]
  path_to_crate: syn::Path,
  #[darling(multiple)]
  attributes: Vec<Ident>,
  #[darling(default)]
  rename: Option<Ident>,
  #[darling(default, map = "Attributes::into_inner")]
  meta: Vec<syn::Attribute>,
}

impl ToTokens for ObjectFieldDeriveInput {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let name = self.rename.clone().unwrap_or_else(|| format_ident!("{}DeriveInput", self.ident));
    let vis = &self.vis;
    let path_to_crate = &self.path_to_crate;
    let (ig, tg, w) = self.generics.split_for_impl();

    let attributes = &self.attributes;
    let meta = &self.meta;

    let fields = match self.data.as_ref() {
      Data::Enum(_) => unreachable!("ObjectFieldDeriveInput should not be used for enums"),
      Data::Struct(fields) => {
        fields.fields.as_slice().iter().map(|f| {
          let name = f.ident.as_ref().expect("the field of the named struct must have a name");
          let ty = &f.ty;
          let vis = &f.vis;
          let attrs = &f.attributes.0;
          quote! {
            #(#attrs)*
            #vis #name: #ty,
          }
        }).collect::<Vec<_>>()
      },
    };

    tokens.extend(quote! {
      #(#meta)*
      #[derive(::core::fmt::Debug, #path_to_crate::__private::darling::FromField)]
      #[darling(attributes(#(#attributes),*), forward_attrs)]
      #vis struct #name #tg #w {
        ident: ::core::option::Option<#path_to_crate::__private::syn::Ident>,
        vis: #path_to_crate::__private::syn::Visibility,
        ty: #path_to_crate::__private::syn::Type,
        attrs: ::std::vec::Vec<#path_to_crate::__private::syn::Attribute>,
        #[darling(flatten)]
        meta: #path_to_crate::__private::object::FieldMeta,

        #(#fields)*
      }

      impl #ig #path_to_crate::__private::object::Field for #name #tg #w {
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

        fn meta(&self) -> &#path_to_crate::__private::object::FieldMeta {
          &self.meta
        }
      }
    }); 
  }
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(grost), forward_attrs, supports(struct_named))]
pub struct ObjectDeriveInput {
  ident: Ident,
  vis: Visibility,
  generics: syn::Generics,
  data: Data<Ignored, Field>,
  #[darling(rename = "crate", default = "super::default_path")]
  path_to_crate: syn::Path,
  #[darling(multiple)]
  attributes: Vec<Ident>,
  #[darling(default)]
  rename: Option<Ident>,
  field: Type,
  #[darling(default, map = "Attributes::into")]
  meta: Vec<syn::Attribute>,
}

impl ToTokens for ObjectDeriveInput {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let name = self.rename.clone().unwrap_or_else(|| format_ident!("{}DeriveInput", self.ident));
    let vis = &self.vis;
    let path_to_crate = &self.path_to_crate;
    let (ig, tg, w) = self.generics.split_for_impl();

    let field = &self.field;
    let attributes = &self.attributes;
    let meta = &self.meta;
    let expect_msg = format!("{} only supports named structs", name);

    let fields = match self.data.as_ref() {
      Data::Enum(_) => unreachable!("ObjectDeriveInput should not be used for enums"),
      Data::Struct(fields) => {
        fields.fields.as_slice().iter().map(|f| {
          let name = f.ident.as_ref().expect("the field of the named struct must have a name");
          let ty = &f.ty;
          let vis = &f.vis;
          let attrs = &f.attributes.0;
          quote! {
            #(#attrs)*
            #vis #name: #ty,
          }
        }).collect::<Vec<_>>()
      },
    };

    tokens.extend(quote! {
      #(#meta)*
      #[derive(::core::fmt::Debug, #path_to_crate::__private::darling::FromDeriveInput)]
      #[darling(attributes(#(#attributes),*), forward_attrs)]
      #vis struct #name #tg #w {
        ident: #path_to_crate::__private::syn::Ident,
        vis: #path_to_crate::__private::syn::Visibility,
        generics: #path_to_crate::__private::syn::Generics,
        attrs: ::std::vec::Vec<#path_to_crate::__private::syn::Attribute>, 
        data: #path_to_crate::__private::darling::Data<#path_to_crate::__private::darling::Ignored, #field>,

        #[darling(flatten)]
        meta: #path_to_crate::__private::object::ObjectMeta,

        #(#fields)*
      }

      impl #ig #path_to_crate::__private::object::Object for #name #tg #w {
        type Field = #field;

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

        fn meta(&self) -> &#path_to_crate::__private::object::ObjectMeta {
          &self.meta
        }
      }
    }); 
  }
}