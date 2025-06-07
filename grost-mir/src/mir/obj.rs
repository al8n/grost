use crate::ast::object::Object as ObjectAst;

use quote::{ToTokens, format_ident, quote};
use syn::{ConstParam, Ident, Type, Visibility, parse_quote};

pub use concrete::*;
pub use generic::*;

mod concrete;
mod generic;

/// The Mid-level Intermediate Representation for objects in grost schema.
#[derive(Debug, Clone, derive_more::IsVariant, derive_more::Unwrap, derive_more::TryUnwrap)]
#[unwrap(ref)]
#[try_unwrap(ref)]
pub enum Object<M = (), F = ()> {
  /// Represents a generic object with generic flavor types.
  Generic(GenericObject<M, F>),
  /// Represents a concrete object with a specific flavor type.
  Concrete(ConcreteObject<M, F>),
}

impl<M, F> ToTokens for Object<M, F> {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    match self {
      Self::Generic(generic) => generic.to_tokens(tokens),
      Self::Concrete(concrete) => concrete.to_tokens(tokens),
    }
  }
}

impl<M, F> Object<M, F> {
  /// Converts an AST representation of an object into its MIR representation.
  pub fn from_ast(object: ObjectAst<M, F>) -> darling::Result<Self>
  where
    M: Clone,
    F: Clone,
  {
    match object {
      ObjectAst::Concrete(concrete_object) => {
        ConcreteObject::from_ast(concrete_object).map(Self::Concrete)
      }
      ObjectAst::Generic(generic_object) => {
        GenericObject::from_ast(generic_object).map(Self::Generic)
      }
    }
  }

  /// Generates the final code for the object.
  pub fn derive(&self) -> darling::Result<proc_macro2::TokenStream> {
    match self {
      Self::Generic(generic) => generic.derive(),
      Self::Concrete(concrete) => concrete.derive(),
    }
  }
}

fn grost_selected_param() -> ConstParam {
  parse_quote!(
    const __GROST_SELECTED__: ::core::primitive::bool = true
  )
}

fn accessors(
  field_name: &Ident,
  vis: &Visibility,
  ty: &Type,
  copy: bool,
) -> proc_macro2::TokenStream {
  let ref_fn = format_ident!("{}_ref", field_name);
  let ref_fn_doc = format!(" Returns a reference to the `{field_name}`");
  let ref_mut_fn = format_ident!("{}_mut", field_name);
  let ref_mut_fn_doc = format!(" Returns a mutable reference to the `{field_name}`");
  let set_fn = format_ident!("set_{}", field_name);
  let set_fn_doc = format!(" Set the `{field_name}` to the given value");
  let with_fn = format_ident!("with_{}", field_name);
  let constable = copy.then(|| quote! { const });

  quote! {
    #[doc = #ref_fn_doc]
    #[inline]
    #vis const fn #ref_fn(&self) -> &#ty {
      &self.#field_name
    }

    #[doc = #ref_mut_fn_doc]
    #[inline]
    #vis const fn #ref_mut_fn(&mut self) -> &mut #ty {
      &mut self.#field_name
    }

    #[doc = #set_fn_doc]
    #[inline]
    #vis #constable fn #set_fn(&mut self, value: #ty) -> &mut Self {
      self.#field_name = value;
      self
    }

    #[doc = #set_fn_doc]
    #[inline]
    #vis #constable fn #with_fn(mut self, value: #ty) -> Self {
      self.#field_name = value;
      self
    }
  }
}
