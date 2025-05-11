use quote::{ToTokens, quote};

use crate::{Object, network::Network};

impl Network {
  // pub(crate) fn generate_encode(
  //   &self,
  //   path_to_grost: &syn::Path,
  //   struct_: &Object,
  // ) -> proc_macro2::TokenStream {
  //   let self_encoded_len = quote! {
  //     <Self as #path_to_grost::__private::Encode<#path_to_grost::__private::flavors::Network, #path_to_grost::__private::flavors::network::LengthDelimited>>::encoded_len(
  //       self,
  //       ctx,
  //     )
  //   };
  //   let self_encoded_length_delimited_len = quote! {
  //     <Self as #path_to_grost::__private::Encode<#path_to_grost::__private::flavors::Network, #path_to_grost::__private::flavors::network::LengthDelimited>>::encoded_length_delimited_len(
  //       self,
  //       ctx,
  //     )
  //   };

  //   let encode_impl = self.generate_self_encode_impl(
  //     path_to_grost,
  //     struct_,
  //     &self_encoded_len,
  //     &self_encoded_length_delimited_len,
  //   );

  //   let partial_struct_encode_impl = self.generate_partial_struct_encode_impl(
  //     path_to_grost,
  //     struct_,
  //     &self_encoded_len,
  //     &self_encoded_length_delimited_len,
  //   );

  //   let partial_struct_ref_encode_impl = self.generate_partial_struct_ref_encode_impl(
  //     path_to_grost,
  //     struct_,
  //     &self_encoded_len,
  //     &self_encoded_length_delimited_len,
  //   );

  //   quote! {
  //     #encode_impl

  //     #partial_struct_encode_impl

  //     #partial_struct_ref_encode_impl
  //   }
  // }

  
}
