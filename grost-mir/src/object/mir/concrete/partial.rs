use std::sync::Arc;

use proc_macro2::TokenStream;
use syn::{punctuated::Punctuated, token::Comma, Attribute, GenericParam, Generics, Ident, Type, WherePredicate};

use quote::{format_ident, quote, ToTokens};

use crate::object::mir::{derive_flatten_state, optional_accessors};

use super::{ConcreteField, ConcreteObjectAst};

#[derive(derive_more::Debug, Clone)]
pub struct ConcretePartialObject {
  name: Ident,
  ty: Type,
  /// The trait type which applies the cooresponding generics to the `Decode` trait.
  #[debug(skip)]
  applied_decode_trait: Arc<dyn Fn(TokenStream) -> syn::Result<Type> + 'static>,
  /// The trait type which applies the cooresponding generics to the `PartialDecode` trait.
  #[debug(skip)]
  applied_partial_decode_trait: Arc<dyn Fn(TokenStream) -> syn::Result<Type> + 'static>,
  generics: Generics,
  /// Extra constraints when deriving `Decode` trait for the partial decoded object.
  decode_generics: Generics,
  /// Extra constraints when deriving `PartialDecode` trait for the partial decoded object.
  partial_decode_generics: Generics,
  attrs: Vec<Attribute>,
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

  /// Returns the generics of the partial object.
  #[inline]
  pub const fn generics(&self) -> &Generics {
    &self.generics
  }

  /// Returns the generics when deriving `Decode` trait for the partial decoded object.
  #[inline]
  pub const fn decode_generics(&self) -> &Generics {
    &self.decode_generics
  }

  /// Returns the generics when deriving `PartialDecode` trait for the partial decoded object.
  #[inline]
  pub const fn partial_decode_generics(&self) -> &Generics {
    &self.partial_decode_generics
  }

  /// Returns `true` if the partial object is copyable, `false` otherwise.
  #[inline]
  pub const fn copy(&self) -> bool {
    self.copy
  }

  pub(super) fn applied_decode_trait(&self, ty: impl ToTokens) -> syn::Result<Type> {
    (self.applied_decode_trait)(quote! { #ty })
  }

  pub(super) fn applied_partial_decode_trait(&self, ty: impl ToTokens) -> syn::Result<Type> {
    (self.applied_partial_decode_trait)(quote! { #ty })
  }

  pub(super) fn from_ast<M, F>(
    object: &ConcreteObjectAst<M, F>,
    fields: &[ConcreteField<F>],
  ) -> darling::Result<Self> {
    let partial_object = object.partial();
    let unknown_buffer_param = object.unknown_buffer_param();

    let mut generics = object.generics().clone();
    let mut decode_constraints: Punctuated<WherePredicate, Comma> = Punctuated::new();
    let mut partial_decode_constraints: Punctuated<WherePredicate, Comma> = Punctuated::new();
    generics
      .params
      .push(GenericParam::Type(unknown_buffer_param.clone()));

    let flavor_ty = object.flavor().ty();
    let path_to_grost = object.path_to_grost();
    let wf = object.flavor().wire_format();
    let ub = &unknown_buffer_param.ident;
    let lt = &object.lifetime_param().lifetime;

    fields
      .iter()
      .filter_map(|f| f.try_unwrap_tagged_ref().ok())
      .try_for_each(|f| {
        if !f.partial().type_constraints().is_empty() {
          generics
            .make_where_clause()
            .predicates
            .extend(f.partial().type_constraints().iter().cloned());

          let ty = f.ty();
          let partial_ty = f.partial().ty();
          let wf = f.wire_format();

          decode_constraints.push(syn::parse2::<WherePredicate>(quote! {
            #ty: #path_to_grost::__private::Decode<#lt, #flavor_ty, #wf, #partial_ty, #ub>
          })?);
          partial_decode_constraints
            .push(syn::parse2::<WherePredicate>(quote! {
              #ty: #path_to_grost::__private::PartialDecode<#lt, #flavor_ty, #wf, #partial_ty, #ub>
            })?);
          partial_decode_constraints.extend(f.selector().type_constraints().iter().cloned())
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
      applied_decode_trait: {
        let path_to_grost = path_to_grost.clone();
        let flavor_ty = flavor_ty.clone();
        let wf = wf.clone();
        let lt = lt.clone();
        let ub = ub.clone();
        Arc::new(move |ty| syn::parse2(quote! {
          #path_to_grost::__private::Decode<#lt, #flavor_ty, #wf, #ty, #ub>
        }))
      },
      applied_partial_decode_trait: {
        let path_to_grost = path_to_grost.clone();
        let flavor_ty = flavor_ty.clone();
        let wf = wf.clone();
        let lt = lt.clone();
        let ub = ub.clone();
        Arc::new(move |ty| syn::parse2(quote! {
          #path_to_grost::__private::PartialDecode<#lt, #flavor_ty, #wf, #ty, #ub>
        }))
      },
      ty,
      decode_generics: {
        let mut output = Generics::default();
        output.params.extend(generics.lifetimes().cloned().map(GenericParam::from));
        output.params.push(GenericParam::Lifetime(object.lifetime_param().clone()));
        output.params.extend(generics.type_params().cloned().map(GenericParam::from));
        output.params.extend(generics.const_params().cloned().map(GenericParam::from));
        output.where_clause = generics.where_clause.clone();
        output.make_where_clause().predicates.extend(decode_constraints);
        output
      },
      partial_decode_generics: {
        let mut output = Generics::default();
        output.params.extend(generics.lifetimes().cloned().map(GenericParam::from));
        output.params.push(GenericParam::Lifetime(object.lifetime_param().clone()));
        output.params.extend(generics.type_params().cloned().map(GenericParam::from));
        output.params.extend(generics.const_params().cloned().map(GenericParam::from));
        output.where_clause = generics.where_clause.clone();
        output.make_where_clause().predicates.extend(partial_decode_constraints);
        output
      },
      generics,
      attrs: partial_object.attrs().to_vec(),
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
    let ubg = &object.unknown_buffer_param().ident;
    let attrs = self.attrs();
    let doc = if !attrs.iter().any(|attr| attr.path().is_ident("doc")) {
      let doc = format!(" Partial struct for the [`{}`]", self.name());
      quote! {
        #[doc = #doc]
      }
    } else {
      quote! {}
    };

    let fields = object.fields().iter().filter_map(|f| {
      let field_name = f.name();
      let attrs = f.attrs();
      let vis = f.vis();

      match f {
        ConcreteField::Skipped(skipped_field) => {
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
        ConcreteField::Tagged(concrete_tagged_field) => {
          let vis = concrete_tagged_field.vis();
          let field_ty = concrete_tagged_field.partial().optional_type();
          Some(quote! {
            #(#attrs)*
            #vis #field_name: #field_ty
          })
        }
      }
    });

    Ok(quote! {
      #doc
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

    let fields_init = object.fields().iter().filter_map(|f| {
      let field_name = f.name();

      match f {
        ConcreteField::Skipped(skipped_field) => {
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
        ConcreteField::Tagged(_) => Some(quote! {
          #field_name: ::core::option::Option::None,
        }),
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

    let is_empty = object
      .fields()
      .iter()
      .filter_map(|f| f.try_unwrap_tagged_ref().ok())
      .map(|f| {
        let field_name = f.name();
        quote! {
          self.#field_name.is_none()
        }
      });
    let ubfn = &self.unknown_buffer_field_name;
    let ubg = &object.unknown_buffer_param().ident;

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
