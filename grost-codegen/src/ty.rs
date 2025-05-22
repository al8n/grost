use quote::{ToTokens, format_ident};
use smol_str::{SmolStr, format_smolstr};
use syn::{Lifetime, Type, parse_quote};

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

#[derive(Clone, derive_more::IsVariant)]
pub(crate) enum TyRepr {
  Primitive(Type),
  List {
    ty: Type,
    item: Box<Ty>,
  },
  Map {
    ty: Type,
    key: Box<Ty>,
    value: Box<Ty>,
  },
  Optional(Box<Ty>),
  Enum(Type),
  Object(Type),
  Union(Type),
  Interface(Type),
}

impl TyRepr {
  pub fn ty(&self) -> Type {
    match self {
      Self::Primitive(ty) => ty.clone(),
      Self::List { ty, .. } => ty.clone(),
      Self::Map { ty, .. } => ty.clone(),
      Self::Enum(ty) => ty.clone(),
      Self::Object(ty) => ty.clone(),
      Self::Union(ty) => ty.clone(),
      Self::Interface(ty) => ty.clone(),
      Self::Optional(ty) => {
        let ty = ty.ty();
        parse_quote!(::core::option::Option<#ty>)
      }
    }
  }

  pub fn ref_ty(&self, mutable: bool, lifetime: Option<Lifetime>) -> Type {
    let mutability = mutable.then_some(format_ident!("mut"));
    match self {
      Self::Primitive(ty) => parse_quote!(& #lifetime #mutability #ty),
      Self::List { ty, .. } => parse_quote!(& #lifetime #mutability #ty),
      Self::Map { ty, .. } => parse_quote!(& #lifetime #mutability #ty),
      Self::Enum(ty) => parse_quote!(& #lifetime #mutability #ty),
      Self::Object(ty) => parse_quote!(& #lifetime #mutability #ty),
      Self::Union(ty) => parse_quote!(& #lifetime #mutability #ty),
      Self::Interface(ty) => parse_quote!(& #lifetime #mutability #ty),
      Self::Optional(ty) => {
        let ty = ty.ty();
        parse_quote!(::core::option::Option<& #lifetime #mutability #ty>)
      }
    }
  }

  /// Returns the type will be used in the partial struct
  pub fn partial_ty(&self) -> Type {
    match self {
      Self::Optional(ty) => ty.repr().partial_ty(),
      _ => self.ty().clone(),
    }
  }

  pub fn encode_atomic_ty(&self) -> Type {
    match self {
      Self::Primitive(ty) => ty.clone(),
      Self::List { item, .. } => item.repr().encode_atomic_ty(),
      Self::Map { key, value, .. } => {
        let key = key.repr().encode_atomic_ty();
        let value = value.repr().encode_atomic_ty();
        parse_quote!((&#key, &#value))
      }
      Self::Enum(ty) | Self::Object(ty) | Self::Union(ty) | Self::Interface(ty) => ty.clone(),
      Self::Optional(ty) => ty.repr().encode_atomic_ty(),
    }
  }
}

#[derive(Clone)]
pub struct Ty {
  repr: TyRepr,
  schema_type: SmolStr,
  description: Option<SmolStr>,
  copy: bool,
}

impl Ty {
  /// Creates a new primitive [`Ty`].
  ///
  /// - The `name` is the schema name of the type, which is used in the Graph protocol buffer schema file.
  /// - The `ty` is the Rust type of the type, which is used in the Rust code.
  pub fn primitive(ty: Type, schema_type: &str) -> Self {
    Self {
      repr: TyRepr::Primitive(ty),
      schema_type: required_schema_type(schema_type),
      copy: false,
      description: None,
    }
  }

  /// Creates a new list [`Ty`].
  pub fn list(ty: Type, item: Ty, schema_type: &str) -> Self {
    Self {
      repr: TyRepr::List {
        ty,
        item: Box::new(item),
      },
      schema_type: required_schema_type(schema_type),
      copy: false,
      description: None,
    }
  }

  /// Creates a new map [`Ty`].
  pub fn map(ty: Type, key: Ty, value: Ty, schema_type: &str) -> Self {
    Self {
      repr: TyRepr::Map {
        ty,
        key: Box::new(key),
        value: Box::new(value),
      },
      schema_type: required_schema_type(schema_type),
      copy: false,
      description: None,
    }
  }

  /// Creates a new unit enum [`Ty`].
  pub fn enum_(ty: Type, schema_type: &str) -> Self {
    Self {
      repr: TyRepr::Enum(ty),
      schema_type: required_schema_type(schema_type),
      copy: false,
      description: None,
    }
  }

  /// Creates a new struct [`Ty`].
  pub fn struct_(ty: Type, schema_type: &str) -> Self {
    Self {
      repr: TyRepr::Object(ty),
      schema_type: required_schema_type(schema_type),
      copy: false,
      description: None,
    }
  }

  /// Creates a new union [`Ty`].
  pub fn union(ty: Type, schema_type: &str) -> Self {
    Self {
      repr: TyRepr::Union(ty),
      schema_type: required_schema_type(schema_type),
      copy: false,
      description: None,
    }
  }

  /// Creates a new interface [`Ty`].
  pub fn interface(ty: Type, schema_type: &str) -> Self {
    Self {
      repr: TyRepr::Interface(ty),
      schema_type: required_schema_type(schema_type),
      copy: false,
      description: None,
    }
  }

  /// Creates a new optional [`Ty`].
  pub fn optional(mut typ: Self) -> Self {
    match typ.repr {
      // nested optional type is not allowed
      TyRepr::Optional(ty) => *ty,
      _ => {
        typ.schema_type = typ.schema_type.trim_end_matches("!").into();
        Self {
          schema_type: typ.schema_type.trim_end_matches("!").into(),
          copy: typ.copy,
          description: typ.description.clone(),
          repr: TyRepr::Optional(Box::new(typ)),
        }
      }
    }
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
  pub fn ty(&self) -> Type {
    self.repr.ty()
  }

  /// Returns the schema type of this ty
  pub fn schema_type(&self) -> &str {
    &self.schema_type
  }

  /// Returns the description of this ty
  pub fn description(&self) -> Option<&str> {
    self.description.as_deref()
  }

  pub(crate) fn repr(&self) -> &TyRepr {
    &self.repr
  }

  // // TODO(al8n): remove this fn, because we can do this by Reflectable trait
  // pub(crate) fn to_type_reflection<F>(
  //   &self,
  //   path_to_grost: &syn::Path,
  //   flavor: &F,
  // ) -> proc_macro2::TokenStream
  // where
  //   F: super::FlavorGenerator + ?Sized,
  // {
  //   let flavor_ty = flavor.ty();
  //   match &self.repr {
  //     TyRepr::Primitive(ty) => {
  //       let description = self.description().unwrap_or_default();
  //       let name = self.schema_type();

  //       quote! {
  //         #path_to_grost::__private::reflection::Type::<#flavor_ty>::Primitive {
  //           name: #name,
  //           description: #description,
  //         }
  //       }
  //     }
  //     TyRepr::List { ty, item } => {
  //       let inner = item.to_type_reflection(path_to_grost, flavor);
  //       quote! {
  //         #path_to_grost::__private::reflection::Type::<#flavor_ty>::List(&#inner)
  //       }
  //     }
  //     TyRepr::Map { ty, key, value } => {
  //       let inner_key = key.to_type_reflection(path_to_grost, flavor);
  //       let inner_value = value.to_type_reflection(path_to_grost, flavor);
  //       quote! {
  //         #path_to_grost::__private::reflection::Type::<#flavor_ty>::Map {
  //           key: &#inner_key,
  //           value: &#inner_value,
  //         }
  //       }
  //     }
  //     TyRepr::Enum(ty) => {
  //       quote! {
  //         #path_to_grost::__private::reflection::Type::<#flavor_ty>::UintEnum(<#ty>::REFLECTION)
  //       }
  //     }
  //     TyRepr::Object(ty) => {
  //       quote! {
  //         #path_to_grost::__private::reflection::Type::<#flavor_ty>::Object(<
  //           #ty as #path_to_grost::__private::reflection::Reflectable<
  //             #flavor_ty,
  //           >
  //         >::REFLECTION)
  //       }
  //     }
  //     TyRepr::Union(_) => todo!(),
  //     TyRepr::Interface(_) => todo!(),
  //     TyRepr::Optional(ty) => {
  //       let inner = ty.to_type_reflection(path_to_grost, flavor);
  //       quote! {
  //         #path_to_grost::__private::reflection::Type::<#flavor_ty>::Optional(&#inner)
  //       }
  //     }
  //   }
  // }
}

impl ToTokens for Ty {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    self.ty().to_tokens(tokens);
  }
}

fn required_schema_type(schema_type: &str) -> SmolStr {
  if schema_type.ends_with('!') {
    schema_type.into()
  } else {
    format_smolstr!("{}!", schema_type)
  }
}
