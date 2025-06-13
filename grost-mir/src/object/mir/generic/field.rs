use crate::{
  object::{FieldIndex, Label},
  utils::Invokable,
};

use super::super::super::ast::{
  FieldDecodeAttribute, FieldEncodeAttribute, FieldFlavor as FieldFlavorAst,
  GenericField as GenericFieldAst, GenericObject as GenericObjectAst,
  GenericTaggedField as GenericTaggedFieldAst, SkippedField,
};

use indexmap::IndexMap;
use quote::quote;
use syn::{
  Attribute, Ident, Type, TypeParam, Visibility, WherePredicate, punctuated::Punctuated,
  token::Comma,
};

pub use partial::*;
pub use partial_decoded::*;
pub use selector::*;

mod partial;
mod partial_decoded;
mod selector;

#[derive(Debug, Clone)]
pub struct PartialDecodedFieldFlavor {
  ty: Type,
  optional_type: Type,
  constraints: Punctuated<WherePredicate, Comma>,
}

impl PartialDecodedFieldFlavor {
  /// Returns the type of the partial decoded field for this flavor.
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the optional type of the partial decoded field for this flavor.
  #[inline]
  pub const fn optional_type(&self) -> &Type {
    &self.optional_type
  }

  /// Returns the type constraints for the partial decoded field for this flavor.
  #[inline]
  pub const fn type_constraints(&self) -> &Punctuated<WherePredicate, Comma> {
    &self.constraints
  }
}

#[derive(Debug, Clone)]
pub struct SelectorFieldFlavor {
  ty: Type,
  selectable: Type,
  constraints: Punctuated<WherePredicate, Comma>,
}

impl SelectorFieldFlavor {
  /// Returns the type of the selector field for this flavor.
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the selectable type for this flavor.
  #[inline]
  pub const fn selectable(&self) -> &Type {
    &self.selectable
  }

  /// Returns the type constraints for the selector field for this flavor.
  #[inline]
  pub const fn type_constraints(&self) -> &Punctuated<WherePredicate, Comma> {
    &self.constraints
  }
}

#[derive(Debug, Clone)]
pub struct FieldFlavor {
  wire_format: Type,
  encode: FieldEncodeAttribute,
  decode: FieldDecodeAttribute,
  partial_decoded: PartialDecodedFieldFlavor,
  selector: SelectorFieldFlavor,
}

impl FieldFlavor {
  /// Returns the wire format type of this flavor.
  #[inline]
  pub const fn wire_format(&self) -> &Type {
    &self.wire_format
  }

  /// Returns the encode attribute of this flavor.
  #[inline]
  pub const fn encode(&self) -> &FieldEncodeAttribute {
    &self.encode
  }

  /// Returns the decode attribute of this flavor.
  #[inline]
  pub const fn decode(&self) -> &FieldDecodeAttribute {
    &self.decode
  }

  /// Returns the partial decoded field information of this flavor.
  #[inline]
  pub const fn partial_decoded(&self) -> &PartialDecodedFieldFlavor {
    &self.partial_decoded
  }

  /// Returns the selector field information of this flavor.
  #[inline]
  pub const fn selector(&self) -> &SelectorFieldFlavor {
    &self.selector
  }

  fn try_new<M, F>(
    object: &GenericObjectAst<M, F>,
    field: &GenericTaggedFieldAst<F>,
    flavor_type: &Type,
    ast: &FieldFlavorAst,
  ) -> darling::Result<Self> {
    let path_to_grost = object.path_to_grost();
    let object_ty = object.ty();
    let object_reflectable = object.reflectable();
    let lifetime_param = object.partial_decoded().lifetime();
    let lifetime = &lifetime_param.lifetime;
    let unknown_buffer_param = object.partial_decoded().unknown_buffer();
    let unknown_buffer = &unknown_buffer_param.ident;
    let field_ty = field.ty();
    let tag = field.tag();
    let wire_format = match ast.format() {
      Some(wt) => wt.clone(),
      None => syn::parse2(quote! {
        <#field_ty as #path_to_grost::__private::flavors::DefaultWireFormat<#flavor_type>>::Format
      })?,
    };

    let mut partial_decoded_constraints = Punctuated::new();
    let mut selector_constraints = Punctuated::new();

    let wfr: Type = syn::parse2(quote! {
      #path_to_grost::__private::reflection::WireFormatReflection<
        #object_ty,
        #flavor_type,
        #tag,
      >
    })?;
    let wf: Type = syn::parse2(quote! {
      <#wfr as #object_reflectable>::Reflection
    })?;

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

    let partial_decoded_ty = match ast.ty().or(field.partial_decoded_type()) {
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
      wire_format,
      encode: ast.encode().clone(),
      decode: ast.decode().clone(),
      partial_decoded: PartialDecodedFieldFlavor {
        ty: partial_decoded_ty,
        optional_type: optional_partial_decoded_type,
        constraints: partial_decoded_constraints,
      },
      selector: SelectorFieldFlavor {
        ty: selector_type,
        selectable,
        constraints: selector_constraints,
      },
    })
  }
}

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
  partial_decoded: GenericPartialDecodedField,
  partial: GenericPartialField,
  selector: GenericSelectorField,
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
  pub const fn partial_decoded(&self) -> &GenericPartialDecodedField {
    &self.partial_decoded
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
    let partial = GenericPartialField::from_ast(
      object.path_to_grost(),
      field.ty(),
      field.partial_type(),
      field.partial_attrs(),
    )?;

    let path_to_grost = object.path_to_grost();
    let field_ty = field.ty();
    let flavor_param = object.partial_decoded().flavor();
    let flavor_ident = &flavor_param.ident;
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

    let partial_decoded_copyable = object.partial_decoded().copy() || field.partial_decoded_copy();
    let partial_decoded_copy_contraint = if partial_decoded_copyable {
      Some(quote! {
        + ::core::marker::Copy
      })
    } else {
      None
    };

    let partial_decoded_ty = {
      let state_type: Type = syn::parse2(quote! {
        #path_to_grost::__private::convert::State<
          #path_to_grost::__private::convert::Decoded<
            #lifetime,
            #flavor_ident,
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
    };

    let optional_partial_decoded_type = syn::parse2(quote! {
      ::core::option::Option<#partial_decoded_ty>
    })?;

    let flavors = object
      .flavors()
      .iter()
      .map(|(name, flavor)| {
        let field_flavor = field
          .flavors()
          .get(name)
          .expect("Field flavor already checked when constructing the AST");
        FieldFlavor::try_new(object, &field, flavor.ty(), field_flavor).map(|ff| (name.clone(), ff))
      })
      .collect::<darling::Result<IndexMap<_, _>>>()?;

    let index = FieldIndex::new(index, field.name(), field.tag())?;

    Ok(GenericTaggedField {
      index,
      attrs: field.attrs().to_vec(),
      vis: field.vis().clone(),
      name: field.name().clone(),
      ty: field.ty().clone(),
      schema_name: field.schema_name().to_string(),
      schema_description: field.schema_description().to_string(),
      flavor_param: field.flavor().clone(),
      label: field.label().clone(),
      partial,
      partial_decoded: GenericPartialDecodedField {
        ty: partial_decoded_ty,
        optional_type: optional_partial_decoded_type,
        attrs: field.partial_decoded_attrs().to_vec(),
        constraints: partial_decoded_constraints,
        copy: partial_decoded_copyable,
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
