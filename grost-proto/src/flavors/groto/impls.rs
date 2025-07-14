// mod map_selector;
pub use map::PackedEntriesDecoder;
pub use packed_decoder::PackedDecoder;
pub use repeated_decoder::RepeatedDecoder;
pub use set::PackedSetDecoder;

use crate::{
  convert::{PartialTransform, Transform},
  flavors::{Groto, WireFormat},
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

      impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::convert::PartialTransform<::core::option::Option<Self>, ::core::option::Option<Self>, $crate::__private::flavors::Nullable<$wf>, $flavor,> for $ty {
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
mod map;
mod nullable;
mod packed_decoder;
mod repeated_decoder;
mod scalar;
mod set;
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

macro_rules! default_wire_format_ref {
  ($($t:ident $(< $($p:ident),+$(,)? >)?),+$(,)?) => {
    $(
      impl<T> $crate::__private::flavors::$t<Groto> for &T
      where
        T: $crate::__private::flavors::$t<Groto> + ?Sized,
      {
        type Format $(< $($p),* >)? = T::Format $(< $($p),* > where $($p: $crate::__private::flavors::WireFormat<Groto> + 'static),*)?;
      }
    )*
  };
}

default_wire_format_ref!(
  DefaultBytesWireFormat,
  DefaultStringWireFormat,
  DefaultScalarWireFormat,
  DefaultObjectWireFormat,
  DefaultEnumWireFormat,
  DefaultUnionWireFormat,
  DefaultNullableWireFormat<V>,
  DefaultFlattenWireFormat<V>,
  DefaultListWireFormat<V>,
  DefaultSetWireFormat<K>,
  DefaultMapWireFormat<K, V>,
);
