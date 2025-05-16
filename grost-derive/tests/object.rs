use grost_derive::Object;

mod sealed {
  pub fn default_user() -> String {
    String::from("user")
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Object)]
pub struct User<I: Sized> {
  #[grost(
    tag = 4,
    schema(description = "The id of the user"),
    wire = "grost::flavors::network::LengthDelimited",
    selector(select = "all"),
    partial_ref(copy,),
    default = sealed::default_user,
  )]
  id: I,

  #[grost(
    tag = 1,
    schema(description = "The nick name of the user"),
    wire = "grost::flavors::network::LengthDelimited",
    selector(select = "all"),
    partial_ref(copy,),
    default = sealed::default_user,
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
    partial_ref(copy),
    optional(repeated)
  )]
  emails: Option<Vec<String>>,
}

#[test]
fn t() {
  // let user = PartialUser {
  //   age: Some(1),
  //   name: Some("user".to_string()),
  //   emails: None,
  // };
}
