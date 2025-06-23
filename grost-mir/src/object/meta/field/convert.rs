use darling::FromMeta;
use syn::Meta;

use crate::utils::{BoolOption, ConvertOperation, Invokable, MissingOperation, NestedMeta};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct FieldConvertFromMeta {
  pub(in crate::object) missing_operation: Option<MissingOperation>,
}

impl FromMeta for FieldConvertFromMeta {
  fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
    let missing_operation: Option<MissingOperation> =
      MissingOperation::parse_from_meta_list(items)?;

    Ok(Self { missing_operation })
  }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PartialFieldConvertFromMeta {
  pub(in crate::object) missing_operation: Option<MissingOperation>,
  pub(in crate::object) convert_operation: Option<ConvertOperation>,
}

impl FromMeta for PartialFieldConvertFromMeta {
  fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
    let missing_operation: Option<MissingOperation> =
      MissingOperation::parse_from_meta_list(items)?;
    let convert_operation = ConvertOperation::parse_from_meta_list(items)?;

    Ok(PartialFieldConvertFromMeta {
      missing_operation,
      convert_operation,
    })
  }
}

#[derive(Debug, Clone, PartialEq, Eq, FromMeta)]
pub enum FieldSkipEncodeOperation {
  #[darling(rename = "skip")]
  Default,
  #[darling(rename = "skip_if")]
  If(Invokable),
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct FieldEncodeFromMeta {
  pub(in crate::object) skip_operation: Option<FieldSkipEncodeOperation>,
  pub(in crate::object) error_if: Option<Invokable>,
}

impl FromMeta for FieldEncodeFromMeta {
  fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
    #[derive(Debug, FromMeta)]
    struct Helper {
      #[darling(default)]
      skip: BoolOption,
      #[darling(default)]
      skip_if: Option<Invokable>,
      #[darling(default)]
      error_if: Option<Invokable>,
    }

    let Helper {
      skip,
      skip_if,
      error_if,
    } = Helper::from_list(items)?;
    if skip.is_some() && skip_if.is_some() {
      return Err(darling::Error::custom(
        "Cannot specify both `skip` and `skip_if` at the same time.",
      ));
    }

    let skip_operation = if skip.is_some() {
      Some(FieldSkipEncodeOperation::Default)
    } else {
      skip_if.map(FieldSkipEncodeOperation::If)
    };

    Ok(FieldEncodeFromMeta {
      skip_operation,
      error_if,
    })
  }
}

#[derive(Debug, Default, Clone)]
pub struct FieldDecodeFromMeta {
  pub(in crate::object) func: Option<Invokable>,
  pub(in crate::object) then: Option<Invokable>,
  pub(in crate::object) missing_operation: Option<MissingOperation>,
}

impl FromMeta for FieldDecodeFromMeta {
  fn from_meta(item: &Meta) -> darling::Result<Self> {
    (match *item {
      Meta::Path(_) => Self::from_word(),
      Meta::NameValue(ref value) => Self::from_expr(&value.value),
      Meta::List(ref value) => Self::from_list(&NestedMeta::parse_meta_list(value.tokens.clone())?),
    })
    .map_err(|e| e.with_span(item))
  }

  fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
    let mut remaining_items = vec![];
    let mut missing_operation_items = vec![];

    items.iter().cloned().for_each(|item| match item {
      darling::ast::NestedMeta::Meta(ref meta)
        if meta.path().is_ident("or_else")
          || meta.path().is_ident("or_else_default")
          || meta.path().is_ident("ok_or_else") =>
      {
        missing_operation_items.push(item);
      }
      _ => remaining_items.push(item),
    });

    let missing_operation: Option<MissingOperation> =
      MissingOperation::parse_from_meta_list(&missing_operation_items)?;

    #[derive(Debug, Default, Clone, FromMeta)]
    struct Helper {
      #[darling(rename = "fn", default)]
      func: Option<Invokable>,
      #[darling(default)]
      then: Option<Invokable>,
    }

    let Helper { func, then } = Helper::from_list(&remaining_items)?;

    Ok(Self {
      missing_operation,
      func,
      then,
    })
  }
}
