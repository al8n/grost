use super::super::*;

impl Network {
  pub(crate) fn derive_selectable_for_object(
    &self,
    path_to_grost: &syn::Path,
    struct_: &Object,
  ) -> proc_macro2::TokenStream {
    let struct_name = struct_.name();
    let partial_struct_name = struct_.partial_struct_name();
    let partial_ref_name = struct_.partial_ref_name();
    let selector = struct_.selector_name();

    let selectable_impl = |name: &syn::Ident| -> proc_macro2::TokenStream {
      quote! {
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl #path_to_grost::__private::Selectable<
          #path_to_grost::__private::flavors::Network,
          #path_to_grost::__private::flavors::network::LengthDelimited,
        > for #name
        {
          type Selector = #selector;
        }

        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl #path_to_grost::__private::Selectable<
          #path_to_grost::__private::flavors::Network,
          #path_to_grost::__private::flavors::network::Repeated<#path_to_grost::__private::flavors::network::LengthDelimited>,
        > for #name
        {
          type Selector = #selector;
        }

        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl<const I: ::core::primitive::u32> #path_to_grost::__private::Selectable<
          #path_to_grost::__private::flavors::Network,
          #path_to_grost::__private::flavors::network::Repeated<
            #path_to_grost::__private::flavors::network::Stream<#path_to_grost::__private::flavors::network::LengthDelimited, I>
          >,
        > for #name
        {
          type Selector = #selector;
        }
      }
    };

    let selectable_impls = [
      selectable_impl(struct_name.name()),
      selectable_impl(&partial_struct_name),
    ];

    quote! {
      #(#selectable_impls)*

      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl<'__grost_lifetime__> #path_to_grost::__private::Selectable<
        #path_to_grost::__private::flavors::Network,
        #path_to_grost::__private::flavors::network::LengthDelimited,
      > for #partial_ref_name<'__grost_lifetime__, #path_to_grost::__private::flavors::Network>
      {
        type Selector = #selector;
      }

      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl<'__grost_lifetime__> #path_to_grost::__private::Selectable<
        #path_to_grost::__private::flavors::Network,
        #path_to_grost::__private::flavors::network::Repeated<#path_to_grost::__private::flavors::network::LengthDelimited>,
      > for #partial_ref_name<'__grost_lifetime__, #path_to_grost::__private::flavors::Network>
      {
        type Selector = #selector;
      }

      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl<'__grost_lifetime__, const I: ::core::primitive::u32> #path_to_grost::__private::Selectable<
        #path_to_grost::__private::flavors::Network,
        #path_to_grost::__private::flavors::network::Repeated<
          #path_to_grost::__private::flavors::network::Stream<#path_to_grost::__private::flavors::network::LengthDelimited, I>
        >,
      > for #partial_ref_name<'__grost_lifetime__, #path_to_grost::__private::flavors::Network>
      {
        type Selector = #selector;
      }
    }
  }
}
