use quote::{ToTokens, quote};
use syn::{GenericParam, Generics, Ident, Type, Visibility, parse::Parser, parse_quote};

use super::super::wire_format_reflection_ty;

use crate::{
  grost_flavor_generic, grost_lifetime,
  meta::object::{Field, ObjectExt as _, Selection, SelectorIterMeta},
};

#[derive(Debug, Clone)]
pub struct SelectorField {
  field: syn::Field,
  default: Selection,
}

impl SelectorField {
  /// Returns the name of the field.
  #[inline]
  pub const fn name(&self) -> &Ident {
    self.field.ident.as_ref().unwrap()
  }

  /// Returns the type of the field.
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.field.ty
  }

  /// Returns the visibility of the field.
  #[inline]
  pub const fn vis(&self) -> &Visibility {
    &self.field.vis
  }

  /// Returns the field.
  #[inline]
  pub const fn field(&self) -> &syn::Field {
    &self.field
  }

  /// Returns the attributes of the field.
  #[inline]
  pub const fn attrs(&self) -> &[syn::Attribute] {
    self.field.attrs.as_slice()
  }

  /// Returns the default selection of the field.
  #[inline]
  pub const fn default_selection(&self) -> &Selection {
    &self.default
  }
}

#[derive(Debug, Clone)]
pub struct SelectorIter {
  selector: Ident,
  indexer: Ident,
  name: Ident,
  vis: Visibility,
  attrs: Vec<syn::Attribute>,
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

  pub const fn attrs(&self) -> &[syn::Attribute] {
    self.attrs.as_slice()
  }
}

impl ToTokens for SelectorIter {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let selector_name = &self.selector;
    let iter_name = self.name();
    let vis = self.vis();
    let indexer_name = &self.indexer;
    let gl = grost_lifetime();
    let fg = grost_flavor_generic();
    let generics = self.generics();
    let where_clause = generics.where_clause.as_ref();
    let attrs = self.attrs();
    let doc = if !attrs.iter().any(|attr| attr.path().is_ident("doc")) {
      let doc = format!(" An iterator over the selected fields of the [`{selector_name}`]",);
      Some(quote! {
        #[doc = #doc]
      })
    } else {
      None
    };

    tokens.extend(quote! {
      #(#attrs)*
      #doc
      #[allow(non_camel_case_types, clippy::type_complexity)]
      #vis struct #iter_name #generics #where_clause
      {
        selector: &#gl #selector_name<#fg>,
        index: ::core::option::Option<#indexer_name>,
        num: ::core::primitive::usize,
        yielded: ::core::primitive::usize,
      }
    });
  }
}

#[derive(Debug, Clone)]
pub struct Selector {
  parent_name: Ident,
  name: Ident,
  vis: Visibility,
  generics: Generics,
  attrs: Vec<syn::Attribute>,
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
  pub const fn attrs(&self) -> &[syn::Attribute] {
    self.attrs.as_slice()
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

        let attrs = f.meta().selector().attrs();
        let vis = f.vis();
        let name = f.name();
        let field = syn::Field::parse_named.parse2(quote! {
          #(#attrs)*
          #vis #name: <#ty as #path_to_grost::__private::Selectable<#fg, #wf>>::Selector
        })?;

        Ok(SelectorField {
          field,
          default: f.meta().selector().select().clone(),
        })
      })
      .collect::<Result<Vec<_>, darling::Error>>()?;

    let attrs = input.meta().selector().attrs().to_vec();
    Ok(Self {
      parent_name: input.name().clone(),
      name,
      vis,
      attrs,
      generics,
      fields,
    })
  }

  pub(super) fn selector_iter(
    &self,
    name: Ident,
    indexer: Ident,
    iter_meta: &SelectorIterMeta,
  ) -> darling::Result<SelectorIter> {
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
      selector: self.name.clone(),
      indexer,
      name,
      attrs: iter_meta.attrs().to_vec(),
      vis: self.vis.clone(),
      generics,
    })
  }
}

impl ToTokens for Selector {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let name = self.name();
    let vis = self.vis();

    let attrs = self.attrs();
    let doc = if !attrs.iter().any(|attr| attr.path().is_ident("doc")) {
      let doc = format!(" The selection type for [`{}`]", self.parent_name);
      Some(quote! {
        #[doc = #doc]
      })
    } else {
      None
    };

    let fields = self.fields().iter().map(SelectorField::field);

    let generics = self.generics();
    let where_clause = generics.where_clause.as_ref();
    tokens.extend(quote! {
      #(#attrs)*
      #doc
      #[allow(non_camel_case_types, clippy::type_complexity)]
      #[derive(::core::fmt::Debug)]
      #vis struct #name #generics #where_clause
      {
        #(#fields),*
      }
    });
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
