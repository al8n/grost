use quote::format_ident;
use quote::quote;
use syn::{Attribute, GenericParam, Generics, Ident};

use crate::object::mir::derive_flatten_state;
use crate::object::mir::nullable_accessors;

use super::{super::super::ast::GenericObject as GenericObjectAst, GenericField};

#[derive(Debug, Clone)]
pub struct GenericPartialRefObject {
  name: Ident,
  attrs: Vec<Attribute>,
  generics: Generics,
  copy: bool,
  buffer_field_name: Ident,
}

impl GenericPartialRefObject {
  /// Returns the name of the partial decoded object.
  #[inline]
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  /// Returns the attributes of the partial decoded object.
  #[inline]
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns the generics of the partial decoded object.
  #[inline]
  pub const fn generics(&self) -> &Generics {
    &self.generics
  }

  /// Returns `true` if the partial decoded object is copyable, `false` otherwise.
  #[inline]
  pub const fn copy(&self) -> bool {
    self.copy
  }

  pub(super) fn from_ast<M, F>(
    object: &GenericObjectAst<M, F>,
    fields: &[GenericField<F>],
  ) -> darling::Result<Self> {
    let path_to_grost = &object.path_to_grost;
    let partial_ref_object = &object.partial_ref;
    let partial_ref_object_name = partial_ref_object.name().clone();
    let flavor_param = &object.flavor_param;
    let buffer_param = &object.buffer_param;
    let lifetime_param = &object.lifetime_param;
    let copy = partial_ref_object.copy();
    let original_generics = &object.generics;
    let mut generics = Generics::default();

    // push the lifetime generic parameter first
    generics.params.extend(
      original_generics
        .lifetimes()
        .cloned()
        .map(GenericParam::from),
    );

    generics
      .params
      .push(syn::GenericParam::Lifetime(lifetime_param.clone()));

    // push the original type generic parameters
    generics.params.extend(
      original_generics
        .type_params()
        .cloned()
        .map(GenericParam::from),
    );

    generics.params.push(syn::GenericParam::Type({
      let ident = &flavor_param.ident;
      syn::parse2(quote! {
        #ident: ?::core::marker::Sized + #path_to_grost::__private::flavors::Flavor
      })?
    }));

    generics
      .params
      .push(syn::GenericParam::Type(buffer_param.clone()));

    // push the original const generic parameters last
    generics.params.extend(
      original_generics
        .const_params()
        .cloned()
        .map(GenericParam::from),
    );

    if let Some(where_clause) = original_generics.where_clause.as_ref() {
      generics
        .make_where_clause()
        .predicates
        .extend(where_clause.predicates.iter().cloned());
    }

    fields
      .iter()
      .filter_map(|f| f.try_unwrap_tagged_ref().ok())
      .for_each(|f| {
        let type_constraints = f.partial_ref().type_constraints();
        if !type_constraints.is_empty() {
          generics
            .make_where_clause()
            .predicates
            .extend(type_constraints.iter().cloned());
        }
      });

    Ok(Self {
      name: partial_ref_object_name,
      attrs: partial_ref_object.attrs().to_vec(),
      generics,
      buffer_field_name: format_ident!("__grost_buffer__"),
      copy,
    })
  }
}

impl<M, F> super::GenericObject<M, F> {
  pub(super) fn derive_partial_ref_object_defination(&self) -> proc_macro2::TokenStream {
    let partial_ref_object = self.partial_ref();
    let name = partial_ref_object.name();
    let vis = self.vis();
    let fields = self.fields().iter().filter_map(|f| {
      let field_name = f.name();
      match f {
        GenericField::Skipped(skipped_field) => {
          if !skipped_field.lifetime_params_usages().is_empty()
            || !skipped_field.type_params_usages().is_empty()
          {
            let ty = skipped_field.ty();
            Some(quote! {
              #field_name: ::core::marker::PhantomData<#ty>
            })
          } else {
            None
          }
        }
        GenericField::Tagged(generic_tagged_field) => {
          let ty = generic_tagged_field.partial_ref().nullable_type();
          let vis = generic_tagged_field.vis();
          let attrs = generic_tagged_field.partial_ref().attrs();

          Some(quote! {
            #(#attrs)*
            #vis #field_name: #ty
          })
        }
      }
    });
    let generics = partial_ref_object.generics();
    let where_clause = generics.where_clause.as_ref();
    let attrs = partial_ref_object.attrs();

    let doc = if !attrs.iter().any(|attr| attr.path().is_ident("doc")) {
      let doc = format!(
        " Partial reference struct for the struct [`{}`]",
        self.name()
      );
      quote! {
        #[doc = #doc]
      }
    } else {
      quote! {}
    };
    let ubfn = &partial_ref_object.buffer_field_name;
    let ubg = &self.buffer_param().ident;

    quote! {
      #(#attrs)*
      #doc
      #[allow(clippy::type_complexity, non_camel_case_types)]
      #vis struct #name #generics #where_clause
      {
        #ubfn: ::core::option::Option<#ubg>,
        #(#fields),*
      }
    }
  }

  pub(super) fn derive_partial_ref_object(&self) -> proc_macro2::TokenStream {
    let partial_ref_object = self.partial_ref();
    let name = partial_ref_object.name();
    let fields_init = self.fields().iter().filter_map(|f| {
      let field_name = f.name();
      match f {
        GenericField::Skipped(skipped_field) => {
          if !skipped_field.lifetime_params_usages().is_empty()
            || !skipped_field.type_params_usages().is_empty()
          {
            Some(quote! {
              #field_name: ::core::marker::PhantomData,
            })
          } else {
            None
          }
        }
        GenericField::Tagged(_) => Some(quote! {
          #field_name: ::core::option::Option::None,
        }),
      }
    });

    let mut fields_accessors = vec![];
    let mut is_empty = vec![];

    self
      .fields()
      .iter()
      .filter_map(|f| f.try_unwrap_tagged_ref().ok())
      .for_each(|f| {
        let field_name = f.name();
        let ty = &f.partial_ref().ty();
        let vis = f.vis();
        fields_accessors.push(nullable_accessors(
          field_name,
          vis,
          ty,
          f.partial_ref().copy(),
        ));
        is_empty.push(quote! {
          self.#field_name.is_none()
        });
      });

    let (ig, tg, where_clauses) = partial_ref_object.generics().split_for_impl();
    let ubfn = &partial_ref_object.buffer_field_name;
    let ubg = &self.buffer_param().ident;
    let flatten_state = derive_flatten_state(
      &self.path_to_grost,
      partial_ref_object.generics(),
      partial_ref_object.name(),
    );

    quote! {
      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl #ig ::core::default::Default for #name #tg #where_clauses
      {
        fn default() -> Self {
          Self::new()
        }
      }

      #flatten_state

      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl #ig #name #tg #where_clauses
      {
        /// Creates an empty partial struct.
        #[inline]
        pub const fn new() -> Self {
          Self {
            #(#fields_init)*
            #ubfn: ::core::option::Option::None,
          }
        }

        /// Returns `true` if the partial struct is empty, which means all fields are `None`.
        #[inline]
        pub const fn is_empty(&self) -> bool {
          self.#ubfn.is_none() && #(#is_empty)&&*
        }

        /// Returns a reference to the unknown buffer, which holds the unknown data when decoding.
        #[inline]
        pub const fn buffer(&self) -> ::core::option::Option<&#ubg> {
          self.#ubfn.as_ref()
        }

        // TODO(al8n): the following fns may lead to name conflicts if the struct has field whose name is buffer
        /// Returns a mutable reference to the unknown buffer, which holds the unknown data when decoding.
        #[inline]
        pub const fn buffer_mut(&mut self) -> ::core::option::Option<&mut #ubg> {
         self.#ubfn.as_mut()
        }

        /// Takes the unknown buffer out if the unknown buffer is not `None`.
        #[inline]
        pub const fn take_buffer(&mut self) -> ::core::option::Option<#ubg> {
          self.#ubfn.take()
        }

        /// Set the value of unknown buffer
        #[inline]
        pub fn set_buffer(&mut self, buffer: #ubg) -> &mut Self {
          self.#ubfn = ::core::option::Option::Some(buffer);
          self
        }

        /// Clears the unknown buffer.
        #[inline]
        pub fn clear_buffer(&mut self) -> &mut Self {
          self.#ubfn = ::core::option::Option::None;
          self
        }

        /// Set the value of unknown buffer
        #[inline]
        pub fn with_buffer(mut self, buffer: #ubg) -> Self {
          self.#ubfn = ::core::option::Option::Some(buffer);
          self
        }

        /// Clears the unknown buffer.
        #[inline]
        pub fn without_buffer(mut self) -> Self {
          self.#ubfn = ::core::option::Option::None;
          self
        }

        #(#fields_accessors)*
      }
    }
  }
}
