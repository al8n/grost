use grost_darling::grost_unknown_buffer_generic;
use quote::quote;
use syn::DeriveInput;

mod indexer;
mod selector;

#[allow(unused)]
mod sealed {
  use super::super::default_path;

  #[derive(grost_darling::ObjectField)]
  #[grost(attributes(grost))]
  pub struct ObjectField;

  #[derive(grost_darling::Object)]
  #[grost(attributes(grost), field = "ObjectFieldDeriveInput")]
  pub struct Object;
}

pub struct Object {
  object: grost_darling::mir::object::Object<sealed::ObjectDeriveInput>,
}

impl Object {
  pub fn from_derive_input(input: &DeriveInput) -> darling::Result<Self> {
    let object =
      grost_darling::mir::object::Object::<sealed::ObjectDeriveInput>::from_derive_input(input)?;

    Ok(Self { object })
  }

  pub fn derive_defination(&self) -> proc_macro2::TokenStream {
    let partial_object = self.derive_partial_defination();
    let partial_ref_object = self.derive_partial_ref_defination();
    let reflection_definition = self.derive_reflection_definations();
    let selector = self.derive_selector_defination();
    let selector_iter = self.derive_selector_iter_defination();
    let indexer = self.derive_indexer_defination();

    quote! {
      #reflection_definition

      #partial_object

      #partial_ref_object

      #indexer

      #selector

      #selector_iter
    }
  }

  fn derive_partial_ref_defination(&self) -> proc_macro2::TokenStream {
    let partial_ref_object = self.object.partial_ref();
    let name = partial_ref_object.name();
    let vis = self.object.vis();
    let ubg = grost_unknown_buffer_generic();

    let fields = partial_ref_object.fields().iter().map(|f| {
      let ty = f.ty();
      let name = f.name();
      let attrs = f.attrs();
      quote! {
        #(#attrs)*
        #name: #ty,
      }
    });

    let generics = partial_ref_object.generics();
    let where_clause = generics.where_clause.as_ref();
    let attrs = partial_ref_object.attrs();

    quote! {
      #(#attrs)*
      #[allow(clippy::type_complexity, non_camel_case_types)]
      #vis struct #name #generics #where_clause
      {
        #(#fields)*
        __grost_unknown_buffer__: ::core::option::Option<#ubg>,
      }
    }
  }

  fn derive_partial_defination(&self) -> proc_macro2::TokenStream {
    let partial = self.object.partial();
    let name = partial.name();
    let visibility = &self.object.vis();
    let fields = partial.fields().iter().map(|f| {
      let field_name = f.name();
      let ty = f.ty();
      let attrs = f.attrs();
      quote! {
        #(#attrs)*
        #field_name: #ty,
      }
    });
    let attrs = partial.attrs();
    let generics = partial.generics();
    let where_clause = generics.where_clause.as_ref();

    quote! {
      #(#attrs)*
      #[allow(non_camel_case_types, clippy::type_complexity)]
      #visibility struct #name #generics #where_clause {
        #(#fields)*
      }
    }
  }

  /// Generates the reflection types for the object.
  fn derive_reflection_definations(&self) -> proc_macro2::TokenStream {
    let reflection_name = &self.object.reflection_name();
    let doc = format!(" The reflection of the [`{}`].", self.object.name());
    let field_reflection_name = &self.object.field_reflection_name();
    let field_reflection_doc = format!(
      " The field reflection of the [`{}`]'s fields.",
      self.object.name()
    );
    quote! {
      #[doc = #field_reflection_doc]
      pub struct #field_reflection_name<R: ?::core::marker::Sized, F: ?::core::marker::Sized, const TAG: ::core::primitive::u32> {
        _reflect: ::core::marker::PhantomData<R>,
        _flavor: ::core::marker::PhantomData<F>,
      }

      #[doc = #doc]
      pub struct #reflection_name<R: ?::core::marker::Sized, F: ?::core::marker::Sized> {
        _reflect: ::core::marker::PhantomData<R>,
        _flavor: ::core::marker::PhantomData<F>,
      }
    }
  }
}
