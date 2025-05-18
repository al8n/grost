use super::{Network, Object};

use quote::quote;

mod reflection;

impl Network {
  pub(super) fn derive_object(&self, object: &Object) -> syn::Result<proc_macro2::TokenStream> {
    let reflection = self.derive_reflection(object)?;

    Ok(quote! {
      #reflection
    })
  }
}
