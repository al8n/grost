use crate::object::Label;

use super::*;

#[derive(Debug, Clone)]
pub struct ObjectEncoder {
  name: Ident,
  generics: Generics,
  encode_generics: Generics,
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
    let mut generics = Generics::default();
    generics.params.push(wbp.clone().into());
    let mut encode_generics = generics.clone();
    encode_generics.make_where_clause().predicates.push(
      syn::parse2(quote! {
        #wbi: #path_to_grost::__private::buffer::WriteBuf
      })?,
    );
    Ok(Self {
      name,
      generics,
      encode_generics,
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

    let fields = self.fields()
      .iter()
      .filter_map(|f| f.try_unwrap_tagged_ref().ok())
      .map(|f| {
        let field_name = f.name();

        quote! {
          #field_name: ::core::primitive::bool,
        }
      });

    Ok(quote! {
      #[derive(::core::marker::Copy, ::core::marker::Clone, ::core::fmt::Debug)]
      #vis struct #name<#wbp> {
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
    let (eig, etg, ewc) = encoder.encode_generics.split_for_impl();
    let wbp = self.write_buffer_param();
    let wbi = &wbp.ident;

    let path_to_grost = self.path_to_grost();
    let flavor_type = self.flavor_type();

    let mut fields_init = Vec::new();
    let mut encode_field_fns = Vec::new();
    let mut partial_encode_field_fns = Vec::new();
    self.fields()
      .iter()
      .filter_map(|f| f.try_unwrap_tagged_ref().ok())
      .for_each(|f| {
        let field_name = f.name();
        let field_type = f.ty();
        let wf = f.wire_format();
        let label = f.label();
        fields_init.push(quote! {
          #field_name: false,
        });

        let identifier = self.identifier_options();
        let identifier_constructor = identifier.constructor();
        let identifier_encode = identifier.encode();
        let tag_constructor = self.tag_options().constructor();
        let repeated = label.is_repeated();

        encode_field_fns.push(derive_encode_field(
          path_to_grost,
          flavor_type, 
          f,
          identifier_constructor,
          identifier_encode,
          tag_constructor,
          repeated,
        ));
        partial_encode_field_fns.push(derive_partial_encode_field(
          path_to_grost,
          flavor_type,
          f,
          identifier_constructor,
          identifier_encode,
          tag_constructor,
          repeated,
        ));

        match label {
          Label::Generic(generic_label_value) => todo!(),
          Label::Nullable(either) => todo!(),
          Label::Map(either) => todo!(),
          Label::Set(either) => todo!(),
          Label::List(either) => todo!(),
          Label::Object(_) | Label::Union(_) | Label::Interface(_) => {
            encode_field_fns.push(derive_encode_field_with_encoder(
              path_to_grost,
              flavor_type,
              wbi,
              f,
              identifier_constructor,
              identifier_encode,
              tag_constructor,
            ));
            
          },
          _ => {}
        }
      });

    Ok(quote! {
      impl #ig #name #tg #wc {
        /// Creates a new encoder with the given write buffer
        #[inline]
        pub const fn new(wb: #wbi) -> Self {
          Self {
            __grost_write_buffer__: wb,
            __grost_write_cursor__: 0,
            #(#fields_init)*
          }
        }

        /// Returns the current position of the underlying write buffer
        #[inline]
        pub const fn position(&self) -> usize {
          self.__grost_write_cursor__
        }

        /// Finalizes the encoder, returning the total bytes written to the write buffer and the write buffer itself
        #[inline]
        pub const fn finalize(self) -> (usize, #wbi) {
          (self.__grost_write_cursor__, self.__grost_write_buffer__)
        }
      }

      impl #eig #name #etg #ewc {
        #(#encode_field_fns)*

        #(#partial_encode_field_fns)*
      }
    })
  }
}

fn derive_encode_field<T>(
  path_to_grost: &Path,
  flavor_type: &Type,
  f: &TaggedField<T>,
  identifier_constructor: &Invokable,
  identifier_encode: &Invokable,
  tag_constructor: &Invokable,
  repeated: bool,
) -> proc_macro2::TokenStream {
  let field_name = f.name();
  let field_type = f.ty();
  let wf = f.wire_format(); 
  let tag = f.tag();

  let encode_field_fn_name = format_ident!("encode_{}", field_name);
  let encode_duplicated_err = format!(
    "{field_name} has already been encoded, cannot encode it again",
  );
  let encode_field_fn_doc = format!(
    "Encodes the field `{field_name}` with the given value, returning the number of bytes written",
  );

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
    pub fn #encode_field_fn_name<__GROST_VALUE__, __GROST_VALUE_WIRE_FORMAT__>(
      &mut self,
      context: &<#flavor_type as #path_to_grost::__private::flavors::Flavor>::Context,
      value: &__GROST_VALUE__,
    ) -> ::core::result::Result<
      ::core::primitive::usize,
      <#flavor_type as #path_to_grost::__private::flavors::Flavor>::Error
    >
    where
      #field_type: #path_to_grost::__private::encode::EquivalentEncode<__GROST_VALUE__, __GROST_VALUE_WIRE_FORMAT__, #flavor_type, WireFormat = #wf>,
    {
      const IDENTIFIER: <#flavor_type as #path_to_grost::__private::flavors::Flavor>::Identifier = (#identifier_constructor)(
        <#wf as #path_to_grost::__private::flavors::WireFormat<#flavor_type>>::WIRE_TYPE,
        (#tag_constructor)(#tag),
      );
      const ENCODED_IDENTIFIER: &[::core::primitive::u8] = (#identifier_encode)(&IDENTIFIER).as_slice();

      #duplicated_check      

      let current_position = self.__grost_write_cursor__;
      let buf = self.__grost_write_buffer__.as_mut_slice();

      if current_position >= buf.len() {
        return ::core::result::Result::Err(
          ::core::convert::Into::into(
            #path_to_grost::__private::error::Error::insufficient_buffer(
              #path_to_grost::__private::encode::EncodeField::<__GROST_VALUE_WIRE_FORMAT__, #flavor_type>::encoded_field(
                value,
                context,
                #path_to_grost::__private::identifier::MaybeEncodedIdentifier::Encoded(ENCODED_IDENTIFIER),
              ),
              0,
            )
          )
        );
      }

      #path_to_grost::__private::encode::EncodeField::<__GROST_VALUE_WIRE_FORMAT__, #flavor_type>::encode_field(
        value,
        context,
        #path_to_grost::__private::identifier::MaybeEncodedIdentifier::Encoded(ENCODED_IDENTIFIER),
        &mut buf[current_position..],
      )
      .inspect(|written| {
        self.__grost_write_cursor__ += written;
        self.#field_name = true;
      })
    }
  }
}

fn derive_partial_encode_field<T>(
  path_to_grost: &Path,
  flavor_type: &Type,
  f: &TaggedField<T>,
  identifier_constructor: &Invokable,
  identifier_encode: &Invokable,
  tag_constructor: &Invokable,
  repeated: bool,
) -> proc_macro2::TokenStream {
  let field_name = f.name();
  let field_type = f.ty();
  let wf = f.wire_format(); 
  let tag = f.tag();

  let partial_encode_field_fn_name = format_ident!("partial_encode_{}", field_name);
  let partial_encode_duplicated_err = format!(
    "{field_name} has already been encoded, cannot encode it again",
  );
  let partial_encode_field_fn_doc = format!(
    "Partially encodes the field `{field_name}` with the given value, returning the number of bytes written",
  );

  let duplicated_check = if !repeated {
    quote! {
      if self.#field_name {
        return ::core::result::Result::Err(
          ::core::convert::Into::into(
            #path_to_grost::__private::error::Error::custom(#partial_encode_duplicated_err)
          )
        );
      }
    }
  } else {
    quote! {}
  };

  quote! {
    #[doc = #partial_encode_field_fn_doc]
    pub fn #partial_encode_field_fn_name<__GROST_VALUE__, __GROST_VALUE_WIRE_FORMAT__>(
      &mut self,
      context: &<#flavor_type as #path_to_grost::__private::flavors::Flavor>::Context,
      value: &__GROST_VALUE__,
      selector: &<#field_type as #path_to_grost::__private::encode::Selectable<#flavor_type>>::Selector,
    ) -> ::core::result::Result<
      ::core::primitive::usize,
      <#flavor_type as #path_to_grost::__private::flavors::Flavor>::Error
    >
    where
      #field_type: #path_to_grost::__private::encode::EquivalentPartialEncode<__GROST_VALUE__, __GROST_VALUE_WIRE_FORMAT__, #flavor_type, WireFormat = #wf>,
    {
      const IDENTIFIER: <#flavor_type as #path_to_grost::__private::flavors::Flavor>::Identifier = (#identifier_constructor)(
        <#wf as #path_to_grost::__private::flavors::WireFormat<#flavor_type>>::WIRE_TYPE,
        (#tag_constructor)(#tag),
      );
      const ENCODED_IDENTIFIER: &[::core::primitive::u8] = (#identifier_encode)(&IDENTIFIER).as_slice();

      #duplicated_check

      let current_position = self.__grost_write_cursor__;
      let buf = self.__grost_write_buffer__.as_mut_slice();
      if current_position >= buf.len() {
        return ::core::result::Result::Err(
          ::core::convert::Into::into(
            #path_to_grost::__private::error::Error::insufficient_buffer(
              #path_to_grost::__private::encode::PartialEncodeField::<__GROST_VALUE_WIRE_FORMAT__, #flavor_type>::partial_encoded_field(
                value,
                context,
                #path_to_grost::__private::identifier::MaybeEncodedIdentifier::Encoded(ENCODED_IDENTIFIER),
                selector,
              ),
              0,
            )
          )
        );
      }

      #path_to_grost::__private::encode::PartialEncodeField::<__GROST_VALUE_WIRE_FORMAT__, #flavor_type>::partial_encode_field(
        value,
        context,
        #path_to_grost::__private::identifier::MaybeEncodedIdentifier::Encoded(ENCODED_IDENTIFIER),
        &mut buf[current_position..],
        selector,
      )
      .inspect(|written| {
        self.__grost_write_cursor__ += written;
        self.#field_name = true;
      })
    }    
  }
}

fn derive_encode_field_with_encoder<T>(
  path_to_grost: &Path,
  flavor_type: &Type,
  wbi: impl ToTokens,
  f: &TaggedField<T>,
  identifier_constructor: &Invokable,
  identifier_encode: &Invokable,
  tag_constructor: &Invokable,
) -> proc_macro2::TokenStream {
  let field_name = f.name();
  let field_type = f.ty();
  let wf = f.wire_format(); 
  let tag = f.tag();

  let encode_field_fn_name = format_ident!("encode_{}_with_encoder", field_name);
  let encode_duplicated_err = format!(
    "{field_name} has already been encoded, cannot encode it again",
  );
  let encode_field_fn_doc = format!(
    "Encodes the field `{field_name}` with a given encoder, returning the number of bytes written",
  );

  quote! {
    #[doc = #encode_field_fn_doc]
    pub fn #encode_field_fn_name<__GROST_ENCODE_FN__>(
      &mut self,
      context: &<#flavor_type as #path_to_grost::__private::flavors::Flavor>::Context,
      encode: __GROST_ENCODE_FN__,
    ) -> ::core::result::Result<
      ::core::primitive::usize,
      <#flavor_type as #path_to_grost::__private::flavors::Flavor>::Error
    >
    where
      __GROST_ENCODE_FN__: ::core::ops::FnOnce(
        &<#flavor_type as #path_to_grost::__private::flavors::Flavor>::Context,
        <#flavor_type as #path_to_grost::__private::encode::Encodable>::Encoder<&mut #wbi>,
      ) -> ::core::result::Result<
        ::core::primitive::usize,
        <#flavor_type as #path_to_grost::__private::flavors::Flavor>::Error 
      >,
    {
      const IDENTIFIER: <#flavor_type as #path_to_grost::__private::flavors::Flavor>::Identifier = (#identifier_constructor)(
        <#wf as #path_to_grost::__private::flavors::WireFormat<#flavor_type>>::WIRE_TYPE,
        (#tag_constructor)(#tag),
      );
      const ENCODED_IDENTIFIER: &[::core::primitive::u8] = (#identifier_encode)(&IDENTIFIER).as_slice();

      if self.#field_name {
        return ::core::result::Result::Err(
          ::core::convert::Into::into(
            #path_to_grost::__private::error::Error::custom(#encode_duplicated_err)
          )
        );
      }

      let current_position = self.__grost_write_cursor__;

      encode(
        context,
        <#field_type as #path_to_grost::__private::encode::Encodable>::with_position(
          &mut self.__grost_write_buffer__,
          current_position,
        ),
      )
      .inspect(|written| {
        self.__grost_write_cursor__ += written;
        self.#field_name = true;
      })
    }
  }
}


#[test]
fn t() {}