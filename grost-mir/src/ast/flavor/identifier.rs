use quote::quote;

use crate::meta::flavor::IdentifierFromMeta;

impl From<IdentifierFromMeta> for IdentifierAttribute {
  fn from(meta: IdentifierFromMeta) -> Self {
    Self {
      constructor: meta.constructor,
      encode: meta.encode,
    }
  }
}

#[derive(Debug, Clone)]
pub struct IdentifierAttribute {
  pub(crate) constructor: syn::Path,
  pub(crate) encode: syn::Path,
}

impl IdentifierAttribute {
  pub(crate) fn network(path_to_grost: &syn::Path) -> Self {
    let constructor =
      syn::parse2(quote! { #path_to_grost::__private::flavors::network::Identifier::new }).unwrap();
    let encode =
      syn::parse2(quote! { #path_to_grost::__private::flavors::network::Identifier::encode })
        .unwrap();

    Self {
      constructor,
      encode,
    }
  }
}
