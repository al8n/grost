use super::*;

impl Enum {
/// Returns the generated enum variant info
pub(crate) fn generate_reflection(&self, path_to_grost: &syn::Path) -> proc_macro2::TokenStream {
  let reflection_name = self.reflection_name();
  let variant_relection_name =
    |v: &EnumVariant| format_ident!("{}_REFLECTION", v.const_variant_name(),);

  let variant_reflection_consts = self.variants.iter().map(|v| {
    let const_name = variant_relection_name(v);
    quote! {
      Self::#const_name
    }
  });
  let variant_infos = self.variants.iter().map(|v| {
    let const_name = variant_relection_name(v);
    let name = v.name.name_str();
    let schema_name = v.schema_name.as_str();
    let doc = format!(" The relection information of the [`{}::{}`] enum variant.", self.name.name_str(), v.name.name_str());
    let uevv = v.value();
    let value = uevv.to_non_zero_value();
    let variant_ident = uevv.to_variant_ident();
    let val = quote! {
      #path_to_grost::__private::reflection::EnumVariantValue::#variant_ident(#value)
    };
    let description = v.description.as_deref().unwrap_or_default();
    quote! {
      #[doc = #doc]
      pub const #const_name: #path_to_grost::__private::reflection::EnumVariantReflection = #path_to_grost::__private::reflection::EnumVariantReflectionBuilder {
        name: #name,
        schema_name: #schema_name,
        description: #description,
        value: #val,
      }.build();
    }
  });

  let name = self.name.name();
  let schema_name = self.schema_name();
  let doc = format!(" The relection information of the [`{}`] enum", name,);
  let description = self.description.as_deref().unwrap_or_default();
  let repr_variant = self.repr.to_variant_ident();

  quote! {
    // #(#variant_infos)*

    // #[doc = #doc]
    // pub const REFLECTION: #path_to_grost::__private::reflection::EnumReflection = #path_to_grost::__private::reflection::EnumReflectionBuilder {
    //   name: #name,
    //   schema_name: #schema_name,
    //   description: #description,
    //   variants: &[
    //     #(#variant_reflection_consts,)*
    //   ],
    //   repr: #path_to_grost::__private::reflection::EnumRepr::#repr_variant,
    // }.build();

    /// The reflection bridge type.
    pub struct #reflection_name<R: ?::core::marker::Sized, F: ?::core::marker::Sized> {
      _reflect: ::core::marker::PhantomData<R>,
      _flavor: ::core::marker::PhantomData<F>,
    }

    #[automatically_derived]
    impl<R, F> ::core::ops::Deref for #reflection_name<R, F>
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
    impl<R, F> ::core::convert::AsRef<<Self as ::core::ops::Deref>::Target> for #reflection_name<R, F>
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
    impl<R, F> ::core::fmt::Debug for #reflection_name<R, F>
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
    impl<R, F> ::core::fmt::Display for #reflection_name<R, F>
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
    impl<F> #reflection_name<#path_to_grost::__private::reflection::EnumReflection, F>
    where
      F: ?::core::marker::Sized + #path_to_grost::__private::flavors::Flavor,
    {
      /// Returns the reflection of the enum.
      #[inline]
      const fn new() -> Self {
        Self::new_in()
      }

      // #(#field_reflection_fns)*
    }

    #[automatically_derived]
    impl #name {
      /// Returns the reflection.
      #[allow(non_camel_case_types)]
      #[inline]
      pub const fn reflection<__GROST_FLAVOR__>() -> #reflection_name<
        #path_to_grost::__private::reflection::EnumReflection,
        __GROST_FLAVOR__,
      >
      where
        __GROST_FLAVOR__: ?::core::marker::Sized + #path_to_grost::__private::flavors::Flavor,
      {
        #reflection_name::new()
      }
    }
  }
}
}