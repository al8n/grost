use heck::ToShoutySnakeCase;
use quote::{ToTokens, format_ident};
use syn::Ident;

use crate::Flavor;

use super::*;

impl Struct {
  pub(crate) fn generate_selection(
    &self,
    path_to_grost: &syn::Path,
    flavors: &[Flavor],
  ) -> proc_macro2::TokenStream {
    let name = self.selection_name();
    let selection_flags_name = format_ident!("{}Flags", name.name_str());
    let selection_flags_iter_name = format_ident!("{}Iter", selection_flags_name);
    let fields = &self.fields;

    let selection_flags = generate_selection_flags(
      self.visibility.as_ref(),
      self.name.name(),
      &selection_flags_name,
      &selection_flags_iter_name,
      fields,
      path_to_grost,
      flavors,
    );
    let fns = fields.iter().map(move |f| {
      let field_name = f.name();
      let select_fn_name = format_ident!("select_{}", field_name.name_str());
      let select_fn_doc = format!(
        " Select the `{}.{}` field",
        self.name.name_str(),
        field_name.name_str()
      );
      let unselect_fn_name = format_ident!("unselect_{}", field_name.name_str());
      let unselect_fn_doc = format!(
        " Unselect the `{}.{}` field",
        self.name.name_str(),
        field_name.name_str()
      );
      let update_fn_name = format_ident!("update_{}", field_name.name_str());
      let update_fn_doc = format!(
        " Update the `{}.{}` field",
        self.name.name_str(),
        field_name.name_str()
      );
      let toggle_fn_name = format_ident!("toggle_{}", field_name.name_str());
      let toggle_fn_doc = format!(
        " Toggle the `{}.{}` field",
        self.name.name_str(),
        field_name.name_str()
      );
      let with_fn_name = format_ident!("with_{}", field_name.name_str());
      let with_fn_doc = format!(
        " Set the `{}.{}` field",
        self.name.name_str(),
        field_name.name_str()
      );
      let without_fn_name = format_ident!("without_{}", field_name.name_str());
      let without_fn_doc = format!(
        " Unset the `{}.{}` field",
        self.name.name_str(),
        field_name.name_str()
      );
      let maybe_fn_name = format_ident!("maybe_{}", field_name.name_str());
      let maybe_fn_doc = format!(
        " Set or unset the `{}.{}` field",
        self.name.name_str(),
        field_name.name_str()
      );
      let contains_fn_name = format_ident!("contains_{}", field_name.name_str());
      let contains_fn_doc = format!(
        " Check if the `{}.{}` field is set",
        self.name.name_str(),
        field_name.name_str()
      );

      quote! {
        #[doc = #select_fn_doc]
        #[inline]
        pub const fn #select_fn_name(&mut self) -> &mut Self {
          self.flags.#select_fn_name();
          self
        }

        #[doc = #unselect_fn_doc]
        #[inline]
        pub const fn #unselect_fn_name(&mut self) -> &mut Self {
          self.flags.#unselect_fn_name();
          self
        }

        #[doc = #update_fn_doc]
        #[inline]
        pub const fn #update_fn_name(&mut self, value: ::core::primitive::bool) -> &mut Self {
          self.flags.#update_fn_name(value);
          self
        }

        #[doc = #toggle_fn_doc]
        #[inline]
        pub const fn #toggle_fn_name(&mut self) -> &mut Self {
          self.flags.#toggle_fn_name();
          self
        }

        #[doc = #with_fn_doc]
        #[inline]
        pub const fn #with_fn_name(mut self) -> Self {
          self.flags = self.flags.#with_fn_name();
          self
        }

        #[doc = #without_fn_doc]
        #[inline]
        pub const fn #without_fn_name(mut self) -> Self {
          self.flags = self.flags.#without_fn_name();
          self
        }

        #[doc = #maybe_fn_doc]
        #[inline]
        pub const fn #maybe_fn_name(mut self, val: ::core::primitive::bool) -> Self {
          self.flags = self.flags.#maybe_fn_name(val);
          self
        }

        #[doc = #contains_fn_doc]
        #[inline]
        pub const fn #contains_fn_name(&self) -> ::core::primitive::bool {
          self.flags.#contains_fn_name()
        }
      }
    });

    let vis = self.visibility.as_ref();
    let doc = format!(" The selection type for {}", self.name.name_str());
    quote! {
      #selection_flags

      #[doc = #doc]
      #[derive(
        ::core::fmt::Debug,
        ::core::clone::Clone,
        ::core::marker::Copy,
        ::core::cmp::PartialEq,
        ::core::cmp::Eq,
        ::core::hash::Hash,
      )]
      #vis struct #name {
        flags: #selection_flags_name,
      }

      impl #name {
        /// Get a flags value with all bits unset.
        #[inline]
        pub const fn empty() -> Self {
          Self {
            flags: #selection_flags_name::empty(),
          }
        }

        /// Get a flags value with all known bits set.
        #[inline]
        pub const fn all() -> Self {
          Self {
            flags: #selection_flags_name::all(),
          }
        }

        /// Whether all bits in this flags value are unset.
        #[inline]
        pub const fn is_empty(&self) -> ::core::primitive::bool {
          self.flags.is_empty()
        }

        /// Whether all bits in this flags value are set.
        #[inline]
        pub const fn is_all(&self) -> ::core::primitive::bool {
          self.flags.is_all()
        }

        /// Returns an iterator over the selected fields, the iterator will yield the `FieldRelection` of the selected fields.
        #[inline]
        pub const fn iter<F: ?::core::marker::Sized>(&self) -> #selection_flags_iter_name<F> {
          self.flags.select_field_reflection_iter()
        }

        /// Merge another selection set into this one.
        #[inline]
        pub const fn merge(&mut self, other: Self) -> &mut Self {
          self.flags = self.flags.merge(other.flags);
          self
        }

        /// Merge another selection set into a new one.
        #[inline]
        pub const fn merge_into(mut self, other: Self) -> Self {
          self.flags = self.flags.merge(other.flags);
          self
        }

        #(#fns)*
      }
    }
  }
}

fn flags_declare<'a, F, O>(
  fields: &'a [Field],
  convert: F,
) -> impl Iterator<Item = proc_macro2::TokenStream> + 'a
where
  F: Fn(usize) -> O + 'a,
  O: ToTokens + 'a,
{
  fields.iter().enumerate().map(move |(idx, f)| {
    let field_name = f.name();
    let field_flag_name = format_ident!("{}", field_name.name_str().to_shouty_snake_case());
    let val = convert(idx);
    quote! {
      const #field_flag_name = #val;
    }
  })
}

fn flag_ops<'a>(fields: &'a [Field]) -> impl Iterator<Item = proc_macro2::TokenStream> + 'a {
  fields.iter().map(move |f| {
    let field_name = f.name();
    let field_flag_name = format_ident!("{}", field_name.name_str().to_shouty_snake_case());
    let select_fn_name = format_ident!("select_{}", field_name.name_str());
    let unselect_fn_name = format_ident!("unselect_{}", field_name.name_str());
    let update_fn_name = format_ident!("update_{}", field_name.name_str());
    let toggle_fn_name = format_ident!("toggle_{}", field_name.name_str());
    let with_fn_name = format_ident!("with_{}", field_name.name_str());
    let without_fn_name = format_ident!("without_{}", field_name.name_str());
    let maybe_fn_name = format_ident!("maybe_{}", field_name.name_str());
    let contains_fn_name = format_ident!("contains_{}", field_name.name_str());

    quote! {
      #[inline]
      const fn #select_fn_name(&mut self) -> &mut Self {
        *self = self.union(Self::#field_flag_name);
        self
      }

      #[inline]
      const fn #unselect_fn_name(&mut self) -> &mut Self {
        *self = self.difference(Self::#field_flag_name);
        self
      }

      #[inline]
      const fn #update_fn_name(&mut self, value: ::core::primitive::bool) -> &mut Self {
        if value {
          self.#select_fn_name()
        } else {
          self.#unselect_fn_name()
        }
      }

      #[inline]
      const fn #toggle_fn_name(&mut self) -> &mut Self {
        *self = self.symmetric_difference(Self::#field_flag_name);
        self
      }

      #[inline]
      const fn #with_fn_name(self) -> Self {
        self.union(Self::#field_flag_name)
      }

      #[inline]
      const fn #without_fn_name(self) -> Self {
        self.difference(Self::#field_flag_name)
      }

      #[inline]
      const fn #maybe_fn_name(self, val: ::core::primitive::bool) -> Self {
        if val {
          self.#with_fn_name()
        } else {
          self.#without_fn_name()
        }
      }

      #[inline]
      const fn #contains_fn_name(&self) -> ::core::primitive::bool {
        self.contains(Self::#field_flag_name)
      }
    }
  })
}

fn generate_bitflags_iter(
  path_to_grost: &syn::Path,
  vis: Option<&Visibility>,
  struct_name: &Ident,
  flags_name: &Ident,
  flags_iter_name: &Ident,
  flavors: &[Flavor],
) -> proc_macro2::TokenStream {
  let impl_iterators = flavors.iter().map(|f| {
    let flavor_ty = f.ty();
    let reflection_name = f.struct_reflection_name();
    quote! {
      impl ::core::iter::Iterator for #flags_iter_name<#flavor_ty> {
        type Item = #path_to_grost::__private::reflection::FieldRelection<#flavor_ty>;

        #[inline]
        fn next(&mut self) -> ::core::option::Option<Self::Item> {
          for f in ::core::iter::Iterator::by_ref(&mut self.iter) {
            if let ::core::option::Option::Some(val) = #struct_name::#reflection_name.fields().get(f.bits().trailing_zeros() as ::core::primitive::usize) {
              return ::core::option::Option::Some(*val);
            }
          }

          ::core::option::Option::None
        }
      }

      impl ::core::iter::FusedIterator for #flags_iter_name<#flavor_ty> {}
    }
  });

  quote! {
    /// Yield a set of selected fields.
    #vis struct #flags_iter_name<F: ?::core::marker::Sized> {
      iter: #path_to_grost::__private::bitflags::iter::Iter<#flags_name>,
      _m: ::core::marker::PhantomData<F>,
    }

    impl<F> #flags_iter_name<F>
    where
      F: ?::core::marker::Sized,
    {
      #[inline]
      const fn new(iter: #path_to_grost::__private::bitflags::iter::Iter<#flags_name>) -> Self {
        Self { iter, _m: ::core::marker::PhantomData }
      }
    }

    #(#impl_iterators)*
  }
}

fn generate_selection_flags(
  vis: Option<&Visibility>,
  struct_name: &Ident,
  name: &Ident,
  selection_flags_iter_name: &Ident,
  fields: &[Field],
  path_to_grost: &syn::Path,
  flavors: &[Flavor],
) -> proc_macro2::TokenStream {
  let num_fields = fields.len();

  let derives = quote! {
    #[derive(
      ::core::fmt::Debug,
      ::core::clone::Clone,
      ::core::marker::Copy,
      ::core::cmp::PartialEq,
      ::core::cmp::Eq,
      ::core::cmp::PartialOrd,
      ::core::cmp::Ord,
      ::core::hash::Hash,
    )]
  };
  let merge = quote! {
    #[inline]
    const fn merge(&self, other: Self) -> Self {
      Self(self.0.union(other.0))
    }
  };

  let iter_fn = quote! {
    #[inline]
    const fn select_field_reflection_iter<F: ?::core::marker::Sized>(&self) -> #selection_flags_iter_name<F> {
      #selection_flags_iter_name::new(self.iter())
    }
  };

  match num_fields {
    ..=8 => {
      let flags = flags_declare(fields, |idx| 1u8 << idx);
      let flag_ops = flag_ops(fields);
      let iter = generate_bitflags_iter(
        path_to_grost,
        vis,
        struct_name,
        name,
        selection_flags_iter_name,
        flavors,
      );
      quote! {
        #path_to_grost::__private::bitflags::bitflags! {
          #derives
          struct #name: ::core::primitive::u8 {
            #(#flags)*
          }
        }

        impl #name {
          #(#flag_ops)*

          #merge

          #iter_fn
        }

        #iter
      }
    }
    9..=16 => {
      let flags = flags_declare(fields, |idx| 1u16 << idx);
      let flag_ops = flag_ops(fields);
      let iter = generate_bitflags_iter(
        path_to_grost,
        vis,
        struct_name,
        name,
        selection_flags_iter_name,
        flavors,
      );
      quote! {
        #path_to_grost::__private::bitflags::bitflags! {
          #derives
          struct #name: ::core::primitive::u16 {
            #(#flags)*
          }
        }

        impl #name {
          #(#flag_ops)*

          #merge

          #iter_fn
        }

        #iter
      }
    }
    17..=32 => {
      let flags = flags_declare(fields, |idx| 1u32 << idx);
      let flag_ops = flag_ops(fields);
      let iter = generate_bitflags_iter(
        path_to_grost,
        vis,
        struct_name,
        name,
        selection_flags_iter_name,
        flavors,
      );
      quote! {
        #path_to_grost::__private::bitflags::bitflags! {
          #derives
          struct #name: ::core::primitive::u32 {
            #(#flags)*
          }
        }

        impl #name {
          #(#flag_ops)*

          #merge

          #iter_fn
        }

        #iter
      }
    }
    33..=64 => {
      let flags = flags_declare(fields, |idx| 1u64 << idx);
      let flag_ops = flag_ops(fields);
      let iter = generate_bitflags_iter(
        path_to_grost,
        vis,
        struct_name,
        name,
        selection_flags_iter_name,
        flavors,
      );
      quote! {
        #path_to_grost::__private::bitflags::bitflags! {
          #derives
          struct #name: ::core::primitive::u64 {
            #(#flags)*
          }
        }

        impl #name {
          #(#flag_ops)*

          #merge

          #iter_fn
        }

        #iter
      }
    }
    65..=128 => {
      let flags = flags_declare(fields, |idx| 1u128 << idx);
      let flag_ops = flag_ops(fields);
      let iter = generate_bitflags_iter(
        path_to_grost,
        vis,
        struct_name,
        name,
        selection_flags_iter_name,
        flavors,
      );
      quote! {
        #path_to_grost::__private::bitflags::bitflags! {
          #derives
          struct #name: ::core::primitive::u128 {
            #(#flags)*
          }
        }

        impl #name {
          #(#flag_ops)*

          #merge

          #iter_fn
        }

        #iter
      }
    }
    val => {
      let digits = val.div_ceil(64);
      let bits = fields.len().min(val) as u32;

      let flags = fields.iter().enumerate().map(move |(idx, f)| {
        let field_name = f.name();
        let field_flag_name = format_ident!("{}", field_name.name_str().to_shouty_snake_case());
        let idx = idx as u32;
        quote! {
          const #field_flag_name: #path_to_grost::__private::bnum::BUint::<#digits> = #path_to_grost::__private::bnum::BUint::<#digits>::ONE.shl(#idx);
        }
      });

      let all = (0..fields.len()).map(|idx| {
        let idx = idx as u32;
        quote! {
          set_bit(#idx, true)
        }
      });

      let flag_ops = fields.iter().enumerate().map(|(idx, field)| {
        let field_name = field.name();
        let select_fn_name = format_ident!("select_{}", field_name.name_str());
        let unselect_fn_name = format_ident!("unselect_{}", field_name.name_str());
        let update_fn_name = format_ident!("update_{}", field_name.name_str());
        let toggle_fn_name = format_ident!("toggle_{}", field_name.name_str());
        let with_fn_name = format_ident!("with_{}", field_name.name_str());
        let without_fn_name = format_ident!("without_{}", field_name.name_str());
        let maybe_fn_name = format_ident!("maybe_{}", field_name.name_str());
        let contains_fn_name = format_ident!("contains_{}", field_name.name_str());
        let idx = idx as u32;

        quote! {
          #[inline]
          const fn #select_fn_name(&mut self) -> &mut Self {
            *self = self.set_bit(#idx, true);
            self
          }

          #[inline]
          const fn #unselect_fn_name(&mut self) -> &mut Self {
            *self = self.set_bit(#idx, false);
            self
          }

          #[inline]
          const fn #update_fn_name(&mut self, value: ::core::primitive::bool) -> &mut Self {
            *self = self.set_bit(#idx, value);
            self
          }

          #[inline]
          const fn #toggle_fn_name(&mut self) -> &mut Self {
            *self = self.set_bit(#idx, !self.0.bit(#idx));
            self
          }

          #[inline]
          const fn #with_fn_name(self) -> Self {
            self.set_bit(#idx, true)
          }

          #[inline]
          const fn #without_fn_name(self) -> Self {
            self.set_bit(#idx, false)
          }

          #[inline]
          const fn #maybe_fn_name(self, val: ::core::primitive::bool) -> Self {
            self.set_bit(#idx, val)
          }

          #[inline]
          const fn #contains_fn_name(&self) -> ::core::primitive::bool {
            self.0.bit(#idx)
          }
        }
      });

      let impl_iterator = flavors.iter().map(|f| {
        let flavor_ty = f.ty();
        let reflection_name = f.struct_reflection_name();
        quote! {
          impl ::core::iter::Iterator for #selection_flags_iter_name<#flavor_ty> {
            type Item = #path_to_grost::__private::reflection::FieldRelection<#flavor_ty>;

            #[inline]
            fn next(&mut self) -> ::core::option::Option<Self::Item> {
              while self.idx < #name::MAX_BIT_POSITION {
                if self.selection.0.bit(self.idx) {
                  if let ::core::option::Option::Some(val) = #struct_name::#reflection_name.fields().get(self.idx as ::core::primitive::usize) {
                    return ::core::option::Option::Some(*val);
                  }
                }

                self.idx += 1;
              }

              ::core::option::Option::None
            }
          }

          impl ::core::iter::FusedIterator for #selection_flags_iter_name<#flavor_ty> {}
        }
      });

      quote! {
        /// Yield a set of selected fields.
        #vis struct #selection_flags_iter_name<F: ?::core::marker::Sized> {
          idx: ::core::primitive::u32,
          selection: #name,
          _m: ::core::marker::PhantomData<F>,
        }

        impl<F> #selection_flags_iter_name<F>
        where
          F: ?::core::marker::Sized,
        {
          #[inline]
          const fn new(idx: ::core::primitive::u32, selection: #name) -> Self {
            Self { idx, selection, _m: ::core::marker::PhantomData }
          }
        }

        #(#impl_iterator)*

        #derives
        struct #name(#path_to_grost::__private::bnum::BUint<#digits>);

        impl #name {
          const ALL: Self = {
            Self::empty()
              #(.#all)*
          };
          const MAX_BIT_POSITION: ::core::primitive::u32 = #bits;

          #(#flags)*

          #(#flag_ops)*

          #[inline]
          const fn empty() -> Self {
            Self(#path_to_grost::__private::bnum::BUint::<#digits>::ZERO)
          }

          #[inline]
          const fn all() -> Self {
            Self::ALL
          }

          #[inline]
          const fn is_empty(&self) -> ::core::primitive::bool {
            self.0.eq(&#path_to_grost::__private::bnum::BUint::<#digits>::ZERO)
          }

          #[inline]
          const fn is_all(&self) -> ::core::primitive::bool {
            self.0.eq(&Self::ALL.0)
          }

          #[inline]
          const fn merge(&self, other: Self) -> Self {
            Self(self.0.bitor(other.0))
          }

          #[inline]
          const fn select_field_reflection_iter(&self) -> #selection_flags_iter_name {
            #selection_flags_iter_name::new(0, *self)
          }

          #[inline]
          const fn set_bit(&self, idx: ::core::primitive::u32, val: ::core::primitive::bool) -> Self {
            let mask = #path_to_grost::__private::bnum::BUint::<#digits>::ONE.shl(idx);
            if val {
              Self(self.0.bitor(mask))
            } else {
              Self(self.0.bitand(mask.not()))
            }
          }
        }
      }
    }
  }
}
