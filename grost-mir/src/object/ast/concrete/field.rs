use std::num::NonZeroU32;

use darling::usage::{IdentSet, LifetimeSet, Purpose, UsesLifetimes, UsesTypeParams};
use either::Either;
use quote::quote;
use syn::{Attribute, Ident, Path, Type, Visibility, WherePredicate, punctuated::Punctuated};

use crate::{
  object::{
    Label,
    ast::{
      concrete::RawObject,
      field::{
        FieldConvertOptions, FieldDecodeOptions, FieldEncodeOptions, Index,
        PartialFieldConvertOptions, RawSkippedField, SelectorFieldOptions, SkippedField,
      },
    },
    meta::concrete::{FieldFromMeta, TaggedFieldFromMeta},
  },
  utils::{Invokable, MissingOperation, SchemaOptions, grost_decode_trait_lifetime},
};

pub use partial::*;
pub use partial_ref::*;
pub use reflection::*;
pub use selector::*;

mod partial;
mod partial_ref;
mod reflection;
mod selector;

#[derive(Debug, Clone)]
pub struct RawTaggedField<E = ()> {
  name: Ident,
  ty: Type,
  vis: Visibility,
  attrs: Vec<Attribute>,
  label: Label,
  schema: SchemaOptions,
  default: Option<Invokable>,
  tag: NonZeroU32,
  wire_format: Option<Type>,
  transform: FieldConvertOptions,
  partial: PartialFieldOptions,
  partial_ref: PartialRefFieldOptions,
  selector: SelectorFieldOptions,
  copy: bool,
  extra: E,
}

impl<E> RawTaggedField<E> {
  pub(super) const fn tag(&self) -> u32 {
    self.tag.get()
  }

  fn extract(self) -> (RawTaggedField, E) {
    let Self {
      name,
      ty,
      vis,
      attrs,
      label,
      schema,
      default,
      tag,
      wire_format,
      transform,
      partial,
      partial_ref,
      selector,
      copy,
      extra,
    } = self;
    (
      RawTaggedField {
        name,
        ty,
        vis,
        attrs,
        label,
        schema,
        default,
        tag,
        wire_format,
        transform,
        partial,
        partial_ref,
        selector,
        copy,
        extra: (),
      },
      extra,
    )
  }
}

#[derive(Debug, Clone, derive_more::IsVariant, derive_more::Unwrap, derive_more::TryUnwrap)]
#[unwrap(ref)]
#[try_unwrap(ref)]
pub enum RawField<TM = (), SM = ()> {
  Skipped(Box<RawSkippedField<SM>>),
  Tagged(Box<RawTaggedField<TM>>),
}

impl<TM, SM> RawField<TM, SM> {
  /// Creates a new raw field
  pub fn new(
    attrs: Vec<Attribute>,
    vis: Visibility,
    name: Ident,
    ty: Type,
    meta: FieldFromMeta<TM, SM>,
  ) -> darling::Result<Self> {
    match meta {
      FieldFromMeta::Skipped(meta) => Ok(Self::Skipped(Box::new(RawSkippedField {
        name,
        ty,
        vis,
        attrs,
        default: meta.default,
        extra: meta.extra,
      }))),
      FieldFromMeta::Tagged(field) => {
        let field = *field;
        let TaggedFieldFromMeta {
          label,
          schema,
          default,
          tag,
          transform,
          wire_format,
          partial,
          partial_ref,
          selector,
          copy,
          extra,
        } = field;

        Ok(Self::Tagged(Box::new(RawTaggedField {
          name,
          ty,
          vis,
          attrs,
          label,
          schema: schema.into(),
          default,
          tag,
          wire_format,
          transform: transform.finalize()?,
          partial: partial.finalize()?,
          partial_ref: partial_ref.finalize()?,
          selector: selector.finalize(),
          copy,
          extra,
        })))
      }
    }
  }

  pub(crate) fn extract(self) -> (RawField, Either<TM, SM>) {
    match self {
      Self::Skipped(skipped) => {
        let (skipped, extra) = skipped.extract();
        (RawField::Skipped(Box::new(skipped)), Either::Right(extra))
      }
      Self::Tagged(tagged) => {
        let (tagged, extra) = tagged.extract();
        (RawField::Tagged(Box::new(tagged)), Either::Left(extra))
      }
    }
  }
}

#[derive(Debug, Clone)]
pub struct TaggedField<T = ()> {
  name: Ident,
  vis: Visibility,
  ty: Type,
  label: Label,
  default: Invokable,
  attrs: Vec<Attribute>,
  wire_format: Type,
  wire_format_reflection: Type,
  type_params_usages: IdentSet,
  lifetimes_usages: LifetimeSet,
  partial: PartialField,
  partial_ref: PartialRefField,
  index: Index,
  reflection: FieldReflection,
  selector: SelectorField,
  schema_name: String,
  schema_description: String,
  transform: FieldConvertOptions,
  tag: u32,
  copy: bool,
  meta: T,
}

impl<T> TaggedField<T> {
  /// Returns the name of the field.
  #[inline]
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  /// Returns the visibility of the field.
  #[inline]
  pub const fn vis(&self) -> &Visibility {
    &self.vis
  }

  /// Returns the type of the field.
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the index of the field.
  #[inline]
  pub const fn index(&self) -> &Index {
    &self.index
  }

  /// Returns the schema name of the field.
  #[inline]
  pub const fn schema_name(&self) -> &str {
    self.schema_name.as_str()
  }

  /// Returns the schema description of the field.
  #[inline]
  pub const fn schema_description(&self) -> &str {
    self.schema_description.as_str()
  }

  /// Returns the attributes of the field.
  #[inline]
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns the default value of the field.
  #[inline]
  pub const fn default(&self) -> &Invokable {
    &self.default
  }

  /// Returns the tag of the field.
  #[inline]
  pub const fn tag(&self) -> u32 {
    self.tag
  }

  /// Returns the label of the field.
  #[inline]
  pub const fn label(&self) -> &Label {
    &self.label
  }

  /// Returns `true` if the field is copyable, `false` otherwise.
  #[inline]
  pub const fn copy(&self) -> bool {
    self.copy
  }

  /// Returns the wire format reflection type of the field.
  #[inline]
  pub const fn wire_format_reflection(&self) -> &Type {
    &self.wire_format_reflection
  }

  /// Returns the wire format type of the field.
  #[inline]
  pub const fn wire_format(&self) -> &Type {
    &self.wire_format
  }

  /// Returns the partial field information.
  #[inline]
  pub const fn partial(&self) -> &PartialField {
    &self.partial
  }

  /// Returns the partial decoded field information.
  #[inline]
  pub const fn partial_ref(&self) -> &PartialRefField {
    &self.partial_ref
  }

  /// Returns the selector field information
  #[inline]
  pub const fn selector(&self) -> &SelectorField {
    &self.selector
  }

  /// Returns the reflection information of the field.
  #[inline]
  pub const fn reflection(&self) -> &FieldReflection {
    &self.reflection
  }

  /// Returns the transformation options for the field.
  #[inline]
  pub const fn transform(&self) -> &FieldConvertOptions {
    &self.transform
  }

  /// Returns the custom metadata associated with the field.
  #[inline]
  pub const fn meta(&self) -> &T {
    &self.meta
  }

  /// Returns the usages of type parameters in the field type.
  #[inline]
  pub const fn type_params_usages(&self) -> &IdentSet {
    &self.type_params_usages
  }

  /// Returns the usages of lifetime parameters in the field type.
  #[inline]
  pub const fn lifetimes_usages(&self) -> &LifetimeSet {
    &self.lifetimes_usages
  }

  /// Returns `true` if the field uses generics, `false` otherwise.
  #[inline]
  pub fn uses_generics(&self) -> bool {
    !self.type_params_usages.is_empty() || !self.lifetimes_usages.is_empty()
  }
}

impl TaggedField {
  pub(super) fn with_meta<M>(self, meta: M) -> TaggedField<M> {
    TaggedField {
      name: self.name,
      vis: self.vis,
      ty: self.ty,
      label: self.label,
      default: self.default,
      attrs: self.attrs,
      wire_format: self.wire_format,
      wire_format_reflection: self.wire_format_reflection,
      type_params_usages: self.type_params_usages,
      lifetimes_usages: self.lifetimes_usages,
      partial: self.partial,
      partial_ref: self.partial_ref,
      index: self.index,
      reflection: self.reflection,
      selector: self.selector,
      schema_name: self.schema_name,
      schema_description: self.schema_description,
      transform: self.transform,
      tag: self.tag,
      copy: self.copy,
      meta,
    }
  }

  fn from_raw(
    object: &RawObject,
    index: usize,
    mut field: RawTaggedField,
  ) -> darling::Result<Self> {
    let path_to_grost = &object.path_to_grost;
    let field_ty = &field.ty;
    let flavor_type = &object.flavor_type;
    let tag = field.tag.get();
    let object_ty = &object.ty;
    let lifetime_param = &object.lifetime_param;
    let lifetime = &lifetime_param.lifetime;
    let unknown_buffer_param = &object.unknown_buffer_param;
    let unknown_buffer = &unknown_buffer_param.ident;
    let read_buffer_param = &object.read_buffer_param;
    let read_buffer = &read_buffer_param.ident;

    let mut partial_ref_constraints = Punctuated::new();
    let mut selector_constraints = Punctuated::new();

    let purpose = Purpose::Declare;
    let field_lifetimes_usages =
      field_ty.uses_lifetimes_cloned(&purpose.into(), &object.lifetimes_usages);
    let field_type_params_usages =
      field_ty.uses_type_params_cloned(&purpose.into(), &object.type_params_usages);
    let use_generics = !field_lifetimes_usages.is_empty() || !field_type_params_usages.is_empty();

    let wfr = syn::parse2(quote! {
      #path_to_grost::__private::reflection::WireFormatReflection<
        #object_ty,
        #flavor_type,
        #tag,
      >
    })?;
    let wf = match field.wire_format {
      Some(wf) => wf,
      None => {
        let dwf = quote! {
          #path_to_grost::__private::flavors::DefaultWireFormat<#flavor_type>
        };

        if use_generics {
          let pred: WherePredicate = syn::parse2(quote! {
            #field_ty: #dwf
          })?;
          selector_constraints.push(pred.clone());
          partial_ref_constraints.push(pred);
        }

        syn::parse2(quote! {
          <#field_ty as #dwf>::Format
        })?
      }
    };

    let selectable = syn::parse2(quote! {
      #path_to_grost::__private::selection::Selectable<
        #flavor_type,
      >
    })?;
    let selector_type = syn::parse2(quote! {
      <#field_ty as #selectable>::Selector
    })?;

    if use_generics {
      selector_constraints.push(syn::parse2(quote! {
        #field_ty: #selectable
      })?);
    }

    let partial_ref_copyable = object.partial_ref.copy() || field.partial_ref.copy;
    let partial_ref_copy_contraint = if partial_ref_copyable {
      Some(quote! {
        + ::core::marker::Copy
      })
    } else {
      None
    };

    let (partial_ref_ty, decoded_state_type) = match &field.partial_ref.ty {
      Some(ty) => (ty.clone(), None),
      None => {
        let state_type: Type = syn::parse2(quote! {
          #path_to_grost::__private::convert::State<
            #path_to_grost::__private::convert::Decoded<
              #lifetime,
              #flavor_type,
              #wf,
              #read_buffer,
              #unknown_buffer,
            >
          >
        })?;

        if use_generics {
          partial_ref_constraints.push(syn::parse2(quote! {
            #field_ty: #state_type
          })?);
          partial_ref_constraints.push(syn::parse2(quote! {
            <#field_ty as #state_type>::Output: ::core::marker::Sized #partial_ref_copy_contraint
          })?);
        }

        (
          syn::parse2(quote! {
            <#field_ty as #state_type>::Output
          })?,
          Some(state_type),
        )
      }
    };

    let flavor_ty = &object.flavor_type;
    let decode_lt = grost_decode_trait_lifetime();
    let decode_trait_type = syn::parse2(quote! {
      #path_to_grost::__private::Decode<#decode_lt, #flavor_ty, #wf, #partial_ref_ty, #read_buffer, #unknown_buffer>
    })?;

    let optional_partial_ref_type = syn::parse2(quote! {
      ::core::option::Option<#partial_ref_ty>
    })?;

    let index = Index::new(index, &field.name, field.tag.get())?;
    let schema_name = field.schema.name.unwrap_or_else(|| field.name.to_string());
    let schema_description = field.schema.description.unwrap_or_default();
    let field_ty = field.ty;
    let partial = PartialField::from_options(object, &field_ty, field.partial, &field.label)?;
    let decode_missing_operation = field
      .partial_ref
      .decode
      .missing_operation()
      .cloned()
      .or_else(|| {
        object
          .partial_ref
          .decode
          .or_default_by_label(&field.label)
          .then_some(MissingOperation::OrDefault)
      });
    let transform_missing_operation = field.transform.missing_operation().cloned().or_else(|| {
      object
        .transform
        .or_default_by_label(&field.label)
        .then_some(MissingOperation::OrDefault)
    });

    let partial_ref = PartialRefField {
      ty: partial_ref_ty,
      optional_type: optional_partial_ref_type,
      decoded_state_type,
      decode_trait_type,
      attrs: field.partial_ref.attrs,
      constraints: partial_ref_constraints,
      copy: partial_ref_copyable,
      encode: field.partial_ref.encode,
      decode: {
        field.partial_ref.decode.missing_operation = decode_missing_operation;
        field.partial_ref.decode
      },
    };
    let selector = SelectorField {
      ty: selector_type,
      selectable,
      attrs: field.selector.attrs,
      constraints: selector_constraints,
      default: field.selector.select,
    };
    let reflection = FieldReflection::try_new(
      object,
      &field_ty,
      tag,
      &schema_name,
      &schema_description,
      use_generics,
    )?;

    Ok(Self {
      partial,
      partial_ref,
      name: field.name,
      vis: field.vis,
      label: field.label,
      tag: field.tag.get(),
      ty: field_ty,
      attrs: field.attrs,
      default: match field.default {
        Some(path) => path,
        None => {
          syn::parse2::<Path>(quote! { ::core::default::Default::default }).map(Into::into)?
        }
      },
      schema_description,
      schema_name,
      type_params_usages: field_type_params_usages,
      lifetimes_usages: field_lifetimes_usages,
      copy: field.copy,
      meta: field.extra,
      wire_format: wf,
      wire_format_reflection: wfr,
      transform: {
        field.transform.missing_operation = transform_missing_operation;
        field.transform
      },
      index,
      selector,
      reflection,
    })
  }
}

#[derive(Debug, Clone, derive_more::IsVariant, derive_more::Unwrap, derive_more::TryUnwrap)]
#[unwrap(ref)]
#[try_unwrap(ref)]
pub enum Field<T = (), S = ()> {
  Skipped(Box<SkippedField<S>>),
  Tagged(Box<TaggedField<T>>),
}

impl<T, S> Field<T, S> {
  #[inline]
  pub const fn name(&self) -> &Ident {
    match self {
      Self::Skipped(skipped) => skipped.name(),
      Self::Tagged(tagged) => tagged.name(),
    }
  }

  #[inline]
  pub const fn ty(&self) -> &Type {
    match self {
      Self::Skipped(skipped) => skipped.ty(),
      Self::Tagged(tagged) => tagged.ty(),
    }
  }

  #[inline]
  pub const fn attrs(&self) -> &[Attribute] {
    match self {
      Self::Skipped(skipped) => skipped.attrs(),
      Self::Tagged(tagged) => tagged.attrs(),
    }
  }

  #[inline]
  pub const fn vis(&self) -> &syn::Visibility {
    match self {
      Self::Skipped(skipped) => skipped.vis(),
      Self::Tagged(tagged) => tagged.vis(),
    }
  }

  #[inline]
  pub const fn default(&self) -> &Invokable {
    match self {
      Self::Skipped(skipped) => skipped.default(),
      Self::Tagged(tagged) => tagged.default(),
    }
  }
}

impl Field {
  pub(super) fn from_raw(
    object: &RawObject,
    index: usize,
    field: RawField,
  ) -> darling::Result<Self> {
    match field {
      RawField::Skipped(f) => {
        let purpose = Purpose::Declare;
        let lifetime_usages = f
          .ty
          .uses_lifetimes_cloned(&purpose.into(), &object.lifetimes_usages);
        let type_params_usages = f
          .ty
          .uses_type_params_cloned(&purpose.into(), &object.type_params_usages);
        SkippedField::from_raw(*f, lifetime_usages, type_params_usages)
          .map(|f| Self::Skipped(Box::new(f)))
      }
      RawField::Tagged(f) => {
        TaggedField::from_raw(object, index, *f).map(|t| Self::Tagged(Box::new(t)))
      }
    }
  }
}
