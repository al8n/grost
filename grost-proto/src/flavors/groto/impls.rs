// mod map_selector;
pub use packed_decoder::PackedDecoder;

use crate::{
  buffer::{Buffer, ReadBuf},
  convert::{Partial, PartialRef, PartialTransform, State, Transform},
  decode::Decode,
  flavors::{DefaultWireFormat, Flavor, Groto, WireFormat, groto::Optional},
  selection::{Selectable, Selector},
};

macro_rules! identity_partial_transform {
  ($flavor:ty {
    $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? as $wf:ty), +$(,)?
  }) => {
    $(
      impl $(< $(const $g: ::core::primitive::usize),* > )? $crate::__private::convert::PartialTransform<$ty, ::core::option::Option<$ty>, $wf, $flavor> for $ty {
        fn partial_transform(input: Self, selector: &bool) -> ::core::result::Result<::core::option::Option<$ty>, <$flavor as $crate::__private::flavors::Flavor>::Error>
        {
          if $crate::__private::selection::Selector::<$flavor>::is_empty(selector) {
            return ::core::result::Result::Ok(::core::option::Option::None);
          }

          ::core::result::Result::Ok(::core::option::Option::Some(input))
        }
      }

      impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::convert::PartialTransform<::core::option::Option<Self>, ::core::option::Option<Self>, $crate::__private::flavors::groto::Optional<$wf>, $flavor,> for $ty {
        fn partial_transform(input: ::core::option::Option<Self>, selector: &<Self as $crate::__private::selection::Selectable<$flavor>>::Selector) -> ::core::result::Result<::core::option::Option<Self>, <$flavor as $crate::__private::flavors::Flavor>::Error>
        {
          match input {
            ::core::option::Option::None => ::core::result::Result::Ok(::core::option::Option::None),
            ::core::option::Option::Some(input) => {
              if $crate::__private::selection::Selector::<$flavor>::is_empty(selector) {
                return ::core::result::Result::Ok(::core::option::Option::None);
              }

              ::core::result::Result::Ok(::core::option::Option::Some(input))
            }
          }
        }
      }
    )*
  };
}

mod list;
mod packed_decoder;
mod repeated;
mod scalar;
mod tuple;

pub trait GrotoTransform<I, O, W>: Transform<I, O, W, Groto>
where
  W: WireFormat<Groto>,
{
}

impl<I, O, W, T> GrotoTransform<I, O, W> for T
where
  W: WireFormat<Groto>,
  T: Transform<I, O, W, Groto>,
{
}

pub trait GrotoPartialTransform<I, O, W>: PartialTransform<I, O, W, Groto>
where
  W: WireFormat<Groto>,
  I: Selectable<Groto, Selector = Self::Selector>,
  O: Selectable<Groto, Selector = Self::Selector>,
  Self: Selectable<Groto>,
{
}

impl<I, O, W, T> GrotoPartialTransform<I, O, W> for T
where
  W: WireFormat<Groto>,
  T: PartialTransform<I, O, W, Groto>,
  I: Selectable<Groto, Selector = T::Selector>,
  O: Selectable<Groto, Selector = T::Selector>,
  T: Selectable<Groto>,
{
}

impl<'de, W, O, B, UB, T> Decode<'de, Groto, Optional<W>, Option<O>, B, UB> for Option<T>
where
  T: Decode<'de, Groto, W, O, B, UB>,
  W: WireFormat<Groto>,
{
  fn decode(
    context: &'de <Groto as Flavor>::Context,
    src: B,
  ) -> Result<(usize, Option<O>), <Groto as Flavor>::Error>
  where
    O: Sized + 'de,
    B: ReadBuf + 'de,
    UB: Buffer<<Groto as Flavor>::Unknown<B>> + 'de,
  {
    T::decode(context, src).map(|(read, val)| (read, Some(val)))
  }
}

impl<W, T> Transform<Self, Self, W, Groto> for Option<T>
where
  W: WireFormat<Groto>,
{
  fn transform(input: Self) -> Result<Self, <Groto as Flavor>::Error> {
    Ok(input)
  }
}

impl<I, O, W, T> PartialTransform<I, Option<O>, W, Groto> for Option<T>
where
  W: WireFormat<Groto>,
  T: PartialTransform<I, Option<O>, W, Groto> + Sized + Selectable<Groto>,
  I: Selectable<Groto, Selector = Self::Selector>,
  O: Selectable<Groto, Selector = Self::Selector>,
{
  fn partial_transform(
    input: I,
    selector: &Self::Selector,
  ) -> Result<Option<O>, <Groto as Flavor>::Error> {
    <T as PartialTransform<I, Option<O>, W, Groto>>::partial_transform(input, selector)
  }
}

impl<'a, RB, UB, W, T> State<PartialRef<'a, RB, UB, Optional<W>, Groto>> for Option<T>
where
  T: State<PartialRef<'a, RB, UB, W, Groto>>,
  T::Output: Sized,
  W: ?Sized,
  RB: ?Sized,
  UB: ?Sized,
{
  type Output = Option<T::Output>;
}

impl<T> State<Partial<Groto>> for Option<T>
where
  T: State<Partial<Groto>>,
  T::Output: Sized,
{
  type Output = Option<T::Output>;
}

impl<T> DefaultWireFormat<Groto> for Option<T>
where
  T: DefaultWireFormat<Groto>,
{
  type Format = Optional<T::Format>;
}

impl<T> DefaultWireFormat<Groto> for &T
where
  T: DefaultWireFormat<Groto> + ?Sized,
{
  type Format = T::Format;
}
