use quote::{format_ident, quote};

use crate::FlavorGeneratorExt;

use super::{Enum, FlavorGenerator, Struct};

mod codec;
mod reflection;

#[derive(Clone)]
pub struct Network {
  ty: syn::Type,
  name: &'static str,
}

impl Network {
  /// Returns a new `Network` flavor
  pub fn new(path_to_grost: &syn::Path) -> Self {
    let ty = syn::parse_quote!(#path_to_grost::__private::flavors::Network);
    Self {
      ty,
      name: "Network",
    }
  }
}

impl FlavorGenerator for Network {
  fn ty(&self) -> &syn::Type {
    &self.ty
  }

  fn set_ty(&mut self, ty: syn::Type) {
    self.ty = ty;
  }

  fn name(&self) -> &'static str {
    self.name
  }

  fn generate_field_identifier(
    &self,
    path_to_grost: &syn::Path,
    field: &crate::Field,
  ) -> proc_macro2::TokenStream {
    let tag = field.tag();

    match field.get_wire_format(self) {
      Some(wf) => quote! {
        #path_to_grost::__private::flavors::network::Identifier::new(
          <#wf as #path_to_grost::__private::flavors::WireFormat<#path_to_grost::__private::flavors::Network>>::WIRE_TYPE,
          #path_to_grost::__private::flavors::network::Tag::new(#tag),
        )
      },
      None => {
        let ty = field.ty().ty();
        quote! {
          #path_to_grost::__private::flavors::network::Identifier::new(
            <
              <#ty
                as #path_to_grost::__private::flavors::DefaultWireFormat<#path_to_grost::__private::flavors::Network>
              >::Format as #path_to_grost::__private::flavors::WireFormat<#path_to_grost::__private::flavors::Network>
            >::WIRE_TYPE,
            #path_to_grost::__private::flavors::network::Tag::new(#tag),
          )
        }
      }
    }
  }

  fn generate_struct_codec(
    &self,
    path_to_grost: &syn::Path,
    struct_: &Struct,
  ) -> proc_macro2::TokenStream {
    let struct_name = struct_.name();
    let partial_encode = self.generate_partial_encode(path_to_grost, struct_);
    let encode = self.generate_encode(path_to_grost, struct_);
    let index = self.generate_struct_index(path_to_grost, struct_);
    let fns = self.generate_field_encode_fns(path_to_grost, struct_);

    quote! {
      #index

      #fns

      #[automatically_derived]
      impl #path_to_grost::__private::flavors::DefaultWireFormat<#path_to_grost::__private::flavors::network::Network> for #struct_name {
        type Format = #path_to_grost::__private::flavors::network::LengthDelimited;
      }

      #encode

      #partial_encode
    }
  }

  fn generate_struct_reflection_impl(
    &self,
    path_to_grost: &syn::Path,
    struct_: &Struct,
  ) -> proc_macro2::TokenStream {
    let struct_name = struct_.name();
    let reflection_name = struct_.reflection_name();
    let name_str = struct_name.name_str();
    let schema_name = struct_.schema_name();

    let field_reflections = self.generate_field_reflections(path_to_grost, struct_);
    let field_tag_reflections = self.generate_field_tag_reflections(path_to_grost, struct_);
    let field_encoded_tag_reflections =
      self.generate_field_encoded_tag_reflections(path_to_grost, struct_);
    let field_encoded_tag_len_reflections =
      self.generate_field_encoded_tag_len_reflections(path_to_grost, struct_);
    let field_identifier_reflections =
      self.generate_field_identifier_reflections(path_to_grost, struct_);
    let field_encoded_identifier_len_reflections =
      self.generate_field_encoded_identifier_len_reflections(path_to_grost, struct_);
    let field_encoded_identifier_reflections =
      self.generate_field_encoded_identifier_reflections(path_to_grost, struct_);
    let field_wire_type_reflections =
      self.generate_field_wire_type_reflections(path_to_grost, struct_);

    let field_reflections_names = struct_.fields().iter().map(|f| {
      let field_reflection = format_ident!("{}", struct_.field_reflection_name());
      let tag = f.tag();
      quote! {
        <
          #field_reflection<
            #path_to_grost::__private::reflection::FieldReflection<#path_to_grost::__private::flavors::Network>,
            #path_to_grost::__private::flavors::Network,
            #tag,
          > as #path_to_grost::__private::reflection::Reflectable<
            #path_to_grost::__private::flavors::Network,
          >
        >::REFLECTION
      }
    });

    quote! {
      const _: () = {
        #(#field_reflections)*

        #(#field_wire_type_reflections)*

        #(#field_tag_reflections)*

        #(#field_encoded_tag_reflections)*

        #(#field_encoded_tag_len_reflections)*

        #(#field_identifier_reflections)*

        #(#field_encoded_identifier_reflections)*

        #(#field_encoded_identifier_len_reflections)*

        impl #path_to_grost::__private::reflection::Reflectable<
          #path_to_grost::__private::flavors::network::Network,
        > for #struct_name {
          type Reflection = #path_to_grost::__private::reflection::StructReflection<#path_to_grost::__private::flavors::network::Network>;
          const REFLECTION: &Self::Reflection = &#path_to_grost::__private::reflection::StructReflectionBuilder::<#path_to_grost::__private::flavors::network::Network> {
            name: #name_str,
            schema_name: #schema_name,
            fields: &[
              #(#field_reflections_names),*
            ],
          }.build();
        }

        impl #path_to_grost::__private::reflection::Reflectable<
          #path_to_grost::__private::flavors::network::Network,
        > for #reflection_name<#path_to_grost::__private::reflection::StructReflection<#path_to_grost::__private::flavors::network::Network>, #path_to_grost::__private::flavors::network::Network> {
          type Reflection = #path_to_grost::__private::reflection::StructReflection<#path_to_grost::__private::flavors::network::Network>;
          const REFLECTION: &Self::Reflection = <
            #struct_name as
              #path_to_grost::__private::reflection::Reflectable<
                #path_to_grost::__private::flavors::network::Network,
              >
            >::REFLECTION;
        }
      };
    }
  }

  fn generate_selection_codec(
    &self,
    path_to_grost: &syn::Path,
    struct_: &Struct,
  ) -> proc_macro2::TokenStream {
    let flavor_ty = self.ty();
    let name = struct_.selector_name();

    let encode_selected_len = struct_.fields().iter().map(|f| {
      let field_name = f.name();
      let ty = f.ty();
      let field_reflection_name = self.field_reflection_name(f.name().name_str());
      let struct_name = struct_.name();

      if ty.primitive_selection_type() {
        quote! {
          if self.#field_name {
            len += #path_to_grost::__private::varing::encoded_u32_varint_len(#struct_name::#field_reflection_name.identifier().tag().get());
          }
        }
      } else {
        quote! {
          if !self.#field_name.is_empty() {
            // the length of the identifier
            len += #path_to_grost::__private::varing::encoded_u32_varint_len(#struct_name::#field_reflection_name.identifier().tag().get());
            let encoded_len = self.#field_name.encoded_len();
            // the length of the length prefix
            len += #path_to_grost::__private::varing::encoded_u32_varint_len(encoded_len as ::core::primitive::u32);
            // the length of the value
            len += encoded_len;
          }
        }
      }
    });

    let encode_unselected_len = struct_.fields().iter().map(|f| {
      let field_name = f.name();
      let ty = f.ty();
      let field_reflection_name = self.field_reflection_name(f.name().name_str());
      let struct_name = struct_.name();

      if ty.primitive_selection_type() {
        quote! {
          if !self.#field_name {
            len += #path_to_grost::__private::varing::encoded_u32_varint_len(#struct_name::#field_reflection_name.identifier().tag().get());
          }
        }
      } else {
        quote! {
          if !self.#field_name.is_empty() {
            // the length of the identifier
            len += #path_to_grost::__private::varing::encoded_u32_varint_len(#struct_name::#field_reflection_name.identifier().tag().get());
            let encoded_len = self.#field_name.encoded_len();
            // the length of the length prefix
            len += #path_to_grost::__private::varing::encoded_u32_varint_len(encoded_len as ::core::primitive::u32);
            // the length of the value
            len += encoded_len;
          }
        }
      }
    });

    let encode_selected = struct_.fields().iter().map(|f| {
      let field_name = f.name();
      let ty = f.ty();
      let field_reflection_name = self.field_reflection_name(f.name().name_str());
      let struct_name = struct_.name();

      if ty.primitive_selection_type() {
        quote! {
          if self.#field_name {
            if offset > buf_len {
              return ::core::result::Result::Err(#path_to_grost::__private::EncodeError::insufficient_buffer(self.encoded_len(), buf_len));
            }

            offset += #path_to_grost::__private::varing::encode_u32_varint_to(#struct_name::#field_reflection_name.identifier().tag().get(), &mut buf[offset..])
              .map_err(|e| {
                #path_to_grost::__private::EncodeError::from_varint_error(e)
                  .update(self.encoded_len(), buf_len)
              })?;
          }
        }
      } else {
        quote! {
          if !self.#field_name.is_empty() {
            if offset > buf_len {
              return ::core::result::Result::Err(#path_to_grost::__private::EncodeError::insufficient_buffer(self.encoded_len(), buf_len));
            }

            // encode the identifier
            offset += #path_to_grost::__private::varing::encode_u32_varint_to(#struct_name::#field_reflection_name.identifier().tag().get(), &mut buf[offset..])
              .map_err(|e| {
                #path_to_grost::__private::EncodeError::from_varint_error(e)
                  .update(self.encoded_len(), buf_len)
              })?;
            
            if offset > buf_len {
              return ::core::result::Result::Err(#path_to_grost::__private::EncodeError::insufficient_buffer(self.encoded_len(), buf_len));
            }

            // encode the length prefix
            let encoded_len = self.#field_name.encoded_len();
            offset += #path_to_grost::__private::varing::encode_u32_varint_to(encoded_len as ::core::primitive::u32, &mut buf[offset..])
              .map_err(|e| {
                #path_to_grost::__private::EncodeError::from_varint_error(e)
                  .update(self.encoded_len(), buf_len)
              })?;

            // encode the value
            if offset > buf_len {
              return ::core::result::Result::Err(#path_to_grost::__private::EncodeError::insufficient_buffer(self.encoded_len(), buf_len));
            }
            offset += self.#field_name.encode(&mut buf[offset..])
              .map_err(|e| {
                e.update(self.encoded_len(), buf_len)
              })?;
          }
        }
      }
    });

    let encode_unselected = struct_.fields().iter().map(|f| {
      let field_name = f.name();
      let ty = f.ty();
      let field_reflection_name = self.field_reflection_name(f.name().name_str());
      let struct_name = struct_.name();

      if ty.primitive_selection_type() {
        quote! {
          if !self.#field_name {
            if offset > buf_len {
              return ::core::result::Result::Err(#path_to_grost::__private::EncodeError::insufficient_buffer(self.encoded_len(), buf_len));
            }

            offset += #path_to_grost::__private::varing::encode_u32_varint_to(#struct_name::#field_reflection_name.identifier().tag().get(), &mut buf[offset..])
              .map_err(|e| {
                #path_to_grost::__private::EncodeError::from_varint_error(e)
                  .update(self.encoded_len(), buf_len)
              })?;
          }
        }
      } else {
        quote! {
          if !self.#field_name.is_empty() {
            if offset > buf_len {
              return ::core::result::Result::Err(#path_to_grost::__private::EncodeError::insufficient_buffer(self.encoded_len(), buf_len));
            }

            // encode the identifier
            offset += #path_to_grost::__private::varing::encode_u32_varint_to(#struct_name::#field_reflection_name.identifier().tag().get(), &mut buf[offset..])
              .map_err(|e| {
                #path_to_grost::__private::EncodeError::from_varint_error(e)
                  .update(self.encoded_len(), buf_len)
              })?;
            
            if offset > buf_len {
              return ::core::result::Result::Err(#path_to_grost::__private::EncodeError::insufficient_buffer(self.encoded_len(), buf_len));
            }

            // encode the length prefix
            let encoded_len = self.#field_name.encoded_len();
            offset += #path_to_grost::__private::varing::encode_u32_varint_to(encoded_len as ::core::primitive::u32, &mut buf[offset..])
              .map_err(|e| {
                #path_to_grost::__private::EncodeError::from_varint_error(e)
                  .update(self.encoded_len(), buf_len)
              })?;

            // encode the value
            if offset > buf_len {
              return ::core::result::Result::Err(#path_to_grost::__private::EncodeError::insufficient_buffer(self.encoded_len(), buf_len));
            }
            offset += self.#field_name.encode(&mut buf[offset..])
              .map_err(|e| {
                e.update(self.encoded_len(), buf_len)
              })?;
          }
        }
      }
    });

    let struct_name = struct_.name();
    let iter_name = struct_.selector_iter_name();

    let field_tag_consts = struct_.fields().iter().map(|f| {
      let tag = f.tag();
      let encoded_len = self.encoded_len_field_tag_const_name(f.name().name_str());
      let encoded = self.encode_field_tag_const_name(f.name().name_str());
      quote! {
        const #encoded_len: ::core::primitive::usize = #path_to_grost::__private::varing::encoded_u32_varint_len(#tag);
        const #encoded: &[::core::primitive::u8] = #path_to_grost::__private::varing::encode_u32_varint(#tag).as_slice();
      }
    });
    quote! {
      const _: () = {
        #[automatically_derived]
        impl<'a, const N: ::core::primitive::bool> ::core::iter::Iterator for #iter_name<'a, #path_to_grost::__private::flavors::Network, N> {
          type Item = &'static #path_to_grost::__private::reflection::FieldReflection<#path_to_grost::__private::flavors::Network>;

          fn next(&mut self) -> ::core::option::Option<Self::Item> {
            loop {
              if self.yielded >= self.num {
                return ::core::option::Option::None;
              }

              match self.index {
                ::core::option::Option::Some(index) => {
                  match self.selector[(index, N)] {
                    ::core::option::Option::Some(reflection) => {
                      self.index = index.next();
                      self.yielded += 1;
                      return ::core::option::Option::Some(reflection);
                    }
                    ::core::option::Option::None => {
                      self.index = index.next();
                    }
                  }
                },
                ::core::option::Option::None => return ::core::option::Option::None,
              }
            }
          }

          fn size_hint(&self) -> (::core::primitive::usize, ::core::option::Option<::core::primitive::usize>) {
            let remaining = self.remaining();
            (remaining, ::core::option::Option::Some(remaining))
          }
        }

        #[automatically_derived]
        impl<'a, const N: ::core::primitive::bool> ::core::iter::FusedIterator for #iter_name<'a, #path_to_grost::__private::flavors::Network, N> {}

        #[automatically_derived]
        impl<'a, const N: ::core::primitive::bool> ::core::iter::ExactSizeIterator for #iter_name<'a, #path_to_grost::__private::flavors::Network, N> {
          #[inline]
          fn len(&self) -> ::core::primitive::usize {
            self.remaining()
          }
        }

        #[automatically_derived]
        impl #name<#path_to_grost::__private::flavors::Network> {
          #(#field_tag_consts)*
        }

        #[automatically_derived]
        impl #path_to_grost::__private::DefaultWireFormat<#path_to_grost::__private::flavors::Select> for #name<#path_to_grost::__private::flavors::Network> {
          type Format = #path_to_grost::__private::flavors::selector::LengthDelimited;
        }

        impl #path_to_grost::__private::Encode<#path_to_grost::__private::flavors::Select, #path_to_grost::__private::flavors::selector::LengthDelimited> for #name<#path_to_grost::__private::flavors::Network> {
          fn encode(&self, _: &<#path_to_grost::__private::flavors::Select as #path_to_grost::__private::Flavor>::Context, buf: &mut [::core::primitive::u8]) -> ::core::result::Result<::core::primitive::usize, <#path_to_grost::__private::flavors::Select as #path_to_grost::__private::Flavor>::EncodeError> {
            ::core::todo!()
          }

          fn encoded_len(&self, _: &<#path_to_grost::__private::flavors::Select as #path_to_grost::__private::Flavor>::Context) -> ::core::primitive::usize {
            ::core::todo!()
          }

          fn encoded_length_delimited_len(&self, ctx: &<#path_to_grost::__private::flavors::Select as #path_to_grost::__private::Flavor>::Context) -> ::core::primitive::usize {
            ::core::todo!()
          }

          fn encode_length_delimited(&self, ctx: &<#path_to_grost::__private::flavors::Select as #path_to_grost::__private::Flavor>::Context, buf: &mut [::core::primitive::u8]) -> ::core::result::Result<::core::primitive::usize, <#path_to_grost::__private::flavors::Select as #path_to_grost::__private::Flavor>::EncodeError> {
            ::core::todo!()
          }
        }

        // #path_to_grost::__private::selectable_scalar!(#flavor_ty: #name<#path_to_grost::__private::flavors::Network>);
        // #path_to_grost::__private::partial_encode_scalar!(#flavor_ty: #name<#path_to_grost::__private::flavors::Network> as #path_to_grost::__private::flavors::network::LengthDelimited);
        // #path_to_grost::__private::decode_owned_scalar!(#flavor_ty: #name<#path_to_grost::__private::flavors::Network> as #path_to_grost::__private::flavors::network::LengthDelimited);

        // impl #name<#path_to_grost::__private::flavors::Network> {
        //   /// Returns the encoded length of the selector.
        //   #[inline]
        //   pub const fn encoded_len(&self) -> ::core::primitive::usize {
        //     if self.is_empty() {
        //       return NONE_IDENTIFIER_ENCODED_LEN;
        //     }

        //     if self.is_all() {
        //       return ALL_IDENTIFIER_ENCODED_LEN;
        //     }

        //     let num_unselected = self.num_unselected();
        //     if num_unselected < Self::OPTIONS / 2 {
        //       let mut len = 0;

        //       #(
        //         #encode_unselected_len
        //       )*

        //       UNSELECT_IDENTIFIER_ENCODED_LEN + #path_to_grost::__private::varing::encoded_u32_varint_len(len as ::core::primitive::u32) + len
        //     } else {
        //       let mut len = 0;

        //       #(
        //         #encode_selected_len
        //       )*

        //       SELECT_IDENTIFIER_ENCODED_LEN + #path_to_grost::__private::varing::encoded_u32_varint_len(len as ::core::primitive::u32) + len
        //     }
        //   }

        //   /// Encodes the selector to the given buffer.
        //   pub fn encode(&self, buf: &mut [::core::primitive::u8]) -> ::core::result::Result<::core::primitive::usize, #path_to_grost::__private::flavors::network::EncodeError> {
        //     let buf_len = buf.len();
        //     if self.is_empty() {
        //       if buf_len < NONE_IDENTIFIER_ENCODED_LEN {
        //         return ::core::result::Result::Err(#path_to_grost::__private::EncodeError::insufficient_buffer(NONE_IDENTIFIER_ENCODED_LEN, buf_len));
        //       }

        //       let (b, _) = buf.split_at_mut(NONE_IDENTIFIER_ENCODED_LEN);
        //       b.copy_from_slice(NONE_IDENTIFIER_ENCODED);
        //       return ::core::result::Result::Ok(NONE_IDENTIFIER_ENCODED_LEN);
        //     }

        //     if self.is_all() {
        //       if buf_len < ALL_IDENTIFIER_ENCODED_LEN {
        //         return ::core::result::Result::Err(#path_to_grost::__private::EncodeError::insufficient_buffer(ALL_IDENTIFIER_ENCODED_LEN, buf_len));
        //       }

        //       let (b, _) = buf.split_at_mut(ALL_IDENTIFIER_ENCODED_LEN);
        //       b.copy_from_slice(ALL_IDENTIFIER_ENCODED);
        //       return ::core::result::Result::Ok(ALL_IDENTIFIER_ENCODED_LEN);
        //     }

        //     let num_unselected = self.num_unselected();
        //     if num_unselected < Self::OPTIONS / 2 {
        //       let mut offset = 0;
        //       if buf_len < UNSELECT_IDENTIFIER_ENCODED_LEN {
        //         return ::core::result::Result::Err(#path_to_grost::__private::EncodeError::insufficient_buffer(self.encoded_len(), buf_len));
        //       }

        //       buf[..UNSELECT_IDENTIFIER_ENCODED_LEN].copy_from_slice(UNSELECT_IDENTIFIER_ENCODED);
        //       offset += UNSELECT_IDENTIFIER_ENCODED_LEN;

        //       if offset > buf_len {
        //         return ::core::result::Result::Err(#path_to_grost::__private::EncodeError::insufficient_buffer(self.encoded_len(), buf_len));
        //       }

        //       let encoded_len = self.encoded_len();
        //       offset += #path_to_grost::__private::varing::encode_u32_varint_to(encoded_len as ::core::primitive::u32, &mut buf[offset..])
        //         .map_err(|e| {
        //           #path_to_grost::__private::EncodeError::from_varint_error(e)
        //             .update(self.encoded_len(), buf_len)
        //         })?;

        //       #(
        //         #encode_unselected
        //       )*

        //       ::core::result::Result::Ok(offset)
        //     } else {
        //       let mut offset = 0;
        //       if buf_len < SELECT_IDENTIFIER_ENCODED_LEN {
        //         return ::core::result::Result::Err(#path_to_grost::__private::EncodeError::insufficient_buffer(self.encoded_len(), buf_len));
        //       }

        //       buf[..SELECT_IDENTIFIER_ENCODED_LEN].copy_from_slice(SELECT_IDENTIFIER_ENCODED);
        //       offset += SELECT_IDENTIFIER_ENCODED_LEN;

        //       if offset > buf_len {
        //         return ::core::result::Result::Err(#path_to_grost::__private::EncodeError::insufficient_buffer(self.encoded_len(), buf_len));
        //       }

        //       let encoded_len = self.encoded_len();
        //       offset += #path_to_grost::__private::varing::encode_u32_varint_to(encoded_len as ::core::primitive::u32, &mut buf[offset..])
        //         .map_err(|e| {
        //           #path_to_grost::__private::EncodeError::from_varint_error(e)
        //             .update(self.encoded_len(), buf_len)
        //         })?;

        //       #(
        //         #encode_selected
        //       )*

        //       ::core::result::Result::Ok(offset)
        //     }
        //   }
        // }



        // impl<'de> #path_to_grost::__private::Decode<'de, #flavor_ty, #path_to_grost::__private::flavors::network::LengthDelimited, Self> for #name<#path_to_grost::__private::flavors::Network> {
        //   #[inline]
        //   fn decode<UB>(
        //     ctx: &<#flavor_ty as #path_to_grost::__private::Flavor>::Context,
        //     src: &'de [::core::primitive::u8],
        //   ) -> ::core::result::Result<(::core::primitive::usize, Self), <#flavor_ty as #path_to_grost::__private::Flavor>::DecodeError>
        //   where
        //     UB: #path_to_grost::__private::Buffer<#path_to_grost::__private::flavors::network::Unknown<&'de [::core::primitive::u8]>> + 'de,
        //   {
        //     ::core::todo!()
        //   }

        //   #[inline]
        //   fn decode_length_delimited<UB>(
        //     ctx: &<#flavor_ty as #path_to_grost::__private::Flavor>::Context,
        //     src: &'de [::core::primitive::u8],
        //   ) -> ::core::result::Result<(::core::primitive::usize, Self), <#flavor_ty as #path_to_grost::__private::Flavor>::DecodeError>
        //   where
        //     UB: #path_to_grost::__private::Buffer<#path_to_grost::__private::flavors::network::Unknown<&'de [::core::primitive::u8]>> + 'de,
        //   {
        //     ::core::todo!()
        //   }
        // }
      };
    }
  }

  fn generate_enum_codec(
    &self,
    path_to_grost: &syn::Path,
    enum_: &Enum,
  ) -> proc_macro2::TokenStream {
    let name_ident = enum_.name();

    quote! {
      #path_to_grost::__private::network_varint!(#name_ident);
    }
  }
}

impl Network {
  fn encoded_len_field_identifier_const_name(&self, field_name: &str) -> syn::Ident {
    format_ident!(
      "NETWORK_FLAVOR_ENCODED_{}_IDENTIFIER_LEN",
      field_name.to_uppercase()
    )
  }

  fn encoded_field_identifier_const_name(&self, field_name: &str) -> syn::Ident {
    format_ident!(
      "NETWORK_FLAVOR_ENCODED_{}_IDENTIFIER",
      field_name.to_uppercase()
    )
  }

  fn encoded_len_field_tag_const_name(&self, field_name: &str) -> syn::Ident {
    format_ident!(
      "__NETWORK_FLAVOR_ENCODED_LEN_{}_TAG__",
      field_name.to_uppercase()
    )
  }

  fn encode_field_tag_const_name(&self, field_name: &str) -> syn::Ident {
    format_ident!(
      "__NETWORK_FLAVOR_ENCODED_{}_TAG__",
      field_name.to_uppercase()
    )
  }

  fn field_identifier_const_name(&self, field_name: &str) -> syn::Ident {
    format_ident!("NETWORK_FLAVOR_{}_IDENTIFIER", field_name.to_uppercase())
  }

  fn field_tag_const_name(&self, field_name: &str) -> syn::Ident {
    format_ident!("NETWORK_FLAVOR_{}_TAG", field_name.to_uppercase())
  }

  fn field_wire_type_const_name(&self, field_name: &str) -> syn::Ident {
    format_ident!("NETWORK_FLAVOR_{}_WIRE_TYPE", field_name.to_uppercase())
  }
}
