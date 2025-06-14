use quote::quote;
use syn::{GenericParam, Generics, Type};

#[derive(Debug, Clone)]
pub struct GenericObjectReflection {
  ty: Type,
  field_reflection_generics: Generics,
  object_reflection_generics: Generics,
}

impl GenericObjectReflection {
  /// Returns the type of the generic object reflection.
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the generics of the generic object reflection.
  #[inline]
  pub const fn field_reflection_generics(&self) -> &Generics {
    &self.field_reflection_generics
  }

  /// Returns the generics of the generic object reflection.
  #[inline]
  pub const fn object_reflection_generics(&self) -> &Generics {
    &self.object_reflection_generics
  }

  pub(super) fn from_ast<M, F>(
    object: &super::GenericObjectAst<M, F>,
    fields: &[super::GenericField<F>],
  ) -> darling::Result<Self> {
    let path_to_grost = object.path_to_grost();
    let flavor_param = object.flavor_param();
    let flavor_ident = &flavor_param.ident;
    let object_type = object.ty();
    let generics = object.generics();
    let mut object_reflection_generics = generics.clone();
    let mut field_reflection_generics = generics.clone();
    field_reflection_generics
      .params
      .push(GenericParam::Type(flavor_param.clone()));
    field_reflection_generics
      .make_where_clause()
      .predicates
      .push(syn::parse2(quote! {
        #flavor_ident: ?::core::marker::Sized
      })?);

    let ty: Type = syn::parse2(quote! {
      #path_to_grost::__private::reflection::ObjectReflection<
        #object_type,
        #path_to_grost::__private::reflection::Object,
        #flavor_ident,
      >
    })?;
    fields
      .iter()
      .filter_map(|f| f.try_unwrap_tagged_ref().ok())
      .for_each(|f| {
        macro_rules! bail {
          ($field:ident: $($generics:ident),+$(,)?) => {{
            paste::paste! {
              $(
                [< $generics _generics>].make_where_clause()
                  .predicates
                  .extend($field.reflection(). [< $generics _constraints >]().iter().cloned());
              )*
            }
          }};
        }

        bail!(f:
          field_reflection,
        );

        object_reflection_generics
          .make_where_clause()
          .predicates
          .extend(
            f.reflection()
              .field_reflection_constraints()
              .iter()
              .cloned(),
          );
      });

    Ok(Self {
      ty,
      field_reflection_generics,
      object_reflection_generics,
    })
  }
}

impl<M, F> super::GenericObject<M, F> {
  pub(super) fn derive_reflection(&self) -> proc_macro2::TokenStream {
    let name = self.name();
    let path_to_grost = self.path_to_grost();
    let schema_name = self.schema_name();
    let schema_description = self.schema_description();
    let flavor_ident = &self.flavor_param().ident;
    let object_reflectable = self.reflectable();
    let object_type = self.ty();

    let (ig, tg, wc) = self.generics().split_for_impl();
    let (object_reflection_ig, _, object_reflection_wc) = self
      .reflection
      .object_reflection_generics()
      .split_for_impl();

    let mut field_reflectable_impl = vec![];
    let mut field_reflections = vec![];
    let field_reflection_fns = self
      .fields()
      .iter()
      .filter_map(|f| f.try_unwrap_tagged_ref().ok())
      .map(|f| {
        field_reflections.push(f.reflection().field_reflection_value());
        field_reflectable_impl.push(f.derive_field_reflections(self));

        f.derive_field_reflection_fn(self)
      })
      .collect::<Vec<_>>();

    quote! {
      #(#field_reflectable_impl)*

      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl #object_reflection_ig #object_reflectable for #object_type #object_reflection_wc {
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
        pub const fn reflection<#flavor_ident>() -> #path_to_grost::__private::reflection::ObjectReflection<
          Self,
          #path_to_grost::__private::reflection::Object,
          #flavor_ident,
        >
        where
          #flavor_ident: ?::core::marker::Sized + #path_to_grost::__private::flavors::Flavor,
        {
          #path_to_grost::__private::reflection::ObjectReflection::new()
        }

        #(#field_reflection_fns)*
      }
    }
  }
}
