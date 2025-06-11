use quote::format_ident;
use syn::{Ident, Path, Type, parse_quote};

use crate::meta::flavor::{BuiltinFlavorRepr, FlavorFromMeta};

pub use super::flavor::{
  decode::DecodeAttribute, encode::EncodeAttribute, identifier::IdentifierAttribute, tag::TagAttribute,
};

mod decode;
mod encode;
mod identifier;

impl FlavorFromMeta {
  pub(crate) fn finalize(self, path_to_grost: &Path) -> syn::Result<Vec<FlavorAttribute>> {
    let mut flavors = Vec::new();

    match self.default {
      BuiltinFlavorRepr::Nested(default_flavor_value_parser) => flavors.push(FlavorAttribute {
        name: format_ident!("default"),
        ty: parse_quote!(#path_to_grost::__private::flavors::Network),
        format: parse_quote!(#path_to_grost::__private::flavors::network::LengthDelimited),
        identifier: IdentifierAttribute::network(path_to_grost),
        encode: default_flavor_value_parser.encode.into(),
        decode: default_flavor_value_parser.decode.into(),
      }),
      BuiltinFlavorRepr::Bool(val) => {
        if val {
          flavors.push(FlavorAttribute::network_object(path_to_grost)?);
        }
      }
    }

    for (name, value) in self.flavors {
      let ty = value.ty;
      let format = value.format;
      let identifier = value.identifier.into();
      let encode = value.encode.into();
      let decode = value.decode.into();

      flavors.push(FlavorAttribute {
        name,
        ty,
        format,
        identifier,
        encode,
        decode,
      });
    }

    Ok(flavors)
  }
}

#[derive(Debug, Clone)]
pub struct FlavorAttribute {
  name: Ident,
  ty: Type,
  format: Type,
  identifier: IdentifierAttribute,
  encode: EncodeAttribute,
  decode: DecodeAttribute,
}

impl FlavorAttribute {
  fn network_object(path_to_grost: &Path) -> syn::Result<Self> {
    let ty = parse_quote!(#path_to_grost::__private::flavors::Network);
    let format = parse_quote!(#path_to_grost::__private::flavors::network::LengthDelimited);
    let identifier = IdentifierAttribute::network(path_to_grost);
    let encode = EncodeAttribute::network(path_to_grost)?;
    let decode = DecodeAttribute::network(path_to_grost);

    Ok(Self {
      name: format_ident!("network"),
      ty,
      format,
      identifier,
      encode,
      decode,
    })
  }

  /// Returns the name of the flavor.
  pub const fn name(&self) -> &syn::Ident {
    &self.name
  }

  /// Returns the type of the flavor.
  pub const fn ty(&self) -> &syn::Type {
    &self.ty
  }

  /// Returns the wire format for the type of the flavor.
  ///
  /// e.g. If the macro is used on object, then this will return the wire format for the object.
  pub const fn wire_format(&self) -> &syn::Type {
    &self.format
  }

  /// Returns the identifier attribute of the flavor.
  pub const fn identifier(&self) -> &IdentifierAttribute {
    &self.identifier
  }

  /// Returns the encode attribute of the flavor.
  pub const fn encode(&self) -> &EncodeAttribute {
    &self.encode
  }

  /// Returns the decode attribute of the flavor.
  pub const fn decode(&self) -> &DecodeAttribute {
    &self.decode
  }
}
