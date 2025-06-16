use quote::quote;

impl<M, F> super::ConcreteObject<M, F> {
  pub(super) fn derive_decode(&self) -> darling::Result<proc_macro2::TokenStream> {
    let partial_decoded_object = self.partial_decoded();

    let decode_generics = {
      let mut output = partial_decoded_object.generics().clone();
      output
        .make_where_clause()
        .predicates
        .extend(partial_decoded_object.decode_constraints().iter().cloned());
      output
    };
    let partial_decode_generics = {
      let mut output = partial_decoded_object.generics().clone();
      output.make_where_clause().predicates.extend(
        partial_decoded_object
          .partial_decode_constraints()
          .iter()
          .cloned(),
      );
      output
    };

    let partial_decoded_object_impl = derive_partial_decoded_object_decode(
      self,
      &decode_generics,
      &partial_decode_generics,
    );

    let partial_object_decode_to_partial_decoded_object = derive_partial_object_decode_to_partial_decoded_object(
      self,
      &decode_generics,
      &partial_decode_generics,
    )?;

    let partial_object_decode_impl = derive_partial_object_decode(self)?;

    Ok(quote! {
      #partial_object_decode_impl
      #partial_object_decode_to_partial_decoded_object
      #partial_decoded_object_impl
    })
  }
}

fn derive_partial_object_decode_to_partial_decoded_object<M, F>(
  object: &super::ConcreteObject<M, F>,
  decode_generics: &syn::Generics,
  partial_decode_generics: &syn::Generics,
) -> darling::Result<proc_macro2::TokenStream> {
  let partial_decoded_object = object.partial_decoded();
  let partial_decoded_object_ty = partial_decoded_object.ty();
  let partial_object = object.partial();
  let partial_object_ty = partial_object.ty();

  let mut decode_generics = decode_generics.clone();
  let mut partial_decode_generics = partial_decode_generics.clone();
  
  if let Some(preds) = partial_object.generics().where_clause.as_ref().map(|wc| &wc.predicates) {
    decode_generics.make_where_clause().predicates.extend(preds.iter().cloned());
    partial_decode_generics.make_where_clause().predicates.extend(preds.iter().cloned());
  }

  let (decode_ig, _, decode_where_clauses) = decode_generics.split_for_impl();
  let (partial_decode_ig, _, partial_decode_where_clauses) =
    partial_decode_generics.split_for_impl();

  let path_to_grost = object.path_to_grost();
  let lt = &object.lifetime_param().lifetime;
  let ubg = &object.unknown_buffer_param().ident;
  let read_buffer_ident = &object.read_buffer_param().ident;
  let flavor_ty = object.flavor_type();
  let selector_ty = object.selector().ty();

  let decode_trait = partial_object.applied_decode_trait(partial_decoded_object_ty)?;
  let partial_decode_trait = partial_object.applied_partial_decode_trait(partial_decoded_object_ty)?;

  Ok(quote! {
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl #decode_ig #decode_trait for #partial_object_ty #decode_where_clauses {
      fn decode<#read_buffer_ident>(
        context: &<#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Context,
        src: #read_buffer_ident,
      ) -> ::core::result::Result<(::core::primitive::usize, #partial_decoded_object_ty), <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Error>
      where
        Self: ::core::marker::Sized + #lt,
        #read_buffer_ident: #path_to_grost::__private::buffer::ReadBuf<#lt>,
        #ubg: #path_to_grost::__private::buffer::Buffer<<#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Unknown<#read_buffer_ident>> + #lt
      {
        <#partial_decoded_object_ty as #decode_trait>::decode(context, src)
      }
    }

    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl #partial_decode_ig #partial_decode_trait for #partial_object_ty #partial_decode_where_clauses {
      fn partial_decode<#read_buffer_ident>(
        context: &<#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Context,
        src: #read_buffer_ident,
        selector: &#selector_ty,
      ) -> ::core::result::Result<(::core::primitive::usize, ::core::option::Option<#partial_decoded_object_ty>), <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Error>
      where
        Self: ::core::marker::Sized + #lt,
        #read_buffer_ident: #path_to_grost::__private::buffer::ReadBuf<#lt>,
        #ubg: #path_to_grost::__private::buffer::Buffer<<#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Unknown<#read_buffer_ident>> + #lt
      {
        <#partial_decoded_object_ty as #partial_decode_trait>::partial_decode(context, src, selector)
      }
    }
  })
}

fn derive_partial_object_decode<M, F>(
  object: &super::ConcreteObject<M, F>,
) -> darling::Result<proc_macro2::TokenStream> {
  let partial_object = object.partial();
  let partial_object_ty = partial_object.ty();

  let decode_generics = partial_object.decode_generics();
  let partial_decode_generics = partial_object.partial_decode_generics();

  let (decode_ig, _, decode_where_clauses) = decode_generics.split_for_impl();
  let (partial_decode_ig, _, partial_decode_where_clauses) =
    partial_decode_generics.split_for_impl();

  let path_to_grost = object.path_to_grost();
  let lt = &object.lifetime_param().lifetime;
  let ubg = &object.unknown_buffer_param().ident;
  let read_buffer_ident = &object.read_buffer_param().ident;
  let flavor_ty = object.flavor_type();
  let selector_ty = object.selector().ty();

  let decode_trait = partial_object.applied_decode_trait(quote! { Self })?;
  let partial_decode_trait = partial_object.applied_partial_decode_trait(quote! { Self })?;

  Ok(quote! {
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl #decode_ig #decode_trait for #partial_object_ty #decode_where_clauses {
      fn decode<#read_buffer_ident>(
        context: &<#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Context,
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

    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl #partial_decode_ig #partial_decode_trait for #partial_object_ty #partial_decode_where_clauses {
      fn partial_decode<#read_buffer_ident>(
        context: &<#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Context,
        src: #read_buffer_ident,
        selector: &#selector_ty,
      ) -> ::core::result::Result<(::core::primitive::usize, ::core::option::Option<Self>), <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Error>
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
  decode_generics: &syn::Generics,
  partial_decode_generics: &syn::Generics,
) -> proc_macro2::TokenStream {
  let partial_decoded_object = object.partial_decoded();
  let partial_decoded_object_ty = partial_decoded_object.ty();
  let (decode_ig, _, decode_where_clauses) = decode_generics.split_for_impl();
  let (partial_decode_ig, _, partial_decode_where_clauses) =
    partial_decode_generics.split_for_impl();

  let path_to_grost = object.path_to_grost();
  let lt = &object.lifetime_param().lifetime;
  let ubg = &object.unknown_buffer_param().ident;
  let read_buffer_ident = &object.read_buffer_param().ident;
  let flavor_ty = object.flavor_type();
  let wf = object.wire_format();
  let selector_ty = object.selector().ty();

  quote! {
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl #decode_ig #path_to_grost::__private::Decode<#lt, #flavor_ty, #wf, Self, #ubg> for #partial_decoded_object_ty #decode_where_clauses {
      fn decode<#read_buffer_ident>(
        context: &<#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Context,
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

    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl #partial_decode_ig #path_to_grost::__private::PartialDecode<#lt, #flavor_ty, #wf, Self, #ubg> for #partial_decoded_object_ty #partial_decode_where_clauses {
      fn partial_decode<#read_buffer_ident>(
        context: &<#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Context,
        src: #read_buffer_ident,
        selector: &#selector_ty,
      ) -> ::core::result::Result<(::core::primitive::usize, ::core::option::Option<Self>), <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Error>
      where
        Self: ::core::marker::Sized + #lt,
        #read_buffer_ident: #path_to_grost::__private::buffer::ReadBuf<#lt>,
        #ubg: #path_to_grost::__private::buffer::Buffer<<#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Unknown<#read_buffer_ident>> + #lt
      {
        ::core::todo!()
      }
    }
  }
}
