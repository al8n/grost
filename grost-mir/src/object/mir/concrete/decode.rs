use quote::{format_ident, quote};
use syn::{GenericParam, TypeParam};

use crate::{
  object::{Label, concrete::partial},
  utils::{MissingOperation, ConvertOperation, grost_decode_trait_lifetime},
};

impl<M, F> super::ConcreteObject<M, F> {
  pub(super) fn derive_decode(&self) -> darling::Result<proc_macro2::TokenStream> {
    let object_impl = derive_object_decode(self)?;
    let partial_ref_object_impl = derive_partial_ref_object_decode(self)?;
    let partial_object_decode_impl = derive_partial_object_decode(self)?;
    let partial_ref_state_impl = derive_partial_ref_state(self)?;

    Ok(quote! {
      #object_impl
      #partial_object_decode_impl
      #partial_ref_object_impl
      #partial_ref_state_impl
    })
  }
}

fn derive_partial_ref_state<M, F>(
  object: &super::ConcreteObject<M, F>,
) -> darling::Result<proc_macro2::TokenStream> {
  let partial_ref_state_type = object.partial_ref_state_type();
  let path_to_grost = object.path_to_grost();
  let generics = object.partial_ref().generics();
  let (ig, _, where_clauses) = generics.split_for_impl();
  let ty = object.ty();
  let lt = &object.lifetime_param().lifetime;
  let partial_ref_object_ty = object.partial_ref().ty();

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
      impl #ig #path_to_grost::__private::state::State<#partial_ref_state_type> for #partial_ty #where_clauses {
        type Output = #partial_ref_object_ty;
      }
    }
  };

  Ok(quote! {
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl #ig #path_to_grost::__private::state::State<#partial_ref_state_type> for #ty #where_clauses {
      type Output = #partial_ref_object_ty;
    }

    #partial_object_impl

    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl #ig #path_to_grost::__private::state::State<#partial_ref_state_type> for #partial_ref_object_ty #where_clauses {
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
  let (ig, _, where_clauses) = partial_object.generics().split_for_impl();
  let (decode_ig, _, decode_where_clauses) = decode_generics.split_for_impl();

  let path_to_grost = object.path_to_grost();
  let lt = &object.lifetime_param().lifetime;
  let ubg = &object.buffer_param().ident;
  let read_buffer_ident = &object.read_buffer_param().ident;
  let flavor_ty = object.flavor_type();
  let wf = object.wire_format();

  let decode_trait = partial_object.applied_decode_trait(quote! { Self })?;
  let partial_ref_object_ty = object.partial_ref().ty();
  let object_ty = object.ty();

  let to_object = object
    .fields()
    .iter()
    .filter_map(|f| f.try_unwrap_tagged_ref().ok())
    .map(|f| {
      let name = f.name();
      let variable_name = format_ident!("__grost_partial_object_{}", name);
      let nullable = f.label().is_nullable();
      let default = f.default();
      match f.label() {
        Label::Bytes => todo!(),
        Label::String => todo!(),
        Label::Map { .. } => todo!(),
        Label::Set(_) => todo!(),
        Label::List(_) => {
          // quote! {
          //   let #variable_name = input.#name;
          // }
          todo!()
        }
        Label::Optional(_) => {
          quote! {
            let #variable_name = input.#name.take();
          }
        }
        Label::Scalar => todo!(),
        Label::Object => todo!(),
        Label::Enum => todo!(),
        Label::Union => todo!(),
        Label::Interface => todo!(),
      }
    });

  Ok(quote! {
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl #decode_ig #decode_trait for #partial_object_ty #decode_where_clauses {
      fn decode(
        context: &#lt <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Context,
        src: #read_buffer_ident,
      ) -> ::core::result::Result<(::core::primitive::usize, Self), <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Error>
      where
        Self: ::core::marker::Sized + #lt,
        #read_buffer_ident: #path_to_grost::__private::buffer::ReadBuf + #lt,
        #ubg: #path_to_grost::__private::buffer::Buffer<<#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Unknown<#read_buffer_ident>> + #lt
      {
        <#partial_object_ty as
          #path_to_grost::__private::decode::Decode<#lt, #partial_ref_object_ty, #wf, #read_buffer_ident, #ubg, #flavor_ty>
        >::decode(context, src)
          .and_then(|(read, input)| {
            <#partial_object_ty as #path_to_grost::__private::convert::Transform::<#flavor_ty, #wf, #partial_ref_object_ty>>::transform(input)
              .map(|input| (read, input))
          })
      }
    }

    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl #ig #path_to_grost::__private::convert::Transform<#flavor_ty, #wf, Self> for #partial_object_ty #where_clauses {
      fn transform(input: Self) -> ::core::result::Result<Self, <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Error> {
        ::core::result::Result::Ok(input)
      }
    }

    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl #ig #path_to_grost::__private::convert::Transform<#flavor_ty, #wf, #partial_object_ty> for #object_ty #where_clauses {
      fn transform(__grost_partial_object_input__: #partial_object_ty) -> ::core::result::Result<#object_ty, <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Error> {
        // ::core::result::Result::Ok(input)
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
  let (ig, _, where_clauses) = object.generics().split_for_impl();

  let path_to_grost = object.path_to_grost();
  let lt = &object.lifetime_param().lifetime;
  let ubg = &object.buffer_param().ident;
  let read_buffer_ident = &object.read_buffer_param().ident;
  let flavor_ty = object.flavor_type();
  let wf = object.wire_format();
  let decode_trait = object.applied_decode_trait(quote! { Self })?;

  Ok(quote! {
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl #decode_ig #decode_trait for #object_ty #decode_where_clauses {
      fn decode(
        context: &#lt <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Context,
        src: #read_buffer_ident,
      ) -> ::core::result::Result<(::core::primitive::usize, Self), <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Error>
      where
        Self: ::core::marker::Sized + #lt,
        #read_buffer_ident: #path_to_grost::__private::buffer::ReadBuf + #lt,
        #ubg: #path_to_grost::__private::buffer::Buffer<<#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Unknown<#read_buffer_ident>> + #lt
      {
        ::core::todo!()
      }
    }

    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl #ig #path_to_grost::__private::convert::Transform<#flavor_ty, #wf, Self> for #object_ty #where_clauses {
      fn transform(input: Self) -> ::core::result::Result<Self, <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Error> {
        ::core::result::Result::Ok(input)
      }
    }
  })
}

fn derive_partial_ref_object_decode<M, F>(
  object: &super::ConcreteObject<M, F>,
) -> darling::Result<proc_macro2::TokenStream> {
  let object_ty = object.ty();
  let object_reflectable = object.reflectable();
  let selector_ty = object.selector().ty();
  let partial_object_ty = object.partial().ty();
  let partial_ref_object = object.partial_ref();
  let partial_ref_object_ty = partial_ref_object.ty();
  let (ig, _, where_clauses) = partial_ref_object.generics().split_for_impl();
  let (decode_ig, _, decode_where_clauses) =
    partial_ref_object.decode_generics().split_for_impl();

  let path_to_grost = object.path_to_grost();
  let ubg = &object.buffer_param().ident;
  let read_buffer_ident = &object.read_buffer_param().ident;
  let flavor_ty = object.flavor_type();
  let decode_to_self_trait = partial_ref_object.applied_decode_trait(quote! { Self })?;
  let decode_to_partial_ref_trait =
    partial_ref_object.applied_decode_trait(quote! { #partial_ref_object_ty })?;
  let lt = grost_decode_trait_lifetime();

  let buffer_field_name = &partial_ref_object.buffer_field_name;
  let mut on_missing = vec![];
  let mut pdtp_on_missing = vec![];
  let mut pdtp = vec![];
  let mut pdptp_on_missing = vec![];
  let mut pdptp = vec![];
  let mut field_decode_branches = vec![];

  object
    .fields()
    .iter()
    .filter_map(|f| f.try_unwrap_tagged_ref().ok())
    .for_each(|f| {
      let field_name = f.name();
      let field_identifier = f.reflection().identifier_reflection();
      let field_ty = f.ty();
      let field_decode_trait_type = f.partial_ref().decode_trait_type();
      let field_wf = f.wire_format();
      let is_field_selected = format_ident!("is_{}_selected", field_name);
      let field_selector = format_ident!("{}_ref", field_name);
      let partial_ref_field_ty = f.partial_ref().ty();

      {
        let convert = f.partial_ref().convert();
        let partial_field_ty = f.partial().ty();
        if let Some(missing_operation) = convert.missing_operation() {
          let on_missing = match missing_operation {
            MissingOperation::Or(invokable) => quote! {
              if this.#field_name.is_none() {
                this.#field_name = (#invokable)();
              }
            },
            MissingOperation::OrDefault(invokable) => quote! {
              if this.#field_name.is_none() {
                this.#field_name = ::core::option::Option::Some((#invokable)());
              }
            },
            MissingOperation::OkOr(invokable) => quote! {
              if this.#field_name.is_none() {
                this.#field_name = ::core::option::Option::Some((#invokable)()?);
              }
            },
          };

          pdptp_on_missing.push(quote! {
            if selector.#is_field_selected() {
              #on_missing
            }
          });

          pdtp_on_missing.push(on_missing);
        }

        pdtp.push(match convert.transform_operation() {
          None => {
            quote! {
              if let ::core::option::Option::Some(value) = input.#field_name {
                this.#field_name = ::core::option::Option::Some(<#partial_field_ty as #path_to_grost::__private::convert::Transform<#flavor_ty, #field_wf, #partial_ref_field_ty>>::transform(value)?);
              }
            }
          }
          Some(transform) => {
            match transform {
              ConvertOperation::Into(invokable) => quote! {
                this.#field_name = (#invokable)(input.#field_name);
              },
              ConvertOperation::TryInto(invokable) => quote! {
                this.#field_name = (#invokable)(input.#field_name)?;
              },
            }
          }
        });

        // TODO: partial transform operation
        pdptp.push(match convert.transform_operation() {
          None => {
            quote! {
              if let ::core::option::Option::Some(value) = input.#field_name {
                if selector.#is_field_selected() {
                  this.#field_name = <#partial_field_ty as #path_to_grost::__private::convert::PartialTransform<#flavor_ty, #field_wf, #partial_ref_field_ty>>::partial_transform(value, selector.#field_selector())?;
                }
              }
            }
          }
          Some(transform) => {
            let transform = match transform {
              ConvertOperation::Into(invokable) => quote! {
                this.#field_name = (#invokable)(input.#field_name);
              },
              ConvertOperation::TryInto(invokable) => quote! {
                this.#field_name = (#invokable)(input.#field_name)?;
              },
            };

            quote! {
              if selector.#is_field_selected() {
                #transform
              }
            }
          }
        });
      }

      let value = f.decode().then().map(|f| {
        quote! {
          let value = (#f)(value)?;
        }
      });

      if let Some(missing_operation) = f.decode().missing_operation() {
        on_missing.push(match missing_operation {
          MissingOperation::Or(invokable) => quote! {
            if this.#field_name.is_none() {
              this.#field_name = (#invokable)();
            }
          },
          MissingOperation::OrDefault(invokable) => quote! {
            if this.#field_name.is_none() {
              this.#field_name = ::core::option::Option::Some((#invokable)());
            }
          },
          MissingOperation::OkOr(invokable) => quote! {
            if this.#field_name.is_none() {
              this.#field_name = ::core::option::Option::Some((#invokable)()?);
            }
          },
        });
      }
      field_decode_branches.push(quote! {
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

          let (read, value) = <#field_ty as  #field_decode_trait_type>::decode(context, src.segment(offset..))?;
          #value
          this.#field_name = ::core::option::Option::Some(value);
          offset += read;
        },
      });
    });

  let decode_partial_generics = {
    let mut output = partial_ref_object.decode_generics().clone();
    if let Some(preds) = object
      .partial()
      .generics()
      .where_clause
      .as_ref()
      .map(|wc| &wc.predicates)
    {
      output
        .make_where_clause()
        .predicates
        .extend(preds.iter().cloned());
    }
    output
  };
  let (decode_partial_ig, _, decode_partial_where_clauses) =
    decode_partial_generics.split_for_impl();

  let transform_from_partial_ref_generics = {
    let mut output = partial_ref_object.transform_generics().clone();
    if let Some(preds) = object
      .partial()
      .generics()
      .where_clause
      .as_ref()
      .map(|wc| &wc.predicates)
    {
      output
        .make_where_clause()
        .predicates
        .extend(preds.iter().cloned());
    }
    output
  };
  let (transform_from_partial_ref_ig, _, transform_from_partial_ref_where_clauses) =
    transform_from_partial_ref_generics.split_for_impl();
  let partial_transform_from_partial_ref_generics = {
    let mut output = partial_ref_object.partial_transform_generics().clone();
    if let Some(preds) = object
      .partial()
      .generics()
      .where_clause
      .as_ref()
      .map(|wc| &wc.predicates)
    {
      output
        .make_where_clause()
        .predicates
        .extend(preds.iter().cloned());
    }
    output
      .make_where_clause()
      .predicates
      .push(syn::parse2(quote! {
        #partial_ref_object_ty: #path_to_grost::__private::selection::Selectable<#flavor_ty, Selector = #selector_ty>
      })?);

    output
  };

  let (
    partial_transform_from_partial_ref_ig,
    _,
    partial_transform_from_partial_ref_where_clauses,
  ) = partial_transform_from_partial_ref_generics.split_for_impl();

  let wf = object.wire_format();

  Ok(quote! {
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl #ig #path_to_grost::__private::convert::Transform<#flavor_ty, #wf, Self> for #partial_ref_object_ty #where_clauses {
      fn transform(input: Self) -> ::core::result::Result<Self, <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Error> {
        ::core::result::Result::Ok(input)
      }
    }

    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl #decode_ig #decode_to_self_trait for #partial_ref_object_ty #decode_where_clauses {
      fn decode(
        context: &#lt <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Context,
        src: #read_buffer_ident,
      ) -> ::core::result::Result<(::core::primitive::usize, Self), <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Error>
      where
        Self: ::core::marker::Sized + #lt,
        #read_buffer_ident: #path_to_grost::__private::buffer::ReadBuf + #lt,
        #ubg: #path_to_grost::__private::buffer::Buffer<<#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Unknown<#read_buffer_ident>> + #lt
      {
        let buf = src.remaining_slice();
        let buf_len = buf.len();
        let mut this = Self::new();

        let mut offset = 0;
        while offset < buf_len {
          let (read, identifier) = <<#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Identifier as  #path_to_grost::__private::identifier::Identifier<#flavor_ty>>::decode::<&[::core::primitive::u8]>(&buf[offset..])?;
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

                offset += <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::skip(context, identifier.wire_type(), src.segment(offset..))?;
              } else {
                let encoded_len = <<#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Identifier as  #path_to_grost::__private::identifier::Identifier<#flavor_ty>>::encoded_len(&identifier);
                let (read, unknown) = <#path_to_grost::__private::flavors::Groto as #path_to_grost::__private::flavors::Flavor>::decode_unknown(context, src.segment(offset - encoded_len..))?;
                offset += read;
                let unknowns_mut = this.#buffer_field_name.get_or_insert_with(|| #ubg::new());

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

        #(#on_missing)*

        ::core::result::Result::Ok((offset, this))
      }
    }

    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl #decode_ig #decode_to_partial_ref_trait for #object_ty #decode_where_clauses {
      fn decode(
        context: &#lt <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Context,
        src: #read_buffer_ident,
      ) -> ::core::result::Result<(::core::primitive::usize, #partial_ref_object_ty), <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Error>
      where
        #partial_ref_object_ty: ::core::marker::Sized + #lt,
        #read_buffer_ident: #path_to_grost::__private::buffer::ReadBuf + #lt,
        #ubg: #path_to_grost::__private::buffer::Buffer<<#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Unknown<#read_buffer_ident>> + #lt
      {
        <#partial_ref_object_ty as #decode_to_partial_ref_trait>::decode(context, src)
      }
    }

    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl #decode_partial_ig #decode_to_partial_ref_trait for #partial_object_ty #decode_partial_where_clauses {
      fn decode(
        context: &#lt <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Context,
        src: #read_buffer_ident,
      ) -> ::core::result::Result<(::core::primitive::usize, #partial_ref_object_ty), <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Error>
      where
        #partial_ref_object_ty: ::core::marker::Sized + #lt,
        #read_buffer_ident: #path_to_grost::__private::buffer::ReadBuf + #lt,
        #ubg: #path_to_grost::__private::buffer::Buffer<<#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Unknown<#read_buffer_ident>> + #lt
      {
        <#partial_ref_object_ty as #decode_to_partial_ref_trait>::decode(context, src)
      }
    }

    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl #transform_from_partial_ref_ig #path_to_grost::__private::convert::Transform<#flavor_ty, #wf, #partial_ref_object_ty> for #partial_object_ty #transform_from_partial_ref_where_clauses {
      fn transform(input: #partial_ref_object_ty) -> ::core::result::Result<Self, <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Error> {
        let mut this = Self::new();

        #(#pdtp)*

        #(#pdtp_on_missing)*

        ::core::result::Result::Ok(this)
      }
    }

    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl #partial_transform_from_partial_ref_ig #path_to_grost::__private::convert::PartialTransform<#flavor_ty, #wf, #partial_ref_object_ty> for #partial_object_ty #partial_transform_from_partial_ref_where_clauses {
      fn partial_transform(
        input: #partial_ref_object_ty,
        selector: &#selector_ty,
      ) -> ::core::result::Result<::core::option::Option<Self>, <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Error> {
        let mut this = Self::new();

        #(#pdptp)*

        #(#pdptp_on_missing)*

        ::core::result::Result::Ok((!this.is_empty()).then_some(this))
      }
    }
  })
}
