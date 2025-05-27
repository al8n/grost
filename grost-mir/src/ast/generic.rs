use darling::FromMeta;

use crate::ast::{grost_flavor_param, grost_lifetime, grost_unknown_buffer_param, grost_wire_format_param};

fn string_to_lifetime(s: String) -> darling::Result<syn::LifetimeParam> {
  syn::parse_str(&s).map_err(Into::into)
}

/// Generic params will be used in the generated code.
#[derive(Debug, Clone, PartialEq, Eq, darling::FromMeta)]
pub struct GenericAttribute {
  #[darling(default = grost_lifetime, and_then = "string_to_lifetime")]
  lifetime: syn::LifetimeParam,
  #[darling(default = grost_flavor_param)]
  flavor: syn::TypeParam,
  #[darling(default = grost_unknown_buffer_param)]
  unknown_buffer: syn::TypeParam,
  #[darling(default = grost_wire_format_param)]
  wire_format: syn::TypeParam,
}

impl Default for GenericAttribute {
  fn default() -> Self {
    Self {
      lifetime: grost_lifetime(),
      flavor: grost_flavor_param(),
      unknown_buffer: grost_unknown_buffer_param(),
      wire_format: grost_wire_format_param(),
    }
  }
}

impl GenericAttribute {
  /// Returns the lifetime generic param
  #[inline]
  pub const fn lifetime(&self) -> &syn::LifetimeParam {
    &self.lifetime
  }

  /// Returns the flavor generic param
  #[inline]
  pub const fn flavor(&self) -> &syn::TypeParam {
    &self.flavor
  }

  /// Returns the unknown buffer generic param
  #[inline]
  pub const fn unknown_buffer(&self) -> &syn::TypeParam {
    &self.unknown_buffer
  }

  /// Returns the wire format generic param
  #[inline]
  pub const fn wire_format(&self) -> &syn::TypeParam {
    &self.wire_format
  }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[allow(clippy::large_enum_variant)]
pub(super) enum GenericFromMeta {
  #[default]
  Deny,
  Allow(GenericAttribute),
}

impl FromMeta for GenericFromMeta {
  fn from_word() -> darling::Result<Self> {
    Ok(Self::default())
  }

  fn from_bool(value: bool) -> darling::Result<Self> {
    if value {
      Ok(Self::Allow(Default::default()))
    } else {
      Ok(Self::Deny)
    }
  }

  fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
    let parser = GenericAttribute::from_list(items)?; 
    Ok(Self::Allow(parser))
  }
}

impl GenericFromMeta {
  pub(super) fn into_option(self) -> Option<GenericAttribute> {
    match self {
      Self::Deny => None,
      Self::Allow(attr) => Some(attr),
    }
  }
}

