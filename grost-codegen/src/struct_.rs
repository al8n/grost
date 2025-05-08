use heck::{ToSnakeCase, ToUpperCamelCase};
use quote::{format_ident, quote};
use smol_str::SmolStr;
use syn::{Attribute, Ident, Visibility, parse_quote};

use crate::{FlavorGenerator, SafeIdent, struct_};

pub use field::Field;

/// The field of a struct
pub mod field;

mod index;
mod reflection;
mod selection;

pub struct Struct {
  name: SafeIdent,
  schema_name: SmolStr,
  selector_name: SafeIdent,
  description: Option<SmolStr>,
  fields: Vec<Field>,
  attrs: Vec<Attribute>,
  visibility: Option<Visibility>,
}

impl Struct {
  pub fn new(name: SafeIdent, mut fields: Vec<Field>) -> Self {
    fields.sort_by_key(|f| f.tag());
    Self {
      selector_name: SafeIdent::new(format!("{}Selector", name.name_str()).as_str()),
      schema_name: name.original_str().into(),
      name,
      description: None,
      fields,
      attrs: Vec::new(),
      visibility: Some(parse_quote!(pub)),
    }
  }

  pub fn with_schema_name(mut self, name: impl Into<SmolStr>) -> Self {
    self.schema_name = name.into();
    self
  }

  /// Change the default selection type name
  pub fn with_selector_name(mut self, name: SafeIdent) -> Self {
    self.selector_name = name;
    self
  }

  pub fn with_description(mut self, description: impl Into<SmolStr>) -> Self {
    self.description = Some(description.into());
    self
  }

  pub fn with_visibility(mut self, visibility: Visibility) -> Self {
    self.visibility = Some(visibility);
    self
  }

  pub fn with_attrs(mut self, attrs: Vec<Attribute>) -> Self {
    self.attrs = attrs;
    self
  }

  pub fn with_fields(mut self, fields: Vec<Field>) -> Self {
    self.fields = fields;
    self
  }

  pub fn add_field(&mut self, field: Field) -> &mut Self {
    self.fields.push(field);
    self
  }

  pub fn selector_name(&self) -> &SafeIdent {
    &self.selector_name
  }

  pub fn selector_iter_name(&self) -> Ident {
    format_ident!("{}Iter", self.selector_name.name_str())
  }

  pub fn indexer_name(&self) -> Ident {
    format_ident!("{}FieldIndex", self.name.name_str())
  }

  pub fn partial_ref_name(&self) -> Ident {
    format_ident!("Partial{}Ref", self.name.name_str())
  }

  pub fn name(&self) -> &SafeIdent {
    &self.name
  }

  pub fn partial_struct_name(&self) -> syn::Ident {
    format_ident!("Partial{}", self.name.name_str())
  }

  pub fn schema_name(&self) -> &str {
    &self.schema_name
  }

  pub fn fields(&self) -> &[Field] {
    &self.fields
  }

  pub fn reflection_name(&self) -> Ident {
    format_ident!("{}Reflection", self.name.name_str())
  }

  pub fn field_reflection_name(&self) -> Ident {
    let struct_name = self.name.name_str();
    format_ident!("{struct_name}FieldReflection")
  }

  pub(crate) fn struct_defination(&self) -> proc_macro2::TokenStream {
    let name = &self.name;
    let description = self.description.as_ref().map(|d| {
      let s = d.as_str();
      quote! {
        #[doc = #s]
      }
    });
    let visibility = self.visibility.as_ref();
    let attrs = &self.attrs;
    let fields = self.fields.iter().map(|f| f.field_defination());

    quote! {
      #[derive(::core::fmt::Debug, ::core::clone::Clone)]
      #description
      #(#attrs)*
      #visibility struct #name {
        #(#fields),*
      }
    }
  }

  pub(crate) fn partial_struct_defination(&self) -> proc_macro2::TokenStream {
    let name = format_ident!("Partial{}", self.name.name_str());
    let visibility = self.visibility.as_ref();
    let fields = self.fields.iter().map(|f| {
      let field_name = f.name();
      let ty = f.ty().repr().partial_ty();
      quote! {
        #field_name: ::core::option::Option<#ty>,
      }
    });
    let doc = format!(" The partial struct of [`{}`]", self.name.name_str());

    quote! {
      #[doc = #doc]
      #visibility struct #name<__GROST_BYTES_BUFFER__, __GROST_UNKNOWN_BUFFER__> {
        #(#fields)*
        __grost_unknown__: ::core::option::Option<__GROST_UNKNOWN_BUFFER__>,
        _bytes_buffer: ::core::marker::PhantomData<__GROST_BYTES_BUFFER__>,
      }
    }
  }

  pub(crate) fn partial_struct_impl(&self, path_to_grost: &syn::Path) -> proc_macro2::TokenStream {
    let name = self.partial_struct_name();
    let struct_name = self.name();
    let fields = self.fields.iter().map(|f| {
      let field_name = f.name();
      quote! {
        #field_name: ::core::option::Option::None,
      }
    });

    let fields_accessors = self.fields.iter().map(|f| {
      let field_name = f.name();
      let ref_fn = format_ident!("{}_ref", field_name.name_str());
      let ref_mut_fn = format_ident!("{}_mut", field_name.name_str());
      let set_fn = format_ident!("set_{}", field_name.name_str());
      let take_fn = format_ident!("take_{}", field_name.name_str());
      let without_fn = format_ident!("without_{}", field_name.name_str());
      let with_fn = format_ident!("with_{}", field_name.name_str());
      let clear_fn = format_ident!("clear_{}", field_name.name_str());
      let ty = f.ty();
      let constable = ty.copy().then(|| quote! { const });
      let ty = ty.repr().partial_ty();

      quote! {
        #[inline]
        pub const fn #ref_fn(&self) -> ::core::option::Option<&#ty> {
          self.#field_name.as_ref()
        }

        #[inline]
        pub const fn #ref_mut_fn(&mut self) -> ::core::option::Option<&mut #ty> {
          self.#field_name.as_mut()
        }

        #[inline]
        pub const fn #take_fn(&mut self) -> ::core::option::Option<#ty> {
          self.#field_name.take()
        }

        #[inline]
        pub #constable fn #clear_fn(&mut self) -> &mut Self {
          self.#field_name = ::core::option::Option::None;
          self
        }

        #[inline]
        pub #constable fn #set_fn(&mut self, value: #ty) -> &mut Self {
          self.#field_name = ::core::option::Option::Some(value);
          self
        }

        #[inline]
        pub #constable fn #with_fn(mut self, value: #ty) -> Self {
          self.#field_name = ::core::option::Option::Some(value);
          self
        }

        #[inline]
        pub #constable fn #without_fn(mut self) -> Self {
          self.#field_name = ::core::option::Option::None;
          self
        }
      }
    });

    quote! {
      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl<__GROST_BYTES_BUFFER__, __GROST_UNKNWON_BUFFER__, __GROST_FLAVOR__> #path_to_grost::__private::Selectable<__GROST_FLAVOR__> for #name<__GROST_BYTES_BUFFER__, __GROST_UNKNWON_BUFFER__>
      where
        __GROST_FLAVOR__: ?::core::marker::Sized,
      {
        type Selector = <#struct_name as #path_to_grost::__private::Selectable<__GROST_FLAVOR__>>::Selector;
      }

      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl<__GROST_BYTES_BUFFER__, __GROST_UNKNWON_BUFFER__> ::core::default::Default for #name<__GROST_BYTES_BUFFER__, __GROST_UNKNWON_BUFFER__> {
        fn default() -> Self {
          Self::new()
        }
      }

      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl<__GROST_BYTES_BUFFER__, __GROST_UNKNWON_BUFFER__> #name<__GROST_BYTES_BUFFER__, __GROST_UNKNWON_BUFFER__> {
        /// Creates an empty partial struct.
        pub const fn new() -> Self {
          Self {
            #(#fields)*
            __grost_unknown__: ::core::option::Option::None,
            _bytes_buffer: ::core::marker::PhantomData,
          }
        }

        pub const fn unknown(&self) -> ::core::option::Option<&__GROST_UNKNWON_BUFFER__> {
          self.__grost_unknown__.as_ref()
        }

        pub const fn unknown_mut(&mut self) -> ::core::option::Option<&mut __GROST_UNKNWON_BUFFER__> {
          self.__grost_unknown__.as_mut()
        }

        pub fn set_unknown(&mut self, value: __GROST_UNKNWON_BUFFER__) -> &mut Self {
          self.__grost_unknown__ = ::core::option::Option::Some(value);
          self
        }

        pub fn with_unknown(mut self, value: __GROST_UNKNWON_BUFFER__) -> Self {
          self.__grost_unknown__ = ::core::option::Option::Some(value);
          self
        }

        pub fn without_unknown(mut self) -> Self {
          self.__grost_unknown__ = ::core::option::Option::None;
          self
        }

        pub fn clear_unknown(&mut self) -> &mut Self {
          self.__grost_unknown__ = ::core::option::Option::None;
          self
        }

        pub fn take_unknown(&mut self) -> ::core::option::Option<__GROST_UNKNWON_BUFFER__> {
          self.__grost_unknown__.take()
        }

        #(#fields_accessors)*
      }
    }
  }

  pub(crate) fn partial_encoded_struct_defination<F>(
    &self,
    path_to_grost: &syn::Path,
    flavor: &F,
  ) -> proc_macro2::TokenStream
  where
    F: FlavorGenerator + ?Sized,
  {
    let struct_name = self.name();
    let name = self.partial_ref_name();
    let vis = self.visibility.as_ref();
    let flavor_ty = flavor.ty();
    let fields = self.fields.iter().map(|f| {
      let field_name = f.name();
      let ty = f.ty();
      let wf = f.get_wire_format_or_default(path_to_grost, flavor);
      let encoded_ref_ty = ty.repr().message_ref_ty(path_to_grost, flavor, &wf);

      quote! {
        #field_name: ::core::option::Option<#encoded_ref_ty>,
      }
    });
    let fields_init = self.fields.iter().map(|f| {
      let field_name = f.name();
      quote! {
        #field_name: ::core::option::Option::None,
      }
    });
    let fields_accessors = self.fields.iter()
      .map(|f| {
        let field_name = f.name();
        let ref_fn = format_ident!("{}_ref", field_name.name_str());
        let ref_mut_fn = format_ident!("{}_mut", field_name.name_str());
        let set_fn = format_ident!("set_{}", field_name.name_str());
        let take_fn = format_ident!("take_{}", field_name.name_str());
        let without_fn = format_ident!("without_{}", field_name.name_str());
        let with_fn = format_ident!("with_{}", field_name.name_str());
        let clear_fn = format_ident!("clear_{}", field_name.name_str());
        let ty = f.ty();

        quote! {
          #[inline]
          pub const fn #ref_fn(&self) -> ::core::option::Option<&<#ty as #path_to_grost::__private::Referenceable<#flavor_ty>>::Ref<'a, UB>> {
            self.#field_name.as_ref()
          }

          #[inline]
          pub const fn #ref_mut_fn(&mut self) -> ::core::option::Option<&mut <#ty as #path_to_grost::__private::Referenceable<#flavor_ty>>::Ref<'a, UB>> {
            self.#field_name.as_mut()
          }

          #[inline]
          pub const fn #take_fn(&mut self) -> ::core::option::Option<<#ty as #path_to_grost::__private::Referenceable<#flavor_ty>>::Ref<'a, UB>> {
            self.#field_name.take()
          }

          #[inline]
          pub fn #clear_fn(&mut self) -> &mut Self {
            self.#field_name = ::core::option::Option::None;
            self
          }

          #[inline]
          pub fn #set_fn(&mut self, value: <#ty as #path_to_grost::__private::Referenceable<#flavor_ty>>::Ref<'a, UB>) -> &mut Self {
            self.#field_name = ::core::option::Option::Some(value);
            self
          }

          #[inline]
          pub fn #with_fn(mut self, value: <#ty as #path_to_grost::__private::Referenceable<#flavor_ty>>::Ref<'a, UB>) -> Self {
            self.#field_name = ::core::option::Option::Some(value);
            self
          }

          #[inline]
          pub fn #without_fn(mut self) -> Self {
            self.#field_name = ::core::option::Option::None;
            self
          }
        }
      });

    quote! {
      #vis struct #name<'a, UB> {
        #(#fields)*
        unknown: ::core::option::Option<UB>,
      }

      // impl #path_to_grost::__private::Referenceable<#flavor_ty, #path_to_grost::__private::flavors::network::LengthDelimited> for #struct_name {
      //   type Ref<'b> = #name<'b> where Self: 'b;
      // }

      impl<'a, UB> ::core::default::Default for #name<'a, UB> {
        fn default() -> Self {
          Self::new()
        }
      }

      impl<'a, UB> #name<'a, UB> {
        /// Creates an empty instance.
        #[inline]
        pub const fn new() -> Self {
          Self {
            #(#fields_init)*
            unknown: ::core::option::Option::None,
          }
        }

        pub const fn unknown(&self) -> ::core::option::Option<&UB> {
          self.unknown.as_ref()
        }

        pub const fn unknown_mut(&mut self) -> ::core::option::Option<&mut UB> {
          self.unknown.as_mut()
        }

        pub fn set_unknown(&mut self, value: UB) -> &mut Self {
          self.unknown = ::core::option::Option::Some(value);
          self
        }

        pub fn with_unknown(mut self, value: UB) -> Self {
          self.unknown = ::core::option::Option::Some(value);
          self
        }

        pub fn without_unknown(mut self) -> Self {
          self.unknown = ::core::option::Option::None;
          self
        }

        pub fn clear_unknown(&mut self) -> &mut Self {
          self.unknown = ::core::option::Option::None;
          self
        }

        pub fn take_unknown(&mut self) -> ::core::option::Option<UB> {
          self.unknown.take()
        }

        // #(#fields_accessors)*
      }
    }
  }

  pub(crate) fn struct_impl(&self) -> proc_macro2::TokenStream {
    let name = &self.name;
    let fields = &self.fields;
    let default_fields = fields.iter().map(|f| {
      let name = f.name();
      let default = f.default();
      if let Some(default) = default {
        quote! {
          #name: #default
        }
      } else {
        quote! {
          #name: ::core::default::Default::default()
        }
      }
    });

    let accessors = fields.iter().map(|f| f.field_accessors());

    quote! {
      impl ::core::default::Default for #name {
        fn default() -> Self {
          Self::new()
        }
      }

      impl #name {
        /// Returns a new default instance of the struct
        pub fn new() -> Self {
          Self {
            #(#default_fields),*
          }
        }

        #(#accessors)*
      }
    }
  }
}
