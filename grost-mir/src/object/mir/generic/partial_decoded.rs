use quote::format_ident;
use quote::quote;
use syn::{Attribute, GenericParam, Generics, Ident};

use super::{super::super::ast::GenericObject as GenericObjectAst, GenericField};

#[derive(Debug, Clone)]
pub struct GenericPartialDecodedObject {
  name: Ident,
  attrs: Vec<Attribute>,
  generics: Generics,
  copy: bool,
  unknown_buffer_field_name: Ident,
}

impl GenericPartialDecodedObject {
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
    let partial_decoded_object = &object.partial_decoded;
    let partial_decoded_object_name = partial_decoded_object.name().clone();
    let flavor_param = &object.flavor_param;
    let unknown_buffer_param = &object.unknown_buffer_param;
    let lifetime_param = &object.lifetime_param;
    let copy = partial_decoded_object.copy();
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
      .push(syn::GenericParam::Type(unknown_buffer_param.clone()));

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
        let type_constraints = f.partial_decoded().type_constraints();
        if !type_constraints.is_empty() {
          generics
            .make_where_clause()
            .predicates
            .extend(type_constraints.iter().cloned());
        }
      });

    Ok(Self {
      name: partial_decoded_object_name,
      attrs: partial_decoded_object.attrs().to_vec(),
      generics,
      unknown_buffer_field_name: format_ident!("__grost_unknown_buffer__"),
      copy,
    })
  }
}

impl<M, F> super::GenericObject<M, F> {
  pub(super) fn derive_partial_decoded_object_defination(&self) -> proc_macro2::TokenStream {
    let partial_decoded_object = self.partial_decoded();
    let name = partial_decoded_object.name();
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
          let ty = generic_tagged_field.partial_decoded().optional_type();
          let vis = generic_tagged_field.vis();
          let attrs = generic_tagged_field.partial_decoded().attrs();

          Some(quote! {
            #(#attrs)*
            #vis #field_name: #ty
          })
        }
      }
    });
    let generics = partial_decoded_object.generics();
    let where_clause = generics.where_clause.as_ref();
    let attrs = partial_decoded_object.attrs();

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
    let ubfn = &partial_decoded_object.unknown_buffer_field_name;
    let ubg = &self.unknown_buffer_param().ident;

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
}
