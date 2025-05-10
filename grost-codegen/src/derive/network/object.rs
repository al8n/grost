use super::*;

impl Network {
  pub(super) fn derive_selectable_for_object(
    &self,
    path_to_grost: &syn::Path,
    struct_: &Struct,
  ) -> proc_macro2::TokenStream {
    let struct_name = struct_.name();
    let partial_struct_name = struct_.partial_struct_name();
    let partial_ref_name = struct_.partial_ref_name();
    let selector = struct_.selector_name();

    let selectable_impl = |name: &syn::Ident, lifetime: bool| -> proc_macro2::TokenStream {
      let lifetime_with_angle = if lifetime {
        Some(quote! {<'__grost_lifetime__>})
      } else {
        None
      };
      let lifetime = if lifetime {
        Some(quote! {'__grost_lifetime__})
      } else {
        None
      };
      quote! {
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl #lifetime_with_angle #path_to_grost::__private::Selectable<
          #path_to_grost::__private::flavors::Network,
          #path_to_grost::__private::flavors::network::LengthDelimited,
        > for #name #lifetime_with_angle
        {
          type Selector = #selector;
        }

        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl #lifetime_with_angle #path_to_grost::__private::Selectable<
          #path_to_grost::__private::flavors::Network,
          #path_to_grost::__private::flavors::network::Repeated<#path_to_grost::__private::flavors::network::LengthDelimited>,
        > for #name #lifetime_with_angle
        {
          type Selector = #selector;
        }

        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl<const I: ::core::primitive::u32, #lifetime> #path_to_grost::__private::Selectable<
          #path_to_grost::__private::flavors::Network,
          #path_to_grost::__private::flavors::network::Repeated<
            #path_to_grost::__private::flavors::network::Stream<#path_to_grost::__private::flavors::network::LengthDelimited, I>
          >,
        > for #name #lifetime_with_angle
        {
          type Selector = #selector;
        }
      }
    };

    let selectable_impls = [
      selectable_impl(struct_name.name(), false),
      selectable_impl(&partial_struct_name, false),
      selectable_impl(&partial_ref_name, true),
    ];
    quote! {
      #(#selectable_impls)*
    }
  }
}
