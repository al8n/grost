use grost_derive::Object;

#[derive(Debug, Clone, PartialEq, Eq, Object)]
pub struct User {
  #[grost(
    tag = 1,
    schema(description = "The nick name of the user"),
    wire = "grost::flavors::network::LengthDelimited",
    select = "all",
    partial_ref(copy,)
  )]
  name: String,
  #[grost(
    tag = 2,
    schema(description = "The age of the user"),
    wire = "grost::flavors::network::Varint",
    copy,
    partial_ref(copy)
  )]
  age: u8,
  #[grost(
    tag = 3,
    schema(description = "The email of the user"),
    wire = "grost::flavors::network::LengthDelimited",
    copy,
    partial_ref(copy, attrs(derive(Copy, Clone))),
    optional(repeated)
  )]
  emails: Option<Vec<String>>,
}

#[test]
fn t() {}
