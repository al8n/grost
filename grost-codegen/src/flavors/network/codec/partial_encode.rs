use quote::{quote, format_ident};
use syn::parse_quote;

use crate::{network::Network, Struct};


impl Network {
  pub(crate) fn generate_partial_encode(
    &self,
    path_to_grost: &syn::Path,
    struct_: &Struct,
  ) -> proc_macro2::TokenStream {
    let struct_name = struct_.name();
    let encoded_len = struct_.fields().iter().map(|f| {
      let is_select_fn = format_ident!("is_{}_selected", f.name().name_str());
      let field_name = f.name();
      let ty = f.ty();
      let rust_ty = ty.ty();
      let wf = f.get_wire_format(self).cloned().unwrap_or_else(|| {
        parse_quote!(<#rust_ty as #path_to_grost::__private::DefaultWireFormat<#path_to_grost::__private::flavors::Network>>::Format)
      });
      let identifier_encoded_len = self.encoded_len_field_identifier_const_name(field_name.name_str());
      let encoded_len_fn = self.field_encoded_len_fn_name(field_name.name_str());
      if ty.primitive_selection_type() {
        quote! {
          if selector.#is_select_fn() {
            len += #encoded_len_fn(&self.#field_name, ctx);
          }
        }
      } else {
        let selector_fn = format_ident!("{}_ref", f.name().name_str());
        quote! {
          if selector.#is_select_fn() {
            len += #encoded_len_fn(&self.#field_name, ctx, ::core::option::Option::Some(selector.#selector_fn()));
          }
        }
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
    let insufficient_error = quote! {
      ::core::result::Result::Err(
        #path_to_grost::__private::flavors::network::EncodeError::insufficient_buffer(
          #self_encoded_len,
          buf_len,
        ),
      )
    };

    let encode = struct_.fields().iter().map(|f| {
      let is_select_fn = format_ident!("is_{}_selected", f.name().name_str());
      let field_name = f.name();
      let ty = f.ty();
      let rust_ty = ty.ty();
      let wf = f.get_wire_format(self).cloned().unwrap_or_else(|| {
        parse_quote!(<#rust_ty as #path_to_grost::__private::DefaultWireFormat<#path_to_grost::__private::flavors::Network>>::Format)
      });
      let identifier_encoded_len = self.encoded_len_field_identifier_const_name(field_name.name_str());
      let identifier_encoded = self.encoded_field_identifier_const_name(field_name.name_str());
      if ty.primitive_selection_type() {
        quote! {
          if selector.#is_select_fn() {
            if offset + #identifier_encoded_len > buf_len {
              return #insufficient_error;
            }

            buf[offset..offset + #identifier_encoded_len].copy_from_slice(#identifier_encoded);
            offset += #identifier_encoded_len;

            if offset >= buf_len {
              return #insufficient_error;
            }

            offset += <#rust_ty as #path_to_grost::__private::Encode<
                #path_to_grost::__private::flavors::network::Network,
                #wf
              >>::encode_length_delimited(
                &self.#field_name,
                ctx,
                &mut buf[offset..],
              ).map_err(|e| e.update(#self_encoded_len, buf_len))?;
          }
        }
      } else {
        let selector_fn = format_ident!("{}_ref", f.name().name_str());
        quote! {
          if selector.#is_select_fn() {
            if offset + #identifier_encoded_len > buf_len {
              return #insufficient_error;
            }

            buf[offset..offset + #identifier_encoded_len].copy_from_slice(#identifier_encoded);
            offset += #identifier_encoded_len;

            if offset >= buf_len {
              return #insufficient_error;
            }

            offset += <#rust_ty as #path_to_grost::__private::PartialEncode<
                #path_to_grost::__private::flavors::network::Network,
                #wf,
              >>::partial_encode_length_delimited(
                &self.#field_name,
                ctx,
                &mut buf[offset..],
                selector.#selector_fn(),
              ).map_err(|e| e.update(#self_encoded_len, buf_len))?;
          }
        }
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