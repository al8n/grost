use core::num::NonZeroU32;

use darling::{FromField, FromMeta as _, util::SpannedValue};
use hint::TypeHintMeta;
use syn::{Attribute, Ident, Type, Visibility};

use super::SchemaMeta;

pub use hint::TypeHint;
pub use select::Selection;

mod hint;
mod select;

#[derive(Debug, darling::FromMeta)]
pub struct PartialRefFieldMeta {
  #[darling(default)]
  copy: bool,
}

#[derive(Debug, FromField)]
#[darling(attributes(grost), forward_attrs)]
pub struct ObjectField {
  ident: Option<Ident>,
  ty: Type,
  vis: Visibility,
  attrs: Vec<Attribute>,
  schema: SchemaMeta,
  tag: NonZeroU32,
  #[darling(default, rename = "wire")]
  wire_format: Option<Type>,
  #[darling(default)]
  partial_ref: Option<PartialRefFieldMeta>,
  #[darling(default)]
  select: Selection,
  #[darling(default)]
  copy: bool,
  #[darling(flatten)]
  hint: TypeHintMeta,
}

impl ObjectField {
  // pub fn parse_type_hint(&mut self) -> darling::Result<()> {
  //   println!("Parsing type hint for field: {:?}", self.ident);
  //   for attr in &self.attrs {
  //     if attr.path().is_ident("grost") {
  //       println!("Parsing attr: {:?}", attr.path());
  //       attr.parse_nested_meta(|meta| {
  //         println!("Parsing meta: {:?}", meta.path.get_ident());
  //         if meta.path.is_ident("optional") || meta.path.is_ident("repeated") || meta.path.is_ident("map") {
  //           println!("Parsing meta 2: {:?}", meta.path.get_ident());
  //           if let Some(ref hint) = self.hint {
  //             return Err(syn::Error::new_spanned(meta.path, format!("already set to {}", hint)));
  //           }

  //           let content;
  //           syn::parenthesized!(content in meta.input);
  //           let inner: syn::Meta = content.parse()?;
  //           self.hint = Some(TypeHint::from_meta(&inner)?);
  //         }
  //         Ok(())
  //       })?;
  //     }
  //   }

  //   Ok(())
  // }
}
