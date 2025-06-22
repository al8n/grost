use darling::usage::GenericsExt;
use indexmap::IndexSet;
use quote::quote;
use syn::{Attribute, Generics, Ident, LifetimeParam, Path, Type, TypeParam, Visibility};

use crate::{
  flavor::FlavorAttribute,
  object::{
    Indexer, RawField, RawObject, RawObjectExt,
    ast::{
      ConcreteField, ConcretePartialRefObject, ConcreteSelector, ConcreteSelectorIter,
      ConcreteTaggedField, PartialObject, SkippedField,
    },
  },
  utils::Invokable,
};

/// The AST for a concrete object, a concrete object which means there is only one flavor and the generated code will not be generic
/// over the flavor type.
#[derive(Debug, Clone)]
pub(in crate::object) struct ConcreteObject<M = (), F = ()> {
  pub(in crate::object) path_to_grost: Path,
  pub(in crate::object) attrs: Vec<Attribute>,
  pub(in crate::object) name: Ident,
  pub(in crate::object) schema_name: String,
  pub(in crate::object) schema_description: String,
  pub(in crate::object) vis: Visibility,
  pub(in crate::object) ty: Type,
  pub(in crate::object) reflectable: Type,
  pub(in crate::object) generics: Generics,
  pub(in crate::object) flavor: FlavorAttribute,
  pub(in crate::object) unknown_buffer_param: TypeParam,
  pub(in crate::object) read_buffer_param: TypeParam,
  pub(in crate::object) write_buffer_param: TypeParam,
  pub(in crate::object) lifetime_param: LifetimeParam,
  pub(in crate::object) fields: Vec<ConcreteField<F>>,
  pub(in crate::object) default: Option<Invokable>,
  pub(in crate::object) indexer: Indexer,
  pub(in crate::object) partial: PartialObject,
  pub(in crate::object) partial_ref: ConcretePartialRefObject,
  pub(in crate::object) selector: ConcreteSelector,
  pub(in crate::object) selector_iter: ConcreteSelectorIter,
  pub(in crate::object) copy: bool,
  pub(in crate::object) meta: M,
}

impl<M, F> ConcreteObject<M, F> {
  /// Returns the path to the `grost` crate
  #[inline]
  pub const fn path_to_grost(&self) -> &Path {
    &self.path_to_grost
  }

  /// Returns the name of the object
  #[inline]
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  /// Returns the schema name of the object
  #[inline]
  pub const fn schema_name(&self) -> &str {
    self.schema_name.as_str()
  }

  /// Returns the schema description of the object
  #[inline]
  pub const fn schema_description(&self) -> &str {
    self.schema_description.as_str()
  }

  /// Returns the visibility of the object
  #[inline]
  pub const fn vis(&self) -> &Visibility {
    &self.vis
  }

  /// Returns the type of the object
  ///
  /// e.g. If a struct is `struct MyObject<T> { ... }`, this will return `MyObject<T>`.
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the reflectable trait which replaces the generic parameter with the type of the object
  /// e.g. If a struct is `struct MyObject<T> { ... }`, this will return `Reflectable<MyObject<T>>`.
  #[inline]
  pub const fn reflectable(&self) -> &Type {
    &self.reflectable
  }

  /// Returns the flavor
  #[inline]
  pub const fn flavor(&self) -> &FlavorAttribute {
    &self.flavor
  }

  /// Returns the generic unknown buffer type parameter.
  #[inline]
  pub const fn unknown_buffer_param(&self) -> &TypeParam {
    &self.unknown_buffer_param
  }

  /// Returns the generic read buffer type parameter.
  #[inline]
  pub const fn read_buffer_param(&self) -> &TypeParam {
    &self.read_buffer_param
  }

  /// Returns the generic write buffer type parameter.
  #[inline]
  pub const fn write_buffer_param(&self) -> &TypeParam {
    &self.write_buffer_param
  }

  /// Returns the generic lifetime parameter.
  #[inline]
  pub const fn lifetime_param(&self) -> &LifetimeParam {
    &self.lifetime_param
  }

  /// Returns the generics in the object definition if any.
  #[inline]
  pub const fn generics(&self) -> &Generics {
    &self.generics
  }

  /// Returns the attributes in the object definition.
  #[inline]
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns `true` if the object is copyable, `false` otherwise.
  #[inline]
  pub const fn copy(&self) -> bool {
    self.copy
  }

  /// Returns the fields of the object
  #[inline]
  pub const fn fields(&self) -> &[ConcreteField<F>] {
    self.fields.as_slice()
  }

  /// Returns the partial object information
  #[inline]
  pub const fn partial(&self) -> &PartialObject {
    &self.partial
  }

  /// Returns the partial decoded object information
  #[inline]
  pub const fn partial_ref(&self) -> &ConcretePartialRefObject {
    &self.partial_ref
  }

  /// Returns the selector information
  #[inline]
  pub const fn selector(&self) -> &ConcreteSelector {
    &self.selector
  }

  /// Returns the selector iterator information
  #[inline]
  pub const fn selector_iter(&self) -> &ConcreteSelectorIter {
    &self.selector_iter
  }

  pub(in crate::object) fn try_new<O>(object: &O, flavor: &FlavorAttribute) -> darling::Result<Self>
  where
    O: RawObject<Meta = M>,
    O::Field: RawField<Meta = F>,
    F: Clone,
    M: Clone,
  {
    let path_to_grost = object.path_to_grost().clone();
    let attrs = object.attrs().to_vec();
    let name = object.name().clone();
    let vis = object.vis().clone();
    let generics = object.generics().clone();
    let (_, tg, _) = generics.split_for_impl();

    let type_params = generics.declared_type_params();
    let lifetimes = generics.declared_lifetimes();

    let ty: Type = syn::parse2(quote! {
      #name #tg
    })?;
    let reflectable: Type = syn::parse2(quote! {
      #path_to_grost::__private::reflection::Reflectable<#ty>
    })?;

    let mut tags = IndexSet::new();
    let fields = object
      .fields()
      .iter()
      .map(|&f| {
        if f.skip() {
          SkippedField::try_new(f, &type_params, &lifetimes)
            .map(|f| ConcreteField::Skipped(Box::new(f)))
        } else {
          ConcreteTaggedField::try_new(f, flavor, &type_params, &lifetimes).and_then(|f| {
            if tags.contains(&f.tag()) {
              Err(darling::Error::custom(format!(
                "{name} has multiple fields have the same tag {}",
                f.tag()
              )))
            } else {
              tags.insert(f.tag());
              Ok(ConcreteField::Tagged(Box::new(f)))
            }
          })
        }
      })
      .collect::<darling::Result<Vec<_>>>()
      .and_then(|fields| {
        if fields.is_empty() {
          Err(darling::Error::custom(format!(
            "{name} must have at least one field"
          )))
        } else {
          Ok(fields)
        }
      })?;

    let partial = PartialObject::from_attribute(&name, object.partial())?;
    let partial_ref =
      ConcretePartialRefObject::from_attribute(&name, object.partial_ref())?;
    let selector = ConcreteSelector::from_attribute(&name, object.selector())?;
    let selector_iter =
      ConcreteSelectorIter::from_attribute(selector.name(), object.selector_iter())?;

    Ok(Self {
      path_to_grost,
      copy: object.copy(),
      attrs,
      indexer: Indexer {
        name: object.indexer_name().clone(),
        attrs: object.indexer().attrs().to_vec(),
      },
      schema_name: object
        .schema()
        .name()
        .map_or_else(|| name.to_string(), |s| s.to_string()),
      schema_description: object
        .schema()
        .description()
        .unwrap_or_default()
        .to_string(),
      name,
      vis,
      ty,
      reflectable,
      generics,
      flavor: flavor.clone(),
      fields,
      default: object.default().cloned(),
      partial,
      partial_ref,
      selector,
      selector_iter,
      meta: object.meta().clone(),
      unknown_buffer_param: object.unknown_buffer_type_param().clone(),
      lifetime_param: object.lifetime_param().clone(),
      read_buffer_param: object.read_buffer_type_param().clone(),
      write_buffer_param: object.write_buffer_type_param().clone(),
    })
  }
}
