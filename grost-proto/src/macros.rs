/// A macro emits traits implementations for primitive types that implements [`varing::Varint`](varing::Varint) and [`Copy`](::core::marker::Copy).
#[macro_export]
macro_rules! network_varint {
  ($(
    $ty:ty $([ $( const $g:ident: usize), +$(,)? ])?
  ),+$(,)?) => {
    $($crate::selectable_scalar!($crate::__private::flavors::Network: $ty $([ $(const $g: usize),* ])?);)*
    $($crate::default_wire_format!($crate::__private::flavors::Network: $ty $([$(const $g: usize),*])? as $crate::__private::network::Varint);)*
    $($crate::message!($crate::__private::flavors::Network: $ty $([$(const $g: usize),*])? as $crate::__private::network::Varint);)*
    $($crate::partial_encode_scalar!($crate::__private::flavors::Network: $ty $([ $(const $g: usize),* ])? as $crate::__private::network::Varint);)*
    $($crate::network_varint!(@encode $ty $([ $(const $g: usize),* ])?);)*
    $($crate::network_varint!(@decode $ty $([ $(const $g: usize),* ])?);)*
    $($crate::network_varint!(@decode_owned $ty $([ $(const $g: usize),* ])?);)*
  };
  (@encode $ty:ty $([ $( const $g:ident: usize), +$(,)? ])?) => {
    impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::network::Varint> for $ty {
      $crate::network_varint!(@encode_impl);
    }
  };
  (@encode_impl) => {
    fn encode(&self, _: &$crate::__private::flavors::network::Context, buf: &mut [::core::primitive::u8]) -> ::core::result::Result<::core::primitive::usize, $crate::__private::flavors::network::EncodeError> {
      $crate::__private::varing::Varint::encode(self, buf).map_err(::core::convert::Into::into)
    }

    fn encoded_len(&self, _: &$crate::__private::flavors::network::Context,) -> ::core::primitive::usize {
      $crate::__private::varing::Varint::encoded_len(self)
    }

    fn encode_length_delimited(&self, ctx: &$crate::__private::flavors::network::Context, buf: &mut [::core::primitive::u8]) -> ::core::result::Result<::core::primitive::usize, $crate::__private::flavors::network::EncodeError> {
      <Self as $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::network::Varint>>::encode(self, ctx, buf)
    }

    fn encoded_length_delimited_len(&self, ctx: &$crate::__private::flavors::network::Context,) -> ::core::primitive::usize {
      <Self as $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::network::Varint>>::encoded_len(self, ctx)
    }
  };
  (@decode $ty:ty $([ $( const $g:ident: usize), +$(,)? ])?) => {
    impl<'de, $($(const $g: ::core::primitive::usize),*)?> $crate::__private::Decode<'de, $crate::__private::flavors::Network, $crate::__private::network::Varint, Self> for $ty {
      $crate::network_varint!(@decode_impl);
    }
  };
  (@decode_impl) => {
    fn decode<UB>(_: &$crate::__private::flavors::network::Context, src: &'de [::core::primitive::u8]) -> ::core::result::Result<(::core::primitive::usize, Self), <$crate::__private::flavors::Network as $crate::__private::flavors::Flavor>::DecodeError>
    where
      Self: ::core::marker::Sized + 'de,
      UB: $crate::__private::Buffer<$crate::__private::network::Unknown<&'de [::core::primitive::u8]>> + 'de,
    {
      $crate::__private::varing::Varint::decode(src).map_err(::core::convert::Into::into)
    }

    fn decode_length_delimited<UB>(ctx: &$crate::__private::flavors::network::Context, src: &'de [::core::primitive::u8]) -> ::core::result::Result<(::core::primitive::usize, Self), <$crate::__private::flavors::Network as $crate::__private::flavors::Flavor>::DecodeError>
    where
      Self: ::core::marker::Sized + 'de,
      UB: $crate::__private::Buffer<$crate::__private::network::Unknown<&'de [::core::primitive::u8]>> + 'de,
    {
      <Self as $crate::__private::Decode<'_, $crate::__private::flavors::Network, $crate::__private::network::Varint, Self>>::decode::<UB>(ctx, src)
    }
  };
  (@decode_owned $ty:ty $([ $( const $g:ident: usize), +$(,)? ])?) => {
    impl$(<$(const $g: ::core::primitive::usize),*>)? $crate::__private::DecodeOwned<$crate::__private::flavors::Network, $crate::__private::network::Varint, Self> for $ty {
      $crate::network_varint!(@decode_owned_impl);
    }
  };
  (@decode_owned_impl) => {
    fn decode_owned<B, UB>(ctx: &$crate::__private::flavors::network::Context, src: B) -> ::core::result::Result<(::core::primitive::usize, Self), <$crate::__private::flavors::Network as $crate::__private::flavors::Flavor>::DecodeError>
    where
      Self: ::core::marker::Sized + 'static,
      B: $crate::__private::BytesBuffer + 'static,
      UB: $crate::__private::Buffer<$crate::__private::network::Unknown<B>> + 'static,
    {
      <Self as $crate::__private::Decode<'_, $crate::__private::flavors::Network, $crate::__private::network::Varint, Self>>::decode::<()>(ctx, $crate::__private::BytesBuffer::as_bytes(&src))
    }

    fn decode_length_delimited_owned<B, UB>(ctx: &$crate::__private::flavors::network::Context, src: B) -> ::core::result::Result<(::core::primitive::usize, Self), <$crate::__private::flavors::Network as $crate::__private::flavors::Flavor>::DecodeError>
    where
      Self: ::core::marker::Sized + 'static,
      B: $crate::__private::BytesBuffer + 'static,
      UB: $crate::__private::Buffer<$crate::__private::network::Unknown<B>> + 'static,
    {
      <Self as $crate::__private::DecodeOwned<$crate::__private::flavors::Network, $crate::__private::network::Varint, Self>>::decode_owned::<B, UB>(ctx, src)
    }
  };
}

/// A macro emits [`PartialEncode`](super::PartialEncode) implementations for primitive types.
#[macro_export]
macro_rules! partial_encode_scalar {
  (@impl $flavor:ty as $format:ty) => {
    fn partial_encode(&self, context: &<$flavor as $crate::__private::Flavor>::Context, buf: &mut [::core::primitive::u8], selector: &Self::Selector) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::Flavor>::EncodeError> {
      if *selector {
        <Self as $crate::__private::Encode<$flavor, $format>>::encode(self, context, buf)
      } else {
        ::core::result::Result::Ok(0)
      }
    }

    fn partial_encoded_len(&self, context: &<$flavor as $crate::__private::Flavor>::Context, selector: &Self::Selector) -> ::core::primitive::usize {
      if *selector {
        <Self as $crate::__private::Encode<$flavor, $format>>::encoded_len(self, context)
      } else {
        0
      }
    }

    fn partial_encode_length_delimited(&self, context: &<$flavor as $crate::__private::Flavor>::Context, buf: &mut [::core::primitive::u8], selector: &Self::Selector) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::Flavor>::EncodeError> {
      if *selector {
        <Self as $crate::__private::Encode<$flavor, $format>>::encode_length_delimited(self, context, buf)
      } else {
        ::core::result::Result::Ok(0)
      }
    }

    fn partial_encoded_length_delimited_len(&self, context: &<$flavor as $crate::__private::Flavor>::Context, selector: &Self::Selector) -> ::core::primitive::usize {
      if *selector {
        <Self as $crate::__private::Encode<$flavor, $format>>::encoded_length_delimited_len(self, context)
      } else {
        0
      }
    }
  };
  ($flavor:ty: $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? as $format:ty),+$(,)?) => {
    $(
      impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::PartialEncode<$flavor, $format> for $ty {
        $crate::partial_encode_scalar!(@impl $flavor as $format);
      }
    )*
  };
}

/// A macro emits [`Selectable`](super::flavors::Selectable) implementations for primitive types.
#[macro_export]
macro_rules! selectable_scalar {
  ($flavor:ty: $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])?),+$(,)?) => {
    $(
      impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::Selectable<$flavor> for $ty {
        type Selector = ::core::primitive::bool;
      }
    )*
  };
}

/// A macro emits [`TypeRef`](super::TypeRef) implementations for `Self`
#[macro_export]
macro_rules! type_ref {
  ( $flavor:ty: $($ty:ty => $target:ty { $expr: expr } $([ $( const $g:ident: usize), +$(,)? ])?),+$(,)?) => {
    $(
      impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::TypeRef<$flavor, $target> for $ty {
        $crate::type_ref!(_impl $flavor: $target { $expr });
      }
    )*
  };
  (_impl $flavor:ty: $target:ty { $expr: expr }) => {
    fn to(&self) -> ::core::result::Result<$target, <$flavor as $crate::__private::Flavor>::DecodeError> {
      $expr(self)
    }
  };
}

/// A macro emits [`TypeOwned`](super::TypeOwned) implementations for `Self`
#[macro_export]
macro_rules! type_owned {
  ( $flavor:ty: $($ty:ty => $target:ty { $expr: expr } $([ $( const $g:ident: usize), +$(,)? ])?),+$(,)?) => {
    $(
      impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::TypeOwned<$flavor, $target> for $ty {
        $crate::type_owned!(_impl $flavor: $target { $expr });
      }
    )*
  };
  (_impl $flavor:ty: $target:ty { $expr: expr }) => {
    fn to(&self) -> ::core::result::Result<$target, <$flavor as $crate::__private::Flavor>::DecodeError> {
      $expr(self)
    }
  };
}

/// A macro emits [`IntoTarget`](super::IntoTarget) implementations for `Self`
#[macro_export]
macro_rules! into_target {
  ($flavor:ty: $($ty:ty => $target:ty { $expr:expr } $([ $( const $g:ident: usize), +$(,)? ])? ),+$(,)?) => {
    $(
      impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::IntoTarget<$flavor, $target> for $ty {
        $crate::into_target!(@impl $flavor: $target { $expr });
      }
    )*
  };
  (@impl $flavor:ty: $target:ty { $expr:expr }) => {
    fn into_target(self) -> ::core::result::Result<$target, <$flavor as $crate::__private::Flavor>::DecodeError> {
      $expr(self)
    }
  }
}

/// A macro emits [`DefaultWireFormat`](crate::flavors::DefaultWireFormat) implementations.
#[macro_export]
macro_rules! default_wire_format {
  ($(
    $flavor:ty: $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? as $format:ty); +$(;)?
  ),+$(,)?) => {
    $(
      $(
        impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::flavors::DefaultWireFormat<$flavor> for $ty {
          type Format = $format;
        }
      )*
    )*
  };
}

/// A macro emits [`Message`](super::Message) implementations for `Self`.
///
/// **NB:** this macro can only be used for types that implements [`Copy`](::core::marker::Copy).
#[macro_export]
macro_rules! message {
  ($flavor:ty: $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? as $format:ty),+$(,)?) => {
    $(
      impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::Message<$flavor, $format> for $ty {
        $crate::message!(@impl);
      }

      impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::PartialMessage<$flavor, $format> for $ty {
        $crate::message!(@impl_partial);
      }
    )*

    // $($crate::conversion!($flavor: $ty $([ $(const $g: usize),* ])? );)*
  };
  (@impl) => {
    type Partial = Self;

    type Encoded<'a>
      = Self
    where
      Self: ::core::marker::Sized + 'a;

    type Borrowed<'a>
      = &'a Self
    where
      Self: 'a;

    type EncodedOwned
      = Self
    where
      Self: ::core::marker::Sized + 'static;
  };
  (@impl_partial) => {
    type UnknownBuffer<B> = ();

    type Encoded<'a>
      = Self
    where
      Self: ::core::marker::Sized + 'a;

    type Borrowed<'a>
      = &'a Self
    where
      Self: 'a;

    type EncodedOwned
      = Self
    where
      Self: ::core::marker::Sized + 'static;
  };
}

/// A macro emits traits implementations for a type that through a bridge to another type.
///
/// ## Example
///
/// ```rust
/// use grost::bridge;
///
/// struct MyU8(u8);
///
/// struct MyU32(u32);
///
/// struct MyOtherU32 {
///   low: u16,
///   high: u16,
/// }
///
/// // Examples of bridge implementations
/// bridge! {
///   u8 {
///     MyU8 {
///       from: |v: u8| Self(v);
///       to: |v: &Self| v.0;
///     }
///   },
///   u32 {
///     MyU32 {
///       from: |v: u32| Self(v);
///       to: |v: &Self| v.0;
///     },
///     MyOtherU32 {
///       from: |v: u32| {
///         Self {
///           low: v as u16,
///           high: (v >> 16) as u16,
///         }
///       };
///       to: |v: &Self| {
///         (v.low as u32) | ((v.high as u32) << 16)
///       };
///     }
///   }
/// }
/// ```
#[macro_export]
macro_rules! bridge {
  ($(
    $flavor:ty: $bridge: ty {
      $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? as $format:ty {
        from: $from:expr;
        to: $to:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $crate::encode_bridge!($flavor: $bridge {
        $($ty $([ $(const $g: usize),* ])? as $format {
          convert: $to;
        },)*
      });

      $crate::decode_bridge!($flavor: $bridge {
        $($ty $([ $(const $g: usize),* ])? as $format {
          convert: $from;
        },)*
      });

      $crate::decode_owned_bridge!($flavor: $bridge {
        $($ty $([ $(const $g: usize),* ])? as $format {
          convert: $from;
        },)*
      });
    )*
  };
}

/// A macro emits traits implementations for a type that through a bridge to another type.
///
/// ## Example
///
/// ```rust
/// use grost::bridge;
///
/// struct MyU8(u8);
///
/// struct MyU32(u32);
///
/// struct MyOtherU32 {
///   low: u16,
///   high: u16,
/// }
///
/// // Examples of bridge implementations
/// bridge! {
///   u8 {
///     MyU8 {
///       from: |v: u8| Self(v);
///       to: |v: &Self| v.0;
///     }
///   },
///   u32 {
///     MyU32 {
///       from: |v: u32| Self(v);
///       to: |v: &Self| v.0;
///     },
///     MyOtherU32 {
///       from: |v: u32| {
///         Self {
///           low: v as u16,
///           high: (v >> 16) as u16,
///         }
///       };
///       to: |v: &Self| {
///         (v.low as u32) | ((v.high as u32) << 16)
///       };
///     }
///   }
/// }
/// ```
#[macro_export]
macro_rules! try_from_bridge {
  ($(
    $flavor:ty: $bridge: ty {
      $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? as $format:ty {
        try_from: $from:expr;
        to: $to:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $crate::encode_bridge!($flavor: $bridge {
        $($ty $([ $(const $g: usize),* ])? as $format {
          convert: $to;
        },)*
      });

      $crate::try_decode_bridge!($flavor: $bridge {
        $($ty $([ $(const $g: usize),* ])? as $format {
          convert: $from;
        },)*
      });
    )*
  };
  (@without_decode_owned $(
    $flavor:ty: $bridge: ty {
      $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? as $format:ty {
        try_from: $from:expr;
        to: $to:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $crate::encode_bridge!($flavor: $bridge {
        $($ty $([ $(const $g: usize),* ])? as $format {
          convert: $to;
        })*
      });

      $crate::try_decode_bridge!(@without_decode_owned $flavor: $bridge {
        $($ty $([ $(const $g: usize),* ])? as $format {
          convert: $from;
        })*
      });
    )*
  };
}

/// A macro emits [`Selectable`](super::flavors::Selectable) implementations for `Self`.
#[macro_export]
macro_rules! selectable_bridge {
  ($(
    $flavor:ty: $bridge: ty [
      $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])?), +$(,)?
    ]
  ),+$(,)?) => {
    $(
      $(
        impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::Selectable<$flavor> for $ty {
          type Selector = <$bridge as $crate::__private::Selectable<$flavor>>::Selector;
        }
      )*
    )*
  };
}

/// A macro emits [`Encode`](super::encode::Encode) implementations for `Self`.
#[macro_export]
macro_rules! encode_bridge {
  ($(
    $flavor:ty: $bridge: ty {
      $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? as $format:ty {
        convert: $to:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        $crate::encode_bridge!(@encode $flavor: $bridge {
          $ty $([ $(const $g: usize),* ])? as $format {
            to: $to;
          }
        });

        $crate::encode_bridge!(@partial_encode $flavor: $bridge {
          $ty $([ $(const $g: usize),* ])? as $format {
            to: $to;
          }
        });
      )*
    )*
  };
  (@encode_impl $flavor:ty: $bridge:ty as $format:ty => $to:expr) => {
    fn encode(
      &self,
      context: &<$flavor as $crate::__private::Flavor>::Context,
      buf: &mut [::core::primitive::u8],
    ) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::Flavor>::EncodeError> {
      <$bridge as $crate::__private::Encode<$flavor, $format>>::encode(&$to(self), context, buf)
    }

    fn encoded_len(&self, context: &<$flavor as $crate::__private::Flavor>::Context,) -> ::core::primitive::usize {
      <$bridge as $crate::__private::Encode<$flavor, $format>>::encoded_len(&$to(self), context)
    }

    fn encode_length_delimited(
      &self,
      context: &<$flavor as $crate::__private::Flavor>::Context,
      buf: &mut [::core::primitive::u8],
    ) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::Flavor>::EncodeError> {
      <$bridge as $crate::__private::Encode<$flavor, $format>>::encode_length_delimited(&$to(self), context, buf)
    }

    fn encoded_length_delimited_len(&self, context: &<$flavor as $crate::__private::Flavor>::Context,) -> ::core::primitive::usize {
      <$bridge as $crate::__private::Encode<$flavor, $format>>::encoded_length_delimited_len(&$to(self), context)
    }
  };
  (@encode $flavor:ty: $(
    $bridge:ty {
      $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? as $format:ty {
        to: $to:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::Encode<$flavor, $format> for $ty {
          $crate::encode_bridge!(@encode_impl $flavor: $bridge as $format => $to);
        }
      )*
    )*
  };
  (@partial_encode_impl $flavor:ty: $bridge:ty as $format:ty => $to:expr) => {
    fn partial_encode(
      &self,
      context: &<$flavor as $crate::__private::Flavor>::Context,
      buf: &mut [::core::primitive::u8],
      selector: &Self::Selector,
    ) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::Flavor>::EncodeError> {
      <$bridge as $crate::__private::PartialEncode<$flavor, $format>>::partial_encode(&$to(self), context, buf, selector)
    }

    fn partial_encoded_len(
      &self,
      context: &<$flavor as $crate::__private::Flavor>::Context,
      selector: &Self::Selector,
    ) -> ::core::primitive::usize {
      <$bridge as $crate::__private::PartialEncode<$flavor, $format>>::partial_encoded_len(&$to(self), context, selector)
    }

    fn partial_encode_length_delimited(
      &self,
      context: &<$flavor as $crate::__private::Flavor>::Context,
      buf: &mut [::core::primitive::u8],
      selector: &Self::Selector,
    ) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::Flavor>::EncodeError> {
      <$bridge as $crate::__private::PartialEncode<$flavor, $format>>::partial_encode_length_delimited(&$to(self), context, buf, selector)
    }

    fn partial_encoded_length_delimited_len(
      &self,
      context: &<$flavor as $crate::__private::Flavor>::Context,
      selector: &Self::Selector,
    ) -> ::core::primitive::usize {
      <$bridge as $crate::__private::PartialEncode<$flavor, $format>>::partial_encoded_length_delimited_len(&$to(self), context, selector)
    }
  };
  (@partial_encode $flavor:ty: $(
    $bridge:ty {
      $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? as $format:ty {
        to: $to:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::PartialEncode<$flavor, $format> for $ty {
          $crate::encode_bridge!(@partial_encode_impl $flavor: $bridge as $format => $to);
        }
      )*
    )*
  };
}

/// A macro emits [`Decode`](super::decode::Decode) implementations for `Self`.
#[macro_export]
macro_rules! decode_bridge {
  ($(
    $flavor:ty: $bridge: ty {
      $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? as $format:ty {
        convert: $from:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        impl<'de, $( $(const $g: ::core::primitive::usize),* )?> $crate::__private::Decode<'de, $flavor, $format, Self> for $ty {
          $crate::decode_bridge!(@decode_impl $flavor: $from => $bridge as $format);
        }
      )*
    )*
  };
  (@decode_impl $flavor:ty: $from:expr => $bridge:ty as $format:ty) => {
    fn decode<UB>(
      context: &<$flavor as $crate::__private::Flavor>::Context,

      src: &'de [::core::primitive::u8],
    ) -> ::core::result::Result<(::core::primitive::usize, Self), <$flavor as $crate::__private::Flavor>::DecodeError>
    where
      Self: ::core::marker::Sized + 'de,
      UB: $crate::__private::Buffer<<$flavor as $crate::__private::Flavor>::Unknown<&'de [::core::primitive::u8]>> + 'de,
    {
      <$bridge as $crate::__private::Decode<$flavor, $format, $bridge>>::decode::<UB>(context, src).map(|(n, v)| (n, $from(v)))
    }

    fn decode_length_delimited<UB>(
      context: &<$flavor as $crate::__private::Flavor>::Context,
      src: &'de [::core::primitive::u8],
    ) -> ::core::result::Result<(::core::primitive::usize, Self), <$flavor as $crate::__private::Flavor>::DecodeError>
    where
      Self: ::core::marker::Sized + 'de,
      UB: $crate::__private::Buffer<<$flavor as $crate::__private::Flavor>::Unknown<&'de [::core::primitive::u8]>> + 'de,
    {
      <$bridge as $crate::__private::Decode<$flavor, $format, $bridge>>::decode_length_delimited::<UB>(context, src).map(|(n, v)| (n, $from(v)))
    }
  };
}

/// A macro emits [`DecodeOwned`](super::decode::DecodeOwned) implementations for `Self`.
#[macro_export]
macro_rules! decode_owned_bridge {
  ($(
    $flavor:ty: $bridge: ty {
      $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? as $format:ty {
        convert: $from:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::DecodeOwned<$flavor, $format, Self> for $ty
        where
          $bridge: $crate::__private::DecodeOwned<$flavor, $format, $bridge>,
        {
          $crate::decode_owned_bridge!(@decode_owned_impl $flavor: $from => $bridge as $format);
        }
      )*
    )*
  };
  (@decode_owned_impl $flavor:ty: $from:expr => $bridge:ty as $format:ty) => {
    fn decode_owned<B, UB>(
      ctx: &<$flavor as $crate::__private::Flavor>::Context,

      src: B,
    ) -> ::core::result::Result<(::core::primitive::usize, Self), <$flavor as $crate::__private::Flavor>::DecodeError>
    where
      Self: ::core::marker::Sized + 'static,
      B: $crate::__private::BytesBuffer + 'static,
      UB: $crate::__private::Buffer<<$flavor as $crate::__private::Flavor>::Unknown<B>> + 'static,
    {
      <$bridge as $crate::__private::DecodeOwned<$flavor, $format, $bridge>>::decode_owned::<B, UB>(ctx, src)
        .map(|(n, v)| (n, $from(v)))
    }

    fn decode_length_delimited_owned<B, UB>(
      ctx: &<$flavor as $crate::__private::Flavor>::Context,

      src: B,
    ) -> ::core::result::Result<(::core::primitive::usize, Self), <$flavor as $crate::__private::Flavor>::DecodeError>
    where
      Self: ::core::marker::Sized + 'static,
      B: $crate::__private::BytesBuffer + 'static,
      UB: $crate::__private::Buffer<<$flavor as $crate::__private::Flavor>::Unknown<B>> + 'static,
    {
      <$bridge as $crate::__private::DecodeOwned<$flavor, $format, $bridge>>::decode_length_delimited_owned::<B, UB>(ctx, src)
        .map(|(n, v)| (n, $from(v)))
    }
  };
}

/// A macro emits [`DecodeOwned`](super::decode::DecodeOwned) implementations for scalar, which will inherit
/// [`Decode`](super::decode::Decode) implementations for `Self`.
///
/// This macro must be used for scalar types that implements [`Copy`](::core::marker::Copy).
#[macro_export]
macro_rules! decode_owned_scalar {
  ($flavor:ty: $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? as $format:ty),+$(,)?) => {
    $(
      impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::DecodeOwned<$flavor, $format, $ty> for $ty
      {
        $crate::decode_owned_scalar!(@impl $flavor as $format);
      }
    )*
  };
  (@impl $flavor:ty as $format:ty) => {
    fn decode_owned<B, UB>(
      ctx: &<$flavor as $crate::__private::Flavor>::Context,
      src: B,
    ) -> ::core::result::Result<(::core::primitive::usize, Self), <$flavor as $crate::__private::Flavor>::DecodeError>
    where
      Self: ::core::marker::Sized + 'static,
      B: $crate::__private::BytesBuffer + 'static,
      UB: $crate::__private::Buffer<<$flavor as $crate::__private::Flavor>::Unknown<B>> + 'static,
    {
      <Self as $crate::__private::Decode<'_, $flavor, $format, Self>>::decode::<()>(ctx, src.as_bytes())
    }

    fn decode_length_delimited_owned<B, UB>(
      ctx: &<$flavor as $crate::__private::Flavor>::Context,
      src: B,
    ) -> ::core::result::Result<(::core::primitive::usize, Self), <$flavor as $crate::__private::Flavor>::DecodeError>
    where
      Self: ::core::marker::Sized + 'static,
      B: $crate::__private::BytesBuffer + 'static,
      UB: $crate::__private::Buffer<<$flavor as $crate::__private::Flavor>::Unknown<B>> + 'static,
    {
      <Self as $crate::__private::Decode<'_, $flavor, $format, Self>>::decode_length_delimited::<()>(ctx, src.as_bytes())
    }
  };
}

/// A macro emits [`Decode`](super::decode::Decode) implementations for `Self`.
#[macro_export]
macro_rules! try_decode_bridge {
  ($(
    $flavor:ty: $bridge: ty {
      $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? as $format:ty {
        convert: $from:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        $crate::try_decode_bridge!(@decode $flavor: $bridge {
          $ty $([ $(const $g: usize),* ])? as $format {
            try_from: $from;
          }
        });

        $crate::try_decode_owned_bridge!(@decode_owned $flavor: $bridge {
          $ty $([ $(const $g: usize),* ])? as $format {
            try_from: $from;
          }
        });
      )*
    )*
  };
  (@without_decode_owned $(
    $flavor:ty: $bridge: ty {
      $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? as $format:ty {
        convert: $from:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        $crate::try_decode_bridge!(@decode $flavor: $bridge {
          $ty $([ $(const $g: usize),* ])? as $format {
            try_from: $from;
          }
        });
      )*
    )*
  };
  (@decode_impl $flavor:ty: $from:expr => $bridge:ty as $format:ty) => {
    fn decode<UB>(
      context: &<$flavor as $crate::__private::Flavor>::Context,
      src: &'de [::core::primitive::u8],
    ) -> ::core::result::Result<(::core::primitive::usize, Self), <$flavor as $crate::__private::Flavor>::DecodeError>
    where
      Self: ::core::marker::Sized + 'de,
      UB: $crate::__private::Buffer<<$flavor as $crate::__private::Flavor>::Unknown<&'de [::core::primitive::u8]>> + 'de,
    {
      <$bridge as $crate::__private::Decode<$flavor, $format, $bridge>>::decode::<UB>(context, src).and_then(|(n, v)| $from(v).map(|v| (n, v)))
    }

    fn decode_length_delimited<UB>(
      context: &<$flavor as $crate::__private::Flavor>::Context,
      src: &'de [::core::primitive::u8],
    ) -> ::core::result::Result<(::core::primitive::usize, Self), <$flavor as $crate::__private::Flavor>::DecodeError>
    where
      Self: ::core::marker::Sized + 'de,
      UB: $crate::__private::Buffer<<$flavor as $crate::__private::Flavor>::Unknown<&'de [::core::primitive::u8]>> + 'de,
    {
      <$bridge as $crate::__private::Decode<$flavor, $format, $bridge>>::decode_length_delimited::<UB>(context, src).and_then(|(n, v)| $from(v).map(|v| (n, v)))
    }
  };
  (@decode $flavor:ty: $(
    $bridge:ty {
      $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? as $format:ty {
        try_from: $from:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        impl<'de> $crate::__private::Decode<'de, $flavor, $format, Self> for $ty {
          $crate::try_decode_bridge!(@decode_impl $flavor: $from => $bridge as $format);
        }
      )*
    )*
  };
}

/// A macro emits [`DecodeOwned`](super::decode::DecodeOwned) implementations for `Self`.
#[macro_export]
macro_rules! try_decode_owned_bridge {
  ($(
    $flavor:ty: $bridge: ty {
      $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? as $format:ty {
        convert: $from:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        $crate::try_decode_owned_bridge!(@decode_owned $flavor: $bridge {
          $ty $([ $(const $g: usize),* ])? as $format {
            try_from: $from;
          }
        });
      )*
    )*
  };
  (@decode_owned_impl $flavor:ty: $from:expr => $bridge:ty as $format:ty) => {
    fn decode_owned<B, UB>(
      ctx: &<$flavor as $crate::__private::Flavor>::Context,
      src: B,
    ) -> ::core::result::Result<(::core::primitive::usize, Self), <$flavor as $crate::__private::Flavor>::DecodeError>
    where
      Self: ::core::marker::Sized + 'static,
      B: $crate::__private::BytesBuffer + 'static,
      UB: $crate::__private::Buffer<<$flavor as $crate::__private::Flavor>::Unknown<B>> + 'static,
    {
      <$bridge as $crate::__private::DecodeOwned<$flavor, $format, $bridge>>::decode_owned::<B, UB>(ctx, src)
        .and_then(|(n, v)| $from(v).map(|v| (n, v)))
    }

    fn decode_length_delimited_owned<B, UB>(
      ctx: &<$flavor as $crate::__private::Flavor>::Context,
      src: B,
    ) -> ::core::result::Result<(::core::primitive::usize, Self), <$flavor as $crate::__private::Flavor>::DecodeError>
    where
      Self: ::core::marker::Sized + 'static,
      B: $crate::__private::BytesBuffer + 'static,
      UB: $crate::__private::Buffer<<$flavor as $crate::__private::Flavor>::Unknown<B>> + 'static,
    {
      <$bridge as $crate::__private::DecodeOwned<$flavor, $format, $bridge>>::decode_length_delimited_owned::<B, UB>(ctx, src)
        .and_then(|(n, v)| $from(v).map(|v| (n, v)))
    }
  };
  (@decode_owned $flavor:ty: $(
    $bridge:ty {
      $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? as $format:ty {
        try_from: $from:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        impl $crate::__private::DecodeOwned<$flavor, $format, Self> for $ty
        where
          $bridge: $crate::__private::DecodeOwned<$flavor, $format, $bridge>,
        {
          $crate::try_decode_owned_bridge!(@decode_owned_impl $flavor: $from => $bridge as $format);
        }
      )*
    )*
  };
}

/// A macro emits traits implementations for a zero-sized type.
///
/// ## Example
///
/// ```rust
/// use grost_proto::network_zst;
///
/// #[derive(Default)]
/// struct MyZst;
///
/// #[derive(Default)]
/// #[repr(transparent)]
/// struct MyZstTwo(MyZst);
///
/// network_zst!(MyZst, MyZstTwo);
/// ```
#[macro_export]
macro_rules! network_zst {
  ($($ty:ty), +$(,)?) => {
    $(
      const _: () = {
        const fn __assert<T>() where T: ::core::default::Default {
          assert!(::core::mem::size_of::<T>() == 0, "Not a zero-sized type");
        }

        __assert::<$ty>();
      };

      impl $crate::__private::PartialMessage<$crate::__private::flavors::Network, $crate::__private::flavors::network::Zst> for $ty {
        type UnknownBuffer<B> = ();

        type Encoded<'a>
          = Self
        where
          Self: Sized + 'a;

        type Borrowed<'a>
          = &'a Self
        where
          Self: 'a;

        type EncodedOwned
          = Self
        where
          Self: Sized + 'static;
      }

      impl $crate::__private::Message<$crate::__private::flavors::Network, $crate::__private::flavors::network::Zst> for $ty {
        type Partial = Self;

        type Encoded<'a>
          = Self
        where
          Self: Sized + 'a;

        type Borrowed<'a>
          = &'a Self
        where
          Self: 'a;

        type EncodedOwned
          = Self
        where
          Self: Sized + 'static;
      }

      impl $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Zst> for $ty {
        #[inline]
        fn encode(
          &self,
          _: &$crate::__private::flavors::network::Context,
          _: &mut [::core::primitive::u8]
        ) -> ::core::result::Result<::core::primitive::usize, $crate::__private::flavors::network::EncodeError> {
          ::core::result::Result::Ok(0)
        }

        #[inline]
        fn encode_length_delimited(
          &self,
          _: &$crate::__private::flavors::network::Context,
          _: &mut [::core::primitive::u8]
        ) -> ::core::result::Result<::core::primitive::usize, $crate::__private::flavors::network::EncodeError> {
          ::core::result::Result::Ok(0)
        }

        #[inline]
        fn encoded_len(
          &self,
          _: &$crate::__private::flavors::network::Context,
        ) -> ::core::primitive::usize {
          0
        }

        #[inline]
        fn encoded_length_delimited_len(
          &self,
          _: &$crate::__private::flavors::network::Context,
        ) -> ::core::primitive::usize {
          0
        }
      }

      impl<'de> $crate::__private::Decode<'de, $crate::__private::flavors::Network, $crate::__private::flavors::network::Zst, Self> for $ty {
        fn decode<UB>(
          _: &$crate::__private::flavors::network::Context,
          _: &'de [::core::primitive::u8],
        ) -> ::core::result::Result<(::core::primitive::usize, Self), <$crate::__private::flavors::Network as $crate::__private::flavors::Flavor>::DecodeError>
        where
          Self: ::core::marker::Sized + 'de,
          UB: $crate::__private::Buffer<$crate::__private::flavors::network::Unknown<&'de [::core::primitive::u8]>> + 'de,
        {
          ::core::result::Result::Ok((0, ::core::default::Default::default()))
        }

        fn decode_length_delimited<UB>(
          ctx: &$crate::__private::flavors::network::Context,
          src: &'de [::core::primitive::u8],
        ) -> ::core::result::Result<(::core::primitive::usize, Self), <$crate::__private::flavors::Network as $crate::__private::flavors::Flavor>::DecodeError>
        where
          Self: ::core::marker::Sized + 'de,
          UB: $crate::__private::Buffer<$crate::__private::flavors::network::Unknown<&'de [::core::primitive::u8]>> + 'de,
        {
          Self::decode::<UB>(ctx, src)
            .map(|(n, _)| (n, ::core::default::Default::default()))
        }
      }

      impl $crate::__private::DecodeOwned<$crate::__private::flavors::Network, $crate::__private::flavors::network::Zst, Self> for $ty {
        fn decode_owned<B, UB>(ctx: &$crate::__private::flavors::network::Context, src: B) -> ::core::result::Result<(::core::primitive::usize, Self), <$crate::__private::flavors::Network as $crate::__private::flavors::Flavor>::DecodeError>
        where
          Self: ::core::marker::Sized + 'static,
          B: $crate::__private::BytesBuffer + 'static,
          UB: $crate::__private::Buffer<$crate::__private::flavors::network::Unknown<B>> + 'static,
        {
          <Self as $crate::__private::Decode<'_, $crate::__private::flavors::Network, $crate::__private::flavors::network::Zst, Self>>::decode::<()>(ctx, src.as_bytes())
            .map(|(n, _)| (n, ::core::default::Default::default()))
        }

        fn decode_length_delimited_owned<B, UB>(ctx: &$crate::__private::flavors::network::Context, src: B) -> ::core::result::Result<(::core::primitive::usize, Self), <$crate::__private::flavors::Network as $crate::__private::flavors::Flavor>::DecodeError>
        where
          Self: ::core::marker::Sized + 'static,
          B: $crate::__private::BytesBuffer + 'static,
          UB: $crate::__private::Buffer<$crate::__private::flavors::network::Unknown<B>> + 'static,
        {
          <Self as $crate::__private::Decode<'_, $crate::__private::flavors::Network, $crate::__private::flavors::network::Zst, Self>>::decode_length_delimited::<()>(ctx, src.as_bytes())
            .map(|(n, _)| (n, ::core::default::Default::default()))
        }
      }

      $crate::selectable_scalar!($crate::__private::flavors::Network: $ty);
      $crate::partial_encode_scalar!($crate::__private::flavors::Network: $ty as $crate::__private::flavors::network::Zst);
    )*
  };
}

/// A macro emits traits implementations for `PhantomData<T>` like types.
///
/// ## Example
///
/// ```rust
/// use grost_proto::network_phantom;
/// use core::marker::PhantomData;
///
/// #[repr(transparent)]
/// struct MyPhantom<T: ?Sized>(PhantomData<T>);
///
/// impl<T: ?Sized> Default for MyPhantom<T> {
///   fn default() -> Self {
///     Self(PhantomData)
///   }
/// }
///
/// impl<T: ?Sized> Clone for MyPhantom<T> {
///  fn clone(&self) -> Self {
///    *self
///  }
/// }
///
/// impl<T: ?Sized> Copy for MyPhantom<T> {}
///
/// network_phantom!(MyPhantom<T>);
/// ```
#[macro_export]
macro_rules! network_phantom {
  ($($ty:ty),+$(,)?) => {
    $(
      impl<T: ?::core::marker::Sized> $crate::__private::Encode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Zst> for $ty {
        #[inline]
        fn encode(
          &self,
          _: &$crate::__private::flavors::network::Context,
          _: &mut [::core::primitive::u8],
        ) -> ::core::result::Result<::core::primitive::usize, $crate::__private::flavors::network::EncodeError> {
          ::core::result::Result::Ok(0)
        }

        #[inline]
        fn encoded_len(
          &self,
          _: &$crate::__private::flavors::network::Context,
        ) -> ::core::primitive::usize {
          0
        }

        #[inline]
        fn encode_length_delimited(
          &self,
          _: &$crate::__private::flavors::network::Context,
          _: &mut [::core::primitive::u8],
        ) -> ::core::result::Result<::core::primitive::usize, $crate::__private::flavors::network::EncodeError> {
          ::core::result::Result::Ok(0)
        }

        #[inline]
        fn encoded_length_delimited_len(
          &self,
          _: &$crate::__private::flavors::network::Context,
        ) -> ::core::primitive::usize {
          0
        }
      }

      impl<T: ?::core::marker::Sized> $crate::__private::Selectable<$crate::__private::flavors::Network> for $ty {
        type Selector = ::core::primitive::bool;
      }

      impl<T: ?::core::marker::Sized> $crate::__private::PartialEncode<$crate::__private::flavors::Network, $crate::__private::flavors::network::Zst> for $ty {
        $crate::partial_encode_scalar!(@impl $crate::__private::flavors::Network as $crate::__private::flavors::network::Zst);
      }

      impl<'de, T: ?::core::marker::Sized> $crate::__private::Decode<'de, $crate::__private::flavors::Network, $crate::__private::flavors::network::Zst, Self> for $ty {
        fn decode<UB>(
          _: &$crate::__private::flavors::network::Context,
          _: &'de [::core::primitive::u8],
        ) -> ::core::result::Result<(::core::primitive::usize, Self), <$crate::__private::flavors::Network as $crate::__private::flavors::Flavor>::DecodeError>
        where
          Self: ::core::marker::Sized + 'de,
          UB: $crate::__private::Buffer<$crate::__private::flavors::network::Unknown<&'de [::core::primitive::u8]>> + 'de,
        {
          ::core::result::Result::Ok((0, ::core::default::Default::default()))
        }

        fn decode_length_delimited<UB>(
          ctx: &$crate::__private::flavors::network::Context,
          src: &'de [::core::primitive::u8],
        ) -> ::core::result::Result<(::core::primitive::usize, Self), <$crate::__private::flavors::Network as $crate::__private::flavors::Flavor>::DecodeError>
        where
          Self: ::core::marker::Sized + 'de,
          UB: $crate::__private::Buffer<$crate::__private::flavors::network::Unknown<&'de [::core::primitive::u8]>> + 'de,
        {
          Self::decode::<UB>(ctx, src)
        }
      }

      impl<T: ?::core::marker::Sized + 'static> $crate::__private::DecodeOwned<$crate::__private::flavors::Network, $crate::__private::flavors::network::Zst, Self> for $ty {
        fn decode_owned<B, UB>(ctx: &$crate::__private::flavors::network::Context, src: B) -> ::core::result::Result<(::core::primitive::usize, Self), <$crate::__private::flavors::Network as $crate::__private::flavors::Flavor>::DecodeError>
        where
          Self: ::core::marker::Sized + 'static,
          B: $crate::__private::BytesBuffer + 'static,
          UB: $crate::__private::Buffer<$crate::__private::flavors::network::Unknown<B>> + 'static,
        {
          <Self as $crate::__private::Decode<'_, $crate::__private::flavors::Network, $crate::__private::flavors::network::Zst, Self>>::decode::<()>(ctx, src.as_bytes())
        }

        fn decode_length_delimited_owned<B, UB>(ctx: &$crate::__private::flavors::network::Context, src: B) -> ::core::result::Result<(::core::primitive::usize, Self), <$crate::__private::flavors::Network as $crate::__private::flavors::Flavor>::DecodeError>
        where
          Self: ::core::marker::Sized + 'static,
          B: $crate::__private::BytesBuffer + 'static,
          UB: $crate::__private::Buffer<$crate::__private::flavors::network::Unknown<B>> + 'static,
        {
          <Self as $crate::__private::Decode<'_, $crate::__private::flavors::Network, $crate::__private::flavors::network::Zst, Self>>::decode_length_delimited::<()>(ctx, src.as_bytes())
        }
      }

      impl<T: ?::core::marker::Sized> $crate::__private::PartialMessage<$crate::__private::flavors::Network, $crate::__private::flavors::network::Zst> for $ty {
        type UnknownBuffer<B> = ();

        type Encoded<'a>
          = Self
        where
          Self: Sized + 'a;

        type Borrowed<'a>
          = &'a Self
        where
          Self: 'a;

        type EncodedOwned
          = Self
        where
          Self: Sized + 'static;
      }

      impl<T: ?::core::marker::Sized> $crate::__private::Message<$crate::__private::flavors::Network, $crate::__private::flavors::network::Zst> for $ty {
        type Partial = Self;

        type Encoded<'a>
          = Self
        where
          Self: Sized + 'a;

        type Borrowed<'a>
          = &'a Self
        where
          Self: 'a;

        type EncodedOwned
          = Self
        where
          Self: Sized + 'static;
      }
    )*
  };
}
