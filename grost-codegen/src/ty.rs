use grost_types::WireType;
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
  wire_ty: Option<WireType>,
  copy: bool,
}

impl Ty {
  /// Creates a new [`Ty`] with the given [`Type`]
  pub fn new(ty: Type) -> Self {
    Self {
      ty,
      wire_ty: None,
      copy: false,
    }
  }

  /// Sets the [`WireType`] of this type
  pub fn with_wire_ty(mut self, wire_ty: Option<WireType>) -> Self {
    self.wire_ty = wire_ty;
    self
  }

  /// Sets if this ty implements `Copy`
  pub fn with_copy(mut self) -> Self {
    self.copy = true;
    self
  }

  /// Gets if this ty implements `Copy`
  pub fn copy(&self) -> bool {
    self.copy
  }

  /// Returns the [`Type`] of this ty
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the [`WireType`] of this ty
  pub const fn wire_type(&self) -> Option<WireType> {
    self.wire_ty
  }
}

impl ToTokens for Ty {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    self.ty.to_tokens(tokens);
  }
}
