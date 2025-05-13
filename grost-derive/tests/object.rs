use grost_derive::object;

#[object]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
  #[doc = concat!("asd", b, "a")]
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
    partial_ref(copy),
    optional(repeated)
  )]
  age: u8,
  #[grost(
    tag = 3,
    schema(description = "The email of the user"),
    wire = "grost::flavors::network::LengthDelimited",
    copy,
    partial_ref(copy),
    optional(repeated(map(value(optional)))),
    repeated(map(value(optional)))
  )]
  emails: Option<Vec<String>>,
}

#[test]
fn t() {}
