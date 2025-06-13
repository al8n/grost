use heck::ToUpperCamelCase;
use quote::{ToTokens, format_ident, quote};
use syn::{
  Attribute, ConstParam, Generics, Ident, Path, Type, Variant, Visibility,
  parse::{Parse, Parser},
  parse_quote,
};

use super::{RawField, RawObject, ast::Object as ObjectAst};

pub use concrete::*;
pub use generic::*;

mod concrete;
mod generic;

#[derive(Debug, Clone)]
pub struct FieldIndex {
  variant: Variant,
  index: usize,
}

impl FieldIndex {
  pub(super) fn new(index: usize, field_name: &Ident, tag: u32) -> darling::Result<Self> {
    let variant = format_ident!("{}", field_name.to_string().to_upper_camel_case());
    let variant_doc = format!(" The field indexer for the field `{field_name}`");

    Ok(Self {
      variant: syn::Variant::parse.parse2(quote! {
        #[doc = #variant_doc]
        #variant = #tag
      })?,
      index,
    })
  }

  /// Returns the variant of the field
  #[inline]
  pub const fn variant(&self) -> &Variant {
    &self.variant
  }

  /// Returns the index of the field, this index is the index of the field in the object.
  #[inline]
  pub const fn index(&self) -> usize {
    self.index
  }
}

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
  pub const fn attrs(&self) -> &[Attribute] {
    match self {
      Self::Generic(generic) => generic.attrs(),
      Self::Concrete(concrete) => concrete.attrs(),
    }
  }

  /// Returns the generics of the object.
  #[inline]
  pub const fn generics(&self) -> &Generics {
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

fn optional_accessors(
  field_name: &Ident,
  vis: &Visibility,
  ty: &Type,
  copy: bool,
) -> proc_macro2::TokenStream {
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

  quote! {
    #[doc = #ref_fn_doc]
    #[inline]
    #vis const fn #ref_fn(&self) -> ::core::option::Option<&#ty> {
      self.#field_name.as_ref()
    }

    #[doc = #ref_mut_fn_doc]
    #[inline]
    #vis const fn #ref_mut_fn(&mut self) -> ::core::option::Option<&mut #ty> {
      self.#field_name.as_mut()
    }

    #[doc = #unwrap_ref_fn_doc]
    ///
    /// ## Panics
    ///
    #[doc = #panic_msg_doc]
    #[inline]
    #vis const fn #unwrap_ref_fn(&self) -> &#ty {
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
    #vis const fn #unwrap_mut_fn(&mut self) -> &mut #ty {
      match self.#field_name.as_mut() {
        ::core::option::Option::Some(value) => value,
        ::core::option::Option::None => panic!(#panic_msg),
      }
    }

    #[doc = #take_fn_doc]
    #[inline]
    #vis const fn #take_fn(&mut self) -> ::core::option::Option<#ty> {
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
    #vis #constable fn #set_fn(&mut self, value: #ty) -> &mut Self {
      self.#field_name = ::core::option::Option::Some(value);
      self
    }

    #[doc = #update_fn_doc]
    #[inline]
    #vis #constable fn #update_fn(&mut self, value: ::core::option::Option<#ty>) -> &mut Self {
      self.#field_name = value;
      self
    }

    #[doc = #set_fn_doc]
    #[inline]
    #vis #constable fn #with_fn(mut self, value: #ty) -> Self {
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
    #vis #constable fn #maybe_fn(mut self, value: ::core::option::Option<#ty>) -> Self {
      self.#field_name = value;
      self
    }
  }
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
    impl #ig #path_to_grost::__private::convert::State<#path_to_grost::__private::convert::Flatten<__GROST_FLATTEN_STATE__>> for #name #tg #w {
      type Output = Self;
      type Input = Self;
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
    #path_to_grost::__private::reflection::WireTypeReflection<
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
