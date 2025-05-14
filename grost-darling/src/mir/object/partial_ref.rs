use std::num::NonZeroU32;

use quote::quote;
use syn::{Attribute, GenericParam, Generics, Ident, Type, Visibility, parse_quote};

use crate::{
  grost_flavor_generic, grost_lifetime, grost_unknown_buffer_generic,
  meta::object::{Field as _, ObjectExt as _},
};

use super::super::wire_format_reflection_ty;

#[derive(Debug, Clone)]
pub struct PartialRefField {
  name: Ident,
  ty: Type,
  vis: Visibility,
  tag: NonZeroU32,
  wire: Type,
  attrs: Vec<Attribute>,
  copy: bool,
}

impl PartialRefField {
  /// Returns the field name.
  #[inline]
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  /// Returns the field type.
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the field visibility.
  #[inline]
  pub const fn vis(&self) -> &Visibility {
    &self.vis
  }

  /// Returns the field tag.
  #[inline]
  pub const fn tag(&self) -> NonZeroU32 {
    self.tag
  }

  /// Returns the field wire format type.
  #[inline]
  pub const fn wire(&self) -> &Type {
    &self.wire
  }

  /// Returns the field attributes.
  #[inline]
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns whether the field is copyable.
  #[inline]
  pub const fn copy(&self) -> bool {
    self.copy
  }
}

#[derive(Debug, Clone)]
pub struct PartialRefObject {
  path_to_grost: syn::Path,
  name: Ident,
  vis: Visibility,
  generics: Generics,
  fields: Vec<PartialRefField>,
  attrs: Vec<Attribute>,
  copy: bool,
}

impl PartialRefObject {
  #[inline]
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  #[inline]
  pub const fn path_to_grost(&self) -> &syn::Path {
    &self.path_to_grost
  }

  #[inline]
  pub const fn vis(&self) -> &Visibility {
    &self.vis
  }

  #[inline]
  pub const fn generics(&self) -> &Generics {
    &self.generics
  }

  #[inline]
  pub fn fields(&self) -> &[PartialRefField] {
    self.fields.as_slice()
  }

  #[inline]
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  #[inline]
  pub const fn copy(&self) -> bool {
    self.copy
  }

  pub(super) fn derive_defination(&self) -> proc_macro2::TokenStream {
    let name = &self.name;
    let vis = &self.vis;
    let ubg = grost_unknown_buffer_generic();

    let fields = self.fields.iter().map(|f| {
      let ty = &f.ty;
      let name = &f.name;
      quote! {
        #name: #ty,
      }
    });

    let (_, ty_generics, where_clauses) = self.generics.split_for_impl();
    quote! {
      #[allow(clippy::type_complexity, non_camel_case_types)]
      #vis struct #name #ty_generics #where_clauses
      {
        #(#fields)*
        __grost_unknown_buffer__: ::core::option::Option<#ubg>,
      }
    }
  }

  pub(super) fn from_input<O>(path_to_grost: &syn::Path, input: &O) -> darling::Result<Self>
  where
    O: crate::meta::object::Object,
  {
    let fields = input.fields();
    let meta = input.meta();
    let field_reflection_name = input.field_reflection_name();
    let fg = grost_flavor_generic();
    let lt = grost_lifetime();
    let ubg = grost_unknown_buffer_generic();

    let mut generics = Generics::default();
    let original_generics = input.generics();

    // push the lifetime generic parameter first
    generics.params.extend(
      original_generics
        .params
        .iter()
        .filter(|param| matches!(param, GenericParam::Lifetime(_)))
        .cloned(),
    );

    generics
      .params
      .push(syn::GenericParam::Lifetime(syn::LifetimeParam::new(
        lt.clone(),
      )));

    // push the original type generic parameters
    generics.params.extend(
      original_generics
        .params
        .iter()
        .filter(|param| matches!(param, GenericParam::Type(_)))
        .cloned(),
    );

    generics.params.push(syn::GenericParam::Type(syn::parse2(
      quote! { #fg: ?::core::marker::Sized },
    )?));

    generics
      .params
      .push(syn::GenericParam::Type(syn::parse2(quote! { #ubg })?));

    // push the original const generic parameters last
    generics.params.extend(
      original_generics
        .params
        .iter()
        .filter(|param| matches!(param, GenericParam::Const(_)))
        .cloned(),
    );

    if let Some(where_clause) = original_generics.where_clause.as_ref() {
      generics
        .make_where_clause()
        .predicates
        .extend(where_clause.predicates.iter().cloned());
    }

    add_partial_ref_constraints(
      &mut generics,
      path_to_grost,
      &field_reflection_name,
      fields.iter().copied(),
      &fg,
      &lt,
    )?;

    let fields = fields
      .iter()
      .map(|f| {
        let ty = f.ty();
        let meta = f.meta();
        let tag = meta.tag();
        let wf = wire_format_reflection_ty(path_to_grost, &field_reflection_name, tag.get(), &fg);
        let encoded_state = encode_state_ty(path_to_grost, &wf, &fg, &lt);

        Ok(PartialRefField {
          name: f.name().clone(),
          ty: syn::parse2(quote! { ::core::option::Option<<#ty as #encoded_state>::Output> })?,
          vis: f.vis().clone(),
          tag,
          wire: wf,
          attrs: meta.partial_ref().attrs().to_vec(),
          copy: meta.partial_ref().copy(),
        })
      })
      .collect::<Result<Vec<_>, darling::Error>>()?;

    Ok(Self {
      path_to_grost: path_to_grost.clone(),
      name: input.partial_ref_name(),
      vis: input.vis().clone(),
      generics,
      fields,
      attrs: meta.partial_ref().attrs().to_vec(),
      copy: meta.partial_ref().copy(),
    })
  }
}

fn encode_state_ty(
  path_to_grost: &syn::Path,
  wf: &syn::Type,
  flavor_generic: &syn::Ident,
  lifetime: &syn::Lifetime,
) -> syn::Type {
  parse_quote! {
    #path_to_grost::__private::convert::State<
      #path_to_grost::__private::convert::Encoded<
        #lifetime,
        #flavor_generic,
        <#wf as #path_to_grost::__private::reflection::Reflectable<#flavor_generic>>::Reflection,
      >
    >
  }
}

fn add_partial_ref_constraints<'a, I>(
  generics: &mut syn::Generics,
  path_to_grost: &syn::Path,
  field_reflection: &syn::Ident,
  mut fields: impl Iterator<Item = &'a I>,
  flavor_generic: &syn::Ident,
  lifetime: &syn::Lifetime,
) -> darling::Result<()>
where
  I: crate::meta::object::Field + 'a,
{
  fields.try_for_each(move |f| {
    let ty = f.ty();
    let meta = f.meta();
    let wf = wire_format_reflection_ty(
      path_to_grost,
      field_reflection,
      meta.tag().get(),
      flavor_generic,
    );
    let encoded_state = encode_state_ty(path_to_grost, &wf, flavor_generic, lifetime);

    let where_clause = generics.make_where_clause();

    where_clause.predicates.push(syn::parse2(quote! {
      #wf: #path_to_grost::__private::reflection::Reflectable<#flavor_generic>
    })?);
    where_clause.predicates.push(syn::parse2(quote! {
      #ty: #encoded_state
    })?);
    where_clause.predicates.push(syn::parse2(quote! {
      <#ty as #encoded_state>::Output: ::core::marker::Sized
    })?);

    Ok(())
  })
}
