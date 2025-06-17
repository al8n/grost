use quote::quote;

impl<M, F> super::ConcreteObject<M, F> {
  pub(super) fn derive_decode(&self) -> darling::Result<proc_macro2::TokenStream> {
    let partial_decoded_object_impl = derive_partial_decoded_object_decode(self)?;

    let partial_object_decode_to_partial_decoded_object =
      derive_partial_object_decode_to_partial_decoded_object(self)?;

    let partial_object_decode_impl = derive_partial_object_decode(self)?;

    let decoded_state_impl = derive_decoded_state(self)?;

    Ok(quote! {
      #partial_object_decode_impl
      #partial_object_decode_to_partial_decoded_object
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
      impl #ig #path_to_grost::__private::convert::State<#decoded_state_type> for #partial_ty #where_clauses {
        type Input = &#lt [::core::primitive::u8];
        type Output = #partial_decoded_object_ty;
      }
    }
  };

  Ok(quote! {
    impl #ig #path_to_grost::__private::convert::State<#decoded_state_type> for #ty #where_clauses {
      type Input = &#lt [::core::primitive::u8];
      type Output = #partial_decoded_object_ty;
    }

    #partial_object_impl

    impl #ig #path_to_grost::__private::convert::State<#decoded_state_type> for #partial_decoded_object_ty #where_clauses {
      type Input = &#lt [::core::primitive::u8];
      type Output = Self;
    }
  })
}

fn derive_partial_object_decode_to_partial_decoded_object<M, F>(
  object: &super::ConcreteObject<M, F>,
) -> darling::Result<proc_macro2::TokenStream> {
  let partial_decoded_object = object.partial_decoded();
  let partial_decoded_object_ty = partial_decoded_object.ty();
  let partial_object = object.partial();
  let partial_object_ty = partial_object.ty();

  let mut decode_generics = partial_decoded_object.decode_generics().clone();
  let mut partial_decode_generics = partial_decoded_object.partial_decode_generics().clone();

  if let Some(preds) = partial_object
    .generics()
    .where_clause
    .as_ref()
    .map(|wc| &wc.predicates)
  {
    if !preds.is_empty() {
      decode_generics
        .make_where_clause()
        .predicates
        .extend(preds.iter().cloned());
      partial_decode_generics
        .make_where_clause()
        .predicates
        .extend(preds.iter().cloned());
    }
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
  let partial_decode_trait =
    partial_object.applied_partial_decode_trait(partial_decoded_object_ty)?;

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
) -> darling::Result<proc_macro2::TokenStream> {
  let partial_decoded_object = object.partial_decoded();
  let partial_decoded_object_ty = partial_decoded_object.ty();
  let (decode_ig, _, decode_where_clauses) =
    partial_decoded_object.decode_generics().split_for_impl();
  let (partial_decode_ig, _, partial_decode_where_clauses) = partial_decoded_object
    .partial_decode_generics()
    .split_for_impl();

  let path_to_grost = object.path_to_grost();
  let lt = &object.lifetime_param().lifetime;
  let ubg = &object.unknown_buffer_param().ident;
  let read_buffer_ident = &object.read_buffer_param().ident;
  let flavor_ty = object.flavor_type();
  let selector_ty = object.selector().ty();
  let decode_trait = partial_decoded_object.applied_decode_trait(quote! { Self })?;
  let partial_decode_trait =
    partial_decoded_object.applied_partial_decode_trait(quote! { Self })?;
  let object_ty = object.ty();
  let object_decode_trait =
    partial_decoded_object.applied_decode_trait(partial_decoded_object_ty)?;
  let object_partial_decode_trait =
    partial_decoded_object.applied_partial_decode_trait(partial_decoded_object_ty)?;

  Ok(quote! {
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl #decode_ig #decode_trait for #partial_decoded_object_ty #decode_where_clauses {
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
    impl #partial_decode_ig #partial_decode_trait for #partial_decoded_object_ty #partial_decode_where_clauses {
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

    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl #decode_ig #object_decode_trait for #object_ty #decode_where_clauses {
      fn decode<#read_buffer_ident>(
        context: &<#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Context,
        src: #read_buffer_ident,
      ) -> ::core::result::Result<(::core::primitive::usize, #partial_decoded_object_ty), <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Error>
      where
        Self: ::core::marker::Sized + #lt,
        #read_buffer_ident: #path_to_grost::__private::buffer::ReadBuf<#lt>,
        #ubg: #path_to_grost::__private::buffer::Buffer<<#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Unknown<#read_buffer_ident>> + #lt
      {
        <#partial_decoded_object_ty as #object_decode_trait>::decode(context, src)
      }
    }

    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl #partial_decode_ig #object_partial_decode_trait for #object_ty #partial_decode_where_clauses {
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
        <#partial_decoded_object_ty as #object_partial_decode_trait>::partial_decode(context, src, selector)
      }
    }
  })
}
