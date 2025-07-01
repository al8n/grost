use std::num::NonZeroU32;

use darling::usage::{IdentSet, LifetimeSet, Purpose, UsesLifetimes, UsesTypeParams};
use either::Either;
use quote::{ToTokens, quote};
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
  utils::{MissingOperation, SchemaOptions},
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
        extra: meta.extra,
      }))),
      FieldFromMeta::Tagged(field) => {
        let field = *field;
        let TaggedFieldFromMeta {
          label,
          schema,
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

    let mut partial_ref_constraints = Punctuated::new();
    let mut selector_constraints = Punctuated::new();

    let purpose = Purpose::Declare;
    let field_lifetimes_usages =
      field_ty.uses_lifetimes_cloned(&purpose.into(), &object.lifetimes_usages);
    let field_type_params_usages =
      field_ty.uses_type_params_cloned(&purpose.into(), &object.type_params_usages);
    let use_generics = !field_lifetimes_usages.is_empty() || !field_type_params_usages.is_empty();

    let wfr = syn::parse2(wire_format_reflection(
      path_to_grost,
      object_ty,
      flavor_type,
      tag,
    ))?;
    let wf = match field.wire_format {
      Some(wf) => wf,
      None => {
        let dwf = default_wire_format(path_to_grost, flavor_type);
        let format = default_wire_format_associated(path_to_grost, flavor_type, field_ty);
        if use_generics {
          let pred: WherePredicate = syn::parse2(quote! {
            #field_ty: #dwf
          })?;
          selector_constraints.push(pred.clone());
          partial_ref_constraints.extend([
            pred,
            syn::parse2(quote! {
              #format: #lifetime
            })?,
          ]);
        }

        syn::parse2(format)?
      }
    };

    let index = Index::new(index, &field.name, field.tag.get())?;
    let schema_name = field.schema.name.unwrap_or_else(|| field.name.to_string());
    let schema_description = field.schema.description.unwrap_or_default();
    let field_ty = field.ty;

    let partial = PartialField::try_new(
      object,
      use_generics,
      &field_ty,
      &wf,
      &field.label,
      field.partial,
    )?;

    let partial_ref = PartialRefField::try_new(
      object,
      use_generics,
      &field_ty,
      &wf,
      &field.label,
      field.partial_ref,
      partial_ref_constraints,
    )?;

    let selector = SelectorField::try_new(
      object,
      use_generics,
      &field_ty,
      field.selector,
      selector_constraints,
    )?;

    let reflection = FieldReflection::try_new(
      object,
      &field_ty,
      tag,
      &schema_name,
      &schema_description,
      use_generics,
    )?;

    Ok(Self {
      index,
      selector,
      reflection,
      partial,
      partial_ref,
      schema_description,
      schema_name,
      type_params_usages: field_type_params_usages,
      lifetimes_usages: field_lifetimes_usages,
      wire_format: wf,
      wire_format_reflection: wfr,
      transform: {
        let mo = field.transform.missing_operation().cloned().or_else(|| {
          object
            .transform
            .or_default_by_label(&field.label)
            .then_some(MissingOperation::OrDefault)
        });
        field.transform.missing_operation = mo;
        field.transform
      },
      name: field.name,
      vis: field.vis,
      label: field.label,
      tag: field.tag.get(),
      ty: field_ty,
      attrs: field.attrs,
      copy: field.copy,
      meta: field.extra,
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

fn wire_format_reflection(
  path_to_grost: &Path,
  object_type: impl ToTokens,
  flavor_type: impl ToTokens,
  tag: u32,
) -> proc_macro2::TokenStream {
  quote! {
    #path_to_grost::__private::reflection::WireFormatReflection<
      #object_type,
      #flavor_type,
      #tag,
    >
  }
}

fn selectable(path_to_grost: &Path, flavor_type: impl ToTokens) -> proc_macro2::TokenStream {
  quote! {
    #path_to_grost::__private::selection::Selectable<#flavor_type>
  }
}

fn selector(
  path_to_grost: &Path,
  flavor_type: impl ToTokens,
  field_type: impl ToTokens,
) -> proc_macro2::TokenStream {
  let selectable = selectable(path_to_grost, flavor_type);
  quote! {
    <#field_type as #selectable>::Selector
  }
}

fn default_wire_format(
  path_to_grost: &Path,
  flavor_type: impl ToTokens,
) -> proc_macro2::TokenStream {
  quote! {
    #path_to_grost::__private::flavors::DefaultWireFormat<#flavor_type>
  }
}

fn default_wire_format_associated(
  path_to_grost: &Path,
  flavor_type: impl ToTokens,
  field_type: impl ToTokens,
) -> proc_macro2::TokenStream {
  let dwf = default_wire_format(path_to_grost, flavor_type);
  quote! {
    <#field_type as #dwf>::Format
  }
}

fn applied_partial_ref_state(
  path_to_grost: &Path,
  lt: impl ToTokens,
  read_buffer: impl ToTokens,
  buffer: impl ToTokens,
  wf: impl ToTokens,
  flavor_type: impl ToTokens,
) -> proc_macro2::TokenStream {
  let ty = applied_partial_ref(path_to_grost, lt, read_buffer, buffer, wf, flavor_type);
  quote! {
    #path_to_grost::__private::convert::State<#ty>
  }
}

fn applied_partial_ref(
  path_to_grost: &Path,
  lt: impl ToTokens,
  read_buffer: impl ToTokens,
  buffer: impl ToTokens,
  wf: impl ToTokens,
  flavor_type: impl ToTokens,
) -> proc_macro2::TokenStream {
  quote! {
    #path_to_grost::__private::convert::PartialRef<
      #lt,
      #read_buffer,
      #buffer,
      #wf,
      #flavor_type,
    >
  }
}
