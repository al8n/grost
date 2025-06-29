use std::rc::Rc;

use proc_macro2::TokenStream;
use syn::{
  Attribute, GenericParam, Generics, Ident, Type, WherePredicate, punctuated::Punctuated,
  token::Comma,
};

use quote::{ToTokens, quote};

use crate::object::{
  ObjectConvertOptions,
  ast::{derive_flatten_state, optional_accessors},
  meta::concrete::PartialObjectFromMeta,
};

use super::{Field, RawObject};

impl PartialObjectFromMeta {
  pub(super) fn finalize(self) -> PartialObjectOptions {
    PartialObjectOptions {
      name: self.name,
      attrs: self.attrs,
      transform: self.transform.into(),
      partial_transform: self.partial_transform.into(),
      copy: self.copy,
    }
  }
}

#[derive(Debug, Clone)]
pub struct PartialObjectOptions {
  pub(super) name: Option<Ident>,
  pub(super) attrs: Vec<Attribute>,
  pub(super) transform: ObjectConvertOptions,
  pub(super) partial_transform: ObjectConvertOptions,
  pub(super) copy: bool,
}

impl PartialObjectOptions {
  /// Returns the name of the partial object
  pub(crate) const fn name(&self) -> Option<&Ident> {
    self.name.as_ref()
  }

  /// Returns the attributes of the partial object
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }
}

#[derive(derive_more::Debug, Clone)]
pub struct PartialObject {
  name: Ident,
  ty: Type,
  generics: Generics,
  /// Extra constraints when deriving `Decode` trait for the partial decoded object.
  decode_generics: Generics,
  pub(super) transform_from_partial_ref_generics: Generics,
  pub(super) partial_transform_from_partial_ref_generics: Generics,
  pub(super) partial_transform_generics: Generics,
  /// The trait type which applies the cooresponding generics to the `Decode` trait.
  #[debug(skip)]
  applied_decode_trait: Rc<dyn Fn(TokenStream) -> syn::Result<Type> + 'static>,
  attrs: Vec<Attribute>,
  copy: bool,
}

impl PartialObject {
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

  /// Returns `true` if the partial object is copyable, `false` otherwise.
  #[inline]
  pub const fn copy(&self) -> bool {
    self.copy
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

  pub(super) fn applied_decode_trait(&self, ty: impl ToTokens) -> syn::Result<Type> {
    (self.applied_decode_trait)(quote! { #ty })
  }

  pub(super) fn from_options<T, S, M>(
    object: &mut RawObject<T, S, M>,
    fields: &[Field],
  ) -> darling::Result<Self> {
    let partial_object_name = object.partial_name();
    let unknown_buffer_param = &object.unknown_buffer_param;
    let read_buffer_param = &object.read_buffer_param;

    let mut generics = object.generics.clone();
    let mut decode_constraints: Punctuated<WherePredicate, Comma> = Punctuated::new();
    let mut transform_ref_constraints: Punctuated<WherePredicate, Comma> = Punctuated::new();
    let mut partial_transform_ref_constraints: Punctuated<WherePredicate, Comma> =
      Punctuated::new();
    let mut partial_transform_constraints: Punctuated<WherePredicate, Comma> = Punctuated::new();

    let flavor_ty = &object.flavor_type;
    let path_to_grost = &object.path_to_grost;
    let wf = &object.wire_format;
    let ub = &unknown_buffer_param.ident;
    let rb = &read_buffer_param.ident;
    let lt = &object.lifetime_param.lifetime;

    fields
      .iter()
      .filter_map(|f| f.try_unwrap_tagged_ref().ok())
      .try_for_each(|f| {
        if f.uses_generics() {
          let ty = f.ty();
          let partial_ref_ty = f.partial_ref().ty();
          let wf = f.wire_format();
          let partial = f.partial();
          let partial_ty = partial.ty();

          generics
            .make_where_clause()
            .predicates
            .extend(partial.type_constraints().iter().cloned());

          transform_ref_constraints.extend(partial.transform_ref_constraints().iter().cloned());
          partial_transform_ref_constraints
            .extend(partial.partial_transform_ref_constraints().iter().cloned());
          partial_transform_constraints
            .extend(partial.partial_transform_constraints().iter().cloned());

          decode_constraints.extend(f.partial_ref().type_constraints().iter().cloned());
          decode_constraints.push(syn::parse2::<WherePredicate>(quote! {
            #ty: #path_to_grost::__private::decode::Decode<#lt, #flavor_ty, #wf, #partial_ref_ty, #rb, #ub>
          })?);
          decode_constraints.push(syn::parse2::<WherePredicate>(quote! {
            #ty: #path_to_grost::__private::convert::Transform<#partial_ref_ty, #partial_ty, #wf, #flavor_ty>
          })?);
        }

        darling::Result::Ok(())
      })?;

    let (_, tg, _) = generics.split_for_impl();
    let ty = syn::parse2(quote! {
      #partial_object_name #tg
    })?;
    let copy = object.copy;

    Ok(Self {
      name: partial_object_name,
      applied_decode_trait: {
        let path_to_grost = path_to_grost.clone();
        let flavor_ty = flavor_ty.clone();
        let wf = wf.clone();
        let lt = lt.clone();
        let ub = ub.clone();
        let rb = rb.clone();
        Rc::new(move |ty| {
          syn::parse2(quote! {
            #path_to_grost::__private::decode::Decode<#lt, #flavor_ty, #wf, #ty, #rb, #ub>
          })
        })
      },
      ty,
      decode_generics: {
        let lt = object.lifetime_param.clone();
        let mut output = Generics::default();
        output
          .params
          .extend(generics.lifetimes().cloned().map(GenericParam::from));
        output.params.push(GenericParam::Lifetime(lt.clone()));
        output
          .params
          .extend(generics.type_params().cloned().map(GenericParam::from));
        output
          .params
          .push(GenericParam::Type(read_buffer_param.clone()));
        output
          .params
          .push(GenericParam::Type(unknown_buffer_param.clone()));
        output
          .params
          .extend(generics.const_params().cloned().map(GenericParam::from));
        output.where_clause = generics.where_clause.clone();
        let wc = output.make_where_clause();
        wc.predicates.extend(decode_constraints);

        generics
          .lifetimes()
          .filter(|lt| lt.lifetime.ident.ne("static"))
          .try_for_each(|ltp| {
            let ident = &ltp.lifetime;
            syn::parse2(quote! {
              #lt: #ident
            })
            .map(|pred| wc.predicates.push(pred))
          })?;
        output
      },
      transform_from_partial_ref_generics: {
        let lt = object.lifetime_param.clone();
        let mut output = Generics::default();
        output
          .params
          .extend(generics.lifetimes().cloned().map(GenericParam::from));
        output.params.push(GenericParam::Lifetime(lt.clone()));
        output
          .params
          .extend(generics.type_params().cloned().map(GenericParam::from));
        output
          .params
          .push(GenericParam::Type(read_buffer_param.clone()));
        output
          .params
          .push(GenericParam::Type(unknown_buffer_param.clone()));
        output
          .params
          .extend(generics.const_params().cloned().map(GenericParam::from));
        output.where_clause = generics.where_clause.clone();
        let wc = output.make_where_clause();
        wc.predicates.extend(transform_ref_constraints);
        wc.predicates.push(syn::parse2(quote! {
          #rb: #path_to_grost::__private::buffer::ReadBuf + #lt
        })?);
        wc.predicates.push(syn::parse2(quote! {
          #ub: #path_to_grost::__private::buffer::Buffer<<#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Unknown<#rb>> + #lt
        })?);
        generics
          .lifetimes()
          .filter(|lt| lt.lifetime.ident.ne("static"))
          .try_for_each(|ltp| {
            let ident = &ltp.lifetime;
            syn::parse2(quote! {
              #lt: #ident
            })
            .map(|pred| wc.predicates.push(pred))
          })?;
        output
      },
      partial_transform_from_partial_ref_generics: {
        let lt = object.lifetime_param.clone();
        let mut output = Generics::default();
        output
          .params
          .extend(generics.lifetimes().cloned().map(GenericParam::from));
        output.params.push(GenericParam::Lifetime(lt.clone()));
        output
          .params
          .extend(generics.type_params().cloned().map(GenericParam::from));
        output
          .params
          .push(GenericParam::Type(read_buffer_param.clone()));
        output
          .params
          .push(GenericParam::Type(unknown_buffer_param.clone()));
        output
          .params
          .extend(generics.const_params().cloned().map(GenericParam::from));
        output.where_clause = generics.where_clause.clone();
        let wc = output.make_where_clause();
        wc.predicates.extend(partial_transform_ref_constraints);
        wc.predicates.push(syn::parse2(quote! {
          #rb: #path_to_grost::__private::buffer::ReadBuf + #lt
        })?);
        wc.predicates.push(syn::parse2(quote! {
          #ub: #path_to_grost::__private::buffer::Buffer<<#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Unknown<#rb>> + #lt
        })?);
        generics
          .lifetimes()
          .filter(|lt| lt.lifetime.ident.ne("static"))
          .try_for_each(|ltp| {
            let ident = &ltp.lifetime;
            syn::parse2(quote! {
              #lt: #ident
            })
            .map(|pred| wc.predicates.push(pred))
          })?;
        output
      },
      partial_transform_generics: {
        let mut output = generics.clone();
        output
          .make_where_clause()
          .predicates
          .extend(partial_transform_constraints);
        output
      },
      generics,
      attrs: core::mem::take(&mut object.partial.attrs),
      copy,
    })
  }

  pub(super) fn derive_defination<T, S, M>(
    &self,
    object: &super::Object<T, S, M>,
  ) -> darling::Result<proc_macro2::TokenStream> {
    let name = self.name();
    let vis = object.vis();
    let generics = self.generics();
    let (_, _, where_clause) = generics.split_for_impl();
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
        Field::Skipped(skipped_field) => {
          let ty = skipped_field.ty();
          if skipped_field.use_generics() {
            Some(quote! {
              #(#attrs)*
              #vis #field_name: ::core::marker::PhantomData<#ty>
            })
          } else {
            None
          }
        }
        Field::Tagged(concrete_tagged_field) => {
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
        #(#fields),*
      }
    })
  }

  pub(super) fn derive<T, S, M>(
    &self,
    object: &super::Object<T, S, M>,
  ) -> darling::Result<proc_macro2::TokenStream> {
    let name = self.name();
    let generics = self.generics();

    let fields_init = object.fields().iter().filter_map(|f| {
      let field_name = f.name();

      match f {
        Field::Skipped(skipped_field) => {
          if skipped_field.use_generics() {
            Some(quote! {
              #field_name: ::core::marker::PhantomData,
            })
          } else {
            None
          }
        }
        Field::Tagged(_) => Some(quote! {
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

        optional_accessors(
          object.path_to_grost(),
          field_name,
          f.vis(),
          ty,
          f.label(),
          copy,
        )
      })
      .collect::<darling::Result<Vec<_>>>()?;

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
            #(#fields_init)*
          }
        }

        /// Returns `true` if the partial object is empty.
        #[inline]
        pub const fn is_empty(&self) -> bool {
          #(#is_empty)&&*
        }

        #(#fields_accessors)*
      }
    })
  }
}
