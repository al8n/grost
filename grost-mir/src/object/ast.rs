use quote::format_ident;
use syn::{Attribute, Generics, Ident, LifetimeParam, Path, Type, TypeParam, Visibility};

use crate::{
  flavor::{DecodeAttribute, EncodeAttribute, FlavorAttribute, IdentifierAttribute, TagAttribute},
  object::meta::ObjectFromMeta,
  utils::{Invokable, SchemaAttribute, grost_flavor_param},
};

pub(super) use concrete::ConcreteObject;
pub use field::*;
pub(super) use generic::GenericObject;
pub use indexer::*;
pub use partial::*;
pub use partial_decoded::*;
pub use selector::*;

mod concrete;
mod field;
mod generic;
mod indexer;
mod partial;
mod partial_decoded;
mod selector;

#[derive(Debug, Clone)]
pub struct Indexer {
  pub(super) name: Ident,
  pub(super) attrs: Vec<Attribute>,
}

impl Indexer {
  /// Returns the name of the indexer
  #[inline]
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  /// Returns the attributes of the indexer
  #[inline]
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }
}

#[derive(Debug, Clone)]
pub struct ObjectFlavor {
  pub(super) ty: Type,
  pub(super) format: Type,
  pub(super) identifier: IdentifierAttribute,
  pub(super) tag: TagAttribute,
  pub(super) encode: EncodeAttribute,
  pub(super) decode: DecodeAttribute,
}

impl ObjectFlavor {
  /// Returns the type of the flavor
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the wire format type for the object of this flavor.
  #[inline]
  pub const fn format(&self) -> &Type {
    &self.format
  }

  /// Returns the identifier attribute for the flavor.
  #[inline]
  pub const fn identifier(&self) -> &IdentifierAttribute {
    &self.identifier
  }

  /// Returns the tag attribute for the flavor.
  #[inline]
  pub const fn tag(&self) -> &TagAttribute {
    &self.tag
  }

  /// Returns the encode attribute for this flavor.
  #[inline]
  pub const fn encode(&self) -> &EncodeAttribute {
    &self.encode
  }

  /// Returns the decode attribute for this flavor.
  #[inline]
  pub const fn decode(&self) -> &DecodeAttribute {
    &self.decode
  }

  fn from_attribute(attribute: &FlavorAttribute) -> darling::Result<Self> {
    Ok(Self {
      ty: attribute.ty().clone(),
      format: attribute.wire_format().clone(),
      identifier: attribute.identifier().clone(),
      tag: attribute.tag().clone(),
      encode: attribute.encode().clone(),
      decode: attribute.decode().clone(),
    })
  }
}

/// The AST for an object, which can be either a concrete or a generic object.
///
/// The main purpose to having an AST for an object is used to validate the input (from the Rust's derive macro or attribute macro)
/// from the schema and to generate the necessary Middle Intermediate Representation (MIR) of the object.
///
/// A Middle Intermediate Representation (MIR) of the object can be
/// generated from this structure. Once the MIR is generated,
/// it can be used to generate the final Rust code for the object in a GraphQL Protocol schema.
#[derive(Debug, Clone, derive_more::IsVariant, derive_more::Unwrap, derive_more::TryUnwrap)]
#[unwrap(ref)]
#[try_unwrap(ref)]
pub(super) enum Object<M, F> {
  Concrete(Box<ConcreteObject<M, F>>),
  Generic(Box<GenericObject<M, F>>),
}

impl<M, F> Object<M, F> {
  /// Creates an `Object` from a raw object input.
  pub fn from_raw<O>(object: &O) -> darling::Result<Self>
  where
    O: RawObject<Meta = M>,
    O::Field: RawField<Meta = F>,
    M: Clone,
    F: Clone,
  {
    let num_flavors = object.flavors().len();
    if object.flavor_type_param().is_none() && num_flavors == 1 {
      let flavor = object
        .flavors()
        .iter()
        .next()
        .expect("There must be a flavor were already checked");
      ConcreteObject::try_new(object, flavor).map(|obj| Object::Concrete(Box::new(obj)))
    } else {
      let flavor_param = object
        .flavor_type_param()
        .cloned()
        .unwrap_or_else(grost_flavor_param);
      GenericObject::try_new(object, flavor_param).map(|obj| Object::Generic(Box::new(obj)))
    }
  }
}

#[derive(Debug, Clone)]
pub struct RawObject<T = (), S = (), O = ()> {
  name: Ident,
  vis: Visibility,
  generics: Generics,
  attrs: Vec<Attribute>,
  fields: Vec<RawField<T, S>>,
  path_to_grost: Path,
  flavors: Vec<FlavorAttribute>,
  default: Option<Invokable>,
  schema: SchemaAttribute,
  partial: PartialObjectAttribute,
  partial_decoded: PartialDecodedObjectAttribute,
  selector: SelectorAttribute,
  selector_iter: SelectorIterAttribute,
  indexer: IndexerAttribute,
  copy: bool,
  flavor_param: Option<TypeParam>,
  unknown_buffer_param: TypeParam,
  lifetime_param: LifetimeParam,
  wire_format_param: TypeParam,
  read_buffer_type_param: TypeParam,
  write_buffer_type_param: TypeParam,
  extra: O,
}

impl<T, S, O> RawObject<T, S, O> {
  /// Creates a new `RawObject` from the given parameters.
  pub fn new(
    path_to_grost: Path,
    name: Ident,
    vis: Visibility,
    generics: Generics,
    attrs: Vec<Attribute>,
    fields: Vec<RawField<T, S>>,
    meta: ObjectFromMeta<O>,
  ) -> darling::Result<Self> {
    Ok(Self {
      name,
      vis,
      generics,
      attrs,
      fields,
      path_to_grost,
      flavors: meta.flavor.finalize(&path_to_grost)?,
      default: meta.default,
      schema: meta.schema.into(),
      partial: meta.partial.finalize(),
      partial_decoded: meta.partial_decoded.finalize(),
      selector: meta.selector.finalize(),
      selector_iter: meta.selector_iter.finalize(),
      indexer: meta.indexer.into(),
      copy: meta.copy,
      flavor_param: meta.generic.flavor,
      unknown_buffer_param: meta.generic.unknown_buffer,
      lifetime_param: meta.generic.lifetime,
      wire_format_param: meta.generic.wire_format,
      read_buffer_type_param: meta.generic.read_buffer,
      write_buffer_type_param: meta.generic.write_buffer,
      extra: meta.extra,
    })
  }

  /// Returns the path to the `grost` crate
  pub const fn path_to_grost(&self) -> &Path {
    &self.path_to_grost
  }

  /// Returns the path to the fn that returns the default value of the object
  pub const fn default(&self) -> Option<&Invokable> {
    self.default.as_ref()
  }

  /// Returns the schema information
  pub const fn schema(&self) -> &SchemaAttribute {
    &self.schema
  }

  /// Returns the partial object information
  pub const fn partial(&self) -> &PartialObjectAttribute {
    &self.partial
  }

  /// Returns the partial decoded object information
  pub const fn partial_decoded(&self) -> &PartialDecodedObjectAttribute {
    &self.partial_decoded
  }

  /// Returns the selector information
  pub const fn selector(&self) -> &SelectorAttribute {
    &self.selector
  }

  /// Returns the selector iterator information
  pub const fn selector_iter(&self) -> &SelectorIterAttribute {
    &self.selector_iter
  }

  /// Returns the indexer information
  pub const fn indexer(&self) -> &IndexerAttribute {
    &self.indexer
  }

  /// Returns whether the object is copyable
  pub const fn copy(&self) -> bool {
    self.copy
  }

  /// Returns the flavors of the object
  pub const fn flavors(&self) -> &[FlavorAttribute] {
    self.flavors.as_slice()
  }

  /// Returns the generic flavor type parameter
  pub const fn flavor_type_param(&self) -> Option<&TypeParam> {
    self.flavor_param.as_ref()
  }

  /// Returns the generic unknown buffer type parameter
  pub const fn unknown_buffer_type_param(&self) -> &TypeParam {
    &self.unknown_buffer_param
  }

  /// Returns the generic lifetime parameter
  pub const fn lifetime_param(&self) -> &LifetimeParam {
    &self.lifetime_param
  }

  /// Returns the generic wire format type parameter
  pub const fn wire_format_type_param(&self) -> &TypeParam {
    &self.wire_format_param
  }

  /// Returns the generic read buffer type parameter
  pub const fn read_buffer_type_param(&self) -> &TypeParam {
    &self.read_buffer_type_param
  }

  /// Returns the generic write buffer type parameter
  pub const fn write_buffer_type_param(&self) -> &TypeParam {
    &self.write_buffer_type_param
  }

  /// Returns the extra metadata associated with the object
  pub const fn extra(&self) -> &O {
    &self.extra
  }

  #[inline]
  pub fn partial_decoded_name(&self) -> Ident {
    self
      .partial_decoded()
      .name()
      .cloned()
      .unwrap_or_else(|| format_ident!("PartialDecoded{}", self.name()))
  }

  #[inline]
  pub fn partial_name(&self) -> Ident {
    self
      .partial()
      .name()
      .cloned()
      .unwrap_or_else(|| format_ident!("Partial{}", self.name()))
  }

  #[inline]
  pub fn selector_name(&self) -> Ident {
    self
      .selector()
      .name()
      .cloned()
      .unwrap_or_else(|| format_ident!("{}Selector", self.name()))
  }

  #[inline]
  pub fn selector_iter_name(&self) -> Ident {
    self
      .selector_iter()
      .name()
      .cloned()
      .unwrap_or_else(|| format_ident!("{}Iter", self.selector_name()))
  }

  #[inline]
  pub fn indexer_name(&self) -> Ident {
    self
      .indexer()
      .name()
      .cloned()
      .unwrap_or_else(|| format_ident!("{}Index", self.name()))
  }
}
