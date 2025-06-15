use quote::quote;
use syn::{Generics, Ident, Type};

use crate::{
  flavor::{DecodeAttribute, EncodeAttribute, IdentifierAttribute, TagAttribute},
  object::ObjectFlavor as ObjectFlavorAst,
};

pub use partial_decoded::PartialDecodedObjectFlavor;
pub use selector::{SelectorFlavor, SelectorIterFlavor};

mod partial_decoded;
mod selector;

#[derive(Debug, Clone)]
pub struct ObjectFlavor {
  flavor_type: Type,
  format: Type,
  identifier: IdentifierAttribute,
  tag: TagAttribute,
  encode: EncodeAttribute,
  decode: DecodeAttribute,
  selector: SelectorFlavor,
  selector_iter: SelectorIterFlavor,
  partial_decoded: PartialDecodedObjectFlavor,
  reflection_type: Type,
  wire_format_reflection_generics: Generics,
  wire_type_reflection_generics: Generics,
  identifier_reflection_generics: Generics,
  encoded_identifier_reflection_generics: Generics,
  encoded_identifier_len_reflection_generics: Generics,
  tag_reflection_generics: Generics,
  encoded_tag_reflection_generics: Generics,
  encoded_tag_len_reflection_generics: Generics,
}

impl ObjectFlavor {
  /// Returns the type of the flavor
  #[inline]
  pub const fn flavor_type(&self) -> &Type {
    &self.flavor_type
  }

  /// Returns the reflection type for the flavor.
  #[inline]
  pub const fn reflection_type(&self) -> &Type {
    &self.reflection_type
  }

  /// Returns the wire format type for the object of this flavor.
  #[inline]
  pub const fn wire_format(&self) -> &Type {
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

  /// Returns the generics for the wire format reflection.
  #[inline]
  pub const fn wire_format_reflection_generics(&self) -> &Generics {
    &self.wire_format_reflection_generics
  }

  /// Returns the generics for the wire type reflection.
  #[inline]
  pub const fn wire_type_reflection_generics(&self) -> &Generics {
    &self.wire_type_reflection_generics
  }

  /// Returns the generics for the identifier reflection.
  #[inline]
  pub const fn identifier_reflection_generics(&self) -> &Generics {
    &self.identifier_reflection_generics
  }

  /// Returns the generics for the encoded identifier reflection.
  #[inline]
  pub const fn encoded_identifier_reflection_generics(&self) -> &Generics {
    &self.encoded_identifier_reflection_generics
  }

  /// Returns the generics for the encoded identifier length reflection.
  #[inline]
  pub const fn encoded_identifier_len_reflection_generics(&self) -> &Generics {
    &self.encoded_identifier_len_reflection_generics
  }

  /// Returns the generics for the tag reflection.
  #[inline]
  pub const fn tag_reflection_generics(&self) -> &Generics {
    &self.tag_reflection_generics
  }

  /// Returns the generics for the encoded tag reflection.
  #[inline]
  pub const fn encoded_tag_reflection_generics(&self) -> &Generics {
    &self.encoded_tag_reflection_generics
  }

  /// Returns the generics for the encoded tag length reflection.
  #[inline]
  pub const fn encoded_tag_len_reflection_generics(&self) -> &Generics {
    &self.encoded_tag_len_reflection_generics
  }

  /// Returns the selector information for this object flavor.
  #[inline]
  pub const fn selector(&self) -> &SelectorFlavor {
    &self.selector
  }

  /// Returns the selector iterator information for this object flavor.
  #[inline]
  pub const fn selector_iter(&self) -> &SelectorIterFlavor {
    &self.selector_iter
  }

  /// Returns the partial decoded object flavor information.
  #[inline]
  pub const fn partial_decoded(&self) -> &PartialDecodedObjectFlavor {
    &self.partial_decoded
  }

  pub(super) fn from_ast<M, F>(
    object: &super::GenericObjectAst<M, F>,
    flavor_name: &Ident,
    ast: &ObjectFlavorAst,
    fields: &[super::GenericField<F>],
  ) -> darling::Result<Self> {
    let path_to_grost = object.path_to_grost();
    let generics = object.generics();
    let object_type = object.ty();
    let flavor_ty = ast.ty();
    let mut wire_format_reflection_generics = generics.clone();
    let mut wire_type_reflection_generics = generics.clone();
    let mut identifier_reflection_generics = generics.clone();
    let mut encoded_identifier_reflection_generics = generics.clone();
    let mut encoded_identifier_len_reflection_generics = generics.clone();
    let mut tag_reflection_generics = generics.clone();
    let mut encoded_tag_reflection_generics = generics.clone();
    let mut encoded_tag_len_reflection_generics = generics.clone();

    let reflection_type: Type = syn::parse2(quote! {
      #path_to_grost::__private::reflection::ObjectReflection<
        #object_type,
        #path_to_grost::__private::reflection::Object,
        #flavor_ty,
      >
    })?;

    fields
      .iter()
      .filter_map(|f| f.try_unwrap_tagged_ref().ok())
      .try_for_each(|f| {
        macro_rules! bail {
          ($field:ident: $($generics:ident),+$(,)?) => {{
            paste::paste! {
              $(
                [< $generics _generics>].make_where_clause()
                  .predicates
                  .extend($field.[< $generics _constraints >]().iter().cloned());
              )*
            }
          }};
        }

        let flavors = f.flavors();
        if let Some(flavor) = flavors.get(flavor_name) {
          bail!(flavor:
            wire_format_reflection,
            wire_type_reflection,
            identifier_reflection,
            encoded_identifier_reflection,
            encoded_identifier_len_reflection,
            tag_reflection,
            encoded_tag_reflection,
            encoded_tag_len_reflection
          );
        } else {
          return Err(darling::Error::custom(format!(
            "Flavor `{}` not found for field `{}`",
            flavor_name,
            f.name()
          )));
        }

        darling::Result::Ok(())
      })?;

    let partial_decoded =
      PartialDecodedObjectFlavor::from_ast(flavor_name, flavor_ty, object, fields)?;
    let selector = SelectorFlavor::from_ast(flavor_name, flavor_ty, object, fields)?;
    let selector_iter = selector.selector_iter(object, flavor_ty)?;
    Ok(Self {
      partial_decoded,
      selector,
      selector_iter,
      flavor_type: flavor_ty.clone(),
      reflection_type,
      format: ast.format().clone(),
      identifier: ast.identifier().clone(),
      tag: ast.tag().clone(),
      encode: ast.encode().clone(),
      decode: ast.decode().clone(),
      wire_format_reflection_generics,
      wire_type_reflection_generics,
      identifier_reflection_generics,
      encoded_identifier_reflection_generics,
      encoded_identifier_len_reflection_generics,
      tag_reflection_generics,
      encoded_tag_reflection_generics,
      encoded_tag_len_reflection_generics,
    })
  }
}
