use syn::{
  Attribute, Ident, Path, Type, Visibility, WherePredicate, punctuated::Punctuated, token::Comma,
};

use quote::quote;

use crate::ast::object::{
  ConcreteField as ConcreteFieldAst, ConcreteTaggedField as ConcreteTaggedFieldAst, Selection,
  SkippedField,
};

use super::{ConcreteObject, ConcreteObjectAst};

pub use partial::*;
pub use partial_decoded::*;
pub use selector::*;

mod partial;
mod partial_decoded;
mod selector;

#[derive(Debug, Clone)]
pub struct ConcreteTaggedField {
  name: Ident,
  vis: Visibility,
  ty: Type,
  default: Path,
  attrs: Vec<Attribute>,
  wire_format: Type,
  wire_format_reflection: Type,
  partial: ConcretePartialField,
  partial_decoded: ConcretePartialDecodedField,
  selector: ConcreteSelectorField,
  copy: bool,
}

impl ConcreteTaggedField {
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

  /// Returns the attributes of the field.
  #[inline]
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns the default value of the field.
  #[inline]
  pub const fn default(&self) -> &Path {
    &self.default
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
  pub const fn partial_decoded(&self) -> &ConcretePartialDecodedField {
    &self.partial_decoded
  }

  /// Returns the selector field information
  #[inline]
  pub const fn selector(&self) -> &ConcreteSelectorField {
    &self.selector
  }

  fn from_ast(object: &ConcreteObjectAst, field: ConcreteTaggedFieldAst) -> darling::Result<Self> {
    let path_to_grost = object.path_to_grost();
    let field_ty = field.ty();
    let flavor_type = object.flavor().ty();
    let tag = field.tag();

    let partial_decoded_object = object.partial_decoded();
    let object_ty = object.ty();
    let object_reflectable = object.reflectable();
    let lifetime_param = partial_decoded_object.lifetime();
    let lifetime = &lifetime_param.lifetime;
    let unknown_buffer_param = partial_decoded_object.unknown_buffer();
    let unknown_buffer = &unknown_buffer_param.ident;

    let mut partial_decoded_constraints = Punctuated::new();
    let mut selector_constraints = Punctuated::new();

    let wfr = syn::parse2(quote! {
      #path_to_grost::__private::reflection::WireFormatReflection<
        #object_ty,
        #flavor_type,
        #tag,
      >
    })?;
    let wf = match field.flavor().format() {
      Some(wf) => wf.clone(),
      None => syn::parse2(quote! {
        <#wfr as #object_reflectable>::Reflection
      })?,
    };

    let selectable = syn::parse2(quote! {
      #path_to_grost::__private::selection::Selectable<
        #flavor_type,
        #wf,
      >
    })?;
    let selector_type = syn::parse2(quote! {
      <#field_ty as #selectable>::Selector
    })?;

    partial_decoded_constraints.push(syn::parse2(quote! {
      #wfr: #object_reflectable
    })?);
    partial_decoded_constraints.push(syn::parse2(quote! {
      #wf: #path_to_grost::__private::flavors::WireFormat<#flavor_type>
    })?);
    selector_constraints.push(syn::parse2(quote! {
      #wfr: #object_reflectable
    })?);
    selector_constraints.push(syn::parse2(quote! {
      #wf: #path_to_grost::__private::flavors::WireFormat<#flavor_type>
    })?);
    selector_constraints.push(syn::parse2(quote! {
      #field_ty: #selectable
    })?);

    let partial_decoded_copyable = object.partial_decoded().copy() || field.partial_decoded_copy();
    let partial_decoded_copy_contraint = if partial_decoded_copyable {
      Some(quote! {
        + ::core::marker::Copy
      })
    } else {
      None
    };

    let partial_decoded_ty = match field.partial_decoded_type() {
      Some(ty) => ty.clone(),
      None => {
        let state_type: Type = syn::parse2(quote! {
          #path_to_grost::__private::convert::State<
            #path_to_grost::__private::convert::Decoded<
              #lifetime,
              #flavor_type,
              <#wfr as #object_reflectable>::Reflection,
              #unknown_buffer,
            >
          >
        })?;

        partial_decoded_constraints.push(syn::parse2(quote! {
          #field_ty: #state_type
        })?);
        partial_decoded_constraints.push(syn::parse2(quote! {
          <#field_ty as #state_type>::Output: ::core::marker::Sized #partial_decoded_copy_contraint
        })?);

        syn::parse2(quote! {
          <#field_ty as #state_type>::Output
        })?
      }
    };

    let optional_partial_decoded_type = syn::parse2(quote! {
      ::core::option::Option<#partial_decoded_ty>
    })?;

    Ok(Self {
      name: field.name().clone(),
      vis: field.vis().clone(),
      ty: field.ty().clone(),
      attrs: field.attrs().to_vec(),
      default: field.default().clone(),
      wire_format: wf,
      wire_format_reflection: wfr,
      partial: ConcretePartialField::from_ast(
        object.path_to_grost(),
        field.ty(),
        field.partial_type(),
        field.partial_attrs(),
      )?,
      partial_decoded: ConcretePartialDecodedField {
        ty: partial_decoded_ty,
        optional_type: optional_partial_decoded_type,
        attrs: field.partial_decoded_attrs().to_vec(),
        constraints: partial_decoded_constraints,
        copy: partial_decoded_copyable,
      },
      selector: ConcreteSelectorField {
        ty: selector_type,
        selectable,
        attrs: field.selector_attrs().to_vec(),
        constraints: selector_constraints,
        default: field.selection().clone(),
      },
      copy: field.copy(),
    })
  }
}

#[derive(Debug, Clone, derive_more::IsVariant, derive_more::Unwrap, derive_more::TryUnwrap)]
#[unwrap(ref)]
#[try_unwrap(ref)]
pub enum ConcreteField {
  Skipped(SkippedField),
  Tagged(ConcreteTaggedField),
}

impl ConcreteField {
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
  pub const fn default(&self) -> &Path {
    match self {
      Self::Skipped(skipped) => skipped.default(),
      Self::Tagged(tagged) => tagged.default(),
    }
  }

  pub(super) fn from_ast(
    object: &ConcreteObjectAst,
    field: ConcreteFieldAst,
  ) -> darling::Result<Self> {
    match field {
      ConcreteFieldAst::Skipped(skipped_field) => Ok(Self::Skipped(skipped_field)),
      ConcreteFieldAst::Tagged(concrete_tagged_field) => {
        ConcreteTaggedField::from_ast(object, concrete_tagged_field).map(Self::Tagged)
      }
    }
  }
}
