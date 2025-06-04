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

mod field;

#[derive(Debug, Default, Clone, FromMeta)]
pub struct PartialObjectFromMeta {
  #[darling(default, rename = "rename")]
  name: Option<Ident>,
  #[darling(default, map = "Attributes::into_inner")]
  attrs: Vec<Attribute>,
}

impl PartialObjectFromMeta {
  fn finalize(self, unknown_buffer_generic: syn::TypeParam) -> PartialObjectAttribute {
    PartialObjectAttribute {
      name: self.name,
      attrs: self.attrs,
      unknown_buffer_generic,
    }
  }
}

#[derive(Debug, Clone)]
pub struct PartialObjectAttribute {
  name: Option<Ident>,
  attrs: Vec<Attribute>,
  unknown_buffer_generic: syn::TypeParam,
}

impl PartialObjectAttribute {
  /// Returns the name of the partial object
  pub(crate) const fn name(&self) -> Option<&Ident> {
    self.name.as_ref()
  }

  /// Returns the attributes of the partial object
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns the unknown buffer generic parameter
  pub const fn unknown_buffer(&self) -> &syn::TypeParam {
    &self.unknown_buffer_generic
  }
}

#[derive(Debug, Default, Clone, FromMeta)]
struct PartialDecodedObjectFromMeta {
  #[darling(default, rename = "rename")]
  name: Option<Ident>,
  #[darling(default, map = "Attributes::into_inner")]
  attrs: Vec<Attribute>,
  #[darling(default)]
  copy: bool,
}

impl PartialDecodedObjectFromMeta {
  fn finalize(
    self,
    flavor_param: Option<syn::TypeParam>,
    unknown_buffer_param: syn::TypeParam,
    lifetime_param: syn::LifetimeParam,
  ) -> PartialDecodedObjectAttribute {
    PartialDecodedObjectAttribute {
      name: self.name,
      attrs: self.attrs,
      copy: self.copy,
      flavor_param,
      unknown_buffer_param,
      lifetime_param,
    }
  }
}

#[derive(Debug, Clone)]
pub struct PartialDecodedObjectAttribute {
  name: Option<Ident>,
  attrs: Vec<Attribute>,
  copy: bool,
  flavor_param: Option<syn::TypeParam>,
  unknown_buffer_param: syn::TypeParam,
  lifetime_param: syn::LifetimeParam,
}

impl PartialDecodedObjectAttribute {
  /// Returns the name of the partial reference object
  pub(crate) const fn name(&self) -> Option<&Ident> {
    self.name.as_ref()
  }

  /// Returns the attributes of the partial reference object
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns whether the partial reference object is copyable
  pub const fn copy(&self) -> bool {
    self.copy
  }

  /// Returns the flavor generic parameter if it exists
  pub const fn flavor(&self) -> Option<&syn::TypeParam> {
    self.flavor_param.as_ref()
  }

  /// Returns the unknown buffer generic parameter
  pub const fn unknown_buffer(&self) -> &syn::TypeParam {
    &self.unknown_buffer_param
  }

  /// Returns the lifetime generic parameter
  pub const fn lifetime(&self) -> &syn::LifetimeParam {
    &self.lifetime_param
  }
}

#[derive(Debug, Default, Clone, FromMeta)]
struct SelectorIterFromMeta {
  #[darling(default, rename = "rename")]
  name: Option<Ident>,
  #[darling(default, map = "Attributes::into_inner")]
  attrs: Vec<Attribute>,
}

impl SelectorIterFromMeta {
  fn finalize(self, flavor_param: Option<syn::TypeParam>) -> SelectorIterAttribute {
    SelectorIterAttribute {
      name: self.name,
      attrs: self.attrs,
      flavor_param,
    }
  }
}

#[derive(Debug, Default, Clone)]
pub struct SelectorIterAttribute {
  name: Option<Ident>,
  attrs: Vec<Attribute>,
  flavor_param: Option<syn::TypeParam>,
}

impl SelectorIterAttribute {
  /// Returns the name of the selector iterator
  pub(crate) const fn name(&self) -> Option<&Ident> {
    self.name.as_ref()
  }

  /// Returns the attributes of the selector iterator
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns the flavor generic parameter if it exists
  pub const fn flavor(&self) -> Option<&syn::TypeParam> {
    self.flavor_param.as_ref()
  }
}

#[derive(Debug, Default, Clone, FromMeta)]
pub struct SelectorFromMeta {
  #[darling(default, rename = "rename")]
  name: Option<Ident>,
  #[darling(default, map = "Attributes::into_inner")]
  attrs: Vec<Attribute>,
}

impl SelectorFromMeta {
  fn finalize(
    self,
    flavor_param: Option<syn::TypeParam>,
    wire_format: syn::TypeParam,
  ) -> SelectorAttribute {
    SelectorAttribute {
      name: self.name,
      attrs: self.attrs,
      flavor_param,
      wire_format_param: wire_format,
    }
  }
}

#[derive(Debug, Clone)]
pub struct SelectorAttribute {
  name: Option<Ident>,
  attrs: Vec<Attribute>,
  flavor_param: Option<syn::TypeParam>,
  wire_format_param: syn::TypeParam,
}

impl SelectorAttribute {
  /// Returns the name of the selector
  pub(crate) const fn name(&self) -> Option<&Ident> {
    self.name.as_ref()
  }

  /// Returns the attributes of the selector
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns the flavor generic parameter if it exists
  pub const fn flavor(&self) -> Option<&syn::TypeParam> {
    self.flavor_param.as_ref()
  }

  /// Returns the wire format generic parameter
  pub const fn wire_format(&self) -> &syn::TypeParam {
    &self.wire_format_param
  }
}

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
  flavor: FieldFlavorAttribute,
  tag: u32,
}

impl ConcreteTaggedField {
  /// Returns the name of the tagged field
  #[inline]
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  /// Returns the visibility of the tagged field
  #[inline]
  pub const fn vis(&self) -> &Visibility {
    &self.vis
  }

  /// Returns the type of the tagged field
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the attributes of the tagged field
  #[inline]
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns the flavor of the tagged field
  #[inline]
  pub const fn flavor(&self) -> &FieldFlavorAttribute {
    &self.flavor
  }

  /// Returns the tag of the tagged field
  #[inline]
  pub const fn tag(&self) -> u32 {
    self.tag
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

    Ok(Self {
      attrs,
      vis,
      name,
      ty,
      flavor: field_flavor,
      tag,
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
}

impl ConcreteObject {
  #[inline]
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  #[inline]
  pub const fn vis(&self) -> &Visibility {
    &self.vis
  }

  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  #[inline]
  pub const fn reflectable(&self) -> &Type {
    &self.reflectable
  }

  #[inline]
  pub const fn flavor(&self) -> &FlavorAttribute {
    &self.flavor
  }

  #[inline]
  pub const fn generics(&self) -> &Generics {
    &self.generics
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
    })
  }
}

/// The AST for a generic object, a generic object which means there are multiple flavors and the generated code will be generic over the flavor type.
#[derive(Debug, Clone)]
pub struct GenericObject {
  path_to_grost: Path,
  attrs: Vec<Attribute>,
  name: Ident,
  vis: Visibility,
  ty: Type,
  reflectable: Type,
  generics: Generics,
  flavors: IndexMap<Ident, FlavorAttribute>,
}

impl GenericObject {
  #[inline]
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  #[inline]
  pub const fn vis(&self) -> &Visibility {
    &self.vis
  }

  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  #[inline]
  pub const fn reflectable(&self) -> &Type {
    &self.reflectable
  }

  #[inline]
  pub const fn generics(&self) -> &Generics {
    &self.generics
  }

  #[inline]
  pub const fn flavors(&self) -> &IndexMap<Ident, FlavorAttribute> {
    &self.flavors
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
#[derive(Debug, Clone)]
pub enum Object {
  Concrete(ConcreteObject),
  Generic(GenericObject),
}

impl Object {
  pub fn new<O>(object: &O) -> darling::Result<Self>
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

  #[inline]
  pub const fn name(&self) -> &Ident {
    match self {
      Self::Concrete(obj) => obj.name(),
      Self::Generic(obj) => obj.name(),
    }
  }

  #[inline]
  pub const fn vis(&self) -> &Visibility {
    match self {
      Self::Concrete(obj) => obj.vis(),
      Self::Generic(obj) => obj.vis(),
    }
  }

  #[inline]
  pub const fn ty(&self) -> &Type {
    match self {
      Self::Concrete(obj) => obj.ty(),
      Self::Generic(obj) => obj.ty(),
    }
  }

  #[inline]
  pub const fn reflectable(&self) -> &Type {
    match self {
      Self::Concrete(obj) => obj.reflectable(),
      Self::Generic(obj) => obj.reflectable(),
    }
  }

  #[inline]
  pub const fn generics(&self) -> &Generics {
    match self {
      Self::Concrete(obj) => obj.generics(),
      Self::Generic(obj) => obj.generics(),
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
