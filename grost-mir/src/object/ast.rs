use quote::{ToTokens, format_ident, quote};
use syn::{Attribute, Generics, Ident, LifetimeParam, Path, Type, TypeParam, Visibility};

use crate::{
  flavor::{DecodeOptions, IdentifierOptions, TagOptions},
  object::{
    Label,
    meta::{ObjectConvertFromMeta, ObjectFromMeta, ObjectLabelConvertFromMeta},
  },
  utils::{Invokable, SchemaOptions, grost_flavor_param},
};

pub use concrete::{
  Object, PartialObject, PartialObjectOptions, PartialRefObject, PartialRefObjectOptions, RawField,
  RawTaggedField,
};

// pub(super) use concrete::ConcreteObject;
// pub use field::*;
// pub(super) use generic::GenericObject;
// pub use indexer::*;
// pub use partial::*;
// pub use partial_ref::*;
// pub use selector::*;

mod concrete;
mod field;
// mod generic;
mod indexer;
mod partial_ref;
mod selector;

#[derive(Debug, Clone)]
pub struct Indexer {
  pub(super) name: Ident,
  pub(super) attrs: Vec<Attribute>,
}

impl Indexer {
  /// Returns the name of the indexer
  #[inline]
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  /// Returns the attributes of the indexer
  #[inline]
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }
}

#[derive(Debug, Clone)]
pub struct ObjectLabelConvertOptions {
  pub(super) or_default: Option<bool>,
}

impl ObjectLabelConvertOptions {
  /// Returns `true` if the conversion should skip default values
  #[inline]
  pub const fn or_default(&self) -> bool {
    match self.or_default {
      Some(true) => true,
      Some(false) => false,
      None => false, // Default behavior is not to skip default values
    }
  }
}

impl From<ObjectLabelConvertFromMeta> for ObjectLabelConvertOptions {
  fn from(meta: ObjectLabelConvertFromMeta) -> Self {
    Self {
      or_default: meta.or_default.into(),
    }
  }
}

#[derive(Debug, Clone)]
pub struct ObjectConvertOptions {
  pub(super) or_default: Option<bool>,
  pub(super) scalar: ObjectLabelConvertOptions,
  pub(super) bytes: ObjectLabelConvertOptions,
  pub(super) string: ObjectLabelConvertOptions,
  pub(super) object: ObjectLabelConvertOptions,
  pub(super) enumeration: ObjectLabelConvertOptions,
  pub(super) interface: ObjectLabelConvertOptions,
  pub(super) union: ObjectLabelConvertOptions,
  pub(super) map: ObjectLabelConvertOptions,
  pub(super) set: ObjectLabelConvertOptions,
  pub(super) list: ObjectLabelConvertOptions,
}

impl ObjectConvertOptions {
  #[inline]
  pub const fn or_default_by_label(&self, label: &Label) -> bool {
    match label {
      Label::Scalar => self.or_default_scalar(),
      Label::Bytes => self.or_default_bytes(),
      Label::String => self.or_default_string(),
      Label::Object => self.or_default_object(),
      Label::Enum => self.or_default_enumeration(),
      Label::Interface => self.or_default_interface(),
      Label::Union => self.or_default_union(),
      Label::Map { .. } => self.or_default_map(),
      Label::Set(_) => self.or_default_set(),
      Label::List(_) => self.or_default_list(),
      Label::Nullable(_) => false,
    }
  }

  /// Returns `true` if the conversion should skip default values
  #[inline]
  pub const fn or_default(&self) -> bool {
    match self.or_default {
      Some(true) => true,
      Some(false) => false,
      None => false, // Default behavior is not to skip default values
    }
  }

  /// Returns the scalar conversion options
  #[inline]
  pub const fn scalar(&self) -> &ObjectLabelConvertOptions {
    &self.scalar
  }

  #[inline]
  pub const fn or_default_scalar(&self) -> bool {
    if self.scalar.or_default.is_some() {
      self.scalar.or_default()
    } else {
      self.or_default()
    }
  }

  /// Returns the bytes conversion options
  #[inline]
  pub const fn bytes(&self) -> &ObjectLabelConvertOptions {
    &self.bytes
  }

  pub const fn or_default_bytes(&self) -> bool {
    if self.bytes.or_default.is_some() {
      self.bytes.or_default()
    } else {
      self.or_default()
    }
  }

  /// Returns the string conversion options
  #[inline]
  pub const fn string(&self) -> &ObjectLabelConvertOptions {
    &self.string
  }

  pub const fn or_default_string(&self) -> bool {
    if self.string.or_default.is_some() {
      self.string.or_default()
    } else {
      self.or_default()
    }
  }

  /// Returns the object conversion options
  #[inline]
  pub const fn object(&self) -> &ObjectLabelConvertOptions {
    &self.object
  }

  pub const fn or_default_object(&self) -> bool {
    if self.object.or_default.is_some() {
      self.object.or_default()
    } else {
      self.or_default()
    }
  }

  /// Returns the enumeration conversion options
  #[inline]
  pub const fn enumeration(&self) -> &ObjectLabelConvertOptions {
    &self.enumeration
  }

  pub const fn or_default_enumeration(&self) -> bool {
    if self.enumeration.or_default.is_some() {
      self.enumeration.or_default()
    } else {
      self.or_default()
    }
  }

  /// Returns the interface conversion options
  #[inline]
  pub const fn interface(&self) -> &ObjectLabelConvertOptions {
    &self.interface
  }

  pub const fn or_default_interface(&self) -> bool {
    if self.interface.or_default.is_some() {
      self.interface.or_default()
    } else {
      self.or_default()
    }
  }

  /// Returns the union conversion options
  #[inline]
  pub const fn union(&self) -> &ObjectLabelConvertOptions {
    &self.union
  }

  pub const fn or_default_union(&self) -> bool {
    if self.union.or_default.is_some() {
      self.union.or_default()
    } else {
      self.or_default()
    }
  }

  /// Returns the map conversion options
  #[inline]
  pub const fn map(&self) -> &ObjectLabelConvertOptions {
    &self.map
  }

  /// Returns the map conversion options
  #[inline]
  pub const fn or_default_map(&self) -> bool {
    if self.map.or_default.is_some() {
      self.map.or_default()
    } else {
      self.or_default()
    }
  }

  /// Returns the set conversion options
  #[inline]
  pub const fn set(&self) -> &ObjectLabelConvertOptions {
    &self.set
  }

  pub const fn or_default_set(&self) -> bool {
    if self.set.or_default.is_some() {
      self.set.or_default()
    } else {
      self.or_default()
    }
  }

  /// Returns the list conversion options
  #[inline]
  pub const fn list(&self) -> &ObjectLabelConvertOptions {
    &self.list
  }

  pub const fn or_default_list(&self) -> bool {
    if self.list.or_default.is_some() {
      self.list.or_default()
    } else {
      self.or_default()
    }
  }
}

impl From<ObjectConvertFromMeta> for ObjectConvertOptions {
  fn from(meta: ObjectConvertFromMeta) -> Self {
    Self {
      or_default: meta.or_default.into(),
      scalar: meta.scalar.into(),
      bytes: meta.bytes.into(),
      string: meta.string.into(),
      object: meta.object.into(),
      enumeration: meta.enumeration.into(),
      interface: meta.interface.into(),
      union: meta.union.into(),
      map: meta.map.into(),
      set: meta.set.into(),
      list: meta.list.into(),
    }
  }
}

fn accessors(
  path_to_grost: &Path,
  field_name: &Ident,
  vis: &Visibility,
  ty: &Type,
  label: &Label,
  copy: bool,
) -> darling::Result<proc_macro2::TokenStream> {
  let ref_fn = format_ident!("{}_ref", field_name);
  let ref_fn_doc = format!(" Returns a reference to the `{field_name}`");
  let ref_mut_fn = format_ident!("{}_mut", field_name);
  let ref_mut_fn_doc = format!(" Returns a mutable reference to the `{field_name}`");
  let set_fn = format_ident!("set_{}", field_name);
  let set_fn_doc = format!(" Set the `{field_name}` to the given value");
  let with_fn = format_ident!("with_{}", field_name);
  let constable = copy.then(|| quote! { const });

  Ok(match label {
    Label::Nullable(_) => {
      let unwrap_ref_fn = format_ident!("unwrap_{}_ref", field_name);
      let unwrap_ref_fn_doc =
        format!(" Returns a reference to the `{field_name}` if it is not `None`");
      let unwrap_mut_fn = format_ident!("unwrap_{}_mut", field_name);
      let unwrap_mut_fn_doc =
        format!(" Returns a mutable reference to the `{field_name}` if it is not `None`");
      let panic_msg = format!("`{field_name}` is `None`");
      let panic_msg_doc = format!(" - Panics if the `{field_name}` is `None`");
      let update_fn = format_ident!("update_{}", field_name);
      let update_fn_doc =
        format!(" Update the `{field_name}` to the given value or clear the `{field_name}`");
      let clear_fn = format_ident!("clear_{}", field_name);
      let clear_fn_doc = format!(" Clear the value of `{field_name}`");
      let take_fn = format_ident!("take_{}", field_name);
      let take_fn_doc = format!(" Takes the value of `{field_name}` out if it is not `None`");
      let without_fn = format_ident!("without_{}", field_name);
      let maybe_fn = format_ident!("maybe_{}", field_name);
      let flatten_ty = quote! {
        <#ty as #path_to_grost::__private::convert::State<#path_to_grost::__private::convert::Flattened>>::Output
      };

      quote! {
        #[doc = #ref_fn_doc]
        #[inline]
        #vis const fn #ref_fn(&self) -> ::core::option::Option<&#flatten_ty> {
          self.#field_name.as_ref()
        }

        #[doc = #ref_mut_fn_doc]
        #[inline]
        #vis const fn #ref_mut_fn(&mut self) -> ::core::option::Option<&mut #flatten_ty> {
          self.#field_name.as_mut()
        }

        #[doc = #set_fn_doc]
        #[inline]
        #vis #constable fn #set_fn(&mut self, value: #flatten_ty) -> &mut Self {
          self.#field_name =::core::option::Option::Some(value);
          self
        }

        #[doc = #set_fn_doc]
        #[inline]
        #vis #constable fn #with_fn(mut self, value: #flatten_ty) -> Self {
          self.#field_name = ::core::option::Option::Some(value);
          self
        }

        #[doc = #unwrap_ref_fn_doc]
        ///
        /// ## Panics
        ///
        #[doc = #panic_msg_doc]
        #[inline]
        #vis const fn #unwrap_ref_fn(&self) -> &#flatten_ty {
          match self.#field_name.as_ref() {
            ::core::option::Option::Some(value) => value,
            ::core::option::Option::None => panic!(#panic_msg),
          }
        }

        #[doc = #unwrap_mut_fn_doc]
        ///
        /// ## Panics
        ///
        #[doc = #panic_msg_doc]
        #[inline]
        #vis const fn #unwrap_mut_fn(&mut self) -> &mut #flatten_ty {
          match self.#field_name.as_mut() {
            ::core::option::Option::Some(value) => value,
            ::core::option::Option::None => panic!(#panic_msg),
          }
        }

        #[doc = #take_fn_doc]
        #[inline]
        #vis const fn #take_fn(&mut self) -> #ty {
          self.#field_name.take()
        }

        #[doc = #clear_fn_doc]
        #[inline]
        #vis #constable fn #clear_fn(&mut self) -> &mut Self {
          self.#field_name = ::core::option::Option::None;
          self
        }

        #[doc = #update_fn_doc]
        #[inline]
        #vis #constable fn #update_fn(&mut self, value: #ty) -> &mut Self {
          self.#field_name = value;
          self
        }

        #[doc = #clear_fn_doc]
        #[inline]
        #vis #constable fn #without_fn(mut self) -> Self {
          self.#field_name = ::core::option::Option::None;
          self
        }

        #[doc = #update_fn_doc]
        #[inline]
        #vis #constable fn #maybe_fn(mut self, value: #ty) -> Self {
          self.#field_name = value;
          self
        }
      }
    }
    _ => {
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
  })
}

fn nullable_accessors(
  path_to_grost: &Path,
  field_name: &Ident,
  vis: &Visibility,
  ty: &Type,
  label: &Label,
  copy: bool,
) -> darling::Result<proc_macro2::TokenStream> {
  let ref_fn = format_ident!("{}_ref", field_name);
  let ref_fn_doc = format!(" Returns a reference to the `{field_name}`");
  let ref_mut_fn = format_ident!("{}_mut", field_name);
  let ref_mut_fn_doc = format!(" Returns a mutable reference to the `{field_name}`");
  let unwrap_ref_fn = format_ident!("unwrap_{}_ref", field_name);
  let unwrap_ref_fn_doc = format!(" Returns a reference to the `{field_name}` if it is not `None`");
  let unwrap_mut_fn = format_ident!("unwrap_{}_mut", field_name);
  let unwrap_mut_fn_doc =
    format!(" Returns a mutable reference to the `{field_name}` if it is not `None`");
  let panic_msg = format!("`{field_name}` is `None`");
  let panic_msg_doc = format!(" - Panics if the `{field_name}` is `None`");
  let set_fn = format_ident!("set_{}", field_name);
  let set_fn_doc = format!(" Set the `{field_name}` to the given value");
  let update_fn = format_ident!("update_{}", field_name);
  let update_fn_doc =
    format!(" Update the `{field_name}` to the given value or clear the `{field_name}`");
  let clear_fn = format_ident!("clear_{}", field_name);
  let clear_fn_doc = format!(" Clear the value of `{field_name}`");
  let take_fn = format_ident!("take_{}", field_name);
  let take_fn_doc = format!(" Takes the value of `{field_name}` out if it is not `None`");
  let with_fn = format_ident!("with_{}", field_name);
  let without_fn = format_ident!("without_{}", field_name);
  let maybe_fn = format_ident!("maybe_{}", field_name);
  let constable = copy.then(|| quote! { const });

  let nullable = label.is_nullable();

  let (nullable_mut_ty, nullable_ref_ty, nullable_ty, flatten_ty): (Type, Type, Type, Type) =
    if nullable {
      let flatten_ty = syn::parse2(quote! {
        <#ty as #path_to_grost::__private::convert::State<#path_to_grost::__private::convert::Flattened>>::Output
      })?;
      let nullable_mut_ty = syn::parse2(quote! { ::core::option::Option<&mut #flatten_ty> })?;
      let nullable_ref_ty = syn::parse2(quote! { ::core::option::Option<&#flatten_ty> })?;
      let nullable_ty = syn::parse2(quote! { #ty })?;
      (nullable_mut_ty, nullable_ref_ty, nullable_ty, flatten_ty)
    } else {
      let nullable_mut_ty = syn::parse2(quote! { ::core::option::Option<&mut #ty> })?;
      let nullable_ref_ty = syn::parse2(quote! { ::core::option::Option<&#ty> })?;
      let nullable_ty = syn::parse2(quote! { ::core::option::Option<#ty> })?;
      (nullable_mut_ty, nullable_ref_ty, nullable_ty, ty.clone())
    };

  Ok(quote! {
    #[doc = #ref_fn_doc]
    #[inline]
    #vis const fn #ref_fn(&self) -> #nullable_ref_ty {
      self.#field_name.as_ref()
    }

    #[doc = #ref_mut_fn_doc]
    #[inline]
    #vis const fn #ref_mut_fn(&mut self) -> #nullable_mut_ty {
      self.#field_name.as_mut()
    }

    #[doc = #unwrap_ref_fn_doc]
    ///
    /// ## Panics
    ///
    #[doc = #panic_msg_doc]
    #[inline]
    #vis const fn #unwrap_ref_fn(&self) -> &#flatten_ty {
      match self.#field_name.as_ref() {
        ::core::option::Option::Some(value) => value,
        ::core::option::Option::None => panic!(#panic_msg),
      }
    }

    #[doc = #unwrap_mut_fn_doc]
    ///
    /// ## Panics
    ///
    #[doc = #panic_msg_doc]
    #[inline]
    #vis const fn #unwrap_mut_fn(&mut self) -> &mut #flatten_ty {
      match self.#field_name.as_mut() {
        ::core::option::Option::Some(value) => value,
        ::core::option::Option::None => panic!(#panic_msg),
      }
    }

    #[doc = #take_fn_doc]
    #[inline]
    #vis const fn #take_fn(&mut self) -> #nullable_ty {
      self.#field_name.take()
    }

    #[doc = #clear_fn_doc]
    #[inline]
    #vis #constable fn #clear_fn(&mut self) -> &mut Self {
      self.#field_name = ::core::option::Option::None;
      self
    }

    #[doc = #set_fn_doc]
    #[inline]
    #vis #constable fn #set_fn(&mut self, value: #flatten_ty) -> &mut Self {
      self.#field_name = ::core::option::Option::Some(value);
      self
    }

    #[doc = #update_fn_doc]
    #[inline]
    #vis #constable fn #update_fn(&mut self, value: #nullable_ty) -> &mut Self {
      self.#field_name = value;
      self
    }

    #[doc = #set_fn_doc]
    #[inline]
    #vis #constable fn #with_fn(mut self, value: #flatten_ty) -> Self {
      self.#field_name = ::core::option::Option::Some(value);
      self
    }

    #[doc = #clear_fn_doc]
    #[inline]
    #vis #constable fn #without_fn(mut self) -> Self {
      self.#field_name = ::core::option::Option::None;
      self
    }

    #[doc = #update_fn_doc]
    #[inline]
    #vis #constable fn #maybe_fn(mut self, value: #nullable_ty) -> Self {
      self.#field_name = value;
      self
    }
  })
}

fn derive_flatten_state(
  path_to_grost: &Path,
  generics: &Generics,
  name: &Ident,
) -> proc_macro2::TokenStream {
  let mut all_generics = generics.clone();
  all_generics.params.push(
    syn::parse2(quote! {
      __GROST_FLATTEN_STATE__: ?::core::marker::Sized
    })
    .unwrap(),
  );

  let (ig, _, w) = all_generics.split_for_impl();
  let (_, tg, _) = generics.split_for_impl();

  quote! {
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl #ig #path_to_grost::__private::convert::State<#path_to_grost::__private::convert::Flattened<__GROST_FLATTEN_STATE__>> for #name #tg #w {
      type Output = Self;
    }
  }
}

fn field_reflection(
  path_to_grost: &Path,
  object_type: &Type,
  flavor_ty: impl ToTokens,
  tag: u32,
) -> syn::Result<Type> {
  syn::parse2(quote! {
    #path_to_grost::__private::reflection::ObjectFieldReflection<
      #object_type,
      #path_to_grost::__private::reflection::ObjectField,
      #flavor_ty,
      #tag,
    >
  })
}

fn wire_format_reflection(
  path_to_grost: &Path,
  object_type: &Type,
  flavor_ty: &Type,
  tag: u32,
) -> syn::Result<Type> {
  syn::parse2(quote! {
    #path_to_grost::__private::reflection::WireFormatReflection<
      #object_type,
      #flavor_ty,
      #tag,
    >
  })
}

fn wire_type_reflection(
  path_to_grost: &Path,
  object_type: &Type,
  flavor_ty: &Type,
  tag: u32,
) -> syn::Result<Type> {
  syn::parse2(quote! {
    #path_to_grost::__private::reflection::WireSchemaTypeReflection<
      #path_to_grost::__private::reflection::ObjectFieldReflection<
        #object_type,
        <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::WireType,
        #flavor_ty,
        #tag,
      >
    >
  })
}

fn identifier_reflection(
  path_to_grost: &Path,
  object_type: &Type,
  flavor_ty: &Type,
  tag: u32,
) -> syn::Result<Type> {
  syn::parse2(quote! {
    #path_to_grost::__private::reflection::IdentifierReflection<
      #path_to_grost::__private::reflection::ObjectFieldReflection<
        #object_type,
        <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Identifier,
        #flavor_ty,
        #tag,
      >
    >
  })
}

fn encoded_identifier_reflection(
  path_to_grost: &Path,
  object_type: &Type,
  flavor_ty: &Type,
  tag: u32,
) -> syn::Result<Type> {
  syn::parse2(quote! {
    #path_to_grost::__private::reflection::EncodeReflection<
      #path_to_grost::__private::reflection::IdentifierReflection<
        #path_to_grost::__private::reflection::ObjectFieldReflection<
          #object_type,
          <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Identifier,
          #flavor_ty,
          #tag,
        >
      >
    >
  })
}

fn encoded_identifier_len_reflection(
  path_to_grost: &Path,
  object_type: &Type,
  flavor_ty: &Type,
  tag: u32,
) -> syn::Result<Type> {
  syn::parse2(quote! {
    #path_to_grost::__private::reflection::Len<
      #path_to_grost::__private::reflection::EncodeReflection<
        #path_to_grost::__private::reflection::IdentifierReflection<
          #path_to_grost::__private::reflection::ObjectFieldReflection<
            #object_type,
            <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Identifier,
            #flavor_ty,
            #tag,
          >
        >
      >
    >
  })
}

fn tag_reflection(
  path_to_grost: &Path,
  object_type: &Type,
  flavor_ty: &Type,
  tag: u32,
) -> syn::Result<Type> {
  syn::parse2(quote! {
    #path_to_grost::__private::reflection::TagReflection<
      #path_to_grost::__private::reflection::ObjectFieldReflection<
        #object_type,
        <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Tag,
        #flavor_ty,
        #tag,
      >
    >
  })
}

fn encoded_tag_reflection(
  path_to_grost: &Path,
  object_type: &Type,
  flavor_ty: &Type,
  tag: u32,
) -> syn::Result<Type> {
  syn::parse2(quote! {
    #path_to_grost::__private::reflection::EncodeReflection<
      #path_to_grost::__private::reflection::TagReflection<
        #path_to_grost::__private::reflection::ObjectFieldReflection<
          #object_type,
          <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Tag,
          #flavor_ty,
          #tag,
        >
      >
    >
  })
}

fn encoded_tag_len_reflection(
  path_to_grost: &Path,
  object_type: &Type,
  flavor_ty: &Type,
  tag: u32,
) -> syn::Result<Type> {
  syn::parse2(quote! {
    #path_to_grost::__private::reflection::Len<
      #path_to_grost::__private::reflection::EncodeReflection<
        #path_to_grost::__private::reflection::TagReflection<
          #path_to_grost::__private::reflection::ObjectFieldReflection<
            #object_type,
            <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Tag,
            #flavor_ty,
            #tag,
          >
        >
      >
    >
  })
}

// #[derive(Debug, Clone)]
// pub struct ObjectFlavor {
//   pub(super) ty: Type,
//   pub(super) format: Type,
//   pub(super) identifier: IdentifierOptions,
//   pub(super) tag: TagOptions,
//   pub(super) decode: DecodeOptions,
// }

// impl ObjectFlavor {
//   /// Returns the type of the flavor
//   #[inline]
//   pub const fn ty(&self) -> &Type {
//     &self.ty
//   }

//   /// Returns the wire format type for the object of this flavor.
//   #[inline]
//   pub const fn format(&self) -> &Type {
//     &self.format
//   }

//   /// Returns the identifier attribute for the flavor.
//   #[inline]
//   pub const fn identifier(&self) -> &IdentifierOptions {
//     &self.identifier
//   }

//   /// Returns the tag attribute for the flavor.
//   #[inline]
//   pub const fn tag(&self) -> &TagOptions {
//     &self.tag
//   }

//   /// Returns the decode attribute for this flavor.
//   #[inline]
//   pub const fn decode(&self) -> &DecodeOptions {
//     &self.decode
//   }

//   fn from_attribute(attribute: &FlavorAttribute) -> darling::Result<Self> {
//     Ok(Self {
//       ty: attribute.ty().clone(),
//       format: attribute.wire_format().clone(),
//       identifier: attribute.identifier().clone(),
//       tag: attribute.tag().clone(),
//       decode: attribute.decode().clone(),
//     })
//   }
// }

// /// The AST for an object, which can be either a concrete or a generic object.
// ///
// /// The main purpose to having an AST for an object is used to validate the input (from the Rust's derive macro or attribute macro)
// /// from the schema and to generate the necessary Middle Intermediate Representation (MIR) of the object.
// ///
// /// A Middle Intermediate Representation (MIR) of the object can be
// /// generated from this structure. Once the MIR is generated,
// /// it can be used to generate the final Rust code for the object in a GraphQL Protocol schema.
// #[derive(Debug, Clone, derive_more::IsVariant, derive_more::Unwrap, derive_more::TryUnwrap)]
// #[unwrap(ref)]
// #[try_unwrap(ref)]
// pub(super) enum Object<M, F> {
//   Concrete(Box<ConcreteObject<M, F>>),
//   Generic(Box<GenericObject<M, F>>),
// }

// impl<M, F> Object<M, F> {
//   /// Creates an `Object` from a raw object input.
//   pub fn from_raw<O>(object: &O) -> darling::Result<Self>
//   where
//     O: RawObject<Meta = M>,
//     O::Field: RawField<Meta = F>,
//     M: Clone,
//     F: Clone,
//   {
//     let num_flavors = object.flavors().len();
//     if object.flavor_type_param().is_none() && num_flavors == 1 {
//       let flavor = object
//         .flavors()
//         .iter()
//         .next()
//         .expect("There must be a flavor were already checked");
//       ConcreteObject::try_new(object, flavor).map(|obj| Object::Concrete(Box::new(obj)))
//     } else {
//       let flavor_param = object
//         .flavor_type_param()
//         .cloned()
//         .unwrap_or_else(grost_flavor_param);
//       GenericObject::try_new(object, flavor_param).map(|obj| Object::Generic(Box::new(obj)))
//     }
//   }
// }

// #[derive(Debug, Clone)]
// pub struct RawObject<T = (), S = (), O = ()> {
//   name: Ident,
//   vis: Visibility,
//   generics: Generics,
//   attrs: Vec<Attribute>,
//   fields: Vec<RawField<T, S>>,
//   path_to_grost: Path,
//   flavors: Vec<FlavorAttribute>,
//   default: Option<Invokable>,
//   schema: SchemaOptions,
//   partial: PartialObjectOptions,
//   partial_ref: PartialRefObjectOptions,
//   selector: SelectorOptions,
//   selector_iter: SelectorIterOptions,
//   indexer: IndexerOptions,
//   copy: bool,
//   flavor_param: Option<TypeParam>,
//   buffer_param: TypeParam,
//   lifetime_param: LifetimeParam,
//   wire_format_param: TypeParam,
//   read_buffer_type_param: TypeParam,
//   write_buffer_type_param: TypeParam,
//   extra: O,
// }

// impl<T, S, O> RawObject<T, S, O> {
//   /// Creates a new `RawObject` from the given parameters.
//   pub fn new(
//     path_to_grost: Path,
//     name: Ident,
//     vis: Visibility,
//     generics: Generics,
//     attrs: Vec<Attribute>,
//     fields: Vec<RawField<T, S>>,
//     meta: ObjectFromMeta<O>,
//   ) -> darling::Result<Self> {
//     Ok(Self {
//       name,
//       vis,
//       generics,
//       attrs,
//       fields,
//       path_to_grost,
//       flavors: meta.flavor.finalize(&path_to_grost)?,
//       default: meta.default,
//       schema: meta.schema.into(),
//       partial: meta.partial.finalize(),
//       partial_ref: meta.partial_ref.finalize(),
//       selector: meta.selector.finalize(),
//       selector_iter: meta.selector_iter.finalize(),
//       indexer: meta.indexer.into(),
//       copy: meta.copy,
//       flavor_param: meta.generic.flavor,
//       buffer_param: meta.generic.buffer,
//       lifetime_param: meta.generic.lifetime,
//       wire_format_param: meta.generic.wire_format,
//       read_buffer_type_param: meta.generic.read_buffer,
//       write_buffer_type_param: meta.generic.write_buffer,
//       extra: meta.extra,
//     })
//   }

//   /// Returns the path to the `grost` crate
//   pub const fn path_to_grost(&self) -> &Path {
//     &self.path_to_grost
//   }

//   /// Returns the path to the fn that returns the default value of the object
//   pub const fn default(&self) -> Option<&Invokable> {
//     self.default.as_ref()
//   }

//   /// Returns the schema information
//   pub const fn schema(&self) -> &SchemaOptions {
//     &self.schema
//   }

//   /// Returns the partial object information
//   pub const fn partial(&self) -> &PartialObjectOptions {
//     &self.partial
//   }

//   /// Returns the partial decoded object information
//   pub const fn partial_ref(&self) -> &PartialRefObjectOptions {
//     &self.partial_ref
//   }

//   /// Returns the selector information
//   pub const fn selector(&self) -> &SelectorOptions {
//     &self.selector
//   }

//   /// Returns the selector iterator information
//   pub const fn selector_iter(&self) -> &SelectorIterOptions {
//     &self.selector_iter
//   }

//   /// Returns the indexer information
//   pub const fn indexer(&self) -> &IndexerOptions {
//     &self.indexer
//   }

//   /// Returns whether the object is copyable
//   pub const fn copy(&self) -> bool {
//     self.copy
//   }

//   /// Returns the flavors of the object
//   pub const fn flavors(&self) -> &[FlavorAttribute] {
//     self.flavors.as_slice()
//   }

//   /// Returns the generic flavor type parameter
//   pub const fn flavor_type_param(&self) -> Option<&TypeParam> {
//     self.flavor_param.as_ref()
//   }

//   /// Returns the generic unknown buffer type parameter
//   pub const fn buffer_type_param(&self) -> &TypeParam {
//     &self.buffer_param
//   }

//   /// Returns the generic lifetime parameter
//   pub const fn lifetime_param(&self) -> &LifetimeParam {
//     &self.lifetime_param
//   }

//   /// Returns the generic wire format type parameter
//   pub const fn wire_format_type_param(&self) -> &TypeParam {
//     &self.wire_format_param
//   }

//   /// Returns the generic read buffer type parameter
//   pub const fn read_buffer_type_param(&self) -> &TypeParam {
//     &self.read_buffer_type_param
//   }

//   /// Returns the generic write buffer type parameter
//   pub const fn write_buffer_type_param(&self) -> &TypeParam {
//     &self.write_buffer_type_param
//   }

//   /// Returns the extra metadata associated with the object
//   pub const fn extra(&self) -> &O {
//     &self.extra
//   }

//   #[inline]
//   pub fn partial_ref_name(&self) -> Ident {
//     self
//       .partial_ref()
//       .name()
//       .cloned()
//       .unwrap_or_else(|| format_ident!("PartialRef{}", self.name()))
//   }

//   #[inline]
//   pub fn partial_name(&self) -> Ident {
//     self
//       .partial()
//       .name()
//       .cloned()
//       .unwrap_or_else(|| format_ident!("Partial{}", self.name()))
//   }

//   #[inline]
//   pub fn selector_name(&self) -> Ident {
//     self
//       .selector()
//       .name()
//       .cloned()
//       .unwrap_or_else(|| format_ident!("{}Selector", self.name()))
//   }

//   #[inline]
//   pub fn selector_iter_name(&self) -> Ident {
//     self
//       .selector_iter()
//       .name()
//       .cloned()
//       .unwrap_or_else(|| format_ident!("{}Iter", self.selector_name()))
//   }

//   #[inline]
//   pub fn indexer_name(&self) -> Ident {
//     self
//       .indexer()
//       .name()
//       .cloned()
//       .unwrap_or_else(|| format_ident!("{}Index", self.name()))
//   }
// }
