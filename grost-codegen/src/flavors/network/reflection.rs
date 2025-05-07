use quote::ToTokens;

use super::*;

impl Network {
  pub(super) fn generate_field_identifier_reflections<'a>(
    &'a self,
    path_to_grost: &'a syn::Path,
    struct_: &'a Struct,
  ) -> impl Iterator<Item = impl ToTokens + 'a> + 'a {
    struct_.fields().iter().map(move |f| {
      let field_reflection = struct_.field_reflection_name();
      let tag = f.tag();

      quote! {
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
                #path_to_grost::__private::reflection::FieldReflection<#path_to_grost::__private::flavors::Network>,
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

  pub(super) fn generate_field_wire_type_reflections<'a>(
    &'a self,
    path_to_grost: &'a syn::Path,
    struct_: &'a Struct,
  ) -> impl Iterator<Item = impl ToTokens + 'a> + 'a {
    struct_.fields().iter().map(move |f| {
      let field_reflection = struct_.field_reflection_name();
      let tag = f.tag();

      quote! {
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

  pub(super) fn generate_field_tag_reflections<'a>(
    &'a self,
    path_to_grost: &'a syn::Path,
    struct_: &'a Struct,
  ) -> impl Iterator<Item = impl ToTokens + 'a> + 'a {
    struct_.fields().iter().map(move |f| {
      let field_reflection = struct_.field_reflection_name();
      let tag = f.tag();

      quote! {
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

  pub(super) fn generate_field_encoded_tag_reflections<'a>(
    &'a self,
    path_to_grost: &'a syn::Path,
    struct_: &'a Struct,
  ) -> impl Iterator<Item = impl ToTokens + 'a> + 'a {
    struct_.fields().iter().map(move |f| {
      let field_reflection = struct_.field_reflection_name();
      let tag = f.tag();

      quote! {
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

  pub(super) fn generate_field_encoded_tag_len_reflections<'a>(
    &'a self,
    path_to_grost: &'a syn::Path,
    struct_: &'a Struct,
  ) -> impl Iterator<Item = impl ToTokens + 'a> + 'a {
    struct_.fields().iter().map(move |f| {
      let field_reflection = struct_.field_reflection_name();
      let tag = f.tag();

      quote! {
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

  pub(super) fn generate_field_encoded_identifier_reflections<'a>(
    &'a self,
    path_to_grost: &'a syn::Path,
    struct_: &'a Struct,
  ) -> impl Iterator<Item = impl ToTokens + 'a> + 'a {
    struct_.fields().iter().map(move |f| {
      let field_reflection = struct_.field_reflection_name();
      let tag = f.tag();

      quote! {
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

  pub(super) fn generate_field_encoded_identifier_len_reflections<'a>(
    &'a self,
    path_to_grost: &'a syn::Path,
    struct_: &'a Struct,
  ) -> impl Iterator<Item = impl ToTokens + 'a> + 'a {
    struct_.fields().iter().map(move |f| {
      let field_reflection = struct_.field_reflection_name();
      let tag = f.tag();

      quote! {
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

  pub(super) fn generate_field_reflections<'a>(
    &'a self,
    path_to_grost: &'a syn::Path,
    struct_: &'a Struct,
  ) -> impl Iterator<Item = impl ToTokens + 'a> + 'a {
    struct_.fields().iter().map(move |f| {
      let field_reflection = struct_.field_reflection_name();
      let tag = f.tag();
      let impl_ = f.field_reflection(path_to_grost, self);

      quote! {
        impl #path_to_grost::__private::reflection::Reflectable<
          #path_to_grost::__private::flavors::Network,
        > for #field_reflection<
          #path_to_grost::__private::reflection::FieldReflection<#path_to_grost::__private::flavors::Network>,
          #path_to_grost::__private::flavors::Network,
          #tag,
        >
        {
          type Reflection = #path_to_grost::__private::reflection::FieldReflection<#path_to_grost::__private::flavors::Network>;
          const REFLECTION: &Self::Reflection = &#impl_;
        }
      }
    })
  }
}
