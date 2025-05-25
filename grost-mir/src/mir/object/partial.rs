use std::num::NonZeroU32;

use quote::quote;
use syn::{Attribute, Generics, Ident, Type, Visibility, parse::Parser};

use crate::ast::object::{ObjectExt as _, TypeSpecification};

use super::Object;

#[derive(Debug, Clone)]
pub struct PartialField {
  field: syn::Field,
  tag: NonZeroU32,
  specification: Option<TypeSpecification>,
  copy: bool,
  object_type: Type,
  output_type: Type,
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

  pub(super) fn from_input<I>(
    input: &I,
    path_to_grost: &syn::Path,
    copy: bool,
  ) -> darling::Result<Self>
  where
    I: crate::ast::object::Field,
  {
    let meta = input.meta();
    let ty = input.ty();
    let name = input.name();
    let vis = input.vis();
    let attrs = input.attrs();
    let output_type = syn::parse2(quote! {
      <#ty as #path_to_grost::__private::convert::State<
        #path_to_grost::__private::convert::Flatten
      >>::Output
    })?;
    let field = syn::Field::parse_named.parse2(quote! {
      #(#attrs)*
      #vis #name: ::core::option::Option<#output_type>
    })?;

    Ok(Self {
      field,
      tag: meta.tag(),
      specification: meta.type_specification().cloned(),
      copy: meta.copy() | copy,
      object_type: ty.clone(),
      output_type,
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
    O: crate::ast::object::Object,
  {
    let meta = input.meta();
    let fields = input
      .fields()
      .into_iter()
      .map(|f| PartialField::from_input(f, path_to_grost, meta.copy()))
      .collect::<Result<Vec<_>, darling::Error>>()?;

    let mut generics = input.generics().clone();
    generics.type_params().map(|p| p.ident.clone()).collect::<Vec<_>>().into_iter().try_for_each(|ident| {
      let where_clause = generics.make_where_clause();

      syn::parse2(quote!(#ident: #path_to_grost::__private::convert::State<#path_to_grost::__private::convert::Flatten>))
        .and_then(|s| {
          syn::parse2(quote!(<#ident as #path_to_grost::__private::convert::State<#path_to_grost::__private::convert::Flatten>>::Output: ::core::marker::Sized))
          .map(|c| (s, c))
        })
        .map(|(s, c)| {
          where_clause.predicates.push(s);
          where_clause.predicates.push(c);
        })
    })?;

    Ok(Self {
      path_to_grost: path_to_grost.clone(),
      name: input.partial_name(),
      vis: input.vis().clone(),
      generics,
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
  M: crate::ast::object::Object,
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
      let output_type = &f.output_type;
      super::optional_accessors(field_name, output_type, f.copy())
    });

    let (ig, tg, where_clauses) = self.partial().generics().split_for_impl();
    let flatten_state = super::derive_flatten_state(
      &self.path_to_grost,
      self.partial().generics(),
      self.partial().name(),
    );

    let is_empty = self.partial().fields().iter().map(|f| {
      let field_name = f.name();
      quote! {
        self.#field_name.is_none()
      }
    });

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
            #(#fields)*
          }
        }

        /// Returns `true` if the partial object is empty.
        #[inline]
        pub const fn is_empty(&self) -> bool {
          #(#is_empty)&&*
        }

        #(#fields_accessors)*
      }
    }
  }
}
