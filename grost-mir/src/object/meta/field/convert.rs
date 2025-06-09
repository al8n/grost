use darling::{FromMeta, ast::NestedMeta};

use crate::utils::{MissingOperation, TransformOperation};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(in crate::object) struct ConvertFromMeta {
  pub(in crate::object) missing_operation: Option<MissingOperation>,
  pub(in crate::object) transform_operation: Option<TransformOperation>,
}

impl ConvertFromMeta {
  /// Parse a path from either a syn::Expr::Path or syn::Expr::Lit(syn::Lit::Str)
  fn parse_path_from_expr(expr: &syn::Expr) -> darling::Result<syn::Path> {
    match expr {
      syn::Expr::Path(path) => Ok(path.path.clone()),
      syn::Expr::Lit(lit) => match &lit.lit {
        syn::Lit::Str(lit_str) => syn::parse_str(&lit_str.value()).map_err(darling::Error::from),
        lit => Err(darling::Error::unexpected_lit_type(lit)),
      },
      value => Err(darling::Error::unexpected_expr_type(value)),
    }
  }

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
            let path = Self::parse_path_from_expr(&nv.value)?;
            missing_operation = Some(MissingOperation::Or(path));
          } else if path.is_ident("or_else_default") {
            Self::check_missing_operation_conflict(missing_operation.as_ref(), "or_else_default")?;

            match meta {
              syn::Meta::List(_) => return Err(darling::Error::unsupported_format("list")),
              syn::Meta::Path(_) => {
                missing_operation = Some(MissingOperation::OrDefault(None));
              }
              syn::Meta::NameValue(meta_name_value) => {
                let path = Self::parse_path_from_expr(&meta_name_value.value)?;
                missing_operation = Some(MissingOperation::OrDefault(Some(path)));
              }
            }
          } else if path.is_ident("ok_or_else") {
            Self::check_missing_operation_conflict(missing_operation.as_ref(), "ok_or_else")?;
            let nv = meta.require_name_value()?;
            let path = Self::parse_path_from_expr(&nv.value)?;
            missing_operation = Some(MissingOperation::OkOr(path));
          } else if path.is_ident("transform") {
            Self::check_transform_operation_conflict(transform_operation.as_ref(), "transform")?;
            let nv = meta.require_name_value()?;
            transform_operation = Some(TransformOperation::Into(Self::parse_path_from_expr(
              &nv.value,
            )?));
          } else if path.is_ident("try_transform") {
            Self::check_transform_operation_conflict(transform_operation.as_ref(), "transform")?;
            let nv = meta.require_name_value()?;
            transform_operation = Some(TransformOperation::TryInto(Self::parse_path_from_expr(
              &nv.value,
            )?));
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
