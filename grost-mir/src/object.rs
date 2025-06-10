pub use meta::{FieldSelection, Label};

pub use ast::{
  ConvertAttribute, FieldAttribute, FieldDecodeAttribute, FieldEncodeAttribute, FieldFlavor,
  FieldFlavorAttribute, Indexer, IndexerAttribute, ObjectAttribute, ObjectFlavor,
  PartialDecodedFieldAttribute, PartialDecodedObjectAttribute, PartialFieldAttribute,
  PartialObjectAttribute, RawField, RawObject, RawObjectExt, SelectorAttribute,
  SelectorFieldAttribute, SelectorIterAttribute,
};
pub use mir::*;

mod ast;
/// The meta for the object and object's fields
pub mod meta;
mod mir;
