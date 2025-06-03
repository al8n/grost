use quote::{ToTokens, format_ident, quote};
use syn::{
  Expr, GenericParam, Generics, Type, TypeParam, WherePredicate, punctuated::Punctuated,
  token::Comma,
};

use crate::ast::object::Field as _;

use super::{Field, Object, TaggedField};

impl TaggedField {
  fn derive_field_reflectable(&self, object: &Object) -> proc_macro2::TokenStream {
    let path_to_grost = object.path_to_grost();
    let object_reflectable = object.reflectable();
    let field = self;
    let field_ty = &field.ty;
    let reflection_type = self.field_reflection.ty();
    let schema_name = field.schema_name();
    let schema_description = field.schema_description();
    let (ig_reflection, _, wc_reflection) = object.reflection.generics().split_for_impl();

    quote! {
      #[automatically_derived]
      #[allow(clippy::type_complexity, non_camel_case_types)]
      impl #ig_reflection #object_reflectable for #reflection_type #wc_reflection {
        type Reflection = #path_to_grost::__private::reflection::ObjectField;

        const REFLECTION: &'static Self::Reflection = &{
          #path_to_grost::__private::reflection::ObjectFieldBuilder {
            name: #schema_name,
            description: #schema_description,
            ty: <#path_to_grost::__private::reflection::TypeReflection<#field_ty> as #path_to_grost::__private::reflection::Reflectable<#field_ty>>::REFLECTION,
          }.build()
        };
      }
    }
  }

  fn derive_field_reflection_fn(
    &self,
    object: &Object,
    flavor_ty: impl ToTokens,
    generic: bool,
  ) -> proc_macro2::TokenStream {
    let path_to_grost = object.path_to_grost();
    let object_name = object.name();
    let field = self;
    let field_name = field.name();
    let doc = format!(" Returns the field reflection of the field `{object_name}.{field_name}`.",);
    let reflection_type = self.field_reflection.ty();
    let field_reflection_name = format_ident!("{}_reflection", field_name);

    if !generic {
      quote! {
        #[doc = #doc]
        #[inline]
        pub const fn #field_reflection_name() -> #reflection_type
        where
          #flavor_ty: #path_to_grost::__private::flavors::Flavor,
        {
          #path_to_grost::__private::reflection::ObjectFieldReflection::new()
        }
      }
    } else {
      quote! {
        #[doc = #doc]
        #[inline]
        pub const fn #field_reflection_name<#flavor_ty>() -> #reflection_type
        where
          #flavor_ty: ?::core::marker::Sized + #path_to_grost::__private::flavors::Flavor,
        {
          #path_to_grost::__private::reflection::ObjectFieldReflection::new()
        }
      }
    }
  }
}

#[derive(Debug, Clone)]
pub struct ConcreteFieldReflection {
  field_reflection: Type,
  wire_format_reflection: Type,
  wire_type_reflection: Type,
  identifier_reflection: Type,
  encoded_identifier_reflection: Type,
  encoded_identifier_len_reflection: Type,
  tag_reflection: Type,
  encoded_tag_reflection: Type,
  encoded_tag_len_reflection: Type,
  value: Expr,
  constraints: Punctuated<WherePredicate, Comma>,
}

impl ConcreteFieldReflection {
  /// Returns the type of the field reflection.
  #[inline]
  pub const fn field_reflection(&self) -> &Type {
    &self.field_reflection
  }

  /// Returns the value of the field reflection.
  #[inline]
  pub const fn field_reflection_value(&self) -> &Expr {
    &self.value
  }

  /// Returns the type of the wire format reflection.
  #[inline]
  pub const fn wire_format_reflection(&self) -> &Type {
    &self.wire_format_reflection
  }

  /// Returns the type of the wire type reflection.
  #[inline]
  pub const fn wire_type_reflection(&self) -> &Type {
    &self.wire_type_reflection
  }

  /// Returns the type of the identifier reflection.
  #[inline]
  pub const fn identifier_reflection(&self) -> &Type {
    &self.identifier_reflection
  }

  /// Returns the type of the encoded identifier reflection.
  #[inline]
  pub const fn encoded_identifier_reflection(&self) -> &Type {
    &self.encoded_identifier_reflection
  }

  /// Returns the type of the encoded identifier length reflection.
  #[inline]
  pub const fn encoded_identifier_len_reflection(&self) -> &Type {
    &self.encoded_identifier_len_reflection
  }

  /// Returns the type of the tag reflection.
  #[inline]
  pub const fn tag_reflection(&self) -> &Type {
    &self.tag_reflection
  }

  /// Returns the type of the encoded tag reflection.
  #[inline]
  pub const fn encoded_tag_reflection(&self) -> &Type {
    &self.encoded_tag_reflection
  }

  /// Returns the type of the encoded tag length reflection.
  #[inline]
  pub const fn encoded_tag_len_reflection(&self) -> &Type {
    &self.encoded_tag_len_reflection
  }

  /// Returns the constraints for this field reflection.
  #[inline]
  pub const fn constraints(&self) -> &Punctuated<WherePredicate, Comma> {
    &self.constraints
  }

  pub(super) fn try_new<O>(
    object: &O,
    field: &O::Field,
    flavor_ty: &Type,
    tag: u32,
  ) -> darling::Result<Self>
  where
    O: crate::ast::object::Object,
  {
    let path_to_grost = object.path_to_grost();
    let object_type = object.ty();
    let mut constraints = Punctuated::new();
    let field_ty = field.ty();
    constraints.push(syn::parse2(quote! {
      #path_to_grost::__private::reflection::TypeReflection<#field_ty>: #path_to_grost::__private::reflection::Reflectable<#field_ty, Reflection = #path_to_grost::__private::reflection::Type>
    })?);
    let field_reflection = field_reflection(path_to_grost, object_type, flavor_ty, tag)?;
    let wire_format_reflection =
      wire_format_reflection(path_to_grost, object_type, flavor_ty, tag)?;
    let wire_type_reflection =
      wire_type_reflection(path_to_grost, &field_reflection, flavor_ty, tag)?;
    let identifier_reflection =
      identifier_reflection(path_to_grost, &field_reflection, flavor_ty, tag)?;
    let encoded_identifier_reflection =
      encoded_identifier_reflection(path_to_grost, &field_reflection, flavor_ty, tag)?;
    let encoded_identifier_len_reflection =
      encoded_identifier_len_reflection(path_to_grost, &field_reflection, flavor_ty, tag)?;
    let tag_reflection = tag_reflection(path_to_grost, &field_reflection, flavor_ty, tag)?;
    let encoded_tag_reflection =
      encoded_tag_reflection(path_to_grost, &field_reflection, flavor_ty, tag)?;
    let encoded_tag_len_reflection =
      encoded_tag_len_reflection(path_to_grost, &field_reflection, flavor_ty, tag)?;

    let schema_name = field
      .schema()
      .name()
      .map(ToString::to_string)
      .unwrap_or_else(|| field.name().to_string());
    let schema_description = field
      .schema()
      .description()
      .map(ToString::to_string)
      .unwrap_or_default();

    Ok(Self {
      field_reflection,
      value: syn::parse2(quote! {
        #path_to_grost::__private::reflection::ObjectFieldBuilder {
          name: #schema_name,
          description: #schema_description,
          ty: <#path_to_grost::__private::reflection::TypeReflection<#field_ty> as #path_to_grost::__private::reflection::Reflectable<#field_ty>>::REFLECTION,
        }.build()
      })?,
      constraints,
      wire_format_reflection,
      wire_type_reflection,
      identifier_reflection,
      encoded_identifier_reflection,
      encoded_identifier_len_reflection,
      tag_reflection,
      encoded_tag_reflection,
      encoded_tag_len_reflection,
    })
  }
}

#[derive(Debug, Clone)]
pub struct FieldReflectionFlavor {
  wire_format_reflection: Type,
  wire_type_reflection: Type,
  identifier_reflection: Type,
  encoded_identifier_reflection: Type,
  encoded_identifier_len_reflection: Type,
  tag_reflection: Type,
  encoded_tag_reflection: Type,
  encoded_tag_len_reflection: Type,
}

#[derive(Debug, Clone)]
pub struct GenericFieldReflection {
  field_reflection: Type,
  value: Expr,
  constraints: Punctuated<WherePredicate, Comma>,
}

impl GenericFieldReflection {
  /// Returns the type of the field reflection.
  #[inline]
  pub const fn field_reflection(&self) -> &Type {
    &self.field_reflection
  }

  /// Returns the value of the field reflection.
  #[inline]
  pub const fn field_reflection_value(&self) -> &Expr {
    &self.value
  }

  /// Returns the constraints for this field reflection.
  #[inline]
  pub const fn constraints(&self) -> &Punctuated<WherePredicate, Comma> {
    &self.constraints
  }

  pub(super) fn try_new<O>(
    object: &O,
    field: &O::Field,
    flavor_param: &TypeParam,
    tag: u32,
  ) -> darling::Result<Self>
  where
    O: crate::ast::object::Object,
  {
    let path_to_grost = object.path_to_grost();
    let object_type = object.ty();
    let mut constraints = Punctuated::new();
    let field_ty = field.ty();
    constraints.push(syn::parse2(quote! {
      #path_to_grost::__private::reflection::TypeReflection<#field_ty>: #path_to_grost::__private::reflection::Reflectable<#field_ty, Reflection = #path_to_grost::__private::reflection::Type>
    })?);
    let schema_name = field
      .schema()
      .name()
      .map(ToString::to_string)
      .unwrap_or_else(|| field.name().to_string());
    let schema_description = field
      .schema()
      .description()
      .map(ToString::to_string)
      .unwrap_or_default();

    let flavor_ident = &flavor_param.ident;
    let field_reflection = field_reflection(path_to_grost, object_type, flavor_ident, tag)?;

    Ok(Self {
      field_reflection,
      value: syn::parse2(quote! {
        #path_to_grost::__private::reflection::ObjectFieldBuilder {
          name: #schema_name,
          description: #schema_description,
          ty: <#path_to_grost::__private::reflection::TypeReflection<#field_ty> as #path_to_grost::__private::reflection::Reflectable<#field_ty>>::REFLECTION,
        }.build()
      })?,
      constraints,
    })
  }
}

#[derive(Debug, Clone)]
pub struct FieldReflection {
  ty: Type,
  value: Expr,
  constraints: Punctuated<WherePredicate, Comma>,
}

impl FieldReflection {
  /// Returns the type of the field reflection.
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the value of the field reflection.
  #[inline]
  pub const fn value(&self) -> &Expr {
    &self.value
  }

  /// Returns the constraints for this field reflection.
  #[inline]
  pub const fn constraints(&self) -> &Punctuated<WherePredicate, Comma> {
    &self.constraints
  }

  pub(super) fn try_new<O>(
    object: &O,
    field: &O::Field,
    flavor_ty: impl ToTokens,
    tag: u32,
  ) -> darling::Result<Self>
  where
    O: crate::ast::object::Object,
  {
    let path_to_grost = object.path_to_grost();
    let object_name = object.name();
    let (_, tg, _) = object.generics().split_for_impl();
    let mut constraints = Punctuated::new();
    let field_ty = field.ty();
    constraints.push(syn::parse2(quote! {
      #path_to_grost::__private::reflection::TypeReflection<#field_ty>: #path_to_grost::__private::reflection::Reflectable<#field_ty, Reflection = #path_to_grost::__private::reflection::Type>
    })?);
    let ty = syn::parse2(quote! {
      #path_to_grost::__private::reflection::ObjectFieldReflection<
        #object_name #tg,
        #path_to_grost::__private::reflection::ObjectField,
        #flavor_ty,
        #tag,
      >
    })?;

    let schema_name = field
      .schema()
      .name()
      .map(ToString::to_string)
      .unwrap_or_else(|| field.name().to_string());
    let schema_description = field
      .schema()
      .description()
      .map(ToString::to_string)
      .unwrap_or_default();

    Ok(Self {
      ty,
      value: syn::parse2(quote! {
        #path_to_grost::__private::reflection::ObjectFieldBuilder {
          name: #schema_name,
          description: #schema_description,
          ty: <#path_to_grost::__private::reflection::TypeReflection<#field_ty> as #path_to_grost::__private::reflection::Reflectable<#field_ty>>::REFLECTION,
        }.build()
      })?,
      constraints,
    })
  }
}

pub enum Reflection {
  Generic(GenericReflection),
  Concrete(ConcreteReflection),
}

impl Reflection {
  /// Returns the generics of the reflection.
  #[inline]
  pub const fn generics(&self) -> &Generics {
    match self {
      Reflection::Generic(g) => g.generics(),
      Reflection::Concrete(c) => c.generics(),
    }
  }

  /// Returns the type of the reflection.
  #[inline]
  pub const fn ty(&self) -> &Type {
    match self {
      Reflection::Generic(g) => &g.ty,
      Reflection::Concrete(c) => &c.ty,
    }
  }

  pub(super) fn derive(&self, object: &Object) -> darling::Result<proc_macro2::TokenStream> {
    match self {
      Reflection::Generic(g) => g.derive(object),
      Reflection::Concrete(c) => c.derive(object),
    }
  }
}

pub struct ConcreteReflection {
  ty: Type,
  flavor_ty: Type,
  generics: Generics,
}

impl ConcreteReflection {
  /// Returns the type of the reflection.
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the flavor type of the reflection.
  #[inline]
  pub const fn flavor_type(&self) -> &Type {
    &self.flavor_ty
  }

  /// Returns the generics of the reflection.
  #[inline]
  pub const fn generics(&self) -> &Generics {
    &self.generics
  }

  pub(super) fn derive(&self, object: &Object) -> darling::Result<proc_macro2::TokenStream> {
    let path_to_grost = object.path_to_grost();
    let name = object.name();
    let object_reflectable = object.reflectable();
    let type_reflection = object.type_reflection();

    let object_reflection_type = self.ty();
    let (ig, tg, wc) = object.generics().split_for_impl();
    let (_, _, wc_reflection) = self.generics.split_for_impl();

    let name_str = name.to_string();
    let schema_name = object.schema.name().unwrap_or(name_str.as_str());
    let schema_description = object.schema.description().unwrap_or_default();

    let flavor_ty = &self.flavor_ty;

    let mut field_reflectable_impl = vec![];
    let mut field_reflections = vec![];
    let field_reflection_fns = object
      .fields()
      .iter()
      .filter_map(|f| match f {
        Field::Tagged(tagged_field) => Some(tagged_field),
        Field::Skipped(_) => None,
      })
      .map(|field| {
        field_reflections.push(field.field_reflection.value());
        field_reflectable_impl.push(field.derive_field_reflectable(object));

        field.derive_field_reflection_fn(object, flavor_ty, false)
      })
      .collect::<Vec<_>>();

    Ok(quote! {
      #(#field_reflectable_impl)*

      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl #ig #object_reflectable for #type_reflection #wc_reflection {
        type Reflection = #path_to_grost::__private::reflection::Type;

        const REFLECTION: &'static Self::Reflection = &{
          #path_to_grost::__private::reflection::Type::Object(
            &#path_to_grost::__private::reflection::ObjectBuilder {
              name: #schema_name,
              description: #schema_description,
              fields: &[
                #(&#field_reflections),*
              ],
            }.build()
          )
        };
      }

      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl #ig #name #tg #wc {
        #(#field_reflection_fns)*

        /// Returns the reflection of the object.
        #[inline]
        pub const fn reflection() -> #object_reflection_type
        where
          #flavor_ty: #path_to_grost::__private::flavors::Flavor,
        {
          #path_to_grost::__private::reflection::ObjectReflection::new()
        }
      }
    })
  }

  pub(super) fn try_new<O>(object: &O, fields: &[Field], flavor_ty: &Type) -> darling::Result<Self>
  where
    O: crate::ast::object::Object,
  {
    let path_to_grost = object.path_to_grost();
    let name = object.name();
    let generics = object.generics();
    let (_, tg, _) = generics.split_for_impl();
    let mut reflection_generics = generics.clone();

    let ty: Type = syn::parse2(quote! {
      #path_to_grost::__private::reflection::ObjectReflection<
        #name #tg,
        #path_to_grost::__private::reflection::Object,
        #flavor_ty,
      >
    })?;

    fields
      .iter()
      .filter_map(|f| {
        if let Field::Tagged(tagged_field) = f {
          Some(tagged_field)
        } else {
          None
        }
      })
      .for_each(|f| {
        reflection_generics
          .make_where_clause()
          .predicates
          .extend(f.field_reflection.constraints().iter().cloned());
      });

    Ok(Self {
      ty,
      generics: reflection_generics,
      flavor_ty: flavor_ty.clone(),
    })
  }
}

pub struct GenericReflection {
  ty: Type,
  flavor_param: TypeParam,
  generics: Generics,
}

impl GenericReflection {
  /// Returns the type of the reflection.
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the flavor type of the reflection.
  #[inline]
  pub const fn flavor_param(&self) -> &TypeParam {
    &self.flavor_param
  }

  /// Returns the generics of the reflection.
  #[inline]
  pub const fn generics(&self) -> &Generics {
    &self.generics
  }

  pub(super) fn derive(&self, object: &Object) -> darling::Result<proc_macro2::TokenStream> {
    let path_to_grost = object.path_to_grost();
    let name = object.name();
    let object_reflectable = object.reflectable();
    let type_reflection = object.type_reflection();

    let object_reflection_type = self.ty();
    let (ig, tg, wc) = object.generics().split_for_impl();
    let (_, _, wc_reflection) = self.generics.split_for_impl();

    let name_str = name.to_string();
    let schema_name = object.schema.name().unwrap_or(name_str.as_str());
    let schema_description = object.schema.description().unwrap_or_default();

    let flavor_ident = &self.flavor_param.ident;
    let mut field_reflectable_impl = vec![];
    let mut field_reflections = vec![];
    let field_reflection_fns = object
      .fields()
      .iter()
      .filter_map(|f| match f {
        Field::Tagged(tagged_field) => Some(tagged_field),
        Field::Skipped(_) => None,
      })
      .map(|field| {
        field_reflections.push(field.field_reflection.value());
        field_reflectable_impl.push(field.derive_field_reflectable(object));

        field.derive_field_reflection_fn(object, flavor_ident, true)
      })
      .collect::<Vec<_>>();

    Ok(quote! {
      #(#field_reflectable_impl)*

      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl #ig #object_reflectable for #type_reflection #wc_reflection {
        type Reflection = #path_to_grost::__private::reflection::Type;

        const REFLECTION: &'static Self::Reflection = &{
          #path_to_grost::__private::reflection::Type::Object(
            &#path_to_grost::__private::reflection::ObjectBuilder {
              name: #schema_name,
              description: #schema_description,
              fields: &[
                #(&#field_reflections),*
              ],
            }.build()
          )
        };
      }

      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl #ig #name #tg #wc {
        #(#field_reflection_fns)*

        /// Returns the reflection of the object.
        #[inline]
        pub const fn reflection<#flavor_ident>() -> #object_reflection_type
        where
          #flavor_ident: ?::core::marker::Sized + #path_to_grost::__private::flavors::Flavor,
        {
          #path_to_grost::__private::reflection::ObjectReflection::new()
        }
      }
    })
  }

  pub(super) fn try_new<O>(
    object: &O,
    fields: &[Field],
    flavor_param: &TypeParam,
  ) -> darling::Result<Self>
  where
    O: crate::ast::object::Object,
  {
    let path_to_grost = object.path_to_grost();
    let name = object.name();
    let generics = object.generics();
    let (_, tg, _) = generics.split_for_impl();
    let mut reflection_generics = generics.clone();
    reflection_generics
      .params
      .push(GenericParam::Type(flavor_param.clone()));

    let flavor_ident = &flavor_param.ident;
    let ty: Type = syn::parse2(quote! {
      #path_to_grost::__private::reflection::ObjectReflection<
        #name #tg,
        #path_to_grost::__private::reflection::Object,
        #flavor_ident,
      >
    })?;

    fields
      .iter()
      .filter_map(|f| {
        if let Field::Tagged(tagged_field) = f {
          Some(tagged_field)
        } else {
          None
        }
      })
      .for_each(|f| {
        reflection_generics
          .make_where_clause()
          .predicates
          .extend(f.field_reflection.constraints().iter().cloned());
      });

    Ok(Self {
      ty,
      generics: reflection_generics,
      flavor_param: flavor_param.clone(),
    })
  }
}

fn field_reflection(
  path_to_grost: &syn::Path,
  object_type: &Type,
  flavor_ty: impl ToTokens,
  tag: u32,
) -> syn::Result<syn::Type> {
  syn::parse2(quote! {
    #path_to_grost::__private::reflection::ObjectFieldReflection<
      #object_type,
      #path_to_grost::__private::reflection::ObjectField,
      #flavor_ty,
      #tag,
    >
  })
}

fn wire_format_reflection(
  path_to_grost: &syn::Path,
  object_type: &Type,
  flavor_ty: &Type,
  tag: u32,
) -> syn::Result<syn::Type> {
  syn::parse2(quote! {
    #path_to_grost::__private::reflection::WireFormatReflection<
      #object_type,
      #flavor_ty,
      #tag,
    >
  })
}

fn wire_type_reflection(
  path_to_grost: &syn::Path,
  object_type: &Type,
  flavor_ty: &Type,
  tag: u32,
) -> syn::Result<syn::Type> {
  syn::parse2(quote! {
    #path_to_grost::__private::reflection::WireTypeReflection<
      #path_to_grost::__private::reflection::ObjectFieldReflection<
        #object_type,
        <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::WireType,
        #flavor_ty,
        #tag,
      >
    >
  })
}

fn identifier_reflection(
  path_to_grost: &syn::Path,
  object_type: &Type,
  flavor_ty: &Type,
  tag: u32,
) -> syn::Result<syn::Type> {
  syn::parse2(quote! {
    #path_to_grost::__private::reflection::IdentifierReflection<
      #path_to_grost::__private::reflection::ObjectFieldReflection<
        #object_type,
        <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Identifier,
        #flavor_ty,
        #tag,
      >
    >
  })
}

fn encoded_identifier_reflection(
  path_to_grost: &syn::Path,
  object_type: &Type,
  flavor_ty: &Type,
  tag: u32,
) -> syn::Result<syn::Type> {
  syn::parse2(quote! {
    #path_to_grost::__private::reflection::EncodeReflection<
      #path_to_grost::__private::reflection::IdentifierReflection<
        #path_to_grost::__private::reflection::ObjectFieldReflection<
          #object_type,
          <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Identifier,
          #flavor_ty,
          #tag,
        >
      >
    >
  })
}

fn encoded_identifier_len_reflection(
  path_to_grost: &syn::Path,
  object_type: &Type,
  flavor_ty: &Type,
  tag: u32,
) -> syn::Result<syn::Type> {
  syn::parse2(quote! {
    #path_to_grost::__private::reflection::Len<
      #path_to_grost::__private::reflection::EncodeReflection<
        #path_to_grost::__private::reflection::IdentifierReflection<
          #path_to_grost::__private::reflection::ObjectFieldReflection<
            #object_type,
            <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Identifier,
            #flavor_ty,
            #tag,
          >
        >
      >
    >
  })
}

fn tag_reflection(
  path_to_grost: &syn::Path,
  object_type: &Type,
  flavor_ty: &Type,
  tag: u32,
) -> syn::Result<syn::Type> {
  syn::parse2(quote! {
    #path_to_grost::__private::reflection::TagReflection<
      #path_to_grost::__private::reflection::ObjectFieldReflection<
        #object_type,
        <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Tag,
        #flavor_ty,
        #tag,
      >
    >
  })
}

fn encoded_tag_reflection(
  path_to_grost: &syn::Path,
  object_type: &Type,
  flavor_ty: &Type,
  tag: u32,
) -> syn::Result<syn::Type> {
  syn::parse2(quote! {
    #path_to_grost::__private::reflection::EncodeReflection<
      #path_to_grost::__private::reflection::TagReflection<
        #path_to_grost::__private::reflection::ObjectFieldReflection<
          #object_type,
          <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Tag,
          #flavor_ty,
          #tag,
        >
      >
    >
  })
}

fn encoded_tag_len_reflection(
  path_to_grost: &syn::Path,
  object_type: &Type,
  flavor_ty: &Type,
  tag: u32,
) -> syn::Result<syn::Type> {
  syn::parse2(quote! {
    #path_to_grost::__private::reflection::Len<
      #path_to_grost::__private::reflection::EncodeReflection<
        #path_to_grost::__private::reflection::TagReflection<
          #path_to_grost::__private::reflection::ObjectFieldReflection<
            #object_type,
            <#flavor_ty as #path_to_grost::__private::flavors::Flavor>::Tag,
            #flavor_ty,
            #tag,
          >
        >
      >
    >
  })
}
