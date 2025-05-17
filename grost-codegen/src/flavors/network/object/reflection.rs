use super::super::Network;
use crate::Object;

use quote::{ToTokens, quote};

impl Network {
  pub(super) fn derive_reflection(&self) -> syn::Result<proc_macro2::TokenStream> {
    Ok(quote! {})
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
