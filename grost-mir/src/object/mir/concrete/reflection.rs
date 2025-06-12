use quote::quote;
use syn::{Generics, Type};

use super::ConcreteField;

#[derive(Debug, Clone)]
pub struct ConcreteObjectReflection {
  ty: Type,
  field_reflection_generics: Generics,
  wire_format_reflection_generics: Generics,
  wire_type_reflection_generics: Generics,
  identifier_reflection_generics: Generics,
  encoded_identifier_reflection_generics: Generics,
  encoded_identifier_len_reflection_generics: Generics,
  tag_reflection_generics: Generics,
  encoded_tag_reflection_generics: Generics,
  encoded_tag_len_reflection_generics: Generics,
}

impl ConcreteObjectReflection {
  /// Returns the type of the concrete object reflection.
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the generics of the concrete object reflection.
  #[inline]
  pub const fn field_reflection_generics(&self) -> &Generics {
    &self.field_reflection_generics
  }

  /// Returns the generics for the wire format reflection.
  #[inline]
  pub const fn wire_format_reflection_generics(&self) -> &Generics {
    &self.wire_format_reflection_generics
  }

  /// Returns the generics for the wire type reflection.
  #[inline]
  pub const fn wire_type_reflection_generics(&self) -> &Generics {
    &self.wire_type_reflection_generics
  }

  /// Returns the generics for the identifier reflection.
  #[inline]
  pub const fn identifier_reflection_generics(&self) -> &Generics {
    &self.identifier_reflection_generics
  }

  /// Returns the generics for the encoded identifier reflection.
  #[inline]
  pub const fn encoded_identifier_reflection_generics(&self) -> &Generics {
    &self.encoded_identifier_reflection_generics
  }

  /// Returns the generics for the encoded identifier length reflection.
  #[inline]
  pub const fn encoded_identifier_len_reflection_generics(&self) -> &Generics {
    &self.encoded_identifier_len_reflection_generics
  }

  /// Returns the generics for the tag reflection.
  #[inline]
  pub const fn tag_reflection_generics(&self) -> &Generics {
    &self.tag_reflection_generics
  }

  /// Returns the generics for the encoded tag reflection.
  #[inline]
  pub const fn encoded_tag_reflection_generics(&self) -> &Generics {
    &self.encoded_tag_reflection_generics
  }

  /// Returns the generics for the encoded tag length reflection.
  #[inline]
  pub const fn encoded_tag_len_reflection_generics(&self) -> &Generics {
    &self.encoded_tag_len_reflection_generics
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
    let mut field_reflection_generics = generics.clone();
    let mut wire_format_reflection_generics = generics.clone();
    let mut wire_type_reflection_generics = generics.clone();
    let mut identifier_reflection_generics = generics.clone();
    let mut encoded_identifier_reflection_generics = generics.clone();
    let mut encoded_identifier_len_reflection_generics = generics.clone();
    let mut tag_reflection_generics = generics.clone();
    let mut encoded_tag_reflection_generics = generics.clone();
    let mut encoded_tag_len_reflection_generics = generics.clone();

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
          wire_format_reflection,
          wire_type_reflection,
          identifier_reflection,
          encoded_identifier_reflection,
          encoded_identifier_len_reflection,
          tag_reflection,
          encoded_tag_reflection,
          encoded_tag_len_reflection
        );
      });

    Ok(Self {
      ty,
      field_reflection_generics,
      wire_format_reflection_generics,
      wire_type_reflection_generics,
      identifier_reflection_generics,
      encoded_identifier_reflection_generics,
      encoded_identifier_len_reflection_generics,
      tag_reflection_generics,
      encoded_tag_reflection_generics,
      encoded_tag_len_reflection_generics,
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

    let (ig, tg, wc) = object.generics().split_for_impl();
    let (field_reflection_ig, field_reflection_tg, field_reflection_wc) = object
      .reflection
      .field_reflection_generics()
      .split_for_impl();

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
      impl #field_reflection_ig #object_reflectable for #name #field_reflection_tg #field_reflection_wc {
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
