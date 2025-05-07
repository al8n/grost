use quote::quote;

use crate::{Struct, network::Network};

impl Network {
  pub(crate) fn generate_partial_encode(
    &self,
    path_to_grost: &syn::Path,
    struct_: &Struct,
  ) -> proc_macro2::TokenStream {
    let struct_name = struct_.name();
    let encoded_len = struct_.fields().iter().map(|f| {
      let field_name = f.name();
      quote! {
        len += (<#struct_name>::reflection::<#path_to_grost::__private::flavors::Network>().#field_name().partial_encoded_len())(
          &self.#field_name,
          ctx,
          &selector.#field_name,
        );
      }
    });

    let self_encoded_len = quote! {
      <Self as #path_to_grost::__private::PartialEncode<#path_to_grost::__private::flavors::network::Network, #path_to_grost::__private::flavors::network::LengthDelimited>>::partial_encoded_len(
        self,
        ctx,
        selector,
      )
    };
    let self_encoded_length_delimited_len = quote! {
      <Self as #path_to_grost::__private::PartialEncode<#path_to_grost::__private::flavors::network::Network, #path_to_grost::__private::flavors::network::LengthDelimited>>::partial_encoded_length_delimited_len(
        self,
        ctx,
        selector,
      )
    };

    let encode = struct_.fields().iter().map(|f| {
      let field_name = f.name();
      quote! {
        if offset >= buf_len {
          return ::core::result::Result::Err(
            #path_to_grost::__private::flavors::network::EncodeError::insufficient_buffer(
              #self_encoded_len,
              buf_len,
            ),
          );
        }
        offset += (<#struct_name>::reflection::<#path_to_grost::__private::flavors::Network>().#field_name().partial_encode())(
          &self.#field_name,
          ctx,
          &mut buf[offset..],
          &selector.#field_name,
        ).map_err(|e| {
          e.update(#self_encoded_len, buf_len)
        })?;
      }
    });

    quote! {
      #[automatically_derived]
      impl #path_to_grost::__private::PartialEncode<#path_to_grost::__private::flavors::network::Network, #path_to_grost::__private::flavors::network::LengthDelimited> for #struct_name {
        fn partial_encode(
          &self,
          ctx: &<#path_to_grost::__private::flavors::network::Network as #path_to_grost::__private::flavors::Flavor>::Context,
          buf: &mut [::core::primitive::u8],
          selector: &<#struct_name as #path_to_grost::__private::Selectable<#path_to_grost::__private::flavors::network::Network>>::Selector,
        ) -> ::core::result::Result<::core::primitive::usize, <#path_to_grost::__private::flavors::network::Network as #path_to_grost::__private::flavors::Flavor>::EncodeError> {
          let mut offset = 0;
          let buf_len = buf.len();

          #(#encode)*

          ::core::result::Result::Ok(offset)
        }

        fn partial_encoded_len(
          &self,
          ctx: &<#path_to_grost::__private::flavors::network::Network as #path_to_grost::__private::flavors::Flavor>::Context,
          selector: &<#struct_name as #path_to_grost::__private::Selectable<#path_to_grost::__private::flavors::network::Network>>::Selector,
        ) -> ::core::primitive::usize {
          let mut len = 0;

          #(#encoded_len)*

          len
        }

        fn partial_encoded_length_delimited_len(
          &self,
          ctx: &<#path_to_grost::__private::flavors::network::Network as #path_to_grost::__private::flavors::Flavor>::Context,
          selector: &<#struct_name as #path_to_grost::__private::Selectable<#path_to_grost::__private::flavors::network::Network>>::Selector,
        ) -> ::core::primitive::usize {
          let encoded_len = #self_encoded_len;

          #path_to_grost::__private::varing::encoded_u32_varint_len(encoded_len as ::core::primitive::u32) + encoded_len
        }

        fn partial_encode_length_delimited(
          &self,
          ctx: &<#path_to_grost::__private::flavors::network::Network as #path_to_grost::__private::flavors::Flavor>::Context,
          buf: &mut [::core::primitive::u8],
          selector: &<#struct_name as #path_to_grost::__private::Selectable<#path_to_grost::__private::flavors::network::Network>>::Selector,
        ) -> ::core::result::Result<::core::primitive::usize, <#path_to_grost::__private::flavors::network::Network as #path_to_grost::__private::flavors::Flavor>::EncodeError> {
          let encoded_len = #self_encoded_len;
          let buf_len = buf.len();
          let offset = #path_to_grost::__private::varing::encode_u32_varint_to(encoded_len as ::core::primitive::u32, buf)
            .map_err(|e| {
              #path_to_grost::__private::flavors::network::EncodeError::from_varint_error(e)
                .update(#self_encoded_length_delimited_len, buf_len)
            })?;

          if offset >= buf_len {
            return ::core::result::Result::Err(
              #path_to_grost::__private::flavors::network::EncodeError::insufficient_buffer(#self_encoded_length_delimited_len, buf_len)
            );
          }

          <Self as #path_to_grost::__private::PartialEncode<
            #path_to_grost::__private::flavors::network::Network,
            #path_to_grost::__private::flavors::network::LengthDelimited,
          >>::partial_encode(self, ctx, &mut buf[offset..], selector)
            .map(|write| {
              #[cfg(debug_assertions)]
              {
                #path_to_grost::__private::debug_assert_write_eq::<Self>(write, encoded_len);
              }

              write + offset
            })
            .map_err(|e| e.update(#self_encoded_length_delimited_len, buf_len))
        }
      }
    }
  }
}
