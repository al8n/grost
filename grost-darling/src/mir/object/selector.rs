use quote::quote;
use syn::{GenericParam, Generics, Ident, Type, Visibility, parse_quote};

use super::super::wire_format_reflection_ty;

use crate::{
  grost_flavor_generic, grost_lifetime,
  meta::object::{Field, ObjectExt as _, Selection},
};

#[derive(Debug, Clone)]
pub struct SelectorField {
  name: Ident,
  ty: Type,
  default: Selection,
}

impl SelectorField {
  /// Returns the name of the field.
  #[inline]
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  /// Returns the type of the field.
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the default selection of the field.
  #[inline]
  pub const fn default_selection(&self) -> &Selection {
    &self.default
  }
}

#[derive(Debug, Clone)]
pub struct SelectorIter {
  name: Ident,
  vis: Visibility,
  generics: Generics,
}

impl SelectorIter {
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  pub const fn vis(&self) -> &Visibility {
    &self.vis
  }

  pub const fn generics(&self) -> &Generics {
    &self.generics
  }
}

#[derive(Debug, Clone)]
pub struct Selector {
  name: Ident,
  vis: Visibility,
  generics: Generics,
  fields: Vec<SelectorField>,
}

impl Selector {
  #[inline]
  pub const fn fields(&self) -> &[SelectorField] {
    self.fields.as_slice()
  }

  #[inline]
  pub const fn generics(&self) -> &Generics {
    &self.generics
  }

  #[inline]
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  #[inline]
  pub const fn vis(&self) -> &Visibility {
    &self.vis
  }

  pub(super) fn from_input<O>(path_to_grost: &syn::Path, input: &O) -> darling::Result<Self>
  where
    O: crate::meta::object::Object,
  {
    let name = input.selector_name();

    let vis = input.vis().clone();
    let field_reflection_ident = input.field_reflection_name();
    let fg = grost_flavor_generic();

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

    add_selector_constraints(
      &mut generics,
      path_to_grost,
      &field_reflection_ident,
      input.fields().iter().copied(),
      &fg,
    )?;

    let fields = input
      .fields()
      .iter()
      .map(|f| {
        let ty = f.ty();
        let tag = f.meta().tag();
        let wfr = wire_format_reflection_ty(path_to_grost, &field_reflection_ident, tag.get(), &fg);
        let wf = wire_format_ty(path_to_grost, &wfr, &fg);

        Ok(SelectorField {
          name: f.name().clone(),
          ty: syn::parse2(
            quote!(<#ty as #path_to_grost::__private::Selectable<#fg, #wf>>::Selector),
          )?,
          default: f.meta().selection().clone(),
        })
      })
      .collect::<Result<Vec<_>, darling::Error>>()?;

    Ok(Self {
      name,
      vis,
      generics,
      fields,
    })
  }

  pub(super) fn selector_iter(&self, name: Ident) -> darling::Result<SelectorIter> {
    let mut generics = Generics::default();
    let original_generics = self.generics.clone();

    // push the lifetime generic parameter first
    generics.params.extend(
      original_generics
        .params
        .iter()
        .filter(|param| matches!(param, GenericParam::Lifetime(_)))
        .cloned(),
    );

    let lt = grost_lifetime();
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

    // push the original const generic parameters last
    generics.params.extend(
      original_generics
        .params
        .iter()
        .filter(|param| matches!(param, GenericParam::Const(_)))
        .cloned(),
    );

    generics.params.push(GenericParam::Const(syn::parse2(
      quote! { const __GROST_SELECTED__: ::core::primitive::bool = true },
    )?));

    if let Some(where_clause) = original_generics.where_clause.as_ref() {
      generics
        .make_where_clause()
        .predicates
        .extend(where_clause.predicates.iter().cloned());
    }

    Ok(SelectorIter {
      name,
      vis: self.vis.clone(),
      generics,
    })
  }
}

fn wire_format_ty(path_to_grost: &syn::Path, wf: &syn::Type, fg: &syn::Ident) -> syn::Type {
  parse_quote! {
    <#wf as #path_to_grost::__private::reflection::Reflectable<#fg>>::Reflection
  }
}

fn selector_ty(path_to_grost: &syn::Path, wf: &syn::Type, fg: &syn::Ident) -> syn::Type {
  parse_quote! {
    #path_to_grost::__private::Selectable<
      #fg,
      #wf,
    >
  }
}

fn add_selector_constraints<'a, I>(
  generics: &mut Generics,
  path_to_grost: &syn::Path,
  field_reflection: &syn::Ident,
  mut fields: impl Iterator<Item = &'a I>,
  fg: &syn::Ident,
) -> darling::Result<()>
where
  I: crate::meta::object::Field + 'a,
{
  fields.try_for_each(move |f| {
    let ty = f.ty();
    let wfr = wire_format_reflection_ty(path_to_grost, field_reflection, f.meta().tag().get(), fg);
    let wf = wire_format_ty(path_to_grost, &wfr, fg);
    let selector_ty = selector_ty(path_to_grost, &wf, fg);

    let where_clause = generics.make_where_clause();

    where_clause.predicates.push(syn::parse2(quote! {
      #wfr: #path_to_grost::__private::reflection::Reflectable<#fg>
    })?);
    where_clause.predicates.push(syn::parse2(quote! {
      #ty: #selector_ty
    })?);

    Ok(())
  })
}
