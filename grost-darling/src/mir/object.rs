use std::num::NonZeroU32;

use quote::{ToTokens, quote};
use syn::{Attribute, Generics, Ident, Path, Type, Visibility};

pub use indexer::Indexer;
pub use partial::{PartialField, PartialObject};
pub use partial_ref::{PartialRefField, PartialRefObject};
pub use reflection::Reflection;
pub use selector::{Selector, SelectorField, SelectorIter};

use crate::meta::{
  SchemaMeta,
  object::{ObjectExt as _, TypeSpecification},
};

mod indexer;
mod partial;
mod partial_ref;
mod reflection;
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
  fields: Vec<Field<M::Field>>,
  partial: PartialObject,
  partial_ref: PartialRefObject,
  reflection: Reflection,
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
  pub const fn reflection(&self) -> &Reflection {
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
}

impl<M> Object<M>
where
  M: crate::meta::object::Object,
{
  pub fn from_derive_input(input: &syn::DeriveInput) -> darling::Result<Self>
  where
    M: darling::FromDeriveInput,
  {
    <M as darling::FromDeriveInput>::from_derive_input(input).and_then(Self::from_object)
  }

  pub fn from_object(input: M) -> darling::Result<Self> {
    let path_to_grost = input.path();
    let partial_object = PartialObject::from_input(path_to_grost, &input)?;
    let partial_ref_object = PartialRefObject::from_input(path_to_grost, &input)?;
    let selector = Selector::from_input(path_to_grost, &input)?;
    let selector_iter = selector.selector_iter(
      input.selector_iter_name(),
      input.indexer_name(),
      input.meta().selector_iter(),
    )?;
    let indexer = Indexer::from_input(&input)?;
    let reflection = Reflection::from_input(&input)?;

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
      reflection,
      selector_iter,
      selector,
      meta: input,
      indexer,
    })
  }

  fn derive(&self) -> proc_macro2::TokenStream {
    let reflection_impl = self.derive_reflection();
    let indexer_impl = self.derive_indexer();
    let selector_iter_impl = self.derive_selector_iter();
    let selector_impl = self.derive_selector();
    let partial_ref_object_impl = self.derive_partial_ref_object();

    quote! {
      #reflection_impl

      #indexer_impl

      #selector_impl

      #selector_iter_impl

      #partial_ref_object_impl
    }
  }
}

impl<M> ToTokens for Object<M>
where
  M: crate::meta::object::Object,
{
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let partial_object = self.partial();
    let partial_ref_object = self.partial_ref().to_token_stream();
    let selector = self.selector().to_token_stream();
    let selector_iter = self.selector_iter().to_token_stream();
    let indexer = self.indexer().to_token_stream();
    let reflection = self.reflection().to_token_stream();

    let impls = self.derive();

    tokens.extend(quote! {
      #reflection

      #partial_object

      #partial_ref_object

      #indexer

      #selector

      #selector_iter

      #impls
    });
  }
}
