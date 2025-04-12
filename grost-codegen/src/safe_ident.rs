use quote::{format_ident, ToTokens};
use smol_str::SmolStr;
use syn::Ident;

/// An [`Ident`] which avoids Rust's keywords and reserved keywords
#[derive(Clone, Debug)]
pub struct SafeIdent {
  original: Ident,
  original_string: SmolStr,
  safe: Ident,
  safe_string: SmolStr,
}

impl PartialEq for SafeIdent {
  fn eq(&self, other: &Self) -> bool {
    self.safe == other.safe
  }
}

impl Eq for SafeIdent {}

impl core::hash::Hash for SafeIdent {
  fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
    self.safe.hash(state);
  }
}

impl SafeIdent {
  /// Creates a new `SafeIdent` from the given `name`.
  pub fn new(name: &str) -> Self {
    let safe = format_ident!("{name}");

    // TODO: add a check for reserved keywords
    Self {
      original: safe.clone(),
      original_string: name.into(),
      safe,
      safe_string: name.into(),
    }
  }

  /// Returns the name
  #[inline]
  pub fn name(&self) -> &Ident {
    &self.safe
  }

  /// Returns the name str
  #[inline]
  pub fn name_str(&self) -> &str {
    &self.safe_string
  }

  /// Returns the original `Ident`.
  #[inline]
  pub fn original(&self) -> &Ident {
    &self.original
  }

  /// Returns the original name str
  #[inline]
  pub fn original_str(&self) -> &str {
    &self.original_string
  }
}

impl ToTokens for SafeIdent {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    self.safe.to_tokens(tokens);
  }
}
