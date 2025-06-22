use quote::format_ident;
use syn::{Ident, Path, Type, parse_quote};

pub use decode::DecodeOptions;
pub use identifier::IdentifierOptions;
pub use tag::TagOptions;

pub(crate) use meta::{
  BuiltinFlavorRepr, DecodeFromMeta, FlavorFromMeta, GenericFlavorFromMeta, IdentifierFromMeta,
  TagFromMeta, complex_flavor_ident_error, duplicate_flavor_error,
};

mod decode;
mod identifier;
mod meta;
mod tag;

// impl GenericFlavorFromMeta {
//   pub(crate) fn finalize(self, path_to_grost: &Path) -> syn::Result<Vec<FlavorAttribute>> {
//     let mut flavors = Vec::new();

//     match self.default {
//       BuiltinFlavorRepr::Nested(default_flavor_value_parser) => flavors.push(FlavorAttribute {
//         name: format_ident!("default"),
//         ty: parse_quote!(#path_to_grost::__private::flavors::Network),
//         format: parse_quote!(#path_to_grost::__private::flavors::network::LengthDelimited),
//         identifier: IdentifierOptions::network(path_to_grost),
//         tag: TagOptions::network(path_to_grost),
//         decode: default_flavor_value_parser.decode.into(),
//       }),
//       BuiltinFlavorRepr::Bool(val) => {
//         if val {
//           flavors.push(FlavorAttribute::network_object(path_to_grost)?);
//         }
//       }
//     }

//     for (name, value) in self.flavors {
//       let ty = value.ty;
//       let format = value.format;
//       let identifier = value.identifier.into();
//       let tag = value.tag.into();
//       let decode = value.decode.into();

//       flavors.push(FlavorAttribute {
//         name,
//         ty,
//         format,
//         tag,
//         identifier,
//         decode,
//       });
//     }

//     Ok(flavors)
//   }
// }

// #[derive(Debug, Clone)]
// pub struct FlavorAttribute {
//   name: Ident,
//   ty: Type,
//   format: Type,
//   identifier: IdentifierOptions,
//   tag: TagOptions,
//   decode: DecodeOptions,
// }

// impl FlavorAttribute {
//   fn network_object(path_to_grost: &Path) -> syn::Result<Self> {
//     let ty = parse_quote!(#path_to_grost::__private::flavors::Network);
//     let format = parse_quote!(#path_to_grost::__private::flavors::network::LengthDelimited);
//     let identifier = IdentifierOptions::network(path_to_grost);
//     let tag = TagOptions::network(path_to_grost);
//     let decode = DecodeOptions::network(path_to_grost);

//     Ok(Self {
//       name: format_ident!("network"),
//       ty,
//       format,
//       identifier,
//       tag,
//       decode,
//     })
//   }

//   /// Returns the name of the flavor.
//   pub const fn name(&self) -> &syn::Ident {
//     &self.name
//   }

//   /// Returns the type of the flavor.
//   pub const fn ty(&self) -> &syn::Type {
//     &self.ty
//   }

//   /// Returns the wire format for the type of the flavor.
//   ///
//   /// e.g. If the macro is used on object, then this will return the wire format for the object.
//   pub const fn wire_format(&self) -> &syn::Type {
//     &self.format
//   }

//   /// Returns the identifier attribute of the flavor.
//   pub const fn identifier(&self) -> &IdentifierOptions {
//     &self.identifier
//   }

//   /// Returns the tag attribute of the flavor.
//   pub const fn tag(&self) -> &TagOptions {
//     &self.tag
//   }

//   /// Returns the decode attribute of the flavor.
//   pub const fn decode(&self) -> &DecodeOptions {
//     &self.decode
//   }
// }
