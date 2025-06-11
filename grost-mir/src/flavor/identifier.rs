use quote::quote;
use syn::Path;

use crate::utils::Invokable;

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
  pub(crate) constructor: Invokable,
  pub(crate) encode: Invokable,
}

impl IdentifierAttribute {
  pub const fn constructor(&self) -> &Invokable {
    &self.constructor
  }

  pub const fn encode(&self) -> &Invokable {
    &self.encode
  }

  pub(crate) fn network(path_to_grost: &Path) -> Self {
    let constructor =
      syn::parse2::<Path>(quote! { #path_to_grost::__private::flavors::network::Identifier::new })
        .unwrap();
    let encode = syn::parse2::<Path>(
      quote! { #path_to_grost::__private::flavors::network::Identifier::encode },
    )
    .unwrap();

    Self {
      constructor: constructor.into(),
      encode: encode.into(),
    }
  }
}
