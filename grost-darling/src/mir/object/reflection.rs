use quote::{quote, ToTokens};
use syn::{parse::{Parse, Parser}, Fields, FieldsNamed, Ident};

use crate::meta::object::{Field, ObjectExt};

pub struct Reflection {
  path_to_grost: syn::Path,
  field_reflection_name: Ident,
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
    O: crate::meta::object::Object,
  {
    let name = input.reflection_name();
    let parent_name = input.name().clone();
    let vis = input.vis().clone();
    let field_reflection_name = input.field_reflection_name();
    let path_to_grost = input.path();

    let fields = input
      .fields()
      .iter()
      .map(|f| {
        let field_name = f.name();
        let field_doc = format!(" The reflection to the `{}.{}` field.", input.name(), field_name);
        let tag = f.meta().tag().get();
        let vis = f.vis();
        syn::Field::parse_named.parse2(quote! {
          #[doc = #field_doc]
          #vis #field_name: #field_reflection_name<
            #path_to_grost::__private::reflection::FieldReflection<F>,
            F,
            #tag,
          >
        })
      })
      .collect::<Result<Vec<_>, _>>()?;

    let fields = Fields::Named(FieldsNamed::parse.parse2(quote! {{
      #(#fields),*
    }})?);

    Ok(Self {
      parent_name,
      path_to_grost: path_to_grost.clone(),
      field_reflection_name: input.field_reflection_name().clone(),
      name,
      fields,
      vis,
    })
  }
}

impl ToTokens for Reflection {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let name = &self.parent_name;
    let reflection_name = &self.name;
    let doc = format!(" The reflection of the [`{}`].", self.parent_name);
    let vis = &self.vis;
    let path_to_grost = &self.path_to_grost;
    let field_reflection_name = &self.field_reflection_name;

    let field_reflection_fns = self.fields.iter().map(|f| {
      let field_name = f.ident.as_ref().unwrap();
      let ty = &f.ty;
      let doc = format!(
        " Returns the field reflection of the field `{name}.{field_name}`.",
      );
      quote! {
        #[doc = #doc]
        #[inline]
        pub const fn #field_name(&self) -> #ty
        {
          #field_reflection_name::new()
        }
      }
    });

    tokens.extend(quote! {
      #[doc = #doc]
      #vis struct #reflection_name<R: ?::core::marker::Sized, F: ?::core::marker::Sized> {
        _reflect: ::core::marker::PhantomData<R>,
        _flavor: ::core::marker::PhantomData<F>,
      }

      const _: () = {
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
        impl #name {
          /// Returns the reflection of the struct.
          #[allow(non_camel_case_types)]
          #[inline]
          pub const fn reflection<__GROST_FLAVOR__>() -> #reflection_name<
            #path_to_grost::__private::reflection::ObjectReflection<__GROST_FLAVOR__>,
            __GROST_FLAVOR__,
          >
          where
            __GROST_FLAVOR__: ?::core::marker::Sized + #path_to_grost::__private::flavors::Flavor,
          {
            #reflection_name::new()
          }
        }
      };
    });
  }
}
