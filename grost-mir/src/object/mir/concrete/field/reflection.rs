use quote::{format_ident, quote};
use syn::{Expr, Generics, Type, WherePredicate, punctuated::Punctuated, token::Comma};

use crate::object::{
  ConcreteObject,
  mir::{
    encoded_identifier_len_reflection, encoded_identifier_reflection, encoded_tag_len_reflection,
    encoded_tag_reflection, field_reflection, identifier_reflection, tag_reflection,
    wire_format_reflection, wire_type_reflection,
  },
};

#[derive(Debug, Clone)]
pub struct ConcreteFieldReflection {
  field_reflection: Type,
  field_reflection_constraints: Punctuated<WherePredicate, Comma>,
  field_reflection_generics: Generics,
  wire_format_reflection: Type,
  wire_format_reflection_generics: Generics,
  wire_type_reflection: Type,
  wire_type_reflection_generics: Generics,
  identifier_reflection: Type,
  identifier_reflection_generics: Generics,
  encoded_identifier_reflection: Type,
  encoded_identifier_reflection_generics: Generics,
  encoded_identifier_len_reflection: Type,
  encoded_identifier_len_reflection_generics: Generics,
  tag_reflection: Type,
  tag_reflection_generics: Generics,
  encoded_tag_reflection: Type,
  encoded_tag_reflection_generics: Generics,
  encoded_tag_len_reflection: Type,
  encoded_tag_len_reflection_generics: Generics,
  value: Expr,
}

impl ConcreteFieldReflection {
  /// Returns the type of the field reflection.
  #[inline]
  pub const fn field_reflection(&self) -> &Type {
    &self.field_reflection
  }

  /// Returns the value of the field reflection.
  #[inline]
  pub const fn field_reflection_value(&self) -> &Expr {
    &self.value
  }

  /// Returns the type of the wire format reflection.
  #[inline]
  pub const fn wire_format_reflection(&self) -> &Type {
    &self.wire_format_reflection
  }

  /// Returns the type of the wire type reflection.
  #[inline]
  pub const fn wire_type_reflection(&self) -> &Type {
    &self.wire_type_reflection
  }

  /// Returns the type of the identifier reflection.
  #[inline]
  pub const fn identifier_reflection(&self) -> &Type {
    &self.identifier_reflection
  }

  /// Returns the type of the encoded identifier reflection.
  #[inline]
  pub const fn encoded_identifier_reflection(&self) -> &Type {
    &self.encoded_identifier_reflection
  }

  /// Returns the type of the encoded identifier length reflection.
  #[inline]
  pub const fn encoded_identifier_len_reflection(&self) -> &Type {
    &self.encoded_identifier_len_reflection
  }

  /// Returns the type of the tag reflection.
  #[inline]
  pub const fn tag_reflection(&self) -> &Type {
    &self.tag_reflection
  }

  /// Returns the type of the encoded tag reflection.
  #[inline]
  pub const fn encoded_tag_reflection(&self) -> &Type {
    &self.encoded_tag_reflection
  }

  /// Returns the type of the encoded tag length reflection.
  #[inline]
  pub const fn encoded_tag_len_reflection(&self) -> &Type {
    &self.encoded_tag_len_reflection
  }

  /// Returns the constraints of the field reflection.
  #[inline]
  pub const fn field_reflection_constraints(&self) -> &Punctuated<WherePredicate, Comma> {
    &self.field_reflection_constraints
  }

  /// Returns the generics when deriving `Reflectable` for the field.
  #[inline]
  pub const fn field_reflection_generics(&self) -> &Generics {
    &self.field_reflection_generics
  }

  /// Returns the generics when deriving `WireFormatReflection` for the field.
  #[inline]
  pub const fn wire_format_reflection_generics(&self) -> &Generics {
    &self.wire_format_reflection_generics
  }

  /// Returns the generics when deriving `WireSchemaTypeReflection` for the field.
  #[inline]
  pub const fn wire_type_reflection_generics(&self) -> &Generics {
    &self.wire_type_reflection_generics
  }

  /// Returns the generics when deriving `IdentifierReflection` for the field.
  #[inline]
  pub const fn identifier_reflection_generics(&self) -> &Generics {
    &self.identifier_reflection_generics
  }

  /// Returns the generics when deriving `EncodedIdentifierReflection` for the field.
  #[inline]
  pub const fn encoded_identifier_reflection_generics(&self) -> &Generics {
    &self.encoded_identifier_reflection_generics
  }

  /// Returns the generics when deriving `EncodedIdentifierLenReflection` for the field.
  #[inline]
  pub const fn encoded_identifier_len_reflection_generics(&self) -> &Generics {
    &self.encoded_identifier_len_reflection_generics
  }

  /// Returns the generics when deriving `TagReflection` for the field.
  #[inline]
  pub const fn tag_reflection_generics(&self) -> &Generics {
    &self.tag_reflection_generics
  }

  /// Returns the generics when deriving `EncodedTagReflection` for the field.
  #[inline]
  pub const fn encoded_tag_reflection_generics(&self) -> &Generics {
    &self.encoded_tag_reflection_generics
  }

  /// Returns the generics when deriving `EncodedTagLenReflection` for the field.
  #[inline]
  pub const fn encoded_tag_len_reflection_generics(&self) -> &Generics {
    &self.encoded_tag_len_reflection_generics
  }

  #[allow(clippy::too_many_arguments)]
  pub(super) fn try_new<M, F>(
    object: &super::ConcreteObjectAst<M, F>,
    flavor_ty: &Type,
    field_ty: &Type,
    tag: u32,
    schema_name: &str,
    schema_description: &str,
    use_generics: bool,
  ) -> darling::Result<Self>
  where
    F: Clone,
  {
    let path_to_grost = object.path_to_grost();
    let object_type = object.ty();
    let generics = object.generics();
    let mut field_reflection_generics = generics.clone();
    let wire_format_reflection_generics = generics.clone();

    let mut field_reflection_constraints = Punctuated::new();

    if use_generics {
      {
        let pred: WherePredicate = syn::parse2(quote! {
          #path_to_grost::__private::reflection::SchemaTypeReflection<#field_ty>: #path_to_grost::__private::reflection::Reflectable<#field_ty, Reflection = #path_to_grost::__private::reflection::SchemaType>
        })?;
        field_reflection_constraints.push(pred.clone());
        field_reflection_generics
          .make_where_clause()
          .predicates
          .push(pred);
      }
    }

    let field_reflection = field_reflection(path_to_grost, object_type, flavor_ty, tag)?;
    let wire_format_reflection =
      wire_format_reflection(path_to_grost, object_type, flavor_ty, tag)?;
    let wire_type_reflection = wire_type_reflection(path_to_grost, object_type, flavor_ty, tag)?;
    let identifier_reflection = identifier_reflection(path_to_grost, object_type, flavor_ty, tag)?;
    let encoded_identifier_reflection =
      encoded_identifier_reflection(path_to_grost, object_type, flavor_ty, tag)?;
    let encoded_identifier_len_reflection =
      encoded_identifier_len_reflection(path_to_grost, object_type, flavor_ty, tag)?;
    let tag_reflection = tag_reflection(path_to_grost, object_type, flavor_ty, tag)?;
    let encoded_tag_reflection =
      encoded_tag_reflection(path_to_grost, object_type, flavor_ty, tag)?;
    let encoded_tag_len_reflection =
      encoded_tag_len_reflection(path_to_grost, object_type, flavor_ty, tag)?;

    let field_reflection_value = syn::parse2(quote! {
      #path_to_grost::__private::reflection::ObjectFieldBuilder {
        name: #schema_name,
        description: #schema_description,
        ty: <#path_to_grost::__private::reflection::SchemaTypeReflection<#field_ty> as #path_to_grost::__private::reflection::Reflectable<#field_ty>>::REFLECTION,
      }.build()
    })?;

    Ok(Self {
      field_reflection,
      value: field_reflection_value,
      wire_format_reflection,
      wire_type_reflection,
      identifier_reflection,
      encoded_identifier_reflection,
      encoded_identifier_len_reflection,
      tag_reflection,
      encoded_tag_reflection,
      encoded_tag_len_reflection,
      field_reflection_constraints,
      field_reflection_generics,
      wire_type_reflection_generics: wire_format_reflection_generics.clone(),
      identifier_reflection_generics: wire_format_reflection_generics.clone(),
      encoded_identifier_reflection_generics: wire_format_reflection_generics.clone(),
      encoded_identifier_len_reflection_generics: wire_format_reflection_generics.clone(),
      tag_reflection_generics: generics.clone(),
      encoded_tag_reflection_generics: generics.clone(),
      encoded_tag_len_reflection_generics: generics.clone(),
      wire_format_reflection_generics,
    })
  }
}

impl<F> super::ConcreteTaggedField<F> {
  pub(in crate::object::mir::concrete) fn derive_field_reflections<M>(
    &self,
    object: &ConcreteObject<M, F>,
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

    let (field_reflection_ig, _, field_reflection_wc) = self
      .reflection()
      .field_reflection_generics()
      .split_for_impl();
    let field_reflection_type = self.reflection().field_reflection();

    let (wire_format_reflection_ig, _, wire_format_reflection_wc) = self
      .reflection()
      .wire_format_reflection_generics()
      .split_for_impl();
    let wire_format_reflection_type = self.reflection().wire_format_reflection();
    let (identifier_reflection_ig, _, identifier_reflection_wc) = self
      .reflection()
      .identifier_reflection_generics()
      .split_for_impl();
    let identifier_reflection_type = self.reflection().identifier_reflection();
    let (encoded_identifier_reflection_ig, _, encoded_identifier_reflection_wc) = self
      .reflection()
      .encoded_identifier_reflection_generics()
      .split_for_impl();
    let encoded_identifier_reflection_type = self.reflection().encoded_identifier_reflection();
    let (encoded_identifier_len_reflection_ig, _, encoded_identifier_len_reflection_wc) = self
      .reflection()
      .encoded_identifier_len_reflection_generics()
      .split_for_impl();
    let encoded_identifier_len_reflection_type =
      self.reflection().encoded_identifier_len_reflection();
    let (tag_reflection_ig, _, tag_reflection_wc) =
      self.reflection().tag_reflection_generics().split_for_impl();
    let tag_reflection_type = self.reflection().tag_reflection();
    let (encoded_tag_reflection_ig, _, encoded_tag_reflection_wc) = self
      .reflection()
      .encoded_tag_reflection_generics()
      .split_for_impl();
    let encoded_tag_reflection_type = self.reflection().encoded_tag_reflection();
    let (encoded_tag_len_reflection_ig, _, encoded_tag_len_reflection_wc) = self
      .reflection()
      .encoded_tag_len_reflection_generics()
      .split_for_impl();
    let encoded_tag_len_reflection_type = self.reflection().encoded_tag_len_reflection();
    let (wire_type_reflection_ig, _, wire_type_reflection_wc) = self
      .reflection()
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
            ty: <#path_to_grost::__private::reflection::SchemaTypeReflection<#field_ty> as #path_to_grost::__private::reflection::Reflectable<#field_ty>>::REFLECTION,
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

  pub(in crate::object::mir::concrete) fn derive_field_reflection_fn<M>(
    &self,
    object: &ConcreteObject<M, F>,
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
