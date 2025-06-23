use std::rc::Rc;

use darling::usage::{GenericsExt, IdentSet, LifetimeSet};
use either::Either;
use indexmap::IndexSet;
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{Attribute, Generics, Ident, LifetimeParam, Path, Type, TypeParam, Visibility};

use crate::{
  flavor::{DecodeOptions, IdentifierOptions, TagOptions},
  object::{
    ast::{
      Indexer, ObjectConvertOptions, accessors,
      concrete::{
        reflection::Reflection,
        selector::{Selector, SelectorIter},
      },
      indexer::IndexerOptions,
      selector::{SelectorIterOptions, SelectorOptions},
    },
    meta::{
      ObjectFromMeta,
      concrete::{ObjectFlavorFromMeta, PartialRefObjectFromMeta},
    },
  },
  utils::{Invokable, SchemaOptions},
};

use field::*;

pub use partial::*;
pub use partial_ref::*;

mod decode;
mod field;
mod indexer;
mod partial;
mod partial_ref;
mod reflection;
mod selector;

impl PartialRefObjectFromMeta {
  pub(super) fn finalize(self) -> PartialRefObjectOptions {
    PartialRefObjectOptions {
      name: self.name,
      attrs: self.attrs,
      copy: self.copy,
      decode: self.decode.into(),
    }
  }
}

#[derive(Debug, Clone)]
pub struct PartialRefObjectOptions {
  name: Option<Ident>,
  attrs: Vec<Attribute>,
  copy: bool,
  decode: DecodeOptions,
}

impl PartialRefObjectOptions {
  /// Returns the name of the partial reference object
  pub(crate) const fn name(&self) -> Option<&Ident> {
    self.name.as_ref()
  }

  /// Returns the attributes of the partial reference object
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns whether the partial reference object is copyable
  pub const fn copy(&self) -> bool {
    self.copy
  }
}

/// The AST for a concrete object, a concrete object which means there is only one flavor and the generated code will not be generic
/// over the flavor type.
#[derive(Debug, Clone)]
struct RawObject<T = (), S = (), O = ()> {
  name: Ident,
  vis: Visibility,
  generics: Generics,
  ty: Type,
  reflectable: Type,
  lifetimes_usages: LifetimeSet,
  type_params_usages: IdentSet,
  attrs: Vec<Attribute>,
  fields: Vec<RawField<T, S>>,
  path_to_grost: Path,
  flavor_type: Type,
  wire_format: Type,
  tag_options: TagOptions,
  identifier_options: IdentifierOptions,
  default: Option<Invokable>,
  schema: SchemaOptions,
  transform: ObjectConvertOptions,
  partial: PartialObjectOptions,
  partial_ref: PartialRefObjectOptions,
  selector: SelectorOptions,
  selector_iter: SelectorIterOptions,
  indexer: IndexerOptions,
  copy: bool,
  unknown_buffer_param: TypeParam,
  lifetime_param: LifetimeParam,
  read_buffer_param: TypeParam,
  write_buffer_param: TypeParam,
  extra: O,
}

impl<T, S, O> RawObject<T, S, O> {
  /// Creates a new `RawObject` from the given parameters.
  fn new(
    path_to_grost: Path,
    name: Ident,
    vis: Visibility,
    generics: Generics,
    attrs: Vec<Attribute>,
    mut fields: Vec<RawField<T, S>>,
    meta: ObjectFromMeta<O>,
  ) -> darling::Result<Self> {
    let ObjectFlavorFromMeta {
      ty: flavor_type,
      wire_format,
      tag,
      identifier,
    } = match meta.flavor {
      Some(meta) => meta,
      None => ObjectFlavorFromMeta::network(&path_to_grost)?,
    };

    fields.sort_by_key(|a| match a {
      RawField::Tagged(f) => f.tag(),
      RawField::Skipped(_) => u32::MAX, // Skipped fields come last
    });

    let mut tags = IndexSet::new();
    fields
      .iter()
      .filter_map(|f| f.try_unwrap_tagged_ref().ok())
      .try_for_each(|f| {
        let tag = f.tag();
        if tags.contains(&tag) {
          Err(darling::Error::custom(format!(
            "{name} has multiple fields have the same tag {tag}",
          )))
        } else {
          tags.insert(tag);
          Ok(())
        }
      })?;

    if tags.is_empty() {
      return Err(darling::Error::custom(format!(
        "{name} must have at least one tagged field"
      )));
    }

    let (_, tg, _) = generics.split_for_impl();
    let ty: Type = syn::parse2(quote! {
      #name #tg
    })?;
    let reflectable: Type = syn::parse2(quote! {
      #path_to_grost::__private::reflection::Reflectable<#ty>
    })?;

    Ok(Self {
      name,
      vis,
      ty,
      reflectable,
      lifetimes_usages: generics.declared_lifetimes(),
      type_params_usages: generics.declared_type_params(),
      generics,
      attrs,
      fields,
      path_to_grost,
      default: meta.default,
      schema: meta.schema.into(),
      partial: meta.partial.finalize(),
      partial_ref: meta.partial_ref.finalize(),
      selector: meta.selector.finalize(),
      selector_iter: meta.selector_iter.finalize(),
      indexer: meta.indexer.into(),
      transform: meta.transform.into(),
      copy: meta.copy,
      unknown_buffer_param: meta.generic.unknown_buffer,
      lifetime_param: meta.generic.lifetime,
      read_buffer_param: meta.generic.read_buffer,
      write_buffer_param: meta.generic.write_buffer,
      extra: meta.extra,
      flavor_type,
      wire_format,
      tag_options: tag.into(),
      identifier_options: identifier.into(),
    })
  }

  #[inline]
  pub fn partial_ref_name(&self) -> Ident {
    self
      .partial_ref
      .name()
      .cloned()
      .unwrap_or_else(|| format_ident!("PartialRef{}", self.name))
  }

  #[inline]
  pub fn partial_name(&self) -> Ident {
    self
      .partial
      .name()
      .cloned()
      .unwrap_or_else(|| format_ident!("Partial{}", self.name))
  }

  #[inline]
  pub fn selector_name(&self) -> Ident {
    self
      .selector
      .name()
      .cloned()
      .unwrap_or_else(|| format_ident!("{}Selector", self.name))
  }

  #[inline]
  pub fn selector_iter_name(&self) -> Ident {
    self
      .selector_iter
      .name
      .clone()
      .unwrap_or_else(|| format_ident!("{}Iter", self.selector_name()))
  }

  #[inline]
  pub fn indexer_name(&self) -> Ident {
    self
      .indexer
      .name()
      .cloned()
      .unwrap_or_else(|| format_ident!("{}Index", self.name))
  }

  fn extract(self) -> (RawObject, (Vec<Either<T, S>>, O)) {
    let Self {
      name,
      vis,
      generics,
      ty,
      reflectable,
      lifetimes_usages,
      type_params_usages,
      attrs,
      fields,
      path_to_grost,
      flavor_type,
      wire_format,
      tag_options,
      identifier_options,
      default,
      schema,
      transform,
      partial,
      partial_ref,
      selector,
      selector_iter,
      indexer,
      copy,
      unknown_buffer_param,
      lifetime_param,
      read_buffer_param,
      write_buffer_param,
      extra,
    } = self;

    let mut fields_metas = Vec::with_capacity(fields.len());
    let mut extracted_fields = Vec::with_capacity(fields.len());
    for f in fields {
      let (field, extra) = f.extract();
      fields_metas.push(extra);
      extracted_fields.push(field);
    }

    (
      RawObject {
        name,
        vis,
        generics,
        ty,
        reflectable,
        lifetimes_usages,
        type_params_usages,
        attrs,
        fields: extracted_fields,
        path_to_grost,
        flavor_type,
        wire_format,
        tag_options,
        identifier_options,
        default,
        schema,
        transform,
        partial,
        partial_ref,
        selector,
        selector_iter,
        indexer,
        copy,
        unknown_buffer_param,
        lifetime_param,
        read_buffer_param,
        write_buffer_param,
        extra: (),
      },
      (fields_metas, extra),
    )
  }
}

/// The AST for a concrete object, a concrete object which means there is only one flavor and the generated code will not be generic
/// over the flavor type.
#[derive(derive_more::Debug, Clone)]
pub struct Object<T = (), S = (), M = ()> {
  path_to_grost: Path,
  attrs: Vec<Attribute>,
  name: Ident,
  schema_name: String,
  schema_description: String,
  vis: Visibility,
  ty: Type,
  copy: bool,
  decoded_state_type: Type,
  reflectable: Type,
  generics: Generics,
  /// The trait type which applies the cooresponding generics to the `Decode` trait.
  #[debug(skip)]
  applied_decode_trait: Rc<dyn Fn(TokenStream) -> syn::Result<Type> + 'static>,
  flavor_type: Type,
  wire_format: Type,
  tag_options: TagOptions,
  identifier_options: IdentifierOptions,
  unknown_buffer_param: TypeParam,
  lifetime_param: LifetimeParam,
  read_buffer_param: TypeParam,
  write_buffer_param: TypeParam,
  fields: Vec<Field<T, S>>,
  indexer: Indexer,
  default: Option<Invokable>,
  partial: PartialObject,
  partial_ref: PartialRefObject,
  selector: Selector,
  selector_iter: SelectorIter,
  reflection: Reflection,
  type_params_usages: IdentSet,
  lifetimes_usages: LifetimeSet,
  meta: M,
}

impl<T, S, M> Object<T, S, M> {
  /// Returns the path to the `grost` crate
  #[inline]
  pub const fn path_to_grost(&self) -> &Path {
    &self.path_to_grost
  }

  /// Returns the name of the object
  #[inline]
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  /// Returns the schema name of the object
  #[inline]
  pub const fn schema_name(&self) -> &str {
    self.schema_name.as_str()
  }

  /// Returns the schema description of the object
  #[inline]
  pub const fn schema_description(&self) -> &str {
    self.schema_description.as_str()
  }

  /// Returns the visibility of the object
  #[inline]
  pub const fn vis(&self) -> &Visibility {
    &self.vis
  }

  /// Returns the type of the object
  ///
  /// e.g. If a struct is `struct MyObject<T> { ... }`, this will return `MyObject<T>`.
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the reflectable trait which replaces the generic parameter with the type of the object
  /// e.g. If a struct is `struct MyObject<T> { ... }`, this will return `Reflectable<MyObject<T>>`.
  #[inline]
  pub const fn reflectable(&self) -> &Type {
    &self.reflectable
  }

  /// Returns the flavor
  #[inline]
  pub const fn flavor_type(&self) -> &Type {
    &self.flavor_type
  }

  /// Returns the wire format type
  #[inline]
  pub const fn wire_format(&self) -> &Type {
    &self.wire_format
  }

  /// Returns the identifier options for the object
  #[inline]
  pub const fn identifier_options(&self) -> &IdentifierOptions {
    &self.identifier_options
  }

  /// Returns the tag options for the object
  #[inline]
  pub const fn tag_options(&self) -> &TagOptions {
    &self.tag_options
  }

  /// Returns the generic unknown buffer type parameter.
  #[inline]
  pub const fn unknown_buffer_param(&self) -> &TypeParam {
    &self.unknown_buffer_param
  }

  /// Returns the generic read buffer type parameter.
  #[inline]
  pub const fn read_buffer_param(&self) -> &TypeParam {
    &self.read_buffer_param
  }

  /// Returns the generic write buffer type parameter.
  #[inline]
  pub const fn write_buffer_param(&self) -> &TypeParam {
    &self.write_buffer_param
  }

  /// Returns the generic lifetime parameter.
  #[inline]
  pub const fn lifetime_param(&self) -> &LifetimeParam {
    &self.lifetime_param
  }

  /// Returns the generics in the object definition if any.
  #[inline]
  pub const fn generics(&self) -> &Generics {
    &self.generics
  }

  /// Returns the attributes in the object definition.
  #[inline]
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns `true` if the object is copyable, `false` otherwise.
  #[inline]
  pub const fn copy(&self) -> bool {
    self.copy
  }

  /// Returns the decoded state type of the object.
  #[inline]
  pub const fn meta(&self) -> &M {
    &self.meta
  }

  /// Returns the fields of the object
  #[inline]
  pub const fn fields(&self) -> &[Field<T, S>] {
    self.fields.as_slice()
  }

  /// Returns the partial object information
  #[inline]
  pub const fn partial(&self) -> &PartialObject {
    &self.partial
  }

  /// Returns the partial decoded object information
  #[inline]
  pub const fn partial_ref(&self) -> &PartialRefObject {
    &self.partial_ref
  }

  /// Returns the reflection information for the object
  #[inline]
  pub const fn reflection(&self) -> &Reflection {
    &self.reflection
  }

  /// Returns the indexer information
  #[inline]
  pub const fn indexer(&self) -> &Indexer {
    &self.indexer
  }

  /// Returns the type parameters usages in the object definition.
  #[inline]
  pub const fn type_params_usages(&self) -> &IdentSet {
    &self.type_params_usages
  }

  /// Returns the lifetimes usages in the object definition.
  #[inline]
  pub const fn lifetimes_usages(&self) -> &LifetimeSet {
    &self.lifetimes_usages
  }

  /// Returns the selector information
  #[inline]
  pub const fn selector(&self) -> &Selector {
    &self.selector
  }

  /// Returns the selector iterator information
  #[inline]
  pub const fn selector_iter(&self) -> &SelectorIter {
    &self.selector_iter
  }

  /// Creates a new `Object` from the given parameters.
  pub fn new(
    path_to_grost: Path,
    name: Ident,
    vis: Visibility,
    generics: Generics,
    attrs: Vec<Attribute>,
    fields: Vec<RawField<T, S>>,
    meta: ObjectFromMeta<M>,
  ) -> darling::Result<Self> {
    let object = RawObject::new(path_to_grost, name, vis, generics, attrs, fields, meta)?;

    let (mut object, (fields_metas, extra)) = object.extract();
    let generics = &object.generics;
    let (_, tg, _) = generics.split_for_impl();

    let ty: Type = {
      let name = &object.name;
      syn::parse2(quote! {
        #name #tg
      })?
    };
    let reflectable: Type = {
      let path_to_grost = &object.path_to_grost;
      syn::parse2(quote! {
        #path_to_grost::__private::reflection::Reflectable<#ty>
      })?
    };

    let fields = object
      .fields
      .iter()
      .cloned()
      .enumerate()
      .map(|(idx, f)| Field::from_raw(&object, idx, f))
      .collect::<darling::Result<Vec<_>>>()?;

    let indexer_name = object.indexer_name();
    let selector_name = object.selector_name();
    let selector_iter_name = object.selector_iter_name();

    let reflection = Reflection::from_ast(&object, &fields)?;
    let partial_ref = PartialRefObject::from_options(&mut object, &fields)?;
    let partial = PartialObject::from_options(&mut object, &fields)?;

    let selector =
      Selector::from_options(&object.generics, selector_name, object.selector, &fields)?;
    let selector_iter = selector.selector_iter(selector_iter_name, object.selector_iter)?;

    let path_to_grost = object.path_to_grost;
    let flavor_type = object.flavor_type;
    let type_params_usages = object.type_params_usages;
    let lifetimes_usages = object.lifetimes_usages;
    let wf = object.wire_format;
    let lp = object.lifetime_param;
    let lt = &lp.lifetime;
    let ubp = object.unknown_buffer_param;
    let ub = &ubp.ident;
    let rbp = object.read_buffer_param;
    let rb = &rbp.ident;

    let decoded_state_type = syn::parse2(quote! {
      #path_to_grost::__private::convert::Decoded<#lt, #flavor_type, #wf, #rb, #ub>
    })?;

    let applied_decode_trait = {
      let path_to_grost = path_to_grost.clone();
      let flavor_ty = flavor_type.clone();
      let wf = wf.clone();
      let lt = lt.clone();
      let ub = ub.clone();
      let rb = rb.clone();
      Rc::new(move |ty| {
        syn::parse2(quote! {
          #path_to_grost::__private::Decode<#lt, #flavor_ty, #wf, #ty, #rb, #ub>
        })
      })
    };
    Ok(Self {
      path_to_grost,
      copy: object.copy,
      attrs: object.attrs,
      indexer: Indexer {
        name: indexer_name,
        attrs: object.indexer.attrs,
      },
      schema_name: object
        .schema
        .name
        .unwrap_or_else(|| object.name.to_string()),
      schema_description: object.schema.description.unwrap_or_default(),
      name: object.name,
      vis: object.vis,
      ty: object.ty,
      reflectable,
      generics: object.generics,
      fields: fields
        .into_iter()
        .zip(fields_metas)
        .map(|(f, extra)| match (f, extra) {
          (Field::Skipped(f), Either::Right(meta)) => Field::Skipped(Box::new(f.with_meta(meta))),
          (Field::Tagged(f), Either::Left(meta)) => Field::Tagged(Box::new(f.with_meta(meta))),
          _ => unreachable!("Field and meta mismatch"),
        })
        .collect(),
      default: object.default,
      partial,
      partial_ref,
      selector,
      selector_iter,
      meta: extra,
      unknown_buffer_param: ubp,
      lifetime_param: lp,
      read_buffer_param: rbp,
      write_buffer_param: object.write_buffer_param,
      decoded_state_type,
      applied_decode_trait,
      flavor_type,
      wire_format: wf,
      tag_options: object.tag_options,
      identifier_options: object.identifier_options,
      reflection,
      type_params_usages,
      lifetimes_usages,
    })
  }

  /// Returns the default value of the concrete object, if any.
  #[inline]
  pub fn derive(&self) -> darling::Result<proc_macro2::TokenStream> {
    let default = self.derive_default()?;
    let accessors = self.derive_accessors()?;

    let indexer_defination = self.derive_indexer_defination();
    let indexer_impl = self.derive_indexer();

    let partial_def = self.partial.derive_defination(self)?;
    let partial_impl = self.partial.derive(self)?;

    let partial_ref_def = self.derive_partial_ref_object_defination();
    let partial_ref_impl = self.derive_partial_ref_object();

    let reflection_impl = self.reflection.derive(self)?;

    let selector = self.derive_selector_defination();
    let selector_impl = self.derive_selector()?;

    let selector_iter_def = self.derive_selector_iter_defination();
    let selector_iter_impl = self.derive_selector_iter();

    let decode_impl = self.derive_decode()?;

    Ok(quote! {
      #indexer_defination
      #selector
      #selector_iter_def
      #partial_def
      #partial_ref_def

      const _: () = {
        #default

        #accessors

        #partial_impl

        #partial_ref_impl

        #reflection_impl

        #indexer_impl

        #selector_impl

        #selector_iter_impl

        #decode_impl
      };
    })
  }

  fn applied_decode_trait(&self, ty: impl ToTokens) -> syn::Result<Type> {
    (self.applied_decode_trait)(quote! { #ty })
  }

  fn derive_accessors(&self) -> darling::Result<proc_macro2::TokenStream> {
    let name = self.name();
    let (ig, tg, wc) = self.generics().split_for_impl();

    let accessors = self
      .fields()
      .iter()
      .filter_map(|f| f.try_unwrap_tagged_ref().ok())
      .map(|f| {
        let field_name = f.name();
        let ty = f.ty();
        let copy = f.copy();

        accessors(field_name, f.vis(), ty, copy)
      });

    Ok(quote! {
      impl #ig #name #tg #wc {
        #(#accessors)*
      }
    })
  }

  fn derive_default(&self) -> darling::Result<proc_macro2::TokenStream> {
    let name = self.name();
    let (ig, tg, wc) = self.generics().split_for_impl();

    if let Some(default) = &self.default {
      Ok(quote! {
        impl #ig ::core::default::Default for #name #tg #wc {
          fn default() -> Self {
            Self::new()
          }
        }

        impl #ig ::core::default::Default for #name #tg #wc {
          /// Creates a new instance of the object with default values.
          pub fn new() -> Self {
            (#default)()
          }
        }
      })
    } else {
      let fields = self.fields().iter().map(|f| {
        let name = f.name();
        let default = f.default();
        quote! {
          #name: (#default)()
        }
      });

      Ok(quote! {
        impl #ig ::core::default::Default for #name #tg #wc {
          fn default() -> Self {
            Self::new()
          }
        }

        impl #ig #name #tg #wc {
          /// Creates a new instance of the object with default values.
          pub fn new() -> Self {
            Self {
              #(#fields),*
            }
          }
        }
      })
    }
  }
}
