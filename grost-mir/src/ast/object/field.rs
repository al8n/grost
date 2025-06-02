use core::num::NonZeroU32;

use darling::FromMeta;
use syn::{Attribute, Ident, Meta, Type, parse::Parser};

use crate::ast::{NestedMetaWithTypeField, SchemaAttribute};

use super::{Attributes, SchemaFromMeta};
use select::SelectorFieldFromMeta;

pub use convert::ConvertAttribute;
pub use flavor::{DecodeAttribute as FieldDecodeAttribute, EncodeAttribute as FieldEncodeAttribute, FieldFlavorAttribute};
pub use label::Label;
pub use select::{Selection, SelectorFieldAttribute};

mod convert;
mod flavor;
mod label;
mod select;

/// The meta of the partial reference object field
#[derive(Debug, Default, Clone)]
struct PartialDecodedFieldFromMeta {
  copy: bool,
  attrs: Vec<Attribute>,
  ty: Option<Type>,
}

impl FromMeta for PartialDecodedFieldFromMeta {
  fn from_meta(item: &Meta) -> darling::Result<Self> {
    (match *item {
      Meta::Path(_) => Self::from_word(),
      Meta::List(ref value) => {
        let punctuated =
          syn::punctuated::Punctuated::<NestedMetaWithTypeField, syn::Token![,]>::parse_terminated
            .parse2(value.tokens.clone())?;

        let mut nested_meta = Vec::new();
        let mut ty = None;
        for item in punctuated {
          match item {
            NestedMetaWithTypeField::Type(t) => {
              if ty.is_some() {
                return Err(darling::Error::duplicate_field("type"));
              }
              ty = Some(t);
            }
            NestedMetaWithTypeField::Nested(value) => {
              nested_meta.push(value);
            }
          }
        }

        /// The meta of the partial reference object field
        #[derive(Debug, Default, Clone, FromMeta)]
        struct Helper {
          copy: bool,
          #[darling(default, map = "Attributes::into_inner")]
          attrs: Vec<Attribute>,
        }

        let Helper { copy, attrs } = Helper::from_list(&nested_meta)?;
        Ok(Self { copy, attrs, ty })
      }
      Meta::NameValue(ref value) => Self::from_expr(&value.value),
    })
    .map_err(|e| e.with_span(item))
  }
}

impl PartialDecodedFieldFromMeta {
  /// Finalizes the partial decoded field meta and returns the attribute
  pub fn finalize(self) -> PartialDecodedFieldAttribute {
    PartialDecodedFieldAttribute {
      copy: self.copy,
      attrs: self.attrs,
      ty: self.ty,
    }
  }
}

#[derive(Debug, Clone)]
pub struct PartialDecodedFieldAttribute {
  copy: bool,
  attrs: Vec<Attribute>,
  ty: Option<Type>,
}

impl PartialDecodedFieldAttribute {
  /// Returns the attributes of the partial reference object field
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns whether the partial reference object field is copyable
  pub const fn copy(&self) -> bool {
    self.copy
  }

  /// Returns the type of the partial decoded object field
  pub const fn ty(&self) -> Option<&Type> {
    self.ty.as_ref()
  }
}

/// The meta of the partial object field
#[derive(Debug, Default, Clone)]
struct PartialFieldFromMeta {
  attrs: Vec<Attribute>,
  ty: Option<Type>,
}

impl FromMeta for PartialFieldFromMeta {
  fn from_meta(item: &Meta) -> darling::Result<Self> {
    (match *item {
      Meta::Path(_) => Self::from_word(),
      Meta::List(ref value) => {
        let punctuated =
          syn::punctuated::Punctuated::<NestedMetaWithTypeField, syn::Token![,]>::parse_terminated
            .parse2(value.tokens.clone())?;

        let mut nested_meta = Vec::new();
        let mut ty = None;
        for item in punctuated {
          match item {
            NestedMetaWithTypeField::Type(t) => {
              if ty.is_some() {
                return Err(darling::Error::duplicate_field("type"));
              }
              ty = Some(t);
            }
            NestedMetaWithTypeField::Nested(value) => {
              nested_meta.push(value);
            }
          }
        }

        /// The meta of the partial reference object field
        #[derive(Debug, Default, Clone, FromMeta)]
        struct Helper {
          #[darling(default, map = "Attributes::into_inner")]
          attrs: Vec<Attribute>,
        }

        let Helper { attrs } = Helper::from_list(&nested_meta)?;
        Ok(Self { attrs, ty })
      }
      Meta::NameValue(ref value) => Self::from_expr(&value.value),
    })
    .map_err(|e| e.with_span(item))
  }
}

impl PartialFieldFromMeta {
  /// Finalizes the partial field meta and returns the attribute
  pub fn finalize(self) -> PartialFieldAttribute {
    PartialFieldAttribute {
      attrs: self.attrs,
      ty: self.ty,
    }
  }
}

#[derive(Debug, Clone)]
pub struct PartialFieldAttribute {
  attrs: Vec<Attribute>,
  ty: Option<Type>,
}

impl PartialFieldAttribute {
  /// Returns the attributes of the partial object field
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns the type of the partial object field
  pub const fn ty(&self) -> Option<&Type> {
    self.ty.as_ref()
  }
}

#[allow(clippy::large_enum_variant)]
enum FieldNestedMeta {
  Label(Label),
  Nested(darling::ast::NestedMeta),
}

impl syn::parse::Parse for FieldNestedMeta {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    if Label::peek(&input)? {
      let label: Label = input.parse()?;
      Ok(Self::Label(label))
    } else {
      darling::ast::NestedMeta::parse(input).map(Self::Nested)
    }
  }
}

/// The meta of the object field
#[derive(Debug, Clone, FromMeta)]
struct FieldFromMetaHelper {
  #[darling(default)]
  schema: SchemaFromMeta,
  #[darling(default)]
  default: Option<syn::Path>,
  #[darling(default)]
  tag: Option<NonZeroU32>,
  #[darling(default)]
  flavor: flavor::FieldFlavorFromMeta,
  #[darling(default)]
  convert: convert::ConvertFromMeta,
  #[darling(default)]
  partial: PartialFieldFromMeta,
  #[darling(default)]
  partial_decoded: PartialDecodedFieldFromMeta,
  #[darling(default)]
  selector: SelectorFieldFromMeta,
  #[darling(default)]
  copy: bool,
  #[darling(default)]
  skip: bool,
}

/// The meta of the object field
#[derive(Debug, Clone)]
pub struct FieldFromMeta {
  label: Label,
  schema: SchemaFromMeta,
  default: Option<syn::Path>,
  tag: Option<NonZeroU32>,
  flavor: flavor::FieldFlavorFromMeta,
  convert: convert::ConvertFromMeta,
  partial: PartialFieldFromMeta,
  partial_decoded: PartialDecodedFieldFromMeta,
  selector: SelectorFieldFromMeta,
  copy: bool,
  skip: bool,
}

impl FromMeta for FieldFromMeta {
  fn from_meta(item: &Meta) -> darling::Result<Self> {
    (match item {
      Meta::Path(_) => Self::from_word(),
      Meta::NameValue(value) => Self::from_expr(&value.value),
      Meta::List(value) => {
        let punctuated =
          syn::punctuated::Punctuated::<FieldNestedMeta, syn::Token![,]>::parse_terminated
            .parse2(value.tokens.clone())?;

        let mut nested_meta = Vec::new();
        let mut label: Option<Label> = None;
        for item in punctuated {
          match item {
            FieldNestedMeta::Label(l) => {
              if let Some(ref label) = label {
                return Err(darling::Error::custom(
                  format!(
                    "Cannot specify both `{label}` and `{l}` at the same time.",
                  )
                ));
              }
              label = Some(l);
            }
            FieldNestedMeta::Nested(value) => {
              nested_meta.push(value);
            }
          }
        }

        let FieldFromMetaHelper { schema, default, tag, flavor, convert, partial, partial_decoded, selector, copy, skip } = FieldFromMetaHelper::from_list(&nested_meta)?;
        Ok(Self {
          label: label.ok_or_else(|| darling::Error::custom("Expected one of [scalar, bytes, string, object, enum, union, interface, map, set, list, optional] to be specified for a field"))?,
            schema,
            default,
            tag,
            flavor,
            convert,
            partial,
            partial_decoded,
            selector,
            copy,
            skip,
        })
      }
    })
    .map_err(|e| e.with_span(item))
  }
}

impl FieldFromMeta {
  pub fn finalize(self) -> darling::Result<FieldAttribute> {
    Ok(FieldAttribute {
      default: self.default,
      schema: self.schema.into(),
      tag: self.tag,
      label: self.label,
      skip: self.skip,
      convert: self.convert.finalize()?,
      flavor: self.flavor.finalize()?,
      partial: self.partial.finalize(),
      partial_decoded: self.partial_decoded.finalize(),
      selector: self.selector.finalize(),
      copy: self.copy,
    })
  }
}

#[derive(Debug, Clone)]
pub struct FieldAttribute {
  convert: ConvertAttribute,
  default: Option<syn::Path>,
  schema: SchemaAttribute,
  tag: Option<NonZeroU32>,
  label: Label,
  flavor: Vec<FieldFlavorAttribute>,
  partial: PartialFieldAttribute,
  partial_decoded: PartialDecodedFieldAttribute,
  selector: SelectorFieldAttribute,
  copy: bool,
  skip: bool,
}

impl FieldAttribute {
  /// Returns the tag of the field
  pub const fn tag(&self) -> Option<NonZeroU32> {
    self.tag
  }

  /// Returns the flavor specified settings for the field
  pub const fn flavors(&self) -> &[FieldFlavorAttribute] {
    self.flavor.as_slice()
  }

  /// Returns the convert attribute for the field
  pub const fn convert(&self) -> &ConvertAttribute {
    &self.convert
  }

  /// Returns the information about the partial object field
  pub const fn partial(&self) -> &PartialFieldAttribute {
    &self.partial
  }

  /// Returns the information about the partial reference object field
  pub const fn partial_decoded(&self) -> &PartialDecodedFieldAttribute {
    &self.partial_decoded
  }

  /// Returns whether the field type is copyable
  pub const fn copy(&self) -> bool {
    self.copy
  }

  /// Returns whether the field should be skipped
  pub const fn skip(&self) -> bool {
    self.skip
  }

  /// Returns the default selection for the field
  pub const fn selector(&self) -> &SelectorFieldAttribute {
    &self.selector
  }

  /// Returns the type label of the field
  pub const fn label(&self) -> &Label {
    &self.label
  }

  /// Returns the schema information of the field
  pub const fn schema(&self) -> &SchemaAttribute {
    &self.schema
  }

  /// Returns the fn which will be used to generate the default value for the field
  pub const fn default(&self) -> Option<&syn::Path> {
    self.default.as_ref()
  }
}

/// The trait for the field derive input
pub trait Field: Clone {
  /// Returns the name of the field
  fn name(&self) -> &Ident;

  /// Returns the type of the field
  fn ty(&self) -> &Type;

  /// Returns the visibility of the field
  fn vis(&self) -> &syn::Visibility;

  /// Returns the attributes of the field
  fn attrs(&self) -> &[Attribute];

  /// Returns the tag of the field
  fn tag(&self) -> Option<NonZeroU32>;

  /// Returns the flavor specified settings for the field
  fn flavor(&self) -> &[FieldFlavorAttribute];

  /// Returns the convert attribute for the field
  fn convert(&self) -> &ConvertAttribute;

  /// Returns the information about the partial object field
  fn partial(&self) -> &PartialFieldAttribute;

  /// Returns the information about the partial reference object field
  fn partial_decoded(&self) -> &PartialDecodedFieldAttribute;

  /// Returns whether the field type is copyable
  fn copy(&self) -> bool;

  /// Returns whether the field should be skipped
  fn skip(&self) -> bool;

  /// Returns the default selection for the field
  fn selector(&self) -> &SelectorFieldAttribute;

  /// Returns the type label of the field
  fn label(&self) -> &Label;

  /// Returns the schema information of the field
  fn schema(&self) -> &SchemaAttribute;

  /// Returns the fn which will be used to generate the default value for the field
  fn default(&self) -> Option<&syn::Path>;

  /// Returns the field flavor attribute for the field
  fn flavors(&self) -> &[FieldFlavorAttribute];
}
