use quote::{format_ident, quote};
use syn::{Expr, Type, WherePredicate, punctuated::Punctuated, token::Comma};

use crate::object::{GenericObject, mir::field_reflection};

#[derive(Debug, Clone)]
pub struct GenericFieldReflection {
  field_reflection: Type,
  field_reflection_constraints: Punctuated<WherePredicate, Comma>,
  value: Expr,
}

impl GenericFieldReflection {
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

  /// Returns the constraints for this field reflection.
  #[inline]
  pub const fn field_reflection_constraints(&self) -> &Punctuated<WherePredicate, Comma> {
    &self.field_reflection_constraints
  }

  pub(super) fn try_new<M, F>(
    object: &super::GenericObjectAst<M, F>,
    field: &super::GenericTaggedFieldAst<F>,
    tag: u32,
  ) -> darling::Result<Self>
  where
    F: Clone,
  {
    let path_to_grost = object.path_to_grost();
    let object_type = object.ty();
    let field_ty = field.ty();
    let mut constraints = Punctuated::new();

    if !field.type_params_usages.is_empty() || !field.lifetime_params_usages.is_empty() {
      let pred: WherePredicate = syn::parse2(quote! {
        #path_to_grost::__private::reflection::TypeReflection<#field_ty>: #path_to_grost::__private::reflection::Reflectable<#field_ty, Reflection = #path_to_grost::__private::reflection::Type>
      })?;
      constraints.push(pred);
    }

    let flavor_ident = &object.flavor_param().ident;
    let field_reflection = field_reflection(path_to_grost, object_type, flavor_ident, tag)?;

    let schema_name = field.schema_name();
    let schema_description = field.schema_description();
    let field_reflection_value = syn::parse2(quote! {
      #path_to_grost::__private::reflection::ObjectFieldBuilder {
        name: #schema_name,
        description: #schema_description,
        ty: <#path_to_grost::__private::reflection::TypeReflection<#field_ty> as #path_to_grost::__private::reflection::Reflectable<#field_ty>>::REFLECTION,
      }.build()
    })?;

    Ok(Self {
      field_reflection,
      field_reflection_constraints: constraints,
      value: field_reflection_value,
    })
  }
}

impl<F> super::GenericTaggedField<F> {
  pub(crate) fn derive_field_reflections<M>(
    &self,
    object: &GenericObject<M, F>,
  ) -> proc_macro2::TokenStream {
    let path_to_grost = object.path_to_grost();
    let object_reflectable = object.reflectable();
    let field_ty = self.ty();
    let schema_name = self.schema_name();
    let schema_description = self.schema_description();

    let (field_reflection_ig, _, field_reflection_wc) = object
      .reflection
      .field_reflection_generics()
      .split_for_impl();
    let field_reflection_type = self.reflection().field_reflection();
    let flavor_impls = self.flavors().iter().map(|(name, ff)| {
      let object_flavor = object.flavors().get(name).expect("flavor already checked");
      ff.derive(object, object_flavor, self)
    });

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

      #(#flavor_impls)*
    }
  }

  pub(crate) fn derive_field_reflection_fn<M>(
    &self,
    object: &GenericObject<M, F>,
  ) -> proc_macro2::TokenStream {
    let path_to_grost = object.path_to_grost();
    let object_name = object.name();
    let field_name = self.name();
    let vis = self.vis();
    let doc = format!(" Returns the field reflection of the field `{object_name}.{field_name}`.",);
    let reflection_type = self.reflection().field_reflection();
    let field_reflection_name = format_ident!("{}_reflection", field_name);
    let flavor_ident = &object.flavor_param().ident;

    quote! {
      #[doc = #doc]
      #[inline]
      #vis const fn #field_reflection_name<#flavor_ident>() -> #reflection_type
      where
        #flavor_ident: ?::core::marker::Sized + #path_to_grost::__private::flavors::Flavor,
      {
        #path_to_grost::__private::reflection::ObjectFieldReflection::new()
      }
    }
  }
}
