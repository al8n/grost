use quote::quote;
use syn::Path;

use super::meta::IdentifierFromMeta;

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
  pub(crate) constructor: Path,
  pub(crate) encode: Path,
}

impl IdentifierAttribute {
  pub const fn constructor(&self) -> &Path {
    &self.constructor
  }

  pub const fn encode(&self) -> &Path {
    &self.encode
  }

  pub(crate) fn network(path_to_grost: &Path) -> Self {
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
