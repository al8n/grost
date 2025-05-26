use std::num::NonZeroU32;

use quote::{ToTokens, format_ident, quote};
use syn::{
  Attribute, GenericParam, Generics, Ident, LifetimeParam, Type, TypeGenerics, TypeParam,
  Visibility, WherePredicate, parse::Parser, parse_quote, punctuated::Punctuated, token::Comma,
};

use crate::ast::{
  grost_flavor_param, grost_lifetime, grost_unknown_buffer_param,
  object::{Field as _, ObjectExt as _},
};

use super::{super::wire_format_reflection_ty, Object};

#[derive(Clone, derive_more::Debug)]
pub struct PartialDecodedField {
  field: syn::Field,
  tag: NonZeroU32,
  wire: Type,
  object_type: Type,
  output_type: Type,
  #[debug(skip)]
  constraints: Punctuated<WherePredicate, Comma>,
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

  /// Returns the default constraints of the field.
  #[inline]
  pub const fn constraints(&self) -> &Punctuated<WherePredicate, Comma> {
    &self.constraints
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
  ty: Type,
  vis: Visibility,
  object_generics: Generics,
  generics: PartialDecodedObjectGenerics,
  fields: Vec<PartialDecodedField>,
  skipped_fields: Vec<syn::Field>,
  attrs: Vec<Attribute>,
  unknown_buffer_field_name: Ident,
  copy: bool,
}

impl PartialDecodedObject {
  /// Returns the name of the decoded object.
  #[inline]
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  /// Returns the type of the partial object.
  ///
  /// e.g. if the [`name`](PartialDecodedObject::name) returns `PartialDecodedUser`, the type will be `PartialDecodedUser<'__grost_lifetime__, __GROST_FLAVOR__, __GROST_UNKNOWN_BUFFER__>`.
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns a type which replace the corresponding generic parameters with the given lifetime or concrete types.
  ///
  /// e.g. if the [`name`](PartialDecodedObject::name) returns `PartialDecodedUser`,
  /// and the given flavor type is `grost::flavors::Network` and the given unknown buffer type is `()`,
  /// the output type will be `PartialDecodedUser<'__grost_lifetime__, grost::flavors::Network, ()>`.
  pub fn type_with(
    &self,
    lifetime: Option<&syn::Lifetime>,
    flavor: Option<&Type>,
    unknown_buffer: Option<&Type>,
  ) -> syn::Result<Type> {
    let iter = self.generics.params.iter().map(|param| match param {
      GenericParam::Lifetime(lt) if lt.lifetime.eq(self.lifetime()) && lifetime.is_some() => {
        quote! { #lifetime }
      }
      GenericParam::Lifetime(lt) => {
        let lt = &lt.lifetime;
        quote! { #lt }
      }
      GenericParam::Type(tp)
        if tp.ident == self.generics.flavor_param().ident && flavor.is_some() =>
      {
        quote! { #flavor }
      }
      GenericParam::Type(tp)
        if tp.ident == self.generics.unknown_buffer_param().ident && unknown_buffer.is_some() =>
      {
        quote! { #unknown_buffer }
      }
      GenericParam::Type(tp) => {
        let ident = &tp.ident;
        quote! { #ident }
      }
      GenericParam::Const(cp) => {
        let ident = &cp.ident;
        quote! { #ident }
      }
    });

    let name = self.name();
    syn::parse2(quote! {
      #name <#(#iter),*>
    })
  }

  /// Returns a new generics which replaces the corresponding generic parameters with the given lifetime or concrete types.
  ///
  /// e.g. if the [`name`](PartialDecodedObject::name) returns `PartialDecodedUser`,
  /// and the given flavor type is `grost::flavors::Network` and the given unknown buffer type is `()`,
  /// the output generics will remove the flavor and unknown buffer generic parameters in both the params and where clause.
  pub fn remove_generics(
    &self,
    lifetime: Option<&syn::Lifetime>,
    flavor: Option<&Type>,
    unknown_buffer: Option<&Type>,
  ) -> syn::Result<Generics> {
    let mut generics = Generics::default();
    self.generics.params.iter().for_each(|param| match param {
      GenericParam::Lifetime(lt) if lt.lifetime.eq(self.lifetime()) && lifetime.is_some() => {
        generics
          .params
          .push(GenericParam::Lifetime(LifetimeParam::new(
            lifetime.unwrap().clone(),
          )));
      }
      GenericParam::Type(tp)
        if tp.ident == self.generics.flavor_param().ident && flavor.is_some() => {}
      GenericParam::Type(tp)
        if tp.ident == self.generics.unknown_buffer_param().ident && unknown_buffer.is_some() => {}
      _ => {
        generics.params.push(param.clone());
      }
    });

    let original_generics = &self.object_generics;
    if let Some(ref where_clause) = self.generics.where_clause {
      where_clause.predicates.iter().for_each(|predicate| {
        if let syn::WherePredicate::Lifetime(predicate_lifetime) = predicate {
          if !(predicate_lifetime.lifetime.eq(self.lifetime()) && lifetime.is_some()) {
            generics
              .make_where_clause()
              .predicates
              .push(predicate.clone());
          } else {
            let mut p = predicate.clone();
            if let syn::WherePredicate::Lifetime(ref mut lt) = p {
              lt.lifetime = lifetime.unwrap().clone();
            }
            generics.make_where_clause().predicates.push(p);
          }
        }
      });
    }

    if let Some(where_clause) = original_generics.where_clause.as_ref() {
      generics
        .make_where_clause()
        .predicates
        .extend(where_clause.predicates.iter().cloned());
    }

    for p in self.constraints_with(lifetime, flavor, unknown_buffer)? {
      generics.make_where_clause().predicates.push(p);
    }

    Ok(generics)
  }

  /// Returns fields which replaces the generic parameters with the given lifetime or concrete types.
  #[inline]
  pub fn fields_with(
    &self,
    lifetime: Option<&syn::Lifetime>,
    flavor: Option<&Type>,
    unknown_buffer: Option<&Type>,
  ) -> syn::Result<Vec<PartialDecodedField>> {
    let path_to_grost = &self.path_to_grost;
    self
      .fields()
      .iter()
      .map(|f| {
        let ty = f.object_type();
        let tag = f.tag();
        let flavor = flavor.map(|t| quote!(#t)).unwrap_or_else(|| {
          let flavor_param = self.flavor_param();
          let ident = &flavor_param.ident;
          quote!(#ident)
        });
        let unknown_buffer = unknown_buffer.map(|t| quote!(#t)).unwrap_or_else(|| {
          let unknown_buffer_param = self.unknown_buffer_param();
          let ident = &unknown_buffer_param.ident;
          quote!(#ident)
        });
        let wf = wire_format_reflection_ty(
          path_to_grost,
          &self.parent_name,
          &self.object_generics.split_for_impl().1,
          tag.get(),
          &flavor,
        );
        let decoded_state = decoded_state_ty(
          path_to_grost,
          &self.parent_name,
          &self.object_generics.split_for_impl().1,
          &wf,
          &flavor,
          lifetime.unwrap_or_else(|| self.lifetime()),
          &unknown_buffer,
        );
        let vis = &f.field.vis;
        let name = f.name();
        let output_type = syn::parse2(quote! { <#ty as #decoded_state>::Output })?;
        let field = syn::Field::parse_named.parse2(quote! {
          #vis #name: ::core::option::Option<#output_type>
        })?;

        let constraints = constraints(
          path_to_grost,
          &self.parent_name,
          &self.object_generics.split_for_impl().1,
          ty,
          &wf,
          &decoded_state,
          &flavor,
          f.copy(),
        )?
        .collect();

        Ok(PartialDecodedField {
          field,
          tag,
          object_type: ty.clone(),
          output_type,
          constraints,
          wire: wf,
          copy: f.copy(),
        })
      })
      .collect()
  }

  /// Returns the constraints which replaces the generic parameters with the given lifetime or concrete types.
  #[inline]
  pub fn constraints_with(
    &self,
    lifetime: Option<&syn::Lifetime>,
    flavor: Option<&Type>,
    unknown_buffer: Option<&Type>,
  ) -> syn::Result<Punctuated<WherePredicate, Comma>> {
    let object_name = &self.parent_name;
    let object_type_generics = self.object_generics.split_for_impl().1;
    let lt = lifetime.unwrap_or_else(|| self.lifetime());
    let flavor = flavor.map(|t| quote!(#t)).unwrap_or_else(|| {
      let flavor_param = self.flavor_param();
      let ident = &flavor_param.ident;
      quote!(#ident)
    });
    let unknown_buffer = unknown_buffer.map(|t| quote!(#t)).unwrap_or_else(|| {
      let unknown_buffer_param = self.unknown_buffer_param();
      let ident = &unknown_buffer_param.ident;
      quote!(#ident)
    });
    let path_to_grost = &self.path_to_grost;
    let mut predicates = Punctuated::new();

    self.fields.iter().try_for_each(|f| {
      let object_field_ty = f.object_type();
      let wf = wire_format_reflection_ty(
        path_to_grost,
        object_name,
        &object_type_generics,
        f.tag.get(),
        &flavor,
      );
      let decoded_state = decoded_state_ty(
        path_to_grost,
        object_name,
        &object_type_generics,
        &wf,
        &flavor,
        lt,
        &unknown_buffer,
      );

      for p in constraints(
        path_to_grost,
        object_name,
        &object_type_generics,
        object_field_ty,
        &wf,
        &decoded_state,
        &flavor,
        f.copy,
      )? {
        predicates.push(p);
      }
      syn::Result::Ok(())
    })?;

    Ok(predicates)
  }

  #[inline]
  pub const fn path(&self) -> &syn::Path {
    &self.path_to_grost
  }

  #[inline]
  pub const fn vis(&self) -> &Visibility {
    &self.vis
  }

  /// Returns the default generic parameters of the partial object.
  #[inline]
  pub const fn generics(&self) -> &Generics {
    &self.generics.generics
  }

  /// Returns the fields of the partial decoded object.
  #[inline]
  pub const fn fields(&self) -> &[PartialDecodedField] {
    self.fields.as_slice()
  }

  /// Returns the skipped fields of the partial decoded object.
  #[inline]
  pub const fn skipped_fields(&self) -> &[syn::Field] {
    self.skipped_fields.as_slice()
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

  /// Returns the field name of the unknown buffer.
  #[inline]
  pub const fn unknown_buffer_field_name(&self) -> &Ident {
    &self.unknown_buffer_field_name
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

    generics.params.push(syn::GenericParam::Type({
      let ident = &flavor_param.ident;
      syn::parse2(quote! {
        #ident: ?::core::marker::Sized + #path_to_grost::__private::flavors::Flavor
      })?
    }));

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
      input.name(),
      input.generics(),
      &mut generics,
      path_to_grost,
      fields.iter().filter(|f| !f.meta().skip()).copied(),
      &flavor_param,
      &lt,
      &unknown_buffer_param,
      copyable,
    )?;

    let generics =
      PartialDecodedObjectGenerics::new(lt, flavor_param, unknown_buffer_param, generics);

    let (_, object_tg, _) = input.generics().split_for_impl();
    let mut partial_decoded_fields = vec![];
    let mut skipped_fields = vec![];
    fields.iter().try_for_each(|f| {
      let ty = f.ty();
      let meta = f.meta();
      if meta.skip() {
        let field_name = f.name();
        let field_ty = f.ty();
        let vis = f.vis();
        let attrs = f.meta().partial_decoded().attrs();
        let field = syn::Field::parse_named.parse2(quote! {
          #(#attrs)*
          #vis #field_name: ::core::option::Option<#field_ty>
        })?;
        skipped_fields.push(field);
        return syn::Result::Ok(());
      }

      let tag = meta.tag().expect("field must have a tag");
      let wf = wire_format_reflection_ty(
        path_to_grost,
        input.name(),
        &object_tg,
        tag.get(),
        &generics.flavor_param().ident,
      );
      let decoded_state = decoded_state_ty(
        path_to_grost,
        input.name(),
        &object_tg,
        &wf,
        &generics.flavor_param().ident,
        generics.lifetime(),
        &generics.unknown_buffer_param().ident,
      );
      let vis = f.vis();
      let name = f.name();
      let attrs = f.meta().partial_decoded().attrs();
      let output_type = syn::parse2(quote! { <#ty as #decoded_state>::Output })?;
      let field = syn::Field::parse_named.parse2(quote! {
        #(#attrs)*
        #vis #name: ::core::option::Option<#output_type>
      })?;

      let constraints = constraints(
        path_to_grost,
        input.name(),
        &object_tg,
        ty,
        &wf,
        &decoded_state,
        &generics.flavor_param().ident,
        f.meta().partial_decoded().copy() || copyable,
      )?
      .collect();

      partial_decoded_fields.push(PartialDecodedField {
        field,
        tag,
        object_type: ty.clone(),
        output_type,
        constraints,
        wire: wf,
        copy: meta.partial_decoded().copy() | copyable,
      });
      Ok(())
    })?;

    let name = input.partial_decoded_name();
    let (_, tg, _) = generics.split_for_impl();
    Ok(Self {
      parent_name: input.name().clone(),
      ty: syn::parse2(quote! {
        #name #tg
      })?,
      unknown_buffer_field_name: format_ident!("__grost_unknown_buffer__"),
      path_to_grost: path_to_grost.clone(),
      name,
      vis: input.vis().clone(),
      object_generics: input.generics().clone(),
      generics,
      fields: partial_decoded_fields,
      skipped_fields,
      attrs: meta.partial_decoded().attrs().to_vec(),
      copy: copyable,
    })
  }

  pub(super) fn to_token_stream(&self) -> proc_macro2::TokenStream {
    let name = self.name();
    let vis = self.vis();
    let fields = self
      .fields()
      .iter()
      .map(PartialDecodedField::field)
      .chain(self.skipped_fields().iter());
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
    let fields_init = partial_decoded_object
      .fields()
      .iter()
      .map(|f| {
        let field_name = f.name();
        quote! {
          #field_name: ::core::option::Option::None,
        }
      })
      .chain(partial_decoded_object.skipped_fields().iter().map(|f| {
        let field_name = f.ident.as_ref().unwrap();
        quote! {
          #field_name: ::core::option::Option::None,
        }
      }));

    let fields_accessors = partial_decoded_object.fields().iter().map(|f| {
      let field_name = f.name();
      let ty = &f.output_type;
      super::optional_accessors(field_name, ty, f.copy())
    });

    let is_empty = partial_decoded_object.fields().iter().map(|f| {
      let field_name = f.name();
      quote! {
        self.#field_name.is_none()
      }
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

        /// Returns `true` if the partial struct is empty, which means all fields are `None`.
        #[inline]
        pub const fn is_empty(&self) -> bool {
          self.#ubfn.is_none() && #(#is_empty)&&*
        }

        /// Returns a reference to the unknown buffer, which holds the unknown data when decoding.
        #[inline]
        pub const fn unknown_buffer(&self) -> ::core::option::Option<&#ubg> {
          self.#ubfn.as_ref()
        }

        // TODO(al8n): the following fns may lead to name conflicts if the struct has field whose name is unknown_buffer
        // /// Returns a mutable reference to the unknown buffer, which holds the unknown data when decoding.
        // #[inline]
        // pub const fn unknown_buffer_mut(&mut self) -> ::core::option::Option<&mut #ubg> {
        //  self.#ubfn.as_mut()
        // }

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
  object_name: &Ident,
  object_type_generics: &TypeGenerics<'_>,
  wf: &syn::Type,
  flavor: impl ToTokens,
  lifetime: &syn::Lifetime,
  unknown_buffer: impl ToTokens,
) -> syn::Type {
  parse_quote! {
    #path_to_grost::__private::convert::State<
      #path_to_grost::__private::convert::Decoded<
        #lifetime,
        #flavor,
        <#wf as #path_to_grost::__private::reflection::Reflectable<#object_name #object_type_generics>>::Reflection,
        #unknown_buffer,
      >
    >
  }
}

#[allow(clippy::too_many_arguments)]
fn constraints(
  path_to_grost: &syn::Path,
  object_name: &syn::Ident,
  object_type_generics: &TypeGenerics<'_>,
  ty: &syn::Type,
  wf: &syn::Type,
  decoded_state: &syn::Type,
  flavor: impl ToTokens,
  copy: bool,
) -> syn::Result<impl Iterator<Item = WherePredicate>> {
  let copy_constraint = copy.then(|| quote! { + ::core::marker::Copy });

  Ok(
    [
      syn::parse2(quote! {
        #wf: #path_to_grost::__private::reflection::Reflectable<#object_name #object_type_generics>
      })?,
      syn::parse2(quote! {
        <#wf as #path_to_grost::__private::reflection::Reflectable<#object_name #object_type_generics>>::Reflection:
          #path_to_grost::__private::flavors::WireFormat<#flavor>
      })?,
      syn::parse2(quote! {
        #ty: #decoded_state
      })?,
      syn::parse2(quote! {
        <#ty as #decoded_state>::Output: ::core::marker::Sized #copy_constraint
      })?,
    ]
    .into_iter(),
  )
}

#[allow(clippy::too_many_arguments)]
fn add_partial_decoded_constraints<'a, I>(
  object_name: &syn::Ident,
  object_generics: &Generics,
  generics: &mut Generics,
  path_to_grost: &syn::Path,
  mut fields: impl Iterator<Item = &'a I>,
  flavor_param: &syn::TypeParam,
  lifetime: &syn::Lifetime,
  unknown_buffer: &syn::TypeParam,
  copy: bool,
) -> darling::Result<()>
where
  I: crate::ast::object::Field + 'a,
{
  let (_, tg, _) = object_generics.split_for_impl();
  fields.try_for_each(move |f| {
    let ty = f.ty();
    let meta = f.meta();
    let wf = wire_format_reflection_ty(
      path_to_grost,
      object_name,
      &tg,
      meta.tag().expect("field tag is required").get(),
      &flavor_param.ident,
    );
    let decoded_state = decoded_state_ty(
      path_to_grost,
      object_name,
      &tg,
      &wf,
      &flavor_param.ident,
      lifetime,
      &unknown_buffer.ident,
    );

    generics.make_where_clause().predicates.extend(constraints(
      path_to_grost,
      object_name,
      &tg,
      ty,
      &wf,
      &decoded_state,
      &flavor_param.ident,
      f.meta().partial_decoded().copy() || copy,
    )?);

    Ok(())
  })
}
