use darling::FromMeta;
use quote::quote;

#[derive(Default, Debug)]
pub enum Selection {
  #[default]
  Default,
  All,
  None,
  Custom(syn::Path),
}

impl FromMeta for Selection {
  fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
    const HINTS: &str = "possible formats are [all, none, default, custom = \"path/to/const/fn\"]";

    if items.len() > 1 {
      return Err(darling::Error::too_many_items(1));
    }

    if let Some(item) = items.iter().next() {
      match item {
        darling::ast::NestedMeta::Lit(lit) => return Err(darling::Error::unexpected_lit_type(lit)),
        darling::ast::NestedMeta::Meta(meta) => match meta {
          syn::Meta::Path(path) => {
            return Ok(match () {
              () if path.is_ident("all") => Self::All,
              () if path.is_ident("none") => Self::None,
              () if path.is_ident("default") => Self::Default,
              () if path.is_ident("custom") => {
                return Err(darling::Error::custom(format!("unknown format, {HINTS}")));
              }
              _ => {
                if let Some(ident) = path.get_ident() {
                  return Err(darling::Error::custom(format!(
                    "unknown `{}`, {HINTS}",
                    ident
                  )));
                } else {
                  return Err(darling::Error::custom(format!("missing ident, {HINTS}")));
                }
              }
            });
          }
          syn::Meta::List(_) => {
            return Err(darling::Error::unsupported_shape("list"));
          }
          syn::Meta::NameValue(name_value) => {
            return if name_value.path.is_ident("custom") {
              let value = &name_value.value;
              let path_str: syn::LitStr = syn::parse2::<syn::LitStr>(quote!(#value))
                .map_err(|_| darling::Error::unexpected_expr_type(&name_value.value))?;
              let path = syn::parse_str::<syn::Path>(&path_str.value())
                .map_err(|_| darling::Error::unexpected_expr_type(&name_value.value))?;
              Ok(Self::Custom(path))
            } else {
              Err(darling::Error::custom(format!("unknown format, {HINTS}")))
            };
          }
        },
      }
    }

    Ok(Self::Default)
  }

  fn from_word() -> darling::Result<Self> {
    Ok(Self::Default)
  }

  fn from_string(value: &str) -> darling::Result<Self> {
    Ok(match value {
      "all" => Self::All,
      "none" => Self::None,
      "default" => Self::Default,
      value => {
        return Err(darling::Error::custom(format!(
          "unknown value `{value}`, possible values are [\"all\", \"none\", \"default\"]"
        )));
      }
    })
  }
}
