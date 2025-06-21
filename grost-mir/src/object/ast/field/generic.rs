use darling::usage::{IdentSet, LifetimeSet, Purpose, UsesLifetimes, UsesTypeParams};
use indexmap::IndexMap;
use quote::quote;
use syn::{Attribute, Ident, Path, Type, TypeParam, Visibility};

use crate::{
  object::{
    FieldSelection, Label, ObjectFlavor, PartialDecodedFieldOptions, PartialFieldOptions, RawField,
    SelectorFieldOptions,
    ast::{
      SkippedField,
      field::flavor::{FieldDecodeFlavor, FieldEncodeFlavor, FieldFlavor},
    },
  },
  utils::{Invokable, MissingOperation},
};

#[derive(Debug, Clone)]
pub(in crate::object) struct GenericTaggedField<M = ()> {
  pub(in crate::object) attrs: Vec<Attribute>,
  pub(in crate::object) vis: Visibility,
  pub(in crate::object) name: Ident,
  pub(in crate::object) ty: Type,
  pub(in crate::object) schema_name: String,
  pub(in crate::object) schema_description: String,
  pub(in crate::object) type_params_usages: IdentSet,
  pub(in crate::object) lifetime_params_usages: LifetimeSet,
  pub(in crate::object) flavor_param: TypeParam,
  pub(in crate::object) label: Label,
  pub(in crate::object) partial_decoded: PartialDecodedFieldOptions,
  pub(in crate::object) partial: PartialFieldOptions,
  pub(in crate::object) selector: SelectorFieldOptions,
  pub(in crate::object) default: Invokable,
  pub(in crate::object) tag: u32,
  pub(in crate::object) flavors: IndexMap<Ident, FieldFlavor>,
  pub(in crate::object) copy: bool,
  pub(in crate::object) meta: M,
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

  pub(in crate::object) fn try_new<F: RawField<Meta = M>>(
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
          _ => (false, None),
        };

        let field_flavor = match f.flavors().iter().find(|ff| ff.name().eq(name)) {
          Some(ff) => FieldFlavor {
            ty: ff.ty().cloned(),
            format: ff.format().cloned(),
            flavor_type: flavor.ty().clone(),
            encode: FieldEncodeFlavor {
              skip_default: ff.encode.skip_default().unwrap_or(skip_default),
              skip_if: ff.encode.skip_if.clone(),
              error_if: ff.encode.error_if.clone(),
            },
            decode: FieldDecodeFlavor {
              missing_operation: ff.decode.missing_operation.clone().or(missing_operation),
              then: ff.decode.then.clone(),
            },
            convert: ff.convert.clone(),
          },
          None => FieldFlavor {
            ty: None,
            format: None,
            flavor_type: flavor.ty().clone(),
            encode: FieldEncodeFlavor {
              skip_default,
              skip_if: None,
              error_if: None,
            },
            decode: FieldDecodeFlavor {
              missing_operation,
              then: None,
            },
            convert: Default::default(),
          },
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
pub(in crate::object) enum GenericField<M = ()> {
  Skipped(Box<SkippedField<M>>),
  Tagged(Box<GenericTaggedField<M>>),
}

impl<M> GenericField<M> {
  pub const fn tag(&self) -> Option<u32> {
    match self {
      GenericField::Skipped(_) => None,
      GenericField::Tagged(tagged) => Some(tagged.tag()),
    }
  }
}
