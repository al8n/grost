use quote::quote;
use syn::{Attribute, Type, WherePredicate, punctuated::Punctuated, token::Comma};

use crate::{
  object::{Label, meta::concrete::PartialFieldFromMeta},
  utils::MissingOperation,
};

use super::PartialFieldConvertOptions;

impl PartialFieldFromMeta {
  /// Finalizes the partial field meta and returns the attribute
  pub(super) fn finalize(self) -> darling::Result<PartialFieldOptions> {
    Ok(PartialFieldOptions {
      ty: self.ty,
      copy: self.copy,
      attrs: self.attrs,
      partial_transform_ref: self.partial_transform_ref.finalize()?,
      transform_ref: self.transform_ref.finalize()?,
      partial_transform: self.partial_transform.finalize()?,
    })
  }
}

#[derive(Debug, Clone)]
pub(super) struct PartialFieldOptions {
  pub(super) ty: Option<Type>,
  pub(super) copy: bool,
  pub(super) attrs: Vec<Attribute>,
  pub(super) transform_ref: PartialFieldConvertOptions,
  pub(super) partial_transform_ref: PartialFieldConvertOptions,
  pub(super) partial_transform: PartialFieldConvertOptions,
}

#[derive(Debug, Clone)]
pub struct PartialField {
  pub(super) ty: Type,
  pub(super) optional_type: Type,
  pub(super) state_type: Option<Type>,
  pub(super) attrs: Vec<Attribute>,
  pub(super) constraints: Punctuated<WherePredicate, Comma>,
  pub(super) transform_ref_constraints: Punctuated<WherePredicate, Comma>,
  pub(super) partial_transform_ref_constraints: Punctuated<WherePredicate, Comma>,
  pub(super) partial_transform_constraints: Punctuated<WherePredicate, Comma>,
  pub(super) transform_ref: PartialFieldConvertOptions,
  pub(super) partial_transform_ref: PartialFieldConvertOptions,
  pub(super) partial_transform: PartialFieldConvertOptions,
}

impl PartialField {
  /// Returns the specified type of the partial field, if any.
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the optional type of the partial field, which is `Option<_>`.
  #[inline]
  pub const fn optional_type(&self) -> &Type {
    &self.optional_type
  }

  /// Returns the partial state type of the partial field, if any.
  #[inline]
  pub const fn state_type(&self) -> Option<&Type> {
    self.state_type.as_ref()
  }

  /// Returns the attributes of the partial field.
  #[inline]
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns the transformation options for the partial field.
  #[inline]
  pub const fn transform_ref(&self) -> &PartialFieldConvertOptions {
    &self.transform_ref
  }

  /// Returns the partial transformation options for the partial field.
  #[inline]
  pub const fn partial_transform_ref(&self) -> &PartialFieldConvertOptions {
    &self.partial_transform_ref
  }

  /// Returns the partial transformation options for the partial field.
  #[inline]
  pub const fn partial_transform(&self) -> &PartialFieldConvertOptions {
    &self.partial_transform
  }

  /// Returns the type constraints of the partial field.
  #[inline]
  pub const fn type_constraints(&self) -> &Punctuated<WherePredicate, Comma> {
    &self.constraints
  }

  /// Returns the transformation reference constraints of the partial field.
  #[inline]
  pub const fn transform_ref_constraints(&self) -> &Punctuated<WherePredicate, Comma> {
    &self.transform_ref_constraints
  }

  /// Returns the partial transformation reference constraints of the partial field.
  #[inline]
  pub const fn partial_transform_ref_constraints(&self) -> &Punctuated<WherePredicate, Comma> {
    &self.partial_transform_ref_constraints
  }

  /// Returns the partial transformation constraints of the partial field.
  #[inline]
  pub const fn partial_transform_constraints(&self) -> &Punctuated<WherePredicate, Comma> {
    &self.partial_transform_constraints
  }

  pub(super) fn try_new<T, S, M>(
    object: &super::RawObject<T, S, M>,
    use_generics: bool,
    field_ty: &Type,
    wf: &Type,
    label: &Label,
    mut opts: PartialFieldOptions,
  ) -> darling::Result<Self> {
    let flavor_type = &object.flavor_type;
    let path_to_grost = &object.path_to_grost;

    let mut type_constraints = Punctuated::new();
    let mut transform_ref_constraints = Punctuated::new();
    let mut partial_transform_ref_constraints = Punctuated::new();
    let mut partial_transform_constraints = Punctuated::new();

    let partial_copyable = object.partial.copy || opts.copy;
    let partial_copy_contraint = if partial_copyable {
      Some(quote! {
        + ::core::marker::Copy
      })
    } else {
      None
    };
    let (ty, state_type) = match &opts.ty {
      Some(ty) => (ty.clone(), None),
      None => {
        let state_type: Type = syn::parse2(quote! {
          #path_to_grost::__private::convert::State<
            #path_to_grost::__private::convert::Partial<
              #flavor_type,
            >
          >
        })?;

        if use_generics {
          type_constraints.extend([
            syn::parse2::<WherePredicate>(quote! {
              #field_ty: #state_type
            })?,
            syn::parse2(quote! {
              <#field_ty as #state_type>::Output: ::core::marker::Sized #partial_copy_contraint
            })?,
          ]);

          let selectable_constraint: WherePredicate = syn::parse2(quote! {
            #field_ty: #path_to_grost::__private::selection::Selectable<#flavor_type>
          })?;
          partial_transform_constraints.extend([
            syn::parse2::<WherePredicate>(quote! {
              <#field_ty as #state_type>::Output: #path_to_grost::__private::convert::PartialTransform<
                <#field_ty as #state_type>::Output,
                ::core::option::Option<<#field_ty as #state_type>::Output>,
                #wf,
                #flavor_type,
              >
              + #path_to_grost::__private::selection::Selectable<#flavor_type, Selector = <#field_ty as #path_to_grost::__private::selection::Selectable<#flavor_type>>::Selector>
              + ::core::marker::Sized
            })?,
            selectable_constraint.clone(),
          ]);

          let ltp = &object.lifetime_param;
          let lt = &ltp.lifetime;
          let rbp = &object.read_buffer_param;
          let rb = &rbp.ident;
          let ubp = &object.unknown_buffer_param;
          let ub = &ubp.ident;

          let ref_state_type: Type = syn::parse2(quote! {
            #path_to_grost::__private::convert::State<
              #path_to_grost::__private::convert::PartialRef<
                #lt,
                #rb,
                #ub,
                #wf,
                #flavor_type,
              >
            >
          })?;

          partial_transform_ref_constraints.extend([
            syn::parse2::<WherePredicate>(quote! {
              #field_ty: #path_to_grost::__private::selection::Selectable<#flavor_type>
            })?,
            syn::parse2::<WherePredicate>(quote! {
              <#field_ty as #state_type>::Output: #path_to_grost::__private::convert::PartialTransform<
                <#field_ty as #ref_state_type>::Output,
                ::core::option::Option<<#field_ty as #state_type>::Output>,
                #wf,
                #flavor_type,
                Selector = <#field_ty as #path_to_grost::__private::selection::Selectable<#flavor_type>>::Selector,
              >
            })?,
            syn::parse2::<WherePredicate>(quote! {
              #field_ty: #ref_state_type
            })?,
            syn::parse2::<WherePredicate>(quote! {
              <#field_ty as #ref_state_type>::Output: ::core::marker::Sized +  #path_to_grost::__private::selection::Selectable<
                #flavor_type,
                Selector = <#field_ty as #path_to_grost::__private::selection::Selectable<#flavor_type>>::Selector,
              >
            })?,
          ]);

          transform_ref_constraints.extend([
            syn::parse2::<WherePredicate>(quote! {
              <#field_ty as #state_type>::Output: #path_to_grost::__private::convert::Transform<
                <#field_ty as #ref_state_type>::Output,
                <#field_ty as #state_type>::Output,
                #wf,
                #flavor_type,
              >
            })?,
            syn::parse2::<WherePredicate>(quote! {
              #field_ty: #ref_state_type
            })?,
            syn::parse2::<WherePredicate>(quote! {
              <#field_ty as #ref_state_type>::Output: ::core::marker::Sized
            })?,
          ]);
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

    Ok(Self {
      ty,
      optional_type,
      state_type,
      attrs: opts.attrs,
      constraints: type_constraints,
      transform_ref_constraints,
      partial_transform_constraints,
      partial_transform_ref_constraints,
      transform_ref: {
        let mo = opts.transform_ref.missing_operation.or_else(|| {
          object
            .partial
            .transform
            .or_default_by_label(label)
            .then_some(MissingOperation::OrDefault)
        });
        opts.transform_ref.missing_operation = mo;
        opts.transform_ref
      },
      partial_transform_ref: {
        let mo = opts.partial_transform_ref.missing_operation.or_else(|| {
          object
            .partial
            .partial_transform
            .or_default_by_label(label)
            .then_some(MissingOperation::OrDefault)
        });
        opts.partial_transform_ref.missing_operation = mo;
        opts.partial_transform_ref
      },
      partial_transform: {
        let mo = opts.partial_transform.missing_operation.or_else(|| {
          object
            .partial
            .partial_transform
            .or_default_by_label(label)
            .then_some(MissingOperation::OrDefault)
        });
        opts.partial_transform.missing_operation = mo;
        opts.partial_transform
      },
    })
  }
}
