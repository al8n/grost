use darling::FromMeta;

#[derive(Debug, Clone, Copy, darling::FromMeta)]
#[non_exhaustive]
pub enum Ty {
  Scalar,
  Bytes,
  String,
  Object,
  Enum,
  Union,
  Interface,
}

#[derive(Debug, Clone, darling::FromMeta)]
struct WireHelper {
  flavor: syn::Ident,
  #[darling(default)]
  format: Option<syn::Type>,
}

#[derive(Debug, Clone)]
pub(super) struct Wire {
  flavor: Option<syn::Ident>,
  format: Option<syn::Type>,
}

impl darling::FromMeta for Wire {
  fn from_string(value: &str) -> darling::Result<Self> {
    let format = syn::parse_str(value)?;
    Ok(Self {
      flavor: None,
      format: Some(format),
    })
  }

  fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
    WireHelper::from_list(items).map(|helper| Self {
      flavor: Some(helper.flavor),
      format: helper.format,
    })
  }
}

#[test]
fn test_default_wire() {
  use quote::quote;

  let src = quote! {
    wire = "grost::flavors::network::LengthDelimited"
  };

  let wire: Wire = Wire::from_meta(&syn::parse2(src).unwrap()).unwrap();
  assert!(wire.flavor.is_none());
  assert_eq!(
    wire.format,
    Some(syn::parse_str("grost::flavors::network::LengthDelimited").unwrap())
  );
}

#[test]
fn test_wire() {
  use quote::quote;

  let src = quote! {
    wire(
      flavor = network,
      format = "grost::flavors::network::LengthDelimited",
      identifier(
        new = "grost::flavors::network::Identifier::new",
        encode = "grost::flavors::network::Identifier::encode",
      ),
    )
  };

  let wire: Wire = Wire::from_meta(&syn::parse2(src).unwrap()).unwrap();
  assert_eq!(wire.flavor, Some(syn::parse_str("network").unwrap()));
  assert_eq!(
    wire.format,
    Some(syn::parse_str("grost::flavors::network::LengthDelimited").unwrap())
  );
}
