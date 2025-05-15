use heck::ToUpperCamelCase as _;
use quote::{format_ident, quote};
use syn::{
  Ident,
  parse::{Parse, Parser},
};

use crate::meta::object::{Field, ObjectExt};

pub struct Indexer {
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
      name,
      vis: input.vis().clone(),
      attrs,
      variants,
    })
  }
}
