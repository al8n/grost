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

impl ObjectFromMeta {
  pub fn finalize(self, path_to_grost: Path) -> syn::Result<ObjectAttribute> {
    let flavors = self.flavor.finalize(&path_to_grost)?;
    let mut flavor_generic = self.generic.flavor;
    if flavors.len() > 1 {
      flavor_generic.get_or_insert_with(grost_flavor_param);
    }

    Ok(ObjectAttribute {
      path_to_grost,
      flavors,
      default: self.default,
      schema: self.schema.into(),
      partial: self.partial.finalize(),
      partial_decoded: self.partial_decoded.finalize(),
      selector: self.selector.finalize(),
      selector_iter: self.selector_iter.finalize(),
      flavor_param: flavor_generic,
      lifetime_param: self.generic.lifetime,
      unknown_buffer_param: self.generic.unknown_buffer,
      wire_format_param: self.generic.wire_format,
      read_buffer_type_param: self.generic.read_buffer,
      write_buffer_type_param: self.generic.write_buffer,
      indexer: self.indexer.into(),
      copy: self.copy,
    })
  }
}

#[derive(Debug, Clone)]
pub struct ObjectAttribute {
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
}

impl ObjectAttribute {
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
}

/// The trait for the object derive input
pub trait RawObject: Clone {
  /// The type of the field
  type Field: RawField;
  /// The custom metadata type for the object
  type Meta: Clone;

  /// Returns the name of the object
  fn name(&self) -> &Ident;

  /// Returns the visibility of the object
  fn vis(&self) -> &Visibility;

  /// Returns the generics in the object defination.
  fn generics(&self) -> &Generics;

  /// Returns the attributes in the object defination.
  fn attrs(&self) -> &[Attribute];

  /// Returns the fields of the object
  fn fields(&self) -> Vec<&Self::Field>;

  /// Returns the path to the `grost` crate
  fn path_to_grost(&self) -> &Path;

  /// Returns the path to the fn that returns the default value of the object
  fn default(&self) -> Option<&Invokable>;

  /// Returns the schema information
  fn schema(&self) -> &SchemaAttribute;

  /// Returns the partial object information
  fn partial(&self) -> &PartialObjectAttribute;

  /// Returns the partial decoded object information
  fn partial_decoded(&self) -> &PartialDecodedObjectAttribute;

  /// Returns the selector information
  fn selector(&self) -> &SelectorAttribute;

  /// Returns the selector iterator information
  fn selector_iter(&self) -> &SelectorIterAttribute;

  /// Returns the indexer information
  fn indexer(&self) -> &IndexerAttribute;

  /// Returns whether the object is copyable
  fn copy(&self) -> bool;

  /// Returns the metadata associated with the object
  fn meta(&self) -> &Self::Meta;

  /// Returns the flavors of the object
  fn flavors(&self) -> &[FlavorAttribute];

  /// Returns the generic flavor type parameter, if any,
  /// `None` if we only have one flavor and the code is not generic over the flavor type.
  fn flavor_type_param(&self) -> Option<&TypeParam>;

  /// Returns the generic unknown buffer type parameter
  fn unknown_buffer_type_param(&self) -> &TypeParam;

  /// Returns the generic lifetime parameter
  fn lifetime_param(&self) -> &LifetimeParam;

  /// Returns the generic wire format type parameter
  fn wire_format_type_param(&self) -> &TypeParam;

  /// Returns the read buffer type parameter
  fn read_buffer_type_param(&self) -> &TypeParam;

  /// Returns the write buffer type parameter
  fn write_buffer_type_param(&self) -> &TypeParam;
}

/// The extension trait for the object
pub trait RawObjectExt: RawObject {
  #[inline]
  fn partial_decoded_name(&self) -> Ident {
    self
      .partial_decoded()
      .name()
      .cloned()
      .unwrap_or_else(|| format_ident!("PartialDecoded{}", self.name()))
  }

  #[inline]
  fn partial_name(&self) -> Ident {
    self
      .partial()
      .name()
      .cloned()
      .unwrap_or_else(|| format_ident!("Partial{}", self.name()))
  }

  #[inline]
  fn selector_name(&self) -> Ident {
    self
      .selector()
      .name()
      .cloned()
      .unwrap_or_else(|| format_ident!("{}Selector", self.name()))
  }

  #[inline]
  fn selector_iter_name(&self) -> Ident {
    self
      .selector_iter()
      .name()
      .cloned()
      .unwrap_or_else(|| format_ident!("{}Iter", self.selector_name()))
  }

  #[inline]
  fn indexer_name(&self) -> Ident {
    self
      .indexer()
      .name()
      .cloned()
      .unwrap_or_else(|| format_ident!("{}Index", self.name()))
  }
}

impl<T: RawObject> RawObjectExt for T {}
