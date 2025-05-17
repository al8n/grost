// // use grost::{
// //   Decode, Encode, Flavor, Message, Referenceable,
// //   buffer::Buffer,
// //   flavors::{
// //     Network, WireFormat,
// //     network::{Context, LengthDelimited},
// //   },
// //   reflection::{
// //     FieldReflection,
// //     encode::{EncodeField, EncodeRefField, EncodeReflection},
// //   },
// // };
// // use grost_codegen_test::user_struct::*;

// // struct CommentReflection<Type, F> {
// //   _m: std::marker::PhantomData<Type>,
// //   _f: std::marker::PhantomData<F>,
// // }

// // struct CommentUserField<T>(std::marker::PhantomData<T>);

// // struct CommentTitleField<T>(std::marker::PhantomData<T>);

// // fn encode_user(
// //   _: &User,
// //   _: &Context,
// //   _: &[u8],
// // ) -> core::result::Result<usize, grost::flavors::network::EncodeError> {
// //   Ok(0)
// // }

// // fn encode_title_ref(
// //   _: &<String as Message<Network, LengthDelimited>>::Decoded<'_>,
// //   _: &Context,
// //   _: &[u8],
// // ) -> core::result::Result<usize, grost::flavors::network::EncodeError> {
// //   Ok(0)
// // }

// // impl core::ops::Deref
// //   for CommentReflection<CommentUserField<EncodeReflection<EncodeField>>, Network>
// // {
// //   type Target =
// //     fn(&User, &Context, &[u8]) -> core::result::Result<usize, grost::flavors::network::EncodeError>;

// //   fn deref(&self) -> &Self::Target {
// //     const ENCODE_FN: fn(
// //       &User,
// //       &Context,
// //       &[u8],
// //     ) -> core::result::Result<usize, grost::flavors::network::EncodeError> = encode_user;
// //     &ENCODE_FN
// //   }
// // }

// // impl<'a> core::ops::Deref
// //   for CommentReflection<CommentTitleField<EncodeReflection<EncodeRefField<'a>>>, Network>
// // {
// //   type Target = fn(
// //     &<String as Message<Network, LengthDelimited>>::Decoded<'a>,
// //     &Context,
// //     &[u8],
// //   ) -> core::result::Result<usize, grost::flavors::network::EncodeError>;

// //   fn deref(&self) -> &Self::Target {
// //     #[allow(clippy::type_complexity)]
// //     const ENCODE_FN: fn(
// //       &<String as Message<Network, LengthDelimited>>::Decoded<'_>,
// //       &Context,
// //       &[u8],
// //     ) -> core::result::Result<usize, grost::flavors::network::EncodeError> = encode_title_ref;
// //     &ENCODE_FN
// //   }
// // }

// // impl core::ops::Deref for CommentReflection<CommentUserField<FieldReflection<Network>>, Network> {
// //   type Target = FieldReflection<Network>;

// //   fn deref(&self) -> &Self::Target {
// //     todo!()
// //   }
// // }

// // // pub struct PartialUserRef<'a, UB> {
// // //   name: ::core::option::Option<
// // //     <::std::string::String as ::grost::__private::Referenceable<
// // //       ::grost::__private::flavors::Network,
// // //     >>::Ref<'a, UB>,
// // //   >,
// // //   age: ::core::option::Option<
// // //     <u32 as ::grost::__private::Referenceable<::grost::__private::flavors::Network>>::Ref<'a, UB>,
// // //   >,
// // //   email: ::core::option::Option<
// // //     <::core::option::Option<::std::string::String> as ::grost::__private::Referenceable<
// // //       ::grost::__private::flavors::Network,
// // //     >>::Ref<'a, UB>,
// // //   >,
// // //   unknown: ::core::option::Option<UB>,
// // // }

// // pub trait Decode1<'de, F, W, O>
// // where
// //   F: Flavor + ?Sized,
// //   W: WireFormat<F>,
// // {
// //   type Buffer;

// //   /// Decodes an instance of this type from a byte buffer.
// //   ///
// //   /// The function consumes the entire buffer and returns both the
// //   /// number of bytes consumed and the decoded instance.
// //   fn decode(context: &F::Context, src: &'de [u8]) -> Result<(usize, O), F::DecodeError>
// //   where
// //     O: Sized + 'de,
// //     Self::Buffer: Buffer<F::Unknown<&'de [u8]>> + 'de;
// // }

// // // impl<'de, B> Decode1<'de, Network, LengthDelimited, PartialUserRef<'de, B>> for User {
// // //   type Buffer = B;

// // //   fn decode(
// // //     context: &<Network as Flavor>::Context,
// // //     src: &'de [u8],
// // //   ) -> Result<(usize, PartialUserRef<'de, B>), <Network as Flavor>::DecodeError>
// // //   where
// // //     PartialUserRef<'de, B>: Sized + 'de,
// // //     Self::Buffer: Buffer<<Network as Flavor>::Unknown<&'de [u8]>> + 'de,
// // //   {
// // //     todo!()
// // //   }
// // // }

// // #[test]
// // fn t() {
// //   let indexer = UserFieldIndex::Age;
// //   let user_reflection = User::reflection::<Network>();
// //   println!("{:#?}", user_reflection);
// //   // let comment_reflection = Comment::reflection::<Network>();
// //   // println!("{:#?}", comment_reflection);

// //   // let s = CommentSelector::<Network>::all();
// //   // let r = s[CommentFieldIndex::User];

// //   // let a = <<User as Referenceable<Network>>::Ref<'_, ()>>::new();
// //   // println!("{:?}", s.without_title());
// // }

// use grost::{
//   Decode, Decoded, Flavor,
//   buffer::Buffer,
//   flavors::{Network, WireFormat, network::LengthDelimited},
// };
// use grost_codegen_test::user_struct::PartialUserRef;

// pub trait Decode1<'de, F, W, O>
// where
//   F: Flavor + ?Sized,
//   W: WireFormat<F>,
// {
//   type UnknownBuffer;

//   /// Decodes an instance of this type from a byte buffer.
//   ///
//   /// The function consumes the entire buffer and returns both the
//   /// number of bytes consumed and the decoded instance.
//   fn decode(context: &F::Context, src: &'de [u8]) -> Result<(usize, O), F::DecodeError>
//   where
//     O: Sized + 'de,
//     Self::UnknownBuffer: Buffer<F::Unknown<&'de [u8]>> + 'de;
// }

// impl<'de, UB>
//   Decode1<'de, Network, LengthDelimited, Decoded<PartialUserRef<'de, Network>, &'de [u8], UB>>
//   for PartialUserRef<'de, Network>
// {
//   type UnknownBuffer = UB;

//   fn decode(
//     context: &<Network as Flavor>::Context,
//     src: &'de [u8],
//   ) -> Result<
//     (usize, Decoded<PartialUserRef<'de, Network>, &'de [u8], UB>),
//     <Network as Flavor>::DecodeError,
//   >
//   where
//     Decoded<PartialUserRef<'de, Network>, &'de [u8], UB>: Sized + 'de,
//     Self::UnknownBuffer: Buffer<<Network as Flavor>::Unknown<&'de [u8]>> + 'de,
//   {
//     todo!()
//   }
// }

// #[test]
// fn t() {
//   let user = PartialUserRef::<Network>::new().with_age(18);
//   println!("{:?}", user.age_ref());
// }
