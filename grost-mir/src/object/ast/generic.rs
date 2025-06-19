use darling::usage::GenericsExt;
use indexmap::{IndexMap, IndexSet};
use quote::quote;
use syn::{Attribute, Generics, Ident, LifetimeParam, Path, Type, TypeParam, Visibility};

use crate::{
  object::{
    Indexer, ObjectFlavor, RawField, RawObject, RawObjectExt,
    ast::{
      GenericField, GenericPartialDecodedObject, GenericSelector, GenericSelectorIter,
      GenericTaggedField, PartialObject, SkippedField,
    },
  },
  utils::Invokable,
};

/// The AST for a generic object, a generic object which means there are multiple flavors and the generated code will be generic over the flavor type.
#[derive(Debug, Clone)]
pub(in crate::object) struct GenericObject<M = (), F = ()> {
  pub(in crate::object) path_to_grost: Path,
  pub(in crate::object) attrs: Vec<Attribute>,
  pub(in crate::object) default: Option<Invokable>,
  pub(in crate::object) name: Ident,
  pub(in crate::object) schema_name: String,
  pub(in crate::object) schema_description: String,
  pub(in crate::object) flavor_param: TypeParam,
  pub(in crate::object) unknown_buffer_param: TypeParam,
  pub(in crate::object) lifetime_param: LifetimeParam,
  pub(in crate::object) wire_format_param: TypeParam,
  pub(in crate::object) read_buffer_param: TypeParam,
  pub(in crate::object) write_buffer_param: TypeParam,
  pub(in crate::object) vis: Visibility,
  pub(in crate::object) ty: Type,
  pub(in crate::object) reflectable: Type,
  pub(in crate::object) fields: Vec<GenericField<F>>,
  pub(in crate::object) generics: Generics,
  pub(in crate::object) partial: PartialObject,
  pub(in crate::object) partial_decoded: GenericPartialDecodedObject,
  pub(in crate::object) selector: GenericSelector,
  pub(in crate::object) selector_iter: GenericSelectorIter,
  pub(in crate::object) indexer: Indexer,
  pub(in crate::object) flavors: IndexMap<Ident, ObjectFlavor>,
  pub(in crate::object) copy: bool,
  pub(in crate::object) meta: M,
}

impl<M, F> GenericObject<M, F> {
  /// Returns the path to the `grost` crate.
  #[inline]
  pub const fn path_to_grost(&self) -> &Path {
    &self.path_to_grost
  }

  /// Returns the type of the object.
  ///
  /// e.g. If a struct is `struct MyObject<T> { ... }`, this will return `MyObject<T>`.
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the generics in the object definition.
  #[inline]
  pub const fn generics(&self) -> &Generics {
    &self.generics
  }

  /// Returns the fields of the object.
  #[inline]
  pub const fn fields(&self) -> &[GenericField<F>] {
    self.fields.as_slice()
  }

  /// Returns the flavor attributes in the object definition.
  #[inline]
  pub const fn flavors(&self) -> &IndexMap<Ident, ObjectFlavor> {
    &self.flavors
  }

  /// Returns the generic flavor type parameter
  pub const fn flavor_param(&self) -> &TypeParam {
    &self.flavor_param
  }

  /// Returns the generic unknown buffer type parameter
  pub const fn unknown_buffer_param(&self) -> &TypeParam {
    &self.unknown_buffer_param
  }

  /// Returns `true` if the object is copyable, `false` otherwise.
  #[inline]
  pub const fn copy(&self) -> bool {
    self.copy
  }

  /// Returns the partial object information.
  #[inline]
  pub const fn partial(&self) -> &PartialObject {
    &self.partial
  }

  /// Returns the partial decoded object information.
  #[inline]
  pub const fn partial_decoded(&self) -> &GenericPartialDecodedObject {
    &self.partial_decoded
  }

  /// Returns the selector information.
  #[inline]
  pub const fn selector(&self) -> &GenericSelector {
    &self.selector
  }

  /// Returns the selector iterator information.
  #[inline]
  pub const fn selector_iter(&self) -> &GenericSelectorIter {
    &self.selector_iter
  }

  pub(super) fn try_new<O>(object: &O, flavor_param: TypeParam) -> darling::Result<Self>
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

    let flavors = object
      .flavors()
      .iter()
      .map(|f| {
        let name = f.name().clone();
        ObjectFlavor::from_attribute(f).map(|f| (name, f))
      })
      .collect::<darling::Result<IndexMap<_, _>>>()?;

    let mut tags = IndexSet::new();
    let fields = object
      .fields()
      .iter()
      .map(|&f| {
        if f.skip() {
          SkippedField::try_new(f, &type_params, &lifetimes)
            .map(|f| GenericField::Skipped(Box::new(f)))
        } else {
          GenericTaggedField::try_new(
            f,
            &flavor_param,
            &flavors,
            &type_params,
            &lifetimes,
            f.copy() || object.copy(),
          )
          .and_then(|f| {
            if tags.contains(&f.tag()) {
              Err(darling::Error::custom(format!(
                "{name} has multiple fields have the same tag {}",
                f.tag()
              )))
            } else {
              tags.insert(f.tag());
              Ok(GenericField::Tagged(Box::new(f)))
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
    let partial_decoded =
      GenericPartialDecodedObject::from_attribute(&name, object.partial_decoded())?;
    let selector = GenericSelector::from_attribute(&name, object.selector())?;
    let selector_iter =
      GenericSelectorIter::from_attribute(selector.name(), object.selector_iter())?;

    Ok(Self {
      path_to_grost,
      attrs,
      default: object.default().cloned(),
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
      flavor_param,
      lifetime_param: object.lifetime_param().clone(),
      unknown_buffer_param: object.unknown_buffer_type_param().clone(),
      wire_format_param: object.wire_format_type_param().clone(),
      read_buffer_param: object.read_buffer_type_param().clone(),
      write_buffer_param: object.write_buffer_type_param().clone(),
      reflectable,
      generics,
      flavors,
      fields,
      partial,
      partial_decoded,
      selector,
      selector_iter,
      copy: object.copy(),
      meta: object.meta().clone(),
    })
  }
}
