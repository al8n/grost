use quote::ToTokens;
use smol_str::SmolStr;
use syn::Ident;

/// An [`Ident`] which avoids Rust's keywords and reserved keywords
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SafeIdent {
  original: Ident,
  original_string: SmolStr,
  safe: Ident,
  safe_string: SmolStr,
}

impl SafeIdent {
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
