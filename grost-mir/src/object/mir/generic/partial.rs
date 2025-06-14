use syn::{Attribute, GenericParam, Generics, Ident, Type};

use quote::{format_ident, quote};

use crate::object::mir::{derive_flatten_state, optional_accessors};

use super::{super::super::ast::GenericObject as GenericObjectAst, GenericField};

#[derive(Debug, Clone)]
pub struct GenericPartialObject {
  name: Ident,
  ty: Type,
  generics: Generics,
  attrs: Vec<Attribute>,
  unknown_buffer_field_name: Ident,
  copy: bool,
}

impl GenericPartialObject {
  /// Returns the name of the partial object
  #[inline]
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  /// Returns the attributes of the partial object
  #[inline]
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns the type of the partial object
  ///
  /// e.g. if the name is `PartialUserObject`, and the `unknown_buffer` returns the `UB`  this will return `PartialUserObject<UB>`
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the generics of the partial object.
  #[inline]
  pub const fn generics(&self) -> &Generics {
    &self.generics
  }

  /// Returns `true` if the partial object is copyable, `false` otherwise.
  #[inline]
  pub const fn copy(&self) -> bool {
    self.copy
  }

  pub(super) fn from_ast<M, F>(
    object: &GenericObjectAst<M, F>,
    fields: &[GenericField<F>],
  ) -> darling::Result<Self> {
    let partial_object = object.partial();
    let unknown_buffer_param = object.unknown_buffer_param();

    let mut generics = object.generics().clone();
    generics
      .params
      .push(GenericParam::Type(unknown_buffer_param.clone()));

    fields
      .iter()
      .filter_map(|f| f.try_unwrap_tagged_ref().ok())
      .try_for_each(|f| {
        if !f.partial().type_constraints().is_empty() {
          generics
            .make_where_clause()
            .predicates
            .extend(f.partial().type_constraints().iter().cloned());
        }

        darling::Result::Ok(())
      })?;

    let (_, tg, _) = generics.split_for_impl();
    let name = partial_object.name().clone();
    let ty = syn::parse2(quote! {
      #name #tg
    })?;
    let copy = object.copy();

    Ok(Self {
      name,
      ty,
      generics,
      attrs: partial_object.attrs().to_vec(),
      unknown_buffer_field_name: format_ident!("__grost_unknown_buffer__"),
      copy,
    })
  }
}

impl<M, F> super::GenericObject<M, F> {
  pub(super) fn derive_partial_object_defination(&self) -> proc_macro2::TokenStream {
    let partial_object = self.partial();
    let name = partial_object.name();
    let vis = self.vis();
    let generics = partial_object.generics();
    let where_clause = generics.where_clause.as_ref();
    let ubfn = &partial_object.unknown_buffer_field_name;
    let ubg = &self.unknown_buffer_param().ident;
    let attrs = partial_object.attrs();
    let doc = if !attrs.iter().any(|attr| attr.path().is_ident("doc")) {
      let doc = format!(" Partial struct for the [`{}`]", self.name());
      quote! {
        #[doc = #doc]
      }
    } else {
      quote! {}
    };

    let fields = self.fields().iter().filter_map(|f| {
      let field_name = f.name();
      let attrs = f.attrs();
      let vis = f.vis();

      match f {
        GenericField::Skipped(skipped_field) => {
          let ty = skipped_field.ty();
          if !skipped_field.lifetime_params_usages().is_empty()
            || !skipped_field.type_params_usages().is_empty()
          {
            Some(quote! {
              #(#attrs)*
              #vis #field_name: ::core::marker::PhantomData<#ty>
            })
          } else {
            None
          }
        }
        GenericField::Tagged(concrete_tagged_field) => {
          let vis = concrete_tagged_field.vis();
          let field_ty = concrete_tagged_field.partial().optional_type();
          Some(quote! {
            #(#attrs)*
            #vis #field_name: #field_ty
          })
        }
      }
    });

    quote! {
      #doc
      #(#attrs)*
      #[allow(non_camel_case_types, clippy::type_complexity)]
      #vis struct #name #generics #where_clause {
        #ubfn: ::core::option::Option<#ubg>,
        #(#fields),*
      }
    }
  }

  pub(super) fn derive_partial_object(&self) -> proc_macro2::TokenStream {
    let partial_object = self.partial();
    let name = partial_object.name();
    let generics = partial_object.generics();

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

    let fields_accessors = self
      .fields()
      .iter()
      .filter_map(|f| f.try_unwrap_tagged_ref().ok())
      .map(|f| {
        let field_name = f.name();
        let ty = f.partial().ty();
        let copy = f.copy();

        optional_accessors(field_name, f.vis(), ty, copy)
      });

    let (ig, tg, where_clauses) = generics.split_for_impl();
    let flatten_state = derive_flatten_state(self.path_to_grost(), generics, name);

    let is_empty = self
      .fields()
      .iter()
      .filter_map(|f| f.try_unwrap_tagged_ref().ok())
      .map(|f| {
        let field_name = f.name();
        quote! {
          self.#field_name.is_none()
        }
      });
    let ubfn = &partial_object.unknown_buffer_field_name;
    let ubg = &self.unknown_buffer_param().ident;

    quote! {
      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl #ig ::core::default::Default for #name #tg #where_clauses {
        fn default() -> Self {
          Self::new()
        }
      }

      #flatten_state

      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl #ig #name #tg #where_clauses {
        /// Creates an empty partial struct.
        #[inline]
        pub const fn new() -> Self {
          Self {
            #ubfn: ::core::option::Option::None,
            #(#fields_init)*
          }
        }

        /// Returns `true` if the partial object is empty.
        #[inline]
        pub const fn is_empty(&self) -> bool {
          #(#is_empty)&&*
        }

        /// Returns a reference to the unknown buffer, which holds the unknown data when decoding.
        #[inline]
        pub const fn unknown_buffer(&self) -> ::core::option::Option<&#ubg> {
          self.#ubfn.as_ref()
        }

        /// Returns a mutable reference to the unknown buffer, which holds the unknown data when decoding.
        #[inline]
        pub const fn unknown_buffer_mut(&mut self) -> ::core::option::Option<&mut #ubg> {
          self.#ubfn.as_mut()
        }

        #(#fields_accessors)*
      }
    }
  }
}
