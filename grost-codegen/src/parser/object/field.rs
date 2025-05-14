use core::num::NonZeroU32;

use darling::{FromField, FromMeta};
use hint::TypeHintMeta;
use syn::{Attribute, Ident, Type, Visibility};

use super::{Attributes, SchemaMeta};

pub use hint::TypeHint;
pub use select::Selection;

mod hint;
mod select;

#[derive(Debug, Default, FromMeta)]
pub struct PartialRefFieldMeta {
  #[darling(default)]
  copy: bool,
  #[darling(default, map = "Attributes::into")]
  attrs: Attributes,
}

impl PartialRefFieldMeta {
  pub fn attrs(&self) -> &[Attribute] {
    &self.attrs.0
  }

  pub fn copy(&self) -> bool {
    self.copy
  }
}

#[derive(Debug, Default, FromMeta)]
pub struct PartialFieldMeta {
  #[darling(default, map = "Attributes::into")]
  attrs: Attributes,
}

impl PartialFieldMeta {
  pub fn attrs(&self) -> &[Attribute] {
    &self.attrs.0
  }
}

#[derive(Debug, FromField)]
#[darling(attributes(grost), forward_attrs)]
pub struct ObjectFieldDeriveInput {
  ident: Option<Ident>,
  ty: Type,
  vis: Visibility,
  attrs: Vec<Attribute>,
  #[darling(default)]
  schema: SchemaMeta,
  #[darling(default)]
  default: Option<syn::Path>,
  tag: NonZeroU32,
  #[darling(default)]
  wire: Option<Type>,
  #[darling(default)]
  partial: PartialFieldMeta,
  #[darling(default)]
  partial_ref: PartialRefFieldMeta,
  #[darling(default)]
  select: Selection,
  #[darling(default)]
  copy: bool,
  #[darling(flatten)]
  hint: TypeHintMeta,
}

impl ObjectFieldDeriveInput {
  pub fn name(&self) -> &Ident {
    &self.ident.as_ref().expect("Field name is required")
  }

  pub fn ty(&self) -> &Type {
    &self.ty
  }

  pub fn vis(&self) -> &Visibility {
    &self.vis
  }

  pub fn tag(&self) -> NonZeroU32 {
    self.tag
  }

  pub fn wire(&self) -> Option<&Type> {
    self.wire.as_ref()
  }

  pub fn partial(&self) -> &PartialFieldMeta {
    &self.partial
  }

  pub fn partial_ref(&self) -> &PartialRefFieldMeta {
    &self.partial_ref
  }

  pub fn copy(&self) -> bool {
    self.copy
  }

  pub fn selection(&self) -> &Selection {
    &self.select
  }

  pub fn hint(&self) -> TypeHint {
    self.hint.clone().into()
  }

  pub fn attrs(&self) -> &[Attribute] {
    &self.attrs
  }

  pub fn schema(&self) -> &SchemaMeta {
    &self.schema
  }

  pub fn default(&self) -> Option<&syn::Path> {
    self.default.as_ref()
  }
}
