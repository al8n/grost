use super::super::Network;
use crate::Object;

use quote::{ToTokens, quote};

impl Network {
  pub(super) fn derive_reflection(&self, object: &Object) -> syn::Result<proc_macro2::TokenStream> {
    let field_reflections = self.derive_field_reflections(object);
    let field_identifier_reflections = self.derive_field_identifier_reflections(object);

    Ok(quote! {
      #(#field_reflections)*

      #(#field_identifier_reflections)*
    })
  }

  fn derive_field_reflections<'a>(
    &'a self,
    object: &'a Object,
  ) -> impl Iterator<Item = impl ToTokens + 'a> + 'a {
    let path_to_grost = object.path();
    let reflection_name = object.reflection().name();
    let generics = object.generics();

    object.fields().iter().map(move |f| {
      let tag = f.tag().get();
      let field_name = f.name();
      let field_ty = f.ty();
      let wf = f.wire().map(|t| quote!( #t )).unwrap_or_else(|| {
        quote! {
          <#field_ty as #path_to_grost::__private::flavors::DefaultWireFormat<
            #path_to_grost::__private::flavors::Network
          >>::Format
        }
      });
      let field_name_str = field_name.to_string();
      let schema = f.schema();
      let schema_name = schema.name().unwrap_or(field_name_str.as_str());

      // generics

      quote! {
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl #path_to_grost::__private::reflection::Reflectable<
          #path_to_grost::__private::flavors::Network,
        > for #reflection_name<
          (
            #path_to_grost::__private::reflection::ObjectFieldReflection<#path_to_grost::__private::flavors::Network>,
            #path_to_grost::__private::RawTag<#tag>,
          ),
          #path_to_grost::__private::flavors::Network,
        >
        {
          type Reflection = #path_to_grost::__private::reflection::ObjectFieldReflection<#path_to_grost::__private::flavors::Network>;

          const REFLECTION: &Self::Reflection = &{
            #path_to_grost::__private::reflection::ObjectFieldReflectionBuilder::<#path_to_grost::__private::flavors::Network> {
              identifier: #path_to_grost::__private::flavors::network::Identifier::new(
                <#wf as #path_to_grost::__private::flavors::WireFormat<#path_to_grost::__private::flavors::Network>>::WIRE_TYPE,
                #path_to_grost::__private::flavors::network::Tag::new(#tag),
              ),
              name: #field_name_str,
              ty: ::core::any::type_name::<#field_ty>,
              schema_name: #schema_name,
              schema_type: <#path_to_grost::__private::reflection::SchemaTypeReflection<#field_ty> as #path_to_grost::__private::reflection::Reflectable<#path_to_grost::__private::flavors::Network>>::REFLECTION,
            }.build()
          };
        }
      }
    })
  }

  fn derive_field_identifier_reflections<'a>(
    &'a self,
    object: &'a Object,
  ) -> impl Iterator<Item = impl ToTokens + 'a> + 'a {
    let path_to_grost = object.path();
    let reflection_name = object.reflection().name();
    object.fields().iter().map(move |f| {
      let tag = f.tag().get();

      quote! {
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl #path_to_grost::__private::reflection::Reflectable<
          #path_to_grost::__private::flavors::Network,
        > for #reflection_name<
          (
            #path_to_grost::__private::reflection::IdentifierReflection<
              #path_to_grost::__private::flavors::network::Identifier,
            >,
            #path_to_grost::__private::RawTag<#tag>,
          ),
          #path_to_grost::__private::flavors::Network,
        >
        {
          type Reflection = #path_to_grost::__private::flavors::network::Identifier;

          const REFLECTION: &Self::Reflection = &{
            <#reflection_name<
                (
                  #path_to_grost::__private::reflection::ObjectFieldReflection<#path_to_grost::__private::flavors::Network>,
                  #path_to_grost::__private::RawTag<#tag>,
                ),
                #path_to_grost::__private::flavors::Network,
              > as #path_to_grost::__private::reflection::Reflectable<
              #path_to_grost::__private::flavors::Network,
            >>::REFLECTION
              .identifier()
          };
        }
      }
    })
  }
}
