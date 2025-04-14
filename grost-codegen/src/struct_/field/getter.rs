use either::Either;
use quote::{format_ident, quote, ToTokens};
use smol_str::{format_smolstr, SmolStr};
use syn::{parse_quote, Type, Visibility};

use crate::SafeIdent;

/// A converter function
#[derive(Clone)]
pub struct Converter {
  /// The converter function
  function: syn::Path,
  /// The output type of the converter
  ty: Type,
}

impl Converter {
  /// Creates a new `Converter`
  /// with the given function and type
  pub fn new(function: syn::Path, ty: Type) -> Self {
    Self { function, ty }
  }

  /// Returns the path of the converter function
  pub fn function(&self) -> &syn::Path {
    &self.function
  }

  /// Returns the output type of the converter function
  pub fn ty(&self) -> &Type {
    &self.ty
  }
}

/// Getter fn for a field
pub struct Getter {
  fn_name: Option<SafeIdent>,
  field_name: SafeIdent,
  description: Option<SmolStr>,
  data: Either<Type, Converter>,
  visibility: Option<Visibility>,
  attributes: Vec<syn::Attribute>,
  const_fn: bool,
  mutable: bool,
  copy: bool,
}

impl Getter {
  /// Creates a new `Getter`
  /// with the given field name and type
  ///
  /// This will yield a getter function like `fn field_name(&self) -> &Type { &self.field_name }`
  pub fn new(field_name: SafeIdent, ty: Type) -> Self {
    Self {
      fn_name: None,
      field_name,
      description: None,
      data: Either::Left(ty),
      visibility: Some(parse_quote!(pub)),
      attributes: Vec::new(),
      const_fn: false,
      mutable: false,
      copy: false,
    }
  }

  /// Creates a new `Getter`
  /// with the given field name and converter function
  ///
  /// This will yield a getter function like `fn field_name(&self) -> &Type { converter(&self.field_name) }`
  pub fn new_with_converter(field_name: SafeIdent, converter: Converter) -> Self {
    Self {
      fn_name: None,
      field_name,
      description: None,
      data: Either::Right(converter),
      visibility: Some(parse_quote!(pub)),
      const_fn: false,
      mutable: false,
      copy: false,
      attributes: Vec::new(),
    }
  }

  /// Sets the copy of the getter function
  /// 
  /// If this is set to `true`, the getter function will return a copy of the field
  /// instead of a reference.
  pub fn with_copy(mut self, copy: bool) -> Self {
    self.copy = copy;
    self
  }

  /// Sets the mutability of the getter function
  pub fn with_mutable(mut self, mutable: bool) -> Self {
    self.mutable = mutable;
    self
  }

  /// Sets the description of the getter function
  pub fn with_description(mut self, description: impl Into<SmolStr>) -> Self {
    self.description = Some(description.into());
    self
  }

  /// Sets the attributes of the getter function
  pub fn with_attributes(mut self, attributes: Vec<syn::Attribute>) -> Self {
    self.attributes = attributes;
    self
  }

  /// Sets the fn_name of the getter function
  ///
  /// This will yield a getter function like
  /// - `fn fn_name(&self) -> &Type { &self.field_name }`
  /// - `fn fn_name(&self) -> &Type { converter(&self.field_name) }`
  pub fn with_fn_name(mut self, fn_name: Option<SafeIdent>) -> Self {
    self.fn_name = fn_name;
    self
  }

  /// Sets the visibility of the getter function
  pub fn with_visibility(mut self, visibility: Visibility) -> Self {
    self.visibility = Some(visibility);
    self
  }

  /// Sets the const fn of the getter function
  pub fn with_const_fn(mut self, const_fn: bool) -> Self {
    self.const_fn = const_fn;
    self
  }
  
  /// Returns the attributes of the getter function
  pub fn attributes(&self) -> &[syn::Attribute] {
    &self.attributes
  }

  /// Returns the copy of the getter function
  pub fn copy(&self) -> bool {
    self.copy
  }

  /// Returns the mutability of the getter function
  pub fn mutable(&self) -> bool {
    self.mutable
  }

  /// Returns the const fn of the getter function
  pub fn const_fn(&self) -> bool {
    self.const_fn
  }

  /// Returns the name of the getter function
  pub fn fn_name(&self) -> &SafeIdent {
    self.fn_name.as_ref().unwrap_or(&self.field_name)
  }

  /// Returns the field name of the getter function
  pub fn field_name(&self) -> &SafeIdent {
    &self.field_name
  }

  /// Returns the output type of the getter function
  pub fn output(&self) -> Type {
    match &self.data {
      Either::Left(ty) => if self.mutable {
        parse_quote! { &mut #ty }
      } else if self.copy {
        ty.clone()
      } else {
        parse_quote! { &#ty }
      },
      Either::Right(converter) => converter.ty().clone(),
    }
  }

  /// Returns the description of the getter function
  pub fn description(&self) -> Option<&str> {
    self.description.as_deref()
  }

  /// Returns the visibility of the getter function
  pub fn visibility(&self) -> Option<&Visibility> {
    self.visibility.as_ref()
  }
}

impl ToTokens for Getter {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let vis = self.visibility();
    let fn_name = if self.mutable {
      match &self.fn_name {
        Some(name) => quote! { #name },
        None => {
          let name = format_ident!("{}_mut", self.field_name().name_str());
          quote! { #name }
        }
      }
    } else {
      let name = self.fn_name();
      quote! { #name }
    };

    let output = self.output();
    let field_name = self.field_name();
    let description = self.description().map(|s| {
      let s = format_smolstr!(" {}", s.trim());
      let s = s.as_str();
      quote! {#[doc = #s]}
    });
    let const_fn = self.const_fn().then_some(quote!{const});
    let mutable = self.mutable().then_some(quote! { mut });
    let attrs = &self.attributes;
    let body = match &self.data {
      Either::Left(_) => {
        if self.copy && !self.mutable {
          quote! { self.#field_name }
        } else {
          quote! {
            & #mutable self.#field_name
          }
        }
      }
      Either::Right(converter) => {
        let function = &converter.function;
        quote! {
          #function(& #mutable self.#field_name)
        }
      }
    };

    tokens.extend(quote! {
      #description
      #(#attrs)*
      #[inline]
      #vis #const_fn fn #fn_name(& #mutable self) -> #output {
        #body
      }
    });
  }
}
