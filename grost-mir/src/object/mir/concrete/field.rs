use darling::usage::{IdentSet, LifetimeSet};
use syn::{Attribute, Ident, Type, Visibility, punctuated::Punctuated};

use quote::{format_ident, quote};

use crate::{
  object::{FieldIndex, Label},
  utils::Invokable,
};

use super::{
  super::super::ast::{
    ConcreteField as ConcreteFieldAst, ConcreteTaggedField as ConcreteTaggedFieldAst, SkippedField,
  },
  ConcreteObjectAst,
};

pub use partial::*;
pub use partial_decoded::*;
pub use reflection::*;
pub use selector::*;

mod partial;
mod partial_decoded;
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
  partial_decoded: ConcretePartialDecodedField,
  index: FieldIndex,
  reflection: ConcreteFieldReflection,
  selector: ConcreteSelectorField,
  schema_name: String,
  schema_description: String,
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
  pub const fn partial_decoded(&self) -> &ConcretePartialDecodedField {
    &self.partial_decoded
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
      Some(wf) => {
        selector_constraints.push(syn::parse2(quote! {
          #wf: #path_to_grost::__private::flavors::WireFormat<#flavor_type>
        })?);
        wf.clone()
      }
      None => {
        selector_constraints.push(syn::parse2(quote! {
          #field_ty: #path_to_grost::__private::flavors::DefaultWireFormat<#flavor_type>
        })?);

        syn::parse2(quote! {
          <#field_ty as #path_to_grost::__private::flavors::DefaultWireFormat<
            #flavor_type,
          >>::Format
        })?
      }
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

    let partial_decoded_ty = match field.flavor().ty().or_else(|| field.partial_decoded_type()) {
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

    let reflection = ConcreteFieldReflection::try_new(object, &field, object.flavor().ty(), tag)?;
    let index = FieldIndex::new(index, field.name(), field.tag())?;
    let partial = ConcretePartialField::from_ast(
      object.path_to_grost(),
      field.ty(),
      field.partial_type(),
      field.partial_attrs(),
    )?;
    let partial_decoded = ConcretePartialDecodedField {
      ty: partial_decoded_ty,
      optional_type: optional_partial_decoded_type,
      attrs: field.partial_decoded.attrs,
      constraints: partial_decoded_constraints,
      copy: partial_decoded_copyable,
    };
    let selector = ConcreteSelectorField {
      ty: selector_type,
      selectable,
      attrs: field.selector.attrs,
      constraints: selector_constraints,
      default: field.selector.select,
    };
    Ok(Self {
      partial,
      partial_decoded,
      name: field.name,
      vis: field.vis,
      label: field.label,
      tag: field.tag,
      ty: field.ty,
      attrs: field.attrs,
      default: field.default,
      schema_description: field.schema_description,
      schema_name: field.schema_name,
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

  pub(super) fn derive_field_reflections<M>(
    &self,
    object: &super::ConcreteObject<M, F>,
  ) -> proc_macro2::TokenStream {
    let path_to_grost = object.path_to_grost();
    let object_reflectable = object.reflectable();
    let field_ty = self.ty();
    let flavor_ty = object.flavor_type();
    let wf = self.wire_format();
    let schema_name = self.schema_name();
    let schema_description = self.schema_description();
    let identifier = object.flavor.identifier();
    let identifier_constructor = identifier.constructor();
    let identifier_encode = identifier.encode();
    let tag_constructor = object.flavor.tag().constructor();
    let tag_encode = object.flavor.tag().encode();
    let tag = self.tag();

    let (field_reflection_ig, _, field_reflection_wc) = object
      .reflection
      .field_reflection_generics()
      .split_for_impl();
    let field_reflection_type = self.reflection().field_reflection();

    let (wire_format_reflection_ig, _, wire_format_reflection_wc) = object
      .reflection
      .wire_format_reflection_generics()
      .split_for_impl();
    let wire_format_reflection_type = self.reflection().wire_format_reflection();
    let (identifier_reflection_ig, _, identifier_reflection_wc) = object
      .reflection
      .identifier_reflection_generics()
      .split_for_impl();
    let identifier_reflection_type = self.reflection().identifier_reflection();
    let (encoded_identifier_reflection_ig, _, encoded_identifier_reflection_wc) = object
      .reflection
      .encoded_identifier_reflection_generics()
      .split_for_impl();
    let encoded_identifier_reflection_type = self.reflection().encoded_identifier_reflection();
    let (encoded_identifier_len_reflection_ig, _, encoded_identifier_len_reflection_wc) = object
      .reflection
      .encoded_identifier_len_reflection_generics()
      .split_for_impl();
    let encoded_identifier_len_reflection_type =
      self.reflection().encoded_identifier_len_reflection();
    let (tag_reflection_ig, _, tag_reflection_wc) =
      object.reflection.tag_reflection_generics().split_for_impl();
    let tag_reflection_type = self.reflection().tag_reflection();
    let (encoded_tag_reflection_ig, _, encoded_tag_reflection_wc) = object
      .reflection
      .encoded_tag_reflection_generics()
      .split_for_impl();
    let encoded_tag_reflection_type = self.reflection().encoded_tag_reflection();
    let (encoded_tag_len_reflection_ig, _, encoded_tag_len_reflection_wc) = object
      .reflection
      .encoded_tag_len_reflection_generics()
      .split_for_impl();
    let encoded_tag_len_reflection_type = self.reflection().encoded_tag_len_reflection();
    let (wire_type_reflection_ig, _, wire_type_reflection_wc) = object
      .reflection
      .wire_type_reflection_generics()
      .split_for_impl();
    let wire_type_reflection_type = self.reflection().wire_type_reflection();

    quote! {
      #[automatically_derived]
      #[allow(clippy::type_complexity, non_camel_case_types)]
      impl #field_reflection_ig #object_reflectable for #field_reflection_type #field_reflection_wc {
        type Reflection = #path_to_grost::__private::reflection::ObjectField;

        const REFLECTION: &'static Self::Reflection = &{
          #path_to_grost::__private::reflection::ObjectFieldBuilder {
            name: #schema_name,
            description: #schema_description,
            ty: <#path_to_grost::__private::reflection::TypeReflection<#field_ty> as #path_to_grost::__private::reflection::Reflectable<#field_ty>>::REFLECTION,
          }.build()
        };
      }

      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl #wire_format_reflection_ig #object_reflectable for #wire_format_reflection_type #wire_format_reflection_wc {
        type Reflection = #wf;

        const REFLECTION: &'static Self::Reflection = &{
          <#wf as #path_to_grost::__private::flavors::WireFormat<#flavor_ty>>::SELF
        };
      }

      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl #identifier_reflection_ig #object_reflectable for #identifier_reflection_type #identifier_reflection_wc {
        type Reflection = <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Identifier;

        const REFLECTION: &Self::Reflection = &{
          (#identifier_constructor)(
            <#wf as #path_to_grost::__private::flavors::WireFormat<#flavor_ty>>::WIRE_TYPE,
            (#tag_constructor)(#tag),
          )
        };
      }

      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl #encoded_identifier_reflection_ig #object_reflectable for #encoded_identifier_reflection_type #encoded_identifier_reflection_wc
      {
        type Reflection = [::core::primitive::u8];

        const REFLECTION: &Self::Reflection = {
          (#identifier_encode)(<#identifier_reflection_type as #object_reflectable>::REFLECTION)
            .as_slice()
        };
      }

      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl #encoded_identifier_len_reflection_ig #object_reflectable for #encoded_identifier_len_reflection_type #encoded_identifier_len_reflection_wc
      {
        type Reflection = ::core::primitive::usize;

        const REFLECTION: &Self::Reflection = {
          &<#encoded_identifier_reflection_type as #object_reflectable>::REFLECTION.len()
        };
      }

      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl #tag_reflection_ig #object_reflectable for #tag_reflection_type #tag_reflection_wc {
        type Reflection = <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Tag;

        const REFLECTION: &Self::Reflection = &{
          (#tag_constructor)(#tag)
        };
      }

      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl #encoded_tag_reflection_ig #object_reflectable for #encoded_tag_reflection_type #encoded_tag_reflection_wc {
        type Reflection = [::core::primitive::u8];

        const REFLECTION: &Self::Reflection = {
          (#tag_encode)(<#tag_reflection_type as #object_reflectable>::REFLECTION)
            .as_slice()
        };
      }

      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl #encoded_tag_len_reflection_ig #object_reflectable for #encoded_tag_len_reflection_type #encoded_tag_len_reflection_wc {
        type Reflection = ::core::primitive::usize;

        const REFLECTION: &Self::Reflection = {
          &<#encoded_tag_reflection_type as #object_reflectable>::REFLECTION.len()
        };
      }

      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl #wire_type_reflection_ig #object_reflectable for #wire_type_reflection_type #wire_type_reflection_wc {
        type Reflection = <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::WireType;

        const REFLECTION: &Self::Reflection = &{
          <#wf as #path_to_grost::__private::flavors::WireFormat<#flavor_ty>>::WIRE_TYPE
        };
      }
    }
  }

  pub(super) fn derive_field_reflection_fn<M>(
    &self,
    object: &super::ConcreteObject<M, F>,
  ) -> proc_macro2::TokenStream {
    let path_to_grost = object.path_to_grost();
    let object_name = object.name();
    let field_name = self.name();
    let vis = self.vis();
    let doc = format!(" Returns the field reflection of the field `{object_name}.{field_name}`.",);
    let reflection_type = self.reflection().field_reflection();
    let field_reflection_name = format_ident!("{}_reflection", field_name);
    let flavor_ty = object.flavor_type();

    quote! {
      #[doc = #doc]
      #[inline]
      #vis const fn #field_reflection_name() -> #reflection_type
      where
        #flavor_ty: #path_to_grost::__private::flavors::Flavor,
      {
        #path_to_grost::__private::reflection::ObjectFieldReflection::new()
      }
    }
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
