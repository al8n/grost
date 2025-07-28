pub use field::*;
pub(super) use indexer::IndexerFromMeta;
pub(super) use selector::{SelectorFromMeta, SelectorIterFromMeta};

pub use concrete::{FieldFromMeta, ObjectFromMeta};
pub use generic::GenericObjectFromMeta;

mod field;
mod indexer;
mod partial;
mod selector;

/// Concrete object meta, a concrete object means there will only be one flavor and the generated code will not be generic over the flavor type.
pub mod concrete;
/// Generic object meta, a generic object means the generated code will be generic over the flavor type.
pub mod generic;
