use super::super::*;

impl Network {
  pub(crate) fn derive_reflectable_for_object(
    &self,
    path_to_grost: &syn::Path,
    struct_: &Object,
  ) -> proc_macro2::TokenStream {
    let struct_name = struct_.name();
    let name_str = struct_name.name_str();
    let schema_name = struct_.schema_name();
    let reflection_name = struct_.reflection_name();

    let field_reflection_impls = self.generate_field_reflections(path_to_grost, struct_);
    let field_tag_reflections = self.generate_field_tag_reflections(path_to_grost, struct_);
    let field_encoded_tag_reflections =
      self.generate_field_encoded_tag_reflections(path_to_grost, struct_);
    let field_encoded_tag_len_reflections =
      self.generate_field_encoded_tag_len_reflections(path_to_grost, struct_);
    let field_identifier_reflections =
      self.generate_field_identifier_reflections(path_to_grost, struct_);
    let field_encoded_identifier_len_reflections =
      self.generate_field_encoded_identifier_len_reflections(path_to_grost, struct_);
    let field_encoded_identifier_reflections =
      self.generate_field_encoded_identifier_reflections(path_to_grost, struct_);
    let field_wire_type_reflections =
      self.generate_field_wire_type_reflections(path_to_grost, struct_);
    let field_wire_format_reflections =
      self.generate_field_wire_format_reflections(path_to_grost, struct_);

    let field_reflections = struct_.fields().iter().map(|f| {
      let field_reflection = struct_.field_reflection_name();
      let tag = f.tag();
      quote! {
        <
          #field_reflection<
            #path_to_grost::__private::reflection::ObjectFieldReflection<#path_to_grost::__private::flavors::Network>,
            #path_to_grost::__private::flavors::Network,
            #tag,
          > as #path_to_grost::__private::reflection::Reflectable<
            #path_to_grost::__private::flavors::Network,
          >
        >::REFLECTION
      }
    });

    quote! {
      #(#field_reflection_impls)*
      #(#field_tag_reflections)*
      #(#field_encoded_tag_reflections)*
      #(#field_encoded_tag_len_reflections)*
      #(#field_identifier_reflections)*
      #(#field_encoded_identifier_reflections)*
      #(#field_encoded_identifier_len_reflections)*
      #(#field_wire_type_reflections)*
      #(#field_wire_format_reflections)*

      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl #path_to_grost::__private::reflection::Reflectable<
        #path_to_grost::__private::flavors::Network,
      > for #struct_name {
        type Reflection = #path_to_grost::__private::reflection::ObjectReflection<#path_to_grost::__private::flavors::Network>;
        const REFLECTION: &Self::Reflection = &#path_to_grost::__private::reflection::ObjectReflectionBuilder::<#path_to_grost::__private::flavors::Network> {
          name: #name_str,
          schema_name: #schema_name,
          fields: &[
            #(#field_reflections),*
          ],
        }.build();
      }

      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl #path_to_grost::__private::reflection::Reflectable<
        #path_to_grost::__private::flavors::Network,
      > for #reflection_name<#path_to_grost::__private::reflection::ObjectReflection<#path_to_grost::__private::flavors::Network>, #path_to_grost::__private::flavors::Network> {
        type Reflection = #path_to_grost::__private::reflection::ObjectReflection<#path_to_grost::__private::flavors::Network>;
        const REFLECTION: &Self::Reflection = <
          #struct_name as
            #path_to_grost::__private::reflection::Reflectable<
              #path_to_grost::__private::flavors::Network,
            >
          >::REFLECTION;
      }
    }
  }

  fn generate_field_identifier_reflections<'a>(
    &'a self,
    path_to_grost: &'a syn::Path,
    struct_: &'a Object,
  ) -> impl Iterator<Item = impl ToTokens + 'a> + 'a {
    struct_.fields().iter().map(move |f| {
      let field_reflection = struct_.field_reflection_name();
      let tag = f.tag();

      quote! {
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl #path_to_grost::__private::reflection::Reflectable<
          #path_to_grost::__private::flavors::Network,
        > for #field_reflection<
          #path_to_grost::__private::reflection::IdentifierReflection<#path_to_grost::__private::flavors::network::Identifier>,
          #path_to_grost::__private::flavors::Network,
          #tag,
        >
        {
          type Reflection = #path_to_grost::__private::flavors::network::Identifier;

          const REFLECTION: &Self::Reflection = &{
            <#field_reflection<
                #path_to_grost::__private::reflection::ObjectFieldReflection<#path_to_grost::__private::flavors::Network>,
                #path_to_grost::__private::flavors::Network,
                #tag,
              > as #path_to_grost::__private::reflection::Reflectable<
              #path_to_grost::__private::flavors::Network,
            >>::REFLECTION
              .identifier()
          };
        }
      }
    })
  }

  fn generate_field_wire_type_reflections<'a>(
    &'a self,
    path_to_grost: &'a syn::Path,
    struct_: &'a Object,
  ) -> impl Iterator<Item = impl ToTokens + 'a> + 'a {
    struct_.fields().iter().map(move |f| {
      let field_reflection = struct_.field_reflection_name();
      let tag = f.tag();

      quote! {
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl #path_to_grost::__private::reflection::Reflectable<
          #path_to_grost::__private::flavors::Network,
        > for #field_reflection<
          #path_to_grost::__private::reflection::WireTypeReflection<#path_to_grost::__private::flavors::network::WireType>,
          #path_to_grost::__private::flavors::Network,
          #tag,
        >
        {
          type Reflection = #path_to_grost::__private::flavors::network::WireType;

          const REFLECTION: &#path_to_grost::__private::flavors::network::WireType = &{
            <#field_reflection<
                #path_to_grost::__private::reflection::IdentifierReflection<#path_to_grost::__private::flavors::network::Identifier>,
                #path_to_grost::__private::flavors::Network,
                #tag,
              > as #path_to_grost::__private::reflection::Reflectable<
              #path_to_grost::__private::flavors::Network,
            >>::REFLECTION
              .wire_type()
          };
        }
      }
    })
  }

  fn generate_field_wire_format_reflections<'a>(
    &'a self,
    path_to_grost: &'a syn::Path,
    struct_: &'a Object,
  ) -> impl Iterator<Item = impl ToTokens + 'a> + 'a {
    struct_.fields().iter().map(move |f| {
      let field_reflection = struct_.field_reflection_name();
      let tag = f.tag();
      let wf = f.get_wire_format_or_default(path_to_grost, self);

      quote! {
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl #path_to_grost::__private::reflection::Reflectable<
          #path_to_grost::__private::flavors::Network,
        > for #field_reflection<
          #path_to_grost::__private::reflection::WireFormatReflection,
          #path_to_grost::__private::flavors::Network,
          #tag,
        >
        {
          type Reflection = #wf;

          const REFLECTION: &#wf = &{
            <#wf as #path_to_grost::__private::flavors::WireFormat<#path_to_grost::__private::flavors::Network>>::SELF
          };
        }
      }
    })
  }

  fn generate_field_tag_reflections<'a>(
    &'a self,
    path_to_grost: &'a syn::Path,
    struct_: &'a Object,
  ) -> impl Iterator<Item = impl ToTokens + 'a> + 'a {
    struct_.fields().iter().map(move |f| {
      let field_reflection = struct_.field_reflection_name();
      let tag = f.tag();

      quote! {
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl #path_to_grost::__private::reflection::Reflectable<
          #path_to_grost::__private::flavors::Network,
        > for #field_reflection<
          #path_to_grost::__private::reflection::TagReflection<#path_to_grost::__private::flavors::network::Tag>,
          #path_to_grost::__private::flavors::Network,
          #tag,
        >
        {
          type Reflection = #path_to_grost::__private::flavors::network::Tag;
          const REFLECTION: &Self::Reflection = &{
            <#field_reflection<
              #path_to_grost::__private::reflection::IdentifierReflection<#path_to_grost::__private::flavors::network::Identifier>,
              #path_to_grost::__private::flavors::Network,
              #tag,
            > as #path_to_grost::__private::reflection::Reflectable<
              #path_to_grost::__private::flavors::Network,
            >>::REFLECTION
              .tag()
          };
        }
      }
    })
  }

  fn generate_field_encoded_tag_reflections<'a>(
    &'a self,
    path_to_grost: &'a syn::Path,
    struct_: &'a Object,
  ) -> impl Iterator<Item = impl ToTokens + 'a> + 'a {
    struct_.fields().iter().map(move |f| {
      let field_reflection = struct_.field_reflection_name();
      let tag = f.tag();

      quote! {
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl #path_to_grost::__private::reflection::Reflectable<
          #path_to_grost::__private::flavors::Network,
        > for
            #field_reflection<
              #path_to_grost::__private::reflection::EncodedTagReflection<#path_to_grost::__private::flavors::network::Tag>,
              #path_to_grost::__private::flavors::Network,
              #tag,
            >
        {
          type Reflection = [::core::primitive::u8];

          const REFLECTION: &Self::Reflection = {
            <#field_reflection<
              #path_to_grost::__private::reflection::TagReflection<#path_to_grost::__private::flavors::network::Tag>,
              #path_to_grost::__private::flavors::Network,
              #tag,
            > as #path_to_grost::__private::reflection::Reflectable<
              #path_to_grost::__private::flavors::Network,
            >>::REFLECTION
              .encode()
              .as_slice()
          };
        }
      }
    })
  }

  fn generate_field_encoded_tag_len_reflections<'a>(
    &'a self,
    path_to_grost: &'a syn::Path,
    struct_: &'a Object,
  ) -> impl Iterator<Item = impl ToTokens + 'a> + 'a {
    struct_.fields().iter().map(move |f| {
      let field_reflection = struct_.field_reflection_name();
      let tag = f.tag();

      quote! {
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl #path_to_grost::__private::reflection::Reflectable<
          #path_to_grost::__private::flavors::Network,
        > for
            #field_reflection<
              #path_to_grost::__private::reflection::Len<
                #path_to_grost::__private::reflection::EncodedTagReflection<#path_to_grost::__private::flavors::network::Tag>,
              >,
              #path_to_grost::__private::flavors::Network,
              #tag,
            >
        {
          type Reflection = ::core::primitive::usize;

          const REFLECTION: &Self::Reflection = &{
            <#field_reflection<
              #path_to_grost::__private::reflection::EncodedTagReflection<#path_to_grost::__private::flavors::network::Tag>,
              #path_to_grost::__private::flavors::Network,
              #tag,
            > as #path_to_grost::__private::reflection::Reflectable<
              #path_to_grost::__private::flavors::Network,
            >>::REFLECTION
              .len()
          };
        }
      }
    })
  }

  fn generate_field_encoded_identifier_reflections<'a>(
    &'a self,
    path_to_grost: &'a syn::Path,
    struct_: &'a Object,
  ) -> impl Iterator<Item = impl ToTokens + 'a> + 'a {
    struct_.fields().iter().map(move |f| {
      let field_reflection = struct_.field_reflection_name();
      let tag = f.tag();

      quote! {
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl #path_to_grost::__private::reflection::Reflectable<
          #path_to_grost::__private::flavors::Network,
        > for
            #field_reflection<
              #path_to_grost::__private::reflection::EncodedIdentifierReflection<#path_to_grost::__private::flavors::network::Identifier>,
              #path_to_grost::__private::flavors::Network,
              #tag,
            >
        {
          type Reflection = [::core::primitive::u8];

          const REFLECTION: &Self::Reflection = {
            <#field_reflection<
              #path_to_grost::__private::reflection::IdentifierReflection<#path_to_grost::__private::flavors::network::Identifier>,
              #path_to_grost::__private::flavors::Network,
              #tag,
            > as #path_to_grost::__private::reflection::Reflectable<
              #path_to_grost::__private::flavors::Network,
            >>::REFLECTION
              .encode()
              .as_slice()
          };
        }
      }
    })
  }

  fn generate_field_encoded_identifier_len_reflections<'a>(
    &'a self,
    path_to_grost: &'a syn::Path,
    struct_: &'a Object,
  ) -> impl Iterator<Item = impl ToTokens + 'a> + 'a {
    struct_.fields().iter().map(move |f| {
      let field_reflection = struct_.field_reflection_name();
      let tag = f.tag();

      quote! {
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl #path_to_grost::__private::reflection::Reflectable<
          #path_to_grost::__private::flavors::Network,
        > for
            #field_reflection<
              #path_to_grost::__private::reflection::Len<
                #path_to_grost::__private::reflection::EncodedIdentifierReflection<#path_to_grost::__private::flavors::network::Identifier>,
              >,
              #path_to_grost::__private::flavors::Network,
              #tag,
            >
        {
          type Reflection = ::core::primitive::usize;
          const REFLECTION: &Self::Reflection = &{
            <#field_reflection<
              #path_to_grost::__private::reflection::EncodedIdentifierReflection<#path_to_grost::__private::flavors::network::Identifier>,
              #path_to_grost::__private::flavors::Network,
              #tag,
            > as #path_to_grost::__private::reflection::Reflectable<
              #path_to_grost::__private::flavors::Network,
            >>::REFLECTION
              .len()
          };
        }
      }
    })
  }

  fn generate_field_reflections<'a>(
    &'a self,
    path_to_grost: &'a syn::Path,
    struct_: &'a Object,
  ) -> impl Iterator<Item = impl ToTokens + 'a> + 'a {
    struct_.fields().iter().map(move |f| {
      let field_reflection = struct_.field_reflection_name();
      let tag = f.tag();
      let impl_ = self.field_reflection(path_to_grost, f);

      quote! {
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl #path_to_grost::__private::reflection::Reflectable<
          #path_to_grost::__private::flavors::Network,
        > for #field_reflection<
          #path_to_grost::__private::reflection::ObjectFieldReflection<#path_to_grost::__private::flavors::Network>,
          #path_to_grost::__private::flavors::Network,
          #tag,
        >
        {
          type Reflection = #path_to_grost::__private::reflection::ObjectFieldReflection<#path_to_grost::__private::flavors::Network>;
          const REFLECTION: &Self::Reflection = &#impl_;
        }
      }
    })
  }
}
