use darling::FromMeta;

use crate::ast::{
  grost_flavor_param, grost_lifetime, grost_unknown_buffer_param, grost_wire_format_param,
};

fn string_to_lifetime(s: String) -> darling::Result<syn::LifetimeParam> {
  syn::parse_str(&s).map_err(Into::into)
}

fn default_grost_flavor_param() -> Option<syn::TypeParam> {
  Some(grost_flavor_param())
}

struct GenericFlavorParam(Option<syn::TypeParam>);

impl Default for GenericFlavorParam {
  fn default() -> Self {
    Self(None)
  }
}

impl FromMeta for GenericFlavorParam {
  fn from_word() -> darling::Result<Self> {
    Ok(Self(Some(grost_flavor_param())))
  }

  fn from_bool(value: bool) -> darling::Result<Self> {
    if value {
      Ok(Self(Some(grost_flavor_param())))
    } else {
      Ok(Self(None))
    }
  }

  fn from_string(value: &str) -> darling::Result<Self> {
    let param = syn::parse_str(value).map_err(darling::Error::custom)?;
    Ok(Self(Some(param)))
  }
}

impl From<GenericFlavorParam> for Option<syn::TypeParam> {
  fn from(value: GenericFlavorParam) -> Self {
    value.0
  }
}

#[derive(Debug, Clone, PartialEq, Eq, darling::FromMeta)]
pub(crate) struct GenericAttribute {
  #[darling(default = grost_lifetime, and_then = "string_to_lifetime")]
  lifetime: syn::LifetimeParam,
  #[darling(default, map = "GenericFlavorParam::into")]
  flavor: Option<syn::TypeParam>,
  #[darling(default = grost_unknown_buffer_param)]
  unknown_buffer: syn::TypeParam,
  #[darling(default = grost_wire_format_param)]
  wire_format: syn::TypeParam,
}

impl Default for GenericAttribute {
  fn default() -> Self {
    Self {
      lifetime: grost_lifetime(),
      flavor: None,
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
  pub const fn flavor(&self) -> Option<&syn::TypeParam> {
    self.flavor.as_ref()
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
