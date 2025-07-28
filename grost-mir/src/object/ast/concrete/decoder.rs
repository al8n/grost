use super::{Object, RawObject};

use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{Generics, Ident, WhereClause, WherePredicate};

#[derive(Debug, Clone)]
pub struct ObjectDecoder {
  pub(super) name: Ident,
  pub(super) generics: Generics,
  default_where_clause: Option<WhereClause>,
  base_impl_generics: TokenStream,
  base_type_generics: TokenStream,
  with_identifier_impl_generics: TokenStream,
  with_identifier_type_generics: TokenStream,
  read_buf_where_clause: WhereClause,
}

impl ObjectDecoder {
  pub fn name(&self) -> &Ident {
    &self.name
  }

  pub fn definition_generics(&self) -> &Generics {
    &self.generics
  }

  pub(super) fn try_new(object: &RawObject) -> darling::Result<Self> {
    let name = format_ident!("{}Decoder", object.name);
    let path_to_grost = &object.path_to_grost;
    let read_buffer_param = &object.read_buffer_param;
    let read_buffer_ident = &read_buffer_param.ident;
    let mut generics = object.generics.clone();
    generics.params.push(read_buffer_param.clone().into());

    let base_impl_generics = {
      let (ig, _, _) = generics.split_for_impl();
      quote!(#ig)
    };
    let with_identifier_impl_generics = base_impl_generics.clone();
    let default_where_clause = generics.where_clause.clone();

    generics
      .params
      .push(syn::parse2::<syn::GenericParam>(quote! {
        __GROST_DECODER_STATE__
      })?);

    let mut read_buf_where_clause = generics.make_where_clause().clone();
    read_buf_where_clause
      .predicates
      .extend([syn::parse2::<WherePredicate>(quote! {
        #read_buffer_ident: #path_to_grost::__private::buffer::ReadBuf
      })?]);

    let base_type_generics = Self::replace_state_type_param(&generics, quote!(()));
    let with_identifier_type_generics = Self::replace_state_type_param(
      &generics,
      quote!(#path_to_grost::__private::state::WithIdentifier),
    );

    Ok(Self {
      name,
      generics,
      default_where_clause,
      base_type_generics,
      with_identifier_type_generics,
      base_impl_generics,
      with_identifier_impl_generics,
      read_buf_where_clause,
    })
  }

  fn replace_state_type_param(generics: &Generics, ty: impl ToTokens) -> proc_macro2::TokenStream {
    let type_generics = generics.type_params().map(|tp| {
      if tp.ident.eq("__GROST_DECODER_STATE__") {
        quote!(#ty)
      } else {
        let ident = &tp.ident;
        quote!(#ident)
      }
    });

    quote! {
      <#(#type_generics),*>
    }
  }
}

impl<M, F, S> Object<M, F, S> {
  pub(super) fn derive_decoder_defination(&self) -> darling::Result<proc_macro2::TokenStream> {
    let decoder_name = self.decoder().name();
    let path_to_grost = self.path_to_grost();
    let flavor_type = self.flavor_type();
    let vis = self.vis();
    let generics = self.decoder().definition_generics();
    let where_clause = &generics.where_clause;
    let read_buffer_ident = &self.read_buffer_param().ident;
    let object_name = self.name();
    let partial_object_name = self.partial.name();
    let partial_ref_object_name = self.partial_ref.name();
    let doc = format!(
      r###"Decoder for the `{object_name}` object.

This is the underlying decoder, which is used when users want to implement highly customized decoding logic.

If you do not want to implement a customized decoding logic, please see:

  - [`{object_name}::decode`]
  - [`{partial_object_name}::decode`]
  - [`{partial_ref_object_name}::decode`]"###
    );

    let fields = self.fields().iter().filter_map(|f| {
      if f.use_generics() {
        let field_name = f.name();
        let field_ty = f.ty();
        Some(quote! {
          #field_name: ::core::marker::PhantomData<#field_ty>,
        })
      } else {
        None
      }
    });

    Ok(quote! {
      #[automatically_derived]
      #[allow(clippy::type_complexity, clippy::non_camel_case_types)]
      #[derive(::core::fmt::Debug)]
      #[doc = #doc]
      #vis struct #decoder_name #generics #where_clause {
        #[doc(hidden)]
        __grost_read_buffer__: #read_buffer_ident,
        #[doc(hidden)]
        __grost_read_offset__: ::core::primitive::usize,
        #[doc(hidden)]
        __grost_next_identifier__: ::core::option::Option<
          <#flavor_type as #path_to_grost::__private::flavors::Flavor>::Identifier
        >,
        #[doc(hidden)]
        __grost_decoder_state__: ::core::marker::PhantomData<__GROST_DECODER_STATE__>,
        #(#fields)*
      }
    })
  }

  pub(super) fn derive_decoder_impl(&self) -> darling::Result<proc_macro2::TokenStream> {
    let name = self.decoder().name();
    let path_to_grost = self.path_to_grost();
    let read_buffer_ident = &self.read_buffer_param().ident;
    let decoder = self.decoder();

    let (ig, tg, wc) = &decoder.generics.split_for_impl();
    let base_ig = &decoder.base_impl_generics;
    let base_tg = &decoder.base_type_generics;
    let read_buf_wc = &decoder.read_buf_where_clause;
    let with_identifier_ig = &decoder.with_identifier_impl_generics;
    let with_identifier_tg = &decoder.with_identifier_type_generics;
    let new_doc = format!("Creates a new `{name}` decoder.");

    let fields_init = self
      .fields()
      .iter()
      .filter_map(|f| {
        let name = f.name();
        if f.use_generics() {
          let field_ty = f.ty();
          Some(quote! {
            #name: ::core::marker::PhantomData::<#field_ty>,
          })
        } else {
          None
        }
      })
      .collect::<Vec<_>>();

    let flavor_type = self.flavor_type();
    let object_indexer = self.indexer.name();
    let object_ty = self.ty();
    let object_reflectable = self.reflectable();
    let identifier =
      quote!(<#flavor_type as #path_to_grost::__private::flavors::Flavor>::Identifier);
    let error = quote!(<#flavor_type as #path_to_grost::__private::flavors::Flavor>::Error);

    let to_index = self
      .fields()
      .iter()
      .filter_map(|f| f.try_unwrap_tagged_ref().ok())
      .map(|f| {
        let name = f.name();
        let name_str = f.name().to_string();
        let field_identifier = f.reflection().identifier_reflection();
        let field_variant = &f.index().variant().ident;

        let branch = if f.default_wire_format_constraints().is_empty() {
          quote! { <#field_identifier as #object_reflectable>::REFLECTION }
        } else {
          quote! { _ if identifier.eq(<#field_identifier as #object_reflectable>::REFLECTION) }
        };

        quote! {
          #branch => {
            self.__grost_read_offset__ += readed;
          }
        }
      });

    Ok(quote! {

      #[automatically_derived]
      #[allow(clippy::type_complexity, clippy::non_camel_case_types)]
      impl #base_ig #name #base_tg #wc {
        #[doc = #new_doc]
        #[inline]
        pub const fn new(buf: #read_buffer_ident) -> Self {
          Self {
            __grost_read_buffer__: buf,
            __grost_read_offset__: 0,
            __grost_next_identifier__: ::core::option::None,
            __grost_decoder_state__: ::core::marker::PhantomData,
            #(#fields_init)*
          }
        }
      }

      #[automatically_derived]
      #[allow(clippy::type_complexity, clippy::non_camel_case_types)]
      impl #ig #name #tg #wc {
        /// Returns the current offset to the underlying read buffer.
        #[inline]
        pub const fn offset(&self) -> ::core::primitive::usize {
          self.__grost_read_offset__
        }
      }

      #[automatically_derived]
      #[allow(clippy::type_complexity, clippy::non_camel_case_types)]
      impl #ig #name #tg #read_buf_wc {
        /// Returns the number of the remaining in the buffer.
        #[inline]
        pub fn remaining(&self) -> ::core::primitive::usize {
          #path_to_grost::__private::buffer::ReadBuf::len(&self.__grost_read_buffer__) - self.__grost_read_offset__
        }
      }

      #[automatically_derived]
      #[allow(clippy::type_complexity, clippy::non_camel_case_types)]
      impl #base_ig #name #base_tg #read_buf_wc {
        /// Returns the next field in the buffer.
        ///
        /// - If the next field's identifier is unknown, it will return `Ok((None, decoder))`.
        /// - If the next field's identifier is known, it will return `Ok((Some(_), decoder)))`.
        /// - If there are some errors when decoding the next field, it will return `Err(error)`.
        pub fn next_field(mut self) -> ::core::result::Result<
          (#object_indexer, #name #with_identifier_tg),
          <#flavor_type as #path_to_grost::__private::flavors::Flavor>::Error
        > {
          if self.__grost_read_offset__ >= #path_to_grost::__private::buffer::ReadBuf::len(&self.__grost_read_buffer__) {
            return ::core::result::Result::Err(
              ::core::convert::Into::into(
                #path_to_grost::__private::error::Error::buffer_underflow()
              )
            );
          }

          let buf = #path_to_grost::__private::buffer::ReadBuf::as_bytes(
            &self.__grost_read_buffer__
          );
          let (readed, identifier) = <#identifier as #path_to_grost::__private::identifier::Identifier>::decode(&buf[self.__grost_read_offset__..])?;

          match identifier {
          }
        }
      }

      #[automatically_derived]
      #[allow(clippy::type_complexity, clippy::non_camel_case_types)]
      impl #with_identifier_ig #name #with_identifier_tg #read_buf_wc {

      }
    })
  }
}
