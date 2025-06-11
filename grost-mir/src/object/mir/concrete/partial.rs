use syn::{Attribute, GenericParam, Generics, Ident, Type, TypeParam};

use quote::{format_ident, quote};

use crate::object::mir::{derive_flatten_state, optional_accessors};

use super::{ConcreteField, ConcreteObjectAst};

#[derive(Debug, Clone)]
pub struct ConcretePartialObject {
  name: Ident,
  ty: Type,
  generics: Generics,
  attrs: Vec<Attribute>,
  unknown_buffer_param: TypeParam,
  unknown_buffer_field_name: Ident,
  copy: bool,
}

impl ConcretePartialObject {
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

  /// Returns the generic unknown buffer type parameter of the partial object.
  #[inline]
  pub const fn unknown_buffer(&self) -> &TypeParam {
    &self.unknown_buffer_param
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
    object: &ConcreteObjectAst<M, F>,
    fields: &[ConcreteField<F>],
  ) -> darling::Result<Self> {
    let partial_object = object.partial();
    let unknown_buffer_param = partial_object.unknown_buffer().clone();

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
      unknown_buffer_param: partial_object.unknown_buffer().clone(),
      unknown_buffer_field_name: format_ident!("__grost_unknown_buffer__"),
      copy,
    })
  }

  pub(super) fn derive_defination<M, F>(
    &self,
    object: &super::ConcreteObject<M, F>,
  ) -> darling::Result<proc_macro2::TokenStream> {
    let name = self.name();
    let vis = object.vis();
    let generics = self.generics();
    let (_, _, where_clause) = generics.split_for_impl();
    let ubfn = &self.unknown_buffer_field_name;
    let ubg = &self.unknown_buffer().ident;
    let attrs = self.attrs();

    let fields = object.fields().iter().map(|f| {
      let field_name = f.name();
      let attrs = f.attrs();
      let vis = f.vis();

      match f {
        ConcreteField::Skipped(skipped_field) => {
          let ty = skipped_field.ty();
          quote! {
            #(#attrs)*
            #vis #field_name: ::core::option::Option<#ty>
          }
        }
        ConcreteField::Tagged(concrete_tagged_field) => {
          let vis = concrete_tagged_field.vis();
          let field_ty = concrete_tagged_field.partial().optional_type();
          quote! {
            #(#attrs)*
            #vis #field_name: #field_ty
          }
        }
      }
    });

    Ok(quote! {
      #(#attrs)*
      #[allow(non_camel_case_types, clippy::type_complexity)]
      #vis struct #name #generics #where_clause {
        #ubfn: ::core::option::Option<#ubg>,
        #(#fields),*
      }
    })
  }

  pub(super) fn derive<M, F>(
    &self,
    object: &super::ConcreteObject<M, F>,
  ) -> darling::Result<proc_macro2::TokenStream> {
    let name = self.name();
    let generics = self.generics();

    let fields_init = object.fields().iter().map(|f| {
      let field_name = f.name();
      quote! {
        #field_name: ::core::option::Option::None,
      }
    });

    let fields_accessors = object
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
    let flatten_state = derive_flatten_state(object.path_to_grost(), generics, name);

    let is_empty = object.fields().iter().map(|f| {
      let field_name = f.name();
      quote! {
        self.#field_name.is_none()
      }
    });
    let ubfn = &self.unknown_buffer_field_name;
    let ubg = &self.unknown_buffer().ident;

    Ok(quote! {
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
    })
  }
}
