use crate::{
  object::{
    Label, ast::concrete::field::applied_partial_ref_state, meta::concrete::RefFieldFromMeta,
  },
  utils::{MissingOperation, grost_decode_trait_lifetime},
};

use super::{FieldDecodeOptions, FieldEncodeOptions};
use quote::quote;
use syn::{Attribute, Type, WherePredicate, punctuated::Punctuated, token::Comma};

impl RefFieldFromMeta {
  pub(super) fn finalize(self) -> darling::Result<RefFieldOptions> {
    Ok(RefFieldOptions {
      copy: self.copy,
      attrs: self.attrs,
      ty: self.ty,
      encode: self.encode.finalize()?,
      decode: self.decode.finalize()?,
      missing_operation: self.missing_operation,
    })
  }
}

#[derive(Debug, Clone)]
pub(super) struct RefFieldOptions {
  pub(in crate::object) copy: bool,
  pub(in crate::object) attrs: Vec<Attribute>,
  pub(in crate::object) ty: Option<Type>,
  pub(in crate::object) encode: FieldEncodeOptions,
  pub(in crate::object) decode: FieldDecodeOptions,
  pub(in crate::object) missing_operation: Option<MissingOperation>,
}

#[derive(Debug, Clone)]
pub struct RefField {
  pub(super) ty: Type,
  pub(super) nullable_type: Type,
  pub(super) state_type: Option<Type>,
  pub(super) decode_trait_type: Type,
  pub(super) attrs: Vec<Attribute>,
  pub(super) constraints: Punctuated<WherePredicate, Comma>,
  pub(super) copy: bool,
  pub(super) encode: FieldEncodeOptions,
  pub(super) decode: FieldDecodeOptions,
  pub(super) missing_operation: Option<MissingOperation>,
}

impl RefField {
  /// Returns the type of the ref field.
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the nullable type of the ref field, which is `Option<_>`.
  #[inline]
  pub const fn nullable_type(&self) -> &Type {
    &self.nullable_type
  }

  /// Returns the decoded state type of the ref field.
  ///
  /// Returns `Some(_)` only if the field use generics,
  #[inline]
  pub const fn state_type(&self) -> Option<&Type> {
    self.state_type.as_ref()
  }

  /// Returns the field decode trait type for this ref field.
  #[inline]
  pub const fn decode_trait_type(&self) -> &Type {
    &self.decode_trait_type
  }

  /// Returns the constraints of the ref field.
  #[inline]
  pub const fn type_constraints(&self) -> &Punctuated<WherePredicate, Comma> {
    &self.constraints
  }

  /// Returns the attributes of the ref field.
  #[inline]
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns whether the ref field is copyable.
  #[inline]
  pub const fn copy(&self) -> bool {
    self.copy
  }

  /// Returns the encode options about how to encode the ref field.
  #[inline]
  pub const fn encode(&self) -> &FieldEncodeOptions {
    &self.encode
  }

  /// Returns the decode options about how to decode the ref field.
  #[inline]
  pub const fn decode(&self) -> &FieldDecodeOptions {
    &self.decode
  }

  /// Returns the missing operation that should be performed if the field is missing during decoding.
  #[inline]
  pub const fn missing_operation(&self) -> Option<&MissingOperation> {
    self.missing_operation.as_ref()
  }

  pub(super) fn try_new<T, S, M>(
    object: &super::RawObject<T, S, M>,
    field_ty: &Type,
    wf: &Type,
    label: &Label,
    opts: RefFieldOptions,
    mut type_constraints: Punctuated<WherePredicate, Comma>,
  ) -> darling::Result<Self> {
    let flavor_type = &object.flavor_type;
    let path_to_grost = &object.path_to_grost;
    let lifetime_param = &object.lifetime_param;
    let lifetime = &lifetime_param.lifetime;
    let buffer_param = &object.buffer_param;
    let buffer = &buffer_param.ident;
    let read_buffer_param = &object.read_buffer_param;
    let read_buffer = &read_buffer_param.ident;

    let partial_ref_copyable = object.partial_ref.copy || opts.copy;
    let partial_ref_copy_contraint = if partial_ref_copyable {
      Some(quote! {
        + ::core::marker::Copy
      })
    } else {
      None
    };
    let (ty, state_type) = match &opts.ty {
      Some(ty) => (ty.clone(), None),
      None => {
        let state_type: Type = syn::parse2(applied_partial_ref_state(
          path_to_grost,
          lifetime,
          read_buffer,
          buffer,
          wf,
          flavor_type,
        ))?;

        if label.is_generic() {
          type_constraints.push(syn::parse2(quote! {
            #field_ty: #state_type
          })?);
          type_constraints.push(syn::parse2(quote! {
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

    let nullable_type = if label.is_nullable() {
      ty.clone()
    } else {
      syn::parse2(quote! {
        ::core::option::Option<#ty>
      })?
    };

    let decode_lt = grost_decode_trait_lifetime();
    let decode_trait_type = syn::parse2(quote! {
      #path_to_grost::__private::decode::Decode<#decode_lt, #ty, #wf, #read_buffer, #buffer, #flavor_type>
    })?;

    Ok(Self {
      ty,
      nullable_type,
      state_type,
      decode_trait_type,
      attrs: opts.attrs,
      constraints: type_constraints,
      copy: partial_ref_copyable,
      encode: opts.encode,
      decode: {
        
        opts.decode
      },
      missing_operation: opts.missing_operation.or_else(|| {
          object
            .partial_ref
            .or_default
            .or_default_by_label(label)
            .then_some(MissingOperation::OrDefault)
      }),
    })
  }
}
