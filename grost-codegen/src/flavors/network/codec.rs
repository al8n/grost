use crate::Struct;

use super::Network;

use quote::{format_ident, quote};
use syn::parse_quote;

mod partial_encode;
mod index;

impl Network {
  pub(super) fn generate_field_encode_fns(&self, path_to_grost: &syn::Path, struct_: &Struct) -> proc_macro2::TokenStream {
    let struct_name = struct_.name();

    // let self_encoded_len = quote! {
    //   <#struct_name as #path_to_grost::__private::PartialEncode<#path_to_grost::__private::flavors::network::Network, #path_to_grost::__private::flavors::network::LengthDelimited>>::partial_encoded_len(
    //     f,
    //     ctx,
    //     selector,
    //   )
    // };
    // let self_encoded_length_delimited_len = quote! {
    //   <#struct_name as #path_to_grost::__private::PartialEncode<#path_to_grost::__private::flavors::network::Network, #path_to_grost::__private::flavors::network::LengthDelimited>>::partial_encoded_length_delimited_len(
    //     self,
    //     ctx,
    //     selector,
    //   )
    // };
    // let insufficient_error = quote! {
    //   ::core::result::Result::Err(
    //     #path_to_grost::__private::flavors::network::EncodeError::insufficient_buffer(
    //       #self_encoded_len,
    //       buf_len,
    //     ),
    //   )
    // };

    let fns = struct_.fields().iter().map(|f| {
      let ty = f.ty();
      let rust_ty = ty.ty();
      let field_name = f.name();
      let encoded_len_fn = self.field_encoded_len_fn_name(field_name.name_str());
      let encode_fn = self.field_encode_fn_name(field_name.name_str());
      let wf = f.get_wire_format(self).cloned().unwrap_or_else(|| {
        parse_quote!(<#rust_ty as #path_to_grost::__private::DefaultWireFormat<#path_to_grost::__private::flavors::Network>>::Format)
      });
      let identifier_encoded_len = self.encoded_len_field_identifier_const_name(field_name.name_str());
      let identifier_encoded = self.encoded_field_identifier_const_name(field_name.name_str());
      if ty.primitive_selection_type() {
        quote! {
          fn #encoded_len_fn(
            f: &#rust_ty,
            ctx: &#path_to_grost::__private::flavors::network::Context,
          ) -> ::core::primitive::usize {
            #identifier_encoded_len
              + <#rust_ty as #path_to_grost::__private::Encode<
                #path_to_grost::__private::flavors::network::Network,
                #wf
              >>::encoded_length_delimited_len(
                f,
                ctx,
              )
          }

          fn #encode_fn(
            f: &#rust_ty,
            ctx: &#path_to_grost::__private::flavors::network::Context,
            buf: &mut [::core::primitive::u8],
          ) -> ::core::result::Result<::core::primitive::usize, #path_to_grost::__private::flavors::network::EncodeError> {
            let buf_len = buf.len();
            let mut offset = 0;
            if offset > buf_len {
              return ::core::result::Result::Err(insufficient_buffer_error::<_, #wf>(f, ctx, ::core::option::Option::None, buf_len));
            }

            buf[offset..offset + #identifier_encoded_len].copy_from_slice(#identifier_encoded);
            offset += #identifier_encoded_len;

            if offset >= buf_len {
              return ::core::result::Result::Err(insufficient_buffer_error::<_, #wf>(f, ctx, ::core::option::Option::None, buf_len));
            }

            <#rust_ty as #path_to_grost::__private::Encode<
              #path_to_grost::__private::flavors::network::Network,
              #wf
            >>::encode_length_delimited(
              f,
              ctx,
              &mut buf[offset..],
            )
            .map(|len| offset + len)
          }
        }
      } else {
        quote! {
          fn #encoded_len_fn(
            f: &#rust_ty,
            ctx: &#path_to_grost::__private::flavors::network::Context,
            selector: ::core::option::Option<&<#rust_ty as #path_to_grost::__private::Selectable<#path_to_grost::__private::flavors::network::Network>>::Selector>,
          ) -> ::core::primitive::usize {
            match selector {
              ::core::option::Option::Some(selector) => {
                #identifier_encoded_len
                  + <#rust_ty as #path_to_grost::__private::PartialEncode<
                      #path_to_grost::__private::flavors::network::Network,
                      #wf,
                    >>::partial_encoded_length_delimited_len(
                      f,
                      ctx,
                      selector,
                    )
              },
              ::core::option::Option::None => {
                #identifier_encoded_len
                  + <#rust_ty as #path_to_grost::__private::Encode<
                      #path_to_grost::__private::flavors::network::Network,
                      #wf,
                    >>::encoded_length_delimited_len(
                      f,
                      ctx,
                    )
              }
            }
          }
        }
      }
    });

    quote! {
      fn insufficient_buffer_error<T, W>(
        f: &T,
        ctx: &<#path_to_grost::__private::flavors::network::Network as #path_to_grost::__private::flavors::Flavor>::Context,
        selector: ::core::option::Option<&<T as #path_to_grost::__private::Selectable<#path_to_grost::__private::flavors::network::Network>>::Selector>,
        buf_len: ::core::primitive::usize,
      ) -> #path_to_grost::__private::flavors::network::EncodeError
      where
        T: #path_to_grost::__private::PartialEncode<
            #path_to_grost::__private::flavors::network::Network,
            W,
          >
          + #path_to_grost::__private::Encode<
            #path_to_grost::__private::flavors::network::Network,
            W,
          >
          + #path_to_grost::__private::Selectable<#path_to_grost::__private::flavors::network::Network>
          + ?::core::marker::Sized,
        W: #path_to_grost::__private::WireFormat<
            #path_to_grost::__private::flavors::network::Network,
          >,
      {
        match selector {
          ::core::option::Option::Some(selector) => {
            #path_to_grost::__private::flavors::network::EncodeError::insufficient_buffer(
              <T as #path_to_grost::__private::PartialEncode<
                #path_to_grost::__private::flavors::network::Network,
                W,
              >>::partial_encoded_len(f, ctx, selector),
              buf_len,
            )
          }
          ::core::option::Option::None => {
            #path_to_grost::__private::flavors::network::EncodeError::insufficient_buffer(
              <T as #path_to_grost::__private::Encode<
                #path_to_grost::__private::flavors::network::Network,
                W,
              >>::encoded_length_delimited_len(f, ctx),
              buf_len,
            )
          }
        }
      }

      #(#fns)*
    }
  }

  fn field_encoded_len_fn_name(&self, field_name: &str) -> syn::Ident {
    format_ident!("encoded_{}_len", field_name)
  }

  fn field_encode_fn_name(&self, field_name: &str) -> syn::Ident {
    format_ident!("encode_{}", field_name)
  }
}
