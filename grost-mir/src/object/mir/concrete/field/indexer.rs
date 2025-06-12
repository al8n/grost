use heck::ToUpperCamelCase;
use quote::{format_ident, quote};
use syn::{
  Variant,
  parse::{Parse, Parser},
};

#[derive(Debug, Clone)]
pub struct FieldIndex {
  variant: Variant,
  index: usize,
}

impl FieldIndex {
  pub(super) fn new<F>(
    index: usize,
    f: &super::ConcreteTaggedFieldAst<F>,
  ) -> darling::Result<Self> {
    let variant = format_ident!("{}", f.name().to_string().to_upper_camel_case());
    let variant_doc = format!(" The field indexer for the field `{}`", f.name());
    let tag = f.tag();

    Ok(Self {
      variant: syn::Variant::parse.parse2(quote! {
        #[doc = #variant_doc]
        #variant = #tag
      })?,
      index,
    })
  }

  /// Returns the variant of the field
  #[inline]
  pub const fn variant(&self) -> &Variant {
    &self.variant
  }

  /// Returns the index of the field
  #[inline]
  pub const fn index(&self) -> usize {
    self.index
  }
}
