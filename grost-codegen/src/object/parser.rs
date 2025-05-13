use darling::{FromDeriveInput, FromField, FromMeta, ast::Data, util::Ignored};
use syn::Generics;

use super::*;

pub use field::{ObjectField, PartialRefFieldMeta, Selection};

mod field;

#[derive(Default, Debug, FromMeta)]
pub struct SchemaMeta {
  name: Option<String>,
  description: Option<String>,
}

#[derive(Debug, FromDeriveInput)]
#[darling(
  attributes(grost),
  supports(struct_named),
  forward_attrs,
  and_then = "validate_fields"
)]
pub struct ObjectDeriveInput {
  ident: Ident,
  vis: Visibility,
  generics: Generics,
  attrs: Vec<Attribute>,
  data: Data<Ignored, ObjectField>,
}

fn validate_fields(mut input: ObjectDeriveInput) -> Result<ObjectDeriveInput, darling::Error> {
  // match input.data {
  //   Data::Struct(ref mut data) => {
  //     for field in &mut data.fields {
  //       field.parse_type_hint()?;
  //     }
  //   }
  //   Data::Enum(_) => unreachable!(),
  // }

  Ok(input)
}

// #[derive(Default, derive_more::IsVariant, derive_more::Display)]
// enum FieldTypeKind {
//   #[default]
//   #[display("type")]
//   Type,
//   #[display("optional")]
//   Optional,
//   #[display("repeated")]
//   Repeated,
//   #[display("map")]
//   Map,
// }
