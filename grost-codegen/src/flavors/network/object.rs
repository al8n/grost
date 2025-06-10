use super::{Network, Object};

use quote::quote;

// mod decode;
// mod reflection;

impl Network {
  pub(super) fn derive_object(&self, object: &Object) -> syn::Result<proc_macro2::TokenStream> {
    // let reflection = self.derive_object_reflection(object)?;
    // let decode = self.derive_object_decode(object)?;

    Ok(quote! {
      // #reflection

      // #decode
    })
  }
}
