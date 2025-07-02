// mod map_selector;
pub use packed_decoder::PackedDecoder;

use crate::{
  buffer::{Buffer, ReadBuf},
  convert::{Partial, PartialRef, PartialTransform, State, Transform},
  decode::Decode,
  encode::{Encode, PartialEncode},
  flavors::{
    DefaultWireFormat, Flavor, Groto, WireFormat,
    groto::{Context, Error, Optional},
  },
  selection::Selectable,
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

macro_rules! bidi_equivalent {
  ($($($($($lt:lifetime), +$(,)?)? :< $($tg:ident:$t:path $(: $ltb:lifetime)?),+$(,)? >:)? $([$(const $g:ident: usize), +$(,)?])? impl<$other:ty, $wf:ty> for <$this:ty, $this_wf:ty>),+$(,)?) => {
    bidi_equivalent!(@encode $($($($lt),*)? $(:< $($tg:$t),* >:)? $([$(const $g: usize),*])? impl<$other, $wf> for <$this, $this_wf>),*);
    bidi_equivalent!(@partial_encode $($($($lt),*)? $(:< $($tg:$t),* >:)? $([$(const $g: usize),*])? impl<$other, $wf> for <$this, $this_wf>),*);
  };
  (@encode $($($($lt:lifetime), +$(,)?)? $(:< $($tg:ident:$t:path $(: $ltb:lifetime)?),+$(,)? >:)? $([$(const $g:ident: usize), +$(,)?])? impl<$other:ty, $wf:ty> for <$this:ty, $this_wf:ty>),+$(,)?) => {
    $(
      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: usize),*)?> $crate::__private::encode::EquivalentEncode<$other, $wf, $crate::__private::flavors::Groto> for $this
      {
        type Flavor = $crate::__private::flavors::Groto;
        type WireFormat = $this_wf;
      }

      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: usize),*)?> $crate::__private::encode::EquivalentEncode<$this, $this_wf, $crate::__private::flavors::Groto> for $other
      {
        type Flavor = $crate::__private::flavors::Groto;
        type WireFormat = $wf;
      }

      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: usize),*)?> $crate::__private::encode::EquivalentEncode<&$other, $wf, $crate::__private::flavors::Groto> for &$this
      {
        type Flavor = $crate::__private::flavors::Groto;
        type WireFormat = $this_wf;
      }

      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: usize),*)?> $crate::__private::encode::EquivalentEncode<&$this, $this_wf, $crate::__private::flavors::Groto> for &$other
      {
        type Flavor = $crate::__private::flavors::Groto;
        type WireFormat = $wf;
      }

      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: usize),*)?> $crate::__private::encode::EquivalentEncode<&$other, $wf, $crate::__private::flavors::Groto> for $this
      {
        type Flavor = $crate::__private::flavors::Groto;
        type WireFormat = $this_wf;
      }

      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: usize),*)?> $crate::__private::encode::EquivalentEncode<&$this, $this_wf, $crate::__private::flavors::Groto> for $other
      {
        type Flavor = $crate::__private::flavors::Groto;
        type WireFormat = $wf;
      }

      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: usize),*)?> $crate::__private::encode::EquivalentEncode<$other, $wf, $crate::__private::flavors::Groto> for &$this
      {
        type Flavor = $crate::__private::flavors::Groto;
        type WireFormat = $this_wf;
      }

      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: usize),*)?> $crate::__private::encode::EquivalentEncode<$this, $this_wf, $crate::__private::flavors::Groto> for &$other
      {
        type Flavor = $crate::__private::flavors::Groto;
        type WireFormat = $wf;
      }
    )*
  };
  (@partial_encode $($($($lt:lifetime), +$(,)? )? $(:< $($tg:ident:$t:path $(: $ltb:lifetime)?),+$(,)? >:)? $([$(const $g:ident: usize), +$(,)?])? impl<$other:ty, $wf:ty> for <$this:ty, $this_wf:ty>),+$(,)?) => {
    $(
      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: usize),*)?> $crate::__private::encode::EquivalentPartialEncode<$other, $wf, $crate::__private::flavors::Groto> for $this
      {
        type Flavor = $crate::__private::flavors::Groto;
        type WireFormat = $this_wf;
      }

      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: usize),*)?> $crate::__private::encode::EquivalentPartialEncode<$this, $this_wf, $crate::__private::flavors::Groto> for $other
      {
        type Flavor = $crate::__private::flavors::Groto;
        type WireFormat = $wf;
      }

      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: usize),*)?> $crate::__private::encode::EquivalentPartialEncode<&$other, $wf, $crate::__private::flavors::Groto> for &$this
      {
        type Flavor = $crate::__private::flavors::Groto;
        type WireFormat = $this_wf;
      }

      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: usize),*)?> $crate::__private::encode::EquivalentPartialEncode<&$this, $this_wf, $crate::__private::flavors::Groto> for &$other
      {
        type Flavor = $crate::__private::flavors::Groto;
        type WireFormat = $wf;
      }

      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: usize),*)?> $crate::__private::encode::EquivalentPartialEncode<&$other, $wf, $crate::__private::flavors::Groto> for $this
      {
        type Flavor = $crate::__private::flavors::Groto;
        type WireFormat = $this_wf;
      }

      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: usize),*)?> $crate::__private::encode::EquivalentPartialEncode<&$this, $this_wf, $crate::__private::flavors::Groto> for $other
      {
        type Flavor = $crate::__private::flavors::Groto;
        type WireFormat = $wf;
      }

      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: usize),*)?> $crate::__private::encode::EquivalentPartialEncode<$other, $wf, $crate::__private::flavors::Groto> for &$this
      {
        type Flavor = $crate::__private::flavors::Groto;
        type WireFormat = $this_wf;
      }

      unsafe impl<$($($lt),*,)? $($($tg:$t $(+ $ltb)?),*,)? $($(const $g: usize),*)?> $crate::__private::encode::EquivalentPartialEncode<$this, $this_wf, $crate::__private::flavors::Groto> for &$other
      {
        type Flavor = $crate::__private::flavors::Groto;
        type WireFormat = $wf;
      }
    )*
  };
}

mod list;
mod packed_decoder;
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

impl<'de, W, O, RB, B, T> Decode<'de, Option<O>, Optional<W>, RB, B, Groto> for Option<T>
where
  T: Decode<'de, O, W, RB, B, Groto>,
  W: WireFormat<Groto>,
{
  fn decode(
    context: &'de <Groto as Flavor>::Context,
    src: RB,
  ) -> Result<(usize, Option<O>), <Groto as Flavor>::Error>
  where
    O: Sized + 'de,
    RB: ReadBuf + 'de,
    B: Buffer<<Groto as Flavor>::Unknown<RB>> + 'de,
  {
    T::decode(context, src).map(|(read, val)| (read, Some(val)))
  }
}

impl<W, I, O, T> Transform<Option<I>, Option<O>, Optional<W>, Groto> for Option<T>
where
  W: WireFormat<Groto>,
  T: Transform<I, O, W, Groto>,
{
  fn transform(input: Option<I>) -> Result<Option<O>, <Groto as Flavor>::Error> {
    match input {
      Some(value) => T::transform(value).map(Some),
      None => Ok(None),
    }
  }
}

impl<I, O, W, T> PartialTransform<Option<I>, Option<O>, Optional<W>, Groto> for Option<T>
where
  W: WireFormat<Groto>,
  T: PartialTransform<I, Option<O>, W, Groto> + Sized + Selectable<Groto>,
  I: Selectable<Groto, Selector = Self::Selector>,
  O: Selectable<Groto, Selector = Self::Selector>,
{
  fn partial_transform(
    input: Option<I>,
    selector: &Self::Selector,
  ) -> Result<Option<O>, <Groto as Flavor>::Error> {
    match input {
      Some(value) => {
        <T as PartialTransform<I, Option<O>, W, Groto>>::partial_transform(value, selector)
      }
      None => Ok(None),
    }
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

impl<W, T> Encode<Optional<W>, Groto> for Option<T>
where
  T: Encode<W, Groto>,
  W: WireFormat<Groto>,
{
  fn encode_raw(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    if let Some(value) = self {
      value.encode_raw(context, buf)
    } else {
      Ok(0)
    }
  }

  fn encoded_raw_len(&self, context: &Context) -> usize {
    if let Some(value) = self {
      value.encoded_raw_len(context)
    } else {
      0
    }
  }

  fn encode(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    if let Some(value) = self {
      value.encode(context, buf)
    } else {
      Ok(0)
    }
  }

  fn encoded_len(&self, context: &Context) -> usize {
    if let Some(value) = self {
      value.encoded_len(context)
    } else {
      0
    }
  }

  fn encode_length_delimited(&self, context: &Context, buf: &mut [u8]) -> Result<usize, Error> {
    if let Some(value) = self {
      value.encode_length_delimited(context, buf)
    } else {
      Ok(0)
    }
  }

  fn encoded_length_delimited_len(&self, context: &Context) -> usize {
    if let Some(value) = self {
      value.encoded_length_delimited_len(context)
    } else {
      0
    }
  }
}

impl<W, T> PartialEncode<W, Groto> for Option<T>
where
  T: PartialEncode<W, Groto>,
  W: WireFormat<Groto>,
{
  fn partial_encode_raw(
    &self,
    context: &<Groto as Flavor>::Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, <Groto as Flavor>::Error> {
    if let Some(value) = self {
      value.partial_encode_raw(context, buf, selector)
    } else {
      Ok(0)
    }
  }

  fn partial_encoded_raw_len(
    &self,
    context: &<Groto as Flavor>::Context,
    selector: &Self::Selector,
  ) -> usize {
    if let Some(value) = self {
      value.partial_encoded_raw_len(context, selector)
    } else {
      0
    }
  }

  fn partial_encode(
    &self,
    context: &Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, Error> {
    if let Some(value) = self {
      value.partial_encode(context, buf, selector)
    } else {
      Ok(0)
    }
  }

  fn partial_encoded_len(&self, context: &Context, selector: &Self::Selector) -> usize {
    if let Some(value) = self {
      value.partial_encoded_len(context, selector)
    } else {
      0
    }
  }

  fn partial_encoded_length_delimited_len(
    &self,
    context: &Context,
    selector: &Self::Selector,
  ) -> usize {
    if let Some(value) = self {
      value.partial_encoded_length_delimited_len(context, selector)
    } else {
      0
    }
  }

  fn partial_encode_length_delimited(
    &self,
    context: &Context,
    buf: &mut [u8],
    selector: &Self::Selector,
  ) -> Result<usize, Error> {
    if let Some(value) = self {
      value.partial_encode_length_delimited(context, buf, selector)
    } else {
      Ok(0)
    }
  }
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

impl<T> Selectable<Groto> for Option<T>
where
  T: Selectable<Groto>,
{
  type Selector = T::Selector;

  fn is_empty(&self) -> bool {
    match self {
      Some(value) => value.is_empty(),
      None => true,
    }
  }
}
