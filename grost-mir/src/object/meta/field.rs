use darling::FromMeta;

pub use convert::{
  FieldConvertFromMeta, FieldDecodeFromMeta, FieldEncodeFromMeta, FieldSkipEncodeOperation,
};
pub use label::Label;
pub use selector::{FieldSelection, SelectorFieldFromMeta};

use crate::utils::{Attributes, Invokable};

mod convert;
mod label;
mod selector;

#[derive(Debug, Clone, FromMeta)]
pub struct SkippedFieldFromMeta<M = ()> {
  #[darling(default)]
  pub(in crate::object) default: Option<Invokable>,
  #[darling(flatten)]
  pub(in crate::object) extra: M,
}
