use std::marker::PhantomData;

use grost::{
  Decode, PartialDecode,
  buffer::Buf,
  flavors::{Network, network::LengthDelimited},
  selection::{Selectable, Selector},
};
use grost_derive::{Object, object};

// mod sealed {
//   pub fn default_user() -> String {
//     String::from("user")
//   }
// }

// #[object(output(path = "grost-derive/tests/user.rs", format))]
// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct User<I>
// {
//   #[grost(
//     tag = 1,
//     schema(description = "The id of the user"),
//     selector(select = "all"),
//     wire = "grost::flavors::network::LengthDelimited",
//     partial_decoded(copy,)
//   )]
//   id: I,
//   #[grost(
//     tag = 2,
//     schema(description = "The nick name of the user"),
//     selector(select = "all"),
//     partial_decoded(copy,)
//   )]
//   name: String,
//   #[grost(
//     tag = 3,
//     schema(description = "The age of the user"),
//     copy,
//     partial_decoded(copy)
//   )]
//   age: u8,
//   // #[grost(
//   //   tag = 4,
//   //   schema(description = "The email of the user"),
//   //   wire = "grost::flavors::network::LengthDelimited",
//   //   partial_decoded(copy),
//   //   optional(repeated)
//   // )]
//   // emails: Option<Vec<String>>,

//   #[grost(skip)]
//   _w: core::marker::PhantomData<*const ()>,
// }

// impl<'de, UB> Selectable<Network, LengthDelimited> for PartialDecodedUser<'de, Network, UB> {
//   type Selector = UserSelector<Network>;
// }

// impl<'de, UB> PartialDecode<'de, Network, LengthDelimited, Self, UB> for PartialDecodedUser<'de, String, Network, UB> {
//   fn partial_decode<B>(
//     context: &<Network as grost::Flavor>::Context,
//     src: B,
//     selector: &Self::Selector,
//   ) -> Result<(usize, Option<Self>), <Network as grost::Flavor>::Error>
//   where
//     Self: Sized + 'de,
//     B: Buf<'de>,
//     UB: grost::buffer::Buffer<<Network as grost::Flavor>::Unknown<B>> + 'de
//   {
//     if selector.is_empty() {
//       return Ok((0, None));
//     }

//     let bytes = src.as_bytes();
//     let mut this = Self::new();

//     let mut offset = 0;
//     let buf_len = bytes.len();
//     while offset < buf_len {

//     }

//     Ok((offset, Some(this)))
//   }

//     fn skip<B>(context: &<Network as grost::Flavor>::Context, src: B) -> Result<usize, <Network as grost::Flavor>::Error>
//     where
//       Self: Sized + 'de,
//       B: Buf<'de>
//     {
//       todo!()
//     }
//   // fn decode<B>(context: &<Network as grost::Flavor>::Context, src: B) -> Result<(usize, Self), <Network as grost::Flavor>::Error>
//   // where
//   //   Self: Sized + 'de,
//   //   B: Buf + 'de,
//   //   UB: grost::buffer::Buffer<<Network as grost::Flavor>::Unknown<B>> + 'de {
//   //   todo!()
//   // }
// }

// fn assert_<F, W, T: DecodeOwned1<F, W, T, ()>>()
// where
//   F: grost::Flavor,
//   W: grost::flavors::WireFormat<F>,
// {
//   // This is a test to ensure that the trait bounds are correct.
//   // If the trait bounds are not correct, this test will fail to compile.
// }

// #[test]
// fn t() {
//   assert_::<_, _, PartialDecodedUser<'static, String, Network, _>>();
// }

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
//     tag = 3,
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

#[test]
fn t() {
  // use grost::flavors::Network;

  // let refl = Comment::<String>::user_reflection::<Network>();
  // println!("{:?}", refl.identifier().encoded());
  // println!("{:?}", refl.tag().encoded());
  // println!("{:?}", refl.wire_type());
  // println!("{:?}", refl.wire_format());
  // let user = PartialDecodedUser {
  //   age: Some(1),
  //   name: Some("user".to_string()),
  //   emails: None,
  // };
  // println!("{:?}", <grost::reflection::SchemaTypeReflection<Option<Vec<Option<String>>>> as grost::reflection::Reflectable<Network>>::REFLECTION);
}
