use std::num::NonZeroU32;

use quote::{format_ident, quote};
use syn::{
  Attribute, GenericParam, Generics, Ident, Type, TypeParam, Visibility, parse::Parser, parse_quote,
};

use crate::ast::{
  grost_flavor_param, grost_lifetime, grost_unknown_buffer_param,
  object::{Field as _, ObjectExt as _},
};

use super::{super::wire_format_reflection_ty, Object};

#[derive(Debug, Clone)]
pub struct PartialDecodedField {
  field: syn::Field,
  tag: NonZeroU32,
  wire: Type,
  object_type: Type,
  output_type: Type,
  copy: bool,
}

impl PartialDecodedField {
  /// Returns the name of the field
  #[inline]
  pub const fn name(&self) -> &Ident {
    self.field.ident.as_ref().unwrap()
  }

  /// Returns the field tag.
  #[inline]
  pub const fn tag(&self) -> NonZeroU32 {
    self.tag
  }

  /// Returns the type of the field
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.field.ty
  }

  /// Returns the corresponding field type of the original object
  #[inline]
  pub const fn object_type(&self) -> &Type {
    &self.object_type
  }

  /// Returns the type of the field without `Option`
  #[inline]
  pub const fn output_type(&self) -> &Type {
    &self.output_type
  }

  /// Returns the field wire format type.
  #[inline]
  pub const fn wire(&self) -> &Type {
    &self.wire
  }

  /// Returns the field
  #[inline]
  pub const fn field(&self) -> &syn::Field {
    &self.field
  }

  /// Returns whether the field is copyable.
  #[inline]
  pub const fn copy(&self) -> bool {
    self.copy
  }
}

/// The generic parameters of the [`PartialDecodedObject`].
#[derive(Debug, Clone)]
struct PartialDecodedObjectGenerics {
  generics: Generics,
  lifetime: syn::Lifetime,
  unknown_buffer_generic: TypeParam,
  flavor_generic: TypeParam,
}

impl core::ops::Deref for PartialDecodedObjectGenerics {
  type Target = Generics;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.generics
  }
}

impl PartialDecodedObjectGenerics {
  const fn new(
    lifetime: syn::Lifetime,
    flavor_generic: TypeParam,
    unknown_buffer_generic: TypeParam,
    generics: Generics,
  ) -> Self {
    Self {
      generics,
      lifetime,
      flavor_generic,
      unknown_buffer_generic,
    }
  }

  /// Returns the lifetime generic parameter of the partial object.
  #[inline]
  pub const fn lifetime(&self) -> &syn::Lifetime {
    &self.lifetime
  }

  /// Returns the unknown buffer generic parameter of the partial object.
  #[inline]
  pub const fn unknown_buffer_param(&self) -> &TypeParam {
    &self.unknown_buffer_generic
  }

  /// Returns the flavor generic parameter of the partial object.
  #[inline]
  pub const fn flavor_param(&self) -> &TypeParam {
    &self.flavor_generic
  }
}

#[derive(Debug, Clone)]
pub struct PartialDecodedObject {
  parent_name: Ident,
  path_to_grost: syn::Path,
  name: Ident,
  vis: Visibility,
  generics: PartialDecodedObjectGenerics,
  fields: Vec<PartialDecodedField>,
  attrs: Vec<Attribute>,
  unknown_buffer_field_name: Ident,
  copy: bool,
}

impl PartialDecodedObject {
  #[inline]
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  #[inline]
  pub const fn path_to_grost(&self) -> &syn::Path {
    &self.path_to_grost
  }

  #[inline]
  pub const fn vis(&self) -> &Visibility {
    &self.vis
  }

  #[inline]
  pub const fn generics(&self) -> &Generics {
    &self.generics.generics
  }

  #[inline]
  pub fn fields(&self) -> &[PartialDecodedField] {
    self.fields.as_slice()
  }

  #[inline]
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns the grost lifetime generic parameter of the partial object.
  #[inline]
  pub const fn lifetime(&self) -> &syn::Lifetime {
    self.generics.lifetime()
  }

  /// Returns unknown buffer generic parameter of the partial object.
  #[inline]
  pub const fn unknown_buffer_param(&self) -> &TypeParam {
    self.generics.unknown_buffer_param()
  }

  /// Returns the flavor generic parameter of the partial object.
  #[inline]
  pub const fn flavor_param(&self) -> &TypeParam {
    self.generics.flavor_param()
  }

  #[inline]
  pub const fn copy(&self) -> bool {
    self.copy
  }

  pub(super) fn from_input<O>(path_to_grost: &syn::Path, input: &O) -> darling::Result<Self>
  where
    O: crate::ast::object::Object,
  {
    let fields = input.fields();
    let meta = input.meta();
    let copyable =
      meta.partial_decoded().copy() | fields.iter().all(|f| f.meta().partial_decoded().copy());
    let reflection_name = input.reflection_name();
    let mut generics = Generics::default();
    let lt = grost_lifetime();
    let flavor_param = grost_flavor_param();
    let unknown_buffer_param = grost_unknown_buffer_param();
    let original_generics = input.generics();

    // push the lifetime generic parameter first
    generics.params.extend(
      original_generics
        .params
        .iter()
        .filter(|param| matches!(param, GenericParam::Lifetime(_)))
        .cloned(),
    );

    generics
      .params
      .push(syn::GenericParam::Lifetime(syn::LifetimeParam::new(
        lt.clone(),
      )));

    // push the original type generic parameters
    generics.params.extend(
      original_generics
        .params
        .iter()
        .filter(|param| matches!(param, GenericParam::Type(_)))
        .cloned(),
    );

    generics
      .params
      .push(syn::GenericParam::Type(flavor_param.clone()));

    generics
      .params
      .push(syn::GenericParam::Type(unknown_buffer_param.clone()));

    // push the original const generic parameters last
    generics.params.extend(
      original_generics
        .params
        .iter()
        .filter(|param| matches!(param, GenericParam::Const(_)))
        .cloned(),
    );

    if let Some(where_clause) = original_generics.where_clause.as_ref() {
      generics
        .make_where_clause()
        .predicates
        .extend(where_clause.predicates.iter().cloned());
    }

    add_partial_decoded_constraints(
      &mut generics,
      path_to_grost,
      &reflection_name,
      fields.iter().copied(),
      &flavor_param,
      &lt,
      copyable,
    )?;

    let generics =
      PartialDecodedObjectGenerics::new(lt, flavor_param, unknown_buffer_param, generics);

    let fields = fields
      .iter()
      .map(|f| {
        let ty = f.ty();
        let meta = f.meta();
        let tag = meta.tag();
        let wf = wire_format_reflection_ty(
          path_to_grost,
          &reflection_name,
          tag.get(),
          generics.flavor_param(),
        );
        let decoded_state = decoded_state_ty(
          path_to_grost,
          &wf,
          generics.flavor_param(),
          generics.lifetime(),
        );
        let vis = f.vis();
        let name = f.name();
        let attrs = f.meta().partial_decoded().attrs();
        let output_type = syn::parse2(quote! { <#ty as #decoded_state>::Output })?;
        let field = syn::Field::parse_named.parse2(quote! {
          #(#attrs)*
          #vis #name: ::core::option::Option<#output_type>
        })?;

        Ok(PartialDecodedField {
          field,
          tag,
          object_type: ty.clone(),
          output_type,
          wire: wf,
          copy: meta.partial_decoded().copy() | copyable,
        })
      })
      .collect::<Result<Vec<_>, darling::Error>>()?;

    Ok(Self {
      parent_name: input.name().clone(),
      unknown_buffer_field_name: format_ident!("__grost_unknown_buffer__"),
      path_to_grost: path_to_grost.clone(),
      name: input.partial_decoded_name(),
      vis: input.vis().clone(),
      generics,
      fields,
      attrs: meta.partial_decoded().attrs().to_vec(),
      copy: copyable,
    })
  }

  pub(super) fn to_token_stream(&self) -> proc_macro2::TokenStream {
    let name = self.name();
    let vis = self.vis();
    let fields = self.fields().iter().map(PartialDecodedField::field);
    let generics = self.generics();
    let where_clause = generics.where_clause.as_ref();
    let attrs = self.attrs();

    let doc = if !attrs.iter().any(|attr| attr.path().is_ident("doc")) {
      let doc = format!(
        " Partial reference struct for the struct [`{}`]",
        self.parent_name
      );
      quote! {
        #[doc = #doc]
      }
    } else {
      quote! {}
    };
    let ubfn = &self.unknown_buffer_field_name;
    let ubg = &self.unknown_buffer_param().ident;

    quote! {
      #(#attrs)*
      #doc
      #[allow(clippy::type_complexity, non_camel_case_types)]
      #vis struct #name #generics #where_clause
      {
        #ubfn: ::core::option::Option<#ubg>,
        #(#fields),*
      }
    }
  }
}

impl<M> Object<M>
where
  M: crate::ast::object::Object,
{
  pub(super) fn derive_partial_decoded_object(&self) -> proc_macro2::TokenStream {
    let partial_decoded_object = self.partial_decoded();
    let name = partial_decoded_object.name();
    let fields_init = self.fields.iter().map(|f| {
      let field_name = f.name();
      quote! {
        #field_name: ::core::option::Option::None,
      }
    });

    let fields_accessors = partial_decoded_object.fields.iter().map(|f| {
      let field_name = f.name();
      let ty = &f.output_type;
      super::accessors(field_name, ty, f.copy())
    });

    let (ig, tg, where_clauses) = partial_decoded_object.generics().split_for_impl();
    let ubfn = &partial_decoded_object.unknown_buffer_field_name;
    let ubg = &partial_decoded_object.unknown_buffer_param().ident;
    let flatten_state = super::derive_flatten_state(
      &self.path_to_grost,
      partial_decoded_object.generics(),
      partial_decoded_object.name(),
    );

    quote! {
      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl #ig ::core::default::Default for #name #tg #where_clauses
      {
        fn default() -> Self {
          Self::new()
        }
      }

      #flatten_state

      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl #ig #name #tg #where_clauses
      {
        /// Creates an empty partial struct.
        #[inline]
        pub const fn new() -> Self {
          Self {
            #(#fields_init)*
            #ubfn: ::core::option::Option::None,
          }
        }

        /// Returns a reference to the unknown buffer, which holds the unknown data when decoding.
        #[inline]
        pub const fn unknown_buffer(&self) -> ::core::option::Option<&#ubg> {
          self.#ubfn.as_ref()
        }

        /// Returns a mutable reference to the unknown buffer, which holds the unknown data when decoding.
        #[inline]
        pub const fn unknown_buffer_mut(&mut self) -> ::core::option::Option<&mut #ubg> {
          self.#ubfn.as_mut()
        }

        // TODO(al8n): the following fns may lead to name conflicts if the struct has field whose name is unknown_buffer
        // /// Takes the unknown buffer out if the unknown buffer is not `None`.
        // #[inline]
        // pub const fn take_unknown_buffer(&mut self) -> ::core::option::Option<#ubg> {
        //   self.#ubfn.take()
        // }

        // /// Set the value of unknown buffer
        // #[inline]
        // pub fn set_unknown_buffer(&mut self, buffer: #ubg) -> &mut Self {
        //   self.#ubfn = ::core::option::Option::Some(buffer);
        //   self
        // }

        // /// Clears the unknown buffer.
        // #[inline]
        // pub fn clear_unknown_buffer(&mut self) -> &mut Self {
        //   self.#ubfn = ::core::option::Option::None;
        //   self
        // }

        // /// Set the value of unknown buffer
        // #[inline]
        // pub fn with_unknown_buffer(mut self, buffer: #ubg) -> Self {
        //   self.#ubfn = ::core::option::Option::Some(buffer);
        //   self
        // }

        // /// Clears the unknown buffer.
        // #[inline]
        // pub fn without_unknown_buffer(mut self) -> Self {
        //   self.#ubfn = ::core::option::Option::None;
        //   self
        // }

        #(#fields_accessors)*
      }
    }
  }
}

fn decoded_state_ty(
  path_to_grost: &syn::Path,
  wf: &syn::Type,
  flavor_param: &syn::TypeParam,
  lifetime: &syn::Lifetime,
) -> syn::Type {
  let flavor_ident = &flavor_param.ident;
  parse_quote! {
    #path_to_grost::__private::convert::State<
      #path_to_grost::__private::convert::Decoded<
        #lifetime,
        #flavor_ident,
        <#wf as #path_to_grost::__private::reflection::Reflectable<#flavor_ident>>::Reflection,
      >
    >
  }
}

fn add_partial_decoded_constraints<'a, I>(
  generics: &mut syn::Generics,
  path_to_grost: &syn::Path,
  field_reflection: &syn::Ident,
  mut fields: impl Iterator<Item = &'a I>,
  flavor_param: &syn::TypeParam,
  lifetime: &syn::Lifetime,
  copy: bool,
) -> darling::Result<()>
where
  I: crate::ast::object::Field + 'a,
{
  let flavor_ident = &flavor_param.ident;
  fields.try_for_each(move |f| {
    let ty = f.ty();
    let meta = f.meta();
    let wf = wire_format_reflection_ty(
      path_to_grost,
      field_reflection,
      meta.tag().get(),
      flavor_param,
    );
    let decoded_state = decoded_state_ty(path_to_grost, &wf, flavor_param, lifetime);

    let where_clause = generics.make_where_clause();

    let copy_constraint =
      (f.meta().partial_decoded().copy() || copy).then(|| quote! { + ::core::marker::Copy });

    where_clause.predicates.push(syn::parse2(quote! {
      #wf: #path_to_grost::__private::reflection::Reflectable<#flavor_ident>
    })?);
    where_clause.predicates.push(syn::parse2(quote! {
      #ty: #decoded_state
    })?);
    where_clause.predicates.push(syn::parse2(quote! {
      <#ty as #decoded_state>::Output: ::core::marker::Sized #copy_constraint
    })?);

    Ok(())
  })
}
