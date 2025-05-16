use quote::quote;
use syn::{
  Fields, FieldsNamed, Ident,
  parse::{Parse, Parser},
};

use crate::ast::{
  grost_flavor_generic,
  object::{Field, ObjectExt},
};

use super::Object;

pub struct Reflection {
  parent_name: Ident,
  name: Ident,
  vis: syn::Visibility,
  fields: Fields,
}

impl Reflection {
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  pub const fn vis(&self) -> &syn::Visibility {
    &self.vis
  }

  pub const fn fields(&self) -> &Fields {
    &self.fields
  }

  pub(super) fn from_input<O>(input: &O) -> darling::Result<Self>
  where
    O: crate::ast::object::Object,
  {
    let name = input.reflection_name();
    let parent_name = input.name().clone();
    let vis = input.vis().clone();
    let path_to_grost = input.path();

    let fields = input
      .fields()
      .iter()
      .map(|f| {
        let field_name = f.name();
        let field_doc = format!(
          " The reflection to the `{}.{}` field.",
          input.name(),
          field_name
        );
        let tag = f.meta().tag().get();
        let vis = f.vis();
        syn::Field::parse_named.parse2(quote! {
          #[doc = #field_doc]
          #vis #field_name: #name<
            (
              #path_to_grost::__private::reflection::ObjectFieldReflection<F>,
              #path_to_grost::__private::RawTag<#tag>,
            ),
            F,
          >
        })
      })
      .collect::<Result<Vec<_>, _>>()?;

    let fields = Fields::Named(FieldsNamed::parse.parse2(quote! {{
      #(#fields),*
    }})?);

    Ok(Self {
      parent_name,
      name,
      fields,
      vis,
    })
  }

  pub(super) fn to_token_stream(&self) -> proc_macro2::TokenStream {
    let reflection_name = &self.name;
    let doc = format!(" The reflection of the [`{}`].", self.parent_name);
    let vis = &self.vis;

    quote! {
      #[doc = #doc]
      #vis struct #reflection_name<R: ?::core::marker::Sized, F: ?::core::marker::Sized> {
        _reflect: ::core::marker::PhantomData<R>,
        _flavor: ::core::marker::PhantomData<F>,
      }
    }
  }
}

impl<M> Object<M>
where
  M: crate::ast::object::Object,
{
  pub(super) fn derive_reflection(&self) -> proc_macro2::TokenStream {
    let path_to_grost = &self.path_to_grost;
    let reflection = self.reflection();
    let reflection_name = reflection.name();
    let name = self.name();
    let fg = grost_flavor_generic();
    let (ig, tg, wc) = self.generics().split_for_impl();
    let field_reflection_fns = reflection.fields.iter().map(|f| {
      let field_name = f.ident.as_ref().unwrap();
      let ty = &f.ty;
      let doc = format!(" Returns the field reflection of the field `{name}.{field_name}`.",);
      quote! {
        #[doc = #doc]
        #[inline]
        pub const fn #field_name(&self) -> #ty
        {
          #reflection_name::new_in()
        }
      }
    });

    quote! {
      const _: () = {
        #[automatically_derived]
        #[allow(clippy::type_complexity, non_camel_case_types)]
        impl<F, const TAG: ::core::primitive::u32> #reflection_name<
          (
            #path_to_grost::__private::reflection::WireFormatReflection,
            #path_to_grost::__private::RawTag<TAG>,
          ),
          F,
        >
        where
          F: ?::core::marker::Sized,
        {
          /// Returns the relection to the wire format of the field.
          #[inline]
          pub const fn wire_format(&self) -> Self
          {
            #reflection_name::new_in()
          }
        }

        #[automatically_derived]
        #[allow(clippy::type_complexity, non_camel_case_types)]
        impl<F, const TAG: ::core::primitive::u32> #reflection_name<
          (
            #path_to_grost::__private::reflection::TagReflection<
              <F as #path_to_grost::__private::flavors::Flavor>::Tag,
            >,
            #path_to_grost::__private::RawTag<TAG>,
          ),
          F,
        >
        where
          F: ?::core::marker::Sized + #path_to_grost::__private::flavors::Flavor,
        {
          /// Returns the relection to a tag of the field.
          #[inline]
          pub const fn tag(&self) -> Self
          {
            #reflection_name::new_in()
          }
        }

        #[automatically_derived]
        #[allow(clippy::type_complexity, non_camel_case_types)]
        impl<F, const TAG: ::core::primitive::u32> #reflection_name<
          (
            #path_to_grost::__private::reflection::EncodedTagReflection<
              <F as #path_to_grost::__private::flavors::Flavor>::Tag,
            >,
            #path_to_grost::__private::RawTag<TAG>,
          ),
          F,
        >
        where
          F: ?::core::marker::Sized + #path_to_grost::__private::flavors::Flavor,
        {
          /// Returns the relection to the encoded tag of the field.
          #[inline]
          pub const fn encoded_tag(&self) -> Self {
            #reflection_name::new_in()
          }
        }

        #[automatically_derived]
        #[allow(clippy::type_complexity, non_camel_case_types)]
        impl<F, const TAG: ::core::primitive::u32> #reflection_name<
          (
            #path_to_grost::__private::reflection::Len<
              #path_to_grost::__private::reflection::EncodedTagReflection<
                <F as #path_to_grost::__private::flavors::Flavor>::Tag,
              >,
            >,
            #path_to_grost::__private::RawTag<TAG>,
          ),
          F,
        >
        where
          F: ?::core::marker::Sized + #path_to_grost::__private::flavors::Flavor,
        {
          /// Returns length of the relection to the encoded tag of the field.
          #[inline]
          pub const fn encoded_tag_len(&self) -> Self {
            #reflection_name::new_in()
          }
        }

        #[automatically_derived]
        #[allow(clippy::type_complexity, non_camel_case_types)]
        impl<F, const TAG: ::core::primitive::u32> #reflection_name<
          (
            #path_to_grost::__private::reflection::WireTypeReflection<
              <F as #path_to_grost::__private::flavors::Flavor>::WireType,
            >,
            #path_to_grost::__private::RawTag<TAG>,
          ),
          F,
        >
        where
          F: ?::core::marker::Sized + #path_to_grost::__private::flavors::Flavor,
        {
          /// Returns the relection to the wire type of the field.
          #[inline]
          pub const fn wire_type(&self) -> Self {
            #reflection_name::new_in()
          }
        }

        #[automatically_derived]
        #[allow(clippy::type_complexity, non_camel_case_types)]
        impl<F, const TAG: ::core::primitive::u32> #reflection_name<
          (
            #path_to_grost::__private::reflection::IdentifierReflection<
              <F as #path_to_grost::__private::flavors::Flavor>::Identifier,
            >,
            #path_to_grost::__private::RawTag<TAG>,
          ),
          F,
        >
        where
          F: ?::core::marker::Sized + #path_to_grost::__private::flavors::Flavor,
        {
          /// Returns the relection to the identifier of the field.
          #[inline]
          pub const fn identifier(&self) -> Self {
            #reflection_name::new_in()
          }
        }

        #[automatically_derived]
        #[allow(clippy::type_complexity, non_camel_case_types)]
        impl<F, const TAG: ::core::primitive::u32> #reflection_name<
          (
            #path_to_grost::__private::reflection::EncodedIdentifierReflection<
              <F as #path_to_grost::__private::flavors::Flavor>::Identifier,
            >,
            #path_to_grost::__private::RawTag<TAG>,
          ),
          F,
        >
        where
          F: ?::core::marker::Sized + #path_to_grost::__private::flavors::Flavor,
        {
          /// Returns the relection to the encoded identifier of the field.
          #[inline]
          pub const fn encoded_identifier(&self) -> Self {
            #reflection_name::new_in()
          }
        }

        #[automatically_derived]
        #[allow(clippy::type_complexity, non_camel_case_types)]
        impl<F, const TAG: ::core::primitive::u32> #reflection_name<
          (
            #path_to_grost::__private::reflection::Len<
              #path_to_grost::__private::reflection::EncodedIdentifierReflection<
                <F as #path_to_grost::__private::flavors::Flavor>::Identifier,
              >,
            >,
            #path_to_grost::__private::RawTag<TAG>,
          ),
          F,
        >
        where
          F: ?::core::marker::Sized + #path_to_grost::__private::flavors::Flavor,
        {
          /// Returns the relection to the encoded identifier of the field.
          #[inline]
          pub const fn encoded_identifier_len(&self) -> Self {
            #reflection_name::new_in()
          }
        }

        #[automatically_derived]
        #[allow(clippy::type_complexity, non_camel_case_types)]
        impl<F, const TAG: ::core::primitive::u32> #reflection_name<
          (
            #path_to_grost::__private::reflection::encode::EncodeReflection<
              #path_to_grost::__private::reflection::encode::EncodeField,
            >,
            #path_to_grost::__private::RawTag<TAG>,
          ),
          F,
        >
        where
          F: ?::core::marker::Sized + #path_to_grost::__private::flavors::Flavor,
        {
          /// Returns the reflection to the encode fn.
          #[inline]
          pub const fn encode(&self) -> Self {
            #reflection_name::new_in()
          }
        }

        #[automatically_derived]
        #[allow(clippy::type_complexity, non_camel_case_types)]
        impl<F, const TAG: ::core::primitive::u32> #reflection_name<
          (
            #path_to_grost::__private::reflection::encode::EncodeReflection<
              #path_to_grost::__private::reflection::Len<#path_to_grost::__private::reflection::encode::EncodeField,>,
            >,
            #path_to_grost::__private::RawTag<TAG>,
          ),
          F,
        >
        where
          F: ?::core::marker::Sized + #path_to_grost::__private::flavors::Flavor,
        {
          /// Returns the reflection to fn which will give the length of the encoded data.
          #[inline]
          pub const fn encoded_len(&self) -> Self {
            #reflection_name::new_in()
          }
        }

        #[automatically_derived]
        #[allow(clippy::type_complexity, non_camel_case_types)]
        impl<F, const TAG: ::core::primitive::u32> #reflection_name<
          (
            #path_to_grost::__private::reflection::encode::EncodeReflection<
              #path_to_grost::__private::reflection::encode::EncodeRefField,
            >,
            #path_to_grost::__private::RawTag<TAG>,
          ),
          F,
        >
        where
          F: ?::core::marker::Sized + #path_to_grost::__private::flavors::Flavor,
        {
          /// Returns the reflection to the reference encode fn.
          #[inline]
          pub const fn encode_ref(&self) -> Self {
            #reflection_name::new_in()
          }
        }

        #[automatically_derived]
        #[allow(clippy::type_complexity, non_camel_case_types)]
        impl<F, const TAG: ::core::primitive::u32> #reflection_name<
          (
            #path_to_grost::__private::reflection::encode::EncodeReflection<
              #path_to_grost::__private::reflection::Len<
                #path_to_grost::__private::reflection::encode::EncodeRefField,
              >,
            >,
            #path_to_grost::__private::RawTag<TAG>,
          ),
          F,
        >
        where
          F: ?::core::marker::Sized + #path_to_grost::__private::flavors::Flavor,
        {
          /// Returns the reflection to the reference encode fn which will give the length of the encoded data.
          #[inline]
          pub const fn encoded_ref_len(&self) -> Self {
            #reflection_name::new_in()
          }
        }

        #[automatically_derived]
        #[allow(clippy::type_complexity, non_camel_case_types)]
        impl<F, const TAG: ::core::primitive::u32> #reflection_name<
          (
            #path_to_grost::__private::reflection::encode::EncodeReflection<
              #path_to_grost::__private::reflection::encode::PartialEncodeField,
            >,
            #path_to_grost::__private::RawTag<TAG>,
          ),
          F,
        >
        where
          F: ?::core::marker::Sized + #path_to_grost::__private::flavors::Flavor,
        {
          /// Returns the reflection to the partial encode fn.
          #[inline]
          pub const fn partial_encode(&self) -> Self {
            #reflection_name::new_in()
          }
        }

        #[automatically_derived]
        #[allow(clippy::type_complexity, non_camel_case_types)]
        impl<F, const TAG: ::core::primitive::u32> #reflection_name<
          (
            #path_to_grost::__private::reflection::encode::EncodeReflection<
              #path_to_grost::__private::reflection::Len<
                #path_to_grost::__private::reflection::encode::PartialEncodeField,
              >,
            >,
            #path_to_grost::__private::RawTag<TAG>,
          ),
          F,
        >
        where
          F: ?::core::marker::Sized + #path_to_grost::__private::flavors::Flavor,
        {
          /// Returns the reflection to the partial encode fn which will give the length of the encoded data.
          #[inline]
          pub const fn partial_encoded_len(&self) -> Self {
            #reflection_name::new_in()
          }
        }

        #[automatically_derived]
        #[allow(clippy::type_complexity, non_camel_case_types)]
        impl<F, const TAG: ::core::primitive::u32> #reflection_name<
          (
            #path_to_grost::__private::reflection::encode::EncodeReflection<
              #path_to_grost::__private::reflection::encode::PartialEncodeRefField,
            >,
            #path_to_grost::__private::RawTag<TAG>,
          ),
          F,
        >
        where
          F: ?::core::marker::Sized + #path_to_grost::__private::flavors::Flavor,
        {
          /// Returns the reflection to the partial reference encode fn.
          #[inline]
          pub const fn partial_encode_ref(&self) -> Self {
            #reflection_name::new_in()
          }
        }

        #[automatically_derived]
        #[allow(clippy::type_complexity, non_camel_case_types)]
        impl<F, const TAG: ::core::primitive::u32> #reflection_name<
          (
            #path_to_grost::__private::reflection::encode::EncodeReflection<
              #path_to_grost::__private::reflection::Len<
                #path_to_grost::__private::reflection::encode::PartialEncodeRefField,
              >,
            >,
            #path_to_grost::__private::RawTag<TAG>,
          ),
          F,
        >
        where
          F: ?::core::marker::Sized + #path_to_grost::__private::flavors::Flavor,
        {
          /// Returns the reflection to the partial reference encode fn which will give the length of the encoded data.
          #[inline]
          pub const fn partial_encoded_ref_len(&self) -> Self {
            #reflection_name::new_in()
          }
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
        impl<F> #reflection_name<#path_to_grost::__private::reflection::ObjectReflection<F>, F>
        where
          F: ?::core::marker::Sized + #path_to_grost::__private::flavors::Flavor,
        {
          /// Returns the reflection of the struct.
          #[inline]
          const fn new() -> Self {
            Self::new_in()
          }

          #(#field_reflection_fns)*
        }

        #[automatically_derived]
        impl #ig #name #tg #wc {
          /// Returns the reflection of the struct.
          #[allow(non_camel_case_types)]
          #[inline]
          pub const fn reflection<#fg>() -> #reflection_name<
            #path_to_grost::__private::reflection::ObjectReflection<#fg>,
            #fg,
          >
          where
            #fg: ?::core::marker::Sized + #path_to_grost::__private::flavors::Flavor,
          {
            #reflection_name::new()
          }
        }
      };
    }
  }
}
