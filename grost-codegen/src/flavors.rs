use heck::ToShoutySnakeCase;
use quote::{format_ident, quote};
use syn::Ident;

use crate::Field;

use super::{Enum, Struct};

/// The network flavor code generator
pub mod network;


/// The flavor
pub trait FlavorGenerator {
  /// Returns the full qualify path of the flavor type.
  fn ty(&self) -> &syn::Type;
  /// Sets the type of the flavor
  fn set_ty(&mut self, ty: syn::Type);
  /// Returns the name of the flavor this generator generates code for
  fn name(&self) -> &'static str;
  /// Generates the field identifier
  fn generate_field_identifier(
    &self,
    path_to_grost: &syn::Path,
    field: &Field,
  ) -> proc_macro2::TokenStream;

  /// Generates the codec for the selection type
  fn generate_selection_codec(
    &self,
    path_to_grost: &syn::Path,
    struct_: &Struct,
  ) -> proc_macro2::TokenStream;

  /// Generates the codec for the unit enum
  fn generate_enum_codec(
    &self,
    path_to_grost: &syn::Path,
    enum_: &Enum,
  ) -> proc_macro2::TokenStream {
    let name_ident = enum_.name();
    let flavor_ty = self.ty();
    let repr = enum_.repr();
    let repr_fqty = repr.to_full_qualified_type();
    let from_fn_name = format_ident!("from_{}", repr.to_type_str());
    let to_fn_name = format_ident!("as_{}", repr.to_type_str());

    quote! {
      impl #path_to_grost::__private::Wirable<#flavor_ty> for #name_ident {
        const WIRE_TYPE: <#flavor_ty as #path_to_grost::__private::Flavor>::WireType = <#repr_fqty as #path_to_grost::__private::Wirable<#flavor_ty>>::WIRE_TYPE;
      }

      impl #path_to_grost::__private::Encode<#flavor_ty> for #name_ident {
        #[inline]
        fn encode(&self, ctx: &<#flavor_ty as #path_to_grost::__private::Flavor>::Context, buf: &mut [::core::primitive::u8]) -> ::core::result::Result<::core::primitive::usize, <#flavor_ty as #path_to_grost::__private::Flavor>::EncodeError> {
          <#repr_fqty as #path_to_grost::__private::Encode<#flavor_ty>>::encode(&self.#to_fn_name(), ctx, buf)
        }

        #[inline]
        fn encoded_len(&self, ctx: &<#flavor_ty as #path_to_grost::__private::Flavor>::Context) -> ::core::primitive::usize {
          <#repr_fqty as #path_to_grost::__private::Encode<#flavor_ty>>::encoded_len(&self.#to_fn_name(), ctx)
        }
      }

      impl #path_to_grost::__private::PartialEncode<#flavor_ty> for #name_ident {
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

      impl<'de> #path_to_grost::__private::Decode<'de, #flavor_ty, Self> for #name_ident {
        #[inline]
        fn decode<UB>(
          ctx: &<#flavor_ty as #path_to_grost::__private::Flavor>::Context,
          src: &'de [::core::primitive::u8],
        ) -> ::core::result::Result<(::core::primitive::usize, Self), <#flavor_ty as #path_to_grost::__private::Flavor>::DecodeError>
        where
          UB: #path_to_grost::__private::Buffer<#path_to_grost::__private::Unknown<#flavor_ty, &'de [::core::primitive::u8]>> + 'de,
        {
          <#repr_fqty as #path_to_grost::__private::Decode<'de, #flavor_ty, #repr_fqty>>::decode::<UB>(ctx, src)
            .map(|(read, val)| (read, Self::#from_fn_name(val)))
        }
      }

      impl #path_to_grost::__private::DecodeOwned<#flavor_ty, Self> for #name_ident {
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
          <Self as #path_to_grost::__private::Decode<'_, #flavor_ty, Self>>::decode::<()>(ctx, src.as_bytes())
        }
      }
    }
  }
}

impl<F: FlavorGenerator + ?Sized> FlavorGenerator for Box<F> {
  fn ty(&self) -> &syn::Type {
    self.as_ref().ty()
  }

  fn name(&self) -> &'static str {
    self.as_ref().name()
  }

  fn set_ty(&mut self, ty: syn::Type) {
    self.as_mut().set_ty(ty)
  }

  fn generate_field_identifier(
    &self,
    path_to_grost: &syn::Path,
    field: &Field,
  ) -> proc_macro2::TokenStream {
    self.as_ref().generate_field_identifier(path_to_grost, field)
  }

  fn generate_enum_codec(
    &self,
    path_to_grost: &syn::Path,
    enum_: &Enum,
  ) -> proc_macro2::TokenStream {
    self.as_ref().generate_enum_codec(path_to_grost, enum_)
  }

  fn generate_selection_codec(
    &self,
    path_to_grost: &syn::Path,
    struct_: &Struct,
  ) -> proc_macro2::TokenStream {
    self
      .as_ref()
      .generate_selection_codec(path_to_grost, struct_)
  }
}

pub trait FlavorGeneratorExt: FlavorGenerator {
  fn field_reflection_name(&self, field_name: &str) -> Ident {
    let flavor_name_ssc = self.name().to_shouty_snake_case();
    format_ident!(
      "{flavor_name_ssc}_FLAVOR_{}_REFLECTION",
      field_name.to_shouty_snake_case()
    )
  }

  fn struct_reflection_name(&self) -> Ident {
    let flavor_name_ssc = self.name().to_shouty_snake_case();
    format_ident!("{flavor_name_ssc}_FLAVOR_REFLECTION")
  }
}

impl<F> FlavorGeneratorExt for F where F: FlavorGenerator + ?Sized {}
