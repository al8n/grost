
use indexmap::IndexMap;
use syn::{punctuated::Punctuated, token::Comma, Attribute, Generics, LifetimeParam, Type, TypeParam, Visibility, WhereClause};

use super::*;

mod field;

#[derive(Debug, Clone)]
pub struct ConcretePartialDecodedObject {
  generics: Generics,
  flavor_type: Type,
  unknown_buffer_param: TypeParam,
  lifetime_param: LifetimeParam,
  /// Constraints when implementing `Encode` trait for this object.
  encode_constraints: Punctuated<WhereClause, Comma>,
  /// Constraints when implementing `PartialEncode` trait for this object.
  partial_encode_constraints: Punctuated<WhereClause, Comma>,
  /// Constraints when implementing `Decode` trait for this object.
  decode_constraints: Punctuated<WhereClause, Comma>,
  /// Constraints when implementing `PartialDecode` trait for this object.
  partial_decode_constraints: Punctuated<WhereClause, Comma>,
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

