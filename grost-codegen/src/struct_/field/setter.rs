use either::Either;
use quote::{ToTokens, format_ident, quote};
use smol_str::{SmolStr, format_smolstr};
use syn::{Type, Visibility, parse_quote};

use crate::SafeIdent;

/// A converter function of the setter
#[derive(Clone)]
pub struct Converter {
  function: syn::Path,
  ty: Type,
}

impl Converter {
  /// Creates a new `Converter`
  pub const fn new(function: syn::Path, ty: Type) -> Self {
    Self { function, ty }
  }

  /// Returns the path of the converter function
  pub fn function(&self) -> &syn::Path {
    &self.function
  }

  /// Returns the type of the converter function
  pub fn ty(&self) -> &Type {
    &self.ty
  }
}

/// Setter for a field
pub struct Setter {
  fn_name: Option<SafeIdent>,
  field_name: SafeIdent,
  description: Option<SmolStr>,
  data: Either<Type, Converter>,
  visibility: Option<Visibility>,
  attributes: Vec<syn::Attribute>,
  const_fn: bool,
  take: bool,
}

impl Setter {
  /// Creates a new `Setter`
  /// with the given field name and type
  ///
  /// This will yield a setter function like `fn field_name(&self) -> &Type { &self.field_name }`
  pub fn new(field_name: SafeIdent, ty: Type) -> Self {
    Self {
      fn_name: None,
      field_name,
      description: None,
      data: Either::Left(ty),
      visibility: Some(parse_quote!(pub)),
      attributes: Vec::new(),
      const_fn: false,
      take: false,
    }
  }

  /// Creates a new `Setter`
  /// with the given field name and converter function
  ///
  /// This will yield a setter function like `fn field_name(&self) -> &Type { converter(&self.field_name) }`
  pub fn new_with_converter(field_name: SafeIdent, converter: Converter) -> Self {
    Self {
      fn_name: None,
      field_name,
      description: None,
      data: Either::Right(converter),
      visibility: Some(parse_quote!(pub)),
      const_fn: false,
      attributes: Vec::new(),
      take: false,
    }
  }

  /// Sets the if the setter takes ownership
  pub fn with_take(mut self, take: bool) -> Self {
    self.take = take;
    self
  }

  /// Sets the description of the setter function
  pub fn with_description(mut self, description: impl Into<SmolStr>) -> Self {
    self.description = Some(description.into());
    self
  }

  /// Sets the attributes of the setter function
  pub fn with_attributes(mut self, attributes: Vec<syn::Attribute>) -> Self {
    self.attributes = attributes;
    self
  }

  /// Sets the fn_name of the setter function
  ///
  /// This will yield a setter function like
  /// - `fn fn_name(&self) -> &Type { &self.field_name }`
  /// - `fn fn_name(&self) -> &Type { converter(&self.field_name) }`
  pub fn with_fn_name(mut self, fn_name: Option<SafeIdent>) -> Self {
    self.fn_name = fn_name;
    self
  }

  /// Sets the visibility of the setter function
  pub fn with_visibility(mut self, visibility: Visibility) -> Self {
    self.visibility = Some(visibility);
    self
  }

  /// Sets the const fn of the setter function
  pub fn with_const_fn(mut self, const_fn: bool) -> Self {
    self.const_fn = const_fn;
    self
  }

  /// Returns the attributes of the setter function
  pub fn attributes(&self) -> &[syn::Attribute] {
    &self.attributes
  }

  /// Returns if the setter function takes ownership
  pub fn take(&self) -> bool {
    self.take
  }

  /// Returns the const fn of the setter function
  pub fn const_fn(&self) -> bool {
    self.const_fn
  }

  /// Returns the name of the setter function
  pub fn fn_name(&self) -> Option<&SafeIdent> {
    self.fn_name.as_ref()
  }

  /// Returns the field name of the setter function
  pub fn field_name(&self) -> &SafeIdent {
    &self.field_name
  }

  /// Returns the input type of the setter function
  pub fn input(&self) -> &Type {
    match &self.data {
      Either::Left(ty) => ty,
      Either::Right(converter) => converter.ty(),
    }
  }

  /// Returns the description of the setter function
  pub fn description(&self) -> Option<&str> {
    self.description.as_deref()
  }

  /// Returns the visibility of the setter function
  pub fn visibility(&self) -> Option<&Visibility> {
    self.visibility.as_ref()
  }
}

impl ToTokens for Setter {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let vis = self.visibility();
    let fn_name = if self.take {
      match &self.fn_name {
        Some(name) => quote! { #name },
        None => {
          let name = format_ident!("with_{}", self.field_name().name_str());
          quote! { #name }
        }
      }
    } else {
      match &self.fn_name {
        Some(name) => quote! { #name },
        None => {
          let name = format_ident!("set_{}", self.field_name().name_str());
          quote! { #name }
        }
      }
    };
    let input = self.input();
    let field_name = self.field_name();
    let description = self.description().map(|s| {
      let s = format_smolstr!(" {}", s.trim());
      let s = s.as_str();
      quote! {#[doc = #s]}
    });
    let const_fn = self.const_fn().then_some(quote! { const });
    let take = (!self.take()).then_some(quote! { & });
    let attrs = &self.attributes;
    let body = match &self.data {
      Either::Left(_) => {
        quote! {
          self.#field_name = #field_name;
          self
        }
      }
      Either::Right(converter) => {
        let function = &converter.function;
        quote! {
          self.#field_name = #function(#field_name);
          self
        }
      }
    };

    tokens.extend(quote! {
      #description
      #(#attrs)*
      #[inline]
      #vis #const_fn fn #fn_name(#take mut self, #field_name: #input) -> #take Self {
        #body
      }
    });
  }
}
