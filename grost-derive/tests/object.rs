use grost::{
  convert::Decoded,
  flavors::{
    Network, RawTag,
    network::{Identifier, LengthDelimited, Tag, WireType},
  },
  reflection::{Identified, Reflectable, Reflection},
};
use grost_derive::Object;

mod sealed {
  pub fn default_user() -> String {
    String::from("user")
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Object)]
pub struct User<I> {
  #[grost(
    tag = 1,
    schema(description = "The id of the user"),
    wire = "grost::flavors::network::LengthDelimited",
    selector(select = "all"),
    partial_decoded(copy,)
  )]
  id: I,
  #[grost(
    tag = 2,
    schema(description = "The nick name of the user"),
    wire = "grost::flavors::network::LengthDelimited",
    selector(select = "all"),
    partial_decoded(copy,),
    default = sealed::default_user,
  )]
  name: String,
  #[grost(
    tag = 3,
    schema(description = "The age of the user"),
    wire = "grost::flavors::network::Varint",
    copy,
    partial_decoded(copy)
  )]
  age: u8,
  #[grost(
    tag = 4,
    schema(description = "The email of the user"),
    wire = "grost::flavors::network::LengthDelimited",
    partial_decoded(copy),
    optional(repeated)
  )]
  emails: Option<Vec<String>>,
}

// #[derive(Debug, Clone, PartialEq, Eq, Object)]
// pub struct Comment<I> {
//   #[grost(
//     tag = 1,
//     schema(description = "The id of the comment"),
//     wire = "grost::flavors::network::LengthDelimited",
//     selector(select = "all"),
//     partial_decoded(copy,)
//   )]
//   id: I,
//   #[grost(
//     tag = 2,
//     schema(description = "The user who made the comment"),
//     wire = "grost::flavors::network::LengthDelimited",
//     partial_decoded(copy)
//   )]
//   user: User<I>,
//   #[grost(
//     tag = 2,
//     schema(description = "The replyers who reply the comment"),
//     wire = "grost::flavors::network::Repeated<grost::flavors::network::LengthDelimited>",
//     partial_decoded(copy),
//     repeated
//   )]
//   replyers: Vec<User<I>>,
//   #[grost(
//     tag = 4,
//     schema(description = "The content of the comment"),
//     wire = "grost::flavors::network::LengthDelimited",
//     partial_decoded(copy)
//   )]
//   content: String,
// }

// impl Reflectable1<User> for Reflection<Identified<User, 1>, Identifier, Network> {
//   type Reflection = Identifier;

//   const REFLECTION: &Self::Reflection = &Identifier::new(WireType::LengthDelimited, Tag::new(1));
// }
// impl Reflectable1<User> for Reflection<Identified<User, 1>, Tag, Network> {
//   type Reflection = Tag;

//   const REFLECTION: &Self::Reflection = &Tag::new(1);
// }
// impl Reflectable1<User> for Reflection<Identified<User, 1>, Tag, ()> {
//   type Reflection = Tag;

//   const REFLECTION: &Self::Reflection = &Tag::new(1);
// }

// impl<'a> Reflectable1<User> for Reflection<Identified<User, 1>, Decoded<'a, Network, LengthDelimited>, Network> {
//   type Reflection = Tag;

//   const REFLECTION: &'static Self::Reflection = &Tag::new(1);
// }

#[test]
fn t() {
  // let user = PartialDecodedUser {
  //   age: Some(1),
  //   name: Some("user".to_string()),
  //   emails: None,
  // };
  // println!("{:?}", <grost::reflection::SchemaTypeReflection<Option<Vec<Option<String>>>> as grost::reflection::Reflectable<Network>>::REFLECTION);
}
