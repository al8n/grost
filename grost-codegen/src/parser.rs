use darling::FromMeta;
use quote::{ToTokens, quote};
use syn::{Attribute, parse::Parser, parse_quote};

pub use object::*;

mod object;

#[derive(Debug, Default, Clone)]
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

fn wire_format_reflection_ty(
  path_to_grost: &syn::Path,
  field_reflection: impl ToTokens,
  tag: u32,
  flavor_generic: impl ToTokens,
) -> syn::Type {
  parse_quote! {
    #field_reflection<
      #path_to_grost::__private::reflection::WireFormatReflection,
      #flavor_generic,
      #tag,
    >
  }
}
