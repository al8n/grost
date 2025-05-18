use heck::ToSnakeCase;
use quote::{format_ident, quote};
use syn::{
  Fields, FieldsNamed, Ident,
  parse::{Parse, Parser},
};

use crate::ast::{
  grost_flavor_param,
  object::{Field, ObjectExt},
};

use super::Object;

pub struct Reflection {
  parent_name: Ident,
  name: Ident,
  vis: syn::Visibility,
  fields: Vec<Box<dyn Fn(&syn::Type) -> syn::Result<syn::Field> + 'static>>,
}

impl Reflection {
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  pub const fn vis(&self) -> &syn::Visibility {
    &self.vis
  }

  pub const fn fields(&self) -> &[Box<dyn Fn(&syn::Type) -> syn::Result<syn::Field> + 'static>] {
    self.fields.as_slice()
  }

  pub(super) fn from_input<O>(input: &O) -> darling::Result<Self>
  where
    O: crate::ast::object::Object,
  {
    let name = input.reflection_name();
    let parent_name = input.name().clone();
    let vis = input.vis().clone();
    let path_to_grost = input.path();
    let object_name = input.name();

    let fields = input
      .fields()
      .iter()
      .map(move |f| {
        let field_name = f.name().clone();
        let object_name = object_name.clone();
        let field_doc = format!(
          " The reflection to the `{}.{}` field.",
          object_name, field_name
        );
        let tag = f.meta().tag().get();
        let vis = f.vis().clone();
        let generics = input.generics().clone();
        let path_to_grost = path_to_grost.clone();

        Box::new(move |flavor: &syn::Type| {
          let (_, tg, _) = generics.split_for_impl();
          syn::Field::parse_named.parse2(quote! {
            #[doc = #field_doc]
            #vis #field_name: #path_to_grost::__private::reflection::Reflection<
              #object_name #tg,
              #path_to_grost::__private::reflection::Identified<
                #path_to_grost::__private::reflection::ObjectFieldReflection,
                #tag,
              >,
              #flavor,
            >
          })
        }) as Box<dyn Fn(&syn::Type) -> syn::Result<syn::Field> + 'static>
      })
      .collect::<Vec<Box<dyn Fn(&syn::Type) -> syn::Result<syn::Field> + 'static>>>();

    Ok(Self {
      parent_name,
      name,
      fields,
      vis,
    })
  }
}

impl<M> Object<M>
where
  M: crate::ast::object::Object,
{
  pub(super) fn derive_reflection(&self) -> syn::Result<proc_macro2::TokenStream> {
    let path_to_grost = &self.path_to_grost;
    let name = self.name();
    let fgp = grost_flavor_param();
    let fg = &fgp.ident;
    let fgty: syn::Type = syn::parse2(quote! { #fg }).unwrap();
    let (ig, tg, wc) = self.generics().split_for_impl();
    let field_reflection_fns = self
      .reflection
      .fields
      .iter()
      .map(|f| {
        (f)(&fgty).map(|field| {
          let field_name = field.ident.unwrap();
          let doc = format!(" Returns the field reflection of the field `{name}.{field_name}`.",);
          let ty = &field.ty;
          let field_reflection_name = format_ident!("{}_reflection", field_name);
          quote! {
            #[doc = #doc]
            #[inline]
            pub const fn #field_reflection_name<#fg>() -> #ty
            where
              #fg: ?::core::marker::Sized + #path_to_grost::__private::flavors::Flavor,
            {
              #path_to_grost::__private::reflection::Reflection::new()
            }
          }
        })
      })
      .collect::<Result<Vec<_>, _>>()?;

    Ok(quote! {
      const _: () = {
        // #[automatically_derived]
        // #[allow(clippy::type_complexity, non_camel_case_types)]
        // impl<F, const TAG: ::core::primitive::u32> #reflection_name<
        //   (
        //     #path_to_grost::__private::reflection::encode::EncodeReflection<
        //       #path_to_grost::__private::reflection::encode::PartialEncodeRefField,
        //     >,
        //     #path_to_grost::__private::RawTag<TAG>,
        //   ),
        //   F,
        // >
        // where
        //   F: ?::core::marker::Sized + #path_to_grost::__private::flavors::Flavor,
        // {
        //   /// Returns the reflection to the partial reference encode fn.
        //   #[inline]
        //   pub const fn partial_encode_ref(&self) -> Self {
        //     #reflection_name::new_in()
        //   }
        // }

        // #[automatically_derived]
        // #[allow(clippy::type_complexity, non_camel_case_types)]
        // impl<F, const TAG: ::core::primitive::u32> #reflection_name<
        //   (
        //     #path_to_grost::__private::reflection::encode::EncodeReflection<
        //       #path_to_grost::__private::reflection::Len<
        //         #path_to_grost::__private::reflection::encode::PartialEncodeRefField,
        //       >,
        //     >,
        //     #path_to_grost::__private::RawTag<TAG>,
        //   ),
        //   F,
        // >
        // where
        //   F: ?::core::marker::Sized + #path_to_grost::__private::flavors::Flavor,
        // {
        //   /// Returns the reflection to the partial reference encode fn which will give the length of the encoded data.
        //   #[inline]
        //   pub const fn partial_encoded_ref_len(&self) -> Self {
        //     #reflection_name::new_in()
        //   }
        // }

        // #[automatically_derived]
        // impl<R, F> ::core::ops::Deref for #reflection_name<R, F>
        // where
        //   R: ?::core::marker::Sized,
        //   F: ?::core::marker::Sized,
        //   Self: #path_to_grost::__private::reflection::Reflectable<F>,
        // {
        //   type Target = <Self as #path_to_grost::__private::reflection::Reflectable<F>>::Reflection;

        //   fn deref(&self) -> &Self::Target {
        //     <Self as #path_to_grost::__private::reflection::Reflectable<F>>::REFLECTION
        //   }
        // }

        // #[automatically_derived]
        // impl<R, F> ::core::convert::AsRef<<Self as ::core::ops::Deref>::Target> for #reflection_name<R, F>
        // where
        //   R: ?::core::marker::Sized,
        //   F: ?::core::marker::Sized,
        //   Self: ::core::ops::Deref,
        // {
        //   fn as_ref(&self) -> &<Self as ::core::ops::Deref>::Target {
        //     self
        //   }
        // }

        // #[automatically_derived]
        // impl<R, F> ::core::fmt::Debug for #reflection_name<R, F>
        // where
        //   R: ?::core::marker::Sized,
        //   F: ?::core::marker::Sized,
        //   Self: #path_to_grost::__private::reflection::Reflectable<F>,
        //   <Self as #path_to_grost::__private::reflection::Reflectable<F>>::Reflection: ::core::fmt::Debug,
        // {
        //   fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::result::Result<(), ::core::fmt::Error> {
        //     ::core::fmt::Debug::fmt(::core::ops::Deref::deref(self), f)
        //   }
        // }

        // #[automatically_derived]
        // impl<R, F> ::core::fmt::Display for #reflection_name<R, F>
        // where
        //   R: ?::core::marker::Sized,
        //   F: ?::core::marker::Sized,
        //   Self: #path_to_grost::__private::reflection::Reflectable<F>,
        //   <Self as #path_to_grost::__private::reflection::Reflectable<F>>::Reflection: ::core::fmt::Display,
        // {
        //   fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::result::Result<(), ::core::fmt::Error> {
        //     ::core::fmt::Display::fmt(::core::ops::Deref::deref(self), f)
        //   }
        // }

        // #[automatically_derived]
        // impl<R: ?::core::marker::Sized, F: ?::core::marker::Sized> ::core::clone::Clone for #reflection_name<R, F> {
        //   fn clone(&self) -> Self {
        //     *self
        //   }
        // }

        // #[automatically_derived]
        // impl<R: ?::core::marker::Sized, F: ?::core::marker::Sized> ::core::marker::Copy for #reflection_name<R, F> {}

        // #[automatically_derived]
        // impl<R: ?::core::marker::Sized, F: ?::core::marker::Sized> #reflection_name<R, F> {
        //   const fn new_in() -> Self {
        //     Self {
        //       _reflect: ::core::marker::PhantomData,
        //       _flavor: ::core::marker::PhantomData,
        //     }
        //   }
        // }

        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl #ig #name #tg #wc {
          /// Returns the reflection of the struct.
          #[inline]
          pub const fn reflection<#fg>() -> #path_to_grost::__private::reflection::Reflection<
            Self,
            #path_to_grost::__private::reflection::ObjectReflection,
            #fg,
          >
          where
            #fg: ?::core::marker::Sized + #path_to_grost::__private::flavors::Flavor,
          {
            #path_to_grost::__private::reflection::Reflection::new()
          }

          #(#field_reflection_fns)*
        }
      };
    })
  }
}
