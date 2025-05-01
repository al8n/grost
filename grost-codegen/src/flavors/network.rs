use quote::quote;
use smol_str::SmolStr;

use super::{Enum, FlavorGenerator, Struct};

#[derive(Clone)]
pub struct Network {
  ty: syn::Type,
  name: SmolStr,
}

impl Network {
  /// Returns a new `Network` flavor
  pub fn new(path_to_grost: &syn::Path) -> Self {
    let ty = syn::parse_quote!(#path_to_grost::__private::flavors::Network);
    Self {
      ty,
      name: core::any::type_name::<Self>().into(),
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

  fn name(&self) -> &str {
    &self.name
  }

  fn generate_selection_codec(
    &self,
    path_to_grost: &syn::Path,
    struct_: &Struct,
  ) -> proc_macro2::TokenStream {
    let flavor_ty = self.ty();
    let name = struct_.selection_name();
    quote! {
      const _: () = {
        const ALL_TAG: #path_to_grost::__private::Tag = #path_to_grost::__private::Tag::new(1);
        const NONE_TAG: #path_to_grost::__private::Tag = #path_to_grost::__private::Tag::new(2);
        const SELECT_TAG: #path_to_grost::__private::Tag = #path_to_grost::__private::Tag::new(3);
        const UNSELECT_TAG: #path_to_grost::__private::Tag = #path_to_grost::__private::Tag::new(4);
        const SELECT_ONE_TAG: #path_to_grost::__private::Tag = #path_to_grost::__private::Tag::new(5);
        const UNSELECT_ONE_TAG: #path_to_grost::__private::Tag = #path_to_grost::__private::Tag::new(6);

        const ALL_IDENTIFIER: #path_to_grost::__private::flavors::network::Identifier = #path_to_grost::__private::flavors::network::Identifier::new(#path_to_grost::__private::flavors::network::WireType::Zst, ALL_TAG);
        const NONE_IDENTIFIER: #path_to_grost::__private::flavors::network::Identifier = #path_to_grost::__private::flavors::network::Identifier::new(#path_to_grost::__private::flavors::network::WireType::Zst, NONE_TAG);
        const SELECT_IDENTIFIER: #path_to_grost::__private::flavors::network::Identifier = #path_to_grost::__private::flavors::network::Identifier::new(#path_to_grost::__private::flavors::network::WireType::LengthDelimited, SELECT_TAG);
        const UNSELECT_IDENTIFIER: #path_to_grost::__private::flavors::network::Identifier = #path_to_grost::__private::flavors::network::Identifier::new(#path_to_grost::__private::flavors::network::WireType::LengthDelimited, UNSELECT_TAG);
        const SELECT_ONE_IDENTIFIER: #path_to_grost::__private::flavors::network::Identifier = #path_to_grost::__private::flavors::network::Identifier::new(#path_to_grost::__private::flavors::network::WireType::Varint, SELECT_ONE_TAG);
        const UNSELECT_ONE_IDENTIFIER: #path_to_grost::__private::flavors::network::Identifier = #path_to_grost::__private::flavors::network::Identifier::new(#path_to_grost::__private::flavors::network::WireType::Varint, UNSELECT_ONE_TAG);

        const ALL_IDENTIFIER_ENCODED_LEN: ::core::primitive::usize = ALL_IDENTIFIER.encoded_len();
        const NONE_IDENTIFIER_ENCODED_LEN: ::core::primitive::usize = NONE_IDENTIFIER.encoded_len();
        const SELECT_IDENTIFIER_ENCODED_LEN: ::core::primitive::usize = SELECT_IDENTIFIER.encoded_len();
        const UNSELECT_IDENTIFIER_ENCODED_LEN: ::core::primitive::usize = UNSELECT_IDENTIFIER.encoded_len();
        const SELECT_ONE_IDENTIFIER_ENCODED_LEN: ::core::primitive::usize = SELECT_ONE_IDENTIFIER.encoded_len();
        const UNSELECT_ONE_IDENTIFIER_ENCODED_LEN: ::core::primitive::usize = UNSELECT_ONE_IDENTIFIER.encoded_len();

        const ALL_IDENTIFIER_ENCODED: &[::core::primitive::u8] = ALL_IDENTIFIER.encode().as_slice();
        const NONE_IDENTIFIER_ENCODED: &[::core::primitive::u8] = NONE_IDENTIFIER.encode().as_slice();
        const SELECT_IDENTIFIER_ENCODED: &[::core::primitive::u8] = SELECT_IDENTIFIER.encode().as_slice();
        const UNSELECT_IDENTIFIER_ENCODED: &[::core::primitive::u8] = UNSELECT_IDENTIFIER.encode().as_slice();
        const SELECT_ONE_IDENTIFIER_ENCODED: &[::core::primitive::u8] = SELECT_ONE_IDENTIFIER.encode().as_slice();
        const UNSELECT_ONE_IDENTIFIER_ENCODED: &[::core::primitive::u8] = UNSELECT_ONE_IDENTIFIER.encode().as_slice();

        impl #path_to_grost::__private::Wirable<#flavor_ty> for #name {
          const WIRE_TYPE: <#flavor_ty as #path_to_grost::__private::Flavor>::WireType = <[::core::primitive::u8] as #path_to_grost::__private::Wirable<#flavor_ty>>::WIRE_TYPE;
        }

        impl #path_to_grost::__private::PartialEncode<#flavor_ty> for #name {
          type Selection = ();

          #[inline]
          fn partial_encode(&self, context: &<#flavor_ty as #path_to_grost::__private::Flavor>::Context, buf: &mut [::core::primitive::u8], _: &Self::Selection) -> ::core::result::Result<::core::primitive::usize, <#flavor_ty as #path_to_grost::__private::Flavor>::EncodeError> {
            #path_to_grost::__private::Encode::<#flavor_ty>::encode(self, context, buf)
          }

          #[inline]
          fn partial_encoded_len(&self, context: &<#flavor_ty as #path_to_grost::__private::Flavor>::Context, _: &Self::Selection,) -> ::core::primitive::usize {
            #path_to_grost::__private::Encode::<#flavor_ty>::encoded_len(self, context)
          }
        }

        impl #path_to_grost::__private::Encode<#flavor_ty> for #name {
          #[inline]
          fn encode(&self, ctx: &<#flavor_ty as #path_to_grost::__private::Flavor>::Context, buf: &mut [::core::primitive::u8]) -> ::core::result::Result<::core::primitive::usize, <#flavor_ty as #path_to_grost::__private::Flavor>::EncodeError> {
            ::core::todo!()
          }

          #[inline]
          fn encoded_len(&self, ctx: &<#flavor_ty as #path_to_grost::__private::Flavor>::Context) -> ::core::primitive::usize {
            ::core::todo!()
          }
        }

        impl #path_to_grost::__private::Decode<'_, #flavor_ty, Self> for #name {
          #[inline]
          fn decode<UB>(
            ctx: &<#flavor_ty as #path_to_grost::__private::Flavor>::Context,
            src: &'de [::core::primitive::u8],
          ) -> ::core::result::Result<(::core::primitive::usize, Self), <#flavor_ty as #path_to_grost::__private::Flavor>::DecodeError>
          where
            UB: #path_to_grost::__private::Buffer<#path_to_grost::__private::Unknown<#flavor_ty, &'de [::core::primitive::u8]>> + 'de,
          {
            ::core::todo!()
          }
        }

        impl #path_to_grost::__private::DecodeOwned<#flavor_ty, Self> for #name {
          #[inline]
          fn decode_owned<B, UB>(
            ctx: &<#flavor_ty as #path_to_grost::__private::Flavor>::Context,
            src: B,
          ) -> ::core::result::Result<(::core::primitive::usize, Self), <#flavor_ty as #path_to_grost::__private::Flavor>::DecodeError>
          where
            Self: ::core::marker::Sized + 'static,
            B: #path_to_grost::__private::BytesBuffer + 'static,
            UB: #path_to_grost::__private::Buffer<#path_to_grost::__private::Unknown<#flavor_ty, B>> + 'static,
          {
            ::core::todo!()
          }
        }


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
      #path_to_grost::__private::default_wire_format!(#path_to_grost::__private::flavors::Network: #name_ident as #path_to_grost::__private::flavors::network::Varint: #path_to_grost::__private::flavors::network::Varint);
      #path_to_grost::__private::partial_encode_scalar!(#path_to_grost::__private::flavors::Network: #name_ident as #path_to_grost::__private::flavors::network::Varint);
      #path_to_grost::__private::decode_owned_scalar!(#path_to_grost::__private::flavors::Network: #name_ident as #path_to_grost::__private::flavors::network::Varint);
      #path_to_grost::__private::message!(#path_to_grost::__private::flavors::Network: #name_ident as #path_to_grost::__private::flavors::network::Varint);

      impl #path_to_grost::__private::Encode<#path_to_grost::__private::flavors::Network, #path_to_grost::__private::flavors::network::Varint> for #name_ident {
        #[inline]
        fn encode(&self, _: &<#path_to_grost::__private::flavors::Network as #path_to_grost::__private::Flavor>::Context, buf: &mut [::core::primitive::u8]) -> ::core::result::Result<::core::primitive::usize, <#path_to_grost::__private::flavors::Network as #path_to_grost::__private::Flavor>::EncodeError> {
          self.const_encode_to(buf).map_err(::core::convert::Into::into)
        }

        #[inline]
        fn encoded_len(&self, _: &<#path_to_grost::__private::flavors::Network as #path_to_grost::__private::Flavor>::Context) -> ::core::primitive::usize {
          self.const_encoded_len()
        }

        #[inline]
        fn encode_length_delimited(&self, ctx: &<#path_to_grost::__private::flavors::Network as #path_to_grost::__private::Flavor>::Context, buf: &mut [::core::primitive::u8]) -> ::core::result::Result<::core::primitive::usize, <#path_to_grost::__private::flavors::Network as #path_to_grost::__private::Flavor>::EncodeError> {
          <Self as #path_to_grost::__private::Encode<#path_to_grost::__private::flavors::Network, #path_to_grost::__private::flavors::network::Varint>>::encode(self, ctx, buf)
        }

        #[inline]
        fn encoded_length_delimited_len(&self, ctx: &<#path_to_grost::__private::flavors::Network as #path_to_grost::__private::Flavor>::Context) -> ::core::primitive::usize {
          <Self as #path_to_grost::__private::Encode<#path_to_grost::__private::flavors::Network, #path_to_grost::__private::flavors::network::Varint>>::encoded_len(self, ctx)
        }
      }

      impl<'de> #path_to_grost::__private::Decode<'de, #path_to_grost::__private::flavors::Network, #path_to_grost::__private::flavors::network::Varint, Self> for #name_ident {
        #[inline]
        fn decode<UB>(
          _: &<#path_to_grost::__private::flavors::Network as #path_to_grost::__private::Flavor>::Context,
          src: &'de [::core::primitive::u8],
        ) -> ::core::result::Result<(::core::primitive::usize, Self), <#path_to_grost::__private::flavors::Network as #path_to_grost::__private::Flavor>::DecodeError>
        where
          UB: #path_to_grost::__private::Buffer<#path_to_grost::__private::flavors::network::Unknown<&'de [::core::primitive::u8]>> + 'de,
        {
          Self::const_decode(src).map_err(::core::convert::Into::into)
        }

        #[inline]
        fn decode_length_delimited<UB>(
          ctx: &<#path_to_grost::__private::flavors::Network as #path_to_grost::__private::Flavor>::Context,
          src: &'de [::core::primitive::u8],
        ) -> ::core::result::Result<(::core::primitive::usize, Self), <#path_to_grost::__private::flavors::Network as #path_to_grost::__private::Flavor>::DecodeError>
        where
          UB: #path_to_grost::__private::Buffer<#path_to_grost::__private::flavors::network::Unknown<&'de [::core::primitive::u8]>> + 'de,
        {
          <Self as #path_to_grost::__private::Decode<'de, #path_to_grost::__private::flavors::Network, #path_to_grost::__private::flavors::network::Varint, Self>>::decode::<UB>(ctx, src)
        }
      }
    }
  }
}
