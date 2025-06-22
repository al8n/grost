// use darling::FromMeta;
// use indexmap::IndexMap;
// use syn::{Ident, Meta, Type};

// use crate::{
//   flavor::{complex_flavor_ident_error, duplicate_flavor_error},
//   object::meta::{FieldConvertFromMeta, FieldEncodeFromMeta, FieldDecodeFromMeta},
//   utils::NestedMeta,
// };

// #[derive(Debug, Default, Clone, FromMeta)]
// pub(in crate::object) struct PartialRefFieldFlavorFromMeta {
//   pub(in crate::object) encode: FieldEncodeFromMeta,
//   pub(in crate::object) decode: FieldDecodeFromMeta,
//   pub(in crate::object) transform: FieldConvertFromMeta,
//   pub(in crate::object) partial_transform: FieldConvertFromMeta,
// }

// #[derive(Debug, Default, Clone, FromMeta)]
// pub(in crate::object) struct PartialFieldFlavorFromMeta {
//   pub(in crate::object) partial_transform: FieldConvertFromMeta, 
// } 

// #[derive(Debug, Default, Clone)]
// pub(in crate::object) struct FieldFlavorFromMeta {
//   pub(in crate::object) wire_format: Option<Type>,
//   pub(in crate::object) partial_ref: PartialRefFieldFlavorFromMeta,
//   pub(in crate::object) partial: PartialFieldFlavorFromMeta,
// }

// impl FromMeta for FieldFlavorFromMeta {
//   fn from_string(value: &str) -> darling::Result<Self> {
//     let format = syn::parse_str(value).map_err(darling::Error::from)?;

//     Ok(FieldFlavorFromMeta {
//       wire_format: Some(format),
//       partial_ref: PartialRefFieldFlavorFromMeta::default(),
//       partial: PartialFieldFlavorFromMeta::default(),
//     })
//   }

//   fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
//     #[derive(Debug, Default, Clone, FromMeta)]
//     pub(in crate::object) struct Helper {
//       #[darling(default)]
//       pub(in crate::object) wire_format: Option<Type>,
//       #[darling(default)]
//       pub(in crate::object) partial_ref: PartialRefFieldFlavorFromMeta, 
//       #[darling(default)]
//       pub(in crate::object) partial: PartialFieldFlavorFromMeta,
//     }

//     let Helper {
//       wire_format,
//       partial_ref,
//       partial,
//     } = Helper::from_list(items)?;

//     Ok(FieldFlavorFromMeta {
//       wire_format,
//       partial_ref,
//       partial,
//     })
//   }
// }

// #[derive(Debug, Default, Clone)]
// pub struct GenericFieldFlavorFromMeta {
//   pub(in crate::object) flavors: IndexMap<Ident, FieldFlavorFromMeta>,
// }

// impl FromMeta for GenericFieldFlavorFromMeta {
//   fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
//     let mut flavors = IndexMap::new();

//     for item in items {
//       match item {
//         darling::ast::NestedMeta::Lit(_) => {
//           return Err(darling::Error::unsupported_format("literal"));
//         }
//         darling::ast::NestedMeta::Meta(meta) => {
//           let ident = meta
//             .path()
//             .get_ident()
//             .ok_or_else(complex_flavor_ident_error)?;

//           if ident.eq("network") {
//             return Err(darling::Error::custom(
//               "The `network` flavor is reserved and equivalent to the default flavor. Use `default` to configure the default behavior instead.",
//             ));
//           }

//           match meta {
//             Meta::Path(_) => {
//               return Err(darling::Error::unsupported_format("word"));
//             }
//             Meta::NameValue(name_value) => {
//               if flavors.contains_key(ident) {
//                 return Err(duplicate_flavor_error(ident));
//               }

//               match &name_value.value {
//                 syn::Expr::Lit(lit) => match &lit.lit {
//                   syn::Lit::Str(lit_str) => {
//                     let format = syn::parse_str(&lit_str.value()).map_err(darling::Error::from)?;

//                     flavors.insert(
//                       ident.clone(),
//                       FieldFlavorFromMeta {
//                         ty: None,
//                         format: Some(format),
//                         encode: EncodeFromMeta::default(),
//                         decode: DecodeFromMeta::default(),
//                         convert: ConvertFromMeta::default(),
//                       },
//                     );
//                   }
//                   lit => return Err(darling::Error::unexpected_lit_type(lit)),
//                 },
//                 expr => {
//                   return Err(darling::Error::unexpected_expr_type(expr));
//                 }
//               }
//             }
//             Meta::List(_) => {
//               // let nested_meta = NestedMeta::parse_meta_list(meta_list.tokens.clone())?;
//               let value = FieldFlavorFromMeta::from_meta(meta)?;
//               if flavors.contains_key(ident) {
//                 return Err(duplicate_flavor_error(ident));
//               }

//               flavors.insert(
//                 ident.clone(),
//                 FieldFlavorFromMeta {
//                   ty: value.ty,
//                   format: value.format,
//                   encode: value.encode,
//                   decode: value.decode,
//                   convert: value.convert,
//                 },
//               );
//             }
//           }
//         }
//       }
//     }

//     Ok(GenericFieldFlavorFromMeta { flavors })
//   }
// }

// #[derive(Debug, Default, Clone)]
// pub struct FieldFlavorFromMeta {
//   pub(in crate::object) flavor_type: Option<syn::Type>,
//   pub(in crate::object) format: Option<syn::Type>,
//   pub(in crate::object) encode: EncodeFromMeta,
//   pub(in crate::object) decode: DecodeFromMeta,
//   pub(in crate::object) convert: ConvertFromMeta,
// }