use std::num::NonZeroU32;

use quote::{ToTokens, format_ident, quote};
use syn::{Attribute, Generics, Ident, Path, Type, Visibility, parse::Parser};

pub use indexer::Indexer;
pub use partial::{PartialField, PartialObject};
pub use partial_decoded::{PartialDecodedField, PartialDecodedObject};
pub use reflection::Reflection;
pub use selector::{Selector, SelectorField, SelectorIter};

use crate::ast::{
  SchemaAttribute,
  object::{Label, RawField as _, RawObjectExt as _},
};

mod indexer;
mod partial;
mod partial_decoded;
mod reflection;
mod selector;

pub struct Field<M> {
  name: Ident,
  ty: Type,
  vis: Visibility,
  schema: SchemaAttribute,
  default: Option<Path>,
  tag: NonZeroU32,
  wire: Option<Type>,
  label: Label,
  attrs: Vec<Attribute>,
  copy: bool,
  meta: M,
}

impl<M> Field<M> {
  /// Returns the field name.
  #[inline]
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  /// Returns the field type.
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the field visibility.
  #[inline]
  pub const fn vis(&self) -> &Visibility {
    &self.vis
  }

  /// Returns the field tag.
  #[inline]
  pub const fn tag(&self) -> NonZeroU32 {
    self.tag
  }

  /// Returns the field wire format type.
  #[inline]
  pub const fn wire(&self) -> Option<&Type> {
    self.wire.as_ref()
  }

  /// Returns the field type label.
  #[inline]
  pub const fn label(&self) -> &Label {
    &self.label
  }

  /// Returns the fn that returns the default value of the field.
  #[inline]
  pub const fn default(&self) -> Option<&Path> {
    self.default.as_ref()
  }

  /// Returns whether the field is copyable.
  #[inline]
  pub const fn copy(&self) -> bool {
    self.copy
  }

  /// Returns the field attributes.
  #[inline]
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns the schema information of the field.
  #[inline]
  pub const fn schema(&self) -> &SchemaAttribute {
    &self.schema
  }

  /// Returns the meta information of the field.
  #[inline]
  pub const fn meta(&self) -> &M {
    &self.meta
  }

  fn from_input(input: M, copy: bool) -> darling::Result<Self>
  where
    M: crate::ast::object::RawField,
  {
    let tag = match input.tag() {
      Some(tag) => tag,
      None => {
        let name = input.name();
        return Err(
          darling::Error::custom(format!(
            "{name} field is missing tag attribute, please add e.g. `tag = 1`"
          ))
          .with_span(&proc_macro2::Span::call_site()),
        );
      }
    };
    Ok(Self {
      name: input.name().clone(),
      ty: input.ty().clone(),
      vis: input.vis().clone(),
      tag,
      label: input.label().clone(),
      attrs: input.attrs().to_vec(),
      copy,
      schema: input.schema().clone(),
      default: input.default().cloned(),
      wire: None,
      meta: input,
    })
  }
}

pub struct SkippedField<M> {
  name: Ident,
  ty: Type,
  vis: Visibility,
  default: Option<Path>,
  attrs: Vec<Attribute>,
  meta: M,
}

impl<M> SkippedField<M> {
  /// Returns the field name.
  #[inline]
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  /// Returns the field type.
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the field visibility.
  #[inline]
  pub const fn vis(&self) -> &Visibility {
    &self.vis
  }

  /// Returns the fn that returns the default value of the field.
  #[inline]
  pub const fn default(&self) -> Option<&Path> {
    self.default.as_ref()
  }
  /// Returns the field attributes.
  #[inline]
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns the meta information of the field.
  #[inline]
  pub const fn meta(&self) -> &M {
    &self.meta
  }

  fn from_input(input: M) -> darling::Result<Self>
  where
    M: crate::ast::object::RawField,
  {
    Ok(Self {
      name: input.name().clone(),
      ty: input.ty().clone(),
      vis: input.vis().clone(),
      attrs: input.attrs().to_vec(),
      default: input.default().cloned(),
      meta: input,
    })
  }
}

pub struct Object<M>
where
  M: crate::ast::object::RawObject,
{
  name: Ident,
  path_to_grost: Path,
  schema: SchemaAttribute,
  vis: Visibility,
  generics: Generics,
  fields: Vec<Field<M::Field>>,
  skipped_fields: Vec<SkippedField<M::Field>>,
  partial: PartialObject,
  partial_decoded: PartialDecodedObject,
  reflection: Reflection,
  selector: Selector,
  selector_iter: SelectorIter,
  attrs: Vec<Attribute>,
  indexer: Indexer,
  meta: M,
}

impl<M> Object<M>
where
  M: crate::ast::object::RawObject,
{
  #[inline]
  pub const fn meta(&self) -> &M {
    &self.meta
  }

  #[inline]
  pub const fn path(&self) -> &Path {
    &self.path_to_grost
  }

  #[inline]
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  #[inline]
  pub const fn vis(&self) -> &Visibility {
    &self.vis
  }

  #[inline]
  pub const fn generics(&self) -> &Generics {
    &self.generics
  }

  /// Returns the fields of the object, excluding the skipped fields.
  #[inline]
  pub const fn fields(&self) -> &[Field<M::Field>]
  where
    M: crate::ast::object::RawObject,
  {
    self.fields.as_slice()
  }

  /// Returns the fields that are skipped.
  #[inline]
  pub const fn skipped_fields(&self) -> &[SkippedField<M::Field>] {
    self.skipped_fields.as_slice()
  }

  #[inline]
  pub const fn shema(&self) -> &SchemaAttribute {
    &self.schema
  }

  #[inline]
  pub const fn partial(&self) -> &PartialObject {
    &self.partial
  }

  #[inline]
  pub const fn partial_decoded(&self) -> &PartialDecodedObject {
    &self.partial_decoded
  }

  #[inline]
  pub const fn reflection(&self) -> &Reflection {
    &self.reflection
  }

  #[inline]
  pub const fn selector(&self) -> &Selector {
    &self.selector
  }

  #[inline]
  pub const fn selector_iter(&self) -> &SelectorIter {
    &self.selector_iter
  }

  #[inline]
  pub const fn indexer(&self) -> &Indexer {
    &self.indexer
  }

  #[inline]
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }
}

impl<M> Object<M>
where
  M: crate::ast::object::RawObject,
{
  pub fn from_derive_input(input: &syn::DeriveInput) -> darling::Result<Self>
  where
    M: darling::FromDeriveInput,
  {
    <M as darling::FromDeriveInput>::from_derive_input(input).and_then(Self::from_object)
  }

  pub fn from_object(input: M) -> darling::Result<Self> {
    let path_to_grost = input.path_to_grost();
    let mut fields = vec![];
    let mut skipped_fields = vec![];

    for f in input.fields() {
      if f.skip() {
        skipped_fields.push(SkippedField::from_input(f.clone())?);
      } else {
        let copy = input.copy() | f.copy();
        fields.push(Field::from_input(f.clone(), copy)?);
      }
    }

    let partial_object = PartialObject::from_input(path_to_grost, &input)?;
    let partial_decoded_object = PartialDecodedObject::from_input(path_to_grost, &input)?;
    let selector = Selector::from_input(path_to_grost, &input)?;
    let selector_iter = selector.selector_iter(
      input.selector_iter_name(),
      input.indexer_name(),
      input.selector_iter(),
    )?;
    let indexer = Indexer::from_input(&input)?;
    let reflection = Reflection::from_input(&input)?;

    Ok(Self {
      name: input.name().clone(),
      skipped_fields,
      attrs: input.attrs().to_vec(),
      path_to_grost: path_to_grost.clone(),
      schema: input.schema().clone(),
      vis: input.vis().clone(),
      generics: input.generics().clone(),
      fields,
      partial: partial_object,
      partial_decoded: partial_decoded_object,
      reflection,
      selector_iter,
      selector,
      meta: input,
      indexer,
    })
  }

  /// Derives the object.
  pub fn derive(&self) -> syn::Result<proc_macro2::TokenStream> {
    let partial_object = self.partial().to_token_stream();
    let partial_decoded_object = self.partial_decoded().to_token_stream();
    let selector = self.selector().to_token_stream();
    let selector_iter = self.selector_iter().to_token_stream();
    let indexer = self.indexer().to_token_stream();

    let reflection_impl = self.derive_reflection()?;
    let indexer_impl = self.derive_indexer();
    let selector_iter_impl = self.derive_selector_iter();
    let selector_impl = self.derive_selector();
    let partial_decoded_object_impl = self.derive_partial_decoded_object();
    let partial_impl = self.derive_partial_object();

    let path_to_grost = self.path();
    let flatten_state = derive_flatten_state(path_to_grost, self.generics(), self.name());
    let accessors = self.derive_accessors();
    let default = self.derive_default();

    Ok(quote! {
      #partial_object

      #partial_decoded_object

      #indexer

      #selector

      #selector_iter

      const _: () = {
        #reflection_impl

        #indexer_impl

        #selector_impl

        #selector_iter_impl

        #partial_decoded_object_impl

        #partial_impl

        #default

        #flatten_state

        #accessors
      };
    })
  }

  fn derive_default(&self) -> proc_macro2::TokenStream {
    let name = self.name();
    let mut generics = Generics::default();
    let original_generics = self.generics();

    original_generics.type_params().for_each(|tp| {
      let ident = &tp.ident;
      generics.make_where_clause().predicates.push(
        syn::parse2(quote! {
          #ident: ::core::default::Default
        })
        .unwrap(),
      );
    });

    if let Some(ref where_clause) = original_generics.where_clause {
      generics
        .make_where_clause()
        .predicates
        .extend(where_clause.predicates.iter().cloned());
    }

    let (_, _, w) = generics.split_for_impl();
    let (ig, tg, _) = original_generics.split_for_impl();
    let fields = self
      .fields
      .iter()
      .map(|f| {
        let field_name = f.name();
        let default = match f.default() {
          Some(p) => quote! { #p() },
          None => quote! { ::core::default::Default::default() },
        };

        quote! {
          #field_name: #default
        }
      })
      .chain(self.skipped_fields.iter().map(|f| {
        let field_name = f.name();
        let default = match f.default() {
          Some(p) => quote! { #p() },
          None => quote! { ::core::default::Default::default() },
        };

        quote! {
          #field_name: #default
        }
      }));

    quote! {
      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl #ig ::core::default::Default for #name #tg #w
      {
        fn default() -> Self {
          Self::new()
        }
      }

      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl #ig #name #tg #w {
        /// Creates a new instance with default values.
        #[inline]
        pub fn new() -> Self {
          Self {
            #(#fields),*
          }
        }
      }
    }
  }

  fn derive_accessors(&self) -> proc_macro2::TokenStream {
    let fns = self
      .fields
      .iter()
      .map(|f| accessors(f.name(), f.ty(), f.copy()));
    let (ig, tg, w) = self.generics().split_for_impl();
    let name = self.name();

    quote! {
      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl #ig #name #tg #w {
        #(#fns)*
      }
    }
  }
}

impl<M> ToTokens for Object<M>
where
  M: crate::ast::object::RawObject,
{
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let name = self.name();
    let vis = self.vis();
    let generics = self.generics();
    let where_clause = generics.where_clause.as_ref();
    let attrs = self.attrs();
    let fields = self
      .fields
      .iter()
      .map(|f| {
        let field_name = f.name();
        let field_ty = f.ty();
        let field_vis = f.vis();
        let field_attrs = f.attrs();
        quote! {
          #(#field_attrs)*
          #field_vis #field_name: #field_ty
        }
      })
      .chain(self.skipped_fields.iter().map(|f| {
        let field_name = f.name();
        let field_ty = f.ty();
        let field_vis = f.vis();
        let field_attrs = f.attrs();
        quote! {
          #(#field_attrs)*
          #field_vis #field_name: #field_ty
        }
      }));

    tokens.extend(quote! {
      #(#attrs)*
      #vis struct #name #generics #where_clause {
        #(#fields),*
      }
    });
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

fn accessors(field_name: &Ident, ty: &Type, copy: bool) -> proc_macro2::TokenStream {
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
    pub const fn #ref_fn(&self) -> &#ty {
      &self.#field_name
    }

    #[doc = #ref_mut_fn_doc]
    #[inline]
    pub const fn #ref_mut_fn(&mut self) -> &mut #ty {
      &mut self.#field_name
    }

    #[doc = #set_fn_doc]
    #[inline]
    pub #constable fn #set_fn(&mut self, value: #ty) -> &mut Self {
      self.#field_name = value;
      self
    }

    #[doc = #set_fn_doc]
    #[inline]
    pub #constable fn #with_fn(mut self, value: #ty) -> Self {
      self.#field_name = value;
      self
    }
  }
}

fn optional_accessors(field_name: &Ident, ty: &Type, copy: bool) -> proc_macro2::TokenStream {
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
    pub const fn #ref_fn(&self) -> ::core::option::Option<&#ty> {
      self.#field_name.as_ref()
    }

    #[doc = #ref_mut_fn_doc]
    #[inline]
    pub const fn #ref_mut_fn(&mut self) -> ::core::option::Option<&mut #ty> {
      self.#field_name.as_mut()
    }

    #[doc = #unwrap_ref_fn_doc]
    ///
    /// ## Panics
    ///
    #[doc = #panic_msg_doc]
    #[inline]
    pub const fn #unwrap_ref_fn(&self) -> &#ty {
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
    pub const fn #unwrap_mut_fn(&mut self) -> &mut #ty {
      match self.#field_name.as_mut() {
        ::core::option::Option::Some(value) => value,
        ::core::option::Option::None => panic!(#panic_msg),
      }
    }

    #[doc = #take_fn_doc]
    #[inline]
    pub const fn #take_fn(&mut self) -> ::core::option::Option<#ty> {
      self.#field_name.take()
    }

    #[doc = #clear_fn_doc]
    #[inline]
    pub #constable fn #clear_fn(&mut self) -> &mut Self {
      self.#field_name = ::core::option::Option::None;
      self
    }

    #[doc = #set_fn_doc]
    #[inline]
    pub #constable fn #set_fn(&mut self, value: #ty) -> &mut Self {
      self.#field_name = ::core::option::Option::Some(value);
      self
    }

    #[doc = #update_fn_doc]
    #[inline]
    pub #constable fn #update_fn(&mut self, value: ::core::option::Option<#ty>) -> &mut Self {
      self.#field_name = value;
      self
    }

    #[doc = #set_fn_doc]
    #[inline]
    pub #constable fn #with_fn(mut self, value: #ty) -> Self {
      self.#field_name = ::core::option::Option::Some(value);
      self
    }

    #[doc = #clear_fn_doc]
    #[inline]
    pub #constable fn #without_fn(mut self) -> Self {
      self.#field_name = ::core::option::Option::None;
      self
    }

    #[doc = #update_fn_doc]
    #[inline]
    pub #constable fn #maybe_fn(mut self, value: ::core::option::Option<#ty>) -> Self {
      self.#field_name = value;
      self
    }
  }
}
