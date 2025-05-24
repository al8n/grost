use super::{Network, Object, quote};

impl Network {
  pub(super) fn derive_object_decode(
    &self,
    object: &Object,
  ) -> syn::Result<proc_macro2::TokenStream> {
    let path_to_grost = object.path();
    let partial_decoded = object.partial_decoded();
    let ubp = partial_decoded.unknown_buffer_param();
    let ubi = &ubp.ident;
    let ltg = partial_decoded.lifetime();
    let ty = partial_decoded.type_with(None, Some(&self.ty), None)?;
    let mut remove_generics_without_flavor_generic =
      partial_decoded.remove_generics(None, Some(&self.ty), None)?;

    {
      let where_clauses = remove_generics_without_flavor_generic.make_where_clause();
      where_clauses.predicates.push(syn::parse2(quote! {
        Self: #path_to_grost::__private::selection::Selectable<#path_to_grost::__private::flavors::Network, #path_to_grost::__private::flavors::network::LengthDelimited>
      })?);
      where_clauses.predicates.push(syn::parse2(quote! {
        #ubi: #ltg
      })?);
      for param in object.generics().type_params() {
        let ident = &param.ident;
        where_clauses.predicates.push(syn::parse2(quote! {
          #ident: #ltg
        })?);
      }
    }

    let unknown_buffer_field_name = partial_decoded.unknown_buffer_field_name();

    let object_name = object.name();
    let object_name_str = object_name.to_string();
    let (_, object_tg, _) = object.generics().split_for_impl();

    let mut decode_branches = vec![];
    let mut partial_decode_branches = vec![];

    let mut decode_generics = remove_generics_without_flavor_generic.clone();
    let mut partial_decode_generics = remove_generics_without_flavor_generic.clone();
    partial_decoded.fields_with(
      None,
      Some(&self.ty),
      None,
    )?.into_iter().try_for_each(|f| {
      let field_name = f.name();
      let field_name_str = field_name.to_string();
      let tag = f.tag().get();
      let output_type = f.output_type();
      let object_ty = f.object_type();
      let wfr = f.wire();
      let field_decode_trait_with_types = quote! {
        #path_to_grost::__private::Decode<
          #ltg,
          #path_to_grost::__private::flavors::Network,
          <#wfr as #path_to_grost::__private::reflection::Reflectable<#object_name #object_tg>>::Reflection,
          #output_type,
          #ubi
        >
      };
      let field_partial_decode_trait_with_types = quote! {
        #path_to_grost::__private::PartialDecode<
          #ltg,
          #path_to_grost::__private::flavors::Network,
          <#wfr as #path_to_grost::__private::reflection::Reflectable<#object_name #object_tg>>::Reflection,
          #output_type,
          #ubi
        >
      };

      decode_generics.make_where_clause()
        .predicates
        .push(syn::parse2(quote! {
          #object_ty: #field_decode_trait_with_types
        })?);
      partial_decode_generics.make_where_clause()
        .predicates
        .push(syn::parse2(quote! {
          #object_ty: #field_partial_decode_trait_with_types
        })?);
  
      decode_branches.push(quote! {
        <
          #path_to_grost::__private::reflection::IdentifierReflection<
            #path_to_grost::__private::reflection::ObjectFieldReflection<
              #object_name #object_tg,
              #path_to_grost::__private::flavors::network::Identifier,
              #path_to_grost::__private::flavors::Network,
              #tag,
            >
          > as #path_to_grost::__private::reflection::Reflectable<#object_name #object_tg>
        >::REFLECTION => {
          if this.#field_name.is_some() {
            return ::core::result::Result::Err(#path_to_grost::__private::flavors::network::Error::duplicate_field(
              #object_name_str,
              #field_name_str,
              identifier,
            ));
          }

          match identifier.wire_type() {
            #path_to_grost::__private::flavors::network::WireType::Zst => {
            }
            _ => {
              if offset >= buf_len {
                return ::core::result::Result::Err(#path_to_grost::__private::flavors::network::Error::buffer_underflow());
              }

              let (read, val) = <#object_ty as #field_decode_trait_with_types>::decode::<__GROST_BUF__>(ctx, buf.slice(offset..))?;
              offset += read;
              this.#field_name = ::core::option::Option::Some(val);
            }
          }
        },
      });

      partial_decode_branches.push(quote! {
        <
          #path_to_grost::__private::reflection::IdentifierReflection<
            #path_to_grost::__private::reflection::ObjectFieldReflection<
              #object_name #object_tg,
              #path_to_grost::__private::flavors::network::Identifier,
              #path_to_grost::__private::flavors::Network,
            >
          > as #path_to_grost::__private::reflection::Reflectable<#object_name #object_tg>
        >::REFLECTION => {
          match identifier.wire_type() {
            #path_to_grost::__private::flavors::network::WireType::Zst => {

            }
            _ => {
              if offset >= buf_len {
                return ::core::result::Result::Err(#path_to_grost::__private::flavors::network::Error::buffer_underflow());
              }

              // let (read, val) = <#object_ty as #path_to_grost::__private::Decode<'_, #path_to_grost::__private::flavors::Network, <#wfr as #path_to_grost::__private::reflection::Reflectable<#object_name #object_tg>>::Reflection, #output_type, #ubi>>::decode::<__GROST_BUF__>(ctx, buf.slice(offset..))?;
              // offset += read;
              // this.#field_name = ::core::option::Option::Some(val);
            }
          }
        },
      });

      syn::Result::Ok(())
    })?;

    let (decode_ig, _, decode_where_clause) = decode_generics.split_for_impl();
    let (partial_decode_ig, _, partial_decode_where_clause) = partial_decode_generics.split_for_impl();
    Ok(quote! {
      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl #decode_ig #path_to_grost::__private::Decode<#ltg, #path_to_grost::__private::flavors::Network, #path_to_grost::__private::flavors::network::LengthDelimited, Self, #ubi> for #ty #decode_where_clause {
        fn decode<__GROST_BUF__>(
          ctx: &#path_to_grost::__private::flavors::network::Context,
          buf: __GROST_BUF__,
        ) -> ::core::result::Result<(::core::primitive::usize, Self), #path_to_grost::__private::flavors::network::Error>
        where
          Self: ::core::marker::Sized,
          __GROST_BUF__: #path_to_grost::__private::Buf<#ltg>,
          #ubi: #path_to_grost::__private::Buffer<#path_to_grost::__private::flavors::network::Unknown<__GROST_BUF__>>,
        {
          let bytes = buf.as_bytes();
          let mut this = Self::new();

          let mut offset = 0;
          let buf_len = bytes.len();
          while offset < buf_len {
            let (encoded_identifier_len, identifier) =
              #path_to_grost::__private::flavors::network::Identifier::decode(&bytes[offset..])?;
            offset += encoded_identifier_len;

            match &identifier {
              #(#decode_branches)*
              _ => {
                if ctx.skip_unknown() {
                  if offset >= buf_len {
                    return ::core::result::Result::Err(#path_to_grost::__private::flavors::network::Error::buffer_underflow());
                  }

                  offset += <#path_to_grost::__private::flavors::Network as #path_to_grost::__private::flavors::Flavor>::skip(ctx, identifier.wire_type(), buf.slice(offset..))?;
                } else {
                  let (read, unknown) = <#path_to_grost::__private::flavors::Network as #path_to_grost::__private::flavors::Flavor>::decode_unknown(ctx, buf.slice(offset - encoded_identifier_len..))?;
                  offset += read;
                  let unknowns_mut = this.#unknown_buffer_field_name.get_or_insert_with(|| #ubi::new());

                  if let ::core::option::Option::Some(unknown) = unknowns_mut.push(unknown) {
                    let len = #path_to_grost::__private::Buffer::len(unknowns_mut);
                    return ::core::result::Result::Err(#path_to_grost::__private::flavors::network::Error::buffer_overflow(
                      len,
                      ::core::num::NonZeroUsize::new(len + 1).unwrap(),
                    ));
                  }
                }
              }
            }
          }

          ::core::todo!()
        }
      }

      #[automatically_derived]
      #[allow(non_camel_case_types)]
      impl #partial_decode_ig #path_to_grost::__private::PartialDecode<#ltg, #path_to_grost::__private::flavors::Network, #path_to_grost::__private::flavors::network::LengthDelimited, Self, #ubi> for #ty #partial_decode_where_clause {
        fn partial_decode<__GROST_BUF__>(
          ctx: &#path_to_grost::__private::flavors::network::Context,
          buf: __GROST_BUF__,
          selector: &Self::Selector,
        ) -> ::core::result::Result<(::core::primitive::usize, ::core::option::Option<Self>), #path_to_grost::__private::flavors::network::Error>
        where
          Self: ::core::marker::Sized,
          __GROST_BUF__: #path_to_grost::__private::Buf<#ltg>,
          #ubi: #path_to_grost::__private::Buffer<#path_to_grost::__private::flavors::network::Unknown<__GROST_BUF__>>,
        {
          use #path_to_grost::__private::{selection::Selector, Buf};

          let bytes = buf.as_bytes();
          let mut this = Self::new();
          let mut offset = 0;
          let buf_len = bytes.len();
          while offset < buf_len {
            let (encoded_identifier_len, identifier) =
              #path_to_grost::__private::flavors::network::Identifier::decode(&bytes[offset..])?;
            offset += encoded_identifier_len;

            match &identifier {
              _ => {
                if ctx.skip_unknown() {
                  if offset >= buf_len {
                    return ::core::result::Result::Err(#path_to_grost::__private::flavors::network::Error::buffer_underflow());
                  }

                  offset += <#path_to_grost::__private::flavors::Network as #path_to_grost::__private::flavors::Flavor>::skip(ctx, identifier.wire_type(), buf.slice(offset..))?;
                } else {
                  let (read, unknown) = <#path_to_grost::__private::flavors::Network as #path_to_grost::__private::flavors::Flavor>::decode_unknown(ctx, buf.slice(offset - encoded_identifier_len..))?;
                  offset += read;
                  let unknowns_mut = this.#unknown_buffer_field_name.get_or_insert_with(|| #ubi::new());

                  if let ::core::option::Option::Some(unknown) = unknowns_mut.push(unknown) {
                    let len = #path_to_grost::__private::Buffer::len(unknowns_mut);
                    return ::core::result::Result::Err(#path_to_grost::__private::flavors::network::Error::buffer_overflow(
                      len,
                      ::core::num::NonZeroUsize::new(len + 1).unwrap(),
                    ));
                  }
                }
              }
            }
          }

          ::core::todo!()
        }
      }
    })
  }
}
