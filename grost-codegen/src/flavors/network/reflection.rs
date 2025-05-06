use quote::ToTokens;
use syn::Ident;

use super::*;

impl Network {
  pub(super) fn generate_field_identifier_reflections<'a>(
    &'a self,
    path_to_grost: &'a syn::Path,
    struct_: &'a Struct,
    reflection_mod_name: &'a Ident,
  ) -> impl Iterator<Item = impl ToTokens + 'a> + 'a {
    struct_.fields().iter().map(move |f| {
      let field_reflection = struct_.field_reflection_name();
      let tag = f.tag();

      quote! {
        impl #path_to_grost::__private::reflection::Reflectable<
          #path_to_grost::__private::flavors::network::Identifier,
          #path_to_grost::__private::flavors::Network,
        > for #reflection_mod_name::#field_reflection<
          #path_to_grost::__private::reflection::IdentifierReflection<#path_to_grost::__private::flavors::network::Identifier>,
          #path_to_grost::__private::flavors::Network,
          #tag,
        >
        {
          const REFLECTION: &#path_to_grost::__private::flavors::network::Identifier = &{
            <#reflection_mod_name::#field_reflection<
                #path_to_grost::__private::reflection::FieldReflection<#path_to_grost::__private::flavors::Network>,
                #path_to_grost::__private::flavors::Network,
                #tag,
              > as #path_to_grost::__private::reflection::Reflectable<
              #path_to_grost::__private::reflection::FieldReflection<#path_to_grost::__private::flavors::Network>,
              #path_to_grost::__private::flavors::Network,
            >>::REFLECTION
              .identifier()
          };
        }

        impl ::core::ops::Deref for #reflection_mod_name::#field_reflection<
            #path_to_grost::__private::reflection::IdentifierReflection<#path_to_grost::__private::flavors::network::Identifier>,
            #path_to_grost::__private::flavors::Network,
            #tag,
          >
        {
          type Target = #path_to_grost::__private::flavors::network::Identifier;

          fn deref(&self) -> &Self::Target {
            <Self as #path_to_grost::__private::reflection::Reflectable<
              #path_to_grost::__private::flavors::network::Identifier,
              #path_to_grost::__private::flavors::Network,
            >>::REFLECTION
          }
        }
      }
    })
  }

  pub(super) fn generate_field_wire_type_reflections<'a>(
    &'a self,
    path_to_grost: &'a syn::Path,
    struct_: &'a Struct,
    reflection_mod_name: &'a Ident,
  ) -> impl Iterator<Item = impl ToTokens + 'a> + 'a {
    struct_.fields().iter().map(move |f| {
      let field_reflection = struct_.field_reflection_name();
      let tag = f.tag();

      quote! {
        impl #path_to_grost::__private::reflection::Reflectable<
          #path_to_grost::__private::flavors::network::WireType,
          #path_to_grost::__private::flavors::Network,
        > for #reflection_mod_name::#field_reflection<
          #path_to_grost::__private::reflection::WireTypeReflection<#path_to_grost::__private::flavors::network::WireType>,
          #path_to_grost::__private::flavors::Network,
          #tag,
        >
        {
          const REFLECTION: &#path_to_grost::__private::flavors::network::WireType = &{
            <#reflection_mod_name::#field_reflection<
                #path_to_grost::__private::reflection::IdentifierReflection<#path_to_grost::__private::flavors::network::Identifier>,
                #path_to_grost::__private::flavors::Network,
                #tag,
              > as #path_to_grost::__private::reflection::Reflectable<
              #path_to_grost::__private::flavors::network::Identifier,
              #path_to_grost::__private::flavors::Network,
            >>::REFLECTION
              .wire_type()
          };
        }

        impl ::core::ops::Deref for #reflection_mod_name::#field_reflection<
          #path_to_grost::__private::reflection::WireTypeReflection<#path_to_grost::__private::flavors::network::WireType>,
          #path_to_grost::__private::flavors::Network,
          #tag,
        >
        {
          type Target = #path_to_grost::__private::flavors::network::WireType;

          fn deref(&self) -> &Self::Target {
            <Self as #path_to_grost::__private::reflection::Reflectable<
              #path_to_grost::__private::flavors::network::WireType,
              #path_to_grost::__private::flavors::Network,
            >>::REFLECTION
          }
        }
      }
    })
  }

  pub(super) fn generate_field_tag_reflections<'a>(
    &'a self,
    path_to_grost: &'a syn::Path,
    struct_: &'a Struct,
    reflection_mod_name: &'a Ident,
  ) -> impl Iterator<Item = impl ToTokens + 'a> + 'a {
    struct_.fields().iter().map(move |f| {
      let field_reflection = struct_.field_reflection_name();
      let tag = f.tag();

      quote! {
        impl #path_to_grost::__private::reflection::Reflectable<
          #path_to_grost::__private::flavors::network::Tag,
          #path_to_grost::__private::flavors::Network,
        > for #reflection_mod_name::#field_reflection<
          #path_to_grost::__private::reflection::TagReflection<#path_to_grost::__private::flavors::network::Tag>,
          #path_to_grost::__private::flavors::Network,
          #tag,
        >
        {
          const REFLECTION: &#path_to_grost::__private::flavors::network::Tag = &{
            <#reflection_mod_name::#field_reflection<
              #path_to_grost::__private::reflection::IdentifierReflection<#path_to_grost::__private::flavors::network::Identifier>,
              #path_to_grost::__private::flavors::Network,
              #tag,
            > as #path_to_grost::__private::reflection::Reflectable<
              #path_to_grost::__private::flavors::network::Identifier,
              #path_to_grost::__private::flavors::Network,
            >>::REFLECTION
              .tag()
          };
        }

        impl ::core::ops::Deref for #reflection_mod_name::#field_reflection<
          #path_to_grost::__private::reflection::TagReflection<#path_to_grost::__private::flavors::network::Tag>,
          #path_to_grost::__private::flavors::Network,
          #tag,
        > {
          type Target = #path_to_grost::__private::flavors::network::Tag;

          fn deref(&self) -> &Self::Target {
            <Self as #path_to_grost::__private::reflection::Reflectable<
              #path_to_grost::__private::flavors::network::Tag,
              #path_to_grost::__private::flavors::Network,
            >>::REFLECTION
          }
        }
      }
    })
  }

  pub(super) fn generate_field_encoded_tag_reflections<'a>(
    &'a self,
    path_to_grost: &'a syn::Path,
    struct_: &'a Struct,
    reflection_mod_name: &'a Ident,
  ) -> impl Iterator<Item = impl ToTokens + 'a> + 'a {
    struct_.fields().iter().map(move |f| {
      let field_reflection = struct_.field_reflection_name();
      let tag = f.tag();

      quote! {
        impl #path_to_grost::__private::reflection::Reflectable<
          [::core::primitive::u8],
          #path_to_grost::__private::flavors::Network,
        > for 
            #reflection_mod_name::#field_reflection<
              #path_to_grost::__private::reflection::EncodedTagReflection<#path_to_grost::__private::flavors::network::Tag>,
              #path_to_grost::__private::flavors::Network,
              #tag,
            >
        {
          const REFLECTION: &[::core::primitive::u8] = {
            <#reflection_mod_name::#field_reflection<
              #path_to_grost::__private::reflection::TagReflection<#path_to_grost::__private::flavors::network::Tag>,
              #path_to_grost::__private::flavors::Network,
              #tag,
            > as #path_to_grost::__private::reflection::Reflectable<
              #path_to_grost::__private::flavors::network::Tag,
              #path_to_grost::__private::flavors::Network,
            >>::REFLECTION
              .encode()
              .as_slice()
          };
        }

        impl ::core::ops::Deref for #reflection_mod_name::#field_reflection<
          #path_to_grost::__private::reflection::EncodedTagReflection<#path_to_grost::__private::flavors::network::Tag>,
          #path_to_grost::__private::flavors::Network,
          #tag,
        >
        {
          type Target = [::core::primitive::u8];

          fn deref(&self) -> &Self::Target {
            <Self as #path_to_grost::__private::reflection::Reflectable<
              [::core::primitive::u8],
              #path_to_grost::__private::flavors::Network,
            >>::REFLECTION
          }
        }
      }
    })
  }

  pub(super) fn generate_field_encoded_tag_len_reflections<'a>(
    &'a self,
    path_to_grost: &'a syn::Path,
    struct_: &'a Struct,
    reflection_mod_name: &'a Ident,
  ) -> impl Iterator<Item = impl ToTokens + 'a> + 'a {
    struct_.fields().iter().map(move |f| {
      let field_reflection = struct_.field_reflection_name();
      let tag = f.tag();

      quote! {
        impl #path_to_grost::__private::reflection::Reflectable<
          ::core::primitive::usize,
          #path_to_grost::__private::flavors::Network,
        > for 
            #reflection_mod_name::#field_reflection<
              #path_to_grost::__private::reflection::Len<
                #path_to_grost::__private::reflection::EncodedTagReflection<#path_to_grost::__private::flavors::network::Tag>,
              >,
              #path_to_grost::__private::flavors::Network,
              #tag,
            >
        {
          const REFLECTION: &::core::primitive::usize = &{
            <#reflection_mod_name::#field_reflection<
              #path_to_grost::__private::reflection::EncodedTagReflection<#path_to_grost::__private::flavors::network::Tag>,
              #path_to_grost::__private::flavors::Network,
              #tag,
            > as #path_to_grost::__private::reflection::Reflectable<
              [::core::primitive::u8],
              #path_to_grost::__private::flavors::Network,
            >>::REFLECTION
              .len()
          };
        }

        impl ::core::ops::Deref for #reflection_mod_name::#field_reflection<
          #path_to_grost::__private::reflection::Len<
            #path_to_grost::__private::reflection::EncodedTagReflection<#path_to_grost::__private::flavors::network::Tag>,
          >,
          #path_to_grost::__private::flavors::Network,
          #tag,
        >
        {
          type Target = ::core::primitive::usize;

          fn deref(&self) -> &Self::Target {
            <Self as #path_to_grost::__private::reflection::Reflectable<
              ::core::primitive::usize,
              #path_to_grost::__private::flavors::Network,
            >>::REFLECTION
          }
        }
      }
    })
  }

  pub(super) fn generate_field_encoded_identifier_reflections<'a>(
    &'a self,
    path_to_grost: &'a syn::Path,
    struct_: &'a Struct,
    reflection_mod_name: &'a Ident,
  ) -> impl Iterator<Item = impl ToTokens + 'a> + 'a {
    struct_.fields().iter().map(move |f| {
      let field_reflection = struct_.field_reflection_name();
      let tag = f.tag();

      quote! {
        impl #path_to_grost::__private::reflection::Reflectable<
          [::core::primitive::u8],
          #path_to_grost::__private::flavors::Network,
        > for 
            #reflection_mod_name::#field_reflection<
              #path_to_grost::__private::reflection::EncodedIdentifierReflection<#path_to_grost::__private::flavors::network::Identifier>,
              #path_to_grost::__private::flavors::Network,
              #tag,
            >
        {
          const REFLECTION: &[::core::primitive::u8] = {
            <#reflection_mod_name::#field_reflection<
              #path_to_grost::__private::reflection::IdentifierReflection<#path_to_grost::__private::flavors::network::Identifier>,
              #path_to_grost::__private::flavors::Network,
              #tag,
            > as #path_to_grost::__private::reflection::Reflectable<
              #path_to_grost::__private::flavors::network::Identifier,
              #path_to_grost::__private::flavors::Network,
            >>::REFLECTION
              .encode()
              .as_slice()
          };
        }

        impl ::core::ops::Deref for #reflection_mod_name::#field_reflection<
          #path_to_grost::__private::reflection::EncodedIdentifierReflection<#path_to_grost::__private::flavors::network::Identifier>,
          #path_to_grost::__private::flavors::Network,
          #tag,
        >
        {
          type Target = [::core::primitive::u8];

          fn deref(&self) -> &Self::Target {
            <Self as #path_to_grost::__private::reflection::Reflectable<
              [::core::primitive::u8],
              #path_to_grost::__private::flavors::Network,
            >>::REFLECTION
          }
        }
      }
    })
  }

  pub(super) fn generate_field_encoded_identifier_len_reflections<'a>(
    &'a self,
    path_to_grost: &'a syn::Path,
    struct_: &'a Struct,
    reflection_mod_name: &'a Ident,
  ) -> impl Iterator<Item = impl ToTokens + 'a> + 'a {
    struct_.fields().iter().map(move |f| {
      let field_reflection = struct_.field_reflection_name();
      let tag = f.tag();

      quote! {
        impl #path_to_grost::__private::reflection::Reflectable<
          ::core::primitive::usize,
          #path_to_grost::__private::flavors::Network,
        > for 
            #reflection_mod_name::#field_reflection<
              #path_to_grost::__private::reflection::Len<
                #path_to_grost::__private::reflection::EncodedIdentifierReflection<#path_to_grost::__private::flavors::network::Identifier>,
              >,
              #path_to_grost::__private::flavors::Network,
              #tag,
            >
        {
          const REFLECTION: &::core::primitive::usize = &{
            <#reflection_mod_name::#field_reflection<
              #path_to_grost::__private::reflection::EncodedIdentifierReflection<#path_to_grost::__private::flavors::network::Identifier>,
              #path_to_grost::__private::flavors::Network,
              #tag,
            > as #path_to_grost::__private::reflection::Reflectable<
              [::core::primitive::u8],
              #path_to_grost::__private::flavors::Network,
            >>::REFLECTION
              .len()
          };
        }

        impl ::core::ops::Deref for #reflection_mod_name::#field_reflection<
          #path_to_grost::__private::reflection::Len<
            #path_to_grost::__private::reflection::EncodedIdentifierReflection<#path_to_grost::__private::flavors::network::Identifier>,
          >,
          #path_to_grost::__private::flavors::Network,
          #tag,
        >
        {
          type Target = ::core::primitive::usize;

          fn deref(&self) -> &Self::Target {
            <Self as #path_to_grost::__private::reflection::Reflectable<
              ::core::primitive::usize,
              #path_to_grost::__private::flavors::Network,
            >>::REFLECTION
          }
        }
      }
    })
  }

  pub(super) fn generate_field_reflections<'a>(
    &'a self,
    path_to_grost: &'a syn::Path,
    struct_: &'a Struct,
    reflection_mod_name: &'a Ident,
  ) -> impl Iterator<Item = impl ToTokens + 'a> + 'a {
    struct_.fields().iter().map(move |f| {
      let field_reflection = struct_.field_reflection_name();
      let tag = f.tag();
      let impl_ = f.field_reflection(path_to_grost, self);

      quote! {
        impl ::core::ops::Deref for 
          #reflection_mod_name::#field_reflection<
            #path_to_grost::__private::reflection::FieldReflection<#path_to_grost::__private::flavors::Network>,
            #path_to_grost::__private::flavors::Network,
            #tag,
          >
        {
          type Target = #path_to_grost::__private::reflection::FieldReflection<#path_to_grost::__private::flavors::network::Network>;

          fn deref(&self) -> &Self::Target {
            <Self as #path_to_grost::__private::reflection::Reflectable<
              #path_to_grost::__private::reflection::FieldReflection<#path_to_grost::__private::flavors::Network>,
              #path_to_grost::__private::flavors::Network,
            >>::REFLECTION
          }
        }

        impl #path_to_grost::__private::reflection::Reflectable<
          #path_to_grost::__private::reflection::FieldReflection<#path_to_grost::__private::flavors::Network>,
          #path_to_grost::__private::flavors::Network,
        > for #reflection_mod_name::#field_reflection<
          #path_to_grost::__private::reflection::FieldReflection<#path_to_grost::__private::flavors::Network>,
          #path_to_grost::__private::flavors::Network,
          #tag,
        >
        {
          const REFLECTION: &#path_to_grost::__private::reflection::FieldReflection<#path_to_grost::__private::flavors::network::Network> = &#impl_;
        }
      }
    })
  }
}
