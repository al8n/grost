use super::Object;
use grost_darling::grost_lifetime;
use quote::quote;

impl Object {
  pub(super) fn derive_selector_defination(&self) -> proc_macro2::TokenStream {
    let selector = self.object.selector();
    let name = selector.name();
    let vis = self.object.vis();
    let doc = format!(" The selection type for [`{name}`]");

    let fields = selector.fields().iter().map(|f| {
      let ty = f.ty();
      let name = f.name();
      let attrs = f.attrs();
      quote! {
        #(#attrs)*
        #name: #ty,
      }
    });
    let attrs = selector.attrs();
    let generics = selector.generics();
    let where_clause = generics.where_clause.as_ref();
    quote! {
      #(#attrs)*
      #[doc = #doc]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      #[derive(::core::fmt::Debug)]
      #vis struct #name #generics #where_clause
      {
        #(#fields)*
      }
    }
  }

  pub(super) fn derive_selector_iter_defination(&self) -> proc_macro2::TokenStream {
    let iter = self.object.selector_iter();
    let name = self.object.selector().name();
    let iter_name = iter.name();
    let vis = self.object.vis();
    let iter_doc = format!(" An iterator over the selected fields of the [`{name}`]",);
    let indexer_name = self.object.indexer().name();
    let gl = grost_lifetime();
    let generics = iter.generics();
    let where_clause = generics.where_clause.as_ref();
    let attrs = iter.attrs();

    quote! {
      #(#attrs)*
      #[doc = #iter_doc]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      #vis struct #iter_name #generics #where_clause
      {
        selector: &#gl #name<__GROST_FLAVOR__>,
        index: ::core::option::Option<#indexer_name>,
        num: ::core::primitive::usize,
        yielded: ::core::primitive::usize,
      }
    }
  }
}
