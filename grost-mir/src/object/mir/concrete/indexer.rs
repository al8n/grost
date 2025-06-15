use quote::quote;

impl<M, F> super::ConcreteObject<M, F> {
  pub(super) fn derive_indexer_defination(&self) -> proc_macro2::TokenStream {
    let name = self.indexer().name();
    let vis = self.vis();

    let variants = self
      .fields()
      .iter()
      .filter_map(|f| f.try_unwrap_tagged_ref().ok())
      .map(|f| f.index().variant());
    let attrs = self.attrs();

    let doc = if !attrs.iter().any(|attr| attr.path().is_ident("doc")) {
      let doc = format!(" Field indexer for the struct [`{}`]", self.name());
      Some(quote! {
        #[doc = #doc]
      })
    } else {
      None
    };

    quote! {
      #(#attrs)*
      #doc
      #[derive(
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::core::cmp::PartialEq,
        ::core::cmp::Eq,
        ::core::cmp::PartialOrd,
        ::core::cmp::Ord,
        ::core::hash::Hash,
        ::core::fmt::Debug,
      )]
      #[repr(u32)]
      #vis enum #name {
        #(#variants),*
      }
    }
  }

  pub(super) fn derive_indexer(&self) -> proc_macro2::TokenStream {
    let name = self.indexer.name();
    let num_fields = self.fields().len();
    let path_to_grost = self.path_to_grost();
    let flavor_ty = self.flavor_type();

    let fields = self
      .fields()
      .iter()
      .filter_map(|f| f.try_unwrap_tagged_ref().ok())
      .collect::<Vec<_>>();
    let first_variant = fields.as_slice().first().unwrap().index().variant();
    let first_variant_name = &first_variant.ident;

    let last_variant = fields.as_slice().last().unwrap().index().variant();
    let last_variant_name = &last_variant.ident;

    let object_type = self.ty();
    let object_reflectable = self.reflectable();
    let generics = self.generics();
    let (ig, _, w) = generics.split_for_impl();

    let index = fields.iter().map(|f| {
      let fi = f.index();
      let idx = fi.index();
      let variant = fi.variant();
      let variant_name = &variant.ident;
      quote! {
        Self::#variant_name => #idx
      }
    });

    let prev = fields.windows(2).rev().map(|f| {
      let curr_variant_name = &f[1].index().variant().ident;
      let prev_variant_name = &f[0].index().variant().ident;

      quote! {
        Self::#curr_variant_name => ::core::option::Option::Some(Self::#prev_variant_name)
      }
    });

    let next = fields.windows(2).map(|f| {
      let curr_variant_name = &f[0].index().variant().ident;
      let next_variant_name = &f[1].index().variant().ident;

      quote! {
        Self::#curr_variant_name => ::core::option::Option::Some(Self::#next_variant_name)
      }
    });

    let reflections = fields
      .iter()
      .map(|f| {
        let field_variant = &f.index().variant().ident;
        let field_reflection_type = f.reflection().field_reflection();

        // reflections_constraints.push(quote! {
        //   #field_reflection_type: #path_to_grost::__private::reflection::Reflectable<
        //     #object_type,
        //     Reflection = #path_to_grost::__private::reflection::ObjectField,
        //   >
        // });

        quote! {
          Self::#field_variant => {
            <#field_reflection_type as #object_reflectable>::REFLECTION
          }
        }
      })
      .collect::<Vec<_>>();
    let object_reflection = self.reflection.object_reflection_generics();
    let (object_ig, _, object_wc) = object_reflection.split_for_impl();

    quote! {
      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl #ig #path_to_grost::__private::indexer::Indexable<#flavor_ty> for #object_type #w {
        type Indexer = #name;
      }

      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl #name {
        /// Returns the field reflection of the corresponding field.
        #[allow(non_camel_case_types, clippy::type_complexity)]
        #[inline]
        pub const fn reflection #object_ig (
          &self,
        ) -> &'static #path_to_grost::__private::reflection::ObjectField
        #object_wc
        {
          match self {
            #(#reflections),*
          }
        }
      }

      #[automatically_derived]
      impl #name {
        /// The number of variants of this field indexer.
        pub const VARIANTS: ::core::primitive::usize = #num_fields;
        /// The first field indexer.
        pub const FIRST: Self = Self::#first_variant_name;
        /// The last field indexer.
        pub const LAST: Self = Self::#last_variant_name;

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
          Self::LAST.index() - self.index()
        }

        const fn index(&self) -> ::core::primitive::usize {
          match self {
            #(#index),*
          }
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
