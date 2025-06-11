use darling::{FromMeta, ast::NestedMeta};

use crate::utils::{Invokable, MissingOperation, TransformOperation};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ConvertFromMeta {
  pub(in crate::object) missing_operation: Option<MissingOperation>,
  pub(in crate::object) transform_operation: Option<TransformOperation>,
}

impl ConvertFromMeta {
  /// Check if an or_else variant is already set and return appropriate error
  fn check_missing_operation_conflict(
    current: Option<&MissingOperation>,
    new_variant: &str,
  ) -> darling::Result<()> {
    if let Some(existing) = current {
      return Err(darling::Error::custom(format!(
        "Cannot specify both `{}` and `{}` at the same time.",
        existing.name(),
        new_variant
      )));
    }
    Ok(())
  }

  /// Check if an or_else variant is already set and return appropriate error
  fn check_transform_operation_conflict(
    current: Option<&TransformOperation>,
    new_variant: &str,
  ) -> darling::Result<()> {
    if let Some(existing) = current {
      return Err(darling::Error::custom(format!(
        "Cannot specify both `{}` and `{}` at the same time.",
        existing.name(),
        new_variant
      )));
    }
    Ok(())
  }
}

impl FromMeta for ConvertFromMeta {
  fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
    let mut missing_operation: Option<MissingOperation> = None;
    let mut transform_operation = None;

    for item in items {
      match item {
        NestedMeta::Lit(_) => return Err(darling::Error::unsupported_format("literal")),
        NestedMeta::Meta(meta) => {
          let path = meta.path();

          if path.is_ident("or_else") {
            Self::check_missing_operation_conflict(missing_operation.as_ref(), "or_else")?;
            let nv = meta.require_name_value()?;
            let invokable = Invokable::try_from(&nv.value)?;
            missing_operation = Some(MissingOperation::Or(invokable));
          } else if path.is_ident("or_else_default") {
            Self::check_missing_operation_conflict(missing_operation.as_ref(), "or_else_default")?;

            match meta {
              syn::Meta::List(_) => return Err(darling::Error::unsupported_format("list")),
              syn::Meta::Path(_) => {
                missing_operation = Some(MissingOperation::OrDefault(None));
              }
              syn::Meta::NameValue(meta_name_value) => {
                let invokable = Invokable::try_from(&meta_name_value.value)?;
                missing_operation = Some(MissingOperation::OrDefault(Some(invokable)));
              }
            }
          } else if path.is_ident("ok_or_else") {
            Self::check_missing_operation_conflict(missing_operation.as_ref(), "ok_or_else")?;
            let nv = meta.require_name_value()?;
            let invokable = Invokable::try_from(&nv.value)?;
            missing_operation = Some(MissingOperation::OkOr(invokable));
          } else if path.is_ident("transform") {
            Self::check_transform_operation_conflict(transform_operation.as_ref(), "transform")?;
            let nv = meta.require_name_value()?;
            transform_operation = Some(TransformOperation::Into(Invokable::try_from(&nv.value)?));
          } else if path.is_ident("try_transform") {
            Self::check_transform_operation_conflict(transform_operation.as_ref(), "transform")?;
            let nv = meta.require_name_value()?;
            transform_operation =
              Some(TransformOperation::TryInto(Invokable::try_from(&nv.value)?));
          } else {
            return Err(darling::Error::unknown_field_path(path));
          }
        }
      }
    }

    Ok(ConvertFromMeta {
      missing_operation,
      transform_operation,
    })
  }
}
