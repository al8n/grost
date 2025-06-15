use quote::quote;
use syn::{Generics, Type};

use super::ConcreteField;

#[derive(Debug, Clone)]
pub struct ConcreteObjectReflection {
  ty: Type,
  type_reflection_generics: Generics,
  object_reflection_generics: Generics,
}

impl ConcreteObjectReflection {
  /// Returns the type of the concrete object reflection.
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the generics when deriving the `Reflectable` trait for the object.
  #[inline]
  pub const fn object_reflection_generics(&self) -> &Generics {
    &self.object_reflection_generics
  }

  /// Returns the generics when deriving the `Reflectable` trait for the `TypeReflection`.
  #[inline]
  pub const fn type_reflection_generics(&self) -> &Generics {
    &self.type_reflection_generics
  }

  pub(super) fn from_ast<M, F>(
    object: &super::ConcreteObjectAst<M, F>,
    fields: &[ConcreteField<F>],
  ) -> darling::Result<Self> {
    let path_to_grost = object.path_to_grost();
    let flavor_ty = object.flavor().ty();
    let object_type = object.ty();
    let generics = object.generics();
    let mut object_reflection_generics = generics.clone();
    let mut type_reflection_generics = generics.clone();

    let ty: Type = syn::parse2(quote! {
      #path_to_grost::__private::reflection::ObjectReflection<
        #object_type,
        #path_to_grost::__private::reflection::Object,
        #flavor_ty,
      >
    })?;
    fields
      .iter()
      .filter_map(|f| f.try_unwrap_tagged_ref().ok())
      .for_each(|f| {
        let constraints = f.reflection().field_reflection_constraints();
        if !constraints.is_empty() {
          object_reflection_generics
            .make_where_clause()
            .predicates
            .extend(constraints.iter().cloned());
          type_reflection_generics
            .make_where_clause()
            .predicates
            .extend(constraints.iter().cloned());
        }
      });

    Ok(Self {
      ty,
      object_reflection_generics,
      type_reflection_generics,
    })
  }

  pub(super) fn derive<M, F>(
    &self,
    object: &super::ConcreteObject<M, F>,
  ) -> darling::Result<proc_macro2::TokenStream> {
    let name = object.name();
    let path_to_grost = object.path_to_grost();
    let schema_name = object.schema_name();
    let schema_description = object.schema_description();
    let flavor_ty = object.flavor_type();
    let object_reflectable = object.reflectable();
    let object_reflection_ty = self.ty();
    let object_type = object.ty();

    let (ig, tg, wc) = object.generics().split_for_impl();
    let (object_reflection_ig, _, object_reflection_wc) =
      self.object_reflection_generics().split_for_impl();
    let (type_reflection_ig, _, type_reflection_wc) =
      self.type_reflection_generics().split_for_impl();

    let mut field_reflectable_impl = vec![];
    let mut field_reflections = vec![];
    let field_reflection_fns = object
      .fields()
      .iter()
      .filter_map(|f| f.try_unwrap_tagged_ref().ok())
      .map(|f| {
        field_reflections.push(f.reflection().field_reflection_value());
        field_reflectable_impl.push(f.derive_field_reflections(object));

        f.derive_field_reflection_fn(object)
      })
      .collect::<Vec<_>>();

    Ok(quote! {
      #(#field_reflectable_impl)*

      // #[automatically_derived]
      // #[allow(non_camel_case_types, clippy::type_complexity)]
      // impl #field_reflection_ig #object_reflectable for #name #field_reflection_tg #field_reflection_wc {
      //   type Reflection = #path_to_grost::__private::reflection::Type;

      //   const REFLECTION: &'static Self::Reflection = &{
      //     #path_to_grost::__private::reflection::Type::Object(
      //       &#path_to_grost::__private::reflection::ObjectBuilder {
      //         name: #schema_name,
      //         description: #schema_description,
      //         fields: &[
      //           #(&#field_reflections),*
      //         ],
      //       }.build()
      //     )
      //   };
      // }
      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl #type_reflection_ig #object_reflectable for #object_type #type_reflection_wc {
        type Reflection = #path_to_grost::__private::reflection::Type;

        const REFLECTION: &'static Self::Reflection = &{
          #path_to_grost::__private::reflection::Type::Object(
            &#path_to_grost::__private::reflection::ObjectBuilder {
              name: #schema_name,
              description: #schema_description,
              fields: &[
                #(&#field_reflections),*
              ],
            }.build()
          )
        };
      }

      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl #object_reflection_ig #object_reflectable for #object_reflection_ty #object_reflection_wc
      {
        type Reflection = #path_to_grost::__private::reflection::Object;

        const REFLECTION: &'static Self::Reflection = &{
          #path_to_grost::__private::reflection::ObjectBuilder {
            name: #schema_name,
            description: #schema_description,
            fields: &[
              #(&#field_reflections),*
            ],
          }.build()
        };
      }

      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl #ig #name #tg #wc {
        /// Returns the reflection of the struct.
        #[inline]
        pub const fn reflection() -> #path_to_grost::__private::reflection::ObjectReflection<
          Self,
          #path_to_grost::__private::reflection::Object,
          #flavor_ty,
        >
        where
          #flavor_ty: #path_to_grost::__private::flavors::Flavor,
        {
          #path_to_grost::__private::reflection::ObjectReflection::new()
        }

        #(#field_reflection_fns)*
      }
    })
  }
}
