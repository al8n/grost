use super::Object;

use quote::quote;

impl Object {
  pub(super) fn derive_indexer_defination(&self) -> proc_macro2::TokenStream {
    let indexer = self.object.indexer();
    let name = indexer.name();
    let vis = self.object.vis();
    let indexer_doc = format!(" Field indexer for the struct [`{}`]", self.object.name());

    let variants = indexer.variants();
    let attrs = indexer.attrs();

    quote! {
      #(#attrs)*
      #[doc = #indexer_doc]
      #[derive(::core::clone::Clone, ::core::marker::Copy, ::core::cmp::PartialEq, ::core::cmp::Eq, ::core::cmp::PartialOrd, ::core::cmp::Ord, ::core::hash::Hash, ::core::fmt::Debug)]
      #[repr(u32)]
      #vis enum #name {
        #(#variants),*
      }
    }
  }
}
