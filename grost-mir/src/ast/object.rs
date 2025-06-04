use std::default;

use darling::FromMeta;
use indexmap::{IndexMap, IndexSet};
use quote::{format_ident, quote};
use syn::{Attribute, Generics, Ident, Path, Type, TypeParam, Visibility};

use crate::ast::MissingOperation;

use super::{
  Attributes, FlavorAttribute, FlavorFromMeta, GenericAttribute, SchemaAttribute, SchemaFromMeta,
  grost_flavor_param,
};

pub use field::*;
pub use partial_decoded::*;
pub use partial::*;
pub use selector::*;

mod field;
mod partial;
mod partial_decoded;
mod selector;

#[derive(Debug, Default, Clone, FromMeta)]
struct IndexerFromMeta {
  #[darling(default, rename = "rename")]
  name: Option<Ident>,
  #[darling(default, map = "Attributes::into_inner")]
  attrs: Vec<Attribute>,
}

impl From<IndexerFromMeta> for IndexerAttribute {
  fn from(meta: IndexerFromMeta) -> Self {
    Self {
      name: meta.name,
      attrs: meta.attrs,
    }
  }
}

#[derive(Debug, Default, Clone)]
pub struct IndexerAttribute {
  name: Option<Ident>,
  attrs: Vec<Attribute>,
}

impl IndexerAttribute {
  /// Returns the name of the indexer
  pub(crate) const fn name(&self) -> Option<&Ident> {
    self.name.as_ref()
  }

  /// Returns the attributes of the indexer
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }
}

#[derive(Debug, Default, Clone, FromMeta)]
pub struct ObjectFromMeta {
  #[darling(default)]
  default: Option<syn::Path>,
  #[darling(default)]
  generic: GenericAttribute,
  #[darling(default)]
  schema: SchemaFromMeta,
  #[darling(default)]
  partial: PartialObjectFromMeta,
  #[darling(default)]
  partial_decoded: PartialDecodedObjectFromMeta,
  #[darling(default)]
  selector: SelectorFromMeta,
  #[darling(default)]
  selector_iter: SelectorIterFromMeta,
  #[darling(default)]
  indexer: IndexerFromMeta,
  #[darling(default)]
  flavor: FlavorFromMeta,
  #[darling(default)]
  copy: bool,
}

impl ObjectFromMeta {
  pub fn finalize(self, path_to_grost: syn::Path) -> syn::Result<ObjectAttribute> {
    let flavors = self.flavor.into_object_flavors(&path_to_grost)?;
    let mut flavor_generic = self.generic.flavor().cloned();
    if flavors.len() > 1 {
      flavor_generic.get_or_insert_with(grost_flavor_param);
    }

    Ok(ObjectAttribute {
      path_to_grost,
      flavors,
      default: self.default,
      schema: self.schema.into(),
      partial: self.partial.finalize(self.generic.unknown_buffer().clone()),
      partial_decoded: self.partial_decoded.finalize(
        flavor_generic.clone(),
        self.generic.unknown_buffer().clone(),
        self.generic.lifetime().clone(),
      ),
      selector: self
        .selector
        .finalize(flavor_generic.clone(), self.generic.wire_format().clone()),
      selector_iter: self.selector_iter.finalize(flavor_generic),
      indexer: self.indexer.into(),
      copy: self.copy,
    })
  }
}

#[derive(Debug, Clone)]
pub struct ObjectAttribute {
  path_to_grost: syn::Path,
  flavors: Vec<FlavorAttribute>,
  default: Option<syn::Path>,
  schema: SchemaAttribute,
  partial: PartialObjectAttribute,
  partial_decoded: PartialDecodedObjectAttribute,
  selector: SelectorAttribute,
  selector_iter: SelectorIterAttribute,
  indexer: IndexerAttribute,
  copy: bool,
}

impl ObjectAttribute {
  /// Returns the path to the `grost` crate
  pub const fn path_to_grost(&self) -> &syn::Path {
    &self.path_to_grost
  }

  /// Returns the path to the fn that returns the default value of the object
  pub const fn default(&self) -> Option<&syn::Path> {
    self.default.as_ref()
  }

  /// Returns the schema information
  pub const fn schema(&self) -> &SchemaAttribute {
    &self.schema
  }

  /// Returns the partial object information
  pub const fn partial(&self) -> &PartialObjectAttribute {
    &self.partial
  }

  /// Returns the partial decoded object information
  pub const fn partial_decoded(&self) -> &PartialDecodedObjectAttribute {
    &self.partial_decoded
  }

  /// Returns the selector information
  pub const fn selector(&self) -> &SelectorAttribute {
    &self.selector
  }

  /// Returns the selector iterator information
  pub const fn selector_iter(&self) -> &SelectorIterAttribute {
    &self.selector_iter
  }

  /// Returns the indexer information
  pub const fn indexer(&self) -> &IndexerAttribute {
    &self.indexer
  }

  /// Returns whether the object is copyable
  pub const fn copy(&self) -> bool {
    self.copy
  }

  /// Returns the flavors of the object
  pub const fn flavors(&self) -> &[FlavorAttribute] {
    self.flavors.as_slice()
  }
}

pub struct Field {
  attrs: Vec<Attribute>,
  vis: Visibility,
  name: Ident,
  ty: Type,
  flavors: IndexMap<Ident, FieldFlavorAttribute>,
}

#[derive(Debug, Clone)]
pub struct SkippedField {
  attrs: Vec<Attribute>,
  vis: Visibility,
  name: Ident,
  ty: Type,
  default: Path,
}

impl SkippedField {
  /// Returns the name of the skipped field
  #[inline]
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  /// Returns the visibility of the skipped field
  #[inline]
  pub const fn vis(&self) -> &Visibility {
    &self.vis
  }

  /// Returns the type of the skipped field
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the attributes of the skipped field
  #[inline]
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns the path to the default value function for the skipped field
  #[inline]
  pub const fn default(&self) -> &Path {
    &self.default
  }

  fn try_new<F: RawField>(f: &F) -> darling::Result<Self> {
    let attrs = f.attrs().to_vec();
    let vis = f.vis().clone();
    let name = f.name().clone();
    let ty = f.ty().clone();
    let default = match f.default().cloned() {
      Some(path) => path,
      None => syn::parse2(quote! { <#ty as ::core::default::Default>::default })?,
    };

    Ok(Self {
      attrs,
      vis,
      name,
      ty,
      default,
    })
  }
}

#[derive(Debug, Clone)]
pub struct ConcreteTaggedField {
  attrs: Vec<Attribute>,
  vis: Visibility,
  name: Ident,
  ty: Type,
  schema_name: String,
  schema_description: String,
  flavor: FieldFlavorAttribute,
  label: Label,
  partial_decoded: PartialDecodedFieldAttribute,
  partial: PartialFieldAttribute,
  selector: SelectorFieldAttribute,
  default: Path,
  tag: u32,
}

impl ConcreteTaggedField {
  /// Returns the name of the field
  #[inline]
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  /// Returns the visibility of the field
  #[inline]
  pub const fn vis(&self) -> &Visibility {
    &self.vis
  }

  /// Returns the type of the field
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the attributes of the field
  #[inline]
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns the flavor of the field
  #[inline]
  pub const fn flavor(&self) -> &FieldFlavorAttribute {
    &self.flavor
  }

  /// Returns the tag of the field
  #[inline]
  pub const fn tag(&self) -> u32 {
    self.tag
  }

  /// Returns the path to the default value function for the field
  #[inline]
  pub const fn default(&self) -> &Path {
    &self.default
  }

  /// Returns the label of the field
  #[inline]
  pub const fn label(&self) -> &Label {
    &self.label
  }

  /// Returns the schema name of the field
  #[inline]
  pub const fn schema_name(&self) -> &str {
    self.schema_name.as_str()
  }

  /// Returns the schema description of the field
  #[inline]
  pub const fn schema_description(&self) -> &str {
    self.schema_description.as_str()
  }

  /// Returns the partial field type for this field, if any
  #[inline]
  pub const fn partial(&self) -> Option<&Type> {
    self.partial.ty()
  }

  /// Returns the attributes of the partial field for the field
  #[inline]
  pub const fn partial_attrs(&self) -> &[Attribute] {
    self.partial.attrs()
  }

  /// Returns the attributes of the partial decoded field for the field
  #[inline]
  pub const fn partial_decoded(&self) -> &[Attribute] {
    self.partial_decoded.attrs()
  }

  /// Returns the type of the partial decoded field for the field, if any
  #[inline]
  pub const fn partial_decoded_type(&self) -> Option<&Type> {
    self.partial_decoded.ty()
  }

  /// Returns the default selection of this field
  #[inline]
  pub const fn selection(&self) -> &Selection {
    self.selector.select()
  }

  /// Returns the attributes of the selector field for the field
  #[inline]
  pub const fn selector_attrs(&self) -> &[Attribute] {
    self.selector.attrs()
  }

  fn try_new<F: RawField>(f: &F, flavor: &FlavorAttribute) -> darling::Result<Self> {
    let attrs = f.attrs().to_vec();
    let vis = f.vis().clone();
    let name = f.name().clone();
    let ty = f.ty().clone();
    let tag = f
      .tag()
      .ok_or_else(|| {
        darling::Error::custom(format!("{name} is missing a tag, please add `tag = ...`"))
      })?
      .get();

    let mut field_flavor = None;
    for ff in f.flavors() {
      if ff.name() != flavor.name() {
        return Err(darling::Error::custom(format!(
          "Field {name} has flavor {}, but the object only supports flavor {}",
          ff.name(),
          flavor.name()
        )));
      }

      if field_flavor.is_some() {
        return Err(darling::Error::custom(format!(
          "Field {name} has duplicate {} flavor specified",
          ff.name()
        )));
      }

      field_flavor = Some(ff.clone());
    }

    let label = f.label().clone();
    let field_flavor = field_flavor.unwrap_or_else(|| {
      macro_rules! bail {
        ($skip:ident, $or_else:ident) => {{
          let skip_default = flavor.encode().$skip();
          let missing_operation = if flavor.decode().$or_else() {
            Some(MissingOperation::OrDefault(None))
          } else {
            None
          };
          (skip_default, missing_operation)
        }};
      }

      let (skip_default, missing_operation) = match f.label() {
        Label::Scalar => bail!(skip_default_scalar, or_else_default_scalar),
        Label::Bytes => bail!(skip_default_bytes, or_else_default_bytes),
        Label::String => bail!(skip_default_string, or_else_default_string),
        Label::Object => bail!(skip_default_object, or_else_default_object),
        Label::Enum => bail!(skip_default_enumeration, or_else_default_enumeration),
        Label::Union => bail!(skip_default_union, or_else_default_union),
        Label::Interface => bail!(skip_default_interface, or_else_default_interface),
        _ => (true, None),
      };

      FieldFlavorAttribute::new(
        flavor.name().clone(),
        None,
        FieldEncodeAttribute::new(Some(skip_default), None, None),
        FieldDecodeAttribute::new(missing_operation, None),
      )
    });

    let default = match f.default().cloned() {
      Some(path) => path,
      None => syn::parse2(quote! { <#ty as ::core::default::Default>::default })?,
    };
    let schema_name = f
      .schema()
      .name()
      .map(|s| s.to_string())
      .unwrap_or_else(|| name.to_string());
  
    let schema_description = f
      .schema()
      .description()
      .map(|s| s.to_string())
      .unwrap_or_default();

    Ok(Self {
      attrs,
      vis,
      name,
      schema_description,
      schema_name,
      ty,
      flavor: field_flavor,
      tag,
      default,
      label,
      partial_decoded: f.partial_decoded().clone(),
      partial: f.partial().clone(),
      selector: f.selector().clone(),
    })
  }
}

#[derive(Debug, Clone)]
pub enum ConcreteField {
  Skipped(SkippedField),
  Tagged(ConcreteTaggedField),
}

#[derive(Debug, Clone)]
pub enum GenericField {
  Skipped(SkippedField),
  Tagged(),
}

/// The AST for a concrete object, a concrete object which means there is only one flavor and the generated code will not be generic
/// over the flavor type.
#[derive(Debug, Clone)]
pub struct ConcreteObject {
  path_to_grost: Path,
  attrs: Vec<Attribute>,
  name: Ident,
  vis: Visibility,
  ty: Type,
  reflectable: Type,
  generics: Generics,
  flavor: FlavorAttribute,
  fields: Vec<ConcreteField>,
  default: Option<Path>,
  partial: PartialObject,
  partial_decoded: ConcretePartialDecodedObject,
  selector: ConcreteSelector,
  selector_iter: ConcreteSelectorIter,
}

impl ConcreteObject {
  /// Returns the path to the `grost` crate
  #[inline]
  pub const fn path_to_grost(&self) -> &Path {
    &self.path_to_grost
  }

  /// Returns the name of the object
  #[inline]
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  /// Returns the visibility of the object
  #[inline]
  pub const fn vis(&self) -> &Visibility {
    &self.vis
  }

  /// Returns the type of the object
  /// 
  /// e.g. If a struct is `struct MyObject<T> { ... }`, this will return `MyObject<T>`.
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the reflectable trait which replaces the generic parameter with the type of the object
  /// e.g. If a struct is `struct MyObject<T> { ... }`, this will return `Reflectable<MyObject<T>>`.
  #[inline]
  pub const fn reflectable(&self) -> &Type {
    &self.reflectable
  }

  /// Returns the flavor
  #[inline]
  pub const fn flavor(&self) -> &FlavorAttribute {
    &self.flavor
  }

  /// Returns the generics in the object definition if any.
  #[inline]
  pub const fn generics(&self) -> &Generics {
    &self.generics
  }

  /// Returns the attributes in the object definition.
  #[inline]
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns the fields of the object
  #[inline]
  pub const fn fields(&self) -> &[ConcreteField] {
    self.fields.as_slice()
  }

  /// Returns the path to the fn that returns the default value of the object, if any.
  #[inline]
  pub const fn default(&self) -> Option<&Path> {
    self.default.as_ref()
  }

  /// Returns the partial object information
  #[inline]
  pub const fn partial(&self) -> &PartialObject {
    &self.partial
  }

  /// Returns the partial decoded object information
  #[inline]
  pub const fn partial_decoded(&self) -> &ConcretePartialDecodedObject {
    &self.partial_decoded
  }

  /// Returns the selector information
  #[inline]
  pub const fn selector(&self) -> &ConcreteSelector {
    &self.selector
  }

  /// Returns the selector iterator information
  #[inline]
  pub const fn selector_iter(&self) -> &ConcreteSelectorIter {
    &self.selector_iter
  }

  fn try_new<O>(object: &O, flavor: &FlavorAttribute) -> darling::Result<Self>
  where
    O: RawObject,
  {
    let path_to_grost = object.path_to_grost().clone();
    let attrs = object.attrs().to_vec();
    let name = object.name().clone();
    let vis = object.vis().clone();
    let generics = object.generics().clone();
    let (_, tg, _) = generics.split_for_impl();

    let ty: Type = syn::parse2(quote! {
      #name #tg
    })?;
    let reflectable: Type = syn::parse2(quote! {
      #path_to_grost::__private::reflection::Reflectable<#ty>
    })?;

    let mut tags = IndexSet::new();

    let fields = object
      .fields()
      .iter()
      .map(|&f| {
        if f.skip() {
          SkippedField::try_new(f).map(ConcreteField::Skipped)
        } else {
          ConcreteTaggedField::try_new(f, flavor).and_then(|f| {
            if tags.contains(&f.tag()) {
              Err(darling::Error::custom(format!(
                "{name} has multiple fields have the same tag {}",
                f.tag()
              )))
            } else {
              tags.insert(f.tag());
              Ok(ConcreteField::Tagged(f))
            }
          })
        }
      })
      .collect::<darling::Result<Vec<_>>>()
      .and_then(|fields| {
        if fields.is_empty() {
          Err(darling::Error::custom(format!(
            "{name} must have at least one field"
          )))
        } else {
          Ok(fields)
        }
      })?;
    
    let partial = PartialObject::from_attribute(&name, object.partial())?;
    let partial_decoded = ConcretePartialDecodedObject::from_attribute(
      &name,
      flavor.ty().clone(),
      object.partial_decoded(),
    )?;
    let selector = ConcreteSelector::from_attribute(&name, flavor.ty(), object.selector())?;
    let selector_iter =
      ConcreteSelectorIter::from_attribute(selector.name(), flavor.ty(), object.selector_iter())?;

    Ok(Self {
      path_to_grost,
      attrs,
      name,
      vis,
      ty,
      reflectable,
      generics,
      flavor: flavor.clone(),
      fields,
      default: object.default().cloned(),
      partial,
      partial_decoded,
      selector,
      selector_iter,
    })
  }
}

/// The AST for a generic object, a generic object which means there are multiple flavors and the generated code will be generic over the flavor type.
#[derive(Debug, Clone)]
pub struct GenericObject {
  path_to_grost: Path,
  attrs: Vec<Attribute>,
  default: Option<Path>,
  name: Ident,
  vis: Visibility,
  ty: Type,
  reflectable: Type,
  generics: Generics,
  flavors: IndexMap<Ident, FlavorAttribute>,
}

impl GenericObject {
  /// Returns the path to the `grost` crate.
  #[inline]
  pub const fn path_to_grost(&self) -> &Path {
    &self.path_to_grost
  }

  /// Returns the name of the object.
  #[inline]
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  /// Returns the visibility of the object.
  #[inline]
  pub const fn vis(&self) -> &Visibility {
    &self.vis
  }

  /// Returns the type of the object.
  /// 
  /// e.g. If a struct is `struct MyObject<T> { ... }`, this will return `MyObject<T>`.
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the reflectable trait which replaces the generic parameter with the type of the object.
  /// 
  /// e.g. If a struct is `struct MyObject<T> { ... }`, this will return `Reflectable<MyObject<T>>`.
  #[inline]
  pub const fn reflectable(&self) -> &Type {
    &self.reflectable
  }

  /// Returns the generics in the object definition.
  #[inline]
  pub const fn generics(&self) -> &Generics {
    &self.generics
  }

  /// Returns the flavor attributes in the object definition.
  #[inline]
  pub const fn flavors(&self) -> &IndexMap<Ident, FlavorAttribute> {
    &self.flavors
  }

  /// Returns the attributes in the object definition.
  #[inline]
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns the path to the fn that returns the default value of the object, if any.
  #[inline]
  pub const fn default(&self) -> Option<&Path> {
    self.default.as_ref()
  }

  fn try_new<O>(object: &O) -> darling::Result<Self>
  where
    O: RawObject,
  {
    let path_to_grost = object.path_to_grost().clone();
    let attrs = object.attrs().to_vec();
    let name = object.name().clone();
    let vis = object.vis().clone();
    let generics = object.generics().clone();
    let (_, tg, _) = generics.split_for_impl();

    let ty: Type = syn::parse2(quote! {
      #name #tg
    })?;
    let reflectable: Type = syn::parse2(quote! {
      #path_to_grost::__private::reflection::Reflectable<#ty>
    })?;

    todo!()
  }
}

/// The AST for an object, which can be either a concrete or a generic object.
/// 
/// The main purpose to having an AST for an object is used to validate the input (from the Rust's derive macro or attribute macro)
/// from the schema and to generate the necessary Middle Intermediate Representation (MIR) of the object.
/// 
/// A Middle Intermediate Representation (MIR) of the object can be
/// generated from this structure. Once the MIR is generated,
/// it can be used to generate the final Rust code for the object in a GraphQL Protocol schema.
#[derive(Debug, Clone, derive_more::IsVariant, derive_more::Unwrap, derive_more::TryUnwrap)]
#[unwrap(ref)]
#[try_unwrap(ref)]
pub enum Object {
  Concrete(ConcreteObject),
  Generic(GenericObject),
}

impl Object {
  /// Creates an `Object` from a raw object input.
  pub fn from_raw<O>(object: &O) -> darling::Result<Self>
  where
    O: RawObject,
  {
    let num_flavors = object.flavors().len();
    if object.partial_decoded().flavor().is_none() && num_flavors == 1 {
      let flavor = object
        .flavors()
        .iter()
        .next()
        .expect("There must be a flavor were already checked");
      ConcreteObject::try_new(object, flavor).map(Object::Concrete)
    } else {
      GenericObject::try_new(object).map(Object::Generic)
    }
  }

  /// Returns the path to the `grost` crate.
  #[inline]
  pub const fn path_to_grost(&self) -> &Path {
    match self {
      Self::Concrete(obj) => obj.path_to_grost(),
      Self::Generic(obj) => obj.path_to_grost(),
    }
  }

  /// Returns the name of the object.
  #[inline]
  pub const fn name(&self) -> &Ident {
    match self {
      Self::Concrete(obj) => obj.name(),
      Self::Generic(obj) => obj.name(),
    }
  }

  /// Returns the visibility of the object.
  #[inline]
  pub const fn vis(&self) -> &Visibility {
    match self {
      Self::Concrete(obj) => obj.vis(),
      Self::Generic(obj) => obj.vis(),
    }
  }

  /// Returns the type of the object.
  /// 
  /// e.g. If a struct is `struct MyObject<T> { ... }`, this will return `MyObject<T>`.
  #[inline]
  pub const fn ty(&self) -> &Type {
    match self {
      Self::Concrete(obj) => obj.ty(),
      Self::Generic(obj) => obj.ty(),
    }
  }

  /// Returns the reflectable trait which replaces the generic parameter with the type of the object.
  /// 
  /// e.g. If a struct is `struct MyObject<T> { ... }`, this will return `Reflectable<MyObject<T>>`.
  #[inline]
  pub const fn reflectable(&self) -> &Type {
    match self {
      Self::Concrete(obj) => obj.reflectable(),
      Self::Generic(obj) => obj.reflectable(),
    }
  }

  /// Returns the generics in the object definition.
  #[inline]
  pub const fn generics(&self) -> &Generics {
    match self {
      Self::Concrete(obj) => obj.generics(),
      Self::Generic(obj) => obj.generics(),
    }
  }

  /// Returns the attributes in the object definition.
  #[inline]
  pub const fn attrs(&self) -> &[Attribute] {
    match self {
      Self::Concrete(obj) => obj.attrs(),
      Self::Generic(obj) => obj.attrs(),
    }
  }

  /// Returns the default value function for the object, if any.
  #[inline]
  pub const fn default(&self) -> Option<&Path> {
    match self {
      Self::Concrete(obj) => obj.default(),
      Self::Generic(obj) => obj.default(),
    }
  }
}

/// The trait for the object derive input
pub trait RawObject: Clone {
  /// The type of the field
  type Field: RawField;

  /// Returns the name of the object
  fn name(&self) -> &Ident;

  /// Returns the type of the object
  ///
  /// e.g. If a struct is `struct MyObject<T> { ... }`, this will return `MyObject<T>`.
  fn ty(&self) -> &syn::Type;

  /// Returns the reflectable trait which replaces the generic parameter with the type of the object
  ///
  /// e.g. If a struct is `struct MyObject<T> { ... }`, this will return `Reflectable<MyObject<T>>`.
  fn reflectable(&self) -> &syn::Type;

  /// Returns the visibility of the object
  fn vis(&self) -> &syn::Visibility;

  /// Returns the generics in the object defination.
  fn generics(&self) -> &syn::Generics;

  /// Returns the attributes in the object defination.
  fn attrs(&self) -> &[Attribute];

  /// Returns the fields of the object
  fn fields(&self) -> Vec<&Self::Field>;

  /// Returns the path to the `grost` crate
  fn path_to_grost(&self) -> &syn::Path;

  /// Returns the path to the fn that returns the default value of the object
  fn default(&self) -> Option<&syn::Path>;

  /// Returns the schema information
  fn schema(&self) -> &SchemaAttribute;

  /// Returns the partial object information
  fn partial(&self) -> &PartialObjectAttribute;

  /// Returns the partial decoded object information
  fn partial_decoded(&self) -> &PartialDecodedObjectAttribute;

  /// Returns the selector information
  fn selector(&self) -> &SelectorAttribute;

  /// Returns the selector iterator information
  fn selector_iter(&self) -> &SelectorIterAttribute;

  /// Returns the indexer information
  fn indexer(&self) -> &IndexerAttribute;

  /// Returns whether the object is copyable
  fn copy(&self) -> bool;

  /// Returns the flavors of the object
  fn flavors(&self) -> &[FlavorAttribute];
}

/// The extension trait for the object
pub trait RawObjectExt: RawObject {
  #[inline]
  fn partial_decoded_name(&self) -> Ident {
    self
      .partial_decoded()
      .name()
      .cloned()
      .unwrap_or_else(|| format_ident!("PartialDecoded{}", self.name()))
  }

  #[inline]
  fn partial_name(&self) -> Ident {
    self
      .partial()
      .name()
      .cloned()
      .unwrap_or_else(|| format_ident!("Partial{}", self.name()))
  }

  #[inline]
  fn selector_name(&self) -> Ident {
    self
      .selector()
      .name()
      .cloned()
      .unwrap_or_else(|| format_ident!("{}Selector", self.name()))
  }

  #[inline]
  fn selector_iter_name(&self) -> Ident {
    self
      .selector_iter()
      .name()
      .cloned()
      .unwrap_or_else(|| format_ident!("{}Iter", self.selector_name()))
  }

  #[inline]
  fn indexer_name(&self) -> Ident {
    self
      .indexer()
      .name()
      .cloned()
      .unwrap_or_else(|| format_ident!("{}Index", self.name()))
  }
}

impl<T: RawObject> RawObjectExt for T {}
