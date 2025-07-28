use syn::GenericParam;

use super::*;

#[derive(Debug, Clone)]
pub struct ObjectEncoder {
  name: Ident,
  base_state_type: Type,
  with_identifier_state_type: Type,
  generics: Generics,
  encode_generics: Generics,
  with_identifier_state_type_generics: TokenStream,
  base_state_type_generics: TokenStream,
  encoder_state_type_param: TypeParam,
}

impl ObjectEncoder {
  /// Returns the name of the encoder
  #[inline]
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  /// Returns the generics of the encoder
  #[inline]
  pub const fn generics(&self) -> &Generics {
    &self.generics
  }

  pub(super) fn try_new(object: &RawObject) -> darling::Result<Self> {
    let name = format_ident!("{}Encoder", object.name);
    let wbp = &object.write_buffer_param;
    let wbi = &wbp.ident;
    let path_to_grost = &object.path_to_grost;
    let flavor_type = &object.flavor_type;
    let mut generics = object.generics.clone();
    generics.params.push(wbp.clone().into());
    generics.params.push(GenericParam::Type(syn::parse2(
      quote! { __GROST_ENCODER_STATE__ },
    )?));
    let mut encode_generics = generics.clone();
    encode_generics
      .make_where_clause()
      .predicates
      .push(syn::parse2(quote! {
        #wbi: #path_to_grost::__private::buffer::WriteBuf
      })?);
    let encoder_state_type_param = TypeParam::from(format_ident!("__GROST_ENCODER_STATE__"));

    let base_state_type_generics = {
      let params = encode_generics.params.iter().map(|param| match param {
        GenericParam::Lifetime(lifetime_param) => quote!(#lifetime_param),
        GenericParam::Type(tp) if tp.ident.eq("__GROST_ENCODER_STATE__") => {
          quote!(())
        }
        GenericParam::Type(type_param) => quote!(#type_param),
        GenericParam::Const(const_param) => quote!(#const_param),
      });

      quote!(<#(#params),*>)
    };

    let with_identifier_state_type_generics = {
      let params = encode_generics.params.iter().map(|param| match param {
        GenericParam::Lifetime(lifetime_param) => quote!(#lifetime_param),
        GenericParam::Type(tp) if tp.ident.eq("__GROST_ENCODER_STATE__") => {
          quote!(#path_to_grost::__private::state::WithIdentifier<#flavor_type>)
        }
        GenericParam::Type(type_param) => quote!(#type_param),
        GenericParam::Const(const_param) => quote!(#const_param),
      });

      quote!(<#(#params),*>)
    };

    let base_state_type = syn::parse2(quote! {
      #name #base_state_type_generics
    })?;
    let with_identifier_state_type = syn::parse2(quote! {
      #name #with_identifier_state_type_generics
    })?;

    Ok(Self {
      name,
      generics,
      encode_generics,
      with_identifier_state_type,
      with_identifier_state_type_generics,
      base_state_type_generics,
      base_state_type,
      encoder_state_type_param,
    })
  }
}

impl<T, S, M> Object<T, S, M> {
  pub(super) fn derive_encoder(&self) -> darling::Result<proc_macro2::TokenStream> {
    let encoder = self.encoder();
    let name = encoder.name();
    let vis = &self.vis;
    let wbp = self.write_buffer_param();
    let wbi = &wbp.ident;

    let fields = self
      .fields()
      .iter()
      .filter_map(|f| f.try_unwrap_tagged_ref().ok())
      .map(|f| {
        let field_name = f.name();
        let marker = if f.uses_generics() {
          let field_type_marker = format_ident!("__{}_marker__", field_name);
          let ty = f.ty();
          quote! {
            #[doc(hidden)]
            #field_type_marker: ::core::marker::PhantomData<#ty>,
          }
        } else {
          quote!()
        };

        quote! {
          #field_name: ::core::primitive::bool,
          #marker
        }
      });
    let generics = self.generics();
    let wc = generics.where_clause.as_ref();
    let encoder_state_type = &encoder.encoder_state_type_param.ident;

    Ok(quote! {
      #[derive(::core::marker::Copy, ::core::marker::Clone, ::core::fmt::Debug)]
      #vis struct #name #generics #wc {
        #[doc(hidden)]
        __grost_encoder_state__: #encoder_state_type,
        #[doc(hidden)]
        __grost_write_buffer__: #wbi,
        #[doc(hidden)]
        __grost_write_cursor__: ::core::primitive::usize,
        #(#fields)*
      }
    })
  }

  pub(super) fn derive_encoder_impl(&self) -> darling::Result<proc_macro2::TokenStream> {
    let encoder = self.encoder();
    let name = encoder.name();
    let generics = encoder.generics();
    let (ig, tg, wc) = generics.split_for_impl();
    let (_, _, ewc) = encoder.encode_generics.split_for_impl();
    let wbp = self.write_buffer_param();
    let wbi = &wbp.ident;

    let path_to_grost = self.path_to_grost();
    let flavor_type = self.flavor_type();

    let base_state_type = &encoder.base_state_type;
    let base_state_type_generics = &encoder.base_state_type_generics;
    let with_identifier_state_type_generics = &encoder.with_identifier_state_type_generics;

    let mut fields_init = Vec::new();
    let mut encode_field_identifier_fns = Vec::new();
    self
      .fields()
      .iter()
      .filter_map(|f| f.try_unwrap_tagged_ref().ok())
      .try_for_each(|f| {
        let field_name = f.name();
        let label = f.label();
        fields_init.push(quote! {
          #field_name: false,
        });
        if f.uses_generics() {
          let field_type_marker = format_ident!("__{}_marker__", field_name);
          fields_init.push(quote! {
            #field_type_marker: ::core::marker::PhantomData,
          });
        }

        let identifier = self.identifier_options();
        let identifier_constructor = identifier.constructor();
        let identifier_encode = identifier.encode();
        let tag_constructor = self.tag_options().constructor();
        let repeated = label.is_repeated();
        let with_identifier_state_type = &encoder.with_identifier_state_type;

        let encode_field_identifier_fn = derive_encode_field_identifier(
          path_to_grost,
          name,
          with_identifier_state_type,
          flavor_type,
          f,
          identifier_constructor,
          tag_constructor,
          repeated,
        );
        encode_field_identifier_fns.push(encode_field_identifier_fn);

        darling::Result::Ok(())
      })?;

    Ok(quote! {
      impl #ig #name #tg #wc {
        /// Returns the current position of the underlying write buffer
        #[inline]
        pub const fn position(&self) -> usize {
          self.__grost_write_cursor__
        }
      }

      impl #ig #name #base_state_type_generics #wc {
        /// Creates a new encoder with the given write buffer
        #[inline]
        pub const fn new(wb: #wbi) -> Self {
          Self {
            __grost_encoder_state__: (),
            __grost_write_buffer__: wb,
            __grost_write_cursor__: 0,
            #(#fields_init)*
          }
        }

        /// Finalizes the encoder, returning the total bytes written to the write buffer and the write buffer itself
        #[inline]
        pub const fn finalize(self) -> (::core::primitive::usize, #wbi) {
          (self.__grost_write_cursor__, self.__grost_write_buffer__)
        }
      }

      impl #ig #name #tg #ewc {
        /// Resizes the write buffer to the given size
        ///
        /// ## Panics
        /// - If the new size is less than the current position.
        pub fn resize(&mut self, new_size: ::core::primitive::usize) -> ::core::result::Result<(), <#wbi as #path_to_grost::__private::buffer::WriteBuf>::Error> {
          if new_size < self.__grost_write_cursor__ {
            ::core::panic!("cannot resize the write buffer to a size smaller than the current position");
          }
          self.__grost_write_buffer__.resize(new_size, 0)
        }
      }

      impl #ig #name #base_state_type_generics #ewc {
        #(#encode_field_identifier_fns)*
      }

      impl #ig #name #with_identifier_state_type_generics #wc {
        /// Returns the current identifier of the encoder
        #[inline]
        pub const fn identifier(&self) -> &'static <#flavor_type as #path_to_grost::__private::flavors::Flavor>::Identifier {
          self.__grost_encoder_state__.identifier()
        }

        /// Clears the encoded identifier, back to the initial encoder state
        ///
        /// This method will reset the encoded identifier.
        pub fn reset_identifier(self) -> #base_state_type {
          let encoded_len = <#flavor_type as #path_to_grost::__private::flavors::Flavor>::Identifier::encoded_len(
            self.identifier()
          );
          let start = self.__grost_write_cursor__.saturating_sub(encoded_len);

          #base_state_type {
            __grost_encoder_state__: (),
            __grost_write_cursor__: start,
            ..self
          }
        }

        /// Finish the current encode state, returning the initial encoder state
        #[inline]
        pub const fn finish(self) -> #base_state_type {
          #base_state_type {
            __grost_encoder_state__: (),
            ..self
          }
        }
      }

      impl #ig #name #with_identifier_state_type_generics #ewc {
        /// Accepts a closure to write data into the given mutable bytes slice.
        pub fn write_data<'a>(
          &'a mut self,
          f: impl ::core::ops::FnOnce(
            &'a mut [::core::primitive::u8],
          ) -> ::core::result::Result<
            ::core::primitive::usize,
            <#flavor_type as #path_to_grost::__private::flavors::Flavor>::Error
          >
        ) -> ::core::result::Result<
          ::core::primitive::usize,
          <#flavor_type as #path_to_grost::__private::flavors::Flavor>::Error
        > {
          let current_position = self.__grost_write_cursor__;
          let buf = self.__grost_write_buffer__.remaining();

          if current_position >= buf.len() {
            return f(&mut []);
          }

          f(&mut buf[current_position..])
            .map(|written| {
              self.__grost_write_cursor__ += written;
              written
            })
        }
      }
    })
  }
}

fn derive_encode_field_identifier<T>(
  path_to_grost: &Path,
  name: &Ident,
  with_identifier_state_type: &Type,
  flavor_type: &Type,
  f: &TaggedField<T>,
  identifier_constructor: &Invokable,
  tag_constructor: &Invokable,
  repeated: bool,
) -> proc_macro2::TokenStream {
  let field_name = f.name();
  let wf = f.wire_format();
  let tag = f.tag();

  let encode_field_fn_name = format_ident!("encode_{}_identifier", field_name);
  let encode_duplicated_err =
    format!("{field_name} has already been encoded, cannot encode it again",);
  let encode_field_fn_doc = format!(
    "Encodes the identifier of `{field_name}`, returning the number of bytes written and the next encoder state on success, otherwise, returns the error and the current encoder state",
  );
  let constraints = if f.reflection().wire_format_constraints().is_empty() {
    quote! {}
  } else {
    let iter = f.reflection().wire_format_constraints().iter();
    quote! {
      where
        #(
          #iter
        ),*
    }
  };

  let duplicated_check = if !repeated {
    quote! {
      if self.#field_name {
        return ::core::result::Result::Err(
          ::core::convert::Into::into(
            #path_to_grost::__private::error::Error::custom(#encode_duplicated_err)
          )
        );
      }
    }
  } else {
    quote! {}
  };

  quote! {
    #[doc = #encode_field_fn_doc]
    pub fn #encode_field_fn_name(
      mut self,
    ) -> ::core::result::Result<
      (::core::primitive::usize, #with_identifier_state_type),
      (
        Self,
        <#flavor_type as #path_to_grost::__private::flavors::Flavor>::Error
      )
    >
    #constraints
    {
      const IDENTIFIER: <#flavor_type as #path_to_grost::__private::flavors::Flavor>::Identifier = (#identifier_constructor)(
        <#wf as #path_to_grost::__private::flavors::WireFormat<#flavor_type>>::WIRE_TYPE,
        (#tag_constructor)(#tag),
      );

      #duplicated_check

      let current_position = self.__grost_write_cursor__;
      let buf = self.__grost_write_buffer__.as_mut_slice();
      let encoded_identifier_len = <<#flavor_type as #path_to_grost::__private::flavors::Flavor>::Identifier as #path_to_grost::__private::flavors::Identifier>::encoded_len(&IDENTIFIER);

      if current_position >= buf.len() {
        return ::core::result::Result::Err(
          (
            self,
            ::core::convert::Into::into(
              #path_to_grost::__private::error::Error::insufficient_buffer(
                encoded_identifier_len,
                current_position - buf.len(),
              )
            )
          )
        );
      }

      let encoded_len = <#flavor_type as #path_to_grost::__private::flavors::Flavor>::Identifier as #path_to_grost::__private::flavors::Identifier>::encode(
        &IDENTIFIER,
        &mut buf[current_position..],
      )?;
      self.__grost_write_cursor__ += encoded_len;

      ::core::result::Result::Ok(
        (
          encoded_len,
          #name {
            __grost_encoder_state__: #path_to_grost::__private::state::WithIdentifier::new(
              &IDENTIFIER
            ),
            ..self
          }
        )
      )
    }
  }
}

#[test]
fn t() {}
