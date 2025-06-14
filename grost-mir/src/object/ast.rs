use darling::usage::{GenericsExt, IdentSet, LifetimeSet, Purpose, UsesLifetimes, UsesTypeParams};
use indexmap::{IndexMap, IndexSet};
use quote::{format_ident, quote};
use syn::{Attribute, Generics, Ident, LifetimeParam, Path, Type, TypeParam, Visibility};

use crate::{
  flavor::{DecodeAttribute, EncodeAttribute, FlavorAttribute, IdentifierAttribute, TagAttribute},
  object::{FieldSelection, Label, meta::ObjectFromMeta},
  utils::{Invokable, MissingOperation, SchemaAttribute, grost_flavor_param},
};

pub use field::*;
pub use indexer::*;
pub use partial::*;
pub use partial_decoded::*;
pub use selector::*;

mod field;
mod indexer;
mod partial;
mod partial_decoded;
mod selector;

#[derive(Debug, Clone)]
pub struct SkippedField<M = ()> {
  attrs: Vec<Attribute>,
  vis: Visibility,
  type_params_usages: IdentSet,
  lifetime_params_usages: LifetimeSet,
  name: Ident,
  ty: Type,
  default: Invokable,
  meta: M,
}

impl<M> SkippedField<M> {
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
  pub const fn default(&self) -> &Invokable {
    &self.default
  }

  /// Returns the metadata associated with the skipped field
  #[inline]
  pub const fn meta(&self) -> &M {
    &self.meta
  }

  /// Returns the type parameters used in the skipped field
  #[inline]
  pub const fn type_params_usages(&self) -> &IdentSet {
    &self.type_params_usages
  }

  /// Returns the lifetime parameters used in the skipped field
  #[inline]
  pub const fn lifetime_params_usages(&self) -> &LifetimeSet {
    &self.lifetime_params_usages
  }

  fn try_new<F: RawField<Meta = M>>(
    f: &F,
    type_params: &IdentSet,
    lifetime_params: &LifetimeSet,
  ) -> darling::Result<Self>
  where
    M: Clone,
  {
    let attrs = f.attrs().to_vec();
    let vis = f.vis().clone();
    let name = f.name().clone();
    let ty = f.ty().clone();
    let default = match f.default().cloned() {
      Some(path) => path,
      None => {
        syn::parse2::<syn::Path>(quote! { ::core::default::Default::default }).map(Into::into)?
      }
    };

    let purpose: darling::usage::Options = Purpose::Declare.into();
    let type_params_usages = ty.uses_type_params_cloned(&purpose, type_params);
    let lifetime_params_usages = ty.uses_lifetimes_cloned(&purpose, lifetime_params);

    Ok(Self {
      attrs,
      vis,
      name,
      type_params_usages,
      lifetime_params_usages,
      ty,
      default,
      meta: f.meta().clone(),
    })
  }
}

#[derive(Debug, Clone)]
pub(super) struct ConcreteTaggedField<M = ()> {
  pub(super) attrs: Vec<Attribute>,
  pub(super) vis: Visibility,
  pub(super) name: Ident,
  pub(super) ty: Type,
  pub(super) schema_name: String,
  pub(super) schema_description: String,
  pub(super) flavor: FieldFlavor,
  pub(super) label: Label,
  pub(super) type_params_usages: IdentSet,
  pub(super) lifetime_params_usages: LifetimeSet,
  pub(super) partial_decoded: PartialDecodedFieldAttribute,
  pub(super) partial: PartialFieldAttribute,
  pub(super) selector: SelectorFieldAttribute,
  pub(super) default: Invokable,
  pub(super) tag: u32,
  pub(super) copy: bool,
  pub(super) meta: M,
}

impl<M> ConcreteTaggedField<M> {
  /// Returns the name of the field
  #[inline]
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  /// Returns the type of the field
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the flavor of the field
  #[inline]
  pub const fn flavor(&self) -> &FieldFlavor {
    &self.flavor
  }

  /// Returns the tag of the field
  #[inline]
  pub const fn tag(&self) -> u32 {
    self.tag
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
  pub const fn partial_type(&self) -> Option<&Type> {
    self.partial.ty()
  }

  /// Returns the attributes of the partial field for the field
  #[inline]
  pub const fn partial_attrs(&self) -> &[Attribute] {
    self.partial.attrs()
  }

  /// Returns the type of the partial decoded field for the field, if any
  #[inline]
  pub const fn partial_decoded_type(&self) -> Option<&Type> {
    self.partial_decoded.ty()
  }

  /// Returns `true` if the field is partial decoded field is copyable, `false` otherwise
  #[inline]
  pub const fn partial_decoded_copy(&self) -> bool {
    self.partial_decoded.copy()
  }

  fn try_new<F: RawField<Meta = M>>(
    f: &F,
    flavor: &FlavorAttribute,
    type_params: &IdentSet,
    lifetime_params: &LifetimeSet,
  ) -> darling::Result<Self>
  where
    M: Clone,
  {
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

    let label = f
      .label()
      .ok_or_else(|| darling::Error::custom(format!("field `{name}` is missing label")))?;
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

      let (skip_default, missing_operation) = match label {
        Label::Scalar => bail!(skip_default_scalar, or_else_default_scalar),
        Label::Bytes => bail!(skip_default_bytes, or_else_default_bytes),
        Label::String => bail!(skip_default_string, or_else_default_string),
        Label::Object => bail!(skip_default_object, or_else_default_object),
        Label::Enum => bail!(skip_default_enumeration, or_else_default_enumeration),
        Label::Union => bail!(skip_default_union, or_else_default_union),
        Label::Interface => bail!(skip_default_interface, or_else_default_interface),
        Label::Map { .. } => bail!(skip_default_map, or_else_default_map),
        Label::List(_) => bail!(skip_default_list, or_else_default_list),
        Label::Set(_) => bail!(skip_default_set, or_else_default_set),
        _ => (true, None),
      };

      FieldFlavorAttribute::new(
        flavor.name().clone(),
        None,
        None,
        FieldEncodeAttribute::new(Some(skip_default), None, None),
        FieldDecodeAttribute::new(missing_operation, None),
      )
    });

    let default = match f.default().cloned() {
      Some(path) => path,
      None => syn::parse2::<Path>(quote! { ::core::default::Default::default }).map(Into::into)?,
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
      type_params_usages: ty.uses_type_params_cloned(&Purpose::Declare.into(), type_params),
      lifetime_params_usages: ty.uses_lifetimes_cloned(&Purpose::Declare.into(), lifetime_params),
      ty,
      flavor: FieldFlavor {
        ty: field_flavor.ty().cloned(),
        format: field_flavor.format().cloned(),
        flavor_type: flavor.ty().clone(),
        encode: field_flavor.encode().clone(),
        decode: field_flavor.decode().clone(),
      },
      tag,
      default,
      label: label.clone(),
      partial_decoded: f.partial_decoded().clone(),
      partial: f.partial().clone(),
      selector: f.selector().clone(),
      copy: f.copy(),
      meta: f.meta().clone(),
    })
  }
}

#[derive(Debug, Clone, derive_more::IsVariant, derive_more::Unwrap, derive_more::TryUnwrap)]
#[unwrap(ref)]
#[try_unwrap(ref)]
pub(super) enum ConcreteField<M = ()> {
  Skipped(Box<SkippedField<M>>),
  Tagged(Box<ConcreteTaggedField<M>>),
}

impl<M> ConcreteField<M> {
  #[inline]
  pub const fn tag(&self) -> Option<u32> {
    match self {
      ConcreteField::Skipped(_) => None,
      ConcreteField::Tagged(tagged) => Some(tagged.tag()),
    }
  }
}

#[derive(Debug, Clone)]
pub struct FieldFlavor {
  ty: Option<Type>,
  format: Option<Type>,
  flavor_type: Type,
  encode: FieldEncodeAttribute,
  decode: FieldDecodeAttribute,
}

impl FieldFlavor {
  /// Returns the wire format type for the field of this flavor, if specified
  #[inline]
  pub const fn format(&self) -> Option<&Type> {
    self.format.as_ref()
  }

  /// Returns the type of the partial decoded field for this flavor, if specified
  #[inline]
  pub const fn ty(&self) -> Option<&Type> {
    self.ty.as_ref()
  }

  /// Returns the type of the flavor
  #[inline]
  pub const fn flavor_type(&self) -> &Type {
    &self.flavor_type
  }

  /// Returns the encode attribute for this flavor
  #[inline]
  pub const fn encode(&self) -> &FieldEncodeAttribute {
    &self.encode
  }

  /// Returns the decode attribute for this flavor
  #[inline]
  pub const fn decode(&self) -> &FieldDecodeAttribute {
    &self.decode
  }
}

#[derive(Debug, Clone)]
pub(super) struct GenericTaggedField<M = ()> {
  pub(super) attrs: Vec<Attribute>,
  pub(super) vis: Visibility,
  pub(super) name: Ident,
  pub(super) ty: Type,
  pub(super) schema_name: String,
  pub(super) schema_description: String,
  pub(super) type_params_usages: IdentSet,
  pub(super) lifetime_params_usages: LifetimeSet,
  pub(super) flavor_param: TypeParam,
  pub(super) label: Label,
  pub(super) partial_decoded: PartialDecodedFieldAttribute,
  pub(super) partial: PartialFieldAttribute,
  pub(super) selector: SelectorFieldAttribute,
  pub(super) default: Invokable,
  pub(super) tag: u32,
  pub(super) flavors: IndexMap<Ident, FieldFlavor>,
  pub(super) copy: bool,
  pub(super) meta: M,
}

impl<M> GenericTaggedField<M> {
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

  /// Returns the generic flavor parameter for the field
  #[inline]
  pub const fn flavor(&self) -> &TypeParam {
    &self.flavor_param
  }

  /// Returns the tag of the field
  #[inline]
  pub const fn tag(&self) -> u32 {
    self.tag
  }

  /// Returns the path to the default value function for the field
  #[inline]
  pub const fn default(&self) -> &Invokable {
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

  /// Returns `true` if the field is copyable, `false` otherwise
  #[inline]
  pub const fn copy(&self) -> bool {
    self.copy
  }

  /// Returns the metadata associated with the field
  #[inline]
  pub const fn meta(&self) -> &M {
    &self.meta
  }

  /// Returns the partial field type for this field, if any
  #[inline]
  pub const fn partial_type(&self) -> Option<&Type> {
    self.partial.ty()
  }

  /// Returns the attributes of the partial field for the field
  #[inline]
  pub const fn partial_attrs(&self) -> &[Attribute] {
    self.partial.attrs()
  }

  /// Returns the attributes of the partial decoded field for the field
  #[inline]
  pub const fn partial_decoded_attrs(&self) -> &[Attribute] {
    self.partial_decoded.attrs()
  }

  /// Returns the type of the partial decoded field for the field, if any
  #[inline]
  pub const fn partial_decoded_type(&self) -> Option<&Type> {
    self.partial_decoded.ty()
  }

  /// Returns `true` if the field is partial decoded field is copyable, `false` otherwise
  #[inline]
  pub const fn partial_decoded_copy(&self) -> bool {
    self.partial_decoded.copy()
  }

  /// Returns the default selection of this field
  #[inline]
  pub const fn selection(&self) -> &FieldSelection {
    self.selector.select()
  }

  /// Returns the attributes of the selector field for the field
  #[inline]
  pub const fn selector_attrs(&self) -> &[Attribute] {
    self.selector.attrs()
  }

  /// Returns the flavors of the field
  #[inline]
  pub const fn flavors(&self) -> &IndexMap<Ident, FieldFlavor> {
    &self.flavors
  }

  fn try_new<F: RawField<Meta = M>>(
    f: &F,
    flavor_param: &TypeParam,
    flavors: &IndexMap<Ident, ObjectFlavor>,
    type_params: &IdentSet,
    lifetime_params: &LifetimeSet,
    copy: bool,
  ) -> darling::Result<Self>
  where
    M: Clone,
  {
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
    let default = match f.default().cloned() {
      Some(path) => path,
      None => syn::parse2::<Path>(quote! { ::core::default::Default::default }).map(Into::into)?,
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

    let label = f
      .label()
      .ok_or_else(|| darling::Error::custom(format!("field `{name}` is missing label")))?;
    let field_flavors = flavors
      .iter()
      .map(|(name, flavor)| {
        let field_flavor = match f.flavors().iter().find(|ff| ff.name().eq(name)) {
          Some(ff) => FieldFlavor {
            ty: ff.ty().cloned(),
            format: ff.format().cloned(),
            flavor_type: flavor.ty().clone(),
            encode: ff.encode().clone(),
            decode: ff.decode().clone(),
          },
          None => {
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

            let (skip_default, missing_operation) = match label {
              Label::Scalar => bail!(skip_default_scalar, or_else_default_scalar),
              Label::Bytes => bail!(skip_default_bytes, or_else_default_bytes),
              Label::String => bail!(skip_default_string, or_else_default_string),
              Label::Object => bail!(skip_default_object, or_else_default_object),
              Label::Enum => bail!(skip_default_enumeration, or_else_default_enumeration),
              Label::Union => bail!(skip_default_union, or_else_default_union),
              Label::Interface => bail!(skip_default_interface, or_else_default_interface),
              Label::Map { .. } => bail!(skip_default_map, or_else_default_map),
              Label::List(_) => bail!(skip_default_list, or_else_default_list),
              Label::Set(_) => bail!(skip_default_set, or_else_default_set),
              _ => (true, None),
            };

            FieldFlavor {
              ty: None,
              format: None,
              flavor_type: flavor.ty().clone(),
              encode: FieldEncodeAttribute::new(Some(skip_default), None, None),
              decode: FieldDecodeAttribute::new(missing_operation, None),
            }
          }
        };
        Ok((name.clone(), field_flavor))
      })
      .collect::<darling::Result<IndexMap<_, _>>>()?;
    Ok(Self {
      attrs,
      vis,
      name,
      type_params_usages: ty.uses_type_params_cloned(&Purpose::Declare.into(), type_params),
      lifetime_params_usages: ty.uses_lifetimes_cloned(&Purpose::Declare.into(), lifetime_params),
      ty,
      schema_name,
      schema_description,
      flavor_param: flavor_param.clone(),
      label: label.clone(),
      partial_decoded: f.partial_decoded().clone(),
      partial: f.partial().clone(),
      selector: f.selector().clone(),
      meta: f.meta().clone(),
      default,
      tag,
      copy,
      flavors: field_flavors,
    })
  }
}

#[derive(Debug, Clone, derive_more::IsVariant, derive_more::Unwrap, derive_more::TryUnwrap)]
#[unwrap(ref)]
#[try_unwrap(ref)]
pub(super) enum GenericField<M = ()> {
  Skipped(Box<SkippedField<M>>),
  Tagged(Box<GenericTaggedField<M>>),
}

impl<M> GenericField<M> {
  pub(super) const fn tag(&self) -> Option<u32> {
    match self {
      GenericField::Skipped(_) => None,
      GenericField::Tagged(tagged) => Some(tagged.tag()),
    }
  }
}

#[derive(Debug, Clone)]
pub struct Indexer {
  pub(super) name: Ident,
  pub(super) attrs: Vec<Attribute>,
}

impl Indexer {
  /// Returns the name of the indexer
  #[inline]
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  /// Returns the attributes of the indexer
  #[inline]
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }
}

/// The AST for a concrete object, a concrete object which means there is only one flavor and the generated code will not be generic
/// over the flavor type.
#[derive(Debug, Clone)]
pub(super) struct ConcreteObject<M = (), F = ()> {
  pub(super) path_to_grost: Path,
  pub(super) attrs: Vec<Attribute>,
  pub(super) name: Ident,
  pub(super) schema_name: String,
  pub(super) schema_description: String,
  pub(super) vis: Visibility,
  pub(super) ty: Type,
  pub(super) reflectable: Type,
  pub(super) generics: Generics,
  pub(super) flavor: FlavorAttribute,
  pub(super) unknown_buffer_param: TypeParam,
  pub(super) lifetime_param: LifetimeParam,
  pub(super) fields: Vec<ConcreteField<F>>,
  pub(super) default: Option<Invokable>,
  pub(super) indexer: Indexer,
  pub(super) partial: PartialObject,
  pub(super) partial_decoded: ConcretePartialDecodedObject,
  pub(super) selector: ConcreteSelector,
  pub(super) selector_iter: ConcreteSelectorIter,
  pub(super) copy: bool,
  pub(super) meta: M,
}

impl<M, F> ConcreteObject<M, F> {
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

  /// Returns the schema name of the object
  #[inline]
  pub const fn schema_name(&self) -> &str {
    self.schema_name.as_str()
  }

  /// Returns the schema description of the object
  #[inline]
  pub const fn schema_description(&self) -> &str {
    self.schema_description.as_str()
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

  /// Returns the generic unknown buffer type parameter.
  #[inline]
  pub const fn unknown_buffer(&self) -> &TypeParam {
    &self.unknown_buffer_param
  }

  /// Returns the generic lifetime parameter.
  #[inline]
  pub const fn lifetime(&self) -> &LifetimeParam {
    &self.lifetime_param
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

  /// Returns `true` if the object is copyable, `false` otherwise.
  #[inline]
  pub const fn copy(&self) -> bool {
    self.copy
  }

  /// Returns the fields of the object
  #[inline]
  pub const fn fields(&self) -> &[ConcreteField<F>] {
    self.fields.as_slice()
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
    O: RawObject<Meta = M>,
    O::Field: RawField<Meta = F>,
    F: Clone,
    M: Clone,
  {
    let path_to_grost = object.path_to_grost().clone();
    let attrs = object.attrs().to_vec();
    let name = object.name().clone();
    let vis = object.vis().clone();
    let generics = object.generics().clone();
    let (_, tg, _) = generics.split_for_impl();

    let type_params = generics.declared_type_params();
    let lifetimes = generics.declared_lifetimes();

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
          SkippedField::try_new(f, &type_params, &lifetimes)
            .map(|f| ConcreteField::Skipped(Box::new(f)))
        } else {
          ConcreteTaggedField::try_new(f, flavor, &type_params, &lifetimes).and_then(|f| {
            if tags.contains(&f.tag()) {
              Err(darling::Error::custom(format!(
                "{name} has multiple fields have the same tag {}",
                f.tag()
              )))
            } else {
              tags.insert(f.tag());
              Ok(ConcreteField::Tagged(Box::new(f)))
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
    let partial_decoded =
      ConcretePartialDecodedObject::from_attribute(&name, object.partial_decoded())?;
    let selector = ConcreteSelector::from_attribute(&name, object.selector())?;
    let selector_iter =
      ConcreteSelectorIter::from_attribute(selector.name(), object.selector_iter())?;

    Ok(Self {
      path_to_grost,
      copy: object.copy(),
      attrs,
      indexer: Indexer {
        name: object.indexer_name().clone(),
        attrs: object.indexer().attrs().to_vec(),
      },
      schema_name: object
        .schema()
        .name()
        .map_or_else(|| name.to_string(), |s| s.to_string()),
      schema_description: object
        .schema()
        .description()
        .unwrap_or_default()
        .to_string(),
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
      meta: object.meta().clone(),
      unknown_buffer_param: object.unknown_buffer_type_param().clone(),
      lifetime_param: object.lifetime_param().clone(),
    })
  }
}

#[derive(Debug, Clone)]
pub struct ObjectFlavor {
  pub(super) ty: Type,
  pub(super) format: Type,
  pub(super) identifier: IdentifierAttribute,
  pub(super) tag: TagAttribute,
  pub(super) encode: EncodeAttribute,
  pub(super) decode: DecodeAttribute,
}

impl ObjectFlavor {
  /// Returns the type of the flavor
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the wire format type for the object of this flavor.
  #[inline]
  pub const fn format(&self) -> &Type {
    &self.format
  }

  /// Returns the identifier attribute for the flavor.
  #[inline]
  pub const fn identifier(&self) -> &IdentifierAttribute {
    &self.identifier
  }

  /// Returns the tag attribute for the flavor.
  #[inline]
  pub const fn tag(&self) -> &TagAttribute {
    &self.tag
  }

  /// Returns the encode attribute for this flavor.
  #[inline]
  pub const fn encode(&self) -> &EncodeAttribute {
    &self.encode
  }

  /// Returns the decode attribute for this flavor.
  #[inline]
  pub const fn decode(&self) -> &DecodeAttribute {
    &self.decode
  }

  fn from_attribute(attribute: &FlavorAttribute) -> darling::Result<Self> {
    Ok(Self {
      ty: attribute.ty().clone(),
      format: attribute.wire_format().clone(),
      identifier: attribute.identifier().clone(),
      tag: attribute.tag().clone(),
      encode: attribute.encode().clone(),
      decode: attribute.decode().clone(),
    })
  }
}

/// The AST for a generic object, a generic object which means there are multiple flavors and the generated code will be generic over the flavor type.
#[derive(Debug, Clone)]
pub(super) struct GenericObject<M = (), F = ()> {
  pub(super) path_to_grost: Path,
  pub(super) attrs: Vec<Attribute>,
  pub(super) default: Option<Invokable>,
  pub(super) name: Ident,
  pub(super) schema_name: String,
  pub(super) schema_description: String,
  pub(super) flavor_param: TypeParam,
  pub(super) unknown_buffer_param: TypeParam,
  pub(super) lifetime_param: LifetimeParam,
  pub(super) wire_format_param: TypeParam,
  pub(super) vis: Visibility,
  pub(super) ty: Type,
  pub(super) reflectable: Type,
  pub(super) fields: Vec<GenericField<F>>,
  pub(super) generics: Generics,
  pub(super) partial: PartialObject,
  pub(super) partial_decoded: GenericPartialDecodedObject,
  pub(super) selector: GenericSelector,
  pub(super) selector_iter: GenericSelectorIter,
  pub(super) indexer: Indexer,
  pub(super) flavors: IndexMap<Ident, ObjectFlavor>,
  pub(super) copy: bool,
  pub(super) meta: M,
}

impl<M, F> GenericObject<M, F> {
  /// Returns the path to the `grost` crate.
  #[inline]
  pub const fn path_to_grost(&self) -> &Path {
    &self.path_to_grost
  }

  /// Returns the type of the object.
  ///
  /// e.g. If a struct is `struct MyObject<T> { ... }`, this will return `MyObject<T>`.
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the generics in the object definition.
  #[inline]
  pub const fn generics(&self) -> &Generics {
    &self.generics
  }

  /// Returns the fields of the object.
  #[inline]
  pub const fn fields(&self) -> &[GenericField<F>] {
    self.fields.as_slice()
  }

  /// Returns the flavor attributes in the object definition.
  #[inline]
  pub const fn flavors(&self) -> &IndexMap<Ident, ObjectFlavor> {
    &self.flavors
  }

  /// Returns the generic flavor type parameter
  pub const fn flavor_param(&self) -> &TypeParam {
    &self.flavor_param
  }

  /// Returns the generic unknown buffer type parameter
  pub const fn unknown_buffer_param(&self) -> &TypeParam {
    &self.unknown_buffer_param
  }

  /// Returns `true` if the object is copyable, `false` otherwise.
  #[inline]
  pub const fn copy(&self) -> bool {
    self.copy
  }

  /// Returns the partial object information.
  #[inline]
  pub const fn partial(&self) -> &PartialObject {
    &self.partial
  }

  /// Returns the partial decoded object information.
  #[inline]
  pub const fn partial_decoded(&self) -> &GenericPartialDecodedObject {
    &self.partial_decoded
  }

  /// Returns the selector information.
  #[inline]
  pub const fn selector(&self) -> &GenericSelector {
    &self.selector
  }

  /// Returns the selector iterator information.
  #[inline]
  pub const fn selector_iter(&self) -> &GenericSelectorIter {
    &self.selector_iter
  }

  fn try_new<O>(object: &O, flavor_param: TypeParam) -> darling::Result<Self>
  where
    O: RawObject<Meta = M>,
    O::Field: RawField<Meta = F>,
    F: Clone,
    M: Clone,
  {
    let path_to_grost = object.path_to_grost().clone();
    let attrs = object.attrs().to_vec();
    let name = object.name().clone();
    let vis = object.vis().clone();
    let generics = object.generics().clone();
    let (_, tg, _) = generics.split_for_impl();
    let type_params = generics.declared_type_params();
    let lifetimes = generics.declared_lifetimes();

    let ty: Type = syn::parse2(quote! {
      #name #tg
    })?;
    let reflectable: Type = syn::parse2(quote! {
      #path_to_grost::__private::reflection::Reflectable<#ty>
    })?;

    let flavors = object
      .flavors()
      .iter()
      .map(|f| {
        let name = f.name().clone();
        ObjectFlavor::from_attribute(f).map(|f| (name, f))
      })
      .collect::<darling::Result<IndexMap<_, _>>>()?;

    let mut tags = IndexSet::new();
    let fields = object
      .fields()
      .iter()
      .map(|&f| {
        if f.skip() {
          SkippedField::try_new(f, &type_params, &lifetimes)
            .map(|f| GenericField::Skipped(Box::new(f)))
        } else {
          GenericTaggedField::try_new(
            f,
            &flavor_param,
            &flavors,
            &type_params,
            &lifetimes,
            f.copy() || object.copy(),
          )
          .and_then(|f| {
            if tags.contains(&f.tag()) {
              Err(darling::Error::custom(format!(
                "{name} has multiple fields have the same tag {}",
                f.tag()
              )))
            } else {
              tags.insert(f.tag());
              Ok(GenericField::Tagged(Box::new(f)))
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
    let partial_decoded =
      GenericPartialDecodedObject::from_attribute(&name, object.partial_decoded())?;
    let selector = GenericSelector::from_attribute(&name, object.selector())?;
    let selector_iter =
      GenericSelectorIter::from_attribute(selector.name(), object.selector_iter())?;

    Ok(Self {
      path_to_grost,
      attrs,
      default: object.default().cloned(),
      indexer: Indexer {
        name: object.indexer_name().clone(),
        attrs: object.indexer().attrs().to_vec(),
      },
      schema_name: object
        .schema()
        .name()
        .map_or_else(|| name.to_string(), |s| s.to_string()),
      schema_description: object
        .schema()
        .description()
        .unwrap_or_default()
        .to_string(),
      name,
      vis,
      ty,
      flavor_param,
      lifetime_param: object.lifetime_param().clone(),
      unknown_buffer_param: object.unknown_buffer_type_param().clone(),
      wire_format_param: object.wire_format_type_param().clone(),
      reflectable,
      generics,
      flavors,
      fields,
      partial,
      partial_decoded,
      selector,
      selector_iter,
      copy: object.copy(),
      meta: object.meta().clone(),
    })
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
pub(super) enum Object<M, F> {
  Concrete(Box<ConcreteObject<M, F>>),
  Generic(Box<GenericObject<M, F>>),
}

impl<M, F> Object<M, F> {
  /// Creates an `Object` from a raw object input.
  pub fn from_raw<O>(object: &O) -> darling::Result<Self>
  where
    O: RawObject<Meta = M>,
    O::Field: RawField<Meta = F>,
    M: Clone,
    F: Clone,
  {
    let num_flavors = object.flavors().len();
    if object.flavor_type_param().is_none() && num_flavors == 1 {
      let flavor = object
        .flavors()
        .iter()
        .next()
        .expect("There must be a flavor were already checked");
      ConcreteObject::try_new(object, flavor).map(|obj| Object::Concrete(Box::new(obj)))
    } else {
      let flavor_param = object
        .flavor_type_param()
        .cloned()
        .unwrap_or_else(grost_flavor_param);
      GenericObject::try_new(object, flavor_param).map(|obj| Object::Generic(Box::new(obj)))
    }
  }
}

impl ObjectFromMeta {
  pub fn finalize(self, path_to_grost: Path) -> syn::Result<ObjectAttribute> {
    let flavors = self.flavor.finalize(&path_to_grost)?;
    let mut flavor_generic = self.generic.flavor;
    if flavors.len() > 1 {
      flavor_generic.get_or_insert_with(grost_flavor_param);
    }

    Ok(ObjectAttribute {
      path_to_grost,
      flavors,
      default: self.default,
      schema: self.schema.into(),
      partial: self.partial.finalize(),
      partial_decoded: self.partial_decoded.finalize(),
      selector: self.selector.finalize(),
      selector_iter: self.selector_iter.finalize(),
      flavor_param: flavor_generic,
      lifetime_param: self.generic.lifetime,
      unknown_buffer_param: self.generic.unknown_buffer,
      wire_format_param: self.generic.wire_format,
      indexer: self.indexer.into(),
      copy: self.copy,
    })
  }
}

#[derive(Debug, Clone)]
pub struct ObjectAttribute {
  path_to_grost: Path,
  flavors: Vec<FlavorAttribute>,
  default: Option<Invokable>,
  schema: SchemaAttribute,
  partial: PartialObjectAttribute,
  partial_decoded: PartialDecodedObjectAttribute,
  selector: SelectorAttribute,
  selector_iter: SelectorIterAttribute,
  indexer: IndexerAttribute,
  copy: bool,
  flavor_param: Option<TypeParam>,
  unknown_buffer_param: TypeParam,
  lifetime_param: LifetimeParam,
  wire_format_param: TypeParam,
}

impl ObjectAttribute {
  /// Returns the path to the `grost` crate
  pub const fn path_to_grost(&self) -> &Path {
    &self.path_to_grost
  }

  /// Returns the path to the fn that returns the default value of the object
  pub const fn default(&self) -> Option<&Invokable> {
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

  /// Returns the generic flavor type parameter
  pub const fn flavor_type_param(&self) -> Option<&TypeParam> {
    self.flavor_param.as_ref()
  }

  /// Returns the generic unknown buffer type parameter
  pub const fn unknown_buffer_type_param(&self) -> &TypeParam {
    &self.unknown_buffer_param
  }

  /// Returns the generic lifetime parameter
  pub const fn lifetime_param(&self) -> &LifetimeParam {
    &self.lifetime_param
  }

  /// Returns the generic wire format type parameter
  pub const fn wire_format_type_param(&self) -> &TypeParam {
    &self.wire_format_param
  }
}

/// The trait for the object derive input
pub trait RawObject: Clone {
  /// The type of the field
  type Field: RawField;
  /// The custom metadata type for the object
  type Meta: Clone;

  /// Returns the name of the object
  fn name(&self) -> &Ident;

  /// Returns the visibility of the object
  fn vis(&self) -> &Visibility;

  /// Returns the generics in the object defination.
  fn generics(&self) -> &Generics;

  /// Returns the attributes in the object defination.
  fn attrs(&self) -> &[Attribute];

  /// Returns the fields of the object
  fn fields(&self) -> Vec<&Self::Field>;

  /// Returns the path to the `grost` crate
  fn path_to_grost(&self) -> &Path;

  /// Returns the path to the fn that returns the default value of the object
  fn default(&self) -> Option<&Invokable>;

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

  /// Returns the metadata associated with the object
  fn meta(&self) -> &Self::Meta;

  /// Returns the flavors of the object
  fn flavors(&self) -> &[FlavorAttribute];

  /// Returns the generic flavor type parameter, if any,
  /// `None` if we only have one flavor and the code is not generic over the flavor type.
  fn flavor_type_param(&self) -> Option<&TypeParam>;

  /// Returns the generic unknown buffer type parameter
  fn unknown_buffer_type_param(&self) -> &TypeParam;

  /// Returns the generic lifetime parameter
  fn lifetime_param(&self) -> &LifetimeParam;

  /// Returns the generic wire format type parameter
  fn wire_format_type_param(&self) -> &TypeParam;
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
