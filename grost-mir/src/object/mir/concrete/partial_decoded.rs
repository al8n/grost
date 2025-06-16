use quote::{format_ident, quote};
use syn::{
  Attribute, GenericParam, Generics, Ident, Type, WherePredicate, punctuated::Punctuated,
  token::Comma,
};

use crate::object::mir::{derive_flatten_state, optional_accessors};

use super::{ConcreteField, ConcreteObjectAst};

#[derive(Debug, Clone)]
pub struct ConcretePartialDecodedObject {
  name: Ident,
  ty: Type,
  attrs: Vec<Attribute>,
  generics: Generics,
  /// Extra constraints when deriving `Decode` trait for the partial decoded object.
  decode_constraints: Punctuated<WherePredicate, Comma>,
  /// Extra constraints when deriving `PartialDecode` trait for the partial decoded object.
  partial_decode_constraints: Punctuated<WherePredicate, Comma>,
  copy: bool,
  unknown_buffer_field_name: Ident,
}

impl ConcretePartialDecodedObject {
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

  /// Returns the extra constraints when deriving `Decode` trait for the partial decoded object.
  #[inline]
  pub const fn decode_constraints(&self) -> &Punctuated<WherePredicate, Comma> {
    &self.decode_constraints
  }

  /// Returns the extra constraints when deriving `PartialDecode` trait for the partial decoded object.
  #[inline]
  pub const fn partial_decode_constraints(&self) -> &Punctuated<WherePredicate, Comma> {
    &self.partial_decode_constraints
  }

  pub(super) fn from_ast<M, F>(
    object: &ConcreteObjectAst<M, F>,
    fields: &[ConcreteField<F>],
  ) -> darling::Result<Self> {
    let partial_decoded_object = object.partial_decoded();
    let unknown_buffer_param = object.unknown_buffer_param();
    let lifetime_param = object.lifetime_param();

    let object_generics = object.generics();
    let mut generics = Generics::default();
    let mut decode_constraints = Punctuated::new();
    let mut partial_decode_constraints = Punctuated::new();

    for lt in object_generics.lifetimes() {
      generics.params.push(GenericParam::Lifetime(lt.clone()));
    }
    generics
      .params
      .push(GenericParam::Lifetime(lifetime_param.clone()));

    for tp in object_generics.type_params() {
      generics.params.push(GenericParam::Type(tp.clone()));
    }
    generics
      .params
      .push(GenericParam::Type(unknown_buffer_param.clone()));

    for const_param in object_generics.const_params() {
      generics
        .params
        .push(GenericParam::Const(const_param.clone()));
    }

    if let Some(ref wc) = object_generics.where_clause {
      if !wc.predicates.is_empty() {
        generics
          .make_where_clause()
          .predicates
          .extend(wc.predicates.iter().cloned());
      }
    }

    for field in fields.iter().filter_map(|f| f.try_unwrap_tagged_ref().ok()) {
      let type_constraints = field.partial_decoded().type_constraints();
      if !type_constraints.is_empty() {
        generics
          .make_where_clause()
          .predicates
          .extend(type_constraints.iter().cloned());

        let ty = field.ty();
        let partial_decoded_ty = field.partial_decoded().ty();
        let wf = field.wire_format();
        let flavor_ty = object.flavor().ty();
        let path_to_grost = object.path_to_grost();
        let lt = &lifetime_param.lifetime;
        let ub = &unknown_buffer_param.ident;
        decode_constraints.push(syn::parse2(quote! {
          #ty: #path_to_grost::__private::Decode<#lt, #flavor_ty, #wf, #partial_decoded_ty, #ub>
        })?);
        partial_decode_constraints
          .push(syn::parse2(quote! {
            #ty: #path_to_grost::__private::PartialDecode<#lt, #flavor_ty, #wf, #partial_decoded_ty, #ub>
          })?);
        partial_decode_constraints.extend(field.selector().type_constraints().iter().cloned())
      }
    }

    let name = partial_decoded_object.name();
    let (_, tg, _) = generics.split_for_impl();
    let ty = syn::parse2(quote! {
      #name #tg
    })?;

    Ok(Self {
      name: name.clone(),
      ty,
      attrs: partial_decoded_object.attrs().to_vec(),
      copy: partial_decoded_object.copy(),
      generics,
      unknown_buffer_field_name: format_ident!("__grost_unknown_buffer__"),
      decode_constraints,
      partial_decode_constraints,
    })
  }
}

impl<M, F> super::ConcreteObject<M, F> {
  pub(super) fn derive_partial_decoded_object_defination(&self) -> proc_macro2::TokenStream {
    let partial_decoded = self.partial_decoded();
    let name = partial_decoded.name();
    let generics = partial_decoded.generics();
    let where_clause = generics.where_clause.as_ref();
    let attrs = partial_decoded.attrs();

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
        let ty = concrete_tagged_field.partial_decoded().optional_type();
        Some(quote! {
          #(#attrs)*
          #vis #name: #ty
        })
      }
    });

    let ubfn = &partial_decoded.unknown_buffer_field_name;
    let ubt = &self.unknown_buffer_param().ident;

    quote! {
      #doc
      #(#attrs)*
       #[allow(non_camel_case_types, clippy::type_complexity)]
      #vis struct #name #generics #where_clause {
        #ubfn: ::core::option::Option<#ubt>,
        #(#fields),*
      }
    }
  }

  pub(super) fn derive_partial_decoded_object(&self) -> proc_macro2::TokenStream {
    let partial_decoded_object = self.partial_decoded();
    let partial_decoded_object_ty = partial_decoded_object.ty();
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
        let ty = &f.partial_decoded().ty();
        let vis = f.vis();
        fields_accessors.push(optional_accessors(
          field_name,
          vis,
          ty,
          f.partial_decoded().copy(),
        ));
        is_empty.push(quote! {
          self.#field_name.is_none()
        });
      });

    let (ig, _, where_clauses) = partial_decoded_object.generics().split_for_impl();
    let ubfn = &partial_decoded_object.unknown_buffer_field_name;
    let ubg = &self.unknown_buffer_param().ident;
    let flatten_state = derive_flatten_state(
      &self.path_to_grost,
      partial_decoded_object.generics(),
      partial_decoded_object.name(),
    );

    quote! {
      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl #ig ::core::default::Default for #partial_decoded_object_ty #where_clauses
      {
        fn default() -> Self {
          Self::new()
        }
      }

      #flatten_state

      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl #ig #partial_decoded_object_ty #where_clauses
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
