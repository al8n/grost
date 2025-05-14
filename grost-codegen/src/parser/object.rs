use std::num::NonZeroU32;

use darling::{FromDeriveInput, FromMeta, ast::Data, util::Ignored};
use quote::format_ident;
use syn::{Attribute, DeriveInput, Ident, Type, Visibility, parse::Parser};

use crate::{grost_flavor_generic, grost_lifetime};

use super::*;

pub use field::ObjectFieldDeriveInput;
mod field;

#[derive(grost_darling::Object)]
#[grost(attributes(grost), field = "ObjectFieldDeriveInput")]
pub struct Object {
  #[grost(darling(rename = "crate", default = "super::default_path"))]
  path_to_crate: syn::Path,
}

// #[derive(Debug, FromDeriveInput)]
// #[darling(attributes(grost), supports(struct_named), forward_attrs)]
// pub struct ObjectDeriveInput {
//   ident: Ident,
//   vis: Visibility,
//   generics: syn::Generics,
//   attrs: Vec<Attribute>,
//   #[darling(rename = "crate", default = "default_grost_path")]
//   path_to_grost: syn::Path,
//   #[darling(default)]
//   default: Option<syn::Path>,
//   #[darling(default)]
//   schema: SchemaMeta,
//   #[darling(default)]
//   partial: PartialObjectMeta,
//   #[darling(default)]
//   partial_ref: PartialRefObjectMeta,
//   data: Data<Ignored, ObjectFieldDeriveInput>,
// }

// #[derive(Default, Debug, Clone, FromMeta)]
// pub struct SchemaMeta {
//   #[darling(default)]
//   name: Option<Ident>,
//   #[darling(default)]
//   description: Option<String>,
// }

// #[derive(Debug, Default, Clone, FromMeta)]
// pub struct PartialObjectMeta {
//   #[darling(default, rename = "rename")]
//   name: Option<Ident>,
//   #[darling(default, map = "Attributes::into")]
//   attrs: Attributes,
// }

// #[derive(Debug, Default, FromMeta)]
// pub struct PartialRefObjectMeta {
//   #[darling(default, rename = "rename")]
//   name: Option<Ident>,
//   #[darling(default, map = "Attributes::into")]
//   attrs: Attributes,
//   #[darling(default)]
//   copy: bool,
// }

// pub struct Selector {}

// pub struct PartialRefField {
//   name: Ident,
//   ty: syn::Type,
//   vis: Visibility,
//   tag: NonZeroU32,
//   wire: syn::Type,
//   attrs: Vec<Attribute>,
//   copy: bool,
// }

// pub struct PartialObject {
//   name: Ident,
//   vis: Visibility,
//   generics: syn::Generics,
//   fields: Vec<PartialField>,
//   attrs: Vec<Attribute>,
//   copy: bool,
// }

// impl PartialObject {
//   fn derive_defination(&self, _path_to_grost: &syn::Path) -> proc_macro2::TokenStream {
//     let name = &self.name;
//     let visibility = &self.vis;
//     let fields = self.fields.iter().map(|f| {
//       let field_name = &f.name;
//       let ty = &f.ty;
//       quote! {
//         #field_name: #ty,
//       }
//     });
//     let attrs = &self.attrs;
//     let (_, ty_generics, where_clause) = self.generics.split_for_impl();

//     quote! {
//       #(#attrs)*
//       #[allow(non_camel_case_types, clippy::type_complexity)]
//       #visibility struct #name #ty_generics #where_clause {
//         #(#fields)*
//       }
//     }
//   }
// }

// pub struct PartialRefObject {
//   name: Ident,
//   vis: Visibility,
//   generics: syn::Generics,
//   fields: Vec<PartialRefField>,
//   attrs: Vec<Attribute>,
//   copy: bool,
// }

// impl TryFrom<&ObjectDeriveInput> for PartialRefObject {
//   type Error = darling::Error;

//   fn try_from(input: &ObjectDeriveInput) -> darling::Result<Self> {

//   }
// }

// impl PartialRefObject {
//   fn derive_defination(&self, path_to_grost: &syn::Path) -> proc_macro2::TokenStream {
//     // let field_reflection = self.field_reflection_name();
//     let name = &self.name;
//     let vis = &self.vis;

//     let fields = self.fields.iter().map(|f| {
//       let ty = &f.ty;
//       let name = &f.name;
//       quote! {
//         #name: #ty,
//       }
//     });

//     let (_, ty_generics, where_clauses) = self.generics.split_for_impl();
//     quote! {
//       #[allow(clippy::type_complexity, non_camel_case_types)]
//       #vis struct #name #ty_generics #where_clauses
//       {
//         #(#fields)*
//         // __grost_unknown_buffer__: ::core::option::Option<#ubg>,
//       }
//     }
//   }
// }

// pub struct Object {
//   name: Ident,
//   path_to_grost: syn::Path,
//   schema: SchemaMeta,
//   vis: Visibility,
//   generics: syn::Generics,
//   fields: Vec<Field>,
//   partial: PartialObject,
//   partial_ref: PartialRefObject,
//   // selector: Selector,
// }

// impl Object {
//   pub fn from_input(input: ObjectDeriveInput) -> darling::Result<Self> {
//     let partial_object = PartialObject::try_from(&input)?;
//     let partial_ref_object = PartialRefObject::try_from(&input)?;
//     Ok(Self {
//       name: input.ident,
//       path_to_grost: input.path_to_grost,
//       schema: input.schema,
//       vis: input.vis,
//       generics: input.generics,
//       fields: input
//         .data
//         .as_ref()
//         .take_struct()
//         .unwrap()
//         .fields
//         .into_iter()
//         .map(Field::try_from)
//         .collect::<Result<Vec<_>, darling::Error>>()?,
//       partial: partial_object,
//       partial_ref: partial_ref_object,
//       // partial_ref: PartialRefObject {
//       //   name: input.partial_ref.name.clone().unwrap_or_else(|| format_ident!("PartialRef{}", input.ident)),
//       //   vis: input.vis.clone(),
//       //   generics: input.generics.clone(),
//       //   fields: Vec::new(), // Placeholder for fields
//       //   attrs: input.partial_ref.attrs.clone().into(),
//       //   copy: input.partial_ref.copy,
//       // },
//       // selector: Selector {},
//     })
//   }

//   pub fn derive_defination(&self) -> proc_macro2::TokenStream {
//     let partial_object = self.partial.derive_defination(&self.path_to_grost);
//     let partial_ref_object = self.partial_ref.derive_defination(&self.path_to_grost);
//     let reflection_definition = self.generate_reflection();

//     quote! {
//       #reflection_definition

//       #partial_object

//       #partial_ref_object
//     }
//   }

//   /// Generates the reflection types for the object.
//   pub fn generate_reflection(&self) -> proc_macro2::TokenStream {
//     let reflection_name = format_ident!("{}Reflection", self.name);
//     let doc = format!(" The reflection of the [`{}`].", self.name);
//     let field_reflection_name = format_ident!("{}FieldReflection", self.name);
//     let field_reflection_doc = format!(" The field reflection of the [`{}`]'s fields.", self.name);
//     quote! {
//       #[doc = #field_reflection_doc]
//       pub struct #field_reflection_name<R: ?::core::marker::Sized, F: ?::core::marker::Sized, const TAG: ::core::primitive::u32> {
//         _reflect: ::core::marker::PhantomData<R>,
//         _flavor: ::core::marker::PhantomData<F>,
//       }

//       #[doc = #doc]
//       pub struct #reflection_name<R: ?::core::marker::Sized, F: ?::core::marker::Sized> {
//         _reflect: ::core::marker::PhantomData<R>,
//         _flavor: ::core::marker::PhantomData<F>,
//       }
//     }
//   }
// }
