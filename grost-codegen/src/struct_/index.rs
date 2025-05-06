use crate::struct_;

use super::*;

impl Struct {
  pub(crate) fn generate_indexer(&self, path_to_grost: &syn::Path) -> proc_macro2::TokenStream {
    let name = self.indexer_name();
    let index_name = self.index_name();
    let vis = &self.visibility;
    let indexer_doc = format!(
      " Field indexer for the struct [`{}`]",
      self.name().name_str()
    );
    let index_doc = format!(
      " The concrete field index for the struct [`{}`]",
      self.name().name_str()
    );
    let field_varint = self.fields().iter().enumerate().map(|(idx, f)| {
      let variant = format_ident!("{}", f.name().name_str().to_upper_camel_case());
      let variant_doc = format!(" The field indexer for the field `{}`", f.name().name_str());
      let idx = idx as u32;
      quote! {
        #[doc = #variant_doc]
        #variant = #idx
      }
    });
    let num_fields = self.fields().len();

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

    let selector_name = self.selector_name();
    let struct_name = self.name();
    quote! {
      impl<F: ?::core::marker::Sized> #path_to_grost::__private::indexer::Indexable<F> for #struct_name {
        type Indexer = #name;
      }

      #[doc = #index_doc]
      pub struct #index_name<O: ?::core::marker::Sized, F: ?::core::marker::Sized> {
        variant: #name,
        _flavor: ::core::marker::PhantomData<F>,
        _output: ::core::marker::PhantomData<O>,
      }

      impl<O: ?::core::marker::Sized, F: ?::core::marker::Sized> ::core::convert::AsRef<#name> for #index_name<O, F> {
        fn as_ref(&self) -> &#name {
          &self.variant
        }
      }

      impl<O: ?::core::marker::Sized, F: ?::core::marker::Sized> ::core::clone::Clone for #index_name<O, F> {
        fn clone(&self) -> Self {
          *self
        }
      }

      impl<O: ?::core::marker::Sized, F: ?::core::marker::Sized> ::core::marker::Copy for #index_name<O, F> {}

      impl<O: ?::core::marker::Sized, F: ?::core::marker::Sized> #index_name<O, F> {
        /// Create a new field index.
        #[inline]
        pub const fn new(variant: #name) -> Self {
          Self {
            variant,
            _flavor: ::core::marker::PhantomData,
            _output: ::core::marker::PhantomData,
          }
        }

        /// Returns the indexer which creates this index.
        #[inline]
        pub const fn indexer(&self) -> #name {
          self.variant
        }
      }

      #[doc = #indexer_doc]
      #[derive(::core::clone::Clone, ::core::marker::Copy, ::core::cmp::PartialEq, ::core::cmp::Eq, ::core::cmp::PartialOrd, ::core::cmp::Ord, ::core::hash::Hash, ::core::fmt::Debug)]
      #[repr(u32)]
      #vis enum #name {
        #(#field_varint),*
      }

      #[automatically_derived]
      impl #name {
        /// The number of variants of this field indexer.
        pub const VARIANTS: ::core::primitive::usize = #num_fields;
        /// The first field indexer.
        pub const FIRST: Self = Self::#first_variant_name;
        /// The last field indexer.
        pub const LAST: Self = Self::#last_variant_name;

        /// Returns the field reflection index, which can be used to index the field reflection.
        #[inline]
        pub const fn field_reflection<F>(&self) -> #index_name<#path_to_grost::__private::reflection::FieldReflection<F>, F>
        where
          F: ?::core::marker::Sized + #path_to_grost::__private::flavors::Flavor,
        {
          #index_name::new(*self)
        }

        /// Returns the tag index, which can be used to index the tag of the field.
        #[inline]
        pub const fn tag<F>(&self) -> #index_name<F::Tag, F>
        where
          F: ?::core::marker::Sized + #path_to_grost::__private::flavors::Flavor,
        {
          #index_name::new(*self)
        }

        /// Returns the identifier index, which can be used to index the identifier of the field.
        #[inline]
        pub const fn identifier<F>(&self) -> #index_name<F::Identifier, F>
        where
          F: ?::core::marker::Sized + #path_to_grost::__private::flavors::Flavor,
        {
          #index_name::new(*self)
        }

        /// Returns the wire type index, which can be used to index the wire type of the field.
        #[inline]
        pub const fn wire_type<F>(&self) -> #index_name<F::WireType, F>
        where
          F: ?::core::marker::Sized + #path_to_grost::__private::flavors::Flavor,
        {
          #index_name::new(*self)
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
