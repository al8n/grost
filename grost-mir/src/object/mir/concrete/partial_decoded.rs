use std::sync::Arc;

use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{
  Attribute, GenericParam, Generics, Ident, Type, WherePredicate, punctuated::Punctuated,
  token::Comma,
};

use crate::{
  object::mir::{derive_flatten_state, optional_accessors},
  utils::grost_decode_trait_lifetime,
};

use super::{ConcreteField, ConcreteObjectAst};

#[derive(Debug, Clone)]
pub struct TyWith {
  lifetime: Option<TokenStream>,
  unknown_buffer: Option<TokenStream>,
}

#[derive(derive_more::Debug, Clone)]
pub struct ConcretePartialRefObject {
  name: Ident,
  ty: Type,
  attrs: Vec<Attribute>,
  generics: Generics,
  /// Extra constraints when deriving `Decode` trait for the partial decoded object.
  decode_generics: Generics,
  /// Extra constraints when deriving `Transform` trait for the partial decoded object.
  transform_generics: Generics,
  /// Extra constraints when deriving `Transform` trait for the partial decoded object.
  partial_transform_generics: Generics,
  /// The trait type which applies the cooresponding generics to the `Decode` trait.
  #[debug(skip)]
  apply_type_generics: Arc<dyn Fn(TyWith) -> syn::Result<Type> + 'static>,
  /// The trait type which applies the cooresponding generics to the `Decode` trait.
  #[debug(skip)]
  applied_decode_trait: Arc<dyn Fn(TokenStream) -> syn::Result<Type> + 'static>,
  copy: bool,
  pub(super) unknown_buffer_field_name: Ident,
  pub(super) read_buffer_field_name: Ident,
}

impl ConcretePartialRefObject {
  /// Returns the name of the partial decoded object.
  #[inline]
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  /// Returns the type of the partial decoded object.
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
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

  /// Returns the generics when deriving `Decode` trait for the partial decoded object.
  #[inline]
  pub const fn decode_generics(&self) -> &Generics {
    &self.decode_generics
  }

  /// Returns the generics when deriving `Transform` trait for the partial decoded object.
  #[inline]
  pub const fn transform_generics(&self) -> &Generics {
    &self.transform_generics
  }

  /// Returns the generics when deriving `PartialTransform` trait for the partial decoded object.
  #[inline]
  pub const fn partial_transform_generics(&self) -> &Generics {
    &self.partial_transform_generics
  }

  pub(super) fn applied_decode_trait(&self, ty: impl ToTokens) -> syn::Result<Type> {
    (self.applied_decode_trait)(quote! { #ty })
  }

  pub(super) fn type_with<L, U>(
    &self,
    lifetime: Option<L>,
    unknown_buffer: Option<U>,
  ) -> syn::Result<Type>
  where
    L: ToTokens,
    U: ToTokens,
  {
    (self.apply_type_generics)(TyWith {
      lifetime: lifetime.map(|lt| quote! { #lt }),
      unknown_buffer: unknown_buffer.map(|ub| quote! { #ub }),
    })
  }

  pub(super) fn from_ast<M, F>(
    object: &ConcreteObjectAst<M, F>,
    fields: &[ConcreteField<F>],
  ) -> darling::Result<Self> {
    let partial_ref_object = object.partial_ref();
    let unknown_buffer_param = object.unknown_buffer_param();
    let lifetime_param = object.lifetime_param();
    let read_buf_param = object.read_buffer_param();

    let object_generics = object.generics();
    let mut generics = Generics::default();
    let mut decode_constraints = Punctuated::<WherePredicate, Comma>::new();
    let mut transform_constraints = Punctuated::<WherePredicate, Comma>::new();
    let mut partial_transform_constraints = Punctuated::<WherePredicate, Comma>::new();

    generics
      .params
      .extend(object_generics.lifetimes().cloned().map(GenericParam::from));
    generics
      .params
      .push(GenericParam::Lifetime(lifetime_param.clone()));
    generics.params.extend(
      object_generics
        .type_params()
        .cloned()
        .map(GenericParam::Type),
    );
    generics
      .params
      .push(GenericParam::Type(read_buf_param.clone()));
    generics
      .params
      .push(GenericParam::Type(unknown_buffer_param.clone()));
    generics.params.extend(
      object_generics
        .const_params()
        .cloned()
        .map(GenericParam::from),
    );

    if let Some(ref wc) = object_generics.where_clause {
      if !wc.predicates.is_empty() {
        generics
          .make_where_clause()
          .predicates
          .extend(wc.predicates.iter().cloned());
      }
    }

    let flavor_ty = object.flavor().ty();
    let path_to_grost = object.path_to_grost();
    let decode_lt = grost_decode_trait_lifetime();
    let ub = &unknown_buffer_param.ident;
    let rb = &read_buf_param.ident;
    let wf = object.flavor().wire_format();
    for field in fields.iter().filter_map(|f| f.try_unwrap_tagged_ref().ok()) {
      let type_constraints = field.partial_ref().type_constraints();
      if !field.type_params_usages().is_empty() || !field.lifetime_params_usages().is_empty() {
        generics
          .make_where_clause()
          .predicates
          .extend(type_constraints.iter().cloned());

        let ty = field.ty();
        let partial_ref_ty = field.partial_ref().ty();
        let wf = field.wire_format();

        decode_constraints.push(syn::parse2(quote! {
          #ty: #path_to_grost::__private::decode::Decode<#decode_lt, #flavor_ty, #wf, #partial_ref_ty, #rb, #ub>
        })?);
        transform_constraints.push(syn::parse2(quote! {
          #ty: #path_to_grost::__private::convert::Transform<#flavor_ty, #wf, #partial_ref_ty>
        })?);
        partial_transform_constraints.push(syn::parse2(quote! {
          #ty: #path_to_grost::__private::convert::PartialTransform<#flavor_ty, #wf, #partial_ref_ty>
        })?);
        partial_transform_constraints.push(syn::parse2(quote! {
          #partial_ref_ty: #path_to_grost::__private::selection::Selectable<
            #flavor_ty,
            Selector = <#ty as #path_to_grost::__private::selection::Selectable<#flavor_ty>>::Selector,
          >
        })?);
      }

      if !field.selector().type_constraints().is_empty() {
        partial_transform_constraints.extend(field.selector().type_constraints().iter().cloned());
      }
    }

    let name = partial_ref_object.name();
    let (_, tg, _) = generics.split_for_impl();
    let ty = syn::parse2(quote! {
      #name #tg
    })?;

    let decode_generics = {
      let decode_lifetime = grost_decode_trait_lifetime();
      let mut output = generics.clone();
      output
        .params
        .push(GenericParam::Lifetime(decode_lifetime.clone()));
      output
        .make_where_clause()
        .predicates
        .extend(decode_constraints);
      generics
        .lifetimes()
        .filter(|lt| lt.lifetime.ident.ne("static"))
        .try_for_each(|lt| {
          syn::parse2(quote! {
            #decode_lifetime: #lt
          })
          .map(|pred| output.make_where_clause().predicates.push(pred))
        })?;
      output
    };
    let transform_generics = {
      let mut output = generics.clone();
      let lt = &object.lifetime_param().lifetime;
      output
        .make_where_clause()
        .predicates
        .extend(transform_constraints);
      output.make_where_clause()
        .predicates
        .extend([
          syn::parse2::<WherePredicate>(quote! {
            #rb: #path_to_grost::__private::buffer::ReadBuf + #lt
          })?,
          syn::parse2::<WherePredicate>(quote! {
            #ub: #path_to_grost::__private::buffer::Buffer<<#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Unknown<#rb>> + #lt
          })?,
        ]);
      output
    };
    let partial_transform_generics = {
      let mut output = generics.clone();
      let lt = &object.lifetime_param().lifetime;
      output
        .make_where_clause()
        .predicates
        .extend(partial_transform_constraints);
      output.make_where_clause()
        .predicates
        .extend([
          syn::parse2::<WherePredicate>(quote! {
            #rb: #path_to_grost::__private::buffer::ReadBuf + #lt
          })?,
          syn::parse2::<WherePredicate>(quote! {
            #ub: #path_to_grost::__private::buffer::Buffer<<#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Unknown<#rb>> + #lt
          })?,
        ]);
      output
    };

    Ok(Self {
      name: name.clone(),
      apply_type_generics: {
        let original_generics = object_generics.clone();
        let lifetime_param = lifetime_param.clone();
        let unknown_buffer_param = unknown_buffer_param.clone();
        let name = name.clone();
        Arc::new(move |ty_with: TyWith| {
          let lifetime = ty_with.lifetime.unwrap_or_else(|| {
            quote! { #lifetime_param }
          });
          let unknown_buffer = ty_with.unknown_buffer.unwrap_or_else(|| {
            quote! { #unknown_buffer_param }
          });
          let mut output = Generics::default();
          output.params.extend(
            original_generics
              .lifetimes()
              .cloned()
              .map(GenericParam::from),
          );
          output
            .params
            .push(GenericParam::Lifetime(syn::parse2(lifetime)?));
          output.params.extend(
            original_generics
              .type_params()
              .cloned()
              .map(GenericParam::Type),
          );
          output
            .params
            .push(GenericParam::Type(syn::parse2(unknown_buffer)?));
          output.params.extend(
            original_generics
              .const_params()
              .cloned()
              .map(GenericParam::from),
          );

          let (_, tg, _) = output.split_for_impl();
          syn::parse2(quote! {
            #name #tg
          })
        })
      },
      applied_decode_trait: {
        let path_to_grost = path_to_grost.clone();
        let flavor_ty = flavor_ty.clone();
        let wf = wf.clone();
        let lt = grost_decode_trait_lifetime();
        let ub = ub.clone();
        let rb = rb.clone();
        Arc::new(move |ty| {
          syn::parse2(quote! {
            #path_to_grost::__private::decode::Decode<#lt, #flavor_ty, #wf, #ty, #rb, #ub>
          })
        })
      },
      ty,
      attrs: partial_ref_object.attrs().to_vec(),
      copy: partial_ref_object.copy(),
      transform_generics,
      partial_transform_generics,
      decode_generics,
      generics,
      read_buffer_field_name: format_ident!("__grost_read_buffer__"),
      unknown_buffer_field_name: format_ident!("__grost_unknown_buffer__"),
    })
  }
}

impl<M, F> super::ConcreteObject<M, F> {
  pub(super) fn derive_partial_ref_object_defination(&self) -> proc_macro2::TokenStream {
    let partial_ref = self.partial_ref();
    let name = partial_ref.name();
    let generics = partial_ref.generics();
    let where_clause = generics.where_clause.as_ref();
    let attrs = partial_ref.attrs();

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

    let vis = self.vis();
    let fields = self.fields().iter().filter_map(|f| match f {
      ConcreteField::Skipped(skipped_field) => {
        let name = skipped_field.name();
        let ty = skipped_field.ty();

        if !skipped_field.lifetime_params_usages().is_empty()
          || !skipped_field.type_params_usages().is_empty()
        {
          Some(quote! {
            #name: ::core::marker::PhantomData<#ty>
          })
        } else {
          None
        }
      }
      ConcreteField::Tagged(concrete_tagged_field) => {
        let attrs = concrete_tagged_field.attrs();
        let vis = concrete_tagged_field.vis();
        let name = concrete_tagged_field.name();
        let ty = concrete_tagged_field.partial_ref().optional_type();
        Some(quote! {
          #(#attrs)*
          #vis #name: #ty
        })
      }
    });

    let ubfn = &partial_ref.unknown_buffer_field_name;
    let ubt = &self.unknown_buffer_param().ident;
    let rbfn = &partial_ref.read_buffer_field_name;
    let rbg = &self.read_buffer_param().ident;

    quote! {
      #doc
      #(#attrs)*
       #[allow(non_camel_case_types, clippy::type_complexity)]
      #vis struct #name #generics #where_clause {
        #ubfn: ::core::option::Option<#ubt>,
        #rbfn: ::core::option::Option<#rbg>,
        #(#fields),*
      }
    }
  }

  pub(super) fn derive_partial_ref_object(&self) -> proc_macro2::TokenStream {
    let partial_ref_object = self.partial_ref();
    let partial_ref_object_ty = partial_ref_object.ty();
    let fields_init = self.fields().iter().filter_map(|f| {
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
        fields_accessors.push(optional_accessors(
          field_name,
          vis,
          ty,
          f.partial_ref().copy(),
        ));
        is_empty.push(quote! {
          self.#field_name.is_none()
        });
      });

    let (ig, _, where_clauses) = partial_ref_object.generics().split_for_impl();
    let ubfn = &partial_ref_object.unknown_buffer_field_name;
    let ubg = &self.unknown_buffer_param().ident;
    let rbfn = &partial_ref_object.read_buffer_field_name;
    let rbg = &self.read_buffer_param().ident;
    let flatten_state = derive_flatten_state(
      &self.path_to_grost,
      partial_ref_object.generics(),
      partial_ref_object.name(),
    );

    quote! {
      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl #ig ::core::default::Default for #partial_ref_object_ty #where_clauses
      {
        fn default() -> Self {
          Self::new()
        }
      }

      #flatten_state

      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl #ig #partial_ref_object_ty #where_clauses
      {
        /// Creates an empty partial struct.
        #[inline]
        pub const fn new() -> Self {
          Self {
            #(#fields_init)*
            #ubfn: ::core::option::Option::None,
            #rbfn: ::core::option::Option::None,
          }
        }

        /// Returns `true` if the partial struct is empty, which means all fields are `None`.
        #[inline]
        pub const fn is_empty(&self) -> bool {
          self.#ubfn.is_none() && #(#is_empty)&&*
        }

        /// Returns the original encoded type of the partial decoded object.
        #[inline]
        pub const fn raw(&self) -> ::core::option::Option<&#rbg> {
          self.#rbfn.as_ref()
        }

        /// Returns a reference to the unknown buffer, which holds the unknown data when decoding.
        #[inline]
        pub const fn unknown_buffer(&self) -> ::core::option::Option<&#ubg> {
          self.#ubfn.as_ref()
        }

        // TODO(al8n): the following fns may lead to name conflicts if the struct has field whose name is unknown_buffer
        /// Returns a mutable reference to the unknown buffer, which holds the unknown data when decoding.
        #[inline]
        pub const fn unknown_buffer_mut(&mut self) -> ::core::option::Option<&mut #ubg> {
         self.#ubfn.as_mut()
        }

        /// Takes the unknown buffer out if the unknown buffer is not `None`.
        #[inline]
        pub const fn take_unknown_buffer(&mut self) -> ::core::option::Option<#ubg> {
          self.#ubfn.take()
        }

        /// Set the value of unknown buffer
        #[inline]
        pub fn set_unknown_buffer(&mut self, buffer: #ubg) -> &mut Self {
          self.#ubfn = ::core::option::Option::Some(buffer);
          self
        }

        /// Clears the unknown buffer.
        #[inline]
        pub fn clear_unknown_buffer(&mut self) -> &mut Self {
          self.#ubfn = ::core::option::Option::None;
          self
        }

        /// Set the value of unknown buffer
        #[inline]
        pub fn with_unknown_buffer(mut self, buffer: #ubg) -> Self {
          self.#ubfn = ::core::option::Option::Some(buffer);
          self
        }

        /// Clears the unknown buffer.
        #[inline]
        pub fn without_unknown_buffer(mut self) -> Self {
          self.#ubfn = ::core::option::Option::None;
          self
        }

        #(#fields_accessors)*
      }
    }
  }
}
