use std::marker::PhantomData;

use grost::{
  Decode, Flavor,
  flavors::{DefaultWireFormat, Groto, WireFormat, groto::LengthDelimited},
  selection::{Selectable, Selector},
};
use grost_derive::{Object, object};

// mod sealed {
//   pub fn default_user() -> String {
//     String::from("user")
//   }
// }

fn default_array<const N: usize>() -> [u8; N] {
  [0; N]
}

fn error_name<'a>() -> Result<&'a str, <Groto as Flavor>::Error> {
  Ok("name")
}

// transform(
//   default = ,
//   from = ,
//   try_from = ,
// ),
// flavor(default(
//   partial_transform(
//     default = ,
//     from = ,
//     try_from = ,
//     or_default = ,
//     or_else = ,
//   ),
//   decode(
//     fn = ,
//     then = ,
//     or_default = ,
//     or_else = ,
//   ),
// )),

#[derive(Debug, Clone, PartialEq, Eq, Object)]
#[grost(
  // output(path = "grost-derive/tests/user.rs", format),
  // flavor(
  //   default(
  //     encode(
  //       skip_default,
  //       enum = "grost::flavors::groto::encoding::enumeration",
  //     )
  //   ),
  // ),
)]
pub struct User<I: Default> {
  #[grost(
    tag = 1,
    schema(description = "The id of the user"),
    selector(select = "all"),
    wire_format = "grost::flavors::groto::LengthDelimited",
    bytes
  )]
  id: I,
  #[grost(
    tag = 2,
    schema(description = "The nick name of the user"),
    selector(select = "all"),
    string
  )]
  name: String,
  #[grost(
    tag = 3,
    scalar,
    schema(description = "The age of the user"),
    copy,
    partial_ref(type = "u8")
  )]
  age: u8,
  #[grost(tag = 4, schema(description = "The email of the user"), list(string))]
  emails: Vec<String>,
  #[grost(
    tag = 5,
    schema(description = "The linkin link of the user"),
    optional(string)
  )]
  linkin: Option<String>,
  // #[grost(skip)]
  // what: W,
  // #[grost(skip, default = "default_array::<N>")]
  // array: [u8; N],
}

// impl<'de, B, UB> Selectable<Groto, LengthDelimited> for PartialRefUser<'de, Groto, UB> {
//   type Selector = UserSelector<Groto>;
// }

// impl<'de, B, UB> PartialDecode<'de, Groto, LengthDelimited, Self, B, UB> for PartialRefUser<'de, String, Groto, UB> {
//   fn partial_decode(
//     context: &<Groto as grost::Flavor>::Context,
//     src: B,
//     selector: &Self::Selector,
//   ) -> Result<(usize, Option<Self>), <Groto as grost::Flavor>::Error>
//   where
//     Self: Sized + 'de,
//     B: ReadBuf,
//     UB: grost::buffer::Buffer<<Groto as grost::Flavor>::Unknown<B>> + 'de
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

//     fn skip<B>(context: &<Groto as grost::Flavor>::Context, src: B) -> Result<usize, <Groto as grost::Flavor>::Error>
//     where
//       Self: Sized + 'de,
//       B: ReadBuf
//     {
//       todo!()
//     }
//   // fn decode(context: &<Groto as grost::Flavor>::Context, src: B) -> Result<(usize, Self), <Groto as grost::Flavor>::Error>
//   // where
//   //   Self: Sized + 'de,
//   //   B: Buf + 'de,
//   //   UB: grost::buffer::Buffer<<Groto as grost::Flavor>::Unknown<B>> + 'de {
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
//   assert_::<_, _, PartialRefUser<'static, String, Groto, _>>();
// }

// #[derive(Debug, Clone, PartialEq, Eq, Object)]
// pub struct Comment<I> {
//   #[grost(
//     tag = 1,
//     schema(description = "The id of the comment"),
//     wire = "grost::flavors::groto::LengthDelimited",
//     selector(select = "all"),
//     partial_ref(copy,)
//   )]
//   id: I,
//   #[grost(
//     tag = 2,
//     schema(description = "The user who made the comment"),
//     wire = "grost::flavors::groto::LengthDelimited",
//     partial_ref(copy)
//   )]
//   user: User<I>,
//   #[grost(
//     tag = 3,
//     schema(description = "The replyers who reply the comment"),
//     wire = "grost::flavors::groto::Repeated<grost::flavors::groto::LengthDelimited>",
//     partial_ref(copy),
//     repeated
//   )]
//   replyers: Vec<User<I>>,
//   #[grost(
//     tag = 4,
//     schema(description = "The content of the comment"),
//     wire = "grost::flavors::groto::LengthDelimited",
//     partial_ref(copy)
//   )]
//   content: String,
// }

#[test]
fn t() {
  // use grost::flavors::Groto;

  // let refl = Comment::<String>::user_reflection::<Groto>();
  // println!("{:?}", refl.identifier().encoded());
  // println!("{:?}", refl.tag().encoded());
  // println!("{:?}", refl.wire_type());
  // println!("{:?}", refl.wire_format());
  // let user = PartialRefUser {
  //   age: Some(1),
  //   name: Some("user".to_string()),
  //   emails: None,
  // };
  // println!("{:?}", <grost::reflection::SchemaSchemaTypeReflection<Option<Vec<Option<String>>>> as grost::reflection::Reflectable<Groto>>::REFLECTION);

  // let user = PartialRefUser::<'_, ()> {
  //   __grost_unknown_buffer__: todo!(),
  //   name: todo!(),
  //   age: todo!(),
  //   emails: todo!(),
  // };

  // let val = User::<String>::emails_reflection();
  // let wf = val.wire_format();
  // let identifier = val.identifier();
  // let encoded_identifier = val.encoded_identifier();
  // let object_refl = User::<String>::reflection();
  // println!("{:?}", encoded_identifier);

  // let mut decoder = PartialUserDecoder::new();
  // decoder.feed(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]).unwrap();
  // loop {
  //   if let Some((idx, identifier)) = decoder.next_field()? {
  //     match idx {
  //       UserIndex::Age => {

  //       },
  //       UserIndex::Emails => {

  //       }
  //     }
  //   } else {
  //     decoder.feed(&[11, 12, 13, 14, 15]).unwrap();
  //   }
  // }
}
