use std::num::NonZeroU32;

use syn::{Attribute, Generics, Ident, Path, Type, Visibility};

pub use indexer::Indexer;
pub use partial::{PartialField, PartialObject};
pub use partial_ref::{PartialRefField, PartialRefObject};
pub use selector::{Selector, SelectorIter};

use crate::meta::{
  SchemaMeta,
  object::{ObjectExt as _, TypeSpecification},
};

mod indexer;
mod partial;
mod partial_ref;
mod selector;

pub struct Field<M> {
  name: Ident,
  ty: Type,
  vis: Visibility,
  schema: SchemaMeta,
  default: Option<Path>,
  tag: NonZeroU32,
  wire: Option<Type>,
  specification: Option<TypeSpecification>,
  attrs: Vec<Attribute>,
  copy: bool,
  meta: M,
}

impl<M> Field<M> {
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
  pub const fn wire(&self) -> Option<&Type> {
    self.wire.as_ref()
  }

  /// Returns the field type specification.
  #[inline]
  pub const fn type_specification(&self) -> Option<&TypeSpecification> {
    self.specification.as_ref()
  }

  /// Returns the fn that returns the default value of the field.
  #[inline]
  pub const fn default(&self) -> Option<&Path> {
    self.default.as_ref()
  }

  /// Returns whether the field is copyable.
  #[inline]
  pub const fn copy(&self) -> bool {
    self.copy
  }

  /// Returns the field attributes.
  #[inline]
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns the schema information of the field.
  #[inline]
  pub const fn schema(&self) -> &SchemaMeta {
    &self.schema
  }

  /// Returns the meta information of the field.
  #[inline]
  pub const fn meta(&self) -> &M {
    &self.meta
  }

  fn from_input(input: M) -> darling::Result<Self>
  where
    M: crate::meta::object::Field,
  {
    let meta = input.meta();
    Ok(Self {
      name: input.name().clone(),
      ty: input.ty().clone(),
      vis: input.vis().clone(),
      tag: meta.tag(),
      specification: meta.type_specification().cloned(),
      attrs: meta.partial().attrs().to_vec(),
      copy: meta.copy(),
      schema: meta.schema().clone(),
      default: meta.default().cloned(),
      wire: meta.wire().cloned(),
      meta: input,
    })
  }
}

pub struct Object<M>
where
  M: crate::meta::object::Object,
{
  name: Ident,
  path_to_grost: Path,
  schema: SchemaMeta,
  vis: Visibility,
  generics: Generics,
  field_reflection: Ident,
  fields: Vec<Field<M::Field>>,
  partial: PartialObject,
  partial_ref: PartialRefObject,
  reflection: Ident,
  selector: Selector,
  selector_iter: SelectorIter,
  indexer: Indexer,
  meta: M,
}

impl<M> Object<M>
where
  M: crate::meta::object::Object,
{
  #[inline]
  pub const fn meta(&self) -> &M {
    &self.meta
  }

  #[inline]
  pub const fn path(&self) -> &Path {
    &self.path_to_grost
  }

  #[inline]
  pub const fn name(&self) -> &Ident {
    &self.name
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
  pub const fn fields(&self) -> &[Field<M::Field>]
  where
    M: crate::meta::object::Object,
  {
    self.fields.as_slice()
  }

  #[inline]
  pub const fn shema(&self) -> &SchemaMeta {
    &self.schema
  }

  #[inline]
  pub const fn partial(&self) -> &PartialObject {
    &self.partial
  }

  #[inline]
  pub const fn partial_ref(&self) -> &PartialRefObject {
    &self.partial_ref
  }

  #[inline]
  pub const fn field_reflection_name(&self) -> &Ident {
    &self.field_reflection
  }

  #[inline]
  pub const fn reflection_name(&self) -> &Ident {
    &self.reflection
  }

  #[inline]
  pub const fn selector(&self) -> &Selector {
    &self.selector
  }

  #[inline]
  pub const fn selector_iter(&self) -> &SelectorIter {
    &self.selector_iter
  }

  #[inline]
  pub const fn indexer(&self) -> &Indexer {
    &self.indexer
  }

  pub fn from_derive_input(input: &syn::DeriveInput) -> darling::Result<Self>
  where
    M: darling::FromDeriveInput + crate::meta::object::Object,
  {
    <M as darling::FromDeriveInput>::from_derive_input(input).and_then(Self::from_object)
  }

  pub fn from_object(input: M) -> darling::Result<Self>
  where
    M: crate::meta::object::Object,
  {
    let path_to_grost = input.path();
    let partial_object = PartialObject::from_input(path_to_grost, &input)?;
    let partial_ref_object = PartialRefObject::from_input(path_to_grost, &input)?;
    let selector = Selector::from_input(path_to_grost, &input)?;
    let selector_iter =
      selector.selector_iter(input.selector_iter_name(), input.meta().selector_iter())?;
    let indexer = Indexer::from_input(&input)?;

    Ok(Self {
      name: input.name().clone(),
      path_to_grost: path_to_grost.clone(),
      schema: input.meta().schema().clone(),
      vis: input.vis().clone(),
      generics: input.generics().clone(),
      fields: input
        .fields()
        .into_iter()
        .cloned()
        .map(Field::from_input)
        .collect::<Result<Vec<_>, darling::Error>>()?,
      partial: partial_object,
      partial_ref: partial_ref_object,
      field_reflection: input.field_reflection_name(),
      reflection: input.reflection_name(),
      selector_iter,
      selector,
      meta: input,
      indexer,
    })
  }
}
