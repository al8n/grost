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
        FieldConvertOptions, FieldDecodeOptions, FieldEncodeOptions, Index, RawSkippedField,
        SelectorFieldOptions, SkippedField,
      },
    },
    meta::concrete::{
      FieldFromMeta, PartialFieldFromMeta, PartialRefFieldFromMeta, TaggedFieldFromMeta,
    },
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

impl PartialFieldFromMeta {
  /// Finalizes the partial field meta and returns the attribute
  pub fn finalize(self) -> darling::Result<PartialFieldOptions> {
    Ok(PartialFieldOptions {
      attrs: self.attrs,
      partial_transform: self.partial_transform.finalize()?,
      transform: self.transform.finalize()?,
    })
  }
}

#[derive(Debug, Clone)]
pub struct PartialFieldOptions {
  pub(in crate::object) attrs: Vec<Attribute>,
  pub(in crate::object) transform: FieldConvertOptions,
  pub(in crate::object) partial_transform: FieldConvertOptions,
}

impl PartialFieldOptions {
  /// Returns the attributes of the partial object field
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns the options for transforming the ref type to owned type.
  pub const fn transform(&self) -> &FieldConvertOptions {
    &self.transform
  }

  /// Returns the options for partially transforming the ref type to owned type.
  pub const fn partial_transform(&self) -> &FieldConvertOptions {
    &self.partial_transform
  }
}

impl PartialRefFieldFromMeta {
  fn finalize(self) -> darling::Result<PartialRefFieldOptions> {
    Ok(PartialRefFieldOptions {
      copy: self.copy,
      attrs: self.attrs,
      ty: self.ty,
      encode: self.encode.finalize()?,
      decode: self.decode.finalize()?,
    })
  }
}

#[derive(Debug, Clone)]
pub struct PartialRefFieldOptions {
  pub(in crate::object) copy: bool,
  pub(in crate::object) attrs: Vec<Attribute>,
  pub(in crate::object) ty: Option<Type>,
  pub(in crate::object) encode: FieldEncodeOptions,
  pub(in crate::object) decode: FieldDecodeOptions,
}

impl PartialRefFieldOptions {
  /// Returns the attributes of the partial reference object field
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns whether the partial reference object field is copyable
  pub const fn copy(&self) -> bool {
    self.copy
  }

  /// Returns the type of the partial decoded object field
  pub const fn ty(&self) -> Option<&Type> {
    self.ty.as_ref()
  }

  /// Returns the options for encoding a field.
  pub const fn encode(&self) -> &FieldEncodeOptions {
    &self.encode
  }

  /// Returns the options for decoding a field.
  pub const fn decode(&self) -> &FieldDecodeOptions {
    &self.decode
  }
}

#[derive(Debug, Clone)]
pub struct RawTaggedField<E = ()> {
  pub name: Ident,
  pub ty: Type,
  pub vis: Visibility,
  pub attrs: Vec<Attribute>,
  pub label: Label,
  pub schema: SchemaOptions,
  pub default: Option<Invokable>,
  pub tag: NonZeroU32,
  pub wire_format: Option<Type>,
  pub transform: FieldConvertOptions,
  pub partial: PartialFieldOptions,
  pub partial_ref: PartialRefFieldOptions,
  pub selector: SelectorFieldOptions,
  pub copy: bool,
  pub extra: E,
}

impl<E> RawTaggedField<E> {
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
  Skipped(RawSkippedField<SM>),
  Tagged(RawTaggedField<TM>),
}

impl<TM, SM> RawField<TM, SM> {
  /// Creates a new raw field
  pub fn new(
    name: Ident,
    ty: Type,
    vis: Visibility,
    attrs: Vec<Attribute>,
    meta: FieldFromMeta<TM, SM>,
  ) -> darling::Result<Self> {
    match meta {
      FieldFromMeta::Skipped(meta) => Ok(Self::Skipped(RawSkippedField {
        name,
        ty,
        vis,
        attrs,
        default: meta.default,
        extra: meta.extra,
      })),
      FieldFromMeta::Tagged(TaggedFieldFromMeta {
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
      }) => Ok(Self::Tagged(RawTaggedField {
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
      })),
    }
  }

  pub(crate) fn extract(self) -> (RawField, Either<TM, SM>) {
    match self {
      Self::Skipped(skipped) => {
        let (skipped, extra) = skipped.extract();
        (RawField::Skipped(skipped), Either::Right(extra))
      }
      Self::Tagged(tagged) => {
        let (tagged, extra) = tagged.extract();
        (RawField::Tagged(tagged), Either::Left(extra))
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

    let partial_ref_copyable = object.partial_ref.copy() || field.partial_ref.copy();
    let partial_ref_copy_contraint = if partial_ref_copyable {
      Some(quote! {
        + ::core::marker::Copy
      })
    } else {
      None
    };

    let (partial_ref_ty, decoded_state_type) = match field.partial_ref.ty() {
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
      .decode()
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
      RawField::Skipped(f) => f.try_into().map(|f| Self::Skipped(Box::new(f))),
      RawField::Tagged(f) => {
        TaggedField::from_raw(object, index, f).map(|t| Self::Tagged(Box::new(t)))
      }
    }
  }
}

// #[derive(Debug, Clone)]
// pub(in crate::object) struct TaggedField<M = ()> {
//   pub(in crate::object) attrs: Vec<Attribute>,
//   pub(in crate::object) vis: Visibility,
//   pub(in crate::object) name: Ident,
//   pub(in crate::object) ty: Type,
//   pub(in crate::object) schema_name: String,
//   pub(in crate::object) schema_description: String,
//   pub(in crate::object) wire_format: Option<Type>,
//   pub(in crate::object) transform: FieldConvertOptions,
//   pub(in crate::object) label: Label,
//   // pub(in crate::object) type_params_usages: IdentSet,
//   // pub(in crate::object) lifetimes_usages: LifetimeSet,
//   pub(in crate::object) partial_ref: PartialRefFieldOptions,
//   pub(in crate::object) partial: PartialFieldOptions,
//   pub(in crate::object) selector: SelectorFieldOptions,
//   pub(in crate::object) default: Invokable,
//   pub(in crate::object) tag: u32,
//   pub(in crate::object) copy: bool,
//   pub(in crate::object) meta: M,
// }

// impl<M> TaggedField<M> {
//   pub(in crate::object) fn try_new(
//     f: RawTaggedField<M>,
//   ) -> darling::Result<Self>
//   where
//     M: Clone,
//   {
//     let attrs = f.attrs;
//     let vis = f.vis;
//     let name = f.name;
//     let ty = f.ty;
//     let tag = f.tag.get();

//     let label = f.label;
//     let default = match f.default {
//       Some(path) => path,
//       None => syn::parse2::<Path>(quote! { ::core::default::Default::default }).map(Into::into)?,
//     };
//     let schema_name = f.schema.name.unwrap_or_else(|| name.to_string());
//     let schema_description = f.schema.description.unwrap_or_default();

//     Ok(Self {
//       attrs,
//       vis,
//       name,
//       schema_description,
//       schema_name,
//       type_params_usages: ty.uses_type_params_cloned(&Purpose::Declare.into(), type_params),
//       lifetimes_usages: ty.uses_lifetimes_cloned(&Purpose::Declare.into(), lifetime_params),
//       ty,
//       wire_format: f.wire_format,
//       transform: f.transform,
//       tag,
//       default,
//       label,
//       partial_ref: f.partial_ref,
//       partial: f.partial,
//       selector: f.selector,
//       copy: f.copy,
//       meta: f.extra,
//     })
//   }
// }
