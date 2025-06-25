use darling::usage::{IdentSet, LifetimeSet};
use syn::{Attribute, Ident, Type, Visibility, WherePredicate, punctuated::Punctuated};

use quote::quote;

use crate::{
  object::{
    ConvertAttribute, FieldIndex, Label,
    ast::{FieldDecodeFlavor, FieldEncodeFlavor},
  },
  utils::{Invokable, grost_decode_trait_lifetime},
};

use super::{
  super::super::ast::{
    ConcreteField as ConcreteFieldAst, ConcreteTaggedField as ConcreteTaggedFieldAst, SkippedField,
  },
  ConcreteObjectAst,
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
pub struct ConcreteTaggedField<F = ()> {
  name: Ident,
  vis: Visibility,
  ty: Type,
  label: Label,
  default: Invokable,
  attrs: Vec<Attribute>,
  wire_format: Type,
  wire_format_reflection: Type,
  type_params_usages: IdentSet,
  lifetime_params_usages: LifetimeSet,
  partial: ConcretePartialField,
  partial_ref: ConcretePartialRefField,
  index: FieldIndex,
  reflection: ConcreteFieldReflection,
  selector: ConcreteSelectorField,
  schema_name: String,
  schema_description: String,
  encode: FieldEncodeFlavor,
  decode: FieldDecodeFlavor,
  tag: u32,
  copy: bool,
  meta: F,
}

impl<F> ConcreteTaggedField<F> {
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
  pub const fn index(&self) -> &FieldIndex {
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
  pub const fn partial(&self) -> &ConcretePartialField {
    &self.partial
  }

  /// Returns the partial decoded field information.
  #[inline]
  pub const fn partial_ref(&self) -> &ConcretePartialRefField {
    &self.partial_ref
  }

  /// Returns the selector field information
  #[inline]
  pub const fn selector(&self) -> &ConcreteSelectorField {
    &self.selector
  }

  /// Returns the reflection information of the field.
  #[inline]
  pub const fn reflection(&self) -> &ConcreteFieldReflection {
    &self.reflection
  }

  /// Returns the custom metadata associated with the field.
  #[inline]
  pub const fn meta(&self) -> &F {
    &self.meta
  }

  /// Returns the usages of type parameters in the field type.
  #[inline]
  pub const fn type_params_usages(&self) -> &IdentSet {
    &self.type_params_usages
  }

  /// Returns the usages of lifetime parameters in the field type.
  #[inline]
  pub const fn lifetime_params_usages(&self) -> &LifetimeSet {
    &self.lifetime_params_usages
  }

  /// Returns the encode flavor of the field.
  #[inline]
  pub const fn encode(&self) -> &FieldEncodeFlavor {
    &self.encode
  }

  /// Returns the decode flavor of the field.
  #[inline]
  pub const fn decode(&self) -> &FieldDecodeFlavor {
    &self.decode
  }

  fn from_ast<M>(
    object: &ConcreteObjectAst<M, F>,
    index: usize,
    field: ConcreteTaggedFieldAst<F>,
  ) -> darling::Result<Self>
  where
    F: Clone,
  {
    let path_to_grost = object.path_to_grost();
    let field_ty = field.ty();
    let flavor_type = object.flavor().ty();
    let tag = field.tag();
    let object_ty = object.ty();
    let lifetime_param = object.lifetime_param();
    let lifetime = &lifetime_param.lifetime;
    let unknown_buffer_param = object.unknown_buffer_param();
    let unknown_buffer = &unknown_buffer_param.ident;
    let read_buffer_param = object.read_buffer_param();
    let read_buffer = &read_buffer_param.ident;

    let mut partial_ref_constraints = Punctuated::new();
    let mut selector_constraints = Punctuated::new();

    let use_generics =
      !field.lifetime_params_usages.is_empty() || !field.type_params_usages.is_empty();

    let wfr = syn::parse2(quote! {
      #path_to_grost::__private::reflection::WireFormatReflection<
        #object_ty,
        #flavor_type,
        #tag,
      >
    })?;
    let wf = match field.flavor().format().cloned() {
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

    let partial_ref_copyable = object.partial_ref().copy() || field.partial_ref_copy();
    let partial_ref_copy_contraint = if partial_ref_copyable {
      Some(quote! {
        + ::core::marker::Copy
      })
    } else {
      None
    };

    let (partial_ref_ty, partial_ref_state_type) = match field
      .flavor()
      .ty()
      .or_else(|| field.partial_ref_type())
    {
      Some(ty) => (ty.clone(), None),
      None => {
        let state_type: Type = syn::parse2(quote! {
          #path_to_grost::__private::convert::State<
            #path_to_grost::__private::convert::PartialRef<
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

    let flavor_ty = object.flavor().ty();
    let decode_lt = grost_decode_trait_lifetime();
    let decode_trait_type = syn::parse2(quote! {
      #path_to_grost::__private::Decode<#decode_lt, #flavor_ty, #wf, #partial_ref_ty, #read_buffer, #unknown_buffer>
    })?;

    let optional_partial_ref_type = syn::parse2(quote! {
      ::core::option::Option<#partial_ref_ty>
    })?;

    let use_generics =
      !field.lifetime_params_usages.is_empty() || !field.type_params_usages.is_empty();
    let index = FieldIndex::new(index, field.name(), field.tag())?;
    let schema_name = field.schema_name;
    let schema_description = field.schema_description;
    let field_ty = field.ty;
    let partial = field.partial;
    let partial = ConcretePartialField::from_ast(&field_ty, partial.ty(), partial.attrs())?;
    let partial_ref = ConcretePartialRefField {
      ty: partial_ref_ty,
      optional_type: optional_partial_ref_type,
      partial_ref_state_type,
      decode_trait_type,
      attrs: field.partial_ref.attrs,
      constraints: partial_ref_constraints,
      copy: partial_ref_copyable,
      convert: field.flavor.convert,
    };
    let selector = ConcreteSelectorField {
      ty: selector_type,
      selectable,
      attrs: field.selector.attrs,
      constraints: selector_constraints,
      default: field.selector.select,
    };
    let reflection = ConcreteFieldReflection::try_new(
      object,
      object.flavor().ty(),
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
      tag: field.tag,
      ty: field_ty,
      attrs: field.attrs,
      default: field.default,
      schema_description,
      schema_name,
      encode: field.flavor.encode,
      decode: field.flavor.decode,
      type_params_usages: field.type_params_usages,
      lifetime_params_usages: field.lifetime_params_usages,
      copy: field.copy,
      meta: field.meta,
      wire_format: wf,
      wire_format_reflection: wfr,
      index,
      selector,
      reflection,
    })
  }
}

#[derive(Debug, Clone, derive_more::IsVariant, derive_more::Unwrap, derive_more::TryUnwrap)]
#[unwrap(ref)]
#[try_unwrap(ref)]
pub enum ConcreteField<F = ()> {
  Skipped(Box<SkippedField<F>>),
  Tagged(Box<ConcreteTaggedField<F>>),
}

impl<F> ConcreteField<F> {
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

  /// Returns the custom metadata associated with the field.
  #[inline]
  pub const fn meta(&self) -> &F {
    match self {
      Self::Skipped(skipped) => skipped.meta(),
      Self::Tagged(tagged) => tagged.meta(),
    }
  }

  pub(super) fn from_ast<M>(
    object: &ConcreteObjectAst<M, F>,
    index: usize,
    field: ConcreteFieldAst<F>,
  ) -> darling::Result<Self>
  where
    F: Clone,
  {
    match field {
      ConcreteFieldAst::Skipped(skipped_field) => Ok(Self::Skipped(skipped_field)),
      ConcreteFieldAst::Tagged(concrete_tagged_field) => {
        ConcreteTaggedField::from_ast(object, index, *concrete_tagged_field)
          .map(|t| Self::Tagged(Box::new(t)))
      }
    }
  }
}
