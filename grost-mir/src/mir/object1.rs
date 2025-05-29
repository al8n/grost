use std::num::NonZeroU32;

use indexmap::IndexMap;
use syn::{punctuated::Punctuated, token::Comma, Ident, Attribute, Generics, LifetimeParam, Type, TypeParam, Visibility, WhereClause};

use crate::ast::{object::{FieldDecodeAttribute, FieldEncodeAttribute}, DecodeAttribute, EncodeAttribute, IdentifierAttribute, MissingOperation, TransformOperation};

#[derive(Debug, Clone)]
pub struct FieldFlavor {
  wire_format: Type,
  encode: FieldEncodeAttribute,
  decode: FieldDecodeAttribute,
}

#[derive(Debug, Clone)]
pub struct PartialDecodedField {
  ty: Type,
  innermost_type: Type,
  attrs: Vec<Attribute>,
  copy: bool,
}

#[derive(Debug, Clone)]
pub struct PartialField {
  ty: Type,
  innermost_type: Type,
  attrs: Vec<Attribute>,
}

#[derive(Debug, Clone)]
pub struct TaggedField {
  attrs: Vec<Attribute>,
  vis: Visibility,
  name: Ident,
  ty: Type,
  tag: NonZeroU32,
  partial: PartialField,
  partial_decoded: PartialDecodedField,
  missing_operation: Option<MissingOperation>,
  transform_operation: Option<TransformOperation>,
  flavors: IndexMap<Ident, FieldFlavor>,
}

#[derive(Debug, Clone)]
pub struct SkippedField {
  attrs: Vec<Attribute>,
  vis: Visibility,
  name: Ident,
  ty: Type,
}

#[derive(Debug, Clone, derive_more::IsVariant, derive_more::Unwrap, derive_more::TryUnwrap)]
#[unwrap(ref)]
#[try_unwrap(ref)]
pub enum Field {
  Tagged(TaggedField),
  Skipped(SkippedField),
}


#[derive(Debug, Clone)]
pub struct ConcretePartialDecodedObject {
  generics: Generics,
  flavor_type: Type,
  unknown_buffer_param: TypeParam,
  lifetime_param: LifetimeParam,
}

#[derive(Debug, Clone)]
pub struct PartialDecodedObject {
  name: Ident,
  ty: Type,
  attrs: Vec<Attribute>,
  vis: Visibility,
  generics: Generics,
  flavor_param: Option<TypeParam>,
  unknown_buffer_param: TypeParam,
  lifetime_param: LifetimeParam,
  flavors: IndexMap<Ident, ConcretePartialDecodedObject>,
}


pub struct Flavor {
  ty: Type,
  wire_format: Type,
  identifier: IdentifierAttribute,
  encode: EncodeAttribute,
  decode: DecodeAttribute,
}


pub struct Object {
  name: Ident,
  ty: Type,
  generics: syn::Generics,
  partial_decoded: PartialDecodedObject,
  flavors: IndexMap<Ident, Flavor>,
  fields: Vec<Field>,
}
