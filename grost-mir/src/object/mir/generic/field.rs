use crate::{
  object::{
    FieldIndex, Label,
    ast::{
      GenericField as GenericFieldAst, GenericObject as GenericObjectAst,
      GenericTaggedField as GenericTaggedFieldAst, SkippedField,
    },
    mir::generic::field::reflection::GenericFieldReflection,
  },
  utils::Invokable,
};

use indexmap::IndexMap;
use quote::quote;
use syn::{Attribute, Ident, Type, TypeParam, Visibility, punctuated::Punctuated};

pub use flavor::*;
pub use partial::*;
pub use partial_ref::*;
pub use selector::*;

mod flavor;
mod partial;
mod partial_ref;
mod reflection;
mod selector;

#[derive(Debug, Clone)]
pub struct GenericTaggedField<M = ()> {
  attrs: Vec<Attribute>,
  vis: Visibility,
  name: Ident,
  ty: Type,
  schema_name: String,
  schema_description: String,
  flavor_param: TypeParam,
  index: FieldIndex,
  label: Label,
  partial_ref: GenericPartialRefField,
  partial: GenericPartialField,
  selector: GenericSelectorField,
  reflection: GenericFieldReflection,
  default: Invokable,
  tag: u32,
  copy: bool,
  meta: M,
  flavors: IndexMap<Ident, FieldFlavor>,
}

impl<F> GenericTaggedField<F> {
  /// Returns the attributes of this field.
  #[inline]
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns the visibility of this field.
  #[inline]
  pub const fn vis(&self) -> &Visibility {
    &self.vis
  }

  /// Returns the name of this field.
  #[inline]
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  /// Returns the type of this field.
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the schema name of this field.
  #[inline]
  pub const fn schema_name(&self) -> &str {
    self.schema_name.as_str()
  }

  /// Returns the schema description of this field.
  #[inline]
  pub const fn schema_description(&self) -> &str {
    self.schema_description.as_str()
  }

  /// Returns the label of this field.
  #[inline]
  pub const fn label(&self) -> &Label {
    &self.label
  }

  /// Returns the tag of this field.
  #[inline]
  pub const fn tag(&self) -> u32 {
    self.tag
  }

  /// Returns whether this field is copyable.
  #[inline]
  pub const fn copy(&self) -> bool {
    self.copy
  }

  /// Returns the custom meta data associated with this field.
  #[inline]
  pub const fn meta(&self) -> &F {
    &self.meta
  }

  /// Returns the index of this field.
  #[inline]
  pub const fn index(&self) -> &FieldIndex {
    &self.index
  }

  /// Returns the default value of this field.
  #[inline]
  pub const fn default(&self) -> &Invokable {
    &self.default
  }

  /// Returns the flavor type parameter of this field.
  #[inline]
  pub const fn flavor_param(&self) -> &TypeParam {
    &self.flavor_param
  }

  /// Returns the selector information of this field.
  #[inline]
  pub const fn selector(&self) -> &GenericSelectorField {
    &self.selector
  }

  /// Returns the partial field information of this field.
  #[inline]
  pub const fn partial(&self) -> &GenericPartialField {
    &self.partial
  }

  /// Returns the partial decoded field information of this field.
  #[inline]
  pub const fn partial_ref(&self) -> &GenericPartialRefField {
    &self.partial_ref
  }

  /// Returns the reflection information of this field.
  #[inline]
  pub const fn reflection(&self) -> &GenericFieldReflection {
    &self.reflection
  }

  /// Returns the flavors of this field.
  #[inline]
  pub const fn flavors(&self) -> &IndexMap<Ident, FieldFlavor> {
    &self.flavors
  }

  fn from_ast<M>(
    object: &GenericObjectAst<M, F>,
    index: usize,
    field: GenericTaggedFieldAst<F>,
  ) -> darling::Result<Self>
  where
    F: Clone,
  {
    let use_generics =
      !field.lifetime_params_usages.is_empty() || !field.type_params_usages.is_empty();
    let partial = GenericPartialField::from_ast(
      object.path_to_grost(),
      field.ty(),
      field.partial_type(),
      field.partial_attrs(),
      use_generics,
    )?;

    let path_to_grost = object.path_to_grost();
    let field_ty = field.ty();
    let flavor_param = &object.flavor_param;
    let flavor_ident = &flavor_param.ident;
    let tag = field.tag();

    let object_ty = &object.ty;
    let object_reflectable = &object.reflectable;
    let lifetime_param = &object.lifetime_param;
    let lifetime = &lifetime_param.lifetime;
    let buffer_param = &object.buffer_param;
    let buffer = &buffer_param.ident;
    let read_buffer_param = &object.read_buffer_param;
    let read_buffer = &read_buffer_param.ident;

    let mut partial_ref_constraints = Punctuated::new();
    let mut selector_constraints = Punctuated::new();

    let wfr: Type = syn::parse2(quote! {
      #path_to_grost::__private::reflection::WireFormatReflection<
        #object_ty,
        #flavor_ident,
        #tag,
      >
    })?;
    let wf: Type = syn::parse2(quote! {
      <#wfr as #object_reflectable>::Reflection
    })?;

    let selectable = syn::parse2(quote! {
      #path_to_grost::__private::selection::Selectable<
        #flavor_ident,
      >
    })?;
    let selector_type = syn::parse2(quote! {
      <#field_ty as #selectable>::Selector
    })?;

    partial_ref_constraints.push(syn::parse2(quote! {
      #wfr: #object_reflectable
    })?);
    partial_ref_constraints.push(syn::parse2(quote! {
      #wf: #path_to_grost::__private::flavors::WireFormat<#flavor_ident>
    })?);
    selector_constraints.push(syn::parse2(quote! {
      #wfr: #object_reflectable
    })?);
    selector_constraints.push(syn::parse2(quote! {
      #wf: #path_to_grost::__private::flavors::WireFormat<#flavor_ident>
    })?);
    selector_constraints.push(syn::parse2(quote! {
      #field_ty: #selectable
    })?);

    let partial_ref_copyable = object.partial_ref().copy() || field.partial_ref_copy();
    let partial_ref_copy_contraint = if partial_ref_copyable {
      Some(quote! {
        + ::core::marker::Copy
      })
    } else {
      None
    };

    let partial_ref_ty = {
      let state_type: Type = syn::parse2(quote! {
        #path_to_grost::__private::convert::State<
          #path_to_grost::__private::convert::PartialRef<
            #lifetime,
            #flavor_ident,
            <#wfr as #object_reflectable>::Reflection,
            #read_buffer,
            #buffer,
          >
        >
      })?;

      partial_ref_constraints.push(syn::parse2(quote! {
        #field_ty: #state_type
      })?);
      partial_ref_constraints.push(syn::parse2(quote! {
        <#field_ty as #state_type>::Output: ::core::marker::Sized #partial_ref_copy_contraint
      })?);

      syn::parse2(quote! {
        <#field_ty as #state_type>::Output
      })?
    };

    let optional_partial_ref_type = syn::parse2(quote! {
      ::core::option::Option<#partial_ref_ty>
    })?;

    let flavors = object
      .flavors()
      .iter()
      .map(|(name, flavor)| {
        let field_flavor = field
          .flavors()
          .get(name)
          .expect("Field flavor already checked when constructing the AST");
        FieldFlavor::try_new(object, flavor, &field, field_flavor).map(|ff| (name.clone(), ff))
      })
      .collect::<darling::Result<IndexMap<_, _>>>()?;

    let index = FieldIndex::new(index, field.name(), field.tag())?;
    let reflection = GenericFieldReflection::try_new(object, &field, tag)?;

    Ok(GenericTaggedField {
      index,
      attrs: field.attrs().to_vec(),
      vis: field.vis().clone(),
      name: field.name().clone(),
      ty: field.ty().clone(),
      reflection,
      schema_name: field.schema_name().to_string(),
      schema_description: field.schema_description().to_string(),
      flavor_param: field.flavor().clone(),
      label: field.label().clone(),
      partial,
      partial_ref: GenericPartialRefField {
        ty: partial_ref_ty,
        optional_type: optional_partial_ref_type,
        attrs: field.partial_ref_attrs().to_vec(),
        constraints: partial_ref_constraints,
        copy: partial_ref_copyable,
      },
      selector: GenericSelectorField {
        ty: selector_type,
        selectable,
        attrs: field.selector_attrs().to_vec(),
        constraints: selector_constraints,
        default: field.selection().clone(),
      },
      default: field.default().clone(),
      tag: field.tag(),
      copy: field.copy(),
      meta: field.meta().clone(),
      flavors,
    })
  }
}

#[derive(Debug, Clone, derive_more::IsVariant, derive_more::Unwrap, derive_more::TryUnwrap)]
#[unwrap(ref)]
#[try_unwrap(ref)]
pub enum GenericField<F = ()> {
  Skipped(Box<SkippedField<F>>),
  Tagged(Box<GenericTaggedField<F>>),
}

impl<F> GenericField<F> {
  /// Returns the name of the field.
  #[inline]
  pub const fn name(&self) -> &Ident {
    match self {
      GenericField::Skipped(f) => f.name(),
      GenericField::Tagged(f) => f.name(),
    }
  }

  /// Returns the type of the field.
  #[inline]
  pub const fn ty(&self) -> &Type {
    match self {
      GenericField::Skipped(f) => f.ty(),
      GenericField::Tagged(f) => f.ty(),
    }
  }

  /// Returns the attributes of the field.
  #[inline]
  pub const fn attrs(&self) -> &[Attribute] {
    match self {
      GenericField::Skipped(f) => f.attrs(),
      GenericField::Tagged(f) => f.attrs(),
    }
  }

  /// Returns the visibility of the field.
  #[inline]
  pub const fn vis(&self) -> &Visibility {
    match self {
      GenericField::Skipped(f) => f.vis(),
      GenericField::Tagged(f) => f.vis(),
    }
  }

  /// Returns the fn returns the default value of the field.
  #[inline]
  pub const fn default(&self) -> &Invokable {
    match self {
      GenericField::Skipped(f) => f.default(),
      GenericField::Tagged(f) => f.default(),
    }
  }

  /// Returns the custom meta data associated with the field.
  #[inline]
  pub const fn meta(&self) -> &F {
    match self {
      GenericField::Skipped(f) => f.meta(),
      GenericField::Tagged(f) => f.meta(),
    }
  }

  pub(super) fn from_ast<M>(
    object: &GenericObjectAst<M, F>,
    index: usize,
    field: GenericFieldAst<F>,
  ) -> darling::Result<Self>
  where
    F: Clone,
  {
    match field {
      GenericFieldAst::Skipped(f) => Ok(GenericField::Skipped(f)),
      GenericFieldAst::Tagged(f) => {
        GenericTaggedField::from_ast(object, index, *f).map(|f| GenericField::Tagged(Box::new(f)))
      }
    }
  }
}
