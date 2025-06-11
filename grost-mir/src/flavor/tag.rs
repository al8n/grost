use quote::quote;
use syn::Path;

use crate::utils::Invokable;

use super::meta::TagFromMeta;

impl From<TagFromMeta> for TagAttribute {
  fn from(meta: TagFromMeta) -> Self {
    Self {
      constructor: meta.constructor,
      encode: meta.encode,
    }
  }
}

#[derive(Debug, Clone)]
pub struct TagAttribute {
  pub(crate) constructor: Invokable,
  pub(crate) encode: Invokable,
}

impl TagAttribute {
  pub const fn constructor(&self) -> &Invokable {
    &self.constructor
  }

  pub const fn encode(&self) -> &Invokable {
    &self.encode
  }

  pub(crate) fn network(path_to_grost: &Path) -> Self {
    let constructor =
      syn::parse2::<Path>(quote! { #path_to_grost::__private::flavors::network::Tag::new })
        .unwrap();
    let encode =
      syn::parse2::<Path>(quote! { #path_to_grost::__private::flavors::network::Tag::encode })
        .unwrap();

    Self {
      constructor: constructor.into(),
      encode: encode.into(),
    }
  }
}
