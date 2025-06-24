use darling::{FromDeriveInput, FromField, FromMeta, ast::Data, util::Ignored};
use quote::{format_ident, quote};
use syn::{Attribute, Generics, Ident, Path, Type, Visibility};

use super::Attributes;

#[derive(Debug, Default, Clone, FromMeta)]
struct DarlingFromMeta {
  #[darling(default, map = "super::super::map_option_meta")]
  darling: Option<syn::Meta>,
}

#[derive(Debug, FromField)]
#[darling(attributes(grost), forward_attrs)]
struct Field {
  ident: Option<Ident>,
  vis: Visibility,
  ty: Type,
  tag: DarlingFromMeta,
  skip: DarlingFromMeta,
}

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

  fn name(&self, prefix: &str, suffix: &str) -> Ident {
    if let Some(rename) = &self.rename {
      format_ident!("{}{}{}", prefix, rename, suffix)
    } else {
      format_ident!("{}{}{}", prefix, self.ident, suffix)
    }
  }

  fn derive_hidden_meta(
    path_to_crate: &Path,
    vis: &Visibility,
    name: &Ident,
    hidden_name: &Ident,
    fields: &[&Field],
  ) -> proc_macro2::TokenStream {
    if fields.is_empty() {
      return quote! {};
    }

    let mut fields_def = vec![];
    let mut fields_into = vec![];
    fields.iter().for_each(|f| {
      let ty = &f.ty;
      let vis = &f.vis;
      let ident = f
        .ident
        .as_ref()
        .expect("Already checked field must have a name");

      if let Some(meta) = f.tag.darling.as_ref() {
        fields_def.push(quote! {
          #[#meta]
          #vis #ident: #ty
        });
        fields_into.push(quote! {
          #ident: input.#ident
        });
      } else {
        fields_def.push(quote! {
          #vis #ident: #ty
        });
        fields_into.push(quote! {
          #ident: input.#ident
        });
      }
    });

    quote! {
      #[derive(::core::fmt::Debug, #path_to_crate::__private::darling::FromMeta)]
      #vis struct #hidden_name {
        #(#fields_def),*
      }

      impl #path_to_crate::__private::darling::FromMeta for #name {
        fn from_meta(
          meta: &#path_to_crate::__private::syn::Meta,
        ) -> #path_to_crate::__private::darling::Result<Self> {
          <#hidden_name as #path_to_crate::__private::darling::FromMeta>::from_meta(meta)
            .map(|input| Self {
              #(#fields_into),*
            })
        }
      }
    }
  }

  fn derive_meta_def(
    vis: &Visibility,
    name: &Ident,
    fields: &[&Field],
  ) -> proc_macro2::TokenStream {
    if fields.is_empty() {
      return quote! {};
    }

    let fields = fields.iter().map(|f| {
      let ty = &f.ty;
      let vis = &f.vis;
      let ident = f
        .ident
        .as_ref()
        .expect("Already checked field must have a name");

      quote! {
        #vis #ident: #ty
      }
    });

    quote! {
      #[derive(::core::fmt::Debug)]
      #vis struct #name {
        #(#fields),*
      }
    }
  }

  pub fn derive(&self) -> darling::Result<proc_macro2::TokenStream> {
    let vis = &self.vis;
    let path_to_crate = &self.path_to_crate;

    let generics = &self.generics;
    if !generics.params.is_empty() || generics.where_clause.is_some() {
      return Err(darling::Error::custom(
        "`field` attribute macro does not support generic parameters",
      ));
    }

    let attr = &self.attribute;
    let meta = &self.meta;
    let input_name = self.name("", "");
    let hidden_input_name = self.name("__", "Input__");
    let hidden_skipped_meta_name = self.name("__", "SkippedMeta__");
    let hidden_tagged_meta_name = self.name("__", "TaggedMeta__");
    let skipped_meta_name = self.name("", "SkippedMeta");
    let tagged_meta_name = self.name("", "TaggedMeta");

    let mut custom_skipped_fields = vec![];
    let mut custom_tagged_fields = vec![];

    match self.data.as_ref() {
      Data::Enum(_) => unreachable!("`field` should not be used for enums"),
      Data::Struct(fields) => {
        if !(fields.is_unit() || fields.is_empty()) {
          if !fields.is_struct() {
            return Err(darling::Error::custom(
              "`field` attribute macro can only be used with struct-like data",
            ));
          }

          for f in fields.into_iter() {
            if f.ident.is_none() {
              return Err(darling::Error::custom(
                "tuple-like structs are not supported by `field` attribute macro",
              ));
            }

            if f.tag.darling.is_some() {
              custom_tagged_fields.push(f);
            }

            if f.skip.darling.is_some() {
              custom_skipped_fields.push(f);
            }
          }
        }
      }
    };

    let skipped_meta = Self::derive_meta_def(vis, &skipped_meta_name, &custom_skipped_fields);

    let tagged_meta = Self::derive_meta_def(vis, &tagged_meta_name, &custom_tagged_fields);

    let hidden_skipped_meta = Self::derive_hidden_meta(
      path_to_crate,
      vis,
      &skipped_meta_name,
      &hidden_skipped_meta_name,
      &custom_skipped_fields,
    );
    let hidden_tagged_meta = Self::derive_hidden_meta(
      path_to_crate,
      vis,
      &tagged_meta_name,
      &hidden_tagged_meta_name,
      &custom_tagged_fields,
    );

    let tagged_fields_meta_ty = if custom_tagged_fields.is_empty() {
      quote!(())
    } else {
      quote!(#tagged_meta_name)
    };

    let skipped_fields_meta_ty = if custom_skipped_fields.is_empty() {
      quote!(())
    } else {
      quote!(#skipped_meta_name)
    };

    Ok(quote! {
      #skipped_meta

      #tagged_meta

      #(#meta)*
      #[derive(::core::fmt::Debug, ::core::clone::Clone)]
      #vis struct #input_name {
        ident: ::core::option::Option<#path_to_crate::__private::syn::Ident>,
        vis: #path_to_crate::__private::syn::Visibility,
        ty: #path_to_crate::__private::syn::Type,
        attrs: ::std::vec::Vec<#path_to_crate::__private::syn::Attribute>,

        meta: #path_to_crate::__private::object::meta::FieldFromMeta<#tagged_fields_meta_ty, #skipped_fields_meta_ty>,
      }

      const _: () = {
        use #path_to_crate::__private::darling;

        #hidden_skipped_meta
        #hidden_tagged_meta

        #[derive(::core::fmt::Debug, ::core::clone::Clone, #path_to_crate::__private::darling::FromField)]
        #[darling(attributes(#attr), forward_attrs)]
        struct #hidden_input_name {
          ident: ::core::option::Option<#path_to_crate::__private::syn::Ident>,
          vis: #path_to_crate::__private::syn::Visibility,
          ty: #path_to_crate::__private::syn::Type,
          attrs: ::std::vec::Vec<#path_to_crate::__private::syn::Attribute>,

          #[darling(flatten)]
          meta: #path_to_crate::__private::object::meta::FieldFromMeta<#tagged_fields_meta_ty, #skipped_fields_meta_ty>,
        }

        impl #path_to_crate::__private::darling::FromField for #input_name {
          fn from_field(
            field: &#path_to_crate::__private::syn::Field,
          ) -> #path_to_crate::__private::darling::Result<Self> {
            <#hidden_input_name as #path_to_crate::__private::darling::FromField>::from_field(field)
              .and_then(|input| {
                let ident = input.ident;
                let vis = input.vis;
                let ty = input.ty;
                let attrs = input.attrs;

                ::core::result::Result::Ok(Self {
                  ident,
                  vis,
                  ty,
                  attrs,
                  meta: input.meta,
                })
              })
          }
        }

        impl ::core::convert::TryFrom<#input_name> for #path_to_crate::__private::object::RawField<#tagged_fields_meta_ty, #skipped_fields_meta_ty> {
          type Error = #path_to_crate::__private::darling::Error;

          fn try_from(input: #input_name) -> ::core::result::Result<Self, Self::Error> {
            #path_to_crate::__private::object::RawField::new(
              input.attrs,
              input.vis,
              input.ident.expect("Already checked field must have a name"),
              input.ty,
              input.meta,
            )
          }
        }

        impl #path_to_crate::__private::object::meta::RawFieldMeta for #input_name {
          type Skipped = #skipped_fields_meta_ty;
          type Tagged = #tagged_fields_meta_ty;
        }
      };
    })
  }
}
