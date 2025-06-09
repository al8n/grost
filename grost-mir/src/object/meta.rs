use darling::FromMeta;
use syn::Path;

use crate::{flavor::FlavorFromMeta, generic::GenericFromMeta, utils::SchemaFromMeta};

pub(super) use indexer::IndexerFromMeta;
pub(super) use partial::PartialObjectFromMeta;
pub(super) use partial_decoded::PartialDecodedObjectFromMeta;
pub(super) use selector::{SelectorFromMeta, SelectorIterFromMeta};

pub use field::*;

mod field;
mod indexer;
mod partial;
mod partial_decoded;
mod selector;

#[derive(Debug, Default, Clone, FromMeta)]
pub struct ObjectFromMeta {
  #[darling(default)]
  pub(super) default: Option<Path>,
  #[darling(default)]
  pub(super) generic: GenericFromMeta,
  #[darling(default)]
  pub(super) schema: SchemaFromMeta,
  #[darling(default)]
  pub(super) partial: PartialObjectFromMeta,
  #[darling(default)]
  pub(super) partial_decoded: PartialDecodedObjectFromMeta,
  #[darling(default)]
  pub(super) selector: SelectorFromMeta,
  #[darling(default)]
  pub(super) selector_iter: SelectorIterFromMeta,
  #[darling(default)]
  pub(super) indexer: IndexerFromMeta,
  #[darling(default)]
  pub(super) flavor: FlavorFromMeta,
  #[darling(default)]
  pub(super) copy: bool,
}
