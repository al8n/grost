/// A macro emits traits implementations for primitive types that implements [`varing::Varint`](varing::Varint) and [`Copy`](::core::marker::Copy).
#[macro_export]
macro_rules! network_varint {
  ($(
    $ty:ty $([ $( const $g:ident: usize), +$(,)? ])?
  ),+$(,)?) => {
    $($crate::message!($crate::__private::flavors::Network: $ty $([$(const $g: usize),*])?);)*
    $($crate::partial_encode_primitives!($crate::__private::flavors::Network: $ty $([ $(const $g: usize),* ])?);)*
    $($crate::network_varint!(@encode $ty $([ $(const $g: usize),* ])?);)*
    $($crate::network_varint!(@decode $ty $([ $(const $g: usize),* ])?);)*
    $($crate::network_varint!(@decode_owned $ty $([ $(const $g: usize),* ])?);)*
  };
  (@encode $ty:ty $([ $( const $g:ident: usize), +$(,)? ])?) => {
    impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::Encode<$crate::__private::flavors::Network> for $ty {
      $crate::network_varint!(@encode_impl);
    }
  };
  (@encode_impl) => {
    fn encode(&self, _: &$crate::__private::flavors::network::Context, wire_type: $crate::__private::flavors::network::WireType, buf: &mut [::core::primitive::u8]) -> ::core::result::Result<::core::primitive::usize, $crate::__private::flavors::network::EncodeError> {
      if let $crate::__private::flavors::network::WireType::Varint = wire_type {
        $crate::__private::varing::Varint::encode(self, buf).map_err(::core::convert::Into::into)
      } else {
        ::core::result::Result::Err($crate::__private::flavors::network::EncodeError::unsupported_wire_type(
          ::core::any::type_name::<Self>(),
          wire_type,
        ))
      }
    }

    fn encoded_len(&self, _: &$crate::__private::flavors::network::Context, wire_type: $crate::__private::flavors::network::WireType) -> ::core::result::Result<::core::primitive::usize, $crate::__private::flavors::network::EncodeError> {
      if let $crate::__private::flavors::network::WireType::Varint = wire_type {
        ::core::result::Result::Ok($crate::__private::varing::Varint::encoded_len(self))
      } else {
        ::core::result::Result::Err($crate::__private::flavors::network::EncodeError::unsupported_wire_type(
          ::core::any::type_name::<Self>(),
          wire_type,
        ))
      }
    }

    fn encode_length_delimited(&self, ctx: &$crate::__private::flavors::network::Context, wire_type: $crate::__private::flavors::network::WireType, buf: &mut [::core::primitive::u8]) -> ::core::result::Result<::core::primitive::usize, $crate::__private::flavors::network::EncodeError> {
      self.encode(ctx, wire_type, buf)
    }

    fn encoded_length_delimited_len(&self, ctx: &$crate::__private::flavors::network::Context, wire_type: $crate::__private::flavors::network::WireType,) -> ::core::result::Result<::core::primitive::usize, $crate::__private::flavors::network::EncodeError> {
      self.encoded_len(ctx, wire_type)
    }
  };
  (@decode $ty:ty $([ $( const $g:ident: usize), +$(,)? ])?) => {
    impl<'de, $($(const $g: ::core::primitive::usize),*)?> $crate::__private::Decode<'de, $crate::__private::flavors::Network, Self> for $ty {
      $crate::network_varint!(@decode_impl);
    }
  };
  (@decode_impl) => {
    fn decode<UB>(_: &$crate::__private::flavors::network::Context, wire_type: $crate::__private::flavors::network::WireType, src: &'de [::core::primitive::u8]) -> ::core::result::Result<(::core::primitive::usize, Self), <$crate::__private::flavors::Network as $crate::__private::flavors::Flavor>::DecodeError>
    where
      Self: ::core::marker::Sized + 'de,
      UB: $crate::__private::Buffer<$crate::__private::network::Unknown<&'de [::core::primitive::u8]>> + 'de,
    {
      if let $crate::__private::flavors::network::WireType::Varint = wire_type {
        $crate::__private::varing::Varint::decode(src).map_err(::core::convert::Into::into)
      } else {
        ::core::result::Result::Err($crate::__private::flavors::network::DecodeError::unsupported_wire_type(
          ::core::any::type_name::<Self>(),
          wire_type,
        ))
      }
    }

    fn decode_length_delimited<UB>(ctx: &$crate::__private::flavors::network::Context, wire_type: $crate::__private::flavors::network::WireType, src: &'de [::core::primitive::u8]) -> ::core::result::Result<(::core::primitive::usize, Self), <$crate::__private::flavors::Network as $crate::__private::flavors::Flavor>::DecodeError>
    where
      Self: ::core::marker::Sized + 'de,
      UB: $crate::__private::Buffer<$crate::__private::network::Unknown<&'de [::core::primitive::u8]>> + 'de,
    {
      Self::decode::<UB>(ctx, wire_type, src)
    }
  };
  (@decode_owned $ty:ty $([ $( const $g:ident: usize), +$(,)? ])?) => {
    impl$(<$(const $g: ::core::primitive::usize),*>)? $crate::__private::DecodeOwned<$crate::__private::flavors::Network, Self> for $ty {
      $crate::network_varint!(@decode_owned_impl);
    }
  };
  (@decode_owned_impl) => {
    fn decode_owned<B, UB>(ctx: &$crate::__private::flavors::network::Context, wire_type: $crate::__private::flavors::network::WireType, src: B) -> ::core::result::Result<(::core::primitive::usize, Self), <$crate::__private::flavors::Network as $crate::__private::flavors::Flavor>::DecodeError>
    where
      Self: ::core::marker::Sized + 'static,
      B: $crate::__private::BytesBuffer + 'static,
      UB: $crate::__private::Buffer<$crate::__private::network::Unknown<B>> + 'static,
    {
      <Self as $crate::__private::Decode<'_, $crate::__private::flavors::Network, Self>>::decode::<()>(ctx, wire_type, $crate::__private::BytesBuffer::as_bytes(&src))
    }

    fn decode_length_delimited_owned<B, UB>(ctx: &$crate::__private::flavors::network::Context, wire_type: $crate::__private::flavors::network::WireType, src: B) -> ::core::result::Result<(::core::primitive::usize, Self), <$crate::__private::flavors::Network as $crate::__private::flavors::Flavor>::DecodeError>
    where
      Self: ::core::marker::Sized + 'static,
      B: $crate::__private::BytesBuffer + 'static,
      UB: $crate::__private::Buffer<$crate::__private::network::Unknown<B>> + 'static,
    {
      Self::decode_owned::<B, UB>(ctx, wire_type, src)
    }
  };
}

/// A macro emits [`PartialEncode`](super::PartialEncode) implementations for primitive types.
#[macro_export]
macro_rules! partial_encode_primitives {
  (@impl $flavor:ty) => {
    type Selection = ();

    fn partial_encode(&self, context: &<$flavor as $crate::__private::Flavor>::Context, wire_type: <$flavor as $crate::__private::Flavor>::WireType, buf: &mut [::core::primitive::u8],  _: &Self::Selection) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::Flavor>::EncodeError> {
      <Self as $crate::__private::Encode<$flavor>>::encode(self, context, wire_type, buf)
    }

    fn partial_encoded_len(&self, context: &<$flavor as $crate::__private::Flavor>::Context, wire_type: <$flavor as $crate::__private::Flavor>::WireType, _: &Self::Selection) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::Flavor>::EncodeError> {
      <Self as $crate::__private::Encode<$flavor>>::encoded_len(self, context, wire_type)
    }

    fn partial_encode_length_delimited(&self, context: &<$flavor as $crate::__private::Flavor>::Context, wire_type: <$flavor as $crate::__private::Flavor>::WireType, buf: &mut [::core::primitive::u8],  _: &Self::Selection) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::Flavor>::EncodeError> {
      <Self as $crate::__private::Encode<$flavor>>::encode_length_delimited(self, context, wire_type, buf)
    }

    fn partial_encoded_length_delimited_len(&self, context: &<$flavor as $crate::__private::Flavor>::Context, wire_type: <$flavor as $crate::__private::Flavor>::WireType, _: &Self::Selection) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::Flavor>::EncodeError> {
      <Self as $crate::__private::Encode<$flavor>>::encoded_length_delimited_len(self, context, wire_type)
    }
  };
  ($flavor:ty: $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])?),+$(,)?) => {
    $(
      impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::PartialEncode<$flavor> for $ty {
        $crate::partial_encode_primitives!(@impl $flavor);
      }
    )*
  };
}

/// A macro emits [`TypeRef`](super::TypeRef) implementations for `Self`
#[macro_export]
macro_rules! type_ref {
  ($flavor:ty: $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])?),+$(,)?) => {
    $(
      impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::TypeRef<$flavor, Self> for $ty {
        $crate::type_ref!(@copy_impl $flavor);
      }
    )*
  };
  (@clone $flavor:ty: $ty:ty $($([ $( const $g:ident: usize), +$(,)? ])?),+$(,)?) => {
    $(
      impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::TypeRef<$flavor, Self> for $ty {
        $crate::type_ref!(@clone_impl $flavor);
      }
    )*
  };
  (@copy_impl $flavor:ty) => {
    fn to(&self) -> ::core::result::Result<Self, <$flavor as $crate::__private::Flavor>::DecodeError> {
      ::core::result::Result::Ok(*self)
    }
  };
  (@clone_impl $flavor:ty) => {
    fn to(&self) -> ::core::result::Result<Self, <$flavor as $crate::__private::Flavor>::DecodeError> {
      ::core::result::Result::Ok(::core::clone::Clone::clone(self))
    }
  }
}

/// A macro emits [`TypeOwned`](super::TypeOwned) implementations for `Self`
#[macro_export]
macro_rules! type_owned {
  ($flavor:ty: $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])?),+$(,)?) => {
    $(
      impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::TypeOwned<$flavor, Self> for $ty {
        $crate::type_ref!(@copy_impl $flavor);
      }
    )*
  };
  (@clone $flavor:ty: $ty:ty $($([ $( const $g:ident: usize), +$(,)? ])?),+$(,)?) => {
    $(
      impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::TypeOwned<$flavor, Self> for $ty {
        $crate::type_ref!(@clone_impl);
      }
    )*
  };
  (@copy_impl $flavor:ty) => {
    fn to(&self) -> ::core::result::Result<Self, <$flavor as $crate::__private::Flavor>::DecodeError> {
      ::core::result::Result::Ok(*self)
    }
  };
  (@clone_impl $flavor:ty) => {
    fn to(&self) -> ::core::result::Result<Self, <$flavor as $crate::__private::Flavor>::DecodeError> {
      ::core::result::Result::Ok(::core::clone::Clone::clone(self))
    }
  }
}

/// A macro emits [`IntoTarget`](super::IntoTarget) implementations for `Self`
#[macro_export]
macro_rules! into_target {
  ($flavor:ty: $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])? ),+$(,)?) => {
    $(
      impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::IntoTarget<$flavor, Self> for $ty {
        $crate::into_target!(@impl $flavor);
      }
    )*
  };
  (@impl $flavor:ty) => {
    fn into_target(self) -> ::core::result::Result<Self, <$flavor as $crate::__private::Flavor>::DecodeError> {
      ::core::result::Result::Ok(self)
    }
  }
}

/// A macro emits convertion traits implementations for `Self`
#[macro_export]
macro_rules! conversion {
  ($flavor:ty: $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])?),+$(,)?) => {
    $crate::type_ref!($flavor: $($ty $([ $(const $g: usize),* ])?),+);
    $crate::type_owned!($flavor: $($ty $([ $(const $g: usize),* ])?),+);
    $crate::into_target!($flavor: $($ty $([ $(const $g: usize),* ])?),+);
  };
  (@clone $flavor:ty: $ty:ty $($([ $( const $g:ident: usize), +$(,)? ])?),+$(,)?) => {
    $crate::type_ref!(@clone $flavor: $($ty $([ $(const $g: usize),* ])?),+);
    $crate::type_owned!(@clone $flavor: $($ty $([ $(const $g: usize),* ])?),+);
    $crate::into_target!($($ty $flavor: $([ $(const $g: usize),* ])?),+);
  };
}

/// A macro emits [`Message`](super::Message) implementations for `Self`.
///
/// **NB:** this macro can only be used for types that implements [`Copy`](::core::marker::Copy).
#[macro_export]
macro_rules! message {
  ($flavor:ty: $($ty:ty $([ $( const $g:ident: usize), +$(,)? ])?),+$(,)?) => {
    $(
      impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::Message<$flavor> for $ty {
        $crate::message!(@impl);
      }

      impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::PartialMessage<$flavor> for $ty {
        $crate::message!(@impl_partial);
      }
    )*

    $($crate::conversion!($flavor: $ty $([ $(const $g: usize),* ])? );)*
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
      $($ty:ty {
        from: $from:expr;
        to: $to:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $crate::encode_bridge!($flavor: $bridge {
        $($ty {
          convert: $to;
        })*
      });

      $crate::decode_bridge!($flavor: $bridge {
        $($ty {
          convert: $from;
        })*
      });

      $crate::decode_owned_bridge!($flavor: $bridge {
        $($ty {
          convert: $from;
        })*
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
      $($ty:ty {
        try_from: $from:expr;
        to: $to:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $crate::encode_bridge!($flavor: $bridge {
        $($ty {
          convert: $to;
        })*
      });

      $crate::try_decode_bridge!($flavor: $bridge {
        $($ty {
          convert: $from;
        })*
      });

      $crate::try_decode_owned_bridge!($flavor: $bridge {
        $($ty {
          convert: $from;
        })*
      });
    )*
  };
}

/// A macro emits [`Encode`](super::encode::Encode) implementations for `Self`.
#[macro_export]
macro_rules! encode_bridge {
  ($(
    $flavor:ty: $bridge: ty {
      $($ty:ty {
        convert: $to:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        $crate::encode_bridge!(@encode $flavor: $bridge {
          $ty {
            to: $to;
          }
        });

        $crate::encode_bridge!(@partial_encode $flavor: $bridge {
          $ty {
            to: $to;
          }
        });
      )*
    )*
  };
  (@encode_impl $flavor:ty: $bridge:ty => $to:expr) => {
    fn encode(
      &self,
      context: &<$flavor as $crate::__private::Flavor>::Context,
      wire_type: <$flavor as $crate::__private::Flavor>::WireType,
      buf: &mut [::core::primitive::u8],
    ) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::Flavor>::EncodeError> {
      <$bridge as $crate::__private::Encode<$flavor>>::encode(&$to(self), context, wire_type, buf)
    }

    fn encoded_len(&self, context: &<$flavor as $crate::__private::Flavor>::Context, wire_type: <$flavor as $crate::__private::Flavor>::WireType,) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::Flavor>::EncodeError> {
      <$bridge as $crate::__private::Encode<$flavor>>::encoded_len(&$to(self), context, wire_type)
    }

    fn encode_length_delimited(
      &self,
      context: &<$flavor as $crate::__private::Flavor>::Context,
      wire_type: <$flavor as $crate::__private::Flavor>::WireType,
      buf: &mut [::core::primitive::u8],
    ) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::Flavor>::EncodeError> {
      <$bridge as $crate::__private::Encode<$flavor>>::encode_length_delimited(&$to(self), context, wire_type, buf)
    }

    fn encoded_length_delimited_len(&self, context: &<$flavor as $crate::__private::Flavor>::Context, wire_type: <$flavor as $crate::__private::Flavor>::WireType,) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::Flavor>::EncodeError> {
      <$bridge as $crate::__private::Encode<$flavor>>::encoded_length_delimited_len(&$to(self), context, wire_type)
    }
  };
  (@encode $flavor:ty: $(
    $bridge:ty {
      $($ty:ty {
        to: $to:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        impl $crate::__private::Encode<$flavor> for $ty {
          $crate::encode_bridge!(@encode_impl $flavor: $bridge => $to);
        }
      )*
    )*
  };
  (@partial_encode_impl $flavor:ty: $bridge:ty => $to:expr) => {
    fn partial_encode(
      &self,
      context: &<$flavor as $crate::__private::Flavor>::Context,
      wire_type: <$flavor as $crate::__private::Flavor>::WireType,
      buf: &mut [::core::primitive::u8], selection: &Self::Selection,
    ) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::Flavor>::EncodeError> {
      <$bridge as $crate::__private::PartialEncode<$flavor>>::partial_encode(&$to(self), context, wire_type, buf, selection)
    }

    fn partial_encoded_len(
      &self,
      context: &<$flavor as $crate::__private::Flavor>::Context,
      wire_type: <$flavor as $crate::__private::Flavor>::WireType,
      selection: &Self::Selection,
    ) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::Flavor>::EncodeError> {
      <$bridge as $crate::__private::PartialEncode<$flavor>>::partial_encoded_len(&$to(self), context, wire_type, selection)
    }

    fn partial_encode_length_delimited(
      &self,
      context: &<$flavor as $crate::__private::Flavor>::Context,
      wire_type: <$flavor as $crate::__private::Flavor>::WireType,
      buf: &mut [::core::primitive::u8], selection: &Self::Selection,
    ) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::Flavor>::EncodeError> {
      <$bridge as $crate::__private::PartialEncode<$flavor>>::partial_encode_length_delimited(&$to(self), context, wire_type, buf, selection)
    }

    fn partial_encoded_length_delimited_len(
      &self,
      context: &<$flavor as $crate::__private::Flavor>::Context,
      wire_type: <$flavor as $crate::__private::Flavor>::WireType,
      selection: &Self::Selection,
    ) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::Flavor>::EncodeError> {
      <$bridge as $crate::__private::PartialEncode<$flavor>>::partial_encoded_length_delimited_len(&$to(self), context, wire_type, selection)
    }
  };
  (@partial_encode $flavor:ty: $(
    $bridge:ty {
      $($ty:ty {
        to: $to:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        impl $crate::__private::PartialEncode<$flavor> for $ty {
          type Selection = <$bridge as $crate::__private::PartialEncode<$flavor>>::Selection;

          $crate::encode_bridge!(@partial_encode_impl $flavor: $bridge => $to);
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
      $($ty:ty {
        convert: $from:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        impl<'de> $crate::__private::Decode<'de, $flavor, Self> for $ty {
          $crate::decode_bridge!(@decode_impl $flavor: $from => $bridge);
        }
      )*
    )*
  };
  (@decode_impl $flavor:ty: $from:expr => $bridge:ty) => {
    fn decode<UB>(
      context: &<$flavor as $crate::__private::Flavor>::Context,
      wire_type: <$flavor as $crate::__private::Flavor>::WireType,
      src: &'de [::core::primitive::u8],
    ) -> ::core::result::Result<(::core::primitive::usize, Self), <$flavor as $crate::__private::Flavor>::DecodeError>
    where
      Self: ::core::marker::Sized + 'de,
      UB: $crate::__private::Buffer<<$flavor as $crate::__private::Flavor>::Unknown<&'de [::core::primitive::u8]>> + 'de,
    {
      <$bridge as $crate::__private::Decode<$flavor, $bridge>>::decode::<UB>(context, wire_type, src).map(|(n, v)| (n, $from(v)))
    }

    fn decode_length_delimited<UB>(
      context: &<$flavor as $crate::__private::Flavor>::Context,
      wire_type: <$flavor as $crate::__private::Flavor>::WireType,
      src: &'de [::core::primitive::u8],
    ) -> ::core::result::Result<(::core::primitive::usize, Self), <$flavor as $crate::__private::Flavor>::DecodeError>
    where
      Self: ::core::marker::Sized + 'de,
      UB: $crate::__private::Buffer<<$flavor as $crate::__private::Flavor>::Unknown<&'de [::core::primitive::u8]>> + 'de,
    {
      <$bridge as $crate::__private::Decode<$flavor, $bridge>>::decode_length_delimited::<UB>(context, wire_type, src).map(|(n, v)| (n, $from(v)))
    }
  };
}

/// A macro emits [`DecodeOwned`](super::decode::DecodeOwned) implementations for `Self`.
#[macro_export]
macro_rules! decode_owned_bridge {
  ($(
    $flavor:ty: $bridge: ty {
      $($ty:ty {
        convert: $from:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        impl $crate::__private::DecodeOwned<$flavor, Self> for $ty
        where
          $bridge: $crate::__private::DecodeOwned<$flavor, $bridge>,
        {
          $crate::decode_owned_bridge!(@decode_owned_impl $flavor: $from => $bridge);
        }
      )*
    )*
  };
  (@decode_owned_impl $flavor:ty: $from:expr => $bridge:ty) => {
    fn decode_owned<B, UB>(
      ctx: &<$flavor as $crate::__private::Flavor>::Context,
      wire_type: <$flavor as $crate::__private::Flavor>::WireType,
      src: B,
    ) -> ::core::result::Result<(::core::primitive::usize, Self), <$flavor as $crate::__private::Flavor>::DecodeError>
    where
      Self: ::core::marker::Sized + 'static,
      B: $crate::__private::BytesBuffer + 'static,
      UB: $crate::__private::Buffer<<$flavor as $crate::__private::Flavor>::Unknown<B>> + 'static,
    {
      <$bridge as $crate::__private::DecodeOwned<$flavor, $bridge>>::decode_owned::<B, UB>(ctx, wire_type, src)
        .map(|(n, v)| (n, $from(v)))
    }

    fn decode_length_delimited_owned<B, UB>(
      ctx: &<$flavor as $crate::__private::Flavor>::Context,
      wire_type: <$flavor as $crate::__private::Flavor>::WireType,
      src: B,
    ) -> ::core::result::Result<(::core::primitive::usize, Self), <$flavor as $crate::__private::Flavor>::DecodeError>
    where
      Self: ::core::marker::Sized + 'static,
      B: $crate::__private::BytesBuffer + 'static,
      UB: $crate::__private::Buffer<<$flavor as $crate::__private::Flavor>::Unknown<B>> + 'static,
    {
      <$bridge as $crate::__private::DecodeOwned<$flavor, $bridge>>::decode_length_delimited_owned::<B, UB>(ctx, wire_type, src)
        .map(|(n, v)| (n, $from(v)))
    }
  };
}

/// A macro emits [`Decode`](super::decode::Decode) implementations for `Self`.
#[macro_export]
macro_rules! try_decode_bridge {
  ($(
    $flavor:ty: $bridge: ty {
      $($ty:ty {
        convert: $from:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        $crate::try_decode_bridge!(@decode $flavor: $bridge {
          $ty {
            try_from: $from;
          }
        });
      )*
    )*
  };
  (@decode_impl $flavor:ty: $from:expr => $bridge:ty) => {
    fn decode<UB>(
      context: &<$flavor as $crate::__private::Flavor>::Context,
      wire_type: <$flavor as $crate::__private::Flavor>::WireType,
      src: &'de [::core::primitive::u8],
    ) -> ::core::result::Result<(::core::primitive::usize, Self), <$flavor as $crate::__private::Flavor>::DecodeError>
    where
      Self: ::core::marker::Sized + 'de,
      UB: $crate::__private::Buffer<<$flavor as $crate::__private::Flavor>::Unknown<&'de [::core::primitive::u8]>> + 'de,
    {
      <$bridge as $crate::__private::Decode<$flavor, $bridge>>::decode::<UB>(context, wire_type, src).and_then(|(n, v)| $from(v).map(|v| (n, v)))
    }

    fn decode_length_delimited<UB>(
      context: &<$flavor as $crate::__private::Flavor>::Context,
      wire_type: <$flavor as $crate::__private::Flavor>::WireType,
      src: &'de [::core::primitive::u8],
    ) -> ::core::result::Result<(::core::primitive::usize, Self), <$flavor as $crate::__private::Flavor>::DecodeError>
    where
      Self: ::core::marker::Sized + 'de,
      UB: $crate::__private::Buffer<<$flavor as $crate::__private::Flavor>::Unknown<&'de [::core::primitive::u8]>> + 'de,
    {
      <$bridge as $crate::__private::Decode<$flavor, $bridge>>::decode_length_delimited::<UB>(context, wire_type, src).and_then(|(n, v)| $from(v).map(|v| (n, v)))
    }
  };
  (@decode $flavor:ty: $(
    $bridge:ty {
      $($ty:ty {
        try_from: $from:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        impl<'de> $crate::__private::Decode<'de, $flavor, Self> for $ty {
          $crate::try_decode_bridge!(@decode_impl $flavor: $from => $bridge);
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
      $($ty:ty {
        convert: $from:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        $crate::try_decode_owned_bridge!(@decode_owned $flavor: $bridge {
          $ty {
            try_from: $from;
          }
        });
      )*
    )*
  };
  (@decode_owned_impl $flavor:ty: $from:expr => $bridge:ty) => {
    fn decode_owned<B, UB>(
      ctx: &<$flavor as $crate::__private::Flavor>::Context,
      wire_type: <$flavor as $crate::__private::Flavor>::WireType,
      src: B,
    ) -> ::core::result::Result<(::core::primitive::usize, Self), <$flavor as $crate::__private::Flavor>::DecodeError>
    where
      Self: ::core::marker::Sized + 'static,
      B: $crate::__private::BytesBuffer + 'static,
      UB: $crate::__private::Buffer<<$flavor as $crate::__private::Flavor>::Unknown<B>> + 'static,
    {
      <$bridge as $crate::__private::DecodeOwned<$flavor, $bridge>>::decode_owned::<B, UB>(ctx, wire_type, src)
        .and_then(|(n, v)| $from(v).map(|v| (n, v)))
    }

    fn decode_length_delimited_owned<B, UB>(
      ctx: &<$flavor as $crate::__private::Flavor>::Context,
      wire_type: <$flavor as $crate::__private::Flavor>::WireType,
      src: B,
    ) -> ::core::result::Result<(::core::primitive::usize, Self), <$flavor as $crate::__private::Flavor>::DecodeError>
    where
      Self: ::core::marker::Sized + 'static,
      B: $crate::__private::BytesBuffer + 'static,
      UB: $crate::__private::Buffer<<$flavor as $crate::__private::Flavor>::Unknown<B>> + 'static,
    {
      <$bridge as $crate::__private::DecodeOwned<$flavor, $bridge>>::decode_length_delimited_owned::<B, UB>(ctx, wire_type, src)
        .and_then(|(n, v)| $from(v).map(|v| (n, v)))
    }
  };
  (@decode_owned $flavor:ty: $(
    $bridge:ty {
      $($ty:ty {
        try_from: $from:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        impl $crate::__private::DecodeOwned<$flavor, Self> for $ty
        where
          $bridge: $crate::__private::DecodeOwned<$flavor, $bridge>,
        {
          $crate::try_decode_owned_bridge!(@decode_owned_impl $flavor: $from => $bridge);
        }
      )*
    )*
  };
}

#[macro_export]
macro_rules! bytes_bridge {
  ($flavor:ty: $($ty:ty $([const $g:ident: usize])? {
    from_slice: $from_bytes: expr;
    as_slice: $to_bytes: expr;

    type EncodedOwned = $owned_ty:ty {
      from_ref: $from_ref: expr;
      from: $from: expr;
    } $(;)?
  }), +$(,)?) => {
    $(
      $crate::encode_bridge!(
        $flavor: [::core::primitive::u8] {
          $ty {
            convert: $to_bytes;
          },
        },
      );

      $crate::decode_bridge!(
        $flavor: &'de [::core::primitive::u8] {
          $ty {
            convert: $from_bytes;
          },
        },
      );

      impl$(<const $g: ::core::primitive::usize>)? $crate::__private::IntoTarget<$flavor, $ty> for &[::core::primitive::u8] {
        $crate::bytes_bridge!(@into_target_impl $flavor: $ty: $from_bytes);
      }

      impl$(<const $g: ::core::primitive::usize>)? $crate::__private::TypeRef<$flavor, $ty> for &[::core::primitive::u8] {
        $crate::bytes_bridge!(@slice_to_impl $flavor: $ty: $from_bytes);
      }

      impl$(<const $g: ::core::primitive::usize>)? $crate::__private::TypeOwned<$flavor, $ty> for &[::core::primitive::u8] {
        $crate::bytes_bridge!(@slice_to_impl $flavor: $ty: $from_bytes);
      }

      impl$(<const $g: ::core::primitive::usize>)? $crate::__private::IntoTarget<$flavor, $ty> for $owned_ty {
        $crate::bytes_bridge!(@into_target_impl $flavor: $ty: $from_ref);
      }

      impl$(<const $g: ::core::primitive::usize>)? $crate::__private::TypeOwned<$flavor, $ty> for $owned_ty {
        $crate::bytes_bridge!(@to_impl $flavor: $ty: $from);
      }
    )*
  };
  (@into_target_impl $flavor:ty: $ty:ty: $from:expr) => {
    fn into_target(self) -> ::core::result::Result<$ty, <$flavor as $crate::__private::Flavor>::DecodeError> {
      ::core::result::Result::Ok($from(self))
    }
  };
  (@slice_to_impl $flavor:ty: $ty:ty: $from:expr) => {
    fn to(&self) -> ::core::result::Result<$ty, <$flavor as $crate::__private::Flavor>::DecodeError> {
      ::core::result::Result::Ok($from(*self))
    }
  };
  (@to_impl $flavor:ty: $ty:ty: $from:expr) => {
    fn to(&self) -> ::core::result::Result<$ty, <$flavor as $crate::__private::Flavor>::DecodeError> {
      ::core::result::Result::Ok($from(self))
    }
  };
}
