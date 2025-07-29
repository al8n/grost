// use quote::{format_ident, quote};

// use crate::{object::ast::concrete::Field, utils::grost_decode_trait_lifetime};

// impl<T, S, M> super::Object<T, S, M> {
//   pub(super) fn derive_decode(&self) -> darling::Result<proc_macro2::TokenStream> {
//     let object_impl = derive_object_decode(self)?;
//     let partial_ref_object_impl = derive_partial_ref_object_decode(self)?;
//     let partial_object_decode_impl = derive_partial_object_decode(self)?;

//     Ok(quote! {
//       #object_impl
//       #partial_object_decode_impl
//       #partial_ref_object_impl
//     })
//   }
// }

// fn derive_partial_object_decode<T, S, M>(
//   object: &super::Object<T, S, M>,
// ) -> darling::Result<proc_macro2::TokenStream> {
//   let partial_object = object.partial();
//   let partial_object_ty = partial_object.ty();

//   let decode_generics = partial_object.decode_generics();
//   let (ig, _, where_clauses) = partial_object.generics().split_for_impl();
//   let (transform_from_partial_ig, _, transform_from_partial_where_clauses) =
//     object.transform_partial_generics.split_for_impl();
//   let (decode_ig, _, decode_where_clauses) = decode_generics.split_for_impl();
//   let (decode_object_ig, _, decode_object_where_clauses) = object.decode_generics.split_for_impl();

//   let path_to_grost = object.path_to_grost();
//   let lt = &object.lifetime_param().lifetime;
//   let ubg = &object.buffer_param().ident;
//   let read_buffer_ident = &object.read_buffer_param().ident;
//   let flavor_ty = object.flavor_type();
//   let wf = object.wire_format();

//   let decode_trait = partial_object.applied_decode_trait(quote! { Self })?;
//   let decode_from_partial_trait =
//     partial_object.applied_decode_trait(quote! { #partial_object_ty })?;
//   let decode_to_object_trait = object.applied_decode_trait(quote! { Self })?;
//   let partial_ref_object_ty = object.partial_ref().ty();
//   let object_ty = object.ty();
//   let object_name = object.name();
//   let selector_ty = object.selector().ty();

//   let (partial_transform_from_partial_ig, _, partial_transform_from_partial_where_clauses) =
//     object.partial.partial_transform_generics.split_for_impl();

//   let mut partial_transform_on_missing = vec![];
//   let mut fields_partial_transform = vec![];
//   object
//     .fields()
//     .iter()
//     .filter_map(|f| f.try_unwrap_tagged_ref().ok())
//     .for_each(|f| {
//       let field_name = f.name();
//       let field_ty = f.ty();
//       let field_wf = f.wire_format();
//       let partial_transform_options = f.partial().partial_transform();
//       let partial_field_ty = f.partial().ty();
//       let is_field_selected = format_ident!("is_{}_selected", field_name);
//       let field_selector = format_ident!("{}_ref", field_name);
//       if let Some(missing_operation) = partial_transform_options.missing_operation() {
//         let call = missing_operation.call();
//         partial_transform_on_missing.push(quote! {
//           if selector.#is_field_selected() {
//             if this.#field_name.is_none() {
//               this.#field_name = ::core::option::Option::Some(#call);
//             }
//           }
//         });
//       }

//       fields_partial_transform.push({
//         let nullable = f.label().is_nullable();
//         let call = match partial_transform_options.convert_operation() {
//           None => {
//             if nullable {
//               quote! {
//                 <#field_ty as #path_to_grost::__private::convert::PartialTransform<
//                   #partial_field_ty,
//                   #partial_field_ty,
//                   #field_wf,
//                   #flavor_ty,
//                 >>::partial_transform(this.#field_name, selector.#field_selector())?
//               }
//             } else {
//               quote! {
//                 <#field_ty as #path_to_grost::__private::convert::PartialTransform<
//                   #partial_field_ty,
//                   ::core::option::Option<#partial_field_ty>,
//                   #field_wf,
//                   #flavor_ty,
//                 >>::partial_transform(value, selector.#field_selector())?
//               }
//             }
//           }
//           Some(transform) => transform.call(&[quote!(value), quote!(selector.#field_selector())]),
//         };

//         if f.label().is_nullable() {
//           quote! {
//             if selector.#is_field_selected() {
//               this.#field_name = #call;
//             }
//           }
//         } else {
//           quote! {
//             if let ::core::option::Option::Some(value) = input.#field_name {
//               if selector.#is_field_selected() {
//                 this.#field_name = #call;
//               }
//             }
//           }
//         }
//       });
//     });

//   let fields_transform = object
//     .fields()
//     .iter()
//     .map(|f| {
//       match f {
//         Field::Skipped(f) => {
//           let name = f.name();
//           let default = &f.default;
//           quote! {
//             #name: (#default)()
//           }
//         },
//         Field::Tagged(f) => {
//           let name = f.name();
//           let on_missing = match f.transform().missing_operation() {
//             Some(missing_operation) => {
//               let call = missing_operation.call();
//               quote! {
//                 else {
//                   #call
//                 }
//               }
//             }
//             None => {
//               if f.label().is_list() || f.label().is_set() || f.label().is_map() {
//                 quote! {
//                   else {
//                     ::core::default::Default::default()
//                   }
//                 }
//               } else {
//                 quote! {
//                   else {
//                     return ::core::result::Result::Err(
//                       ::core::convert::Into::into(#path_to_grost::__private::error::Error::field_not_found(
//                         ::core::stringify!(#object_name),
//                         ::core::stringify!(#name),
//                       ))
//                     );
//                   }
//                 }
//               }
//             },
//           };
//           let ty = f.ty();
//           let partial_field_ty = f.partial().ty();
//           let field_wf = f.wire_format();

//           if f.label().is_nullable() {
//             quote! {
//               #name: input.#name
//             }
//           } else {
//             quote! {
//               #name: {
//                 if let ::core::option::Option::Some(value) = input.#name {
//                   <#ty as #path_to_grost::__private::convert::Transform<
//                     #partial_field_ty,
//                     #ty,
//                     #field_wf,
//                     #flavor_ty,
//                   >>::transform(value)?
//                 } #on_missing
//               }
//             }
//           }
//         },
//       }
//     });

//   Ok(quote! {
//     #[automatically_derived]
//     #[allow(non_camel_case_types, clippy::type_complexity)]
//     impl #decode_ig #decode_trait for #partial_object_ty #decode_where_clauses {
//       fn decode(
//         context: &#lt <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Context,
//         src: #read_buffer_ident,
//       ) -> ::core::result::Result<(::core::primitive::usize, Self), <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Error>
//       where
//         Self: ::core::marker::Sized + #lt,
//         #read_buffer_ident: #path_to_grost::__private::buffer::ReadBuf + #lt,
//         #ubg: #path_to_grost::__private::buffer::Buffer<<#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Unknown<#read_buffer_ident>> + #lt
//       {
//         <#partial_object_ty as
//           #path_to_grost::__private::decode::Decode<#lt, #partial_ref_object_ty, #wf, #read_buffer_ident, #ubg, #flavor_ty>
//         >::decode(context, src)
//           .and_then(|(read, input)| {
//             <#partial_object_ty as #path_to_grost::__private::convert::Transform::<
//               #partial_ref_object_ty,
//               #partial_object_ty,
//               #wf,
//               #flavor_ty,
//             >>::transform(input)
//               .map(|input| (read, input))
//           })
//       }
//     }

//     #[automatically_derived]
//     #[allow(non_camel_case_types, clippy::type_complexity)]
//     impl #decode_object_ig #decode_to_object_trait for #object_ty #decode_object_where_clauses {
//       fn decode(
//         context: &#lt <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Context,
//         src: #read_buffer_ident,
//       ) -> ::core::result::Result<(::core::primitive::usize, Self), <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Error>
//       where
//         Self: ::core::marker::Sized + #lt,
//         #read_buffer_ident: #path_to_grost::__private::buffer::ReadBuf + #lt,
//         #ubg: #path_to_grost::__private::buffer::Buffer<<#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Unknown<#read_buffer_ident>> + #lt
//       {
//         <#partial_object_ty as #decode_from_partial_trait>::decode(context, src)
//           .and_then(|(read, input)| {
//             <#object_ty as #path_to_grost::__private::convert::Transform::<#partial_object_ty, #object_ty, #wf, #flavor_ty>>::transform(input)
//               .map(|input| (read, input))
//           })
//       }
//     }

//     #[automatically_derived]
//     #[allow(non_camel_case_types, clippy::type_complexity)]
//     impl #ig #path_to_grost::__private::convert::Transform<Self, Self, #wf, #flavor_ty> for #partial_object_ty #where_clauses {
//       fn transform(input: Self) -> ::core::result::Result<Self, <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Error> {
//         ::core::result::Result::Ok(input)
//       }
//     }

//     #[automatically_derived]
//     #[allow(non_camel_case_types, clippy::type_complexity)]
//     impl #transform_from_partial_ig #path_to_grost::__private::convert::Transform<#partial_object_ty, #object_ty, #wf, #flavor_ty> for #object_ty #transform_from_partial_where_clauses {
//       fn transform(input: #partial_object_ty) -> ::core::result::Result<#object_ty, <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Error> {
//         ::core::result::Result::Ok(Self {
//           #(#fields_transform),*
//         })
//       }
//     }

//     #[automatically_derived]
//     #[allow(non_camel_case_types, clippy::type_complexity)]
//     impl #partial_transform_from_partial_ig #path_to_grost::__private::convert::PartialTransform<Self, ::core::option::Option<Self>, #wf, #flavor_ty> for #partial_object_ty #partial_transform_from_partial_where_clauses {
//       fn partial_transform(input: #partial_object_ty, selector: &#selector_ty) -> ::core::result::Result<::core::option::Option<#partial_object_ty>, <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Error> {
//         let mut this = Self::new();

//         #(#fields_partial_transform)*

//         #(#partial_transform_on_missing)*

//         ::core::result::Result::Ok((!this.is_empty()).then_some(this))
//       }
//     }
//   })
// }

// fn derive_object_decode<T, S, M>(
//   object: &super::Object<T, S, M>,
// ) -> darling::Result<proc_macro2::TokenStream> {
//   let object_ty = object.ty();
//   let (ig, _, where_clauses) = object.generics().split_for_impl();

//   let path_to_grost = object.path_to_grost();
//   let flavor_ty = object.flavor_type();
//   let wf = object.wire_format();

//   Ok(quote! {
//     #[automatically_derived]
//     #[allow(non_camel_case_types, clippy::type_complexity)]
//     impl #ig #path_to_grost::__private::convert::Transform<Self, Self, #wf, #flavor_ty> for #object_ty #where_clauses {
//       fn transform(input: Self) -> ::core::result::Result<Self, <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Error> {
//         ::core::result::Result::Ok(input)
//       }
//     }
//   })
// }

// fn derive_partial_ref_object_decode<T, S, M>(
//   object: &super::Object<T, S, M>,
// ) -> darling::Result<proc_macro2::TokenStream> {
//   let object_ty = object.ty();
//   let object_reflectable = object.reflectable();
//   let selector_ty = object.selector().ty();
//   let partial_object_ty = object.partial().ty();
//   let partial_ref_object = object.partial_ref();
//   let partial_ref_object_ty = partial_ref_object.ty();
//   let (ig, _, where_clauses) = partial_ref_object.generics().split_for_impl();
//   let (decode_ig, _, decode_where_clauses) = partial_ref_object.decode_generics().split_for_impl();

//   let path_to_grost = object.path_to_grost();
//   let ubg = &object.buffer_param().ident;
//   let read_buffer_ident = &object.read_buffer_param().ident;
//   let flavor_ty = object.flavor_type();
//   let decode_to_self_trait = partial_ref_object.applied_decode_trait(quote! { Self })?;
//   let decode_to_partial_ref_trait =
//     partial_ref_object.applied_decode_trait(quote! { #partial_ref_object_ty })?;
//   let lt = grost_decode_trait_lifetime();

//   let buffer_field_name = &partial_ref_object.buffer_field_name;
//   let mut on_missing = vec![];
//   let mut ptpr_on_missing = vec![];
//   let mut ptpr = vec![];
//   let mut pptpr_on_missing = vec![];
//   let mut pptpr = vec![];
//   let mut field_decode_branches = vec![];

//   object
//     .fields()
//     .iter()
//     .filter_map(|f| f.try_unwrap_tagged_ref().ok())
//     .for_each(|f| {
//       let field_name = f.name();
//       let field_identifier = f.reflection().identifier_reflection();
//       let field_ty = f.ty();
//       let field_decode_trait_type = f.partial_ref().decode_trait_type();
//       let field_wf = f.wire_format();
//       let is_field_selected = format_ident!("is_{}_selected", field_name);
//       let field_selector = format_ident!("{}_ref", field_name);
//       let partial_ref_field_ty = f.partial_ref().ty();
//       let nullable = f.label().is_nullable();

//       {
//         let partial_transform_ref_options = f.partial().partial_transform_ref();
//         let partial_field_ty = f.partial().ty();
//         if let Some(missing_operation) = partial_transform_ref_options.missing_operation() {
//           let call = missing_operation.call();
//           pptpr_on_missing.push(quote! {
//             if selector.#is_field_selected() {
//               if this.#field_name.is_none() {
//                 this.#field_name = ::core::option::Option::Some(#call);
//               }
//             }
//           });
//         }

//         pptpr.push({
//           let call = match partial_transform_ref_options.convert_operation() {
//             None => {
//               if nullable {
//                 quote! {
//                   <#field_ty as #path_to_grost::__private::convert::PartialTransform<#partial_ref_field_ty, #partial_field_ty, #field_wf, #flavor_ty>>::partial_transform(input.#field_name, selector.#field_selector())?
//                 }
//               } else {
//                 quote! {
//                   <#field_ty as #path_to_grost::__private::convert::PartialTransform<#partial_ref_field_ty, ::core::option::Option<#partial_field_ty>, #field_wf, #flavor_ty>>::partial_transform(value, selector.#field_selector())?
//                 }
//               }
//             }
//             Some(transform) => {
//               transform.call(&[quote!(value), quote!(selector.#field_selector())])
//             }
//           };

//           if nullable {
//             quote! {
//               if selector.#is_field_selected() {
//                 this.#field_name = #call;
//               }
//             }
//           } else {
//             quote! {
//               if let ::core::option::Option::Some(value) = input.#field_name {
//                 if selector.#is_field_selected() {
//                   this.#field_name = #call;
//                 }
//               }
//             }
//           }
//         });

//         let transform_ref_options = f.partial().transform_ref();
//         if let Some(missing_operation) = transform_ref_options.missing_operation() {
//           let call = missing_operation.call();
//           ptpr_on_missing.push(quote! {
//             if selector.#is_field_selected() {
//               if this.#field_name.is_none() {
//                 this.#field_name = ::core::option::Option::Some(#call);
//               }
//             }
//           });
//         }

//         ptpr.push({
//           let call = match transform_ref_options.convert_operation() {
//             None => {
//               if nullable {
//                 quote! {
//                   <#field_ty as #path_to_grost::__private::convert::Transform<#partial_ref_field_ty, #partial_field_ty, #field_wf, #flavor_ty>>::transform(input.#field_name)?
//                 }
//               } else {
//                 quote! {
//                   <#field_ty as #path_to_grost::__private::convert::Transform<#partial_ref_field_ty, #partial_field_ty, #field_wf, #flavor_ty>>::transform(value)?
//                 }
//               }
//             }
//             Some(transform) => {
//               transform.call(&[quote!(value)])
//             }
//           };

//           if nullable {
//             quote! {
//               this.#field_name = #call;
//             }
//           } else {
//             quote! {
//               if let ::core::option::Option::Some(value) = input.#field_name {
//                 this.#field_name = ::core::option::Option::Some(#call);
//               }
//             }
//           }
//         });
//       }

//       let decode_fn = match f.partial_ref().decode().func() {
//         Some(func) => quote!( #func ),
//         None => quote! {
//           <#field_ty as  #field_decode_trait_type>::decode
//         },
//       };

//       let value = f.partial_ref().decode().then().map(|f| {
//         quote! {
//           let value = (#f)(value)?;
//         }
//       });

//       if let Some(missing_operation) = f.partial_ref().decode().missing_operation() {
//         on_missing.push({
//           let call = missing_operation.call();
//           quote! {
//             if this.#field_name.is_none() {
//               this.#field_name = ::core::option::Option::Some(#call);
//             }
//           }
//         });
//       }

//       let set_value = if nullable {
//         quote! {
//           this.#field_name = value;
//         }
//       } else {
//         quote! {
//           this.#field_name = ::core::option::Option::Some(value);
//         }
//       };

//       let branch = if f.default_wire_format_constraints().is_empty() {
//         quote! { <#field_identifier as #object_reflectable>::REFLECTION }
//       } else {
//         quote! { _ if identifier.eq(<#field_identifier as #object_reflectable>::REFLECTION) }
//       };

//       field_decode_branches.push(quote! {
//         #branch => {
//           if offset >= buf_len {
//             return ::core::result::Result::Err(
//               ::core::convert::Into::into(#path_to_grost::__private::error::Error::buffer_underflow())
//             );
//           }

//           if this.#field_name.is_some() {
//             return ::core::result::Result::Err(
//               ::core::convert::Into::into(#path_to_grost::__private::error::Error::duplicated_field(
//                 stringify!(#field_name),
//                 ::core::any::type_name::<#field_ty>(),
//                 *<#field_identifier as #object_reflectable>::REFLECTION
//               ))
//             );
//           }

//           let (read, value) = (#decode_fn)(context, src.segment(offset..))?;
//           #value
//           #set_value
//           offset += read;
//         },
//       });
//     });

//   let decode_partial_generics = {
//     let mut output = partial_ref_object.decode_generics().clone();
//     if let Some(preds) = object
//       .partial()
//       .generics()
//       .where_clause
//       .as_ref()
//       .map(|wc| &wc.predicates)
//     {
//       output
//         .make_where_clause()
//         .predicates
//         .extend(preds.iter().cloned());
//     }
//     output
//   };
//   let (decode_partial_ig, _, decode_partial_where_clauses) =
//     decode_partial_generics.split_for_impl();
//   let (transform_from_partial_ref_ig, _, transform_from_partial_ref_where_clauses) = object
//     .partial
//     .transform_from_partial_ref_generics
//     .split_for_impl();
//   let (partial_transform_from_partial_ref_ig, _, partial_transform_from_partial_ref_where_clauses) =
//     object
//       .partial
//       .partial_transform_from_partial_ref_generics
//       .split_for_impl();

//   let wf = object.wire_format();

//   Ok(quote! {
//     #[automatically_derived]
//     #[allow(non_camel_case_types, clippy::type_complexity)]
//     impl #ig #path_to_grost::__private::convert::Transform<Self, Self, #wf, #flavor_ty> for #partial_ref_object_ty #where_clauses {
//       fn transform(input: Self) -> ::core::result::Result<Self, <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Error> {
//         ::core::result::Result::Ok(input)
//       }
//     }

//     #[automatically_derived]
//     #[allow(non_camel_case_types, clippy::type_complexity)]
//     impl #decode_ig #decode_to_self_trait for #partial_ref_object_ty #decode_where_clauses {
//       fn decode(
//         context: &#lt <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Context,
//         src: #read_buffer_ident,
//       ) -> ::core::result::Result<(::core::primitive::usize, Self), <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Error>
//       where
//         Self: ::core::marker::Sized + #lt,
//         #read_buffer_ident: #path_to_grost::__private::buffer::ReadBuf + #lt,
//         #ubg: #path_to_grost::__private::buffer::Buffer<<#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Unknown<#read_buffer_ident>> + #lt
//       {
//         let buf = src.remaining_slice();
//         let buf_len = buf.len();
//         let mut this = Self::new();

//         let mut offset = 0;
//         while offset < buf_len {
//           let (read, identifier) = <<#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Identifier as  #path_to_grost::__private::identifier::Identifier<#flavor_ty>>::decode::<&[::core::primitive::u8]>(&buf[offset..])?;
//           offset += read;

//           match &identifier {
//             #(#field_decode_branches)*
//             _ => {
//               if context.err_on_unknown() {
//                 return ::core::result::Result::Err(
//                   ::core::convert::Into::into(
//                     #path_to_grost::__private::error::Error::unknown_identifier(
//                       ::core::any::type_name::<#object_ty>(),
//                       identifier,
//                     )
//                   )
//                 );
//               }

//               if context.skip_unknown() {
//                 if offset >= buf_len {
//                   return ::core::result::Result::Err(::core::convert::Into::into(#path_to_grost::__private::error::Error::buffer_underflow()));
//                 }

//                 offset += <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::skip(context, identifier.wire_type(), src.segment(offset..))?;
//               } else {
//                 let encoded_len = <<#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Identifier as  #path_to_grost::__private::identifier::Identifier<#flavor_ty>>::encoded_len(&identifier);
//                 let (read, unknown) = <#path_to_grost::__private::flavors::Groto as #path_to_grost::__private::flavors::Flavor>::decode_unknown(context, src.segment(offset - encoded_len..))?;
//                 offset += read;
//                 let unknowns_mut = this.#buffer_field_name.get_or_insert_with(|| #ubg::new());

//                 if let ::core::option::Option::Some(unknown) = unknowns_mut.push(unknown) {
//                   let len = #path_to_grost::__private::Buffer::len(unknowns_mut);
//                   return ::core::result::Result::Err(
//                     ::core::convert::Into::into(#path_to_grost::__private::error::Error::buffer_overflow(
//                       len,
//                       ::core::num::NonZeroUsize::new(len + 1).unwrap(),
//                     ))
//                   );
//                 }
//               }
//             }
//           }
//         }

//         #(#on_missing)*

//         ::core::result::Result::Ok((offset, this))
//       }
//     }

//     #[automatically_derived]
//     #[allow(non_camel_case_types, clippy::type_complexity)]
//     impl #decode_ig #decode_to_partial_ref_trait for #object_ty #decode_where_clauses {
//       fn decode(
//         context: &#lt <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Context,
//         src: #read_buffer_ident,
//       ) -> ::core::result::Result<(::core::primitive::usize, #partial_ref_object_ty), <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Error>
//       where
//         #partial_ref_object_ty: ::core::marker::Sized + #lt,
//         #read_buffer_ident: #path_to_grost::__private::buffer::ReadBuf + #lt,
//         #ubg: #path_to_grost::__private::buffer::Buffer<<#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Unknown<#read_buffer_ident>> + #lt
//       {
//         <#partial_ref_object_ty as #decode_to_partial_ref_trait>::decode(context, src)
//       }
//     }

//     #[automatically_derived]
//     #[allow(non_camel_case_types, clippy::type_complexity)]
//     impl #decode_partial_ig #decode_to_partial_ref_trait for #partial_object_ty #decode_partial_where_clauses {
//       fn decode(
//         context: &#lt <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Context,
//         src: #read_buffer_ident,
//       ) -> ::core::result::Result<(::core::primitive::usize, #partial_ref_object_ty), <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Error>
//       where
//         #partial_ref_object_ty: ::core::marker::Sized + #lt,
//         #read_buffer_ident: #path_to_grost::__private::buffer::ReadBuf + #lt,
//         #ubg: #path_to_grost::__private::buffer::Buffer<<#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Unknown<#read_buffer_ident>> + #lt
//       {
//         <#partial_ref_object_ty as #decode_to_partial_ref_trait>::decode(context, src)
//       }
//     }

//     #[automatically_derived]
//     #[allow(non_camel_case_types, clippy::type_complexity)]
//     impl #transform_from_partial_ref_ig #path_to_grost::__private::convert::Transform<#partial_ref_object_ty, #partial_object_ty, #wf, #flavor_ty> for #partial_object_ty #transform_from_partial_ref_where_clauses {
//       fn transform(input: #partial_ref_object_ty) -> ::core::result::Result<Self, <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Error> {
//         let mut this = Self::new();

//         #(#ptpr)*

//         #(#ptpr_on_missing)*

//         ::core::result::Result::Ok(this)
//       }
//     }

//     #[automatically_derived]
//     #[allow(non_camel_case_types, clippy::type_complexity)]
//     impl #partial_transform_from_partial_ref_ig #path_to_grost::__private::convert::PartialTransform<#partial_ref_object_ty, ::core::option::Option<#partial_object_ty>, #wf, #flavor_ty> for #partial_object_ty #partial_transform_from_partial_ref_where_clauses {
//       fn partial_transform(
//         input: #partial_ref_object_ty,
//         selector: &#selector_ty,
//       ) -> ::core::result::Result<::core::option::Option<Self>, <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Error> {
//         let mut this = Self::new();

//         #(#pptpr)*

//         #(#pptpr_on_missing)*

//         ::core::result::Result::Ok((!this.is_empty()).then_some(this))
//       }
//     }
//   })
// }
