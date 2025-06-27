use quote::{ToTokens, quote};

use crate::{Object, groto::Groto};

impl Groto {
  pub(crate) fn derive_partial_encode(
    &self,
    path_to_grost: &syn::Path,
    struct_: &Object,
  ) -> proc_macro2::TokenStream {
    let self_encoded_len = quote! {
      <Self as #path_to_grost::__private::PartialEncode<#path_to_grost::__private::flavors::Groto, #path_to_grost::__private::flavors::groto::LengthDelimited>>::partial_encoded_len(
        self,
        ctx,
        selector,
      )
    };
    let self_encoded_length_delimited_len = quote! {
      <Self as #path_to_grost::__private::PartialEncode<#path_to_grost::__private::flavors::Groto, #path_to_grost::__private::flavors::groto::LengthDelimited>>::partial_encoded_length_delimited_len(
        self,
        ctx,
        selector,
      )
    };

    let partial_encode_partial_object = self.derive_partial_encode_partial_object(
      path_to_grost,
      struct_,
      &self_encoded_len,
      &self_encoded_length_delimited_len,
    );

    let partial_encode_object = self.derive_partial_encode_object(
      path_to_grost,
      struct_,
      &self_encoded_len,
      &self_encoded_length_delimited_len,
    );

    let partial_encode_partial_ref_object = self.derive_partial_encode_partial_ref_object(
      path_to_grost,
      struct_,
      &self_encoded_len,
      &self_encoded_length_delimited_len,
    );

    quote! {
      #partial_encode_partial_object

      #partial_encode_object

      #partial_encode_partial_ref_object
    }
  }

  fn derive_partial_encode_object(
    &self,
    path_to_grost: &syn::Path,
    struct_: &Object,
    self_encoded_len: impl ToTokens,
    self_encoded_length_delimited_len: impl ToTokens,
  ) -> proc_macro2::TokenStream {
    let struct_name = struct_.name();

    let encoded_len = struct_.fields().iter().map(|f| {
      let field_name = f.name();
      quote! {
        len += (<#struct_name>::reflection::<#path_to_grost::__private::flavors::Groto>().#field_name().partial_encoded_len())(
          &self.#field_name,
          ctx,
          &selector.#field_name,
        );
      }
    });

    let encode = struct_.fields().iter().map(|f| {
      let field_name = f.name();
      quote! {
        if offset >= buf_len {
          return ::core::result::Result::Err(
            #path_to_grost::__private::flavors::groto::Error::insufficient_buffer(
              #self_encoded_len,
              buf_len,
            ),
          );
        }
        offset += (<#struct_name>::reflection::<#path_to_grost::__private::flavors::Groto>().#field_name().partial_encode())(
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
      impl #path_to_grost::__private::PartialEncode<#path_to_grost::__private::flavors::Groto, #path_to_grost::__private::flavors::groto::LengthDelimited> for #struct_name {
        fn partial_encode(
          &self,
          ctx: &<#path_to_grost::__private::flavors::Groto as #path_to_grost::__private::flavors::Flavor>::Context,
          buf: &mut [::core::primitive::u8],
          selector: &<#struct_name as #path_to_grost::__private::selection::Selectable<
            #path_to_grost::__private::flavors::Groto,
            #path_to_grost::__private::flavors::groto::LengthDelimited,
          >>::Selector,
        ) -> ::core::result::Result<::core::primitive::usize, <#path_to_grost::__private::flavors::Groto as #path_to_grost::__private::flavors::Flavor>::Error> {
          let mut offset = 0;
          let buf_len = buf.len();

          #(#encode)*

          ::core::result::Result::Ok(offset)
        }

        fn partial_encoded_len(
          &self,
          ctx: &<#path_to_grost::__private::flavors::Groto as #path_to_grost::__private::flavors::Flavor>::Context,
          selector: &<#struct_name as #path_to_grost::__private::selection::Selectable<
            #path_to_grost::__private::flavors::Groto,
            #path_to_grost::__private::flavors::groto::LengthDelimited,
          >>::Selector,
        ) -> ::core::primitive::usize {
          let mut len = 0;

          #(#encoded_len)*

          len
        }

        fn partial_encoded_length_delimited_len(
          &self,
          ctx: &<#path_to_grost::__private::flavors::Groto as #path_to_grost::__private::flavors::Flavor>::Context,
          selector: &<#struct_name as #path_to_grost::__private::selection::Selectable<
            #path_to_grost::__private::flavors::Groto,
            #path_to_grost::__private::flavors::groto::LengthDelimited,
          >>::Selector,
        ) -> ::core::primitive::usize {
          let encoded_len = #self_encoded_len;

          #path_to_grost::__private::varing::encoded_u32_varint_len(encoded_len as ::core::primitive::u32) + encoded_len
        }

        fn partial_encode_length_delimited(
          &self,
          ctx: &<#path_to_grost::__private::flavors::Groto as #path_to_grost::__private::flavors::Flavor>::Context,
          buf: &mut [::core::primitive::u8],
          selector: &<#struct_name as #path_to_grost::__private::selection::Selectable<
            #path_to_grost::__private::flavors::Groto,
            #path_to_grost::__private::flavors::groto::LengthDelimited,
          >>::Selector,
        ) -> ::core::result::Result<::core::primitive::usize, <#path_to_grost::__private::flavors::Groto as #path_to_grost::__private::flavors::Flavor>::Error> {
          let encoded_len = #self_encoded_len;
          let buf_len = buf.len();
          let offset = #path_to_grost::__private::varing::encode_u32_varint_to(encoded_len as ::core::primitive::u32, buf)
            .map_err(|e| {
              #path_to_grost::__private::flavors::groto::Error::from_varint_error(e)
                .update(#self_encoded_length_delimited_len, buf_len)
            })?;

          if offset >= buf_len {
            return ::core::result::Result::Err(
              #path_to_grost::__private::flavors::groto::Error::insufficient_buffer(#self_encoded_length_delimited_len, buf_len)
            );
          }

          <Self as #path_to_grost::__private::PartialEncode<
            #path_to_grost::__private::flavors::Groto,
            #path_to_grost::__private::flavors::groto::LengthDelimited,
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

  fn derive_partial_encode_partial_object(
    &self,
    path_to_grost: &syn::Path,
    struct_: &Object,
    self_encoded_len: impl ToTokens,
    self_encoded_length_delimited_len: impl ToTokens,
  ) -> proc_macro2::TokenStream {
    let partial_struct_name = struct_.partial_struct_name();
    let struct_name = struct_.name();

    let partial_struct_encoded_len = struct_.fields().iter().map(|f| {
      let field_name = f.name();
      if f.ty().repr().is_optional() {
        quote! {
          len += (<#struct_name>::reflection::<#path_to_grost::__private::flavors::Groto>().#field_name().partial_encoded_len())(
            &self.#field_name,
            ctx,
            &selector.#field_name,
          );
        }
      } else {
        quote! {
          if let ::core::option::Option::Some(ref f) = self.#field_name {
            len += (<#struct_name>::reflection::<#path_to_grost::__private::flavors::Groto>().#field_name().partial_encoded_len())(
              f,
              ctx,
              &selector.#field_name,
            );
          }
        }
      }
    });

    let partial_struct_encode = struct_.fields().iter().map(|f| {
      let field_name = f.name();
      if f.ty().repr().is_optional() {
        quote! {
          if offset >= buf_len {
            return ::core::result::Result::Err(
              #path_to_grost::__private::flavors::groto::Error::insufficient_buffer(
                #self_encoded_len,
                buf_len,
              ),
            );
          }

          offset += (<#struct_name>::reflection::<#path_to_grost::__private::flavors::Groto>().#field_name().partial_encode())(
            &self.#field_name,
            ctx,
            &mut buf[offset..],
            &selector.#field_name,
          ).map_err(|e| {
            e.update(#self_encoded_len, buf_len)
          })?;
        }
      } else {
        quote! {
          if let ::core::option::Option::Some(ref f) = self.#field_name {
            if offset >= buf_len {
              return ::core::result::Result::Err(
                #path_to_grost::__private::flavors::groto::Error::insufficient_buffer(
                  #self_encoded_len,
                  buf_len,
                ),
              );
            }

            offset += (<#struct_name>::reflection::<#path_to_grost::__private::flavors::Groto>().#field_name().partial_encode())(
              f,
              ctx,
              &mut buf[offset..],
              &selector.#field_name,
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
      impl #path_to_grost::__private::PartialEncode<#path_to_grost::__private::flavors::Groto, #path_to_grost::__private::flavors::groto::LengthDelimited> for #partial_struct_name
      {
        fn partial_encode(
          &self,
          ctx: &<#path_to_grost::__private::flavors::Groto as #path_to_grost::__private::flavors::Flavor>::Context,
          buf: &mut [::core::primitive::u8],
          selector: &<#struct_name as #path_to_grost::__private::selection::Selectable<
            #path_to_grost::__private::flavors::Groto,
            #path_to_grost::__private::flavors::groto::LengthDelimited,
          >>::Selector,
        ) -> ::core::result::Result<::core::primitive::usize, <#path_to_grost::__private::flavors::Groto as #path_to_grost::__private::flavors::Flavor>::Error> {
          let mut offset = 0;
          let buf_len = buf.len();

          #(#partial_struct_encode)*

          // if !ctx.skip_unknown() {
          //   if let ::core::option::Option::Some(ref unknowns) = self.__grost_unknown__ {
          //     let unknowns = unknowns.as_slice();
          //     for unknown in unknowns {
          //       if offset >= buf_len {
          //         return ::core::result::Result::Err(
          //           #path_to_grost::__private::flavors::groto::Error::insufficient_buffer(#self_encoded_len, buf_len),
          //         );
          //       }

          //       offset += <#path_to_grost::__private::flavors::Groto as #path_to_grost::__private::flavors::Flavor>::encode_unknown(
          //         ctx,
          //         unknown,
          //         &mut buf[offset..],
          //       )?;
          //     }
          //   }
          // }

          ::core::result::Result::Ok(offset)
        }

        fn partial_encoded_len(
          &self,
          ctx: &<#path_to_grost::__private::flavors::Groto as #path_to_grost::__private::flavors::Flavor>::Context,
          selector: &<#struct_name as #path_to_grost::__private::selection::Selectable<
            #path_to_grost::__private::flavors::Groto,
            #path_to_grost::__private::flavors::groto::LengthDelimited,
          >>::Selector,
        ) -> ::core::primitive::usize {
          let mut len = 0;

          #(#partial_struct_encoded_len)*

          // if !ctx.skip_unknown() {
          //   if let ::core::option::Option::Some(ref unknowns) = self.__grost_unknown__ {
          //     let unknowns = unknowns.as_slice();
          //     for unknown in unknowns {
          //       len += <#path_to_grost::__private::flavors::Groto as #path_to_grost::__private::flavors::Flavor>::encoded_unknown_len(
          //         ctx,
          //         unknown,
          //       );
          //     }
          //   }
          // }

          len
        }

        fn partial_encoded_length_delimited_len(
          &self,
          ctx: &<#path_to_grost::__private::flavors::Groto as #path_to_grost::__private::flavors::Flavor>::Context,
          selector: &<#struct_name as #path_to_grost::__private::selection::Selectable<
            #path_to_grost::__private::flavors::Groto,
            #path_to_grost::__private::flavors::groto::LengthDelimited,
          >>::Selector,
        ) -> ::core::primitive::usize {
          let encoded_len = #self_encoded_len;

          #path_to_grost::__private::varing::encoded_u32_varint_len(encoded_len as ::core::primitive::u32) + encoded_len
        }

        fn partial_encode_length_delimited(
          &self,
          ctx: &<#path_to_grost::__private::flavors::Groto as #path_to_grost::__private::flavors::Flavor>::Context,
          buf: &mut [::core::primitive::u8],
          selector: &<#struct_name as #path_to_grost::__private::selection::Selectable<
            #path_to_grost::__private::flavors::Groto,
            #path_to_grost::__private::flavors::groto::LengthDelimited,
          >>::Selector,
        ) -> ::core::result::Result<::core::primitive::usize, <#path_to_grost::__private::flavors::Groto as #path_to_grost::__private::flavors::Flavor>::Error> {
          let encoded_len = #self_encoded_len;
          let buf_len = buf.len();
          let offset = #path_to_grost::__private::varing::encode_u32_varint_to(encoded_len as ::core::primitive::u32, buf)
            .map_err(|e| {
              #path_to_grost::__private::flavors::groto::Error::from_varint_error(e)
                .update(#self_encoded_length_delimited_len, buf_len)
            })?;

          if offset >= buf_len {
            return ::core::result::Result::Err(
              #path_to_grost::__private::flavors::groto::Error::insufficient_buffer(#self_encoded_length_delimited_len, buf_len)
            );
          }

          <Self as #path_to_grost::__private::PartialEncode<
            #path_to_grost::__private::flavors::Groto,
            #path_to_grost::__private::flavors::groto::LengthDelimited,
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

  fn derive_partial_encode_partial_ref_object(
    &self,
    path_to_grost: &syn::Path,
    struct_: &Object,
    self_encoded_len: impl ToTokens,
    self_encoded_length_delimited_len: impl ToTokens,
  ) -> proc_macro2::TokenStream {
    let partial_ref_name = struct_.partial_ref_name();
    let struct_name = struct_.name();
    let partial_struct_encoded_len = struct_.fields().iter().map(|f| {
      let field_name = f.name();
      if f.ty().repr().is_optional() {
        quote! {
          len += (<#struct_name>::reflection::<#path_to_grost::__private::flavors::Groto>().#field_name().partial_encoded_ref_len())(
            &self.#field_name,
            ctx,
            &selector.#field_name,
          );
        }
      } else {
        quote! {
          if let ::core::option::Option::Some(ref f) = self.#field_name {
            len += (<#struct_name>::reflection::<#path_to_grost::__private::flavors::Groto>().#field_name().partial_encoded_ref_len())(
              f,
              ctx,
              &selector.#field_name,
            );
          }
        }
      }
    });

    let partial_struct_encode = struct_.fields().iter().map(|f| {
      let field_name = f.name();
      if f.ty().repr().is_optional() {
        quote! {
          if offset >= buf_len {
            return ::core::result::Result::Err(
              #path_to_grost::__private::flavors::groto::Error::insufficient_buffer(
                #self_encoded_len,
                buf_len,
              ),
            );
          }

          offset += (<#struct_name>::reflection::<#path_to_grost::__private::flavors::Groto>().#field_name().partial_encode_ref())(
            &self.#field_name,
            ctx,
            &mut buf[offset..],
            &selector.#field_name,
          ).map_err(|e| {
            e.update(#self_encoded_len, buf_len)
          })?;
        }
      } else {
        quote! {
          if let ::core::option::Option::Some(ref f) = self.#field_name {
            if offset >= buf_len {
              return ::core::result::Result::Err(
                #path_to_grost::__private::flavors::groto::Error::insufficient_buffer(
                  #self_encoded_len,
                  buf_len,
                ),
              );
            }

            offset += (<#struct_name>::reflection::<#path_to_grost::__private::flavors::Groto>().#field_name().partial_encode_ref())(
              f,
              ctx,
              &mut buf[offset..],
              &selector.#field_name,
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
      impl<'__grost_lifetime__> #path_to_grost::__private::PartialEncode<#path_to_grost::__private::flavors::Groto, #path_to_grost::__private::flavors::groto::LengthDelimited> for #partial_ref_name<'__grost_lifetime__, #path_to_grost::__private::flavors::Groto>
      {
        fn partial_encode(
          &self,
          ctx: &<#path_to_grost::__private::flavors::Groto as #path_to_grost::__private::flavors::Flavor>::Context,
          buf: &mut [::core::primitive::u8],
          selector: &<#struct_name as #path_to_grost::__private::selection::Selectable<
            #path_to_grost::__private::flavors::Groto,
            #path_to_grost::__private::flavors::groto::LengthDelimited,
          >>::Selector,
        ) -> ::core::result::Result<::core::primitive::usize, <#path_to_grost::__private::flavors::Groto as #path_to_grost::__private::flavors::Flavor>::Error> {
          let mut offset = 0;
          let buf_len = buf.len();

          #(#partial_struct_encode)*

          // if !ctx.skip_unknown() {
          //   if let ::core::option::Option::Some(ref unknowns) = self.__grost_unknown__ {
          //     let unknowns = unknowns.as_slice();
          //     for unknown in unknowns {
          //       if offset >= buf_len {
          //         return ::core::result::Result::Err(
          //           #path_to_grost::__private::flavors::groto::Error::insufficient_buffer(#self_encoded_len, buf_len),
          //         );
          //       }

          //       offset += <#path_to_grost::__private::flavors::Groto as #path_to_grost::__private::flavors::Flavor>::encode_unknown(
          //         ctx,
          //         unknown,
          //         &mut buf[offset..],
          //       )?;
          //     }
          //   }
          // }

          ::core::result::Result::Ok(offset)
        }

        fn partial_encoded_len(
          &self,
          ctx: &<#path_to_grost::__private::flavors::Groto as #path_to_grost::__private::flavors::Flavor>::Context,
          selector: &<#struct_name as #path_to_grost::__private::selection::Selectable<
            #path_to_grost::__private::flavors::Groto,
            #path_to_grost::__private::flavors::groto::LengthDelimited,
          >>::Selector,
        ) -> ::core::primitive::usize {
          let mut len = 0;

          #(#partial_struct_encoded_len)*

          // if !ctx.skip_unknown() {
          //   if let ::core::option::Option::Some(ref unknowns) = self.__grost_unknown__ {
          //     let unknowns = unknowns.as_slice();
          //     for unknown in unknowns {
          //       len += <#path_to_grost::__private::flavors::Groto as #path_to_grost::__private::flavors::Flavor>::encoded_unknown_len(
          //         ctx,
          //         unknown,
          //       );
          //     }
          //   }
          // }

          len
        }

        fn partial_encoded_length_delimited_len(
          &self,
          ctx: &<#path_to_grost::__private::flavors::Groto as #path_to_grost::__private::flavors::Flavor>::Context,
          selector: &<#struct_name as #path_to_grost::__private::selection::Selectable<
            #path_to_grost::__private::flavors::Groto,
            #path_to_grost::__private::flavors::groto::LengthDelimited,
          >>::Selector,
        ) -> ::core::primitive::usize {
          let encoded_len = #self_encoded_len;

          #path_to_grost::__private::varing::encoded_u32_varint_len(encoded_len as ::core::primitive::u32) + encoded_len
        }

        fn partial_encode_length_delimited(
          &self,
          ctx: &<#path_to_grost::__private::flavors::Groto as #path_to_grost::__private::flavors::Flavor>::Context,
          buf: &mut [::core::primitive::u8],
          selector: &<#struct_name as #path_to_grost::__private::selection::Selectable<
            #path_to_grost::__private::flavors::Groto,
            #path_to_grost::__private::flavors::groto::LengthDelimited,
          >>::Selector,
        ) -> ::core::result::Result<::core::primitive::usize, <#path_to_grost::__private::flavors::Groto as #path_to_grost::__private::flavors::Flavor>::Error> {
          let encoded_len = #self_encoded_len;
          let buf_len = buf.len();
          let offset = #path_to_grost::__private::varing::encode_u32_varint_to(encoded_len as ::core::primitive::u32, buf)
            .map_err(|e| {
              #path_to_grost::__private::flavors::groto::Error::from_varint_error(e)
                .update(#self_encoded_length_delimited_len, buf_len)
            })?;

          if offset >= buf_len {
            return ::core::result::Result::Err(
              #path_to_grost::__private::flavors::groto::Error::insufficient_buffer(#self_encoded_length_delimited_len, buf_len)
            );
          }

          <Self as #path_to_grost::__private::PartialEncode<
            #path_to_grost::__private::flavors::Groto,
            #path_to_grost::__private::flavors::groto::LengthDelimited,
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
