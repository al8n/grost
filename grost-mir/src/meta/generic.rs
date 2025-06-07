use darling::FromMeta;

use syn::{LifetimeParam, TypeParam};

use crate::meta::{
  grost_flavor_param, grost_lifetime, grost_unknown_buffer_param, grost_wire_format_param,
};

fn string_to_lifetime(s: String) -> darling::Result<LifetimeParam> {
  syn::parse_str(&s).map_err(Into::into)
}

#[derive(Debug, Default, Clone)]
struct GenericFlavorParam(Option<TypeParam>);

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

impl From<GenericFlavorParam> for Option<TypeParam> {
  fn from(value: GenericFlavorParam) -> Self {
    value.0
  }
}

#[derive(Debug, Clone, PartialEq, Eq, darling::FromMeta)]
pub(crate) struct GenericFromMeta {
  #[darling(default = grost_lifetime, and_then = "string_to_lifetime")]
  lifetime: LifetimeParam,
  #[darling(default, map = "GenericFlavorParam::into")]
  flavor: Option<TypeParam>,
  #[darling(default = grost_unknown_buffer_param)]
  unknown_buffer: TypeParam,
  #[darling(default = grost_wire_format_param)]
  wire_format: TypeParam,
}

impl Default for GenericFromMeta {
  fn default() -> Self {
    Self {
      lifetime: grost_lifetime(),
      flavor: None,
      unknown_buffer: grost_unknown_buffer_param(),
      wire_format: grost_wire_format_param(),
    }
  }
}

impl GenericFromMeta {
  /// Returns the lifetime generic param
  #[inline]
  pub const fn lifetime(&self) -> &LifetimeParam {
    &self.lifetime
  }

  /// Returns the flavor generic param
  #[inline]
  pub const fn flavor(&self) -> Option<&TypeParam> {
    self.flavor.as_ref()
  }

  /// Returns the unknown buffer generic param
  #[inline]
  pub const fn unknown_buffer(&self) -> &TypeParam {
    &self.unknown_buffer
  }

  /// Returns the wire format generic param
  #[inline]
  pub const fn wire_format(&self) -> &TypeParam {
    &self.wire_format
  }
}
