use darling::{FromMeta, ast::NestedMeta};

use crate::utils::{Invokable, MissingOperation};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(in crate::object) struct DecodeFromMeta {
  pub(in crate::object) missing_operation: Option<MissingOperation>,
  pub(in crate::object) then: Option<Invokable>,
}

impl DecodeFromMeta {
  /// Check if an or_else variant is already set and return appropriate error
  fn check_or_else_conflict(
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
}

impl FromMeta for DecodeFromMeta {
  fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
    let mut missing_operation: Option<MissingOperation> = None;
    let mut then = None;

    for item in items {
      match item {
        NestedMeta::Lit(_) => return Err(darling::Error::unsupported_format("literal")),
        NestedMeta::Meta(meta) => {
          let path = meta.path();

          if path.is_ident("or_else") {
            Self::check_or_else_conflict(missing_operation.as_ref(), "or_else")?;

            let nv = meta.require_name_value()?;
            let invokable = Invokable::try_from(&nv.value)?;
            missing_operation = Some(MissingOperation::Or(invokable));
          } else if path.is_ident("or_else_default") {
            Self::check_or_else_conflict(missing_operation.as_ref(), "or_else_default")?;

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
            Self::check_or_else_conflict(missing_operation.as_ref(), "ok_or_else")?;

            let nv = meta.require_name_value()?;
            let invokable = Invokable::try_from(&nv.value)?;
            missing_operation = Some(MissingOperation::OkOr(invokable));
          } else if path.is_ident("then") {
            if then.is_some() {
              return Err(darling::Error::duplicate_field("then"));
            }

            let nv = meta.require_name_value()?;
            then = Some(Invokable::try_from(&nv.value)?);
          } else {
            return Err(darling::Error::unknown_field_path(path));
          }
        }
      }
    }

    Ok(DecodeFromMeta {
      missing_operation,
      then,
    })
  }
}

#[cfg(test)]
mod tests {
  use quote::quote;
  use syn::parse::{Parse, Parser};

  use super::*;

  #[test]
  fn test_decode_from_meta() {
    let meta = syn::parse_quote! {
      decode(
        or_else_default,
        then = "my_crate::error_function"
      )
    };
    let decode: DecodeFromMeta = FromMeta::from_meta(&meta).unwrap();

    assert_eq!(
      decode.missing_operation,
      Some(MissingOperation::OrDefault(None))
    );
    assert_eq!(
      decode.then,
      Some(Invokable::from(
        syn::Path::parse
          .parse2(quote! {my_crate::error_function})
          .unwrap()
      ))
    );
  }

  #[test]
  fn test_decode_from_meta2() {
    let meta = syn::parse_quote! {
      decode(
        or_else_default = "my_crate::default_function",
        then = "my_crate::error_function"
      )
    };
    let decode: DecodeFromMeta = FromMeta::from_meta(&meta).unwrap();

    assert_eq!(
      decode.missing_operation,
      Some(MissingOperation::OrDefault(Some(
        syn::Path::parse
          .parse2(quote! {my_crate::default_function})
          .unwrap()
          .into()
      )))
    );
    assert_eq!(
      decode.then,
      Some(Invokable::from(
        syn::Path::parse
          .parse2(quote! {my_crate::error_function})
          .unwrap()
      ))
    );
  }

  #[test]
  fn test_conflicting_or_else_variants() {
    let meta = syn::parse_quote! {
      decode(
        or_else = "some_fn",
        or_else_default
      )
    };
    let result: darling::Result<DecodeFromMeta> = FromMeta::from_meta(&meta);

    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("Cannot specify both"));
  }

  #[test]
  fn test_duplicate_error_if() {
    let meta = syn::parse_quote! {
      decode(
        error_if = "fn1",
        error_if = "fn2"
      )
    };
    let result: darling::Result<DecodeFromMeta> = FromMeta::from_meta(&meta);

    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("Duplicate"));
  }

  #[test]
  fn test_unknown_field() {
    let meta = syn::parse_quote! {
      decode(unknown_field = "value")
    };
    let result: darling::Result<DecodeFromMeta> = FromMeta::from_meta(&meta);

    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("Unknown field"));
  }
}
