use indexmap::IndexMap;
use quote::format_ident;
use quote::quote;
use syn::{Attribute, GenericParam, Generics, Ident, LifetimeParam, Type, TypeParam};

use crate::{
  ast::object::{GenericObject as GenericObjectAst, ObjectFlavor},
  obj::GenericField,
};

#[derive(Debug, Clone)]
pub struct PartialDecodedObjectFlavor {
  ty: Type,
  generics: Generics,
}

impl PartialDecodedObjectFlavor {
  /// Returns the type of the partial decoded object flavor.
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the generics of the partial decoded object flavor.
  #[inline]
  pub const fn generics(&self) -> &Generics {
    &self.generics
  }

  fn from_ast<M, F>(
    object: &GenericObjectAst<M, F>,
    flavor_name: &Ident,
    flavor: &ObjectFlavor,
    fields: &[GenericField<F>],
  ) -> darling::Result<Self> {
    let partial_decoded_object = object.partial_decoded();
    let partial_decoded_object_name = partial_decoded_object.name();
    let flavor_type = flavor.ty();
    let unknown_buffer_param = partial_decoded_object.unknown_buffer();
    let lifetime_param = partial_decoded_object.lifetime();

    let original_generics = object.generics();
    let mut generics = Generics::default();

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
      .push(syn::GenericParam::Lifetime(lifetime_param.clone()));

    // push the original type generic parameters
    generics.params.extend(
      original_generics
        .params
        .iter()
        .filter(|param| matches!(param, GenericParam::Type(_)))
        .cloned(),
    );

    generics
      .params
      .push(syn::GenericParam::Type(unknown_buffer_param.clone()));

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

    fields
      .iter()
      .filter_map(|f| f.try_unwrap_tagged_ref().ok())
      .for_each(|f| {
        let ff = f
          .flavors()
          .get(flavor_name)
          .expect("Field flavor already checked when constructing the AST");
        let type_constraints = ff.partial_decoded().type_constraints();
        if !type_constraints.is_empty() {
          generics
            .make_where_clause()
            .predicates
            .extend(type_constraints.iter().cloned());
        }
      });

    let (_, tg, _) = generics.split_for_impl();
    let ty = syn::parse2(quote! {
      #partial_decoded_object_name #tg
    })?;

    Ok(Self { ty, generics })
  }
}

#[derive(Debug, Clone)]
pub struct GenericPartialDecodedObject {
  name: Ident,
  attrs: Vec<Attribute>,
  generics: Generics,
  copy: bool,
  unknown_buffer_field_name: Ident,
  unknown_buffer_param: TypeParam,
  lifetime_param: LifetimeParam,
  flavors: IndexMap<Ident, PartialDecodedObjectFlavor>,
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

  /// Returns the generic unknown buffer type parameter of the partial decoded object.
  #[inline]
  pub const fn unknown_buffer(&self) -> &TypeParam {
    &self.unknown_buffer_param
  }

  /// Returns the lifetime parameter of the partial decoded object.
  #[inline]
  pub const fn lifetime(&self) -> &LifetimeParam {
    &self.lifetime_param
  }

  /// Returns `true` if the partial decoded object is copyable, `false` otherwise.
  #[inline]
  pub const fn copy(&self) -> bool {
    self.copy
  }

  /// Returns the flavors of the partial decoded object.
  #[inline]
  pub const fn flavors(&self) -> &IndexMap<Ident, PartialDecodedObjectFlavor> {
    &self.flavors
  }

  pub(super) fn from_ast<M, F>(
    object: &GenericObjectAst<M, F>,
    fields: &[GenericField<F>],
  ) -> darling::Result<Self> {
    let path_to_grost = object.path_to_grost();
    let partial_decoded_object = object.partial_decoded();
    let partial_decoded_object_name = partial_decoded_object.name().clone();
    let flavor_param = partial_decoded_object.flavor().clone();
    let unknown_buffer_param = partial_decoded_object.unknown_buffer().clone();
    let lifetime_param = partial_decoded_object.lifetime().clone();
    let copy = partial_decoded_object.copy();
    let original_generics = object.generics();
    let mut generics = Generics::default();

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
      .push(syn::GenericParam::Lifetime(lifetime_param.clone()));

    // push the original type generic parameters
    generics.params.extend(
      original_generics
        .params
        .iter()
        .filter(|param| matches!(param, GenericParam::Type(_)))
        .cloned(),
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

    let flavors = object
      .flavors()
      .iter()
      .map(|(name, flavor)| {
        PartialDecodedObjectFlavor::from_ast(object, name, flavor, fields)
          .map(|flavor| (name.clone(), flavor))
      })
      .collect::<darling::Result<IndexMap<_, _>>>()?;

    Ok(Self {
      name: partial_decoded_object_name,
      attrs: partial_decoded_object.attrs().to_vec(),
      generics,
      unknown_buffer_param,
      unknown_buffer_field_name: format_ident!("__grost_unknown_buffer__"),
      lifetime_param,
      copy,
      flavors,
    })
  }
}
