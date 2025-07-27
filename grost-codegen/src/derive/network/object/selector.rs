use heck::ToUpperCamelCase;

use super::super::*;

impl Groto {
  pub(crate) fn derive_selectable_for_object(
    &self,
    path_to_grost: &syn::Path,
    struct_: &Object,
  ) -> proc_macro2::TokenStream {
    let struct_name = struct_.name();
    let partial_struct_name = struct_.partial_struct_name();
    let partial_ref_name = struct_.partial_ref_name();
    let selector = struct_.selector_name();

    let selectable_impl = |name: &syn::Ident| -> proc_macro2::TokenStream {
      quote! {
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl #path_to_grost::__private::selection::Selectable<
          #path_to_grost::__private::flavors::Groto,
          #path_to_grost::__private::flavors::groto::LengthDelimited,
        > for #name
        {
          type Selector = #selector<#path_to_grost::__private::flavors::Groto>;
        }

        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl #path_to_grost::__private::selection::Selectable<
          #path_to_grost::__private::flavors::Groto,
          #path_to_grost::__private::flavors::groto::Repeated<#path_to_grost::__private::flavors::groto::LengthDelimited>,
        > for #name
        {
          type Selector = #selector<#path_to_grost::__private::flavors::Groto>;
        }

        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl<const I: ::core::primitive::u32> #path_to_grost::__private::selection::Selectable<
          #path_to_grost::__private::flavors::Groto,
          #path_to_grost::__private::flavors::groto::Repeated<
            #path_to_grost::__private::flavors::groto::Stream<#path_to_grost::__private::flavors::groto::LengthDelimited, I>
          >,
        > for #name
        {
          type Selector = #selector<#path_to_grost::__private::flavors::Groto>;
        }
      }
    };

    let selectable_impls = [
      selectable_impl(struct_name.name()),
      selectable_impl(&partial_struct_name),
    ];

    quote! {
      #(#selectable_impls)*

      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl<'__grost_lifetime__> #path_to_grost::__private::selection::Selectable<
        #path_to_grost::__private::flavors::Groto,
        #path_to_grost::__private::flavors::groto::LengthDelimited,
      > for #partial_ref_name<'__grost_lifetime__, #path_to_grost::__private::flavors::Groto>
      {
        type Selector = #selector<#path_to_grost::__private::flavors::Groto>;
      }

      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl<'__grost_lifetime__> #path_to_grost::__private::selection::Selectable<
        #path_to_grost::__private::flavors::Groto,
        #path_to_grost::__private::flavors::groto::Repeated<#path_to_grost::__private::flavors::groto::LengthDelimited>,
      > for #partial_ref_name<'__grost_lifetime__, #path_to_grost::__private::flavors::Groto>
      {
        type Selector = #selector<#path_to_grost::__private::flavors::Groto>;
      }

      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl<'__grost_lifetime__, const I: ::core::primitive::u32> #path_to_grost::__private::selection::Selectable<
        #path_to_grost::__private::flavors::Groto,
        #path_to_grost::__private::flavors::groto::Repeated<
          #path_to_grost::__private::flavors::groto::Stream<#path_to_grost::__private::flavors::groto::LengthDelimited, I>
        >,
      > for #partial_ref_name<'__grost_lifetime__, #path_to_grost::__private::flavors::Groto>
      {
        type Selector = #selector<#path_to_grost::__private::flavors::Groto>;
      }
    }
  }

  pub(crate) fn derive_selector_for_object(
    &self,
    path_to_grost: &syn::Path,
    struct_: &Object,
  ) -> proc_macro2::TokenStream {
    let iter_name = struct_.selector_iter_name();
    quote! {
      #[automatically_derived]
      impl<'__grost_lifetime__, const N: ::core::primitive::bool> ::core::iter::Iterator for #iter_name<'__grost_lifetime__, #path_to_grost::__private::flavors::Groto, N> {
        type Item = &'static #path_to_grost::__private::reflection::Field<#path_to_grost::__private::flavors::Groto>;

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
      impl<'__grost_lifetime__, const N: ::core::primitive::bool> ::core::iter::FusedIterator for #iter_name<'__grost_lifetime__, #path_to_grost::__private::flavors::Groto, N> {}

      #[automatically_derived]
      impl<'__grost_lifetime__, const N: ::core::primitive::bool> ::core::iter::ExactSizeIterator for #iter_name<'__grost_lifetime__, #path_to_grost::__private::flavors::Groto, N> {
        #[inline]
        fn len(&self) -> ::core::primitive::usize {
          self.remaining()
        }
      }
    }
  }

  pub(crate) fn derive_index_for_object(
    &self,
    path_to_grost: &syn::Path,
    struct_: &Object,
  ) -> proc_macro2::TokenStream {
    let indexer_name = struct_.indexer_name();
    let selector_name = struct_.selector_name();

    let index_trait_select = struct_.fields().iter().map(|f| {
      let field_name = f.name();
      let field_variant = format_ident!("{}", field_name.name_str().to_upper_camel_case());
      let field_reflection = struct_.field_reflection_name();
      let tag = f.tag();
      let reflection = quote! {
        const REFLECTION: ::core::option::Option<&#path_to_grost::__private::reflection::Field<#path_to_grost::__private::flavors::Groto>> = ::core::option::Option::Some(
          <
            #field_reflection<
              #path_to_grost::__private::reflection::Field<#path_to_grost::__private::flavors::Groto>,
              #path_to_grost::__private::flavors::Groto,
              #tag,
            > as #path_to_grost::__private::reflection::Reflectable<
              #path_to_grost::__private::flavors::Groto,
            >
          >::REFLECTION
        );
      };

      quote! {
        #indexer_name::#field_variant => {
          #reflection

          match (select, self.#field_name.is_empty()) {
            (true, false) => &REFLECTION,
            (true, true) => NONE,
            (false, false) => NONE,
            (false, true) => &REFLECTION,
          }
        }
      }
    });

    let index_selector = struct_.fields().iter().map(|f| {
      let field_name = f.name();
      let field_variant = format_ident!("{}", field_name.name_str().to_upper_camel_case());
      let field_reflection = struct_.field_reflection_name();
      let tag = f.tag();
      let reflection = quote! {
        const REFLECTION: ::core::option::Option<&#path_to_grost::__private::reflection::Field<#path_to_grost::__private::flavors::Groto>> = ::core::option::Option::Some(
          <
            #field_reflection<
              #path_to_grost::__private::reflection::Field<#path_to_grost::__private::flavors::Groto>,
              #path_to_grost::__private::flavors::Groto,
              #tag,
            > as #path_to_grost::__private::reflection::Reflectable<
              #path_to_grost::__private::flavors::Groto,
            >
          >::REFLECTION
        );
      };

      quote! {
        #indexer_name::#field_variant => {
          #reflection

          if #path_to_grost::__private::selection::Selector::<#path_to_grost::__private::flavors::Groto>::is_empty(&self.#field_name) {
            NONE
          } else {
            &REFLECTION
          }
        }
      }
    });

    quote! {
      #[automatically_derived]
      impl ::core::ops::Index<(#indexer_name, ::core::primitive::bool)> for #selector_name<#path_to_grost::__private::flavors::Groto> {
        type Output = ::core::option::Option<&'static #path_to_grost::__private::reflection::Field<#path_to_grost::__private::flavors::Groto>>;

        fn index(
          &self,
          (indexer, select): (#indexer_name, ::core::primitive::bool),
        ) -> &Self::Output {
          const NONE: &::core::option::Option<&'static #path_to_grost::__private::reflection::Field<#path_to_grost::__private::flavors::Groto>> = &::core::option::Option::None;

          match indexer {
            #(#index_trait_select),*
          }
        }
      }

      #[automatically_derived]
      impl ::core::ops::Index<#indexer_name> for #selector_name<#path_to_grost::__private::flavors::Groto> {
        type Output = ::core::option::Option<&'static #path_to_grost::__private::reflection::Field<#path_to_grost::__private::flavors::Groto>>;

        fn index(
          &self,
          indexer: #indexer_name,
        ) -> &Self::Output {
          const NONE: &::core::option::Option<&#path_to_grost::__private::reflection::Field<#path_to_grost::__private::flavors::Groto>> = &::core::option::Option::None;

          match indexer {
            #(#index_selector),*
          }
        }
      }
    }
  }
}
