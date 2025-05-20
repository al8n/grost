use crate::{Field, Object};

use super::super::*;

use quote::quote;

mod encode;
mod partial_encode;

impl Network {
  pub(crate) fn derive_encode_reflection(
    &self,
    path_to_grost: &syn::Path,
    struct_: &Object,
  ) -> proc_macro2::TokenStream {
    let fns = self.derive_field_encode_reflections(path_to_grost, struct_);

    quote! {
      #fns
    }
  }

  fn derive_field_encode_reflections(
    &self,
    path_to_grost: &syn::Path,
    struct_: &Object,
  ) -> proc_macro2::TokenStream {
    let fns = struct_.fields().iter().map(|f| {
      let wf = f.get_wire_format_or_default(path_to_grost, self);
      let partial_encoded_len_fn =
        self.derive_field_partial_encoded_len_reflection(path_to_grost, struct_, f, &wf);
      let partial_encode_fn =
        self.derive_partial_encode_field_reflection(path_to_grost, struct_, f, &wf);
      let encoded_len_fn = self.derive_field_encoded_len_reflection(path_to_grost, struct_, f, &wf);
      let encode_fn = self.derive_field_encode_reflection(path_to_grost, struct_, f, &wf);
      let partial_encoded_len_ref_fn =
        self.derive_field_partial_decoded_encoded_len_reflection(path_to_grost, struct_, f, &wf);
      let partial_encode_ref_fn =
        self.derive_field_partial_decoded_encode_reflection(path_to_grost, struct_, f, &wf);
      let encoded_len_ref_fn =
        self.derive_field_ref_encoded_len_reflection(path_to_grost, struct_, f, &wf);
      let encode_ref_fn = self.derive_field_ref_encode_reflection(path_to_grost, struct_, f, &wf);

      quote! {
        #partial_encode_fn

        #partial_encoded_len_fn

        #encoded_len_fn

        #encode_fn

        #partial_encoded_len_ref_fn

        #partial_encode_ref_fn

        #encoded_len_ref_fn

        #encode_ref_fn
      }
    });

    quote! {
      fn insufficient_buffer_error<T, W>(
        f: &T,
        ctx: &<#path_to_grost::__private::flavors::Network as #path_to_grost::__private::flavors::Flavor>::Context,
        selector: ::core::option::Option<&<T as #path_to_grost::__private::selection::Selectable<#path_to_grost::__private::flavors::Network, W>>::Selector>,
        buf_len: ::core::primitive::usize,
      ) -> #path_to_grost::__private::flavors::network::EncodeError
      where
        T: #path_to_grost::__private::PartialEncode<
            #path_to_grost::__private::flavors::Network,
            W,
          >
          + #path_to_grost::__private::Encode<
            #path_to_grost::__private::flavors::Network,
            W,
          >
          + #path_to_grost::__private::selection::Selectable<#path_to_grost::__private::flavors::Network, W>
          + ?::core::marker::Sized,
        W: #path_to_grost::__private::flavors::WireFormat<
            #path_to_grost::__private::flavors::Network,
          >,
      {
        match selector {
          ::core::option::Option::Some(selector) => {
            #path_to_grost::__private::flavors::network::EncodeError::insufficient_buffer(
              <T as #path_to_grost::__private::PartialEncode<
                #path_to_grost::__private::flavors::Network,
                W,
              >>::partial_encoded_len(f, ctx, selector),
              buf_len,
            )
          }
          ::core::option::Option::None => {
            #path_to_grost::__private::flavors::network::EncodeError::insufficient_buffer(
              <T as #path_to_grost::__private::Encode<
                #path_to_grost::__private::flavors::Network,
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

  fn derive_field_encode_reflection(
    &self,
    path_to_grost: &syn::Path,
    struct_: &Object,
    f: &Field,
    wf: &syn::Type,
  ) -> proc_macro2::TokenStream {
    let struct_name = struct_.name();
    let field_name = f.name();
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
      const ENCODED_LEN_FN: #field_reflection<
        #path_to_grost::__private::reflection::encode::EncodeReflection<
          #path_to_grost::__private::reflection::Len<#path_to_grost::__private::reflection::encode::EncodeField>
        >,
        #path_to_grost::__private::flavors::Network,
        #tag,
      > = <#struct_name>::reflection::<#path_to_grost::__private::flavors::Network>().#field_name().encoded_len();
    };

    let reflection = quote! {
      fn(&#ty, &#path_to_grost::__private::flavors::network::Context, &mut [::core::primitive::u8]) -> ::core::result::Result<::core::primitive::usize, #path_to_grost::__private::flavors::network::EncodeError>
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
                #path_to_grost::__private::flavors::Network,
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
            #path_to_grost::__private::flavors::Network,
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
      #[allow(clippy::type_complexity)]
      #[automatically_derived]
      impl #path_to_grost::__private::reflection::Reflectable<
        #path_to_grost::__private::flavors::Network,
      > for
        #field_reflection<
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

  fn derive_partial_encode_field_reflection(
    &self,
    path_to_grost: &syn::Path,
    struct_: &Object,
    f: &Field,
    wf: &syn::Type,
  ) -> proc_macro2::TokenStream {
    let struct_name = struct_.name();
    let field_name = f.name();
    let field_reflection = struct_.field_reflection_name();
    let ty = f.ty();
    let rust_ty = ty.ty();
    let tag = f.tag();

    let encoded_identifier = quote! {
      <#struct_name>::reflection::<#path_to_grost::__private::flavors::Network>().#field_name().encoded_identifier()
    };
    let encoded_identifier_len = quote! {
      let identifier_len = *<#struct_name>::reflection::<#path_to_grost::__private::flavors::Network>().#field_name().encoded_identifier_len();
    };
    let encoded_len_fn = quote! {
      const ENCODED_LEN_FN: #field_reflection<
        #path_to_grost::__private::reflection::encode::EncodeReflection<
          #path_to_grost::__private::reflection::Len<#path_to_grost::__private::reflection::encode::PartialEncodeField>
        >,
        #path_to_grost::__private::flavors::Network,
        #tag,
      > = <#struct_name>::reflection::<#path_to_grost::__private::flavors::Network>().#field_name().partial_encoded_len();
    };

    let reflection = quote! {
      fn(
        &#rust_ty,
        &#path_to_grost::__private::flavors::network::Context,
        &mut [::core::primitive::u8],
        &<#rust_ty as #path_to_grost::__private::selection::Selectable<#path_to_grost::__private::flavors::Network, #wf>>::Selector,
      ) -> ::core::result::Result<::core::primitive::usize, #path_to_grost::__private::flavors::network::EncodeError>
    };

    let fn_impl = if ty.repr().is_optional() {
      let atomic_ty = ty.repr().encode_atomic_ty();
      quote! {
        fn encode(
          f: &::core::option::Option<#atomic_ty>,
          ctx: &#path_to_grost::__private::flavors::network::Context,
          buf: &mut [::core::primitive::u8],
          selector: &<#rust_ty as #path_to_grost::__private::selection::Selectable<#path_to_grost::__private::flavors::Network, #wf>>::Selector,
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
                #path_to_grost::__private::flavors::Network,
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
          selector: &<#ty as #path_to_grost::__private::selection::Selectable<#path_to_grost::__private::flavors::Network, #wf>>::Selector,
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
            #path_to_grost::__private::flavors::Network,
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
      #[allow(clippy::type_complexity)]
      #[automatically_derived]
      impl #path_to_grost::__private::reflection::Reflectable<
        #path_to_grost::__private::flavors::Network,
      > for #field_reflection<
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

  fn derive_field_encoded_len_reflection(
    &self,
    path_to_grost: &syn::Path,
    struct_: &Object,
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
            #path_to_grost::__private::flavors::Network,
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
            ::core::option::Option::None => 0,
            ::core::option::Option::Some(f) => {
              #impl_
            },
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

    let field_reflection = struct_.field_reflection_name();
    let tag = f.tag();

    let reflection = quote! {
      fn(&#ty, &#path_to_grost::__private::flavors::network::Context) -> ::core::primitive::usize
    };

    quote! {
      #[allow(clippy::type_complexity)]
      #[automatically_derived]
      impl #path_to_grost::__private::reflection::Reflectable<
        #path_to_grost::__private::flavors::Network,
      > for
        #field_reflection<
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

  fn derive_field_partial_encoded_len_reflection(
    &self,
    path_to_grost: &syn::Path,
    struct_: &Object,
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
            #path_to_grost::__private::flavors::Network,
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
          selector: &<#ty as #path_to_grost::__private::selection::Selectable<#path_to_grost::__private::flavors::Network, #wf>>::Selector,
        ) -> ::core::primitive::usize {
          match f {
            ::core::option::Option::None => 0,
            ::core::option::Option::Some(f) => {
              #impl_
            },
          }
        }
      }
    } else {
      quote! {
        fn encoded_len(
          f: &#ty,
          ctx: &#path_to_grost::__private::flavors::network::Context,
          selector: &<#ty as #path_to_grost::__private::selection::Selectable<#path_to_grost::__private::flavors::Network, #wf>>::Selector,
        ) -> ::core::primitive::usize {
          #impl_
        }
      }
    };

    let field_reflection = struct_.field_reflection_name();
    let tag = f.tag();

    let reflection = quote! {
      fn(
        &#ty,
        &#path_to_grost::__private::flavors::network::Context,
        &<#ty as #path_to_grost::__private::selection::Selectable<#path_to_grost::__private::flavors::Network, #wf>>::Selector,
      ) -> ::core::primitive::usize
    };

    quote! {
      #[allow(clippy::type_complexity)]
      #[automatically_derived]
      impl #path_to_grost::__private::reflection::Reflectable<
        #path_to_grost::__private::flavors::Network,
      > for
        #field_reflection<
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

  fn derive_field_partial_decoded_encode_reflection(
    &self,
    path_to_grost: &syn::Path,
    struct_: &Object,
    f: &Field,
    wf: &syn::Type,
  ) -> proc_macro2::TokenStream {
    let struct_name = struct_.name();
    let field_name = f.name();
    let field_reflection = struct_.field_reflection_name();
    let tag = f.tag();
    let ty = f.ty();
    let optional = ty.repr().is_optional();
    let ret = quote! {
      ::core::result::Result<::core::primitive::usize, #path_to_grost::__private::flavors::network::EncodeError>
    };

    let encoded_identifier = quote! {
      <#struct_name>::reflection::<#path_to_grost::__private::flavors::Network>().#field_name().encoded_identifier()
    };
    let encoded_identifier_len = quote! {
      let identifier_len = *<#struct_name>::reflection::<#path_to_grost::__private::flavors::Network>().#field_name().encoded_identifier_len();
    };
    let encoded_len_fn = quote! {
      const ENCODED_LEN_FN: #field_reflection<
        #path_to_grost::__private::reflection::encode::EncodeReflection<
          #path_to_grost::__private::reflection::Len<#path_to_grost::__private::reflection::encode::PartialEncodeRefField>
        >,
        #path_to_grost::__private::flavors::Network,
        #tag,
      > = <#struct_name>::reflection::<#path_to_grost::__private::flavors::Network>().#field_name().partial_encoded_ref_len();
    };

    let fn_impl = if optional {
      quote! {
        fn encode(
          field: &::core::option::Option<<#ty as #path_to_grost::__private::convert::State<
            #path_to_grost::__private::convert::Encoded<
              '_,
              #path_to_grost::__private::flavors::Network,
              #wf,
            >
          >>::Output>,
          ctx: &#path_to_grost::__private::flavors::network::Context,
          buf: &mut [::core::primitive::u8],
          selector: &<#ty as #path_to_grost::__private::selection::Selectable<#path_to_grost::__private::flavors::Network, #wf>>::Selector,
        ) -> #ret {
          match field {
            ::core::option::Option::None => ::core::result::Result::Ok(0),
            ::core::option::Option::Some(f) => {
              #encoded_identifier_len

              #encoded_len_fn

              let buf_len = buf.len();
              let mut offset = 0;
              if offset > buf_len {
                return ::core::result::Result::Err(
                  #path_to_grost::__private::flavors::network::EncodeError::insufficient_buffer(
                    (ENCODED_LEN_FN)(field, ctx, selector),
                    buf_len,
                  ),
                );
              }

              buf[offset..offset + identifier_len].copy_from_slice(&#encoded_identifier);
              offset += identifier_len;

              if offset >= buf_len {
                return ::core::result::Result::Err(
                  #path_to_grost::__private::flavors::network::EncodeError::insufficient_buffer(
                    (ENCODED_LEN_FN)(field, ctx, selector),
                    buf_len,
                  ),
                );
              }

              <<#ty as #path_to_grost::__private::convert::State<
                #path_to_grost::__private::convert::Encoded<
                  '_,
                  #path_to_grost::__private::flavors::Network,
                  #wf,
                >
              >>::Output as #path_to_grost::__private::PartialEncode<
                #path_to_grost::__private::flavors::Network,
                #wf,
              >>::partial_encode_length_delimited(
                f,
                ctx,
                buf,
                selector,
              )
              .map(|len| offset + len)
              .map_err(|e| e.update((ENCODED_LEN_FN)(field, ctx, selector), buf_len))
            },
          }
        }
      }
    } else {
      quote! {
        fn encode(
          f: &<#ty as #path_to_grost::__private::convert::State<
            #path_to_grost::__private::convert::Encoded<
              '_,
              #path_to_grost::__private::flavors::Network,
              #wf,
            >
          >>::Output,
          ctx: &#path_to_grost::__private::flavors::network::Context,
          buf: &mut [::core::primitive::u8],
          selector: &<#ty as #path_to_grost::__private::selection::Selectable<#path_to_grost::__private::flavors::Network, #wf>>::Selector,
        ) -> #ret {
          #encoded_identifier_len

          #encoded_len_fn

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

          <<#ty as #path_to_grost::__private::convert::State<
            #path_to_grost::__private::convert::Encoded<
              '_,
              #path_to_grost::__private::flavors::Network,
              #wf,
            >
          >>::Output as #path_to_grost::__private::PartialEncode<
            #path_to_grost::__private::flavors::Network,
            #wf,
          >>::partial_encode_length_delimited(
            f,
            ctx,
            buf,
            selector,
          )
          .map(|len| offset + len)
          .map_err(|e| e.update((ENCODED_LEN_FN)(f, ctx, selector), buf_len))
        }
      }
    };

    let field_reflection = struct_.field_reflection_name();
    let tag = f.tag();
    let wf = f.get_wire_format_or_default(path_to_grost, self);

    let reflection = if optional {
      quote! {
        fn(
          &::core::option::Option<<#ty as #path_to_grost::__private::convert::State<
            #path_to_grost::__private::convert::Encoded<
              '_,
              #path_to_grost::__private::flavors::Network,
              #wf,
            >
          >>::Output>,
          &#path_to_grost::__private::flavors::network::Context,
          &mut [::core::primitive::u8],
          &<#ty as #path_to_grost::__private::selection::Selectable<#path_to_grost::__private::flavors::Network, #wf>>::Selector,
        ) -> ::core::result::Result<::core::primitive::usize, #path_to_grost::__private::flavors::network::EncodeError>
      }
    } else {
      quote! {
        fn(
          &<#ty as #path_to_grost::__private::convert::State<
            #path_to_grost::__private::convert::Encoded<
              '_,
              #path_to_grost::__private::flavors::Network,
              #wf,
            >
          >>::Output,
          &#path_to_grost::__private::flavors::network::Context,
          &mut [::core::primitive::u8],
          &<#ty as #path_to_grost::__private::selection::Selectable<#path_to_grost::__private::flavors::Network, #wf>>::Selector,
        ) -> ::core::result::Result<::core::primitive::usize, #path_to_grost::__private::flavors::network::EncodeError>
      }
    };

    quote! {
      #[allow(clippy::type_complexity)]
      #[automatically_derived]
      impl #path_to_grost::__private::reflection::Reflectable<
        #path_to_grost::__private::flavors::Network,
      > for
        #field_reflection<
          #path_to_grost::__private::reflection::encode::EncodeReflection<
            #path_to_grost::__private::reflection::encode::PartialEncodeRefField,
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

  fn derive_field_partial_decoded_encoded_len_reflection(
    &self,
    path_to_grost: &syn::Path,
    struct_: &Object,
    f: &Field,
    wf: &syn::Type,
  ) -> proc_macro2::TokenStream {
    let struct_name = struct_.name();
    let field_name = f.name();
    let ty = f.ty();
    let optional = ty.repr().is_optional();

    let fn_impl = if optional {
      quote! {
        fn encoded_len(
          f: &::core::option::Option<<#ty as #path_to_grost::__private::convert::State<
            #path_to_grost::__private::convert::Encoded<
              '_,
              #path_to_grost::__private::flavors::Network,
              #wf,
            >
          >>::Output>,
          ctx: &#path_to_grost::__private::flavors::network::Context,
          selector: &<#ty as #path_to_grost::__private::selection::Selectable<#path_to_grost::__private::flavors::Network, #wf>>::Selector,
        ) -> ::core::primitive::usize {
          match f {
            ::core::option::Option::None => 0,
            ::core::option::Option::Some(f) => {
              (*<#struct_name>::reflection::<#path_to_grost::__private::flavors::Network>().#field_name().encoded_identifier_len())
                + <<#ty as #path_to_grost::__private::convert::State<
                    #path_to_grost::__private::convert::Encoded<
                      '_,
                      #path_to_grost::__private::flavors::Network,
                      #wf,
                    >
                  >>::Output as #path_to_grost::__private::PartialEncode<
                    #path_to_grost::__private::flavors::Network,
                    #wf,
                  >>::partial_encoded_length_delimited_len(
                    f,
                    ctx,
                    selector,
                  )
            },
          }
        }
      }
    } else {
      quote! {
        fn encoded_len(
          f: &<#ty as #path_to_grost::__private::convert::State<
            #path_to_grost::__private::convert::Encoded<
              '_,
              #path_to_grost::__private::flavors::Network,
              #wf,
            >
          >>::Output,
          ctx: &#path_to_grost::__private::flavors::network::Context,
          selector: &<#ty as #path_to_grost::__private::selection::Selectable<#path_to_grost::__private::flavors::Network, #wf>>::Selector,
        ) -> ::core::primitive::usize {
          (*<#struct_name>::reflection::<#path_to_grost::__private::flavors::Network>().#field_name().encoded_identifier_len())
            + <<#ty as #path_to_grost::__private::convert::State<
                #path_to_grost::__private::convert::Encoded<
                  '_,
                  #path_to_grost::__private::flavors::Network,
                  #wf,
                >
              >>::Output as #path_to_grost::__private::PartialEncode<
                #path_to_grost::__private::flavors::Network,
                #wf,
              >>::partial_encoded_length_delimited_len(
                f,
                ctx,
                selector,
              )
        }
      }
    };

    let field_reflection = struct_.field_reflection_name();
    let tag = f.tag();
    let wf = f.get_wire_format_or_default(path_to_grost, self);

    let reflection = if optional {
      quote! {
        fn(
          &::core::option::Option<<#ty as #path_to_grost::__private::convert::State<
            #path_to_grost::__private::convert::Encoded<
              '_,
              #path_to_grost::__private::flavors::Network,
              #wf,
            >
          >>::Output>,
          &#path_to_grost::__private::flavors::network::Context,
          &<#ty as #path_to_grost::__private::selection::Selectable<#path_to_grost::__private::flavors::Network, #wf>>::Selector,
        ) -> ::core::primitive::usize
      }
    } else {
      quote! {
        fn(
          &<#ty as #path_to_grost::__private::convert::State<
            #path_to_grost::__private::convert::Encoded<
              '_,
              #path_to_grost::__private::flavors::Network,
              #wf,
            >
          >>::Output,
          &#path_to_grost::__private::flavors::network::Context,
          &<#ty as #path_to_grost::__private::selection::Selectable<#path_to_grost::__private::flavors::Network, #wf>>::Selector,
        ) -> ::core::primitive::usize
      }
    };

    quote! {
      #[allow(clippy::type_complexity)]
      #[automatically_derived]
      impl #path_to_grost::__private::reflection::Reflectable<
        #path_to_grost::__private::flavors::Network,
      > for
        #field_reflection<
          #path_to_grost::__private::reflection::encode::EncodeReflection<
            #path_to_grost::__private::reflection::Len<#path_to_grost::__private::reflection::encode::PartialEncodeRefField>,
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

  fn derive_field_ref_encode_reflection(
    &self,
    path_to_grost: &syn::Path,
    struct_: &Object,
    f: &Field,
    wf: &syn::Type,
  ) -> proc_macro2::TokenStream {
    let struct_name = struct_.name();
    let field_name = f.name();
    let field_reflection = struct_.field_reflection_name();
    let tag = f.tag();
    let ty = f.ty();
    let optional = ty.repr().is_optional();
    let ret = quote! {
      ::core::result::Result<::core::primitive::usize, #path_to_grost::__private::flavors::network::EncodeError>
    };

    let encoded_identifier = quote! {
      <#struct_name>::reflection::<#path_to_grost::__private::flavors::Network>().#field_name().encoded_identifier()
    };
    let encoded_identifier_len = quote! {
      let identifier_len = *<#struct_name>::reflection::<#path_to_grost::__private::flavors::Network>().#field_name().encoded_identifier_len();
    };
    let encoded_len_fn = quote! {
      const ENCODED_LEN_FN: #field_reflection<
        #path_to_grost::__private::reflection::encode::EncodeReflection<
          #path_to_grost::__private::reflection::Len<#path_to_grost::__private::reflection::encode::EncodeRefField>
        >,
        #path_to_grost::__private::flavors::Network,
        #tag,
      > = <#struct_name>::reflection::<#path_to_grost::__private::flavors::Network>().#field_name().encoded_ref_len();
    };

    let fn_impl = if optional {
      quote! {
        fn encode(
          field: &::core::option::Option<<#ty as #path_to_grost::__private::convert::State<
            #path_to_grost::__private::convert::Encoded<
              '_,
              #path_to_grost::__private::flavors::Network,
              #wf,
            >
          >>::Output>,
          ctx: &#path_to_grost::__private::flavors::network::Context,
          buf: &mut [::core::primitive::u8],
        ) -> #ret {
          match field {
            ::core::option::Option::Some(f) => {
              #encoded_identifier_len

              #encoded_len_fn

              let buf_len = buf.len();
              let mut offset = 0;
              if offset > buf_len {
                return ::core::result::Result::Err(
                  #path_to_grost::__private::flavors::network::EncodeError::insufficient_buffer(
                    (ENCODED_LEN_FN)(field, ctx),
                    buf_len,
                  ),
                );
              }

              buf[offset..offset + identifier_len].copy_from_slice(&#encoded_identifier);
              offset += identifier_len;

              if offset >= buf_len {
                return ::core::result::Result::Err(
                  #path_to_grost::__private::flavors::network::EncodeError::insufficient_buffer(
                    (ENCODED_LEN_FN)(field, ctx),
                    buf_len,
                  ),
                );
              }

              <<#ty as #path_to_grost::__private::convert::State<
                #path_to_grost::__private::convert::Encoded<
                  '_,
                  #path_to_grost::__private::flavors::Network,
                  #wf,
                >
              >>::Output as #path_to_grost::__private::Encode<
                #path_to_grost::__private::flavors::Network,
                #wf,
              >>::encode_length_delimited(
                f,
                ctx,
                buf,
              )
              .map(|len| offset + len)
              .map_err(|e| e.update((ENCODED_LEN_FN)(field, ctx), buf_len))
            },
            ::core::option::Option::None => ::core::result::Result::Ok(0),
          }
        }
      }
    } else {
      quote! {
        fn encode(
          f: &<#ty as #path_to_grost::__private::convert::State<
            #path_to_grost::__private::convert::Encoded<
              '_,
              #path_to_grost::__private::flavors::Network,
              #wf,
            >
          >>::Output,
          ctx: &#path_to_grost::__private::flavors::network::Context,
          buf: &mut [::core::primitive::u8],
        ) -> #ret {
          #encoded_identifier_len

          #encoded_len_fn

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

          <<#ty as #path_to_grost::__private::convert::State<
            #path_to_grost::__private::convert::Encoded<
              '_,
              #path_to_grost::__private::flavors::Network,
              #wf,
            >
          >>::Output as #path_to_grost::__private::Encode<
            #path_to_grost::__private::flavors::Network,
            #wf,
          >>::encode_length_delimited(
            f,
            ctx,
            buf,
          )
          .map(|len| offset + len)
          .map_err(|e| e.update((ENCODED_LEN_FN)(f, ctx), buf_len))
        }
      }
    };

    let field_reflection = struct_.field_reflection_name();
    let tag = f.tag();
    let wf = f.get_wire_format_or_default(path_to_grost, self);

    let reflection = if optional {
      quote! {
        fn(
          &::core::option::Option<<#ty as #path_to_grost::__private::convert::State<
            #path_to_grost::__private::convert::Encoded<
              '_,
              #path_to_grost::__private::flavors::Network,
              #wf,
            >
          >>::Output>,
          &#path_to_grost::__private::flavors::network::Context,
          &mut [::core::primitive::u8],
        ) -> ::core::result::Result<::core::primitive::usize, #path_to_grost::__private::flavors::network::EncodeError>
      }
    } else {
      quote! {
        fn(
          &<#ty as #path_to_grost::__private::convert::State<
            #path_to_grost::__private::convert::Encoded<
              '_,
              #path_to_grost::__private::flavors::Network,
              #wf,
            >
          >>::Output,
          &#path_to_grost::__private::flavors::network::Context,
          &mut [::core::primitive::u8],
        ) -> ::core::result::Result<::core::primitive::usize, #path_to_grost::__private::flavors::network::EncodeError>
      }
    };

    quote! {
      #[allow(clippy::type_complexity)]
      #[automatically_derived]
      impl #path_to_grost::__private::reflection::Reflectable<
        #path_to_grost::__private::flavors::Network,
      > for
        #field_reflection<
          #path_to_grost::__private::reflection::encode::EncodeReflection<
            #path_to_grost::__private::reflection::encode::EncodeRefField,
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

  fn derive_field_ref_encoded_len_reflection(
    &self,
    path_to_grost: &syn::Path,
    struct_: &Object,
    f: &Field,
    wf: &syn::Type,
  ) -> proc_macro2::TokenStream {
    let struct_name = struct_.name();
    let field_name = f.name();
    let ty = f.ty();
    let optional = ty.repr().is_optional();

    let fn_impl = if optional {
      quote! {
        fn encoded_len(
          f: &::core::option::Option<<#ty as #path_to_grost::__private::convert::State<
            #path_to_grost::__private::convert::Encoded<
              '_,
              #path_to_grost::__private::flavors::Network,
              #wf,
            >
          >>::Output>,
          ctx: &#path_to_grost::__private::flavors::network::Context,
        ) -> ::core::primitive::usize {
          match f {
            ::core::option::Option::None => 0,
            ::core::option::Option::Some(f) => {
              (*<#struct_name>::reflection::<#path_to_grost::__private::flavors::Network>().#field_name().encoded_identifier_len())
                + <<#ty as #path_to_grost::__private::convert::State<
                    #path_to_grost::__private::convert::Encoded<
                      '_,
                      #path_to_grost::__private::flavors::Network,
                      #wf,
                    >
                  >>::Output as #path_to_grost::__private::Encode<
                    #path_to_grost::__private::flavors::Network,
                    #wf,
                  >>::encoded_length_delimited_len(
                    f,
                    ctx,
                  )
            },
          }
        }
      }
    } else {
      quote! {
        fn encoded_len(
          f: &<#ty as #path_to_grost::__private::convert::State<
            #path_to_grost::__private::convert::Encoded<
              '_,
              #path_to_grost::__private::flavors::Network,
              #wf,
            >
          >>::Output,
          ctx: &#path_to_grost::__private::flavors::network::Context,
        ) -> ::core::primitive::usize {
          (*<#struct_name>::reflection::<#path_to_grost::__private::flavors::Network>().#field_name().encoded_identifier_len())
            + <<#ty as #path_to_grost::__private::convert::State<
                #path_to_grost::__private::convert::Encoded<
                  '_,
                  #path_to_grost::__private::flavors::Network,
                  #wf,
                >
              >>::Output as #path_to_grost::__private::Encode<
                #path_to_grost::__private::flavors::Network,
                #wf,
              >>::encoded_length_delimited_len(
                f,
                ctx,
              )
        }
      }
    };

    let field_reflection = struct_.field_reflection_name();
    let tag = f.tag();
    let wf = f.get_wire_format_or_default(path_to_grost, self);

    let reflection = if optional {
      quote! {
        fn(
          &::core::option::Option<<#ty as #path_to_grost::__private::convert::State<
            #path_to_grost::__private::convert::Encoded<
              '_,
              #path_to_grost::__private::flavors::Network,
              #wf,
            >
          >>::Output>,
          &#path_to_grost::__private::flavors::network::Context,
        ) -> ::core::primitive::usize
      }
    } else {
      quote! {
        fn(
          &<#ty as #path_to_grost::__private::convert::State<
            #path_to_grost::__private::convert::Encoded<
              '_,
              #path_to_grost::__private::flavors::Network,
              #wf,
            >
          >>::Output,
          &#path_to_grost::__private::flavors::network::Context,
        ) -> ::core::primitive::usize
      }
    };

    quote! {
      #[allow(clippy::type_complexity)]
      #[automatically_derived]
      impl #path_to_grost::__private::reflection::Reflectable<
        #path_to_grost::__private::flavors::Network,
      > for
        #field_reflection<
          #path_to_grost::__private::reflection::encode::EncodeReflection<
            #path_to_grost::__private::reflection::Len<#path_to_grost::__private::reflection::encode::EncodeRefField>,
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
