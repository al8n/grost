use quote::{quote, format_ident};
use syn::parse_quote;

use crate::{network::Network, Struct};


impl Network {
  pub(crate) fn generate_encode(
    &self,
    path_to_grost: &syn::Path,
    struct_: &Struct,
  ) -> proc_macro2::TokenStream {
    let struct_name = struct_.name();
    let encoded_len = struct_.fields().iter().map(|f| {
      let field_name = f.name();
      let fn_name = super::encoded_length_delimited_len_fn_name(field_name.name_str());
      quote! {
        len += #fn_name(
          &self.#field_name,
          ctx,
        );
      }
    });

    let self_encoded_len = quote! {
      <Self as #path_to_grost::__private::Encode<#path_to_grost::__private::flavors::network::Network, #path_to_grost::__private::flavors::network::LengthDelimited>>::encoded_len(
        self,
        ctx,
      )
    };
    let self_encoded_length_delimited_len = quote! {
      <Self as #path_to_grost::__private::Encode<#path_to_grost::__private::flavors::network::Network, #path_to_grost::__private::flavors::network::LengthDelimited>>::encoded_length_delimited_len(
        self,
        ctx,
      )
    };

    let encode = struct_.fields().iter().map(|f| {
      let field_name = f.name();
      let fn_name = super::encode_length_delimited_fn_name(field_name.name_str());
      quote! {
        if offset >= buf_len {
          return ::core::result::Result::Err(
            #path_to_grost::__private::flavors::network::EncodeError::insufficient_buffer(
              #self_encoded_len,
              buf_len,
            ),
          );
        }
        offset += #fn_name(
          &self.#field_name,
          ctx,
          &mut buf[offset..],
        ).map_err(|e| {
          e.update(#self_encoded_len, buf_len)
        })?;
      }
    });

    quote! {
      #[automatically_derived]
      impl #path_to_grost::__private::Encode<#path_to_grost::__private::flavors::network::Network, #path_to_grost::__private::flavors::network::LengthDelimited> for #struct_name {
        fn encode(
          &self,
          ctx: &<#path_to_grost::__private::flavors::network::Network as #path_to_grost::__private::flavors::Flavor>::Context,
          buf: &mut [::core::primitive::u8],
        ) -> ::core::result::Result<::core::primitive::usize, <#path_to_grost::__private::flavors::network::Network as #path_to_grost::__private::flavors::Flavor>::EncodeError> {
          let mut offset = 0;
          let buf_len = buf.len();

          #(#encode)*

          ::core::result::Result::Ok(offset)
        }

        fn encoded_len(
          &self,
          ctx: &<#path_to_grost::__private::flavors::network::Network as #path_to_grost::__private::flavors::Flavor>::Context,
        ) -> ::core::primitive::usize {
          let mut len = 0;

          #(#encoded_len)*

          len
        }

        fn encoded_length_delimited_len(
          &self,
          ctx: &<#path_to_grost::__private::flavors::network::Network as #path_to_grost::__private::flavors::Flavor>::Context,
        ) -> ::core::primitive::usize {
          let encoded_len = #self_encoded_len;

          #path_to_grost::__private::varing::encoded_u32_varint_len(encoded_len as ::core::primitive::u32) + encoded_len
        }

        fn encode_length_delimited(
          &self,
          ctx: &<#path_to_grost::__private::flavors::network::Network as #path_to_grost::__private::flavors::Flavor>::Context,
          buf: &mut [::core::primitive::u8],
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

          <Self as #path_to_grost::__private::Encode<
            #path_to_grost::__private::flavors::network::Network,
            #path_to_grost::__private::flavors::network::LengthDelimited,
          >>::encode(self, ctx, &mut buf[offset..])
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