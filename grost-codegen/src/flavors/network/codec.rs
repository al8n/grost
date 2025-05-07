use crate::{Field, Struct};

use super::Network;

use quote::quote;
use syn::parse_quote;

mod encode;
mod index;
mod partial_encode;

impl Network {
  pub(super) fn generate_field_encode_fns(
    &self,
    path_to_grost: &syn::Path,
    struct_: &Struct,
  ) -> proc_macro2::TokenStream {
    let fns = struct_.fields().iter().map(|f| {
      let ty = f.ty();
      let rust_ty = ty.ty();
      let wf = f.get_wire_format(self).cloned().unwrap_or_else(|| {
        parse_quote!(<#rust_ty as #path_to_grost::__private::DefaultWireFormat<#path_to_grost::__private::flavors::Network>>::Format)
      });
      let partial_encoded_len_fn = self.generate_partial_encoded_field_len_reflection(
        path_to_grost,
        struct_,
        f,
        &wf,
      );
      let partial_encode_fn = self.generate_partial_encode_field_reflection(
        path_to_grost,
        struct_,
        f,
        &wf,
      );
      let encoded_len_fn = self.generate_encoded_field_len_reflection(
        path_to_grost,
        struct_,
        f,
        &wf,
      );
      let encode_fn = self.generate_encode_field_reflection(
        path_to_grost,
        struct_,
        f,
        &wf,
      );

      quote! {
        #partial_encoded_len_fn

        #encoded_len_fn

        #encode_fn

        #partial_encode_fn
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

  fn generate_encode_field_reflection(
    &self,
    path_to_grost: &syn::Path,
    struct_: &Struct,
    f: &Field,
    wf: &syn::Type,
  ) -> proc_macro2::TokenStream {
    let struct_name = struct_.name();
    let field_name = f.name();
    let reflection_mod_name = struct_.reflection_mod_name();
    let field_reflection = struct_.field_reflection_name();
    let encoded_identifier = quote! {
      <#struct_name>::reflection::<#path_to_grost::__private::flavors::Network>().#field_name().encoded_identifier()
    };
    let encoded_identifier_len = quote! {
      let identifier_len = *<#struct_name>::reflection::<#path_to_grost::__private::flavors::Network>().#field_name().encoded_identifier_len();
    };
    let ty = f.ty();
    let tag = f.tag();
    let encoded_len_fn = quote! {
      const ENCODED_LEN_FN: #reflection_mod_name::#field_reflection<
        #path_to_grost::__private::reflection::encode::EncodeReflection<
          #path_to_grost::__private::reflection::Len<#path_to_grost::__private::reflection::encode::EncodeField>
        >,
        #path_to_grost::__private::flavors::Network,
        #tag,
      > = <#struct_name>::reflection::<#path_to_grost::__private::flavors::Network>().#field_name().encoded_len();
    };

    let reflection = quote! {
      fn(&#ty, &#path_to_grost::__private::network::Context, &mut [::core::primitive::u8]) -> ::core::result::Result<::core::primitive::usize, #path_to_grost::__private::flavors::network::EncodeError>
    };
    let fn_impl = if ty.repr().is_optional() {
      let atomic_ty = ty.repr().encode_atomic_ty();
      quote! {
        fn encode(
          f: &::core::option::Option<#atomic_ty>,
          ctx: &#path_to_grost::__private::flavors::network::Context,
          buf: &mut [::core::primitive::u8],
        ) -> ::core::result::Result<::core::primitive::usize, #path_to_grost::__private::flavors::network::EncodeError> {
          #encoded_len_fn

          #encoded_identifier_len

          match f {
            ::core::option::Option::None => ::core::result::Result::Ok(0),
            ::core::option::Option::Some(field) => {
              let buf_len = buf.len();
              let mut offset = 0;
              if offset > buf_len {
                return ::core::result::Result::Err(
                  #path_to_grost::__private::flavors::network::EncodeError::insufficient_buffer(
                    (ENCODED_LEN_FN)(f, ctx),
                    buf_len,
                  ),
                );
              }

              buf[offset..offset + identifier_len].copy_from_slice(&#encoded_identifier);
              offset += identifier_len;

              if offset >= buf_len {
                return ::core::result::Result::Err(
                  #path_to_grost::__private::flavors::network::EncodeError::insufficient_buffer(
                    (ENCODED_LEN_FN)(f, ctx),
                    buf_len,
                  ),
                );
              }

              <#atomic_ty as #path_to_grost::__private::Encode<
                #path_to_grost::__private::flavors::network::Network,
                #wf
              >>::encode_length_delimited(
                field,
                ctx,
                &mut buf[offset..],
              )
              .map(|len| offset + len)
              .map_err(|e| e.update((ENCODED_LEN_FN)(f, ctx), buf_len))
            },
          }
        }
      }
    } else {
      quote! {
        fn encode(
          f: &#ty,
          ctx: &#path_to_grost::__private::flavors::network::Context,
          buf: &mut [::core::primitive::u8],
        ) -> ::core::result::Result<::core::primitive::usize, #path_to_grost::__private::flavors::network::EncodeError> {
          #encoded_len_fn

          #encoded_identifier_len

          let buf_len = buf.len();
          let mut offset = 0;
          if offset > buf_len {
            return ::core::result::Result::Err(
              #path_to_grost::__private::flavors::network::EncodeError::insufficient_buffer(
                (ENCODED_LEN_FN)(f, ctx),
                buf_len,
              ),
            );
          }

          buf[offset..offset + identifier_len].copy_from_slice(&#encoded_identifier);
          offset += identifier_len;

          if offset >= buf_len {
            return ::core::result::Result::Err(
              #path_to_grost::__private::flavors::network::EncodeError::insufficient_buffer(
                (ENCODED_LEN_FN)(f, ctx),
                buf_len,
              ),
            );
          }

          <#ty as #path_to_grost::__private::Encode<
            #path_to_grost::__private::flavors::network::Network,
            #wf
          >>::encode_length_delimited(
            f,
            ctx,
            &mut buf[offset..],
          )
          .map(|len| offset + len)
          .map_err(|e| e.update((ENCODED_LEN_FN)(f, ctx), buf_len))
        }
      }
    };

    quote! {
      impl #path_to_grost::__private::reflection::Reflectable<
        #path_to_grost::__private::flavors::Network,
      > for
        #reflection_mod_name::#field_reflection<
          #path_to_grost::__private::reflection::encode::EncodeReflection<
            #path_to_grost::__private::reflection::encode::EncodeField
          >,
          #path_to_grost::__private::flavors::Network,
          #tag,
        >
      {
        type Reflection = #reflection;
        const REFLECTION: &Self::Reflection = &{
          #fn_impl

          encode
        };
      }
    }
  }

  fn generate_partial_encode_field_reflection(
    &self,
    path_to_grost: &syn::Path,
    struct_: &Struct,
    f: &Field,
    wf: &syn::Type,
  ) -> proc_macro2::TokenStream {
    let struct_name = struct_.name();
    let field_name = f.name();
    let reflection_mod_name = struct_.reflection_mod_name();
    let field_reflection = struct_.field_reflection_name();

    let encoded_identifier = quote! {
      <#struct_name>::reflection::<#path_to_grost::__private::flavors::Network>().#field_name().encoded_identifier()
    };
    let encoded_identifier_len = quote! {
      let identifier_len = *<#struct_name>::reflection::<#path_to_grost::__private::flavors::Network>().#field_name().encoded_identifier_len();
    };
    let ty = f.ty();
    let rust_ty = ty.ty();
    let tag = f.tag();

    let reflection = quote! {
      fn(
        &#rust_ty,
        &#path_to_grost::__private::network::Context,
        &mut [::core::primitive::u8],
        &<#rust_ty as #path_to_grost::__private::Selectable<#path_to_grost::__private::flavors::network::Network>>::Selector,
      ) -> ::core::result::Result<::core::primitive::usize, #path_to_grost::__private::flavors::network::EncodeError>
    };
    let encoded_len_fn = quote! {
      const ENCODED_LEN_FN: #reflection_mod_name::#field_reflection<
        #path_to_grost::__private::reflection::encode::EncodeReflection<
          #path_to_grost::__private::reflection::Len<#path_to_grost::__private::reflection::encode::PartialEncodeField>
        >,
        #path_to_grost::__private::flavors::Network,
        #tag,
      > = <#struct_name>::reflection::<#path_to_grost::__private::flavors::Network>().#field_name().partial_encoded_len();
    };

    let fn_impl = if ty.repr().is_optional() {
      let atomic_ty = ty.repr().encode_atomic_ty();
      quote! {
        fn encode(
          f: &::core::option::Option<#atomic_ty>,
          ctx: &#path_to_grost::__private::flavors::network::Context,
          buf: &mut [::core::primitive::u8],
          selector: &<#atomic_ty as #path_to_grost::__private::Selectable<#path_to_grost::__private::flavors::network::Network>>::Selector,
        ) -> ::core::result::Result<::core::primitive::usize, #path_to_grost::__private::flavors::network::EncodeError> {
          #encoded_len_fn

          #encoded_identifier_len

          match f {
            ::core::option::Option::None => ::core::result::Result::Ok(0),
            ::core::option::Option::Some(field) => {
              let buf_len = buf.len();
              let mut offset = 0;
              if offset > buf_len {
                return ::core::result::Result::Err(
                  #path_to_grost::__private::flavors::network::EncodeError::insufficient_buffer(
                    (ENCODED_LEN_FN)(f, ctx, selector),
                    buf_len,
                  ),
                );
              }

              buf[offset..offset + identifier_len].copy_from_slice(&#encoded_identifier);
              offset += identifier_len;

              if offset >= buf_len {
                return ::core::result::Result::Err(
                  #path_to_grost::__private::flavors::network::EncodeError::insufficient_buffer(
                    (ENCODED_LEN_FN)(f, ctx, selector),
                    buf_len,
                  ),
                );
              }

              <#atomic_ty as #path_to_grost::__private::PartialEncode<
                #path_to_grost::__private::flavors::network::Network,
                #wf
              >>::partial_encode_length_delimited(
                field,
                ctx,
                &mut buf[offset..],
                selector,
              )
              .map(|len| offset + len)
              .map_err(|e| e.update((ENCODED_LEN_FN)(f, ctx, selector), buf_len))
            },
          }
        }
      }
    } else {
      quote! {
        fn encode(
          f: &#ty,
          ctx: &#path_to_grost::__private::flavors::network::Context,
          buf: &mut [::core::primitive::u8],
          selector: &<#ty as #path_to_grost::__private::Selectable<#path_to_grost::__private::flavors::network::Network>>::Selector,
        ) -> ::core::result::Result<::core::primitive::usize, #path_to_grost::__private::flavors::network::EncodeError> {
          #encoded_len_fn

          #encoded_identifier_len

          let buf_len = buf.len();
          let mut offset = 0;
          if offset > buf_len {
            return ::core::result::Result::Err(
              #path_to_grost::__private::flavors::network::EncodeError::insufficient_buffer(
                (ENCODED_LEN_FN)(f, ctx, selector),
                buf_len,
              ),
            );
          }

          buf[offset..offset + identifier_len].copy_from_slice(&#encoded_identifier);
          offset += identifier_len;

          if offset >= buf_len {
            return ::core::result::Result::Err(
              #path_to_grost::__private::flavors::network::EncodeError::insufficient_buffer(
                (ENCODED_LEN_FN)(f, ctx, selector),
                buf_len,
              ),
            );
          }

          <#ty as #path_to_grost::__private::PartialEncode<
            #path_to_grost::__private::flavors::network::Network,
            #wf
          >>::partial_encode_length_delimited(
            f,
            ctx,
            &mut buf[offset..],
            selector,
          )
          .map(|len| offset + len)
          .map_err(|e| e.update((ENCODED_LEN_FN)(f, ctx, selector), buf_len))
        }
      }
    };

    quote! {
      impl #path_to_grost::__private::reflection::Reflectable<
        #path_to_grost::__private::flavors::Network,
      > for #reflection_mod_name::#field_reflection<
          #path_to_grost::__private::reflection::encode::EncodeReflection<
            #path_to_grost::__private::reflection::encode::PartialEncodeField
          >,
          #path_to_grost::__private::flavors::Network,
          #tag,
        >
      {
        type Reflection = #reflection;
        const REFLECTION: &Self::Reflection = &{
          #fn_impl

          encode
        };
      }
    }
  }

  fn generate_encoded_field_len_reflection(
    &self,
    path_to_grost: &syn::Path,
    struct_: &Struct,
    f: &Field,
    wf: &syn::Type,
  ) -> proc_macro2::TokenStream {
    let struct_name = struct_.name();
    let field_name = f.name();
    let ty = f.ty();
    let optional = ty.repr().is_optional();
    let atomic_ty = if optional {
      ty.repr().encode_atomic_ty()
    } else {
      ty.ty()
    };
    let impl_ = quote! {
      (*<#struct_name>::reflection::<#path_to_grost::__private::flavors::Network>().#field_name().encoded_identifier_len())
        + <#atomic_ty as #path_to_grost::__private::Encode<
            #path_to_grost::__private::flavors::network::Network,
            #wf,
          >>::encoded_length_delimited_len(
            f,
            ctx,
          )
    };

    let fn_impl = if ty.repr().is_optional() {
      quote! {
        fn encoded_len(
          f: &#ty,
          ctx: &#path_to_grost::__private::flavors::network::Context,
        ) -> ::core::primitive::usize {
          match f {
            ::core::option::Option::Some(f) => {
              #impl_
            },
            ::core::option::Option::None => 0,
          }
        }
      }
    } else {
      quote! {
        fn encoded_len(
          f: &#ty,
          ctx: &#path_to_grost::__private::flavors::network::Context,
        ) -> ::core::primitive::usize {
          #impl_
        }
      }
    };

    let reflection_mod_name = struct_.reflection_mod_name();
    let field_reflection = struct_.field_reflection_name();
    let tag = f.tag();

    let reflection = quote! {
      fn(&#ty, &#path_to_grost::__private::network::Context) -> ::core::primitive::usize
    };

    quote! {
      impl #path_to_grost::__private::reflection::Reflectable<
        #path_to_grost::__private::flavors::Network,
      > for
        #reflection_mod_name::#field_reflection<
          #path_to_grost::__private::reflection::encode::EncodeReflection<
            #path_to_grost::__private::reflection::Len<#path_to_grost::__private::reflection::encode::EncodeField>,
          >,
          #path_to_grost::__private::flavors::Network,
          #tag,
        >
      {
        type Reflection = #reflection;
        const REFLECTION: &Self::Reflection = &{
          #fn_impl

          encoded_len
        };
      }
    }
  }

  fn generate_partial_encoded_field_len_reflection(
    &self,
    path_to_grost: &syn::Path,
    struct_: &Struct,
    f: &Field,
    wf: &syn::Type,
  ) -> proc_macro2::TokenStream {
    let struct_name = struct_.name();
    let field_name = f.name();
    let ty = f.ty();
    let optional = ty.repr().is_optional();
    let atomic_ty = if optional {
      ty.repr().encode_atomic_ty()
    } else {
      ty.ty()
    };
    let impl_ = quote! {
      (*<#struct_name>::reflection::<#path_to_grost::__private::flavors::Network>().#field_name().encoded_identifier_len())
        + <#atomic_ty as #path_to_grost::__private::PartialEncode<
            #path_to_grost::__private::flavors::network::Network,
            #wf,
          >>::partial_encoded_length_delimited_len(
            f,
            ctx,
            selector,
          )
    };
    let fn_impl = if optional {
      quote! {
        fn encoded_len(
          f: &#ty,
          ctx: &#path_to_grost::__private::flavors::network::Context,
          selector: &<#ty as #path_to_grost::__private::Selectable<#path_to_grost::__private::flavors::network::Network>>::Selector,
        ) -> ::core::primitive::usize {
          match f {
            ::core::option::Option::Some(f) => {
              #impl_
            },
            ::core::option::Option::None => 0,
          }
        }
      }
    } else {
      quote! {
        fn encoded_len(
          f: &#ty,
          ctx: &#path_to_grost::__private::flavors::network::Context,
          selector: &<#ty as #path_to_grost::__private::Selectable<#path_to_grost::__private::flavors::network::Network>>::Selector,
        ) -> ::core::primitive::usize {
          #impl_
        }
      }
    };

    let reflection_mod_name = struct_.reflection_mod_name();
    let field_reflection = struct_.field_reflection_name();
    let tag = f.tag();

    let reflection = quote! {
      fn(
        &#ty,
        &#path_to_grost::__private::network::Context,
        &<#ty as #path_to_grost::__private::Selectable<#path_to_grost::__private::flavors::network::Network>>::Selector,
      ) -> ::core::primitive::usize
    };

    quote! {
      impl #path_to_grost::__private::reflection::Reflectable<
        #path_to_grost::__private::flavors::Network,
      > for
        #reflection_mod_name::#field_reflection<
          #path_to_grost::__private::reflection::encode::EncodeReflection<
            #path_to_grost::__private::reflection::Len<#path_to_grost::__private::reflection::encode::PartialEncodeField>,
          >,
          #path_to_grost::__private::flavors::Network,
          #tag,
        >
      {
        type Reflection = #reflection;
        const REFLECTION: &Self::Reflection = &{
          #fn_impl

          encoded_len
        };
      }
    }
  }
}
