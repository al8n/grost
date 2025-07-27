use darling::FromMeta;

pub use convert::{
  FieldConvertFromMeta, FieldDecodeFromMeta, FieldEncodeFromMeta, FieldSkipEncodeOperation,
  PartialFieldConvertFromMeta,
};
pub use label::Label;
pub use selector::{FieldSelection, SelectorFieldFromMeta};

use crate::utils::{Attributes, Invokable};

mod convert;
mod label;
mod selector;

#[derive(Debug, Clone, FromMeta)]
pub struct SkippedFieldFromMeta<M = ()> {
  #[darling(flatten)]
  pub(in crate::object) extra: M,
  #[darling(default = default_invokable)]
  pub(in crate::object) default: Invokable,
}

fn default_invokable() -> Invokable {
  let path: syn::Path = syn::parse_quote! {
    ::core::default::Default::default
  };
  Invokable::from(path)
}

/// A raw field meta trait that links the tagged and skipped meta types together.
pub trait RawFieldMeta {
  /// The type of skipped meta for this field.
  type Skipped;
  /// The type of tagged meta for this field.
  type Tagged;
}
