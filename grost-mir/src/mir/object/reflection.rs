use quote::{format_ident, quote};
use syn::{Ident, parse::Parser};

use crate::ast::{
  SchemaMeta, grost_flavor_param,
  object::{Field, ObjectExt},
};

use super::Object;

pub struct ReflectionField {
  field: syn::Field,
  tag: u32,
  object_ty: syn::Type,
  schema: SchemaMeta,
}

impl ReflectionField {
  pub const fn field(&self) -> &syn::Field {
    &self.field
  }

  pub const fn tag(&self) -> u32 {
    self.tag
  }

  pub const fn object_ty(&self) -> &syn::Type {
    &self.object_ty
  }

  pub const fn schema(&self) -> &SchemaMeta {
    &self.schema
  }
}

pub struct Reflection {
  parent_name: Ident,
  name: Ident,
  vis: syn::Visibility,
  generics: syn::Generics,
  schema: SchemaMeta,
  #[allow(clippy::type_complexity)]
  fields: Vec<Box<dyn Fn(&syn::Type) -> syn::Result<ReflectionField> + 'static>>,
}

impl Reflection {
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  pub const fn vis(&self) -> &syn::Visibility {
    &self.vis
  }

  #[allow(clippy::type_complexity)]
  pub const fn fields(
    &self,
  ) -> &[Box<dyn Fn(&syn::Type) -> syn::Result<ReflectionField> + 'static>] {
    self.fields.as_slice()
  }

  pub(super) fn from_input<O>(input: &O) -> darling::Result<Self>
  where
    O: crate::ast::object::Object,
  {
    let name = input.reflection_name();
    let parent_name = input.name().clone();
    let vis = input.vis().clone();
    let path_to_grost = input.path();
    let object_name = input.name();
    let flavor_param = grost_flavor_param();

    let mut reflection_generics = input.generics().clone();
    reflection_generics
      .params
      .push(syn::GenericParam::Type(flavor_param.clone()));
    let wc = reflection_generics.make_where_clause();
    let fg = &flavor_param.ident;
    let fields = input
      .fields()
      .iter()
      .map(|f| {
        let field_name = f.name().clone();
        let object_name = object_name.clone();
        let field_doc = format!(
          " The reflection to the `{}.{}` field.",
          object_name, field_name
        );
        let tag = f.meta().tag().get();
        let vis = f.vis().clone();
        let generics = input.generics().clone();
        let path_to_grost = path_to_grost.clone();
        let ty = f.ty().clone();
        let schema = f.meta().schema().clone();

        syn::parse2(quote! {
          #path_to_grost::__private::reflection::Reflection<#ty, #path_to_grost::__private::reflection::Type, #fg>: #path_to_grost::__private::reflection::Reflectable<#ty, Reflection = #path_to_grost::__private::reflection::Type>
        })
        .map(|constraints| {
          wc.predicates.push(constraints);

          Box::new(move |flavor: &syn::Type| {
          let (_, tg, _) = generics.split_for_impl();
          syn::Field::parse_named
            .parse2(quote! {
              #[doc = #field_doc]
              #vis #field_name: #path_to_grost::__private::reflection::Reflection<
                #object_name #tg,
                #path_to_grost::__private::reflection::Identified<
                  #path_to_grost::__private::reflection::Field,
                  #tag,
                >,
                #flavor,
              >
            })
            .map(|f| ReflectionField {
              field: f,
              tag,
              object_ty: ty.clone(),
              schema: schema.clone(),
            })
          }) as Box<dyn Fn(&syn::Type) -> syn::Result<ReflectionField> + 'static>
        })
      })
      .collect::<Result<Vec<_>, _>>()?;

    Ok(Self {
      parent_name,
      schema: input.meta().schema().clone(),
      name,
      fields,
      generics: reflection_generics,
      vis,
    })
  }
}

impl<M> Object<M>
where
  M: crate::ast::object::Object,
{
  pub(super) fn derive_reflection(&self) -> syn::Result<proc_macro2::TokenStream> {
    let path_to_grost = &self.path_to_grost;
    let name = self.name();
    let fgp = grost_flavor_param();
    let fg = &fgp.ident;
    let fgty: syn::Type = syn::parse2(quote! { #fg }).unwrap();

    let (ig, tg, wc) = self.generics().split_for_impl();
    let (ig_with_flavor, _, wc_with_flavor) = self.reflection().generics.split_for_impl();
    let mut field_reflectable_impl = vec![];
    let mut field_reflections = vec![];
    let field_reflection_fns = self
      .reflection
      .fields
      .iter()
      .map(|f| {
        (f)(&fgty).map(|field| {
          let field_name = field.field.ident.unwrap();
          let field_name_str = field_name.to_string();
          let doc = format!(" Returns the field reflection of the field `{name}.{field_name}`.",);
          let object_ty = &field.object_ty;
          let ty = &field.field.ty;
          let field_reflection_name = format_ident!("{}_reflection", field_name);
          let schema_name = field.schema.name().unwrap_or(field_name_str.as_str());
          let schema_description = field
            .schema
            .description()
            .unwrap_or_default();
          field_reflections.push(quote! {
            <#ty as #path_to_grost::__private::reflection::Reflectable<#name #tg>>::REFLECTION
          });
          field_reflectable_impl.push(quote! {
            #[automatically_derived]
            #[allow(clippy::type_complexity, non_camel_case_types)]
            impl #ig_with_flavor #path_to_grost::__private::reflection::Reflectable<#name #tg> for #ty #wc_with_flavor
            {
              type Reflection = #path_to_grost::__private::reflection::ObjectField;

              const REFLECTION: &'static Self::Reflection = &{
                #path_to_grost::__private::reflection::ObjectFieldBuilder {
                  name: #field_name_str,
                  ty: ::core::any::type_name::<#object_ty>,
                  schema_name: #schema_name,
                  schema_description: #schema_description,
                  schema_type: <#path_to_grost::__private::reflection::Reflection<#object_ty, #path_to_grost::__private::reflection::Type, #fg> as #path_to_grost::__private::reflection::Reflectable<#object_ty>>::REFLECTION,
                }.build()
              };
            }
          });

          quote! {
            #[doc = #doc]
            #[inline]
            pub const fn #field_reflection_name<#fg>() -> #ty
            where
              #fg: ?::core::marker::Sized + #path_to_grost::__private::flavors::Flavor,
            {
              #path_to_grost::__private::reflection::Reflection::new()
            }
          }
        })
      })
      .collect::<Result<Vec<_>, _>>()?;

    let name_str = name.to_string();
    let schema_name = self.reflection.schema.name().unwrap_or(name_str.as_str());
    let schema_description = self.reflection.schema.description().unwrap_or_default();
    Ok(quote! {
      const _: () = {
        #(#field_reflectable_impl)*

        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl #ig_with_flavor #path_to_grost::__private::reflection::Reflectable<#name #tg> for #path_to_grost::__private::reflection::Reflection<
          #name #tg,
          #path_to_grost::__private::reflection::Type,
          #fg,
        >
        #wc_with_flavor {
          type Reflection = #path_to_grost::__private::reflection::Type;

          const REFLECTION: &'static Self::Reflection = &{
            #path_to_grost::__private::reflection::Type::Object(
              <#path_to_grost::__private::reflection::Reflection<
                #name #tg,
                #path_to_grost::__private::reflection::Object,
                #fg,
              > as #path_to_grost::__private::reflection::Reflectable<#name #tg>>::REFLECTION
            )
          };
        }

        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl #ig_with_flavor #path_to_grost::__private::reflection::Reflectable<#name #tg> for #path_to_grost::__private::reflection::Reflection<
          #name #tg,
          #path_to_grost::__private::reflection::Object,
          #fg,
        >
        #wc_with_flavor
        {
          type Reflection = #path_to_grost::__private::reflection::Object;

          const REFLECTION: &'static Self::Reflection = &{
            #path_to_grost::__private::reflection::ObjectBuilder {
              name: #name_str,
              schema_name: #schema_name,
              schema_description: #schema_description,
              fields: &[
                #(#field_reflections),*
              ],
            }.build()
          };
        }

        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl #ig #name #tg #wc {
          /// Returns the reflection of the struct.
          #[inline]
          pub const fn reflection<#fg>() -> #path_to_grost::__private::reflection::Reflection<
            Self,
            #path_to_grost::__private::reflection::Object,
            #fg,
          >
          where
            #fg: ?::core::marker::Sized + #path_to_grost::__private::flavors::Flavor,
          {
            #path_to_grost::__private::reflection::Reflection::new()
          }

          #(#field_reflection_fns)*
        }
      };
    })
  }
}
