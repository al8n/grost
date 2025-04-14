use quote::ToTokens;
use smol_str::SmolStr;
use syn::Type;

pub use int::Int;
pub use uint::Uint;

mod int;
mod uint;

#[derive(Debug, Clone, derive_more::Display)]
#[display("{}", name)]
pub struct Feature {
  name: SmolStr,
}

impl Feature {
  pub fn new(name: impl Into<SmolStr>) -> Self {
    Self { name: name.into() }
  }

  pub fn name(&self) -> &str {
    &self.name
  }
}

#[derive(Debug, Clone)]
pub enum Dependency {
  /// Requires nothing, which means supports `no-std` and `no-alloc`
  None,
  /// Requires `alloc`
  Alloc,
  /// Requires `std`
  Std,
  /// Requries third party crate
  External(Feature),
}

#[derive(Clone)]
pub struct Ty {
  ty: Type,
  debug: bool,
  copy: bool,
  partial_eq: bool,
  partial_ord: bool,
  eq: bool,
  ord: bool,
  hash: bool,
  clone: bool,
}

impl Ty {
  pub fn new(ty: Type) -> Self {
    Self {
      ty,
      debug: false,
      copy: false,
      partial_eq: false,
      partial_ord: false,
      eq: false,
      ord: false,
      hash: false,
      clone: false
    }
  }

  /// Sets this ty implements `Copy`, `Clone`.
  pub fn with_copy(mut self) -> Self {
    self.copy = true;
    self.clone = true;
    self
  }

  pub fn with_clone(mut self, clone: bool) -> Self {
    self.clone = clone;
    self
  }

  pub fn debug(&self) -> bool {
    self.debug
  }

  pub fn copy(&self) -> bool {
    self.copy
  }

  pub fn partial_eq(&self) -> bool {
    self.partial_eq
  }

  pub fn partial_ord(&self) -> bool {
    self.partial_ord
  }

  pub fn eq(&self) -> bool {
    self.eq
  }

  pub fn ord(&self) -> bool {
    self.ord
  }

  pub fn hash(&self) -> bool {
    self.hash
  }

  pub fn clonable(&self) -> bool {
    self.clone
  }

  pub fn ty(&self) -> &Type {
    &self.ty
  }
}

impl ToTokens for Ty {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    self.ty.to_tokens(tokens);
  }
}
