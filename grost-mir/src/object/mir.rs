use quote::{ToTokens, format_ident, quote};
use syn::{ConstParam, Ident, Type, Visibility, parse_quote};

use super::{RawField, RawObject, ast::Object as ObjectAst};

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
  Generic(Box<GenericObject<M, F>>),
  /// Represents a concrete object with a specific flavor type.
  Concrete(Box<ConcreteObject<M, F>>),
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
  /// Creates a MIR of object from a raw object representation.
  pub fn from_raw<O>(object: O) -> darling::Result<Self>
  where
    O: RawObject<Meta = M>,
    O::Field: RawField<Meta = F>,
    M: Clone,
    F: Clone,
  {
    let object = ObjectAst::from_raw(&object)?;
    match object {
      ObjectAst::Concrete(concrete_object) => {
        ConcreteObject::from_ast(*concrete_object).map(|object| Self::Concrete(Box::new(object)))
      }
      ObjectAst::Generic(generic_object) => {
        GenericObject::from_ast(*generic_object).map(|object| Self::Generic(Box::new(object)))
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

  /// Returns the path to the `grost` crate.
  #[inline]
  pub const fn path_to_grost(&self) -> &syn::Path {
    match self {
      Self::Generic(generic) => generic.path_to_grost(),
      Self::Concrete(concrete) => concrete.path_to_grost(),
    }
  }

  /// Returns the name of the object.
  #[inline]
  pub const fn name(&self) -> &Ident {
    match self {
      Self::Generic(generic) => generic.name(),
      Self::Concrete(concrete) => concrete.name(),
    }
  }

  /// Returns the visibility of the object.
  #[inline]
  pub const fn vis(&self) -> &Visibility {
    match self {
      Self::Generic(generic) => generic.vis(),
      Self::Concrete(concrete) => concrete.vis(),
    }
  }

  /// Returns the attributes of the object.
  #[inline]
  pub const fn attrs(&self) -> &[syn::Attribute] {
    match self {
      Self::Generic(generic) => generic.attrs(),
      Self::Concrete(concrete) => concrete.attrs(),
    }
  }

  /// Returns the generics of the object.
  #[inline]
  pub const fn generics(&self) -> &syn::Generics {
    match self {
      Self::Generic(generic) => generic.generics(),
      Self::Concrete(concrete) => concrete.generics(),
    }
  }

  /// Returns the custom metadata of the object.
  #[inline]
  pub const fn meta(&self) -> &M {
    match self {
      Self::Generic(generic) => generic.meta(),
      Self::Concrete(concrete) => concrete.meta(),
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
