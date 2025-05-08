use quote::{ToTokens, quote};

use crate::{Struct, network::Network};

impl Network {
  pub(crate) fn generate_encode(
    &self,
    path_to_grost: &syn::Path,
    struct_: &Struct,
  ) -> proc_macro2::TokenStream {
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

    let encode_impl = self.generate_self_encode_impl(
      path_to_grost,
      struct_,
      &self_encoded_len,
      &self_encoded_length_delimited_len,
    );

    let partial_struct_encode_impl = self.generate_partial_struct_encode_impl(
      path_to_grost,
      struct_,
      &self_encoded_len,
      &self_encoded_length_delimited_len,
    );

    quote! {
      #encode_impl

      #partial_struct_encode_impl
    }
  }

  fn generate_self_encode_impl(
    &self,
    path_to_grost: &syn::Path,
    struct_: &Struct,
    self_encoded_len: impl ToTokens,
    self_encoded_length_delimited_len: impl ToTokens,
  ) -> proc_macro2::TokenStream {
    let struct_name = struct_.name();
    let encoded_len = struct_.fields().iter().map(|f| {
      let field_name = f.name();
      quote! {
        len += (<#struct_name>::reflection::<#path_to_grost::__private::flavors::Network>().#field_name().encoded_len())(
          &self.#field_name,
          ctx,
        );
      }
    });
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
        offset += (<#struct_name>::reflection::<#path_to_grost::__private::flavors::Network>().#field_name().encode())(
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

  fn generate_partial_struct_encode_impl(
    &self,
    path_to_grost: &syn::Path,
    struct_: &Struct,
    self_encoded_len: impl ToTokens,
    self_encoded_length_delimited_len: impl ToTokens,
  ) -> proc_macro2::TokenStream {
    let partial_struct_name = struct_.partial_struct_name();
    let struct_name = struct_.name();
    let encoded_len = struct_.fields().iter().map(|f| {
      let field_name = f.name();
      let ty = f.ty();

      if ty.repr().is_optional() {
        quote! {
          len += (<#struct_name>::reflection::<#path_to_grost::__private::flavors::Network>().#field_name().encoded_len())(
            &self.#field_name,
            ctx,
          );
        }
      } else {
        quote! {
          if let ::core::option::Option::Some(ref f) = self.#field_name {
            len += (<#struct_name>::reflection::<#path_to_grost::__private::flavors::Network>().#field_name().encoded_len())(
              f,
              ctx,
            );
          }
        }
      }
    });
    let encode = struct_.fields().iter().map(|f| {
      let field_name = f.name();
      let ty = f.ty();

      if ty.repr().is_optional() {
        quote! {
          if offset >= buf_len {
            return ::core::result::Result::Err(
              #path_to_grost::__private::flavors::network::EncodeError::insufficient_buffer(
                #self_encoded_len,
                buf_len,
              ),
            );
          }
          offset += (<#struct_name>::reflection::<#path_to_grost::__private::flavors::Network>().#field_name().encode())(
            &self.#field_name,
            ctx,
            &mut buf[offset..],
          ).map_err(|e| {
            e.update(#self_encoded_len, buf_len)
          })?;
        }
      } else {
        quote! {
          if let ::core::option::Option::Some(ref f) = self.#field_name {
            if offset >= buf_len {
              return ::core::result::Result::Err(
                #path_to_grost::__private::flavors::network::EncodeError::insufficient_buffer(
                  #self_encoded_len,
                  buf_len,
                ),
              );
            }
            offset += (<#struct_name>::reflection::<#path_to_grost::__private::flavors::Network>().#field_name().encode())(
              f,
              ctx,
              &mut buf[offset..],
            ).map_err(|e| {
              e.update(#self_encoded_len, buf_len)
            })?;
          }
        }
      }
    });

    quote! {
      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl<__GROST_BYTES_BUFFER__, __GROST_UNKNOWN_BUFFER__> #path_to_grost::__private::Encode<#path_to_grost::__private::flavors::network::Network, #path_to_grost::__private::flavors::network::LengthDelimited> for #partial_struct_name<__GROST_BYTES_BUFFER__, __GROST_UNKNOWN_BUFFER__>
      where
        __GROST_UNKNOWN_BUFFER__: #path_to_grost::__private::Buffer<#path_to_grost::__private::flavors::network::Unknown<__GROST_BYTES_BUFFER__>>,
        __GROST_BYTES_BUFFER__: #path_to_grost::__private::BytesBuffer,
      {
        fn encode(
          &self,
          ctx: &<#path_to_grost::__private::flavors::network::Network as #path_to_grost::__private::flavors::Flavor>::Context,
          buf: &mut [::core::primitive::u8],
        ) -> ::core::result::Result<::core::primitive::usize, <#path_to_grost::__private::flavors::network::Network as #path_to_grost::__private::flavors::Flavor>::EncodeError> {
          let mut offset = 0;
          let buf_len = buf.len();

          #(#encode)*

          if !ctx.skip_unknown() {
            if let ::core::option::Option::Some(ref unknowns) = self.__grost_unknown__ {
              let unknowns = unknowns.as_slice();
              for unknown in unknowns {
                if offset >= buf_len {
                  return ::core::result::Result::Err(
                    #path_to_grost::__private::flavors::network::EncodeError::insufficient_buffer(#self_encoded_len, buf_len),
                  );
                }

                offset += <#path_to_grost::__private::flavors::Network as #path_to_grost::__private::flavors::Flavor>::encode_unknown(
                  ctx,
                  unknown,
                  &mut buf[offset..],
                )?;
              }
            }
          }

          ::core::result::Result::Ok(offset)
        }

        fn encoded_len(
          &self,
          ctx: &<#path_to_grost::__private::flavors::network::Network as #path_to_grost::__private::flavors::Flavor>::Context,
        ) -> ::core::primitive::usize {
          let mut len = 0;

          #(#encoded_len)*

          if !ctx.skip_unknown() {
            if let ::core::option::Option::Some(ref unknowns) = self.__grost_unknown__ {
              let unknowns = unknowns.as_slice();
              for unknown in unknowns {
                len += <#path_to_grost::__private::flavors::Network as #path_to_grost::__private::flavors::Flavor>::encoded_unknown_len(
                  ctx,
                  unknown,
                );
              }
            }
          }

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
