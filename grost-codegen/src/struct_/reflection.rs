use crate::FlavorGenerator;

use super::*;

impl Struct {
  pub fn generate_reflection(
    &self,
    path_to_grost: &syn::Path,
  ) -> proc_macro2::TokenStream
  {
    let mod_name = self.relfection_mod_name();
    let reflection_name = self.relfection_name();
    let name = self.name.name();
    let name_str = self.name.name_str();
    let schema_name = self.schema_name.as_str();
    let fields = &self.fields;
    // let fields_reflection_defination = fields
    //   .iter()
    //   .map(|f| f.field_reflections(path_to_grost, flavor));
    // let field_reflections = fields.iter().map(|f| {
    //   let ident = flavor.field_reflection_name(f.name().name_str());
    //   quote! { Self::#ident }
    // });

    // let reflection_name = flavor.struct_reflection_name();
    // let reflection_doc = format!(
    //   " The reflection of the struct `{name_str}` for [`{}`]({}) flavor.",
    //   flavor.name().to_upper_camel_case(),
    //   flavor.ty().to_token_stream().to_string().replace(" ", "")
    // );
    // let flavor_ty = flavor.ty();

    let field_reflections_defination = fields.iter().map(|f| {
      let field_reflection_name = self.field_reflection_name(f.name().name_str());
      let doc = format!(
        " The reflection of the field `{}` in the struct `{}`",
        f.name().name_str(),
        name_str,
      );
      quote! {
        #[doc = #doc]
        #[derive(::core::fmt::Debug)]
        pub struct #field_reflection_name<R: ?::core::marker::Sized, F: ?::core::marker::Sized> {
          _reflect: ::core::marker::PhantomData<R>,
          _flavor: ::core::marker::PhantomData<F>,
        }

        impl<R, F> #field_reflection_name<R, F>
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

        impl<F> #field_reflection_name<#path_to_grost::__private::reflection::FieldReflection<F>, F>
        where
          F: ?::core::marker::Sized + #path_to_grost::__private::Flavor,
        {
          /// Returns the reflection of the field.
          #[inline]
          pub const fn new() -> Self {
            #field_reflection_name::new_in()
          }

          /// Returns the relection to a tag of the field.
          #[inline]
          pub const fn tag(&self) -> #field_reflection_name<
            #path_to_grost::__private::reflection::TagReflection<
              F::Tag,
            >,
            F,
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
          > {
            #field_reflection_name::new_in()
          }
        }

        impl<F> ::core::default::Default for #field_reflection_name<#path_to_grost::__private::reflection::FieldReflection<F>, F>
        where
          F: ?::core::marker::Sized + #path_to_grost::__private::Flavor,
        {
          fn default() -> Self {
            Self::new()
          }
        }

        impl<R, F> ::core::clone::Clone for #field_reflection_name<R, F>
        where
          R: ?::core::marker::Sized,
          F: ?::core::marker::Sized,
        {
          fn clone(&self) -> Self {
            *self
          }
        }

        impl<R, F> ::core::marker::Copy for #field_reflection_name<R, F>
        where
          R: ?::core::marker::Sized,
          F: ?::core::marker::Sized,
        {}
      }
    });

    quote! {
      mod #mod_name {
        use super::{#name,};

        #(#field_reflections_defination)*

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

        impl<F> ::core::default::Default for #reflection_name<#path_to_grost::__private::reflection::StructReflection<F>, F>
        where
          F: ?::core::marker::Sized + #path_to_grost::__private::Flavor,
        {
          fn default() -> Self {
            Self::new()
          }
        }

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
          pub const fn new() -> Self {
            Self::new_in()
          }
        }

        impl #name {
          /// Returns the reflection of the struct.
          #[allow(non_camel_case_types)]
          #[inline]
          pub const fn reflection<__GROST_FLAOVR__>() -> #reflection_name<
            #path_to_grost::__private::reflection::StructReflection<__GROST_FLAOVR__>,
            __GROST_FLAOVR__,
          >
          where
            __GROST_FLAOVR__: ?::core::marker::Sized + #path_to_grost::__private::Flavor,
          {
            #reflection_name::new()
          }
        }
      } 
    }
  }
}