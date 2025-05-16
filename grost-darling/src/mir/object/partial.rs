use std::num::NonZeroU32;

use quote::{format_ident, quote};
use syn::{Attribute, Generics, Ident, Type, Visibility, parse::Parser};

use crate::meta::object::{ObjectExt as _, TypeSpecification};

use super::Object;

#[derive(Debug, Clone)]
pub struct PartialField {
  field: syn::Field,
  tag: NonZeroU32,
  specification: Option<TypeSpecification>,
  copy: bool,
  object_type: Type,
}

impl PartialField {
  /// Returns the field name.
  #[inline]
  pub const fn name(&self) -> &Ident {
    self.field.ident.as_ref().unwrap()
  }

  /// Returns the field type.
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.field.ty
  }

  /// Returns the corresponding type to the object.
  #[inline]
  pub const fn object_type(&self) -> &Type {
    &self.object_type
  }

  /// Returns the field visibility.
  #[inline]
  pub const fn vis(&self) -> &Visibility {
    &self.field.vis
  }

  /// Returns the field
  #[inline]
  pub const fn field(&self) -> &syn::Field {
    &self.field
  }

  /// Returns the field tag.
  #[inline]
  pub const fn tag(&self) -> NonZeroU32 {
    self.tag
  }

  /// Returns the type specification.
  #[inline]
  pub const fn specification(&self) -> Option<&TypeSpecification> {
    self.specification.as_ref()
  }

  /// Returns whether the field is copyable.
  #[inline]
  pub const fn copy(&self) -> bool {
    self.copy
  }

  /// Returns the field attributes.
  #[inline]
  pub const fn attrs(&self) -> &[Attribute] {
    self.field.attrs.as_slice()
  }

  pub(super) fn from_input<I>(input: &I, copy: bool) -> darling::Result<Self>
  where
    I: crate::meta::object::Field,
  {
    let meta = input.meta();
    let ty = input.ty();
    let name = input.name();
    let vis = input.vis();
    let attrs = input.attrs();
    let optional = input
      .meta()
      .type_specification()
      .is_some_and(|f| f.is_optional());
    let field = if optional {
      syn::Field::parse_named.parse2(quote! {
        #(#attrs)*
        #vis #name: #ty
      })?
    } else {
      syn::Field::parse_named.parse2(quote! {
        #(#attrs)*
        #vis #name: ::core::option::Option<#ty>
      })?
    };

    Ok(Self {
      field,
      tag: meta.tag(),
      specification: meta.type_specification().cloned(),
      copy: meta.copy() | copy,
      object_type: ty.clone(),
    })
  }
}

#[derive(Debug, Clone)]
pub struct PartialObject {
  path_to_grost: syn::Path,
  name: Ident,
  vis: Visibility,
  generics: Generics,
  fields: Vec<PartialField>,
  attrs: Vec<Attribute>,
  copy: bool,
}

impl PartialObject {
  /// Returns the name of the partial object.
  #[inline]
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  /// Returns the path to grost.
  #[inline]
  pub const fn path_to_grost(&self) -> &syn::Path {
    &self.path_to_grost
  }

  /// Returns the visibility of the partial object.
  #[inline]
  pub const fn vis(&self) -> &Visibility {
    &self.vis
  }

  /// Returns the generics of the partial object.
  #[inline]
  pub const fn generics(&self) -> &Generics {
    &self.generics
  }

  /// Returns the fields of the partial object.
  #[inline]
  pub fn fields(&self) -> &[PartialField] {
    self.fields.as_slice()
  }

  /// Returns the attributes of the partial object.
  #[inline]
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns whether the partial object is copyable.
  #[inline]
  pub const fn copy(&self) -> bool {
    self.copy
  }

  pub(super) fn from_input<O>(path_to_grost: &syn::Path, input: &O) -> darling::Result<Self>
  where
    O: crate::meta::object::Object,
  {
    let meta = input.meta();
    let fields = input
      .fields()
      .into_iter()
      .map(|f| PartialField::from_input(f, meta.copy()))
      .collect::<Result<Vec<_>, darling::Error>>()?;

    Ok(Self {
      path_to_grost: path_to_grost.clone(),
      name: input.partial_name(),
      vis: input.vis().clone(),
      generics: input.generics().clone(),
      fields,
      attrs: meta.partial().attrs().to_vec(),
      copy: meta.copy(),
    })
  }

  pub(super) fn to_token_stream(&self) -> proc_macro2::TokenStream {
    let name = self.name();
    let visibility = &self.vis();
    let fields = self.fields().iter().map(PartialField::field);
    let attrs = self.attrs();
    let generics = self.generics();
    let where_clause = generics.where_clause.as_ref();

    quote! {
      #(#attrs)*
      #[allow(non_camel_case_types, clippy::type_complexity)]
      #visibility struct #name #generics #where_clause {
        #(#fields),*
      }
    }
  }
}

impl<M> Object<M>
where
  M: crate::meta::object::Object,
{
  /// Generates a partial object of the object
  pub fn derive_partial_object(&self) -> proc_macro2::TokenStream {
    let partial = self.partial();
    let name = partial.name();
    let fields = self.fields.iter().map(|f| {
      let field_name = f.name();
      quote! {
        #field_name: ::core::option::Option::None,
      }
    });

    let fields_accessors = self.partial().fields().iter().map(|f| {
      let field_name = f.name();
      let ref_fn = format_ident!("{}_ref", field_name);
      let ref_fn_doc = format!(" Returns a reference to the `{field_name}`");
      let ref_mut_fn = format_ident!("{}_mut", field_name);
      let ref_mut_fn_doc = format!(" Returns a mutable reference to the `{field_name}`");
      let set_fn = format_ident!("set_{}", field_name);
      let set_fn_doc = format!(" Set the `{field_name}` to the given value");
      let update_fn = format_ident!("update_{}", field_name);
      let update_fn_doc =
        format!(" Update the `{field_name}` to the given value or clear the `{field_name}`");
      let clear_fn = format_ident!("clear_{}", field_name);
      let clear_fn_doc = format!(" Clear the value of `{field_name}`");
      let take_fn = format_ident!("take_{}", field_name);
      let take_fn_doc = format!(" Takes the value of `{field_name}` out if it is not `None`");
      let with_fn = format_ident!("with_{}", field_name);
      let without_fn = format_ident!("without_{}", field_name);
      let maybe_fn = format_ident!("maybe_{}", field_name);
      let constable = f.copy.then(|| quote! { const });

      quote! {
        // #[doc = #ref_fn_doc]
        // #[inline]
        // pub const fn #ref_fn(&self) -> #ref_ty {
        //   self.#field_name.as_ref()
        // }

        // #[doc = #ref_mut_fn_doc]
        // #[inline]
        // pub const fn #ref_mut_fn(&mut self) -> #ref_mut_ty {
        //   self.#field_name.as_mut()
        // }

        // #[doc = #take_fn_doc]
        // #[inline]
        // pub const fn #take_fn(&mut self) -> #owned_ty {
        //   self.#field_name.take()
        // }

        // #[doc = #clear_fn_doc]
        // #[inline]
        // pub #constable fn #clear_fn(&mut self) -> &mut Self {
        //   self.#field_name = ::core::option::Option::None;
        //   self
        // }

        // #[doc = #set_fn_doc]
        // #[inline]
        // pub #constable fn #set_fn(&mut self, value: #) -> &mut Self {
        //   self.#field_name = ::core::option::Option::Some(value);
        //   self
        // }

        // #[doc = #update_fn_doc]
        // #[inline]
        // pub #constable fn #update_fn(&mut self, value: #owned_ty) -> &mut Self {
        //   self.#field_name = value;
        //   self
        // }

        // #[doc = #set_fn_doc]
        // #[inline]
        // pub #constable fn #with_fn(mut self, value: #ty) -> Self {
        //   self.#field_name = ::core::option::Option::Some(value);
        //   self
        // }

        // #[doc = #clear_fn_doc]
        // #[inline]
        // pub #constable fn #without_fn(mut self) -> Self {
        //   self.#field_name = ::core::option::Option::None;
        //   self
        // }

        // #[doc = #update_fn_doc]
        // #[inline]
        // pub #constable fn #maybe_fn(mut self, value: #owned_ty) -> Self {
        //   self.#field_name = value;
        //   self
        // }
      }
    });

    let (ig, tg, where_clauses) = self.partial().generics().split_for_impl();

    quote! {
      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl #ig ::core::default::Default for #name #tg #where_clauses {
        fn default() -> Self {
          Self::new()
        }
      }

      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl #ig #name #tg #where_clauses {
        /// Creates an empty partial struct.
        #[inline]
        pub const fn new() -> Self {
          Self {
            #(#fields)*
          }
        }

        #(#fields_accessors)*
      }
    }
  }
}
