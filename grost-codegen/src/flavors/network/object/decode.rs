use super::{Network, Object, quote};

impl Network {
  pub(super) fn derive_object_decode(
    &self,
    object: &Object,
  ) -> syn::Result<proc_macro2::TokenStream> {
    let path_to_grost = object.path();
    let partial_decoded = object.partial_decoded();
    let ubp = partial_decoded.unknown_buffer_param();
    let ubi = &ubp.ident;
    let ltg = partial_decoded.lifetime();
    let ty = partial_decoded.type_with(None, Some(&self.ty), None)?;
    let remove_genericsout_flavor_generic =
      partial_decoded.remove_generics(None, Some(&self.ty), None)?;
    let (ig_without_flavor, _, wc_without_flavor) =
      remove_genericsout_flavor_generic.split_for_impl();

    Ok(quote! {
      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl #ig_without_flavor #path_to_grost::__private::Decode<#ltg, #path_to_grost::__private::flavors::Network, #path_to_grost::__private::flavors::network::LengthDelimited, Self, #ubi> for #ty #wc_without_flavor {
        fn decode<B>(
          ctx: &#path_to_grost::__private::flavors::network::Context,
          buf: B,
        ) -> ::core::result::Result<(::core::primitive::usize, Self), #path_to_grost::__private::flavors::network::Error>
        where
          Self: ::core::marker::Sized,
          B: #path_to_grost::__private::Buf<#ltg>,
          #ubi: #path_to_grost::__private::Buffer<#path_to_grost::__private::flavors::network::Unknown<B>>,
        {
          ::core::todo!()
        }
      }
    })
  }
}
