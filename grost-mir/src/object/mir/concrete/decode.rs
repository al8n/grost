use quote::quote;

use crate::utils::grost_decode_trait_lifetime;

impl<M, F> super::ConcreteObject<M, F> {
  pub(super) fn derive_decode(&self) -> darling::Result<proc_macro2::TokenStream> {
    let object_impl = derive_object_decode(self)?;
    let partial_decoded_object_impl = derive_partial_decoded_object_decode(self)?;
    let partial_object_decode_impl = derive_partial_object_decode(self)?;
    let decoded_state_impl = derive_decoded_state(self)?;

    Ok(quote! {
      #object_impl
      #partial_object_decode_impl
      #partial_decoded_object_impl
      #decoded_state_impl
    })
  }
}

fn derive_decoded_state<M, F>(
  object: &super::ConcreteObject<M, F>,
) -> darling::Result<proc_macro2::TokenStream> {
  let decoded_state_type = object.decoded_state_type();
  let path_to_grost = object.path_to_grost();
  let generics = object.partial_decoded().generics();
  let (ig, _, where_clauses) = generics.split_for_impl();
  let ty = object.ty();
  let lt = &object.lifetime_param().lifetime;
  let partial_decoded_object_ty = object.partial_decoded().ty();

  let partial_object_impl = {
    let mut g = generics.clone();
    let partial = object.partial();
    let partial_ty = partial.ty();
    if let Some(preds) = partial
      .generics()
      .where_clause
      .as_ref()
      .map(|wc| &wc.predicates)
    {
      if !preds.is_empty() {
        g.make_where_clause()
          .predicates
          .extend(preds.iter().cloned());
      }
    }
    let (ig, _, where_clauses) = g.split_for_impl();
    quote! {
      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl #ig #path_to_grost::__private::convert::State<#decoded_state_type> for #partial_ty #where_clauses {
        type Input = &#lt [::core::primitive::u8];
        type Output = #partial_decoded_object_ty;
      }
    }
  };

  Ok(quote! {
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl #ig #path_to_grost::__private::convert::State<#decoded_state_type> for #ty #where_clauses {
      type Input = &#lt [::core::primitive::u8];
      type Output = #partial_decoded_object_ty;
    }

    #partial_object_impl

    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl #ig #path_to_grost::__private::convert::State<#decoded_state_type> for #partial_decoded_object_ty #where_clauses {
      type Input = &#lt [::core::primitive::u8];
      type Output = Self;
    }
  })
}

fn derive_partial_object_decode<M, F>(
  object: &super::ConcreteObject<M, F>,
) -> darling::Result<proc_macro2::TokenStream> {
  let partial_object = object.partial();
  let partial_object_ty = partial_object.ty();

  let decode_generics = partial_object.decode_generics();

  let (decode_ig, _, decode_where_clauses) = decode_generics.split_for_impl();

  let path_to_grost = object.path_to_grost();
  let lt = &object.lifetime_param().lifetime;
  let ubg = &object.unknown_buffer_param().ident;
  let read_buffer_ident = &object.read_buffer_param().ident;
  let flavor_ty = object.flavor_type();

  let decode_trait = partial_object.applied_decode_trait(quote! { Self })?;

  Ok(quote! {
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl #decode_ig #decode_trait for #partial_object_ty #decode_where_clauses {
      fn decode<#read_buffer_ident>(
        context: &#lt <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Context,
        src: #read_buffer_ident,
      ) -> ::core::result::Result<(::core::primitive::usize, Self), <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Error>
      where
        Self: ::core::marker::Sized + #lt,
        #read_buffer_ident: #path_to_grost::__private::buffer::ReadBuf<#lt>,
        #ubg: #path_to_grost::__private::buffer::Buffer<<#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Unknown<#read_buffer_ident>> + #lt
      {
        ::core::todo!()
      }
    }
  })
}

fn derive_object_decode<M, F>(
  object: &super::ConcreteObject<M, F>,
) -> darling::Result<proc_macro2::TokenStream> {
  let object_ty = object.ty();
  let decode_generics = object.decode_generics();
  let (decode_ig, _, decode_where_clauses) = decode_generics.split_for_impl();

  let path_to_grost = object.path_to_grost();
  let lt = &object.lifetime_param().lifetime;
  let ubg = &object.unknown_buffer_param().ident;
  let read_buffer_ident = &object.read_buffer_param().ident;
  let flavor_ty = object.flavor_type();
  let decode_trait = object.applied_decode_trait(quote! { Self })?;

  Ok(quote! {
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl #decode_ig #decode_trait for #object_ty #decode_where_clauses {
      fn decode<#read_buffer_ident>(
        context: &#lt <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Context,
        src: #read_buffer_ident,
      ) -> ::core::result::Result<(::core::primitive::usize, Self), <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Error>
      where
        Self: ::core::marker::Sized + #lt,
        #read_buffer_ident: #path_to_grost::__private::buffer::ReadBuf<#lt>,
        #ubg: #path_to_grost::__private::buffer::Buffer<<#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Unknown<#read_buffer_ident>> + #lt
      {
        ::core::todo!()
      }
    }
  })
}

fn derive_partial_decoded_object_decode<M, F>(
  object: &super::ConcreteObject<M, F>,
) -> darling::Result<proc_macro2::TokenStream> {
  let partial_decoded_object = object.partial_decoded();
  let partial_decoded_object_ty = partial_decoded_object.ty();
  let (decode_ig, _, decode_where_clauses) =
    partial_decoded_object.decode_generics().split_for_impl();

  let path_to_grost = object.path_to_grost();
  let ubg = &object.unknown_buffer_param().ident;
  let read_buffer_ident = &object.read_buffer_param().ident;
  let flavor_ty = object.flavor_type();
  let decode_trait = partial_decoded_object.applied_decode_trait(quote! { Self })?;
  let lt = grost_decode_trait_lifetime();

  let object_ty = object.ty();
  let object_reflectable = object.reflectable();
  let unknown_buffer_field_name = &partial_decoded_object.unknown_buffer_field_name;
  let field_decode_branches = object
    .fields()
    .iter()
    .filter_map(|f| f.try_unwrap_tagged_ref().ok())
    .map(|f| {
      let field_name = f.name();
      let field_identifier = f.reflection().identifier_reflection();
      let field_ty = f.ty();
      let field_decode_trait_type = f.partial_decoded().decode_trait_type();
      quote! {
        <#field_identifier as #object_reflectable>::REFLECTION => {
          if offset >= buf_len {
            return ::core::result::Result::Err(
              ::core::convert::Into::into(#path_to_grost::__private::error::Error::buffer_underflow())
            );
          }

          if this.#field_name.is_some() {
            return ::core::result::Result::Err(
              ::core::convert::Into::into(#path_to_grost::__private::error::Error::duplicated_field(
                stringify!(#field_name),
                ::core::any::type_name::<#field_ty>(),
                *<#field_identifier as #object_reflectable>::REFLECTION
              ))
            );
          }

          let (read, value) = <#field_ty as  #field_decode_trait_type>::decode(context, src.slice(offset..))?;
          this.#field_name = ::core::option::Option::Some(value);
          offset += read;
        },
      }
    });

  Ok(quote! {
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl #decode_ig #decode_trait for #partial_decoded_object_ty #decode_where_clauses {
      fn decode<#read_buffer_ident>(
        context: &#lt <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Context,
        src: #read_buffer_ident,
      ) -> ::core::result::Result<(::core::primitive::usize, Self), <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Error>
      where
        Self: ::core::marker::Sized + #lt,
        #read_buffer_ident: #path_to_grost::__private::buffer::ReadBuf<#lt>,
        #ubg: #path_to_grost::__private::buffer::Buffer<<#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Unknown<#read_buffer_ident>> + #lt
      {
        let buf = src.as_bytes();
        let buf_len = buf.len();
        let mut this = Self::new();

        let mut offset = 0;
        while offset < buf_len {
          let (read, identifier) = <<#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Identifier as  #path_to_grost::__private::flavors::Identifier<#flavor_ty>>::decode::<&[::core::primitive::u8]>(&buf[offset..])?;
          offset += read;

          match &identifier {
            #(#field_decode_branches)*
            _ => {
              if context.err_on_unknown() {
                return ::core::result::Result::Err(
                  ::core::convert::Into::into(
                    #path_to_grost::__private::error::Error::unknown_identifier(
                      ::core::any::type_name::<#object_ty>(),
                      identifier,
                    )
                  )
                );
              }

              if context.skip_unknown() {
                if offset >= buf_len {
                  return ::core::result::Result::Err(::core::convert::Into::into(#path_to_grost::__private::error::Error::buffer_underflow()));
                }

                offset += <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::skip(context, identifier.wire_type(), src.slice(offset..))?;
              } else {
                let encoded_len = <<#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Identifier as  #path_to_grost::__private::flavors::Identifier<#flavor_ty>>::encoded_len(&identifier);
                let (read, unknown) = <#path_to_grost::__private::flavors::Network as #path_to_grost::__private::flavors::Flavor>::decode_unknown(context, src.slice(offset - encoded_len..))?;
                offset += read;
                let unknowns_mut = this.#unknown_buffer_field_name.get_or_insert_with(|| #ubg::new());

                if let ::core::option::Option::Some(unknown) = unknowns_mut.push(unknown) {
                  let len = #path_to_grost::__private::Buffer::len(unknowns_mut);
                  return ::core::result::Result::Err(
                    ::core::convert::Into::into(#path_to_grost::__private::error::Error::buffer_overflow(
                      len,
                      ::core::num::NonZeroUsize::new(len + 1).unwrap(),
                    ))
                  );
                }
              }
            }
          }
        }

        ::core::result::Result::Ok((offset, this))
      }
    }
  })
}
