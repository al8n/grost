use grost::{
  Encode, Flavor, Message,
  flavors::{
    Network,
    network::{Context, LengthDelimited},
  },
  reflection::{
    FieldReflection,
    encode::{EncodeField, EncodeRefField, EncodeReflection},
  },
};
use grost_codegen_test::user_struct::*;

struct CommentReflection<Type, F> {
  _m: std::marker::PhantomData<Type>,
  _f: std::marker::PhantomData<F>,
}

struct CommentUserField<T>(std::marker::PhantomData<T>);

struct CommentTitleField<T>(std::marker::PhantomData<T>);

fn encode_user(
  _: &User,
  _: &Context,
  _: &[u8],
) -> core::result::Result<usize, grost::flavors::network::EncodeError> {
  Ok(0)
}

fn encode_title_ref(
  _: &<String as Message<Network, LengthDelimited>>::Encoded<'_>,
  _: &Context,
  _: &[u8],
) -> core::result::Result<usize, grost::flavors::network::EncodeError> {
  Ok(0)
}

impl core::ops::Deref
  for CommentReflection<CommentUserField<EncodeReflection<EncodeField>>, Network>
{
  type Target =
    fn(&User, &Context, &[u8]) -> core::result::Result<usize, grost::flavors::network::EncodeError>;

  fn deref(&self) -> &Self::Target {
    const ENCODE_FN: fn(
      &User,
      &Context,
      &[u8],
    ) -> core::result::Result<usize, grost::flavors::network::EncodeError> = encode_user;
    &ENCODE_FN
  }
}

impl<'a> core::ops::Deref
  for CommentReflection<CommentTitleField<EncodeReflection<EncodeRefField<'a>>>, Network>
{
  type Target = fn(
    &<String as Message<Network, LengthDelimited>>::Encoded<'a>,
    &Context,
    &[u8],
  ) -> core::result::Result<usize, grost::flavors::network::EncodeError>;

  fn deref(&self) -> &Self::Target {
    #[allow(clippy::type_complexity)]
    const ENCODE_FN: fn(
      &<String as Message<Network, LengthDelimited>>::Encoded<'_>,
      &Context,
      &[u8],
    ) -> core::result::Result<usize, grost::flavors::network::EncodeError> = encode_title_ref;
    &ENCODE_FN
  }
}

impl core::ops::Deref for CommentReflection<CommentUserField<FieldReflection<Network>>, Network> {
  type Target = FieldReflection<Network>;

  fn deref(&self) -> &Self::Target {
    todo!()
  }
}

#[test]
fn t() {
  let indexer = UserFieldIndex::Age;
  let user_reflection = User::reflection::<Network>();
  println!("{:#?}", user_reflection);
  let comment_reflection = Comment::reflection::<Network>();
  println!("{:#?}", comment_reflection);

  let s = CommentSelector::<Network>::all();
  let r = s[CommentFieldIndex::User];
  println!("{:?}", s.without_title());
}
