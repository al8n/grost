use darling::FromMeta;
use quote::quote;
use syn::{Attribute, parse::Parser};

pub use object::*;

mod object;

fn from_optional_string<T>(s: Option<String>) -> darling::Result<Option<T>>
where
  T: syn::parse::Parse,
{
  match s {
    Some(s) => syn::parse_str(&s).map(Some).map_err(Into::into),
    None => Ok(None),
  }
}

#[derive(Debug, Default)]
struct Attributes(Vec<Attribute>);

impl From<Attributes> for Vec<Attribute> {
  fn from(attrs: Attributes) -> Self {
    attrs.0
  }
}

impl FromMeta for Attributes {
  fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
    let mut attributes = Vec::new();
    for item in items {
      match item {
        darling::ast::NestedMeta::Lit(lit) => {
          return Err(darling::Error::unexpected_lit_type(lit));
        }
        darling::ast::NestedMeta::Meta(meta) => {
          let attr = Attribute::parse_outer
            .parse2(quote! { #[#meta] })
            .map_err(|e| darling::Error::from(e).with_span(meta))?;
          attributes.extend(attr);
        }
      }
    }

    Ok(Self(attributes))
  }
}
