use heck::ToUpperCamelCase as _;
use quote::{ToTokens, format_ident, quote};
use syn::{
  Ident,
  parse::{Parse, Parser},
};

use crate::meta::object::{Field, ObjectExt};

pub struct Indexer {
  parent_name: Ident,
  name: Ident,
  vis: syn::Visibility,
  attrs: Vec<syn::Attribute>,
  variants: Vec<syn::Variant>,
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

  pub const fn variants(&self) -> &[syn::Variant] {
    self.variants.as_slice()
  }

  pub(super) fn from_input<O>(input: &O) -> darling::Result<Self>
  where
    O: crate::meta::object::Object,
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
        syn::Variant::parse.parse2(quote! {
          #[doc = #variant_doc]
          #variant = #idx
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
}

impl ToTokens for Indexer {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let name = self.name();
    let vis = self.vis();

    let variants = self.variants();
    let attrs = self.attrs();

    let doc = if !attrs.iter().any(|attr| attr.path().is_ident("doc")) {
      let doc = format!(" Field indexer for the struct [`{}`]", self.parent_name);
      Some(quote! {
        #[doc = #doc]
      })
    } else {
      None
    };

    tokens.extend(quote! {
      #(#attrs)*
      #doc
      #[derive(::core::clone::Clone, ::core::marker::Copy, ::core::cmp::PartialEq, ::core::cmp::Eq, ::core::cmp::PartialOrd, ::core::cmp::Ord, ::core::hash::Hash, ::core::fmt::Debug)]
      #[repr(u32)]
      #vis enum #name {
        #(#variants),*
      }
    });
  }
}
