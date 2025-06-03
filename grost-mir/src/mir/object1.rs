use std::num::NonZeroU32;

use indexmap::{IndexMap, IndexSet};
use quote::{ToTokens, format_ident, quote};
use syn::{
  punctuated::Punctuated, token::Comma, Attribute, Expr, GenericParam, Generics, Ident, LifetimeParam, Path, Type, TypeParam, Visibility, WhereClause, WherePredicate
};

use crate::ast::{
  DecodeAttribute, EncodeAttribute, FlavorAttribute, IdentifierAttribute, MissingOperation,
  SchemaAttribute, TransformOperation,
  object::{
    Field as _, FieldAttribute, FieldDecodeAttribute, FieldEncodeAttribute, FieldFlavorAttribute,
    ObjectExt as _, PartialDecodedFieldAttribute, PartialFieldAttribute,
  },
};

use super::wire_format_reflection_ty;

#[derive(Debug, Clone)]
pub struct PartialDecodedFieldFlavor {
  wire_format: Type,
  wire_format_reflection: Type,
  state_type: Type,
  ty: Type,
  optional_type: Type,
  constraints: Punctuated<WherePredicate, Comma>,
}

impl PartialDecodedFieldFlavor {
  /// Returns the type constraints for this field.
  pub const fn type_constraints(&self) -> &Punctuated<WherePredicate, Comma> {
    &self.constraints
  }

  /// Returns the wire format type for this field.
  #[inline]
  pub const fn wire_format(&self) -> &Type {
    &self.wire_format
  }

  /// Returns the wire format reflection type for this field.
  #[inline]
  pub const fn wire_format_reflection(&self) -> &Type {
    &self.wire_format_reflection
  }

  pub const fn state_type(&self) -> &Type {
    &self.state_type
  }

  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  pub const fn optional_type(&self) -> &Type {
    &self.optional_type
  }

  fn try_new<O>(
    object: &O,
    object_type: &Type,
    object_reflectable: &Type,
    flavor_type: impl ToTokens,
    field: &O::Field,
    field_flavor: &FieldFlavorAttribute,
    tag: u32,
  ) -> darling::Result<Self>
  where
    O: crate::ast::object::Object,
  {
    let path_to_grost = object.path_to_grost();

    let partial_decoded = object.partial_decoded();
    let field_ty = field.ty();
    let lifetime = &partial_decoded.lifetime().lifetime.ident;
    let unknown_buffer = &partial_decoded.unknown_buffer().ident;

    let wf: Type = syn::parse2(quote! {
      #path_to_grost::__private::reflection::WireFormatReflection<
        #object_type,
        #flavor_type,
        #tag,
      >
    })?;

    let state_type: Type = syn::parse2(quote! {
      #path_to_grost::__private::convert::State<
        #path_to_grost::__private::convert::Decoded<
          #lifetime,
          #flavor_type,
          <#wf as #object_reflectable>::Reflection,
          #unknown_buffer,
        >
      >
    })?;

    let mut constraints = Punctuated::new();

    constraints.push(syn::parse2(quote! {
      #wf: #object_reflectable
    })?);
    constraints.push(syn::parse2(quote! {
      <#wf as #object_reflectable>::Reflection:
        #path_to_grost::__private::flavors::WireFormat<#flavor_type>
    })?);

    let ty = if let Some(ty) = field.partial_decoded().ty().cloned() {
      ty
    } else {
      let copy_constraint = if field.partial_decoded().copy() || partial_decoded.copy() {
        Some(quote! { + ::core::marker::Copy })
      } else {
        None
      };
      constraints.push(syn::parse2(quote! {
        <#field_ty as #state_type>::Output: ::core::marker::Sized #copy_constraint
      })?);
      constraints.push(syn::parse2(quote! {
        #field_ty: #state_type
      })?);
      syn::parse2(quote! { <#field_ty as #state_type>::Output })?
    };

    Ok(Self {
      wire_format_reflection: wf,
      wire_format: match field_flavor.format().cloned() {
        Some(format) => format,
        None => syn::parse2(quote! {
          <#field_ty as #path_to_grost::__private::flavors::DefaultWireFormat<#flavor_type>>::Format
        })?,
      },
      state_type,
      optional_type: syn::parse2(quote! {
        ::core::option::Option<#ty>
      })?,
      ty,
      constraints,
    })
  }
}

#[derive(Debug, Clone)]
pub struct FieldFlavor {
  partial_decoded: PartialDecodedFieldFlavor,
  encode: FieldEncodeAttribute,
  decode: FieldDecodeAttribute,
}

impl FieldFlavor {
  fn try_new<O>(
    object: &O,
    object_type: &Type,
    object_reflectable: &Type,
    field: &O::Field,
    flavor: &Flavor,
    field_flavor_attr: &FieldFlavorAttribute,
  ) -> darling::Result<Self>
  where
    O: crate::ast::object::Object,
  {
    let tag = field.tag().expect("Field must have a tag.").get();
    Ok(Self {
      partial_decoded: PartialDecodedFieldFlavor::try_new(
        object,
        object_type,
        object_reflectable,
        flavor.ty(),
        field,
        field_flavor_attr,
        tag,
      )?,
      encode: field_flavor_attr.encode().clone(),
      decode: field_flavor_attr.decode().clone(),
    })
  }
}

#[derive(Debug, Clone)]
pub struct PartialDecodedField {
  ty: Type,
  optional_type: Type,
  state_type: Type,
  wire_format_reflection: Type,
  constraints: Punctuated<WherePredicate, Comma>,
  attrs: Vec<Attribute>,
  copy: bool,
}

impl PartialDecodedField {
  /// Returns the wire format reflection type for this field.
  #[inline]
  pub const fn wire_format_reflection(&self) -> &Type {
    &self.wire_format_reflection
  }

  /// Returns the constraints for this field.
  #[inline]
  pub const fn type_constraints(&self) -> &Punctuated<WherePredicate, Comma> {
    &self.constraints
  }

  fn try_new<O>(
    object: &O,
    object_type: &Type,
    object_reflectable: &Type,
    field: &O::Field,
    flavor_ty: impl ToTokens,
  ) -> darling::Result<Self>
  where
    O: crate::ast::object::Object,
  {
    let path_to_grost = object.path_to_grost();
    let ty = field.ty();
    let tag = field.tag().expect("Field must have a tag.").get();
    let attr = field.partial_decoded();
    let partial_decoded = object.partial_decoded();
    let lifetime = &partial_decoded.lifetime().lifetime;
    let unknown_buffer = &partial_decoded.unknown_buffer().ident;

    let mut constraints = Punctuated::new();
    let wf: Type = syn::parse2(quote! {
      #path_to_grost::__private::reflection::WireFormatReflection<
        #object_type,
        #flavor_ty,
        #tag,
      >
    })?;

    let state_type: Type = syn::parse2(quote! {
      #path_to_grost::__private::convert::State<
        #path_to_grost::__private::convert::Decoded<
          #lifetime,
          #flavor_ty,
          <#wf as #object_reflectable>::Reflection,
          #unknown_buffer,
        >
      >
    })?;

    constraints.push(syn::parse2(quote! {
      #wf: #object_reflectable
    })?);
    constraints.push(syn::parse2(quote! {
      <#wf as #object_reflectable>::Reflection:
        #path_to_grost::__private::flavors::WireFormat<#flavor_ty>
    })?);

    let ty = if let Some(ty) = attr.ty().cloned() {
      ty
    } else {
      let copy_constraint = if field.partial_decoded().copy() || partial_decoded.copy() {
        Some(quote! { + ::core::marker::Copy })
      } else {
        None
      };

      constraints.push(syn::parse2(quote! {
        <#ty as #state_type>::Output: ::core::marker::Sized #copy_constraint
      })?);
      constraints.push(syn::parse2(quote! {
        #ty: #state_type
      })?);
      syn::parse2(quote! { <#ty as #state_type>::Output })?
    };

    Ok(Self {
      optional_type: syn::parse2(quote! {
        ::core::option::Option<#ty>
      })?,
      ty,
      state_type,
      wire_format_reflection: wf,
      constraints,
      attrs: field.partial_decoded().attrs().to_vec(),
      copy: field.partial_decoded().copy(),
    })
  }
}

#[derive(Debug, Clone)]
pub struct PartialField {
  ty: Type,
  optional_type: Type,
  attrs: Vec<Attribute>,
}

impl PartialField {
  fn try_new(
    path_to_grost: &Path,
    ty: &Type,
    attr: &PartialFieldAttribute,
  ) -> darling::Result<Self> {
    let ty: Type = match attr.ty().cloned() {
      Some(ty) => ty,
      None => syn::parse2(quote! {
        <#ty as #path_to_grost::__private::convert::State<
          #path_to_grost::__private::convert::Flatten
        >>::Output
      })?,
    };

    Ok(Self {
      optional_type: syn::parse2(quote! {
        ::core::option::Option<#ty>
      })?,
      ty,
      attrs: attr.attrs().to_vec(),
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

  fn try_new<O>(
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
      value: syn::parse2(
        quote! {
          #path_to_grost::__private::reflection::ObjectFieldBuilder {
            name: #schema_name,
            description: #schema_description,
            ty: <#path_to_grost::__private::reflection::TypeReflection<#field_ty> as #path_to_grost::__private::reflection::Reflectable<#field_ty>>::REFLECTION,
          }.build()
        }
      )?,
      constraints,
    })
  }
}

#[derive(Debug, Clone)]
pub struct TaggedField {
  attrs: Vec<Attribute>,
  vis: Visibility,
  name: Ident,
  ty: Type,
  tag: u32,
  schema_name: String,
  schema_description: String,
  partial: PartialField,
  partial_decoded: PartialDecodedField,
  field_reflection: FieldReflection,
  missing_operation: Option<MissingOperation>,
  transform_operation: Option<TransformOperation>,
  default: syn::Path,
  flavors: IndexMap<Ident, FieldFlavor>,
  copy: bool,
}

impl TaggedField {
  #[inline]
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  #[inline]
  pub const fn schema_name(&self) -> &str {
    self.schema_name.as_str()
  }

  #[inline]
  pub const fn schema_description(&self) -> &str {
    self.schema_description.as_str()
  }

  #[inline]
  pub const fn tag(&self) -> u32 {
    self.tag
  }

  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the fn which returns a default value for this field
  #[inline]
  pub const fn default(&self) -> &syn::Path {
    &self.default
  }

  /// Returns `true` if this field is copyable, `false` otherwise.
  #[inline]
  pub const fn copy(&self) -> bool {
    self.copy
  }

  fn try_new<O>(
    object: &O,
    object_type: &Type,
    object_reflectable: &Type,
    flavors: &IndexMap<Ident, Flavor>,
    flavor_type: impl ToTokens,
    field: &O::Field,
    tag: u32,
  ) -> darling::Result<Self>
  where
    O: crate::ast::object::Object,
  {
    let field_ty = field.ty();
    let default = match field.default().cloned() {
      Some(path) => path,
      None => syn::parse2(quote! {
        <#field_ty as ::core::default::Default>::default
      })?,
    };

    let missing_operation = field.convert().missing_operation().cloned();
    let transform_operation = field.convert().transform_operation().cloned();
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
    let field_reflection = FieldReflection::try_new(object, field, &flavor_type, tag)?;

    Ok(Self {
      attrs: field.attrs().to_vec(),
      vis: field.vis().clone(),
      name: field.name().clone(),
      ty: field_ty.clone(),
      tag,
      schema_name,
      schema_description,
      partial: PartialField::try_new(object.path_to_grost(), field.ty(), field.partial())?,
      partial_decoded: PartialDecodedField::try_new(
        object,
        object_type,
        object_reflectable,
        field,
        &flavor_type,
      )?,
      field_reflection,
      missing_operation,
      transform_operation,
      default,
      flavors: field
        .flavors()
        .iter()
        .map(|attr| {
          let ident = attr.name().clone();
          let flavor = flavors.get(&ident).ok_or_else(|| {
            darling::Error::custom(format!(
              "Flavor `{}` is not registered for field `{}`.",
              ident,
              field.name()
            ))
          })?;
          FieldFlavor::try_new(object, object_type, object_reflectable, field, flavor, attr)
            .map(|flavor| (ident, flavor))
        })
        .collect::<darling::Result<_>>()?,
      copy: object.copy() || field.copy(),
    })
  }

  fn field_reflectable(
    &self,
    object: &Object,
  ) -> proc_macro2::TokenStream {
    let path_to_grost = object.path_to_grost();
    let object_type = object.ty();
    let field = self;
    let field_ty = &field.ty;
    let reflection_ty = self.field_reflection.ty();
    let schema_name = field.schema_name();
    let schema_description = field.schema_description();
    let (ig_reflection, _, wc_reflection) = object.reflection.generics().split_for_impl();

    quote! {
      #[automatically_derived]
      #[allow(clippy::type_complexity, non_camel_case_types)]
      impl #ig_reflection #path_to_grost::__private::reflection::Reflectable<#object_type> for #reflection_ty #wc_reflection {
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

  fn field_reflection_fn(
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
    let reflection_ty = self.field_reflection.ty();
    let field_reflection_name = format_ident!("{}_reflection", field_name);

    if !generic {
      quote! {
        #[doc = #doc]
        #[inline]
        pub const fn #field_reflection_name() -> #reflection_ty
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
        pub const fn #field_reflection_name<#flavor_ty>() -> #reflection_ty
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
pub struct SkippedField {
  attrs: Vec<Attribute>,
  vis: Visibility,
  name: Ident,
  ty: Type,
  default: syn::Path,
  copy: bool,
}

impl SkippedField {
  #[inline]
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the fn which returns a default value for this field
  #[inline]
  pub const fn default(&self) -> &syn::Path {
    &self.default
  }

  /// Returns `true` if this field is copyable, `false` otherwise.
  #[inline]
  pub const fn copy(&self) -> bool {
    self.copy
  }

  fn try_new<O>(object: &O, f: &O::Field) -> darling::Result<Self>
  where
    O: crate::ast::object::Object,
  {
    let ty = f.ty();

    Ok(SkippedField {
      attrs: f.attrs().to_vec(),
      vis: f.vis().clone(),
      name: f.name().clone(),
      ty: ty.clone(),
      default: match f.default().cloned() {
        Some(path) => path,
        None => syn::parse2(quote! {
          <#ty as ::core::default::Default>::default
        })?,
      },
      copy: f.copy() || object.copy(),
    })
  }
}

#[derive(Debug, Clone, derive_more::IsVariant, derive_more::Unwrap, derive_more::TryUnwrap)]
#[unwrap(ref)]
#[try_unwrap(ref)]
pub enum Field {
  Tagged(TaggedField),
  Skipped(SkippedField),
}

impl Field {
  #[inline]
  pub const fn ty(&self) -> &Type {
    match self {
      Field::Tagged(f) => &f.ty,
      Field::Skipped(f) => &f.ty,
    }
  }

  /// Returns the fn which returns a default value for this field, if `None`,
  /// [`core::default::Default::default()`] will be used.
  #[inline]
  pub const fn default(&self) -> &syn::Path {
    match self {
      Field::Tagged(f) => f.default(),
      Field::Skipped(f) => f.default(),
    }
  }

  /// Returns the name of the field.
  #[inline]
  pub const fn name(&self) -> &Ident {
    match self {
      Field::Tagged(f) => &f.name,
      Field::Skipped(f) => &f.name,
    }
  }

  /// Returns `true` if this field is copyable, `false` otherwise.
  #[inline]
  pub const fn copy(&self) -> bool {
    match self {
      Field::Tagged(f) => f.copy,
      Field::Skipped(f) => f.copy,
    }
  }
}

#[derive(Debug, Clone)]
pub struct ConcretePartialDecodedObject {
  name: Ident,
  generics: Generics,
  attrs: Vec<Attribute>,
  ty: Type,
  flavor_type: Type,
  unknown_buffer_param: TypeParam,
  lifetime_param: LifetimeParam,
}

impl ConcretePartialDecodedObject {
  fn try_new<O>(
    object: &O,
    fields: &[Field],
    flavor_name: &Ident,
    flavor_type: Type,
  ) -> darling::Result<Self>
  where
    O: crate::ast::object::Object,
  {
    let partial_decoded = object.partial_decoded();
    let partial_decoded_object_name = object.partial_decoded_name();
    let lifetime_param = partial_decoded.lifetime();
    let unknown_buffer_param = partial_decoded.unknown_buffer();

    let generics = object.generics();
    let mut partial_decoded_generics = generics.clone();

    partial_decoded_generics
      .params
      .push(GenericParam::Lifetime(lifetime_param.clone()));
    partial_decoded_generics
      .params
      .push(GenericParam::Type(unknown_buffer_param.clone()));

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
        let field_flavor = f.flavors.get(flavor_name).unwrap();

        field_flavor
          .partial_decoded
          .type_constraints()
          .iter()
          .for_each(|constraint| {
            partial_decoded_generics
              .make_where_clause()
              .predicates
              .push(constraint.clone());
          });
      });
    let (_, tg, _) = partial_decoded_generics.split_for_impl();

    Ok(Self {
      ty: syn::parse2(quote! {
        #partial_decoded_object_name #tg
      })?,
      name: partial_decoded_object_name,
      attrs: partial_decoded.attrs().to_vec(),
      generics: partial_decoded_generics,
      flavor_type,
      unknown_buffer_param: unknown_buffer_param.clone(),
      lifetime_param: lifetime_param.clone(),
    })
  }
}

#[derive(Debug, Clone)]
pub struct GenericPartialDecodedObjectFlavor {}

impl GenericPartialDecodedObjectFlavor {
  fn try_new<O>(object: &O, fields: &[Field], flavor: &FlavorAttribute) -> darling::Result<Self>
  where
    O: crate::ast::object::Object,
  {
    todo!()
  }
}

#[derive(Debug, Clone)]
pub struct GenericPartialDecodedObject {
  name: Ident,
  ty: Type,
  attrs: Vec<Attribute>,
  vis: Visibility,
  generics: Generics,
  flavor_param: Option<TypeParam>,
  unknown_buffer_param: TypeParam,
  lifetime_param: LifetimeParam,
  flavors: IndexMap<Ident, GenericPartialDecodedObjectFlavor>,
}

impl GenericPartialDecodedObject {
  fn try_new<O>(object: &O, flavor_param: &TypeParam, fields: &[Field]) -> darling::Result<Self>
  where
    O: crate::ast::object::Object,
  {
    let name = object.name();
    let partial_decoded = object.partial_decoded();
    let partial_decoded_name = partial_decoded
      .name()
      .cloned()
      .unwrap_or_else(|| format_ident!("PartialDecoded{}", name));
    let lifetime_param = partial_decoded.lifetime();
    let unknown_buffer_param = partial_decoded.unknown_buffer();

    let generics = object.generics();
    let mut partial_decoded_generics = generics.clone();

    partial_decoded_generics
      .params
      .push(GenericParam::Lifetime(lifetime_param.clone()));
    partial_decoded_generics
      .params
      .push(GenericParam::Type(unknown_buffer_param.clone()));
    partial_decoded_generics
      .params
      .push(GenericParam::Type(flavor_param.clone()));

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
        f.partial_decoded
          .type_constraints()
          .iter()
          .for_each(|constraint| {
            partial_decoded_generics
              .make_where_clause()
              .predicates
              .push(constraint.clone());
          });
      });
    let (_, tg, _) = partial_decoded_generics.split_for_impl();

    Ok(Self {
      ty: syn::parse2(quote! {
        #partial_decoded_name #tg
      })?,
      name: partial_decoded_name,
      attrs: partial_decoded.attrs().to_vec(),
      vis: object.vis().clone(),
      generics: partial_decoded_generics,
      flavor_param: partial_decoded.flavor().cloned(),
      unknown_buffer_param: partial_decoded.unknown_buffer().clone(),
      lifetime_param: partial_decoded.lifetime().clone(),
      flavors: object
        .flavors()
        .iter()
        .map(|flavor| {
          let ident = flavor.name().clone();
          GenericPartialDecodedObjectFlavor::try_new(object, fields, flavor)
            .map(|flavor| (ident, flavor))
        })
        .collect::<darling::Result<IndexMap<_, _>>>()?,
    })
  }
}


pub enum PartialDecodedObject {
  Generic(GenericPartialDecodedObject),
  Concrete(ConcretePartialDecodedObject),
}

impl PartialDecodedObject {
  #[inline]
  pub const fn name(&self) -> &Ident {
    match self {
      Self::Generic(g) => &g.name,
      Self::Concrete(c) => &c.name,
    }
  }

  #[inline]
  pub const fn ty(&self) -> &Type {
    match self {
      Self::Generic(g) => &g.ty,
      Self::Concrete(c) => &c.ty,
    }
  }

  #[inline]
  pub const fn generics(&self) -> &Generics {
    match self {
      Self::Generic(g) => &g.generics,
      Self::Concrete(c) => &c.generics,
    }
  }
}

pub struct Flavor {
  ty: Type,
  wire_format: Type,
  identifier: IdentifierAttribute,
  encode: EncodeAttribute,
  decode: DecodeAttribute,
}

impl Flavor {
  /// Returns the type of the flavor
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
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

  fn derive(&self, object: &Object) -> darling::Result<proc_macro2::TokenStream> {
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

  fn derive(&self, object: &Object) -> darling::Result<proc_macro2::TokenStream> {
    let path_to_grost = object.path_to_grost();
    let name = object.name();
    let object_type = object.ty();
    let reflectable = object.reflectable();
    let type_reflection = object.type_reflection();

    let object_reflection_ty = &self.ty;
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
        field_reflectable_impl.push(field.field_reflectable(object));

        field.field_reflection_fn(object, flavor_ty, false)
      })
      .collect::<Vec<_>>();
    

    Ok(quote! {
      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl #ig #reflectable for #type_reflection #wc_reflection {
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
        pub const fn reflection() -> #object_reflection_ty
        where
          #flavor_ty: #path_to_grost::__private::flavors::Flavor,
        {
          #path_to_grost::__private::reflection::ObjectReflection::new()
        }
      }
    })
  }

  fn try_new<O>(object: &O, fields: &[Field], flavor_ty: &Type) -> darling::Result<Self>
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

    fields.iter().filter_map(|f| {
      if let Field::Tagged(tagged_field) = f {
        Some(tagged_field)
      } else {
        None
      }
    }).for_each(|f| {
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

  fn derive(&self, object: &Object) -> darling::Result<proc_macro2::TokenStream> {
    let path_to_grost = object.path_to_grost();
    let name = object.name();
    let reflectable = object.reflectable();
    let type_reflection = object.type_reflection();

    let object_reflection_ty = &self.ty;
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
        field_reflectable_impl.push(field.field_reflectable(object));

        field.field_reflection_fn(object, flavor_ident, true)
      })
      .collect::<Vec<_>>();

    Ok(quote! {
      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl #ig #reflectable for #type_reflection #wc_reflection {
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
        pub const fn reflection<#flavor_ident>() -> #object_reflection_ty
        where
          #flavor_ident: ?::core::marker::Sized + #path_to_grost::__private::flavors::Flavor,
        {
          #path_to_grost::__private::reflection::ObjectReflection::new()
        }
      }
    })
  }

  fn try_new<O>(
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
    reflection_generics.params.push(GenericParam::Type(flavor_param.clone()));

    let flavor_ident = &flavor_param.ident;
    let ty: Type = syn::parse2(quote! {
      #path_to_grost::__private::reflection::ObjectReflection<
        #name #tg,
        #path_to_grost::__private::reflection::Object,
        #flavor_ident,
      >
    })?;

    fields.iter().filter_map(|f| {
      if let Field::Tagged(tagged_field) = f {
        Some(tagged_field)
      } else {
        None
      }
    }).for_each(|f| {
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

pub struct Object {
  attrs: Vec<Attribute>,
  vis: Visibility,
  name: Ident,
  ty: Type,
  type_reflection: Type,
  path_to_grost: Path,
  schema: SchemaAttribute,
  flavor_param: Option<TypeParam>,
  generics: syn::Generics,
  partial_decoded: PartialDecodedObject,
  flavors: IndexMap<Ident, Flavor>,
  default: Option<syn::Path>,
  fields: Vec<Field>,
  reflection: Reflection,
  reflectable: Type,
}

impl ToTokens for Object {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let name = &self.name;
    let vis = &self.vis;
    let generics = &self.generics;
    let where_clause = &self.generics.where_clause;
    let attrs = &self.attrs;

    let fields = self.fields.iter().map(|f| match f {
      Field::Tagged(tagged_field) => {
        let attrs = &tagged_field.attrs;
        let vis = &tagged_field.vis;
        let name = &tagged_field.name;
        let ty = &tagged_field.ty;
        quote! {
          #(#attrs)*
          #vis #name: #ty
        }
      }
      Field::Skipped(skipped_field) => {
        let attrs = &skipped_field.attrs;
        let vis = &skipped_field.vis;
        let name = &skipped_field.name;
        let ty = &skipped_field.ty;
        quote! {
          #(#attrs)*
          #vis #name: #ty
        }
      }
    });

    tokens.extend(quote! {
      #(#attrs)*
      #vis struct #name #generics #where_clause {
        #(#fields),*
      }
    });
  }
}

impl Object {
  pub fn name(&self) -> &Ident {
    &self.name
  }

  /// Returns the object as a type.
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the `TypeReflection` type of the object.
  #[inline]
  pub const fn type_reflection(&self) -> &Type {
    &self.type_reflection
  }

  /// Returns the reflectable trait which applies the object type as the generic of the `Reflectable` trait.
  #[inline]
  pub const fn reflectable(&self) -> &Type {
    &self.reflectable
  }

  pub fn generics(&self) -> &syn::Generics {
    &self.generics
  }

  pub fn fields(&self) -> &[Field] {
    &self.fields
  }

  /// Returns the path to the `grost` crate.
  #[inline]
  pub const fn path_to_grost(&self) -> &Path {
    &self.path_to_grost
  }

  pub fn attrs(&self) -> &[Attribute] {
    &self.attrs
  }
}

impl Object {
  pub fn from_derive_input<O>(input: &O) -> darling::Result<Self>
  where
    O: crate::ast::object::Object,
  {
    let path_to_grost = input.path_to_grost();
    let name = input.name().clone();
    let vis = input.vis().clone();
    let generics = input.generics().clone();
    let flavor_param = input.partial_decoded().flavor().cloned();
    let schema = input.schema().clone();
    let default = input.default().cloned();
    let (ig, tg, wc) = generics.split_for_impl();

    let object_type: Type = syn::parse2(quote! {
      #name #tg
    })?;
    let reflectable: Type = syn::parse2(quote! {
      #path_to_grost::__private::reflection::Reflectable<#object_type>
    })?;
    let type_reflection: Type = syn::parse2(quote! {
      #path_to_grost::__private::reflection::TypeReflection<#object_type>
    })?;

    let flavors = input
      .flavors()
      .iter()
      .map(|flavor| {
        let name = flavor.name().clone();
        let ty = flavor.ty().clone();
        let wire_format = flavor.wire_format().clone();
        let identifier = flavor.identifier().clone();
        let encode = flavor.encode().clone();
        let decode = flavor.decode().clone();

        Ok((
          name,
          Flavor {
            ty,
            wire_format,
            identifier,
            encode,
            decode,
          },
        ))
      })
      .collect::<syn::Result<IndexMap<Ident, Flavor>>>()?;

    let mut tags_cache = IndexSet::new();
    let mut fields = |flavor_type: proc_macro2::TokenStream| {
      input
        .fields()
        .iter()
        .map(|f| {
          if f.skip() {
            SkippedField::try_new(input, f).map(Field::Skipped)
          } else if let Some(tag) = f.tag() {
            let tag = tag.get();
            if !tags_cache.insert(tag) {
              return Err(darling::Error::custom(format!(
                "Multiple fields with the same tag `{tag}` in object `{}`.",
                input.name()
              )));
            }

            TaggedField::try_new(input, &object_type, &reflectable, &flavors, &flavor_type, f, tag).map(Field::Tagged)
          } else {
            return Err(darling::Error::missing_field("tag"));
          }
        })
        .collect::<darling::Result<Vec<Field>>>()
    };

    let (partial_decoded, reflection, fields) = if let Some(param) = &flavor_param {
      let fields = {
        let ident = &param.ident;
        fields(quote! { #ident })?
      };
      let decoded_object = GenericPartialDecodedObject::try_new(input, param, &fields)?;

      (
        PartialDecodedObject::Generic(decoded_object),
        Reflection::Generic(GenericReflection::try_new(input, &fields, param)?),
        fields,
      )
    } else {
      let (flavor_name, flavor) = flavors
        .first()
        .expect("There should be only one flavor registered when not derving generic code.");

      let fields = {
        let ty = flavor.ty();
        fields(quote! { #ty })?
      };
      let decoded_object =
        ConcretePartialDecodedObject::try_new(input, &fields, flavor_name, flavor.ty().clone())?;
      let reflection = ConcreteReflection::try_new(input, &fields, flavor.ty())?;

      (PartialDecodedObject::Concrete(decoded_object), Reflection::Concrete(reflection), fields)
    };

    Ok(Self {
      attrs: input.attrs().to_vec(),
      vis,
      name,
      reflectable,
      type_reflection,
      ty: object_type,
      path_to_grost: path_to_grost.clone(),
      schema,
      partial_decoded,
      flavor_param,
      generics,
      flavors,
      default,
      fields,
      reflection,
    })
  }

  pub fn derive(&self) -> syn::Result<proc_macro2::TokenStream> {
    let path_to_grost = self.path_to_grost();
    let flatten_state = derive_flatten_state(path_to_grost, self.generics(), self.name());
    let accessors = self.derive_accessors();
    let default = self.derive_default();
    let reflection = self.derive_reflection()?;

    Ok(quote! {
      const _: () = {
        #flatten_state

        #accessors

        #default

        #reflection
      };
    })
  }

  fn derive_default(&self) -> proc_macro2::TokenStream {
    let name = self.name();

    if let Some(ref default) = self.default {
      let (ig, tg, w) = self.generics.split_for_impl();
      return quote! {
        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl #ig ::core::default::Default for #name #tg #w {
          #[inline]
          fn default() -> Self {
            Self::new()
          }
        }

        #[automatically_derived]
        #[allow(non_camel_case_types, clippy::type_complexity)]
        impl #ig #name #tg #w {
          /// Creates a new instance with default values.
          pub fn new() -> Self {
            #default()
          }
        }
      };
    }

    let fields = self.fields.iter().map(|f| {
      let field_name = f.name();
      let default = f.default();
      quote! {
        #field_name: #default()
      }
    });

    let (ig, tg, w) = self.generics.split_for_impl();

    quote! {
      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl #ig ::core::default::Default for #name #tg #w {
        fn default() -> Self {
          Self::new()
        }
      }

      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl #ig #name #tg #w {
        /// Creates a new instance with default values.
        #[inline]
        pub fn new() -> Self {
          Self {
            #(#fields),*
          }
        }
      }
    }
  }

  fn derive_accessors(&self) -> proc_macro2::TokenStream {
    let fns = self
      .fields
      .iter()
      .map(|f| accessors(f.name(), f.ty(), f.copy()));
    let (ig, tg, w) = self.generics().split_for_impl();
    let name = self.name();

    quote! {
      #[automatically_derived]
      #[allow(non_camel_case_types, clippy::type_complexity)]
      impl #ig #name #tg #w {
        #(#fns)*
      }
    }
  }

  fn derive_reflection(&self) -> darling::Result<proc_macro2::TokenStream> {
    self.reflection.derive(self)
  }
}

fn derive_flatten_state(
  path_to_grost: &Path,
  generics: &Generics,
  name: &Ident,
) -> proc_macro2::TokenStream {
  let mut all_generics = generics.clone();
  all_generics.params.push(
    syn::parse2(quote! {
      __GROST_FLATTEN_STATE__: ?::core::marker::Sized
    })
    .unwrap(),
  );

  let (ig, _, w) = all_generics.split_for_impl();
  let (_, tg, _) = generics.split_for_impl();

  quote! {
    #[automatically_derived]
    #[allow(non_camel_case_types, clippy::type_complexity)]
    impl #ig #path_to_grost::__private::convert::State<#path_to_grost::__private::convert::Flatten<__GROST_FLATTEN_STATE__>> for #name #tg #w {
      type Input = Self;
      type Output = Self;
    }
  }
}

fn accessors(field_name: &Ident, ty: &Type, copy: bool) -> proc_macro2::TokenStream {
  let ref_fn = format_ident!("{}_ref", field_name);
  let ref_fn_doc = format!(" Returns a reference to the `{field_name}`");
  let ref_mut_fn = format_ident!("{}_mut", field_name);
  let ref_mut_fn_doc = format!(" Returns a mutable reference to the `{field_name}`");
  let set_fn = format_ident!("set_{}", field_name);
  let set_fn_doc = format!(" Set the `{field_name}` to the given value");
  let with_fn = format_ident!("with_{}", field_name);
  let constable = copy.then(|| quote! { const });

  quote! {
    #[doc = #ref_fn_doc]
    #[inline]
    pub const fn #ref_fn(&self) -> &#ty {
      &self.#field_name
    }

    #[doc = #ref_mut_fn_doc]
    #[inline]
    pub const fn #ref_mut_fn(&mut self) -> &mut #ty {
      &mut self.#field_name
    }

    #[doc = #set_fn_doc]
    #[inline]
    pub #constable fn #set_fn(&mut self, value: #ty) -> &mut Self {
      self.#field_name = value;
      self
    }

    #[doc = #set_fn_doc]
    #[inline]
    pub #constable fn #with_fn(mut self, value: #ty) -> Self {
      self.#field_name = value;
      self
    }
  }
}
