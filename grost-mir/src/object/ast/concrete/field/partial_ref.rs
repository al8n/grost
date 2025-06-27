use crate::{
  object::{
    Label, ast::concrete::field::applied_partial_ref_state, meta::concrete::PartialRefFieldFromMeta,
  },
  utils::{MissingOperation, grost_decode_trait_lifetime},
};

use super::{FieldDecodeOptions, FieldEncodeOptions};
use quote::quote;
use syn::{Attribute, Type, WherePredicate, punctuated::Punctuated, token::Comma};

impl PartialRefFieldFromMeta {
  pub(super) fn finalize(self) -> darling::Result<PartialRefFieldOptions> {
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
pub(super) struct PartialRefFieldOptions {
  pub(in crate::object) copy: bool,
  pub(in crate::object) attrs: Vec<Attribute>,
  pub(in crate::object) ty: Option<Type>,
  pub(in crate::object) encode: FieldEncodeOptions,
  pub(in crate::object) decode: FieldDecodeOptions,
}

#[derive(Debug, Clone)]
pub struct PartialRefField {
  pub(super) ty: Type,
  pub(super) optional_type: Type,
  pub(super) state_type: Option<Type>,
  pub(super) decode_trait_type: Type,
  pub(super) attrs: Vec<Attribute>,
  pub(super) constraints: Punctuated<WherePredicate, Comma>,
  pub(super) copy: bool,
  pub(super) encode: FieldEncodeOptions,
  pub(super) decode: FieldDecodeOptions,
}

impl PartialRefField {
  /// Returns the type of the partial ref field.
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the optional type of the partial ref field, which is `Option<_>`.
  #[inline]
  pub const fn optional_type(&self) -> &Type {
    &self.optional_type
  }

  /// Returns the decoded state type of the partial ref field.
  ///
  /// Returns `Some(_)` only if the field use generics,
  #[inline]
  pub const fn state_type(&self) -> Option<&Type> {
    self.state_type.as_ref()
  }

  /// Returns the field decode trait type for this partial ref field.
  #[inline]
  pub const fn decode_trait_type(&self) -> &Type {
    &self.decode_trait_type
  }

  /// Returns the constraints of the partial ref field.
  #[inline]
  pub const fn type_constraints(&self) -> &Punctuated<WherePredicate, Comma> {
    &self.constraints
  }

  /// Returns the attributes of the partial ref field.
  #[inline]
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns whether the partial ref field is copyable.
  #[inline]
  pub const fn copy(&self) -> bool {
    self.copy
  }

  /// Returns the encode options about how to encode the partial ref field.
  #[inline]
  pub const fn encode(&self) -> &FieldEncodeOptions {
    &self.encode
  }

  /// Returns the decode options about how to decode the partial ref field.
  #[inline]
  pub const fn decode(&self) -> &FieldDecodeOptions {
    &self.decode
  }

  pub(super) fn try_new<T, S, M>(
    object: &super::RawObject<T, S, M>,
    use_generics: bool,
    field_ty: &Type,
    wf: &Type,
    label: &Label,
    mut opts: PartialRefFieldOptions,
    mut type_constraints: Punctuated<WherePredicate, Comma>,
  ) -> darling::Result<Self> {
    let flavor_type = &object.flavor_type;
    let path_to_grost = &object.path_to_grost;
    let lifetime_param = &object.lifetime_param;
    let lifetime = &lifetime_param.lifetime;
    let unknown_buffer_param = &object.unknown_buffer_param;
    let unknown_buffer = &unknown_buffer_param.ident;
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
          unknown_buffer,
          wf,
          flavor_type,
        ))?;

        if use_generics {
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

    let optional_type = syn::parse2(quote! {
      ::core::option::Option<#ty>
    })?;

    let decode_lt = grost_decode_trait_lifetime();
    let decode_trait_type = syn::parse2(quote! {
      #path_to_grost::__private::decode::Decode<#decode_lt, #flavor_type, #wf, #ty, #read_buffer, #unknown_buffer>
    })?;

    Ok(Self {
      ty,
      optional_type,
      state_type,
      decode_trait_type,
      attrs: opts.attrs,
      constraints: type_constraints,
      copy: partial_ref_copyable,
      encode: opts.encode,
      decode: {
        let mo = opts.decode.missing_operation().cloned().or_else(|| {
          object
            .partial_ref
            .decode
            .or_default_by_label(label)
            .then_some(MissingOperation::OrDefault)
        });
        opts.decode.missing_operation = mo;
        opts.decode
      },
    })
  }
}
