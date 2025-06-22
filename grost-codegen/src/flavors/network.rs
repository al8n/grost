use crate::Object;

use super::FlavorGenerator;

// use crate::Field;

// use super::{Enum, FlavorGenerator, Object};

mod object;

#[derive(Clone)]
pub struct Network {
  ty: syn::Type,
  name: &'static str,
}

impl Network {
  /// Returns a new `Network` flavor
  pub fn new(path_to_grost: &syn::Path) -> Self {
    let ty = syn::parse_quote!(#path_to_grost::__private::flavors::Network);
    Self {
      ty,
      name: "Network",
    }
  }

  // fn field_identifier(&self, path_to_grost: &syn::Path, field: &Field) -> proc_macro2::TokenStream {
  //   let tag = field.tag();
  //   let wf = field.get_wire_format_or_default(path_to_grost, self);

  //   quote! {
  //     #path_to_grost::__private::flavors::network::Identifier::new(
  //       <#wf as #path_to_grost::__private::flavors::WireFormat<#path_to_grost::__private::flavors::Network>>::WIRE_TYPE,
  //       #path_to_grost::__private::flavors::network::Tag::new(#tag),
  //     )
  //   }
  // }

  // fn field_reflection(&self, path_to_grost: &syn::Path, field: &Field) -> proc_macro2::TokenStream {
  //   let field_name: &str = field.name().name_str();
  //   let flavor_ty = self.ty();
  //   let field_ty = field.ty().ty();
  //   let schema_name = field.schema_name();
  //   let relection_ty = field.ty().to_type_reflection(path_to_grost, self);

  //   let identifier = self.field_identifier(path_to_grost, field);
  //   quote! {
  //     #path_to_grost::__private::reflection::ObjectFieldBuilder::<#flavor_ty> {
  //       identifier: #identifier,
  //       name: #field_name,
  //       ty: ::core::any::type_name::<#field_ty>,
  //       schema_name: #schema_name,
  //       schema_type: #relection_ty,
  //     }.build()
  //   }
  // }

  // fn derive_default_format(
  //   &self,
  //   path_to_grost: &syn::Path,
  //   struct_name: &syn::Ident,
  // ) -> proc_macro2::TokenStream {
  //   let ty = self.ty();
  //   quote! {
  //     #[automatically_derived]
  //     impl #path_to_grost::__private::flavors::DefaultWireFormat<#ty> for #struct_name {
  //       type Format = #path_to_grost::__private::flavors::network::LengthDelimited;
  //     }
  //   }
  // }

  // fn derive_encoded_state(
  //   &self,
  //   path_to_grost: &syn::Path,
  //   struct_name: &syn::Ident,
  //   partial_ref_struct_name: &syn::Ident,
  // ) -> proc_macro2::TokenStream {
  //   quote! {
  //     #path_to_grost::__private::decoded_state!(
  //       &'__grost_lifetime__ #path_to_grost::__private::flavors::Network: #struct_name
  //         as #path_to_grost::__private::flavors::network::LengthDelimited
  //         => #partial_ref_struct_name<'__grost_lifetime__, #path_to_grost::__private::flavors::Network>
  //     );
  //   }
  // }
}

impl FlavorGenerator for Network {
  fn ty(&self) -> &syn::Type {
    &self.ty
  }

  fn name(&self) -> &'static str {
    self.name
  }

  // fn derive_enum(
  //   &self,
  //   path_to_grost: &syn::Path,
  //   enum_: &Enum,
  // ) -> Result<proc_macro2::TokenStream, syn::Error> {
  //   let name_ident = enum_.name();

  //   Ok(quote! {
  //     #path_to_grost::__private::network_varint!(#name_ident);
  //   })
  // }

  fn derive_object(&self, object: &Object) -> Result<proc_macro2::TokenStream, syn::Error> {
    Self::derive_object(self, object)
    // let default_format = self.derive_default_format(path_to_grost, struct_.name().name());
    // let partial_ref_struct_name = struct_.partial_ref_name();
    // let struct_encoded_state = self.derive_encoded_state(
    //   path_to_grost,
    //   struct_.name().name(),
    //   &partial_ref_struct_name,
    // );
    // let partial_struct_encoded_state = self.derive_encoded_state(
    //   path_to_grost,
    //   &struct_.partial_struct_name(),
    //   &partial_ref_struct_name,
    // );
    // let selectable = self.derive_selectable_for_object(path_to_grost, struct_);
    // let selector = self.derive_selector_for_object(path_to_grost, struct_);
    // let reflectable = self.derive_reflectable_for_object(path_to_grost, struct_);
    // let indexing = self.derive_index_for_object(path_to_grost, struct_);
    // let encode = self.derive_encode(path_to_grost, struct_);
    // let encode_reflection = self.derive_encode_reflection(path_to_grost, struct_);
    // let partial_encode = self.derive_partial_encode(path_to_grost, struct_);

    // Ok(quote! {
    //   // #struct_encoded_state

    //   // #partial_struct_encoded_state

    //   // #path_to_grost::__private::decoded_state!(
    //   //   &'__grost_lifetime__ #path_to_grost::__private::flavors::Network: #partial_ref_struct_name<'__grost_lifetime__, #path_to_grost::__private::flavors::Network>
    //   //     as #path_to_grost::__private::flavors::network::LengthDelimited
    //   //     => #partial_ref_struct_name<'__grost_lifetime__, #path_to_grost::__private::flavors::Network>
    //   // );

    //   // #default_format

    //   // #selectable

    //   // #selector

    //   // #reflectable

    //   // #indexing

    //   // #partial_encode

    //   // #encode

    //   // #encode_reflection
    // })
  }
}
