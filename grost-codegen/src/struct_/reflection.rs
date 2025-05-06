use crate::FlavorGenerator;

use super::*;

impl Struct {
  pub fn generate_reflection(&self, path_to_grost: &syn::Path) -> proc_macro2::TokenStream {
    let mod_name = self.reflection_mod_name();
    let reflection_name = self.reflection_name();
    let name = self.name.name();

    let field_reflection_name = self.field_reflection_name();
    let field_reflection_fns = self.fields.iter().map(|f| {
      let name = f.name();
      let tag = f.tag();
      let doc = format!(
        " Returns the field reflection of the field `{}.{}`.",
        self.name.name_str(),
        name.name_str()
      );
      quote! {
        #[doc = #doc]
        #[inline]
        pub const fn #name(&self) ->
          #field_reflection_name<
            #path_to_grost::__private::reflection::FieldReflection<F>,
            F,
            #tag,
          >
        {
          #field_reflection_name::new()
        }
      }
    });

    quote! {
      mod #mod_name {
        use super::#name;

        /// The field reflection of the struct.
        pub struct #field_reflection_name<R: ?::core::marker::Sized, F: ?::core::marker::Sized, const TAG: ::core::primitive::u32> {
          _reflect: ::core::marker::PhantomData<R>,
          _flavor: ::core::marker::PhantomData<F>,
        }

        impl<R, F, const TAG: ::core::primitive::u32> #field_reflection_name<R, F, TAG>
        where
          R: ?::core::marker::Sized,
          F: ?::core::marker::Sized,
        {
          const fn new_in() -> Self {
            Self {
              _reflect: ::core::marker::PhantomData,
              _flavor: ::core::marker::PhantomData,
            }
          }
        }

        impl<F, const TAG: ::core::primitive::u32> #field_reflection_name<#path_to_grost::__private::reflection::FieldReflection<F>, F, TAG>
        where
          F: ?::core::marker::Sized + #path_to_grost::__private::Flavor,
        {
          /// Returns the reflection of the field.
          #[inline]
          const fn new() -> Self {
            Self::new_in()
          }

          /// Returns the relection to a tag of the field.
          #[inline]
          pub const fn tag(&self) -> #field_reflection_name<
            #path_to_grost::__private::reflection::TagReflection<
              F::Tag,
            >,
            F,
            TAG,
          > {
            #field_reflection_name::new_in()
          }

          /// Returns the relection to the encoded tag of the field.
          #[inline]
          pub const fn encoded_tag(&self) -> #field_reflection_name<
            #path_to_grost::__private::reflection::EncodedTagReflection<
              F::Identifier,
            >,
            F,
            TAG,
          > {
            #field_reflection_name::new_in()
          }

          /// Returns the relection to the encoded tag of the field.
          #[inline]
          pub const fn encoded_tag_len(&self) -> #field_reflection_name<
            #path_to_grost::__private::reflection::Len<
              #path_to_grost::__private::reflection::EncodedTagReflection<
                F::Identifier,
              >,
            >,
            F,
            TAG,
          > {
            #field_reflection_name::new_in()
          }

          /// Returns the relection to a tag of the field.
          #[inline]
          pub const fn wire_type(&self) -> #field_reflection_name<
            #path_to_grost::__private::reflection::WireTypeReflection<
              F::WireType,
            >,
            F,
            TAG,
          > {
            #field_reflection_name::new_in()
          }

          /// Returns the relection to the identifier of the field.
          #[inline]
          pub const fn identifier(&self) -> #field_reflection_name<
            #path_to_grost::__private::reflection::IdentifierReflection<
              F::Identifier,
            >,
            F,
            TAG,
          > {
            #field_reflection_name::new_in()
          }

          /// Returns the relection to the encoded identifier of the field.
          #[inline]
          pub const fn encoded_identifier(&self) -> #field_reflection_name<
            #path_to_grost::__private::reflection::EncodedIdentifierReflection<
              F::Identifier,
            >,
            F,
            TAG,
          > {
            #field_reflection_name::new_in()
          }

          /// Returns the relection to the encoded identifier of the field.
          #[inline]
          pub const fn encoded_identifier_len(&self) -> #field_reflection_name<
            #path_to_grost::__private::reflection::Len<
              #path_to_grost::__private::reflection::EncodedIdentifierReflection<
                F::Identifier,
              >,
            >,
            F,
            TAG,
          > {
            #field_reflection_name::new_in()
          }

          /// Returns the reflection to the encode fn.
          #[inline]
          pub const fn encode(&self) -> #field_reflection_name<
            #path_to_grost::__private::reflection::encode::EncodeReflection<
              #path_to_grost::__private::reflection::encode::EncodeField,
            >,
            F,
            TAG,
          > {
            #field_reflection_name::new_in()
          }

          /// Returns the reflection to fn which will give the length of the encoded data.
          #[inline]
          pub const fn encoded_len(&self) -> #field_reflection_name<
            #path_to_grost::__private::reflection::encode::EncodeReflection<
              #path_to_grost::__private::reflection::Len<#path_to_grost::__private::reflection::encode::EncodeField,>,
            >,
            F,
            TAG,
          > {
            #field_reflection_name::new_in()
          }

          /// Returns the reflection to the reference encode fn.
          #[inline]
          pub const fn encode_ref<'a>(&self) -> #field_reflection_name<
            #path_to_grost::__private::reflection::encode::EncodeReflection<
              #path_to_grost::__private::reflection::encode::EncodeRefField<'a>,
            >,
            F,
            TAG,
          > {
            #field_reflection_name::new_in()
          }

          /// Returns the reflection to the reference encode fn which will give the length of the encoded data.
          #[inline]
          pub const fn encoded_ref_len<'a>(&self) -> #field_reflection_name<
            #path_to_grost::__private::reflection::encode::EncodeReflection<
              #path_to_grost::__private::reflection::Len<
                #path_to_grost::__private::reflection::encode::EncodeRefField<'a>,
              >,
            >,
            F,
            TAG,
          > {
            #field_reflection_name::new_in()
          }

          /// Returns the reflection to the partial encode fn.
          #[inline]
          pub const fn partial_encode(&self) -> #field_reflection_name<
            #path_to_grost::__private::reflection::encode::EncodeReflection<
              #path_to_grost::__private::reflection::encode::PartialEncodeField,
            >,
            F,
            TAG,
          > {
            #field_reflection_name::new_in()
          }

          /// Returns the reflection to the partial encode fn which will give the length of the encoded data.
          #[inline]
          pub const fn partial_encoded_len(&self) -> #field_reflection_name<
            #path_to_grost::__private::reflection::encode::EncodeReflection<
              #path_to_grost::__private::reflection::Len<
                #path_to_grost::__private::reflection::encode::PartialEncodeField,
              >,
            >,
            F,
            TAG,
          > {
            #field_reflection_name::new_in()
          }

          /// Returns the reflection to the partial reference encode fn.
          #[inline]
          pub const fn partial_encode_ref<'a>(&self) -> #field_reflection_name<
            #path_to_grost::__private::reflection::encode::EncodeReflection<
              #path_to_grost::__private::reflection::encode::PartialEncodeRefField<'a>,
            >,
            F,
            TAG,
          > {
            #field_reflection_name::new_in()
          }

          /// Returns the reflection to the partial reference encode fn which will give the length of the encoded data.
          #[inline]
          pub const fn partial_encoded_ref_len<'a>(&self) -> #field_reflection_name<
            #path_to_grost::__private::reflection::encode::EncodeReflection<
              #path_to_grost::__private::reflection::Len<
                #path_to_grost::__private::reflection::encode::PartialEncodeRefField<'a>,
              >,
            >,
            F,
            TAG,
          > {
            #field_reflection_name::new_in()
          }
        }

        impl<R, F, const TAG: ::core::primitive::u32> ::core::clone::Clone for #field_reflection_name<R, F, TAG>
        where
          R: ?::core::marker::Sized,
          F: ?::core::marker::Sized,
        {
          fn clone(&self) -> Self {
            *self
          }
        }

        impl<R, F, const TAG: ::core::primitive::u32> ::core::marker::Copy for #field_reflection_name<R, F, TAG>
        where
          R: ?::core::marker::Sized,
          F: ?::core::marker::Sized,
        {}

        /// The reflection bridge type.
        #[derive(::core::fmt::Debug)]
        pub struct #reflection_name<R: ?::core::marker::Sized, F: ?::core::marker::Sized> {
          _reflect: ::core::marker::PhantomData<R>,
          _flavor: ::core::marker::PhantomData<F>,
        }

        impl<R: ?::core::marker::Sized, F: ?::core::marker::Sized> ::core::clone::Clone for #reflection_name<R, F> {
          fn clone(&self) -> Self {
            *self
          }
        }

        impl<R: ?::core::marker::Sized, F: ?::core::marker::Sized> ::core::marker::Copy for #reflection_name<R, F> {}

        impl<R: ?::core::marker::Sized, F: ?::core::marker::Sized> #reflection_name<R, F> {
          const fn new_in() -> Self {
            Self {
              _reflect: ::core::marker::PhantomData,
              _flavor: ::core::marker::PhantomData,
            }
          }
        }

        impl<F> #reflection_name<#path_to_grost::__private::reflection::StructReflection<F>, F>
        where
          F: ?::core::marker::Sized + #path_to_grost::__private::Flavor,
        {
          /// Returns the reflection of the struct.
          #[inline]
          const fn new() -> Self {
            Self::new_in()
          }

          #(#field_reflection_fns)*
        }

        impl #name {
          /// Returns the reflection of the struct.
          #[allow(non_camel_case_types)]
          #[inline]
          pub const fn reflection<__GROST_FLAVOR__>() -> #reflection_name<
            #path_to_grost::__private::reflection::StructReflection<__GROST_FLAVOR__>,
            __GROST_FLAVOR__,
          >
          where
            __GROST_FLAVOR__: ?::core::marker::Sized + #path_to_grost::__private::Flavor,
          {
            #reflection_name::new()
          }
        }
      }
    }
  }
}
