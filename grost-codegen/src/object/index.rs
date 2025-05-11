use super::*;

impl Object {
  /// Generates the field indexer for the object
  pub fn generate_indexer(&self) -> proc_macro2::TokenStream {
    let name = self.indexer_name();
    let indexer_doc = format!(
      " Field indexer for the struct [`{}`]",
      self.name().name_str()
    );
    let vis = &self.visibility;
    let field_varint = self.fields().iter().enumerate().map(|(idx, f)| {
      let variant = format_ident!("{}", f.name().name_str().to_upper_camel_case());
      let variant_doc = format!(" The field indexer for the field `{}`", f.name().name_str());
      let idx = idx as u32;
      quote! {
        #[doc = #variant_doc]
        #variant = #idx
      }
    });

    quote! {
      #[doc = #indexer_doc]
      #[derive(::core::clone::Clone, ::core::marker::Copy, ::core::cmp::PartialEq, ::core::cmp::Eq, ::core::cmp::PartialOrd, ::core::cmp::Ord, ::core::hash::Hash, ::core::fmt::Debug)]
      #[repr(u32)]
      #vis enum #name {
        #(#field_varint),*
      }
    }
  }

  /// Derives implementations for the indexer of the object
  pub fn derive_indexer(&self, path_to_grost: &syn::Path) -> proc_macro2::TokenStream {
    let name = self.indexer_name();
    let num_fields = self.fields().len();
    let field_reflection_ident = self.field_reflection_name();

    let first_variant_name = format_ident!(
      "{}",
      self
        .fields()
        .first()
        .unwrap()
        .name()
        .name_str()
        .to_upper_camel_case()
    );
    let prev = self.fields().windows(2).rev().map(|f| {
      let curr_variant_name = format_ident!("{}", f[1].name().name_str().to_upper_camel_case());
      let prev_variant_name = format_ident!("{}", f[0].name().name_str().to_upper_camel_case());

      quote! {
        Self::#curr_variant_name => ::core::option::Option::Some(Self::#prev_variant_name)
      }
    });

    let next = self.fields().windows(2).map(|f| {
      let curr_variant_name = format_ident!("{}", f[0].name().name_str().to_upper_camel_case());
      let next_variant_name = format_ident!("{}", f[1].name().name_str().to_upper_camel_case());

      quote! {
        Self::#curr_variant_name => ::core::option::Option::Some(Self::#next_variant_name)
      }
    });
    let mut reflections_constraints = vec![];
    let reflections = self.fields().iter().map(|f| {
      let field_name = f.name();
      let field_variant = format_ident!("{}", field_name.name_str().to_upper_camel_case());
      let tag = f.tag();

      reflections_constraints.push(quote! {
        #field_reflection_ident<
          #path_to_grost::__private::reflection::FieldReflection<__GROST_FLAVOR__>,
          __GROST_FLAVOR__,
          #tag,
        >: #path_to_grost::__private::reflection::Reflectable<__GROST_FLAVOR__, Reflection = #path_to_grost::__private::reflection::FieldReflection<__GROST_FLAVOR__>>
      });
      quote! {
        Self::#field_variant => {
          <
            #field_reflection_ident<
              #path_to_grost::__private::reflection::FieldReflection<__GROST_FLAVOR__>,
              __GROST_FLAVOR__,
              #tag,
            > as #path_to_grost::__private::reflection::Reflectable<
              __GROST_FLAVOR__,
            >
          >::REFLECTION
        }
      }
    }).collect::<Vec<_>>();

    let last_variant_name = format_ident!(
      "{}",
      self
        .fields()
        .last()
        .unwrap()
        .name()
        .name_str()
        .to_upper_camel_case()
    );

    let struct_name = self.name();
    quote! {
      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl<__GROST_FLAVOR__: ?::core::marker::Sized> #path_to_grost::__private::indexer::Indexable<__GROST_FLAVOR__> for #struct_name {
        type Indexer = #name;
      }

      #[automatically_derived]
      impl #name {
        /// The number of variants of this field indexer.
        pub const VARIANTS: ::core::primitive::usize = #num_fields;
        /// The first field indexer.
        pub const FIRST: Self = Self::#first_variant_name;
        /// The last field indexer.
        pub const LAST: Self = Self::#last_variant_name;

        /// Returns the field reflection of the corresponding field.
        #[allow(non_camel_case_types, clippy::type_complexity)]
        #[inline]
        pub const fn reflection<__GROST_FLAVOR__>(
          &self,
        ) -> &'static #path_to_grost::__private::reflection::FieldReflection<__GROST_FLAVOR__>
        where
          __GROST_FLAVOR__: #path_to_grost::__private::flavors::Flavor + ?::core::marker::Sized,
          #(#reflections_constraints),*
        {
          match self {
            #(#reflections),*
          }
        }

        /// Returns the next field indexer.
        ///
        /// Returns `None` if there are no more fields.
        #[inline]
        pub const fn next(&self) -> ::core::option::Option<Self> {
          match self {
            Self::#last_variant_name => ::core::option::Option::None,
            #(#next),*
          }
        }

        /// Returns the previous field indexer.
        ///
        /// Returns `None` if there are no previous fields.
        #[inline]
        pub const fn prev(&self) -> ::core::option::Option<Self> {
          match self {
            Self::#first_variant_name => ::core::option::Option::None,
            #(#prev),*
          }
        }

        /// Returns the remaining number of fields.
        #[inline]
        pub const fn remaining(&self) -> ::core::primitive::usize {
          Self::LAST as ::core::primitive::usize - *self as ::core::primitive::u32 as ::core::primitive::usize
        }
      }

      #[automatically_derived]
      impl ::core::iter::Iterator for #name {
        type Item = Self;

        fn next(&mut self) -> ::core::option::Option<Self> {
          Self::next(self)
        }

        fn size_hint(&self) -> (::core::primitive::usize, ::core::option::Option<::core::primitive::usize>) {
          let remaining = self.remaining();
          (remaining, ::core::option::Option::Some(remaining))
        }
      }

      #[automatically_derived]
      impl ::core::iter::DoubleEndedIterator for #name {
        fn next_back(&mut self) -> ::core::option::Option<Self> {
          Self::prev(self)
        }
      }

      #[automatically_derived]
      impl ::core::iter::FusedIterator for #name {}
      #[automatically_derived]
      impl ::core::iter::ExactSizeIterator for #name {
        fn len(&self) -> ::core::primitive::usize {
          self.remaining()
        }
      }
    }
  }
}
