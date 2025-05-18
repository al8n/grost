use super::super::Network;
use crate::Object;

use quote::{ToTokens, quote};

impl Network {
  pub(super) fn derive_reflection(&self, object: &Object) -> syn::Result<proc_macro2::TokenStream> {
    let field_identifier_reflections = derive_field_identifier_reflections(object);
    let field_encoded_identifier_reflections = derive_field_encoded_identifier_reflections(object);
    let field_encoded_identifier_len_reflections =
      derive_field_encoded_identifier_len_reflections(object);
    let field_tag_reflections = derive_field_tag_reflections(object);
    let field_encoded_tag_reflections = derive_field_encoded_tag_reflections(object);
    let field_encoded_tag_len_reflections = derive_field_encoded_tag_len_reflections(object);
    let field_wire_type_reflections = derive_field_wire_type_reflections(object);
    let field_wire_format_reflections = derive_field_wire_format_reflections(object);

    Ok(quote! {
      #(#field_identifier_reflections)*
      #(#field_encoded_identifier_reflections)*
      #(#field_encoded_identifier_len_reflections)*
      #(#field_tag_reflections)*
      #(#field_encoded_tag_reflections)*
      #(#field_encoded_tag_len_reflections)*
      #(#field_wire_type_reflections)*
      #(#field_wire_format_reflections)*
    })
  }
}

fn derive_field_wire_format_reflections<'a>(
  object: &'a Object,
) -> impl Iterator<Item = impl ToTokens + 'a> + 'a {
  let path_to_grost = object.path();
  let name = object.name();
  let generics = object.generics();
  let (ig, tg, wc) = generics.split_for_impl();
  object.fields().iter().map(move |f| {
    let tag = f.tag().get();
    let ty = f.ty();
    let wf = f.wire().map(|wt| quote!(#wt)).unwrap_or_else(|| quote! {
      <#ty as #path_to_grost::__private::flavors::DefaultWireFormat<#path_to_grost::__private::flavors::Network>>::Format
    });

    quote! {
      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl #ig #path_to_grost::__private::reflection::Reflectable<
        #name #tg
      > for #path_to_grost::__private::reflection::Reflection<
        #name #tg,
        #path_to_grost::__private::reflection::Identified<
          #path_to_grost::__private::reflection::WireFormatReflection,
          #tag,
        >,
        #path_to_grost::__private::flavors::Network,
      >
      #wc
      {
        type Reflection = #wf;

        const REFLECTION: &'static Self::Reflection = &{
          <#wf as #path_to_grost::__private::flavors::WireFormat<#path_to_grost::__private::flavors::Network>>::SELF
        };
      }
    }
  })
}

fn derive_field_identifier_reflections<'a>(
  object: &'a Object,
) -> impl Iterator<Item = impl ToTokens + 'a> + 'a {
  let path_to_grost = object.path();
  let name = object.name();
  let generics = object.generics();
  let (ig, tg, wc) = generics.split_for_impl();
  object.fields().iter().map(move |f| {
      let tag = f.tag().get();
      let ty = f.ty();
      let wf = f.wire().map(|wt| quote!(#wt)).unwrap_or_else(|| quote! {
        <#ty as #path_to_grost::__private::flavors::DefaultWireFormat<#path_to_grost::__private::flavors::Network>>::Format
      });

      quote! {
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl #ig #path_to_grost::__private::reflection::Reflectable<
          #name #tg
        > for #path_to_grost::__private::reflection::Reflection<
          #name #tg,
          #path_to_grost::__private::reflection::Identified<
            #path_to_grost::__private::reflection::IdentifierReflection<#path_to_grost::__private::flavors::network::Identifier>,
            #tag,
          >,
          #path_to_grost::__private::flavors::Network,
        >
        #wc
        {
          type Reflection = #path_to_grost::__private::flavors::network::Identifier;

          const REFLECTION: &Self::Reflection = &{
            #path_to_grost::__private::flavors::network::Identifier::new(
              <#wf as #path_to_grost::__private::flavors::WireFormat<#path_to_grost::__private::flavors::Network>>::WIRE_TYPE,
              #path_to_grost::__private::flavors::network::Tag::new(#tag),
            )
          };
        }
      }
    })
}

fn derive_field_encoded_identifier_reflections<'a>(
  object: &'a Object,
) -> impl Iterator<Item = impl ToTokens + 'a> + 'a {
  let path_to_grost = object.path();
  let name = object.name();
  let generics = object.generics();
  let (ig, tg, wc) = generics.split_for_impl();
  object.fields().iter().map(move |f| {
      let tag = f.tag().get();
      quote! {
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl #ig #path_to_grost::__private::reflection::Reflectable<
          #name #tg
        > for #path_to_grost::__private::reflection::Reflection<
          #name #tg,
          #path_to_grost::__private::reflection::Identified<
            #path_to_grost::__private::reflection::EncodeReflection<
              #path_to_grost::__private::reflection::IdentifierReflection<
                #path_to_grost::__private::flavors::network::Identifier,
              >
            >,
            #tag,
          >,
          #path_to_grost::__private::flavors::Network,
        >
        #wc
        {
          type Reflection = [::core::primitive::u8];

          const REFLECTION: &Self::Reflection = {
            <
              #path_to_grost::__private::reflection::Reflection<
                #name #tg,
                #path_to_grost::__private::reflection::Identified<
                  #path_to_grost::__private::reflection::IdentifierReflection<#path_to_grost::__private::flavors::network::Identifier>,
                  #tag,
                >,
                #path_to_grost::__private::flavors::Network,
              > as #path_to_grost::__private::reflection::Reflectable<
                #name #tg,
              >
            >::REFLECTION
              .encode()
              .as_slice()
          };
        }
      }
    })
}

fn derive_field_encoded_identifier_len_reflections<'a>(
  object: &'a Object,
) -> impl Iterator<Item = impl ToTokens + 'a> + 'a {
  let path_to_grost = object.path();
  let name = object.name();
  let generics = object.generics();
  let (ig, tg, wc) = generics.split_for_impl();
  object.fields().iter().map(move |f| {
    let tag = f.tag().get();
    quote! {
      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl #ig #path_to_grost::__private::reflection::Reflectable<
        #name #tg
      > for #path_to_grost::__private::reflection::Reflection<
        #name #tg,
        #path_to_grost::__private::reflection::Identified<
          #path_to_grost::__private::reflection::EncodeReflection<
            #path_to_grost::__private::reflection::Len<
              #path_to_grost::__private::reflection::IdentifierReflection<
                #path_to_grost::__private::flavors::network::Identifier,
              >,
            >,
          >,
          #tag,
        >,
        #path_to_grost::__private::flavors::Network,
      >
      #wc
      {
        type Reflection = ::core::primitive::usize;

        const REFLECTION: &Self::Reflection = &{
          <
            #path_to_grost::__private::reflection::Reflection<
              #name #tg,
              #path_to_grost::__private::reflection::Identified<
                #path_to_grost::__private::reflection::EncodeReflection<
                  #path_to_grost::__private::reflection::IdentifierReflection<
                    #path_to_grost::__private::flavors::network::Identifier,
                  >
                >,
                #tag,
              >,
              #path_to_grost::__private::flavors::Network,
            > as #path_to_grost::__private::reflection::Reflectable<
              #name #tg,
            >
          >::REFLECTION
            .len()
        };
      }
    }
  })
}

fn derive_field_tag_reflections<'a>(
  object: &'a Object,
) -> impl Iterator<Item = impl ToTokens + 'a> + 'a {
  let path_to_grost = object.path();
  let name = object.name();
  let generics = object.generics();
  let (ig, tg, wc) = generics.split_for_impl();
  object.fields().iter().map(move |f| {
      let tag = f.tag().get();
      quote! {
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl #ig #path_to_grost::__private::reflection::Reflectable<
          #name #tg
        > for #path_to_grost::__private::reflection::Reflection<
          #name #tg,
          #path_to_grost::__private::reflection::Identified<
            #path_to_grost::__private::reflection::TagReflection<#path_to_grost::__private::flavors::network::Tag>,
            #tag,
          >,
          #path_to_grost::__private::flavors::Network,
        >
        #wc
        {
          type Reflection = #path_to_grost::__private::flavors::network::Tag;

          const REFLECTION: &Self::Reflection = &{
            <
              #path_to_grost::__private::reflection::Reflection<
                #name #tg,
                #path_to_grost::__private::reflection::Identified<
                  #path_to_grost::__private::reflection::IdentifierReflection<#path_to_grost::__private::flavors::network::Identifier>,
                  #tag,
                >,
                #path_to_grost::__private::flavors::Network,
              > as #path_to_grost::__private::reflection::Reflectable<
                #name #tg,
              >
            >::REFLECTION.tag()
          };
        }
      }
    })
}

fn derive_field_encoded_tag_reflections<'a>(
  object: &'a Object,
) -> impl Iterator<Item = impl ToTokens + 'a> + 'a {
  let path_to_grost = object.path();
  let name = object.name();
  let generics = object.generics();
  let (ig, tg, wc) = generics.split_for_impl();
  object.fields().iter().map(move |f| {
      let tag = f.tag().get();
      quote! {
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl #ig #path_to_grost::__private::reflection::Reflectable<
          #name #tg
        > for #path_to_grost::__private::reflection::Reflection<
          #name #tg,
          #path_to_grost::__private::reflection::Identified<
            #path_to_grost::__private::reflection::EncodeReflection<
              #path_to_grost::__private::reflection::TagReflection<
                #path_to_grost::__private::flavors::network::Tag
              >
            >,
            #tag,
          >,
          #path_to_grost::__private::flavors::Network,
        >
        #wc
        {
          type Reflection = [::core::primitive::u8];

          const REFLECTION: &Self::Reflection = {
            <
              #path_to_grost::__private::reflection::Reflection<
                #name #tg,
                #path_to_grost::__private::reflection::Identified<
                  #path_to_grost::__private::reflection::TagReflection<#path_to_grost::__private::flavors::network::Tag>,
                  #tag,
                >,
                #path_to_grost::__private::flavors::Network,
              > as #path_to_grost::__private::reflection::Reflectable<
                #name #tg,
              >
            >::REFLECTION
            .encode()
            .as_slice()
          };
        }
      }
    })
}

fn derive_field_encoded_tag_len_reflections<'a>(
  object: &'a Object,
) -> impl Iterator<Item = impl ToTokens + 'a> + 'a {
  let path_to_grost = object.path();
  let name = object.name();
  let generics = object.generics();
  let (ig, tg, wc) = generics.split_for_impl();
  object.fields().iter().map(move |f| {
    let tag = f.tag().get();
    quote! {
      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl #ig #path_to_grost::__private::reflection::Reflectable<
        #name #tg
      > for #path_to_grost::__private::reflection::Reflection<
        #name #tg,
        #path_to_grost::__private::reflection::Identified<
          #path_to_grost::__private::reflection::EncodeReflection<
            #path_to_grost::__private::reflection::Len<
              #path_to_grost::__private::reflection::TagReflection<
                #path_to_grost::__private::flavors::network::Tag
              >
            >,
          >,
          #tag,
        >,
        #path_to_grost::__private::flavors::Network,
      >
      #wc
      {
        type Reflection = ::core::primitive::usize;

        const REFLECTION: &Self::Reflection = &{
          <
            #path_to_grost::__private::reflection::Reflection<
              #name #tg,
              #path_to_grost::__private::reflection::Identified<
                #path_to_grost::__private::reflection::EncodeReflection<
                  #path_to_grost::__private::reflection::TagReflection<
                    #path_to_grost::__private::flavors::network::Tag
                  >
                >,
                #tag,
              >,
              #path_to_grost::__private::flavors::Network,
            > as #path_to_grost::__private::reflection::Reflectable<
              #name #tg,
            >
          >::REFLECTION
          .len()
        };
      }
    }
  })
}

fn derive_field_wire_type_reflections<'a>(
  object: &'a Object,
) -> impl Iterator<Item = impl ToTokens + 'a> + 'a {
  let path_to_grost = object.path();
  let name = object.name();
  let generics = object.generics();
  let (ig, tg, wc) = generics.split_for_impl();
  object.fields().iter().map(move |f| {
      let tag = f.tag().get();
      quote! {
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl #ig #path_to_grost::__private::reflection::Reflectable<
          #name #tg
        > for #path_to_grost::__private::reflection::Reflection<
          #name #tg,
          #path_to_grost::__private::reflection::Identified<
            #path_to_grost::__private::reflection::WireTypeReflection<#path_to_grost::__private::flavors::network::WireType>,
            #tag,
          >,
          #path_to_grost::__private::flavors::Network,
        >
        #wc
        {
          type Reflection = #path_to_grost::__private::flavors::network::WireType;

          const REFLECTION: &Self::Reflection = &{
            <
              #path_to_grost::__private::reflection::Reflection<
                #name #tg,
                #path_to_grost::__private::reflection::Identified<
                  #path_to_grost::__private::reflection::IdentifierReflection<#path_to_grost::__private::flavors::network::Identifier>,
                  #tag,
                >,
                #path_to_grost::__private::flavors::Network,
              > as #path_to_grost::__private::reflection::Reflectable<
                #name #tg,
              >
            >::REFLECTION.wire_type()
          };
        }
      }
    })
}
