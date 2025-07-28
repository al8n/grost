pub use identifier::IdentifierOptions;
pub use tag::TagOptions;

pub(crate) use meta::{GenericFlavorFromMeta, IdentifierFromMeta, TagFromMeta};

mod identifier;
mod meta;
mod tag;

// impl GenericFlavorFromMeta {
//   pub(crate) fn finalize(self, path_to_grost: &Path) -> syn::Result<Vec<FlavorAttribute>> {
//     let mut flavors = Vec::new();

//     match self.default {
//       BuiltinFlavorRepr::Nested(default_flavor_value_parser) => flavors.push(FlavorAttribute {
//         name: format_ident!("default"),
//         ty: parse_quote!(#path_to_grost::__private::flavors::Groto),
//         format: parse_quote!(#path_to_grost::__private::flavors::groto::LengthDelimited),
//         identifier: IdentifierOptions::groto(path_to_grost),
//         tag: TagOptions::groto(path_to_grost),
//         decode: default_flavor_value_parser.decode.into(),
//       }),
//       BuiltinFlavorRepr::Bool(val) => {
//         if val {
//           flavors.push(FlavorAttribute::groto_object(path_to_grost)?);
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
//   fn groto_object(path_to_grost: &Path) -> syn::Result<Self> {
//     let ty = parse_quote!(#path_to_grost::__private::flavors::Groto);
//     let format = parse_quote!(#path_to_grost::__private::flavors::groto::LengthDelimited);
//     let identifier = IdentifierOptions::groto(path_to_grost);
//     let tag = TagOptions::groto(path_to_grost);
//     let decode = DecodeOptions::groto(path_to_grost);

//     Ok(Self {
//       name: format_ident!("groto"),
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
