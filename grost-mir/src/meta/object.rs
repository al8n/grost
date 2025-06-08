use darling::FromMeta;
use syn::Path;

use super::{SchemaFromMeta, flavor::FlavorFromMeta, generic::GenericFromMeta};

use indexer::IndexerFromMeta;
use partial::PartialObjectFromMeta;
use partial_decoded::PartialDecodedObjectFromMeta;
use selector::{SelectorFromMeta, SelectorIterFromMeta};

pub use field::*;

pub(crate) mod field;
pub(crate) mod indexer;
pub(crate) mod partial;
pub(crate) mod partial_decoded;
pub(crate) mod selector;

#[derive(Debug, Default, Clone, FromMeta)]
pub struct ObjectFromMeta {
  #[darling(default)]
  pub(crate) default: Option<Path>,
  #[darling(default)]
  pub(crate) generic: GenericFromMeta,
  #[darling(default)]
  pub(crate) schema: SchemaFromMeta,
  #[darling(default)]
  pub(crate) partial: PartialObjectFromMeta,
  #[darling(default)]
  pub(crate) partial_decoded: PartialDecodedObjectFromMeta,
  #[darling(default)]
  pub(crate) selector: SelectorFromMeta,
  #[darling(default)]
  pub(crate) selector_iter: SelectorIterFromMeta,
  #[darling(default)]
  pub(crate) indexer: IndexerFromMeta,
  #[darling(default)]
  pub(crate) flavor: FlavorFromMeta,
  #[darling(default)]
  pub(crate) copy: bool,
}
