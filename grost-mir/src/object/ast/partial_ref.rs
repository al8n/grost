// use quote::format_ident;
// use syn::{Attribute, Ident};

// use crate::object::meta::concrete::PartialRefObjectFromMeta;

// #[derive(Debug, Clone)]
// pub struct GenericPartialRefObject {
//   name: Ident,
//   attrs: Vec<Attribute>,
//   copy: bool,
// }

// impl GenericPartialRefObject {
//   pub(super) fn from_attribute(
//     object_name: &Ident,
//     attribute: &PartialRefObjectOptions,
//   ) -> darling::Result<Self> {
//     let name = if let Some(name) = attribute.name() {
//       name.clone()
//     } else {
//       format_ident!("PartialRef{}", object_name)
//     };

//     Ok(Self {
//       name,
//       attrs: attribute.attrs().to_vec(),
//       copy: attribute.copy(),
//     })
//   }

//   /// Returns the name of the generic partial decoded object
//   pub const fn name(&self) -> &Ident {
//     &self.name
//   }

//   /// Returns the attributes of the generic partial decoded object
//   pub const fn attrs(&self) -> &[Attribute] {
//     self.attrs.as_slice()
//   }

//   /// Returns whether the generic partial decoded object is copyable
//   pub const fn copy(&self) -> bool {
//     self.copy
//   }
// }
