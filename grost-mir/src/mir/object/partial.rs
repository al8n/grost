use std::num::NonZeroU32;

use quote::{format_ident, quote};
use syn::{Attribute, Generics, Ident, Type, TypeParam, Visibility, parse::Parser};

use crate::ast::{
  grost_unknown_buffer_param,
  object::{RawField, Label, RawObjectExt as _},
};

use super::Object;

/// The generic parameters of the [`PartialDecodedObject`].
#[derive(Debug, Clone)]
struct PartialObjectGenerics {
  generics: Generics,
  unknown_buffer_generic: TypeParam,
}

impl core::ops::Deref for PartialObjectGenerics {
  type Target = Generics;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.generics
  }
}

impl PartialObjectGenerics {
  const fn new(unknown_buffer_generic: TypeParam, generics: Generics) -> Self {
    Self {
      generics,
      unknown_buffer_generic,
    }
  }

  /// Returns the unknown buffer generic parameter of the partial object.
  #[inline]
  pub const fn unknown_buffer_param(&self) -> &TypeParam {
    &self.unknown_buffer_generic
  }
}

#[derive(Debug, Clone)]
pub struct PartialField {
  field: syn::Field,
  tag: NonZeroU32,
  specification: Label,
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
  pub const fn specification(&self) -> &Label {
    &self.specification
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
    I: crate::ast::object::RawField,
  {
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
      tag: input.tag().expect("field tag is required"),
      specification: input.label().clone(),
      copy: input.copy() | copy,
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
  generics: PartialObjectGenerics,
  fields: Vec<PartialField>,
  skipped_fields: Vec<syn::Field>,
  attrs: Vec<Attribute>,
  unknown_buffer_field_name: Ident,
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
    &self.generics.generics
  }

  /// Returns unknown buffer generic parameter of the partial object.
  #[inline]
  pub const fn unknown_buffer_param(&self) -> &TypeParam {
    self.generics.unknown_buffer_param()
  }

  /// Returns the field name of the unknown buffer.
  #[inline]
  pub const fn unknown_buffer_field_name(&self) -> &Ident {
    &self.unknown_buffer_field_name
  }

  /// Returns the fields of the partial object.
  #[inline]
  pub const fn fields(&self) -> &[PartialField] {
    self.fields.as_slice()
  }

  /// Returns the skipped fields of the partial object.
  #[inline]
  pub const fn skipped_fields(&self) -> &[syn::Field] {
    self.skipped_fields.as_slice()
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
    O: crate::ast::object::RawObject,
  {
    let meta = input;
    let mut fields = vec![];
    let mut skipped_fields = vec![];
    input.fields().into_iter().try_for_each(|f| {
      if f.skip() {
        let field_name = f.name();
        let vis = f.vis();
        let ty = f.ty();
        let attrs = f.attrs();
        let field = syn::Field::parse_named.parse2(quote! {
          #(#attrs)*
          #vis #field_name: ::core::option::Option<#ty>
        })?;

        skipped_fields.push(field);
        syn::Result::Ok(())
      } else {
        fields.push(PartialField::from_input(f, path_to_grost, meta.copy())?);
        Ok(())
      }
    })?;

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
    let unknown_buffer_param = grost_unknown_buffer_param();
    generics
      .params
      .push(syn::GenericParam::Type(unknown_buffer_param.clone()));

    Ok(Self {
      path_to_grost: path_to_grost.clone(),
      name: input.partial_name(),
      vis: input.vis().clone(),
      generics: PartialObjectGenerics::new(unknown_buffer_param, generics),
      fields,
      skipped_fields,
      attrs: meta.partial().attrs().to_vec(),
      unknown_buffer_field_name: format_ident!("__grost_unknown_buffer__"),
      copy: meta.copy(),
    })
  }

  pub(super) fn to_token_stream(&self) -> proc_macro2::TokenStream {
    let name = self.name();
    let visibility = &self.vis();
    let fields = self
      .fields
      .iter()
      .map(PartialField::field)
      .chain(self.skipped_fields.iter());
    let attrs = self.attrs();
    let generics = self.generics();
    let where_clause = generics.where_clause.as_ref();
    let unknown_buffer_field_name = self.unknown_buffer_field_name();
    let unknown_buffer_param = &self.unknown_buffer_param().ident;
    quote! {
      #(#attrs)*
      #[allow(non_camel_case_types, clippy::type_complexity)]
      #visibility struct #name #generics #where_clause {
        #unknown_buffer_field_name: ::core::option::Option<#unknown_buffer_param>,
        #(#fields),*
      }
    }
  }
}

impl<M> Object<M>
where
  M: crate::ast::object::RawObject,
{
  /// Generates a partial object of the object
  pub fn derive_partial_object(&self) -> proc_macro2::TokenStream {
    let partial = self.partial();
    let name = partial.name();
    let fields = self
      .partial()
      .fields()
      .iter()
      .map(|f| {
        let field_name = f.name();
        quote! {
          #field_name: ::core::option::Option::None,
        }
      })
      .chain(self.partial().skipped_fields().iter().map(|f| {
        let field_name = f.ident.as_ref().unwrap();
        quote! {
          #field_name: ::core::option::Option::None,
        }
      }));

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
    let ubfn = self.partial().unknown_buffer_field_name();
    let ubg = &self.partial().unknown_buffer_param().ident;

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
            #(#fields)*
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

        // /// Returns a mutable reference to the unknown buffer, which holds the unknown data when decoding.
        // #[inline]
        // pub const fn unknown_buffer_mut(&mut self) -> ::core::option::Option<&mut #ubg> {
        //   self.#ubfn.as_mut()
        // }

        #(#fields_accessors)*
      }
    }
  }
}
