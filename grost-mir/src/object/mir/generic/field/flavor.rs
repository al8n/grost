use quote::quote;
use syn::{Type, WherePredicate, punctuated::Punctuated, token::Comma};

use crate::object::{
  GenericObject,
  ast::{
    FieldDecodeFlavor, FieldEncodeFlavor, FieldFlavor as FieldFlavorAst,
    GenericObject as GenericObjectAst, GenericTaggedField as GenericTaggedFieldAst,
    ObjectFlavor as ObjectFlavorAst,
  },
  mir::{
    encoded_identifier_len_reflection, encoded_identifier_reflection, encoded_tag_len_reflection,
    encoded_tag_reflection, generic::ObjectFlavor, identifier_reflection, tag_reflection,
    wire_format_reflection, wire_type_reflection,
  },
};

#[derive(Debug, Clone)]
pub struct PartialRefFieldFlavor {
  ty: Type,
  optional_type: Type,
  constraints: Punctuated<WherePredicate, Comma>,
}

impl PartialRefFieldFlavor {
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
  encode: FieldEncodeFlavor,
  decode: FieldDecodeFlavor,
  partial_ref: PartialRefFieldFlavor,
  selector: SelectorFieldFlavor,
  wire_format_reflection: Type,
  wire_format_reflection_constraints: Punctuated<WherePredicate, Comma>,
  wire_type_reflection: Type,
  wire_type_reflection_constraints: Punctuated<WherePredicate, Comma>,
  identifier_reflection: Type,
  identifier_reflection_constraints: Punctuated<WherePredicate, Comma>,
  encoded_identifier_reflection: Type,
  encoded_identifier_reflection_constraints: Punctuated<WherePredicate, Comma>,
  encoded_identifier_len_reflection: Type,
  encoded_identifier_len_reflection_constraints: Punctuated<WherePredicate, Comma>,
  tag_reflection: Type,
  tag_reflection_constraints: Punctuated<WherePredicate, Comma>,
  encoded_tag_reflection: Type,
  encoded_tag_reflection_constraints: Punctuated<WherePredicate, Comma>,
  encoded_tag_len_reflection: Type,
  encoded_tag_len_reflection_constraints: Punctuated<WherePredicate, Comma>,
}

impl FieldFlavor {
  /// Returns the wire format type of this flavor.
  #[inline]
  pub const fn wire_format(&self) -> &Type {
    &self.wire_format
  }

  /// Returns the encode attribute of this flavor.
  #[inline]
  pub const fn encode(&self) -> &FieldEncodeFlavor {
    &self.encode
  }

  /// Returns the decode attribute of this flavor.
  #[inline]
  pub const fn decode(&self) -> &FieldDecodeFlavor {
    &self.decode
  }

  /// Returns the partial decoded field information of this flavor.
  #[inline]
  pub const fn partial_ref(&self) -> &PartialRefFieldFlavor {
    &self.partial_ref
  }

  /// Returns the selector field information of this flavor.
  #[inline]
  pub const fn selector(&self) -> &SelectorFieldFlavor {
    &self.selector
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

  /// Returns the constraints for the wire format reflection.
  #[inline]
  pub const fn wire_format_reflection_constraints(&self) -> &Punctuated<WherePredicate, Comma> {
    &self.wire_format_reflection_constraints
  }

  /// Returns the constraints for the wire type reflection.
  #[inline]
  pub const fn wire_type_reflection_constraints(&self) -> &Punctuated<WherePredicate, Comma> {
    &self.wire_type_reflection_constraints
  }

  /// Returns the constraints for the identifier reflection.
  #[inline]
  pub const fn identifier_reflection_constraints(&self) -> &Punctuated<WherePredicate, Comma> {
    &self.identifier_reflection_constraints
  }

  /// Returns the constraints for the encoded identifier reflection.
  #[inline]
  pub const fn encoded_identifier_reflection_constraints(
    &self,
  ) -> &Punctuated<WherePredicate, Comma> {
    &self.encoded_identifier_reflection_constraints
  }

  /// Returns the constraints for the encoded identifier length reflection.
  #[inline]
  pub const fn encoded_identifier_len_reflection_constraints(
    &self,
  ) -> &Punctuated<WherePredicate, Comma> {
    &self.encoded_identifier_len_reflection_constraints
  }

  /// Returns the constraints for the tag reflection.
  #[inline]
  pub const fn tag_reflection_constraints(&self) -> &Punctuated<WherePredicate, Comma> {
    &self.tag_reflection_constraints
  }

  /// Returns the constraints for the encoded tag reflection.
  #[inline]
  pub const fn encoded_tag_reflection_constraints(&self) -> &Punctuated<WherePredicate, Comma> {
    &self.encoded_tag_reflection_constraints
  }

  /// Returns the constraints for the encoded tag length reflection.
  #[inline]
  pub const fn encoded_tag_len_reflection_constraints(&self) -> &Punctuated<WherePredicate, Comma> {
    &self.encoded_tag_len_reflection_constraints
  }

  pub(super) fn try_new<M, F>(
    object: &GenericObjectAst<M, F>,
    object_flavor: &ObjectFlavorAst,
    field: &GenericTaggedFieldAst<F>,
    ast: &FieldFlavorAst,
  ) -> darling::Result<Self> {
    let path_to_grost = &object.path_to_grost;
    let flavor_type = object_flavor.ty();
    let object_ty = &object.ty;
    let object_reflectable = &object.reflectable;
    let lifetime_param = &object.lifetime_param;
    let lifetime = &lifetime_param.lifetime;
    let buffer_param = &object.buffer_param;
    let buffer = &buffer_param.ident;
    let field_ty = field.ty();
    let tag = field.tag();
    let wire_format = match ast.format() {
      Some(wt) => wt.clone(),
      None => syn::parse2(quote! {
        <#field_ty as #path_to_grost::__private::flavors::DefaultWireFormat<#flavor_type>>::Format
      })?,
    };

    let mut partial_ref_constraints = Punctuated::new();
    let mut selector_constraints = Punctuated::new();
    let use_generics =
      !field.lifetime_params_usages.is_empty() || !field.type_params_usages.is_empty();

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

    if use_generics {
      partial_ref_constraints.push(syn::parse2(quote! {
        #wfr: #object_reflectable
      })?);
      partial_ref_constraints.push(syn::parse2(quote! {
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
    }

    let partial_ref_copyable = object.partial_ref().copy() || field.partial_ref_copy();
    let partial_ref_copy_contraint = if partial_ref_copyable {
      Some(quote! {
        + ::core::marker::Copy
      })
    } else {
      None
    };

    let partial_ref_ty = match ast.ty().or(field.partial_ref_type()) {
      Some(ty) => ty.clone(),
      None => {
        let state_type: Type = syn::parse2(quote! {
          #path_to_grost::__private::convert::State<
            #path_to_grost::__private::convert::PartialRef<
              #lifetime,
              #buffer,
              <#wfr as #object_reflectable>::Reflection,
              #flavor_type,
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

        syn::parse2(quote! {
          <#field_ty as #state_type>::Output
        })?
      }
    };

    let optional_partial_ref_type = syn::parse2(quote! {
      ::core::option::Option<#partial_ref_ty>
    })?;

    let mut wire_format_reflection_constraints = Punctuated::new();

    if use_generics {
      match ast.format() {
        Some(fmt) => {
          wire_format_reflection_constraints.push(syn::parse2(quote! {
            #fmt: #path_to_grost::__private::flavors::WireFormat<#flavor_type>
          })?);
        }
        None => {
          wire_format_reflection_constraints.push(syn::parse2(quote! {
            #field_ty: #path_to_grost::__private::flavors::DefaultWireFormat<#flavor_type>
          })?);

          wire_format_reflection_constraints.push(syn::parse2(quote! {
            <#field_ty as #path_to_grost::__private::flavors::DefaultWireFormat<#flavor_type>>::Format: 'static
          })?);
        }
      }
    }

    let wire_format_reflection =
      wire_format_reflection(path_to_grost, object_ty, flavor_type, tag)?;
    let wire_type_reflection = wire_type_reflection(path_to_grost, object_ty, flavor_type, tag)?;
    let identifier_reflection = identifier_reflection(path_to_grost, object_ty, flavor_type, tag)?;
    let encoded_identifier_reflection =
      encoded_identifier_reflection(path_to_grost, object_ty, flavor_type, tag)?;
    let encoded_identifier_len_reflection =
      encoded_identifier_len_reflection(path_to_grost, object_ty, flavor_type, tag)?;
    let tag_reflection = tag_reflection(path_to_grost, object_ty, flavor_type, tag)?;
    let encoded_tag_reflection =
      encoded_tag_reflection(path_to_grost, object_ty, flavor_type, tag)?;
    let encoded_tag_len_reflection =
      encoded_tag_len_reflection(path_to_grost, object_ty, flavor_type, tag)?;

    Ok(Self {
      wire_format,
      encode: ast.encode().clone(),
      decode: ast.decode().clone(),
      partial_ref: PartialRefFieldFlavor {
        ty: partial_ref_ty,
        optional_type: optional_partial_ref_type,
        constraints: partial_ref_constraints,
      },
      selector: SelectorFieldFlavor {
        ty: selector_type,
        selectable,
        constraints: selector_constraints,
      },
      wire_format_reflection,
      wire_type_reflection,
      identifier_reflection,
      encoded_identifier_reflection,
      encoded_identifier_len_reflection,
      tag_reflection,
      encoded_tag_reflection,
      encoded_tag_len_reflection,
      wire_type_reflection_constraints: wire_format_reflection_constraints.clone(),
      identifier_reflection_constraints: wire_format_reflection_constraints.clone(),
      encoded_identifier_reflection_constraints: wire_format_reflection_constraints.clone(),
      encoded_identifier_len_reflection_constraints: wire_format_reflection_constraints.clone(),
      tag_reflection_constraints: Punctuated::new(),
      encoded_tag_reflection_constraints: Punctuated::new(),
      encoded_tag_len_reflection_constraints: Punctuated::new(),
      wire_format_reflection_constraints,
    })
  }

  pub(super) fn derive<M, F>(
    &self,
    object: &GenericObject<M, F>,
    object_flavor: &ObjectFlavor,
    field: &super::GenericTaggedField<F>,
  ) -> proc_macro2::TokenStream {
    let path_to_grost = object.path_to_grost();
    let object_reflectable = object.reflectable();
    let flavor_ty = object_flavor.flavor_type();
    let wf = self.wire_format();
    let identifier = object_flavor.identifier();
    let identifier_constructor = identifier.constructor();
    let identifier_encode = identifier.encode();
    let tag_constructor = object_flavor.tag().constructor();
    let tag_encode = object_flavor.tag().encode();
    let tag = field.tag();

    let (wire_format_reflection_ig, _, wire_format_reflection_wc) = object_flavor
      .wire_format_reflection_generics()
      .split_for_impl();
    let wire_format_reflection_type = self.wire_format_reflection();
    let (identifier_reflection_ig, _, identifier_reflection_wc) = object_flavor
      .identifier_reflection_generics()
      .split_for_impl();
    let identifier_reflection_type = self.identifier_reflection();
    let (encoded_identifier_reflection_ig, _, encoded_identifier_reflection_wc) = object_flavor
      .encoded_identifier_reflection_generics()
      .split_for_impl();
    let encoded_identifier_reflection_type = self.encoded_identifier_reflection();
    let (encoded_identifier_len_reflection_ig, _, encoded_identifier_len_reflection_wc) =
      object_flavor
        .encoded_identifier_len_reflection_generics()
        .split_for_impl();
    let encoded_identifier_len_reflection_type = self.encoded_identifier_len_reflection();
    let (tag_reflection_ig, _, tag_reflection_wc) =
      object_flavor.tag_reflection_generics().split_for_impl();
    let tag_reflection_type = self.tag_reflection();
    let (encoded_tag_reflection_ig, _, encoded_tag_reflection_wc) = object_flavor
      .encoded_tag_reflection_generics()
      .split_for_impl();
    let encoded_tag_reflection_type = self.encoded_tag_reflection();
    let (encoded_tag_len_reflection_ig, _, encoded_tag_len_reflection_wc) = object_flavor
      .encoded_tag_len_reflection_generics()
      .split_for_impl();
    let encoded_tag_len_reflection_type = self.encoded_tag_len_reflection();
    let (wire_type_reflection_ig, _, wire_type_reflection_wc) = object_flavor
      .wire_type_reflection_generics()
      .split_for_impl();
    let wire_type_reflection_type = self.wire_type_reflection();

    quote! {
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
}
