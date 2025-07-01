use quote::{format_ident, quote};

impl<T, S, M> super::Object<T, S, M> {
  pub(super) fn derive_encode(&self) -> darling::Result<proc_macro2::TokenStream> {
    let object_impl = derive_object_encode(self)?;
    let partial_ref_object_impl = derive_partial_ref_object_encode(self)?;
    let partial_object_encode_impl = derive_partial_object_encode(self)?;

    Ok(quote! {
      #object_impl
      #partial_object_encode_impl
      #partial_ref_object_impl
    })
  }
}

fn derive_partial_object_encode<T, S, M>(
  object: &super::Object<T, S, M>,
) -> darling::Result<proc_macro2::TokenStream> {
  Ok(quote! {})
}

fn derive_object_encode<T, S, M>(
  object: &super::Object<T, S, M>,
) -> darling::Result<proc_macro2::TokenStream> {
  Ok(quote! {})
}

fn derive_partial_ref_object_encode<T, S, M>(
  object: &super::Object<T, S, M>,
) -> darling::Result<proc_macro2::TokenStream> {
  Ok(quote! {})
}
