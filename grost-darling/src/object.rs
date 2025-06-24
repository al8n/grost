use darling::{FromDeriveInput, FromField, FromMeta, ast::Data, util::Ignored};
use quote::{ToTokens, format_ident, quote};
use syn::{Attribute, Generics, Ident, Path, Type, Visibility};

use super::Attributes;

pub(super) use field::ObjectField;

mod field;

#[derive(Debug, FromField)]
#[darling(attributes(grost), forward_attrs)]
struct Field {
  ident: Option<Ident>,
  vis: Visibility,
  ty: Type,
  #[darling(default, map = "super::map_option_meta")]
  darling: Option<syn::Meta>,
}

#[derive(Debug, FromMeta)]
struct ObjectMeta {
  #[darling(rename = "crate", default = "super::default_path")]
  path_to_crate: Path,
  #[darling(default)]
  attribute: Option<Ident>,
  #[darling(default)]
  rename: Option<Ident>,
  field: Type,
  #[darling(default, map = "Attributes::into_inner")]
  meta: Vec<Attribute>,
  #[darling(default = "super::default_grost_path")]
  grost: Path,
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(grost), forward_attrs, supports(struct_named, struct_unit))]
struct ObjectAttributeInput {
  ident: Ident,
  vis: Visibility,
  generics: Generics,
  data: Data<Ignored, Field>,
  attrs: Vec<Attribute>,
}

pub(super) struct Object {
  ident: Ident,
  vis: Visibility,
  attrs: Vec<Attribute>,
  generics: Generics,
  data: Data<Ignored, Field>,
  path_to_crate: Path,
  attribute: Option<Ident>,
  rename: Option<Ident>,
  field: Type,
  meta: Vec<Attribute>,
  grost: Path,
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
      attrs: input.attrs,
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

  fn derive_custom_meta(
    &self,
    name: &Ident,
    fields: &darling::ast::Fields<&Field>,
  ) -> proc_macro2::TokenStream {
    let generics = &self.generics;
    let (ig, tg, w) = generics.split_for_impl();

    let vis = &self.vis;
    let fields_declare = fields.iter().map(|f| {
      let ty = &f.ty;
      let vis = &f.vis;
      let ident = f
        .ident
        .as_ref()
        .expect("`object` should only have named fields");
      quote! {
        #vis #ident: #ty,
      }
    });

    let accessors = fields.iter().map(|f| {
      let ident = f.ident.as_ref().unwrap();
      let vis = &f.vis;
      let ty = &f.ty;
      let doc = format!("Returns the `{}` field of the meta.", ident);
      quote! {
        #[doc = #doc]
        #[inline]
        #vis const fn #ident(&self) -> &#ty {
          &self.#ident
        }
      }
    });

    quote! {
      #[derive(::core::fmt::Debug, ::core::clone::Clone)]
      #vis struct #name #generics #w {
        #(#fields_declare),*
      }

      impl #ig #name #tg #w {
        #(#accessors)*
      }
    }
  }

  fn derive_hidden_custom_meta(
    &self,
    name: &Ident,
    hidden_name: &Ident,
    fields: &darling::ast::Fields<&Field>,
  ) -> proc_macro2::TokenStream {
    let path_to_crate = &self.path_to_crate;

    let generics = &self.generics;
    let (ig, tg, w) = generics.split_for_impl();

    let vis = &self.vis;
    let fields_declare = fields.iter().map(|f| {
      let ty = &f.ty;
      let vis = &f.vis;
      let ident = f
        .ident
        .as_ref()
        .expect("`object` should only have named fields");
      if let Some(meta) = f.darling.as_ref() {
        quote! {
          #[#meta]
          #vis #ident: #ty,
        }
      } else {
        quote! {
          #vis #ident: #ty,
        }
      }
    });

    let fields_into = fields
      .iter()
      .map(|f| {
        let ident = f.ident.as_ref().unwrap();
        quote! {
          #ident: input.#ident
        }
      })
      .collect::<Vec<_>>();

    quote! {
      #[derive(::core::fmt::Debug, #path_to_crate::__private::darling::FromMeta)]
      #vis struct #hidden_name #generics #w {
        #(#fields_declare),*
      }

      impl #ig #path_to_crate::__private::darling::FromMeta for #name #tg #w {
        fn from_meta(
          item: &#path_to_crate::__private::syn::Meta,
        ) -> #path_to_crate::__private::darling::Result<Self> {
          (match *item {
            #path_to_crate::__private::syn::Meta::Path(_) => Self::from_word(),
            #path_to_crate::__private::syn::Meta::List(ref value) => {
              Self::from_list(&#path_to_crate::__private::utils::NestedMeta::parse_meta_list(value.tokens.clone())?[..])
            }
            #path_to_crate::__private::syn::Meta::NameValue(ref value) => Self::from_expr(&value.value),
          })
          .map_err(|e| e.with_span(item))
        }

        fn from_expr(
          expr: &#path_to_crate::__private::syn::Expr,
        ) -> #path_to_crate::__private::darling::Result<Self> {
          <#hidden_name as #path_to_crate::__private::darling::FromMeta>::from_expr(expr)
            .map(|input| Self {
              #(#fields_into),*
            })
        }

        fn from_word() -> #path_to_crate::__private::darling::Result<Self> {
          <#hidden_name as #path_to_crate::__private::darling::FromMeta>::from_word()
            .map(|input| Self {
              #(#fields_into),*
            })
        }

        fn from_list(
          items: &[#path_to_crate::__private::darling::ast::NestedMeta],
        ) -> #path_to_crate::__private::darling::Result<Self> {
          <#hidden_name as #path_to_crate::__private::darling::FromMeta>::from_list(items)
            .map(|input| Self {
              #(#fields_into),*
            })
        }
      }
    }
  }

  fn name(&self, prefix: &str, suffix: &str) -> Ident {
    match self.rename.as_ref() {
      Some(rename) => format_ident!("{prefix}{rename}{suffix}",),
      None => format_ident!("{prefix}{}{suffix}", self.ident),
    }
  }
}

impl ToTokens for Object {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let derive_input_name = self.name("__", "DeriveInput__");
    let attribute_input_name = self.name("__", "AttributeInput__");
    let darling_object_from_meta_name = self.name("__", "DarlingObjectFromMeta__");
    let input_name = self.name("__", "Input__");
    let hidden_meta_name = self.name("__", "HiddenMeta__");
    let meta_name = self.name("", "Meta");

    let vis = &self.vis;
    let path_to_crate = &self.path_to_crate;
    let generics = &self.generics;
    let where_clause = generics.where_clause.as_ref();
    let (ig, tg, w) = self.generics.split_for_impl();

    let field = &self.field;
    let attribute = self.attribute.as_ref();
    let meta = &self.meta;
    let path = &self.grost.to_token_stream().to_string();
    let name = &self.ident;
    let attrs = &self.attrs;

    let input_type_with_turbofish = if !generics.params.is_empty() {
      quote!(#input_name::#tg::)
    } else {
      quote!(#input_name::)
    };

    let (custom_meta, hidden_meta, meta_ty) = match self.data.as_ref() {
      Data::Enum(_) => unreachable!("`object` should not be used for enums"),
      Data::Struct(fields) => {
        if fields.is_unit() || fields.is_empty() {
          (
            quote!(),
            quote!(),
            quote!(#path_to_crate::__private::utils::NoopFromMeta),
          )
        } else {
          (
            self.derive_custom_meta(&meta_name, &fields),
            self.derive_hidden_custom_meta(&meta_name, &hidden_meta_name, &fields),
            quote!(#meta_name #tg),
          )
        }
      }
    };

    let token_stream_ident = format_ident!("token_stream");
    let obj_ident = format_ident!("obj");

    tokens.extend(quote! {
      #custom_meta

      #(#attrs)*
      #vis struct #name #generics #where_clause {
        object: #path_to_crate::__private::object::Object<
          <#field as #path_to_crate::__private::object::meta::RawFieldMeta>::Tagged,
          <#field as #path_to_crate::__private::object::meta::RawFieldMeta>::Skipped,
          #meta_ty,
        >,
        derived: ::core::primitive::bool,
      }

      const _: () = {
        use #path_to_crate::__private::darling;

        #hidden_meta

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
        #[derive(::core::fmt::Debug, ::core::clone::Clone, #path_to_crate::__private::darling::FromMeta)]
        struct #darling_object_from_meta_name #generics #where_clause {
          #[darling(rename = "crate", default = #path)]
          #[doc(hidden)]
          __path_to_crate__: #path_to_crate::__private::syn::Path,
          #[darling(flatten)]
          #[doc(hidden)]
          __meta__: #path_to_crate::__private::object::meta::ObjectFromMeta<#meta_ty>,
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
          __args__: #darling_object_from_meta_name #tg,
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
            <#field as #path_to_crate::__private::object::meta::RawFieldMeta>::Tagged,
            <#field as #path_to_crate::__private::object::meta::RawFieldMeta>::Skipped,
            #meta_ty,
          >> {
            let input: #path_to_crate::__private::syn::DeriveInput = #path_to_crate::__private::syn::parse2(input)?;
            let input = <#derive_input_name #tg as #path_to_crate::__private::darling::FromDeriveInput>::from_derive_input(&input)?;
            let args = input.__args__;

            #path_to_crate::__private::object::Object::new(
              args.__path_to_crate__,
              input.attrs,
              input.vis,
              input.ident,
              input.generics,
              input.data
                .take_struct()
                .expect("Already checked that the input is a structure")
                .into_iter()
                .map(::core::convert::TryInto::try_into)
                .collect::<::core::result::Result<_, _>>()?,
              args.__meta__,
            )
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
            <#field as #path_to_crate::__private::object::meta::RawFieldMeta>::Tagged,
            <#field as #path_to_crate::__private::object::meta::RawFieldMeta>::Skipped,
            #meta_ty,
          >>
          {
            let input: #path_to_crate::__private::syn::DeriveInput = #path_to_crate::__private::syn::parse2(input)?;
            let input = <#attribute_input_name #tg as #path_to_crate::__private::darling::FromDeriveInput>::from_derive_input(&input)?;
            let args = #path_to_crate::__private::darling::ast::NestedMeta::parse_meta_list(args)?;
            let args = <#darling_object_from_meta_name #tg as #path_to_crate::__private::darling::FromMeta>::from_list(&args)?;

            #path_to_crate::__private::object::Object::new(
              args.__path_to_crate__,
              input.attrs,
              input.vis,
              input.ident,
              input.generics,
              input.data
                .take_struct()
                .expect("Already checked that the input is a structure")
                .into_iter()
                .map(::core::convert::TryInto::try_into)
                .collect::<::core::result::Result<_, _>>()?,
              args.__meta__,
            )
          }
        }

        impl #ig ::core::cmp::PartialEq for #name #tg #w {
          fn eq(&self, other: &Self) -> ::core::primitive::bool {
            self.object.name().eq(other.object.name())
          }
        }

        impl #ig ::core::cmp::Eq for #name #tg #w {}

        impl #ig ::core::hash::Hash for #name #tg #w {
          fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
            self.object.name().hash(state);
          }
        }

        impl #ig #path_to_crate::__private::quote::ToTokens for #name #tg #w {
          fn to_tokens(&self, tokens: &mut #path_to_crate::__private::proc_macro2::TokenStream) {
            self.object.to_tokens(tokens);
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
            #input_type_with_turbofish from_derive_input(input).map(|object| Self {
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
            #input_type_with_turbofish from_attribute_input(args, input).map(|object| Self {
              object,
              derived: false,
            })
          }

          /// Returns the MIR representation of the object.
          #[inline]
          pub const fn mir(&self) -> &#path_to_crate::__private::object::Object<
            <#field as #path_to_crate::__private::object::meta::RawFieldMeta>::Tagged,
            <#field as #path_to_crate::__private::object::meta::RawFieldMeta>::Skipped,
            #meta_ty,
          > {
            &self.object
          }

          /// Derives the generated code of the object.
          pub fn derive(&self) -> #path_to_crate::__private::darling::Result<#path_to_crate::__private::proc_macro2::TokenStream> {
            self.object.derive().map(|token_stream| {
              if self.derived {
                token_stream
              } else {
                let obj = &self.object;
                #path_to_crate::__private::quote::quote! {
                  # #obj_ident

                  # #token_stream_ident
                }
              }
            })
          }
        }
      };
    });
  }
}
