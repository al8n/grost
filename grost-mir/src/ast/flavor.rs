use darling::FromMeta;
pub use encode::*;
pub use identifier::*;
use registration::*;
use syn::{Ident, spanned::Spanned};

mod encode;
mod identifier;
mod registration;

#[cfg(feature = "serde")]
mod serde;

fn default_flavor_attribute() -> bool {
  true
}

#[derive(Debug, darling::FromMeta)]
struct FlavorParser {
  #[darling(default = "default_flavor_attribute")]
  default: bool,
  #[darling(default)]
  generic: bool,
  #[darling(default)]
  register: RegistrationAttribute,
}

#[derive(Debug, Clone)]
pub struct Flavor {
  name: Ident,
  ty: syn::Type,
  or_else_default: bool,
  identifier: IdentifierAttribute,
  encode: EncodeAttribute,
}

impl Flavor {
  /// Returns the name of the flavor.
  #[inline]
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  /// Returns the type of the flavor.
  #[inline]
  pub const fn ty(&self) -> &syn::Type {
    &self.ty
  }

  /// Returns `true` if this flavor should be used as a default.
  #[inline]
  pub const fn or_else_default(&self) -> bool {
    self.or_else_default
  }

  /// Returns the identifier attribute for this flavor.
  #[inline]
  pub const fn identifier(&self) -> &IdentifierAttribute {
    &self.identifier
  }

  /// Returns the encode attribute for this flavor.
  #[inline]
  pub const fn encode(&self) -> &EncodeAttribute {
    &self.encode
  }
}

#[derive(Debug, Clone)]
pub struct FlavorAttribute {
  default: bool,
  generic: bool,
  flavors: Vec<Flavor>,
}

impl Default for FlavorAttribute {
  fn default() -> Self {
    Self {
      default: default_flavor_attribute(),
      generic: false,
      flavors: Vec::new(),
    }
  }
}

impl FlavorAttribute {
  /// Returns `true` if need to generate the default flavor.
  #[inline]
  pub const fn default(&self) -> bool {
    self.default
  }

  /// Returns `true` if need to generate generic types over flavor.
  #[inline]
  pub const fn generic(&self) -> bool {
    self.generic
  }

  /// Returns the flavors defined in this attribute.
  #[inline]
  pub const fn flavors(&self) -> &[Flavor] {
    self.flavors.as_slice()
  }
}

impl FromMeta for FlavorAttribute {
  fn from_meta(item: &syn::Meta) -> darling::Result<Self> {
    let parser = FlavorParser::from_meta(item)?;
    let mut flavors = Vec::new();

    for (name, value) in parser.register.flavors {
      flavors.push(Flavor {
        name,
        ty: value.ty,
        or_else_default: value.or_else_default,
        identifier: value.identifier,
        encode: value.encode,
      });
    }

    let this = Self {
      default: parser.default,
      generic: parser.generic,
      flavors,
    };

    if this.flavors.is_empty() && !this.default {
      return Err(
        darling::Error::custom(
          "at least one flavor must be registered or the default flavor must be enabled.",
        )
        .with_span(&item.span()),
      );
    }

    Ok(this)
  }
}
