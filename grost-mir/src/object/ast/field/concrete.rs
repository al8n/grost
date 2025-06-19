use darling::usage::{IdentSet, LifetimeSet, Purpose, UsesLifetimes, UsesTypeParams};
use quote::quote;
use syn::{Attribute, Ident, Path, Type, Visibility};

use crate::{
  flavor::FlavorAttribute,
  object::{
    FieldDecodeAttribute, FieldEncodeAttribute, FieldFlavorAttribute, Label,
    PartialDecodedFieldAttribute, PartialFieldAttribute, RawField, SelectorFieldAttribute,
    ast::{
      SkippedField,
      field::flavor::{FieldDecodeFlavor, FieldEncodeFlavor, FieldFlavor},
    },
  },
  utils::{Invokable, MissingOperation},
};

#[derive(Debug, Clone)]
pub(in crate::object) struct ConcreteTaggedField<M = ()> {
  pub(in crate::object) attrs: Vec<Attribute>,
  pub(in crate::object) vis: Visibility,
  pub(in crate::object) name: Ident,
  pub(in crate::object) ty: Type,
  pub(in crate::object) schema_name: String,
  pub(in crate::object) schema_description: String,
  pub(in crate::object) flavor: FieldFlavor,
  pub(in crate::object) label: Label,
  pub(in crate::object) type_params_usages: IdentSet,
  pub(in crate::object) lifetime_params_usages: LifetimeSet,
  pub(in crate::object) partial_decoded: PartialDecodedFieldAttribute,
  pub(in crate::object) partial: PartialFieldAttribute,
  pub(in crate::object) selector: SelectorFieldAttribute,
  pub(in crate::object) default: Invokable,
  pub(in crate::object) tag: u32,
  pub(in crate::object) copy: bool,
  pub(in crate::object) meta: M,
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

  pub(in crate::object) fn try_new<F: RawField<Meta = M>>(
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

    macro_rules! bail {
      ($flavor:ident($label:ident)) => {{
        match $label {
          Label::Scalar => bail!(@$flavor(skip_default_scalar, or_else_default_scalar)),
          Label::Bytes => bail!(@$flavor(skip_default_bytes, or_else_default_bytes)),
          Label::String => bail!(@$flavor(skip_default_string, or_else_default_string)),
          Label::Object => bail!(@$flavor(skip_default_object, or_else_default_object)),
          Label::Enum => bail!(@$flavor(skip_default_enumeration, or_else_default_enumeration)),
          Label::Union => bail!(@$flavor(skip_default_union, or_else_default_union)),
          Label::Interface => bail!(@$flavor(skip_default_interface, or_else_default_interface)),
          Label::Map { .. } => bail!(@$flavor(skip_default_map, or_else_default_map)),
          Label::List(_) => bail!(@$flavor(skip_default_list, or_else_default_list)),
          Label::Set(_) => bail!(@$flavor(skip_default_set, or_else_default_set)),
          _ => (true, None),
        }}
      };
      (@$ty:ident($skip:ident, $or_else:ident)) => {{
        let skip_default = $ty.encode().$skip();
        let missing_operation = if $ty.decode().$or_else() {
          Some(MissingOperation::OrDefault(None))
        } else {
          None
        };
        (skip_default, missing_operation)
      }};
    }

    let label = f
      .label()
      .ok_or_else(|| darling::Error::custom(format!("field `{name}` is missing label")))?;
    let (skip_default, missing_operation) = bail!(flavor(label));

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
      FieldFlavorAttribute::new(
        flavor.name().clone(),
        None,
        None,
        FieldEncodeAttribute::new(Some(skip_default), None, None),
        FieldDecodeAttribute::new(missing_operation.clone(), None),
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
        encode: FieldEncodeFlavor {
          skip_default: field_flavor.encode.skip_default.unwrap_or(skip_default),
          skip_if: field_flavor.encode.skip_if.clone(),
          error_if: field_flavor.encode.error_if.clone(),
        },
        decode: FieldDecodeFlavor {
          missing_operation: field_flavor
            .decode
            .missing_operation
            .clone()
            .or(missing_operation),
          then: field_flavor.decode.then.clone(),
        },
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
pub(in crate::object) enum ConcreteField<M = ()> {
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
