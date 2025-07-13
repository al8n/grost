use darling::FromMeta;

use crate::utils::{BoolOption, ConvertOperation, Invokable, MissingOperation};

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
  pub(in crate::object) convert_operation: Option<ConvertOperation>,
}

impl FromMeta for PartialFieldConvertFromMeta {
  fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
    let convert_operation = ConvertOperation::parse_from_meta_list(items)?;

    Ok(PartialFieldConvertFromMeta {
      convert_operation,
    })
  }
}

#[derive(Debug, Clone, PartialEq, Eq, FromMeta)]
pub enum FieldSkipEncodeOperation {
  #[darling(rename = "skip")]
  Always,
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
      Some(FieldSkipEncodeOperation::Always)
    } else {
      skip_if.map(FieldSkipEncodeOperation::If)
    };

    Ok(FieldEncodeFromMeta {
      skip_operation,
      error_if,
    })
  }
}

#[derive(Debug, Default, Clone, FromMeta)]
pub struct FieldDecodeFromMeta {
  #[darling(default, rename = "fn")]
  pub(in crate::object) func: Option<Invokable>,
  #[darling(default)]
  pub(in crate::object) then: Option<Invokable>,
}
