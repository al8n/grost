use core::num::NonZeroU32;

use heck::ToUpperCamelCase as _;
use quote::{format_ident, quote};
use syn::{
  Ident,
  parse::{Parse, Parser},
};

use crate::ast::{
  grost_flavor_param,
  object::{Field, ObjectExt},
};

use super::Object;

pub struct IndexerVariant {
  tag: NonZeroU32,
  variant: syn::Variant,
  field_name: Ident,
}

impl IndexerVariant {
  pub const fn tag(&self) -> NonZeroU32 {
    self.tag
  }

  pub const fn variant(&self) -> &syn::Variant {
    &self.variant
  }

  pub const fn name(&self) -> &Ident {
    &self.variant.ident
  }

  /// Returns the corresponding field name to the variant.
  pub const fn field_name(&self) -> &Ident {
    &self.field_name
  }
}

pub struct Indexer {
  parent_name: Ident,
  name: Ident,
  vis: syn::Visibility,
  attrs: Vec<syn::Attribute>,
  variants: Vec<IndexerVariant>,
}

impl Indexer {
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  pub const fn vis(&self) -> &syn::Visibility {
    &self.vis
  }

  pub const fn attrs(&self) -> &[syn::Attribute] {
    self.attrs.as_slice()
  }

  pub const fn variants(&self) -> &[IndexerVariant] {
    self.variants.as_slice()
  }

  pub(super) fn from_input<O>(input: &O) -> darling::Result<Self>
  where
    O: crate::ast::object::Object,
  {
    let name = input.indexer_name();
    let attrs = input.meta().indexer().attrs().to_vec();

    let variants = input
      .fields()
      .iter()
      .enumerate()
      .map(|(idx, f)| {
        let variant = format_ident!("{}", f.name().to_string().to_upper_camel_case());
        let variant_doc = format!(" The field indexer for the field `{}`", f.name());
        let idx = idx as u32;

        syn::Variant::parse
          .parse2(quote! {
            #[doc = #variant_doc]
            #variant = #idx
          })
          .map(|variant| IndexerVariant {
            tag: f.meta().tag(),
            variant,
            field_name: f.name().clone(),
          })
      })
      .collect::<Result<Vec<_>, _>>()?;

    Ok(Self {
      parent_name: input.name().clone(),
      name,
      vis: input.vis().clone(),
      attrs,
      variants,
    })
  }

  pub(super) fn to_token_stream(&self) -> proc_macro2::TokenStream {
    let name = self.name();
    let vis = self.vis();

    let variants = self.variants().iter().map(|v| &v.variant);
    let attrs = self.attrs();

    let doc = if !attrs.iter().any(|attr| attr.path().is_ident("doc")) {
      let doc = format!(" Field indexer for the struct [`{}`]", self.parent_name);
      Some(quote! {
        #[doc = #doc]
      })
    } else {
      None
    };

    quote! {
      #(#attrs)*
      #doc
      #[derive(::core::clone::Clone, ::core::marker::Copy, ::core::cmp::PartialEq, ::core::cmp::Eq, ::core::cmp::PartialOrd, ::core::cmp::Ord, ::core::hash::Hash, ::core::fmt::Debug)]
      #[repr(u32)]
      #vis enum #name {
        #(#variants),*
      }
    }
  }
}

impl<M> Object<M>
where
  M: crate::ast::object::Object,
{
  pub(super) fn derive_indexer(&self) -> proc_macro2::TokenStream {
    let name = self.indexer.name();
    let num_fields = self.fields().len();
    let fgp = grost_flavor_param();
    let fg = &fgp.ident;
    let path_to_grost = &self.path_to_grost;

    let first_variant_name = self.indexer.variants().first().unwrap().name();

    let last_variant_name = self.indexer.variants().last().unwrap().name();

    let struct_name = self.name();
    let (_, tg, w) = self.generics().split_for_impl();
    let mut generics = self.generics().clone();
    generics.params.push(syn::GenericParam::Type(fgp.clone()));
    let (ig, _, _) = generics.split_for_impl();

    let prev = self.indexer.variants().windows(2).rev().map(|f| {
      let curr_variant_name = f[1].name();
      let prev_variant_name = f[0].name();

      quote! {
        Self::#curr_variant_name => ::core::option::Option::Some(Self::#prev_variant_name)
      }
    });

    let next = self.indexer.variants().windows(2).map(|f| {
      let curr_variant_name = f[0].name();
      let next_variant_name = f[1].name();

      quote! {
        Self::#curr_variant_name => ::core::option::Option::Some(Self::#next_variant_name)
      }
    });

    let mut reflections_constraints = vec![];
    let reflections = self
      .indexer
      .variants()
      .iter()
      .map(|f| {
        let tag = f.tag().get();
        let field_variant = f.name();

        reflections_constraints.push(quote! {
          #path_to_grost::__private::reflection::ObjectFieldReflection<
            #struct_name #tg,
            #path_to_grost::__private::reflection::ObjectField,
            #fg,
            #tag,
          >: #path_to_grost::__private::reflection::Reflectable<
            #struct_name #tg,
            Reflection = #path_to_grost::__private::reflection::ObjectField,
          >
        });

        quote! {
          Self::#field_variant => {
            <
              #path_to_grost::__private::reflection::ObjectFieldReflection<
                #struct_name #tg,
                #path_to_grost::__private::reflection::ObjectField,
                #fg,
                #tag,
              > as #path_to_grost::__private::reflection::Reflectable<
                #struct_name #tg,
              >
            >::REFLECTION
          }
        }
      })
      .collect::<Vec<_>>();

    quote! {
      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl #ig #path_to_grost::__private::indexer::Indexable<#fg> for #struct_name #tg #w {
        type Indexer = #name;
      }

      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl #name
      {
        /// Returns the field reflection of the corresponding field.
        #[allow(non_camel_case_types, clippy::type_complexity)]
        #[inline]
        pub const fn reflection #ig (
          &self,
        ) -> &'static #path_to_grost::__private::reflection::ObjectField
        where
          #(#reflections_constraints),*
        {
          match self {
            #(#reflections),*
          }
        }
      }

      #[automatically_derived]
      impl #name {
        /// The number of variants of this field indexer.
        pub const VARIANTS: ::core::primitive::usize = #num_fields;
        /// The first field indexer.
        pub const FIRST: Self = Self::#first_variant_name;
        /// The last field indexer.
        pub const LAST: Self = Self::#last_variant_name;

        /// Returns the next field indexer.
        ///
        /// Returns `None` if there are no more fields.
        #[inline]
        pub const fn next(&self) -> ::core::option::Option<Self> {
          match self {
            Self::#last_variant_name => ::core::option::Option::None,
            #(#next),*
          }
        }

        /// Returns the previous field indexer.
        ///
        /// Returns `None` if there are no previous fields.
        #[inline]
        pub const fn prev(&self) -> ::core::option::Option<Self> {
          match self {
            Self::#first_variant_name => ::core::option::Option::None,
            #(#prev),*
          }
        }

        /// Returns the remaining number of fields.
        #[inline]
        pub const fn remaining(&self) -> ::core::primitive::usize {
          Self::LAST as ::core::primitive::usize - *self as ::core::primitive::u32 as ::core::primitive::usize
        }
      }

      #[automatically_derived]
      impl ::core::iter::Iterator for #name {
        type Item = Self;

        fn next(&mut self) -> ::core::option::Option<Self> {
          Self::next(self)
        }

        fn size_hint(&self) -> (::core::primitive::usize, ::core::option::Option<::core::primitive::usize>) {
          let remaining = self.remaining();
          (remaining, ::core::option::Option::Some(remaining))
        }
      }

      #[automatically_derived]
      impl ::core::iter::DoubleEndedIterator for #name {
        fn next_back(&mut self) -> ::core::option::Option<Self> {
          Self::prev(self)
        }
      }

      #[automatically_derived]
      impl ::core::iter::FusedIterator for #name {}
      #[automatically_derived]
      impl ::core::iter::ExactSizeIterator for #name {
        fn len(&self) -> ::core::primitive::usize {
          self.remaining()
        }
      }
    }
  }
}
