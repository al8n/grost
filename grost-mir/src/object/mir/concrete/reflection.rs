use quote::quote;
use syn::{Generics, Type};

use super::ConcreteField;

#[derive(Debug, Clone)]
pub struct ConcreteObjectReflection {
  ty: Type,
  generics: Generics,
}

impl ConcreteObjectReflection {
  /// Returns the type of the concrete object reflection.
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the generics of the concrete object reflection.
  #[inline]
  pub const fn generics(&self) -> &Generics {
    &self.generics
  }

  pub(super) fn from_ast<M, F>(
    object: &super::ConcreteObjectAst<M, F>,
    fields: &[ConcreteField<F>],
  ) -> darling::Result<Self> {
    let path_to_grost = object.path_to_grost();
    let name = object.name();
    let flavor_ty = object.flavor().ty();
    let generics = object.generics();
    let (_, tg, _) = generics.split_for_impl();
    let mut reflection_generics = generics.clone();

    let ty: Type = syn::parse2(quote! {
      #path_to_grost::__private::reflection::ObjectReflection<
        #name #tg,
        #path_to_grost::__private::reflection::Object,
        #flavor_ty,
      >
    })?;
    fields
      .iter()
      .filter_map(|f| f.try_unwrap_tagged_ref().ok())
      .for_each(|f| {
        reflection_generics
          .make_where_clause()
          .predicates
          .extend(f.reflection().constraints().iter().cloned());
      });

    Ok(Self {
      ty,
      generics: reflection_generics,
    })
  }

  pub(super) fn derive<M, F>(
    &self,
    object: &super::ConcreteObject<M, F>,
  ) -> darling::Result<proc_macro2::TokenStream> {
    let name = object.name();
    let path_to_grost = object.path_to_grost();
    let (ig, tg, wc) = self.generics().split_for_impl();
    let (_, tg_reflection, wc_reflection) = object.reflection.generics().split_for_impl();
    let schema_name = object.schema_name();
    let schema_description = object.schema_description();
    let flavor_ty = object.flavor_type();
    let object_reflectable = object.reflectable();

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

      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl #ig #object_reflectable for #name #tg_reflection #wc_reflection {
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
