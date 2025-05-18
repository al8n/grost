use super::*;

impl Enum {
  /// Returns the generated enum variant info
  pub(crate) fn generate_reflection(&self, path_to_grost: &syn::Path) -> proc_macro2::TokenStream {
    let reflection_name = self.reflection_name();
    let variant_reflection_name = self.variant_reflection_name();
    let variant_reflections = self.variants.iter().map(|v| {
      let name = v.name.name_str();
      let schema_name = v.schema_name.as_str();
      let uevv = v.value();
      let value = uevv.to_non_zero_value();
      let raw_value = uevv.to_value();
      let variant_ident = uevv.to_variant_ident();
      let repr_encode_fn = self.repr.to_encode_fn(path_to_grost);
      let repr_max_encoded_len = self.repr.to_max_encoded_len();
      let val = quote! {
        #path_to_grost::__private::reflection::EnumVariantValue::#variant_ident(#value)
      };
      let description = v.description.as_deref().unwrap_or_default();
      quote! {
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl<__GROST_FLAVOR__: ?::core::marker::Sized> #path_to_grost::__private::reflection::Reflectable<
          __GROST_FLAVOR__,
        > for #variant_reflection_name<
            #path_to_grost::__private::reflection::EnumVariantReflection,
            __GROST_FLAVOR__,
            #raw_value,
        >
        {
          type Reflection = #path_to_grost::__private::reflection::EnumVariantReflection;
          const REFLECTION: &Self::Reflection = &{
            #path_to_grost::__private::reflection::EnumVariantReflectionBuilder {
              name: #name,
              schema_name: #schema_name,
              description: #description,
              value: #val,
            }.build()
          };
        }

        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl<__GROST_FLAVOR__: ?::core::marker::Sized> #path_to_grost::__private::reflection::Reflectable<
          __GROST_FLAVOR__,
        > for #variant_reflection_name<
          #path_to_grost::__private::reflection::encode::EncodeReflection<
            #path_to_grost::__private::reflection::EnumVariantReflection,
          >,
          __GROST_FLAVOR__,
          #raw_value,
        >
        {
          type Reflection = #path_to_grost::__private::varing::utils::Buffer<{ #repr_max_encoded_len + 1 }>;
          const REFLECTION: &Self::Reflection = &#repr_encode_fn(#raw_value);
        }

        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl<__GROST_FLAVOR__: ?::core::marker::Sized> #path_to_grost::__private::reflection::Reflectable<
          __GROST_FLAVOR__,
        > for #variant_reflection_name<
          #path_to_grost::__private::reflection::Len<
            #path_to_grost::__private::reflection::encode::EncodeReflection<
              #path_to_grost::__private::reflection::EnumVariantReflection,
            >,
          >,
          __GROST_FLAVOR__,
          #raw_value,
        >
        {
          type Reflection = ::core::primitive::usize;
          const REFLECTION: &Self::Reflection = &{
            <#variant_reflection_name<
              #path_to_grost::__private::reflection::encode::EncodeReflection<
                #path_to_grost::__private::reflection::EnumVariantReflection,
              >,
              __GROST_FLAVOR__,
              #raw_value,
            > as #path_to_grost::__private::reflection::Reflectable<
              __GROST_FLAVOR__,
            >>::REFLECTION.len()
          };
        }
      }
    });

    let variant_reflections_names = self.variants().iter().map(|f| {
      let variant_reflection = format_ident!("{}", self.variant_reflection_name());
      let raw_value = f.value().to_value();
      quote! {
        <
          #variant_reflection<
            #path_to_grost::__private::reflection::EnumVariantReflection,
            __GROST_FLAVOR__,
            #raw_value,
          > as #path_to_grost::__private::reflection::Reflectable<
            __GROST_FLAVOR__,
          >
        >::REFLECTION
      }
    });

    let variant_reflection_fns = self.variants().iter().map(|f| {
      let name = f.name();
      let value = f.value().to_value();
      let doc = format!(
        " Returns the variant reflection of the field [`{}::{}`].",
        self.name.name_str(),
        name.name_str()
      );
      let fn_name = format_ident!("{}", name.name_str().to_snake_case());
      quote! {
        #[doc = #doc]
        #[inline]
        pub const fn #fn_name(&self) ->
          #variant_reflection_name<
            #path_to_grost::__private::reflection::EnumVariantReflection,
            F,
            #value,
          >
        {
          #variant_reflection_name::new()
        }
      }
    });

    let name = self.name.name();
    let name_str = self.name.name_str();
    let schema_name = self.schema_name();
    let doc = format!(" The relection of the [`{}`] enum", name,);
    let variant_doc = format!(" The variant relection of the [`{}`] enum", name,);
    let description = self.description.as_deref().unwrap_or_default();
    let repr_variant = self.repr.to_variant_ident();
    let fqty = self.repr.to_full_qualified_type();

    quote! {
      #[doc = #variant_doc]
      pub struct #variant_reflection_name<R: ?::core::marker::Sized, F: ?::core::marker::Sized, const VALUE: #fqty> {
        _reflect: ::core::marker::PhantomData<R>,
        _flavor: ::core::marker::PhantomData<F>,
      }

      #[automatically_derived]
      impl<R, F, const VALUE: #fqty> ::core::ops::Deref for #variant_reflection_name<R, F, VALUE>
      where
        R: ?::core::marker::Sized,
        F: ?::core::marker::Sized + #path_to_grost::__private::flavors::Flavor,
        Self: #path_to_grost::__private::reflection::Reflectable<F>,
      {
        type Target = <Self as #path_to_grost::__private::reflection::Reflectable<F>>::Reflection;

        fn deref(&self) -> &Self::Target {
          <Self as #path_to_grost::__private::reflection::Reflectable<F>>::REFLECTION
        }
      }

      #[automatically_derived]
      impl<R, F, const VALUE: #fqty> ::core::convert::AsRef<<Self as ::core::ops::Deref>::Target> for #variant_reflection_name<R, F, VALUE>
      where
        R: ?::core::marker::Sized,
        F: ?::core::marker::Sized + #path_to_grost::__private::flavors::Flavor,
        Self: ::core::ops::Deref,
      {
        fn as_ref(&self) -> &<Self as ::core::ops::Deref>::Target {
          self
        }
      }

      #[automatically_derived]
      impl<R, F, const VALUE: #fqty> ::core::fmt::Debug for #variant_reflection_name<R, F, VALUE>
      where
        R: ?::core::marker::Sized,
        F: ?::core::marker::Sized + #path_to_grost::__private::flavors::Flavor,
        Self: #path_to_grost::__private::reflection::Reflectable<F>,
        <Self as #path_to_grost::__private::reflection::Reflectable<F>>::Reflection: ::core::fmt::Debug,
      {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::result::Result<(), ::core::fmt::Error> {
          ::core::fmt::Debug::fmt(::core::ops::Deref::deref(self), f)
        }
      }

      #[automatically_derived]
      impl<R, F, const VALUE: #fqty> ::core::fmt::Display for #variant_reflection_name<R, F, VALUE>
      where
        R: ?::core::marker::Sized,
        F: ?::core::marker::Sized + #path_to_grost::__private::flavors::Flavor,
        Self: #path_to_grost::__private::reflection::Reflectable<F>,
        <Self as #path_to_grost::__private::reflection::Reflectable<F>>::Reflection: ::core::fmt::Display,
      {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::result::Result<(), ::core::fmt::Error> {
          ::core::fmt::Display::fmt(::core::ops::Deref::deref(self), f)
        }
      }

      #[automatically_derived]
      impl<R, F, const VALUE: #fqty> #variant_reflection_name<R, F, VALUE>
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

      #[automatically_derived]
      #[allow(clippy::type_complexity)]
      impl<F, const VALUE: #fqty> #variant_reflection_name<#path_to_grost::__private::reflection::EnumVariantReflection, F, VALUE>
      where
        F: ?::core::marker::Sized,
      {
        /// Returns the reflection of the field.
        #[inline]
        const fn new() -> Self {
          Self::new_in()
        }

        /// Returns the relection to the encoded value in varint format of the variant.
        #[inline]
        pub const fn encoded_varint(&self) -> #variant_reflection_name<
          #path_to_grost::__private::reflection::encode::EncodeReflection<
            #path_to_grost::__private::reflection::EnumVariantReflection,
          >,
          F,
          VALUE,
        > {
          #variant_reflection_name::new_in()
        }

        /// Returns the relection to the encoded value in varint format of the variant.
        #[inline]
        pub const fn encoded_varint_len(&self) -> #variant_reflection_name<
          #path_to_grost::__private::reflection::Len<
            #path_to_grost::__private::reflection::encode::EncodeReflection<
              #path_to_grost::__private::reflection::EnumVariantReflection,
            >,
          >,
          F,
          VALUE,
        > {
          #variant_reflection_name::new_in()
        }
      }

      #[automatically_derived]
      impl<R, F, const VALUE: #fqty> ::core::clone::Clone for #variant_reflection_name<R, F, VALUE>
      where
        R: ?::core::marker::Sized,
        F: ?::core::marker::Sized,
      {
        fn clone(&self) -> Self {
          *self
        }
      }

      #[automatically_derived]
      impl<R, F, const VALUE: #fqty> ::core::marker::Copy for #variant_reflection_name<R, F, VALUE>
      where
        R: ?::core::marker::Sized,
        F: ?::core::marker::Sized,
      {}

      #[doc = #doc]
      pub struct #reflection_name<R: ?::core::marker::Sized, F: ?::core::marker::Sized> {
        _reflect: ::core::marker::PhantomData<R>,
        _flavor: ::core::marker::PhantomData<F>,
      }

      #[automatically_derived]
      impl<R, F> ::core::ops::Deref for #reflection_name<R, F>
      where
        R: ?::core::marker::Sized,
        F: ?::core::marker::Sized,
        Self: #path_to_grost::__private::reflection::Reflectable<F>,
      {
        type Target = <Self as #path_to_grost::__private::reflection::Reflectable<F>>::Reflection;

        fn deref(&self) -> &Self::Target {
          <Self as #path_to_grost::__private::reflection::Reflectable<F>>::REFLECTION
        }
      }

      #[automatically_derived]
      impl<R, F> ::core::convert::AsRef<<Self as ::core::ops::Deref>::Target> for #reflection_name<R, F>
      where
        R: ?::core::marker::Sized,
        F: ?::core::marker::Sized,
        Self: ::core::ops::Deref,
      {
        fn as_ref(&self) -> &<Self as ::core::ops::Deref>::Target {
          self
        }
      }

      #[automatically_derived]
      impl<R, F> ::core::fmt::Debug for #reflection_name<R, F>
      where
        R: ?::core::marker::Sized,
        F: ?::core::marker::Sized,
        Self: #path_to_grost::__private::reflection::Reflectable<F>,
        <Self as #path_to_grost::__private::reflection::Reflectable<F>>::Reflection: ::core::fmt::Debug,
      {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::result::Result<(), ::core::fmt::Error> {
          ::core::fmt::Debug::fmt(::core::ops::Deref::deref(self), f)
        }
      }

      #[automatically_derived]
      impl<R, F> ::core::fmt::Display for #reflection_name<R, F>
      where
        R: ?::core::marker::Sized,
        F: ?::core::marker::Sized,
        Self: #path_to_grost::__private::reflection::Reflectable<F>,
        <Self as #path_to_grost::__private::reflection::Reflectable<F>>::Reflection: ::core::fmt::Display,
      {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::result::Result<(), ::core::fmt::Error> {
          ::core::fmt::Display::fmt(::core::ops::Deref::deref(self), f)
        }
      }

      #[automatically_derived]
      impl<R: ?::core::marker::Sized, F: ?::core::marker::Sized> ::core::clone::Clone for #reflection_name<R, F> {
        fn clone(&self) -> Self {
          *self
        }
      }

      #[automatically_derived]
      impl<R: ?::core::marker::Sized, F: ?::core::marker::Sized> ::core::marker::Copy for #reflection_name<R, F> {}

      #[automatically_derived]
      impl<R: ?::core::marker::Sized, F: ?::core::marker::Sized> #reflection_name<R, F> {
        const fn new_in() -> Self {
          Self {
            _reflect: ::core::marker::PhantomData,
            _flavor: ::core::marker::PhantomData,
          }
        }
      }

      #[automatically_derived]
      impl<F> #reflection_name<#path_to_grost::__private::reflection::Enum, F>
      where
        F: ?::core::marker::Sized,
      {
        /// Returns the reflection of the enum.
        #[inline]
        const fn new() -> Self {
          Self::new_in()
        }

        #(#variant_reflection_fns)*
      }

      #[automatically_derived]
      impl #name {
        /// Returns the reflection.
        #[allow(non_camel_case_types)]
        #[inline]
        pub const fn reflection<__GROST_FLAVOR__>() -> #reflection_name<
          #path_to_grost::__private::reflection::Enum,
          __GROST_FLAVOR__,
        >
        where
          __GROST_FLAVOR__: ?::core::marker::Sized,
        {
          #reflection_name::new()
        }
      }

      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl<__GROST_FLAVOR__: ?::core::marker::Sized> #path_to_grost::__private::reflection::Reflectable<
        __GROST_FLAVOR__,
      > for #name {
        type Reflection = #path_to_grost::__private::reflection::Enum;
        const REFLECTION: &Self::Reflection = & {
          #path_to_grost::__private::reflection::EnumBuilder {
            name: #name_str,
            schema_name: #schema_name,
            description: #description,
            variants: &[
              #(#variant_reflections_names),*
            ],
            repr: #path_to_grost::__private::reflection::EnumRepr::#repr_variant,
          }.build()
        };
      }

      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl<__GROST_FLAVOR__: ?::core::marker::Sized> #path_to_grost::__private::reflection::Reflectable<
        __GROST_FLAVOR__,
      > for #reflection_name<#path_to_grost::__private::reflection::Enum, __GROST_FLAVOR__> {
        type Reflection = #path_to_grost::__private::reflection::Enum;
        const REFLECTION: &Self::Reflection = <
          #name as
            #path_to_grost::__private::reflection::Reflectable<
              __GROST_FLAVOR__,
            >
          >::REFLECTION;
      }

      #(#variant_reflections)*
    }
  }
}
