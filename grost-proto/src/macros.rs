/// A macro emits traits implementations for primitive types that implements [`varing::Varint`](varing::Varint) and [`Copy`](::core::marker::Copy).
#[macro_export]
macro_rules! groto_varint {
  (@scalar $(
    $ty:ty $([ $( const $g:ident: $gty:ty), +$(,)? ])?
  ),+$(,)?) => {
    $(
      $crate::__default_wire_format__!(@impl scalar<$crate::__private::flavors::Groto>: $ty $([$(const $g: $gty),*])? as $crate::__private::flavors::groto::Varint);
    )*

    $crate::varint!(
      @common $crate::__private::flavors::Groto:$crate::__private::flavors::groto::Varint {
        $(
          $ty $([$(const $g: $gty),*])?
        ),*
      }
    );

    $($crate::partial_identity!(@scalar $crate::__private::flavors::Groto: $ty $([ $(const $g: $gty),* ])?);)*
  };
}

/// A macro emits traits implementations for primitive types that implements [`varing::Varint`](varing::Varint) and [`Copy`](::core::marker::Copy).
#[macro_export]
macro_rules! varint {
  (@scalar $flavor:ty:$wf:ty {
    $(
      $ty:ty $([ $( const $g:ident: $gty:ty), +$(,)? ])?
    ),+$(,)?
  }) => {
    $(
      $crate::__default_wire_format__!(@impl scalar<$flavor>: $ty $([$(const $g: $gty),*])? as $wf);
    )*

    $crate::varint!(
      @common $flavor:$wf {
        $(
          $ty $([$(const $g: $gty),*])?
        ),*
      }
    );

    $($crate::identity_transform!($flavor {
      $ty $([ $(const $g: $gty),* ])? as $wf,
    });)*
  };
  (@enum $flavor:ty:$wf:ty {
    $(
      $ty:ty $([ $( const $g:ident: $gty:ty), +$(,)? ])?
    ),+$(,)?
  }) => {
    $(
      $crate::__default_wire_format__!(@impl enum<$flavor>: $ty $([$(const $g: $gty),*])? as $wf);
    )*

    $crate::varint!(
      @common $flavor:$wf {
        $(
          $ty $([$(const $g: $gty),*])?
        ),*
      }
    );
  };
  (@common $flavor:ty:$wf:ty {
    $(
      $ty:ty $([ $( const $g:ident: $gty:ty), +$(,)? ])?
    ),+$(,)?
  }) => {
    $($crate::selectable!(@scalar $flavor: $ty $([ $(const $g: $gty),* ])?);)*
    $($crate::ref_state!(@scalar &'a $flavor: $ty $([ $(const $g: $gty),* ])? as $wf);)*
    $($crate::partial_ref_state!(@scalar &'a $flavor: $ty $([ $(const $g: $gty),* ])? as $wf);)*
    $($crate::partial_state!(@scalar $flavor: $ty $([ $(const $g: $gty),* ])?);)*
    $($crate::flatten_state!($ty $([ $(const $g: $gty),* ])?);)*

    $($crate::partial_encode_scalar!($flavor: $ty $([ $(const $g: $gty),* ])? as $wf);)*
    $($crate::varint!(@encode $flavor:$wf:$ty $([ $(const $g: $gty),* ])?);)*
    $($crate::varint!(@decode $flavor:$wf:$ty $([ $(const $g: $gty),* ])?);)*

    // $(
    //   impl $(< $(const $g: ::core::primitive::usize),* > )? $crate::__private::convert::PartialTransform<$ty, ::core::option::Option<$ty>, $wf, $flavor> for $ty {
    //     fn partial_transform(input: Self, selector: &bool) -> ::core::result::Result<::core::option::Option<$ty>, <$flavor as $crate::__private::flavors::Flavor>::Error>
    //     {
    //       if $crate::__private::selection::Selector::<$flavor>::is_empty(selector) {
    //         return ::core::result::Result::Ok(::core::option::Option::None);
    //       }

    //       ::core::result::Result::Ok(::core::option::Option::Some(input))
    //     }
    //   }

    //   impl $( < $(const $g: ::core::primitive::usize),* > )? $crate::__private::convert::PartialTransform<::core::option::Option<Self>, ::core::option::Option<Self>, $crate::__private::flavors::Nullable<$wf>, $flavor,> for $ty {
    //     fn partial_transform(input: ::core::option::Option<Self>, selector: &<Self as $crate::__private::selection::Selectable<$flavor>>::Selector) -> ::core::result::Result<::core::option::Option<Self>, <$flavor as $crate::__private::flavors::Flavor>::Error>
    //     {
    //       match input {
    //         ::core::option::Option::None => ::core::result::Result::Ok(::core::option::Option::None),
    //         ::core::option::Option::Some(input) => {
    //           if $crate::__private::selection::Selector::<$flavor>::is_empty(selector) {
    //             return ::core::result::Result::Ok(::core::option::Option::None);
    //           }

    //           ::core::result::Result::Ok(::core::option::Option::Some(input))
    //         }
    //       }
    //     }
    //   }
    // )*
  };
  (@encode $flavor:ty:$wf:ty:$ty:ty $([ $( const $g:ident: $gty:ty), +$(,)? ])?) => {
    impl $( < $(const $g: $gty),* > )? $crate::__private::Encode<$wf, $flavor> for $ty {
      $crate::varint!(@encode_impl $flavor:$wf);
    }
  };
  (@encode_impl $flavor:ty:$wf:ty) => {
    fn encode_raw<WB>(&self, _: &<$flavor as $crate::__private::flavors::Flavor>::Context, buf: &mut WB) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::flavors::Flavor>::Error>
    where
      WB: $crate::__private::buffer::WriteBuf + ?::core::marker::Sized,
    {
      $crate::__private::varing::Varint::encode(self, buf.as_mut_slice()).map_err(::core::convert::Into::into)
    }

    fn encoded_raw_len(&self, _: &<$flavor as $crate::__private::flavors::Flavor>::Context,) -> ::core::primitive::usize {
      $crate::__private::varing::Varint::encoded_len(self)
    }

    fn encode<WB>(&self, _: &<$flavor as $crate::__private::flavors::Flavor>::Context, buf: &mut WB) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::flavors::Flavor>::Error>
    where
      WB: $crate::__private::buffer::WriteBuf + ?::core::marker::Sized,
    {
      $crate::__private::varing::Varint::encode(self, buf.as_mut_slice()).map_err(::core::convert::Into::into)
    }

    fn encoded_len(&self, _: &<$flavor as $crate::__private::flavors::Flavor>::Context,) -> ::core::primitive::usize {
      $crate::__private::varing::Varint::encoded_len(self)
    }

    fn encode_length_delimited<WB>(&self, ctx: &<$flavor as $crate::__private::flavors::Flavor>::Context, buf: &mut WB) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::flavors::Flavor>::Error>
    where
      WB: $crate::__private::buffer::WriteBuf + ?::core::marker::Sized,
    {
      <Self as $crate::__private::Encode<$wf, $flavor>>::encode(self, ctx, buf)
    }

    fn encoded_length_delimited_len(&self, ctx: &<$flavor as $crate::__private::flavors::Flavor>::Context,) -> ::core::primitive::usize {
      <Self as $crate::__private::Encode<$wf, $flavor>>::encoded_len(self, ctx)
    }
  };
  (@decode $flavor:ty:$wf:ty:$ty:ty $([ $( const $g:ident: $gty:ty), +$(,)? ])?) => {
    impl<'de, RB, B, $($(const $g: $gty),*)?> $crate::__private::decode::Decode<'de, $wf, RB, B, $flavor> for $ty {
      $crate::varint!(@decode_impl $flavor:$wf);
    }
  };
  (@decode_impl $flavor:ty:$wf:ty) => {
    fn decode(_: &<$flavor as $crate::__private::flavors::Flavor>::Context, src: RB) -> ::core::result::Result<(::core::primitive::usize, Self), <$flavor as $crate::__private::flavors::Flavor>::Error>
    where
      Self: ::core::marker::Sized + 'de,
      RB: $crate::__private::ReadBuf,
      B: $crate::__private::UnknownBuffer<RB, $flavor> + 'de,
    {
      $crate::__private::varing::Varint::decode(src.as_bytes()).map_err(::core::convert::Into::into)
    }
  };
}

/// A macro emits [`PartialEncode`](super::encode::PartialEncode) implementations for scalar types.
#[macro_export]
macro_rules! partial_encode_scalar {
  (@impl $flavor:ty as $format:ty) => {
    fn partial_encode_raw<WB>(&self, context: &<$flavor as $crate::__private::flavors::Flavor>::Context, buf: &mut WB, s: &Self::Selector) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::flavors::Flavor>::Error>
    where
      WB: $crate::__private::buffer::WriteBuf + ?::core::marker::Sized,
    {
      if *s {
        <Self as $crate::__private::Encode<$format, $flavor>>::encode_raw(self, context, buf)
      } else {
        ::core::result::Result::Ok(0)
      }
    }

    fn partial_encoded_raw_len(&self, context: &<$flavor as $crate::__private::flavors::Flavor>::Context, s: &Self::Selector) -> ::core::primitive::usize {
      if *s {
        <Self as $crate::__private::Encode<$format, $flavor>>::encoded_raw_len(self, context)
      } else {
        0
      }
    }

    fn partial_encode<WB>(&self, context: &<$flavor as $crate::__private::flavors::Flavor>::Context, buf: &mut WB, s: &Self::Selector) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::flavors::Flavor>::Error>
    where
      WB: $crate::__private::buffer::WriteBuf + ?::core::marker::Sized,
    {
      if *s {
        <Self as $crate::__private::Encode<$format, $flavor>>::encode(self, context, buf)
      } else {
        ::core::result::Result::Ok(0)
      }
    }

    fn partial_encoded_len(&self, context: &<$flavor as $crate::__private::flavors::Flavor>::Context, s: &Self::Selector) -> ::core::primitive::usize {
      if *s {
        <Self as $crate::__private::Encode<$format, $flavor>>::encoded_len(self, context)
      } else {
        0
      }
    }

    fn partial_encode_length_delimited<WB>(&self, context: &<$flavor as $crate::__private::flavors::Flavor>::Context, buf: &mut WB, s: &Self::Selector) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::flavors::Flavor>::Error>
    where
      WB: $crate::__private::buffer::WriteBuf + ?::core::marker::Sized,
    {
      if *s {
        <Self as $crate::__private::Encode<$format, $flavor>>::encode_length_delimited(self, context, buf)
      } else {
        ::core::result::Result::Ok(0)
      }
    }

    fn partial_encoded_length_delimited_len(&self, context: &<$flavor as $crate::__private::flavors::Flavor>::Context, s: &Self::Selector) -> ::core::primitive::usize {
      if *s {
        <Self as $crate::__private::Encode<$format, $flavor>>::encoded_length_delimited_len(self, context)
      } else {
        0
      }
    }
  };
  ($flavor:ty: $($ty:ty $([ $( const $g:ident: $gty:ty), +$(,)? ])? as $format:ty),+$(,)?) => {
    $(
      impl $( < $(const $g: $gty),* > )? $crate::__private::PartialEncode<$format, $flavor> for $ty {
        $crate::partial_encode_scalar!(@impl $flavor as $format);
      }
    )*
  };
}

/// A macro emits [`Selectable`](super::selector::Selectable) implementations for scalar types.
#[macro_export]
macro_rules! selectable {
  (@scalar $flavor:ty: $($ty:ty $([ $( const $g:ident: $gty:ty), +$(,)?])?),+$(,)?) => {
    $(
      #[allow(non_camel_case_types)]
      impl$(< $(const $g: $gty),* >)? $crate::__private::selection::Selectable<$flavor> for $ty {
        type Selector = ::core::primitive::bool;
      }
    )*
  };
  (@bridge $flavor:ty: $(
    $bridge: ty [
      $($ty:ty $([ $( const $g:ident: $gty:ty), +$(,)? ])? as $wf:ty), +$(,)?
    ]
  ),+$(,)?) => {
    $(
      $(
        impl $( < $(const $g: $gty),* > )? $crate::__private::selection::Selectable<$flavor> for $ty {
          type Selector = <$bridge as $crate::__private::selection::Selectable<$flavor>>::Selector;
        }
      )*
    )*
  };
}

/// A macro emits basic [`State<PartialRef<'_, WireFormat, Src, UB, Flavor>>`](super::State) implementations for `Self`
#[macro_export]
macro_rules! partial_ref_state {
  (&$lifetime:lifetime $flavor:ty: $($ty:ty $([ $( const $g:ident: $gty:ty), +$(,)? ])? as $wf:ty => $target:ty ),+$(,)?) => {
    $(
      #[allow(non_camel_case_types)]
      impl<$lifetime, __GROST_READ_BUF__, __GROST_BUFFER__, $( $(const $g: $gty),* )?> $crate::__private::state::State<$crate::__private::state::PartialRef<$lifetime, $wf, __GROST_READ_BUF__, __GROST_BUFFER__, $flavor>> for $ty {
        type Output = $target;
      }
    )*
  };
  (@scalar &$lifetime:lifetime $flavor:ty: $($ty:ty $([ $( const $g:ident: $gty:ty), +$(,)? ])? as $wf:ty),+$(,)?) => {
    $crate::partial_ref_state!(& $lifetime $flavor: $($ty $([ $(const $g: $gty),* ])? as $wf => $ty),+);
  };
}

/// A macro emits basic [`State<Ref<'_, WireFormat, Src, UB, Flavor>>`](super::State) implementations for `Self`
#[macro_export]
macro_rules! ref_state {
  (&$lifetime:lifetime $flavor:ty: $($ty:ty $([ $( const $g:ident: $gty:ty), +$(,)? ])? as $wf:ty => $target:ty ),+$(,)?) => {
    $(
      #[allow(non_camel_case_types)]
      impl<$lifetime, __GROST_READ_BUF__, __GROST_BUFFER__, $( $(const $g: $gty),* )?> $crate::__private::state::State<$crate::__private::state::Ref<$lifetime, $wf, __GROST_READ_BUF__, __GROST_BUFFER__, $flavor>> for $ty {
        type Output = $target;
      }
    )*
  };
  (@scalar &$lifetime:lifetime $flavor:ty: $($ty:ty $([ $( const $g:ident: $gty:ty), +$(,)? ])? as $wf:ty),+$(,)?) => {
    $crate::ref_state!(& $lifetime $flavor: $($ty $([ $(const $g: $gty),* ])? as $wf => $ty),+);
  };
}

/// A macro emits basic [`State<Partial<Flavor>>`](super::State) implementations for `Self`
#[macro_export]
macro_rules! partial_state {
  ($flavor:ty: $($ty:ty $([ $( const $g:ident: $gty:ty), +$(,)? ])? => $target:ty ),+$(,)?) => {
    $(
      #[allow(non_camel_case_types)]
      impl$(< $(const $g: $gty),* > )? $crate::__private::state::State<$crate::__private::state::Partial<$flavor>> for $ty {
        type Output = $target;
      }
    )*
  };
  (@scalar $flavor:ty: $($ty:ty $([ $( const $g:ident: $gty:ty), +$(,)? ])?),+$(,)?) => {
    $crate::partial_state!($flavor: $($ty $([ $(const $g: $gty),* ])? => $ty),+);
  };
}

/// A macro emits [`impl Reflectable<Self> for Reflection<Self, Type, Flavor>`](super::reflection::Reflectable) implementations for `Self`
#[macro_export]
macro_rules! type_reflection {
  ($flavor:ty: $($ty:ty $([ $( const $g:ident: $gty:ty), +$(,)? ])? => $expr:expr),+$(,)?) => {
    $(
      impl$(<$(const $g: $gty),*>)? $crate::__private::reflection::Reflectable<$ty> for $crate::__private::reflection::SchemaTypeReflection<$ty> {
        type Reflection = $crate::__private::reflection::SchemaType;

        const REFLECTION: &Self::Reflection = &{
          $expr
        };
      }
    )*
  };
}

/// A macro emits basic [`State<Extracted<_>>`](super::State) implementations for `Self`
#[macro_export]
macro_rules! flatten_state {
  ($($ty:ty $([ $( const $g:ident: $gty:ty), +$(,)? ])?),+$(,)?) => {
    $(
      impl<S: ?::core::marker::Sized, $( $(const $g: $gty),* )?> $crate::__private::state::State<
        $crate::__private::convert::Extracted<S>
      > for $ty {
        type Output = $ty;
      }
    )*
  };
  (@scalar $($ty:ty $([ $( const $g:ident: $gty:ty), +$(,)? ])?),+$(,)?) => {
    $crate::flatten_state!($($ty $([ $(const $g: $gty),* ])? => $ty),+);
  };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __default_wire_format__ {
  (@impl $(
    $t:ident<$flavor:ty $(, $($p:ident),+$(,)?)?>: $ty:ty $([ $( const $g:ident: $gty:ty), +$(,)? ])? as $format:ty
  ),+$(,)?) => {
    $crate::__private::paste::paste! {
      $(
        impl $( < $(const $g: $gty),* > )? $crate::__private::flavors::[< Default $t:camel WireFormat >]<$flavor> for $ty {
          type Format $(< $($p),* >)? = $format $(where $($p: $crate::__private::flavors::WireFormat<$flavor>),*)?;
        }
      )*
    }
  };
}

/// A macro emits [`DefaultScalarWireFormat`](crate::flavors::DefaultScalarWireFormat) implementations.
#[macro_export]
macro_rules! default_scalar_wire_format {
  ($(
    $flavor:ty: $($ty:ty $([ $( const $g:ident: $gty:ty), +$(,)? ])? as $format:ty); +$(;)?
  ),+$(,)?) => {
    $(
      $crate::__default_wire_format__!(@impl $(
        scalar<$flavor>: $ty $( $(const $g: $gty),* )? as $format
      ),*);
    )*
  };
}

/// A macro emits [`DefaultStringWireFormat`](crate::flavors::DefaultStringWireFormat) implementations.
#[macro_export]
macro_rules! default_string_wire_format {
  ($(
    $flavor:ty: $($ty:ty $([ $( const $g:ident: $gty:ty), +$(,)? ])? as $format:ty); +$(;)?
  ),+$(,)?) => {
    $(
      $crate::__default_wire_format__!(@impl $(
        string<$flavor>: $ty $( [$( const $g: $gty),* ])? as $format
      ),*);
    )*
  };
}

/// A macro emits [`DefaultBytesWireFormat`](crate::flavors::DefaultBytesWireFormat) implementations.
#[macro_export]
macro_rules! default_bytes_wire_format {
  ($(
    $flavor:ty: $($ty:ty $([ $( const $g:ident: $gty:ty), +$(,)? ])? as $format:ty); +$(;)?
  ),+$(,)?) => {
    $(
      $crate::__default_wire_format__!(@impl $(
        bytes<$flavor>: $ty $( [$(const $g: $gty),* ])? as $format
      ),*);
    )*
  };
}

/// A macro emits [`DefaultEnumWireFormat`](crate::flavors::DefaultEnumWireFormat) implementations.
#[macro_export]
macro_rules! default_enum_wire_format {
  ($(
    $flavor:ty: $($ty:ty $([ $( const $g:ident: $gty:ty), +$(,)? ])? as $format:ty); +$(;)?
  ),+$(,)?) => {
    $(
      $crate::__default_wire_format__!(@impl $(
        enum<$flavor>: $ty $( $(const $g: $gty),* )? as $format
      ),*);
    )*
  };
}

/// A macro emits [`DefaultObjectWireFormat`](crate::flavors::DefaultObjectWireFormat) implementations.
#[macro_export]
macro_rules! default_object_wire_format {
  ($(
    $flavor:ty: $($ty:ty $([ $( const $g:ident: $gty:ty), +$(,)? ])? as $format:ty); +$(;)?
  ),+$(,)?) => {
    $(
      $crate::__default_wire_format__!(@impl $(
        object<$flavor>: $ty $( $(const $g: $gty),* )? as $format
      ),*);
    )*
  };
}

/// A macro emits [`DefaultUnionWireFormat`](crate::flavors::DefaultUnionWireFormat) implementations.
#[macro_export]
macro_rules! default_union_wire_format {
  ($(
    $flavor:ty: $($ty:ty $([ $( const $g:ident: $gty:ty), +$(,)? ])? as $format:ty); +$(;)?
  ),+$(,)?) => {
    $(
      $crate::__default_wire_format__!(@impl $(
        union<$flavor>: $ty $( $(const $g: $gty),* )? as $format
      ),*);
    )*
  };
}

/// A macro emits [`DefaultInterfaceWireFormat`](crate::flavors::DefaultInterfaceWireFormat) implementations.
#[macro_export]
macro_rules! default_interface_wire_format {
  ($(
    $flavor:ty: $($ty:ty $([ $( const $g:ident: $gty:ty), +$(,)? ])? as $format:ty); +$(;)?
  ),+$(,)?) => {
    $(
      $crate::__default_wire_format__!(@impl $(
        interface<$flavor>: $ty $( $(const $g: $gty),* )? as $format
      ),*);
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
macro_rules! bridge {
  ($(
    $flavor:ty: $bridge: ty {
      $($ty:ty $([ $( const $g:ident: $gty:ty), +$(,)? ])? as $format:ty {
        from: $from:expr;
        to: $to:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $crate::encode_bridge!($flavor: $bridge {
        $($ty $([ $(const $g: $gty),* ])? as $format {
          convert: $to;
        },)*
      });

      $crate::decode_bridge!($flavor: $bridge => $bridge {
        $($ty $([ $(const $g: $gty),* ])? as $format {
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
    $flavor:ty: $bridge: ty => $output:ty {
      $($ty:ty $([ $( const $g:ident: $gty:ty), +$(,)? ])? as $format:ty {
        try_from: $from:expr;
        to: $to:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $crate::encode_bridge!($flavor: $bridge {
        $($ty $([ $(const $g: $gty),* ])? as $format {
          convert: $to;
        },)*
      });

      $crate::try_decode_bridge!($flavor: $bridge => $output {
        $($ty $([ $(const $g: $gty),* ])? as $format {
          convert: $from;
        },)*
      });
    )*
  };
  ($(
    $flavor:ty: $bridge: ty {
      $($ty:ty $([ $( const $g:ident: $gty:ty), +$(,)? ])? as $format:ty {
        try_from: $from:expr;
        to: $to:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $crate::encode_bridge!($flavor: $bridge {
        $($ty $([ $(const $g: $gty),* ])? as $format {
          convert: $to;
        },)*
      });

      $crate::try_decode_bridge!($flavor: $bridge => $bridge {
        $($ty $([ $(const $g: $gty),* ])? as $format {
          convert: $from;
        },)*
      });
    )*
  };
}

/// A macro emits [`Encode`](super::encode::Encode) implementations for `Self`.
#[macro_export]
macro_rules! encode_bridge {
  ($(
    $flavor:ty: $bridge: ty {
      $($ty:ty $([ $( const $g:ident: $gty:ty), +$(,)? ])? as $format:ty {
        convert: $to:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        $crate::encode_bridge!(@encode $flavor: $bridge {
          $ty $([ $(const $g: $gty),* ])? as $format {
            to: $to;
          }
        });

        $crate::encode_bridge!(@partial_encode $flavor: $bridge {
          $ty $([ $(const $g: $gty),* ])? as $format {
            to: $to;
          }
        });
      )*
    )*
  };
  (@encode_impl $flavor:ty: $bridge:ty as $format:ty => $to:expr) => {
    fn encode_raw<WB>(
      &self,
      context: &<$flavor as $crate::__private::flavors::Flavor>::Context,
      buf: &mut WB,
    ) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::flavors::Flavor>::Error>
    where
      WB: $crate::__private::buffer::WriteBuf + ?::core::marker::Sized,
    {
      <$bridge as $crate::__private::Encode<$format, $flavor>>::encode_raw(&$to(self), context, buf)
    }

    fn encoded_raw_len(&self, context: &<$flavor as $crate::__private::flavors::Flavor>::Context,) -> ::core::primitive::usize {
      <$bridge as $crate::__private::Encode<$format, $flavor>>::encoded_raw_len(&$to(self), context)
    }

    fn encode<WB>(
      &self,
      context: &<$flavor as $crate::__private::flavors::Flavor>::Context,
      buf: &mut WB,
    ) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::flavors::Flavor>::Error>
    where
      WB: $crate::__private::buffer::WriteBuf + ?::core::marker::Sized,
    {
      <$bridge as $crate::__private::Encode<$format, $flavor>>::encode(&$to(self), context, buf)
    }

    fn encoded_len(&self, context: &<$flavor as $crate::__private::flavors::Flavor>::Context,) -> ::core::primitive::usize {
      <$bridge as $crate::__private::Encode<$format, $flavor>>::encoded_len(&$to(self), context)
    }

    fn encode_length_delimited<WB>(
      &self,
      context: &<$flavor as $crate::__private::flavors::Flavor>::Context,
      buf: &mut WB,
    ) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::flavors::Flavor>::Error>
    where
      WB: $crate::__private::buffer::WriteBuf + ?::core::marker::Sized,
    {
      <$bridge as $crate::__private::Encode<$format, $flavor>>::encode_length_delimited(&$to(self), context, buf)
    }

    fn encoded_length_delimited_len(&self, context: &<$flavor as $crate::__private::flavors::Flavor>::Context,) -> ::core::primitive::usize {
      <$bridge as $crate::__private::Encode<$format, $flavor>>::encoded_length_delimited_len(&$to(self), context)
    }
  };
  (@encode $flavor:ty: $(
    $bridge:ty {
      $($ty:ty $([ $( const $g:ident: $gty:ty), +$(,)? ])? as $format:ty {
        to: $to:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        impl $( < $(const $g: $gty),* > )? $crate::__private::Encode<$format, $flavor> for $ty {
          $crate::encode_bridge!(@encode_impl $flavor: $bridge as $format => $to);
        }
      )*
    )*
  };
  (@partial_encode_impl $flavor:ty: $bridge:ty as $format:ty => $to:expr) => {
    fn partial_encode_raw<WB>(
      &self,
      context: &<$flavor as $crate::__private::flavors::Flavor>::Context,
      buf: &mut WB,
      selector: &Self::Selector,
    ) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::flavors::Flavor>::Error>
    where
      WB: $crate::__private::buffer::WriteBuf + ?::core::marker::Sized,
    {
      <$bridge as $crate::__private::PartialEncode<$format, $flavor>>::partial_encode_raw(&$to(self), context, buf, selector)
    }

    fn partial_encoded_raw_len(
      &self,
      context: &<$flavor as $crate::__private::flavors::Flavor>::Context,
      selector: &Self::Selector,
    ) -> ::core::primitive::usize {
      <$bridge as $crate::__private::PartialEncode<$format, $flavor>>::partial_encoded_raw_len(&$to(self), context, selector)
    }

    fn partial_encode<WB>(
      &self,
      context: &<$flavor as $crate::__private::flavors::Flavor>::Context,
      buf: &mut WB,
      selector: &Self::Selector,
    ) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::flavors::Flavor>::Error>
    where
      WB: $crate::__private::buffer::WriteBuf + ?::core::marker::Sized,
    {
      <$bridge as $crate::__private::PartialEncode<$format, $flavor>>::partial_encode(&$to(self), context, buf, selector)
    }

    fn partial_encoded_len(
      &self,
      context: &<$flavor as $crate::__private::flavors::Flavor>::Context,
      selector: &Self::Selector,
    ) -> ::core::primitive::usize {
      <$bridge as $crate::__private::PartialEncode<$format, $flavor>>::partial_encoded_len(&$to(self), context, selector)
    }

    fn partial_encode_length_delimited<WB>(
      &self,
      context: &<$flavor as $crate::__private::flavors::Flavor>::Context,
      buf: &mut WB,
      selector: &Self::Selector,
    ) -> ::core::result::Result<::core::primitive::usize, <$flavor as $crate::__private::flavors::Flavor>::Error>
    where
      WB: $crate::__private::buffer::WriteBuf + ?::core::marker::Sized,
    {
      <$bridge as $crate::__private::PartialEncode<$format, $flavor>>::partial_encode_length_delimited(&$to(self), context, buf, selector)
    }

    fn partial_encoded_length_delimited_len(
      &self,
      context: &<$flavor as $crate::__private::flavors::Flavor>::Context,
      selector: &Self::Selector,
    ) -> ::core::primitive::usize {
      <$bridge as $crate::__private::PartialEncode<$format, $flavor>>::partial_encoded_length_delimited_len(&$to(self), context, selector)
    }
  };
  (@partial_encode $flavor:ty: $(
    $bridge:ty {
      $($ty:ty $([ $( const $g:ident: $gty:ty), +$(,)? ])? as $format:ty {
        to: $to:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        impl $( < $(const $g: $gty),* > )? $crate::__private::PartialEncode<$format, $flavor> for $ty {
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
    $flavor:ty: $bridge: ty => $output: ty {
      $($ty:ty $([ $( const $g:ident: $gty:ty), +$(,)? ])? as $format:ty {
        convert: $from:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        impl<'de, RB, B, $( $(const $g: $gty),* )?> $crate::__private::decode::Decode<
          'de,
          $format,
          RB,
          B,
          $flavor,
        > for $ty {
          $crate::decode_bridge!(@decode_impl $flavor: $from => $bridge as $format => $output);
        }
      )*
    )*
  };
  ($(
    $flavor:ty: $bridge: ty {
      $($ty:ty $([ $( const $g:ident: $gty:ty), +$(,)? ])? as $format:ty {
        convert: $from:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        impl<'de, RB, B, $( $(const $g: $gty),* )?> $crate::__private::decode::Decode<
          'de,
          $format,
          B,
          UB,
          $flavor,
        > for $ty {
          $crate::decode_bridge!(@decode_impl $flavor: $from => $bridge as $format => $bridge);
        }
      )*
    )*
  };
  (@decode_impl $flavor:ty: $from:expr => $bridge:ty as $format:ty => $output:ty) => {
    fn decode(
      context: &'de <$flavor as $crate::__private::flavors::Flavor>::Context,
      src: RB,
    ) -> ::core::result::Result<(::core::primitive::usize, Self), <$flavor as $crate::__private::flavors::Flavor>::Error>
    where
      Self: ::core::marker::Sized + 'de,
      RB: $crate::__private::ReadBuf + 'de,
      B: $crate::__private::UnknownBuffer<RB, $flavor> + 'de,
    {
      <$bridge as $crate::__private::decode::Decode<'de, $format, RB, B, $flavor>>::decode(context, src).map(|(n, v)| (n, $from(v)))
    }

    fn decode_length_delimited(
      context: &'de <$flavor as $crate::__private::flavors::Flavor>::Context,
      src: RB,
    ) -> ::core::result::Result<(::core::primitive::usize, Self), <$flavor as $crate::__private::flavors::Flavor>::Error>
    where
      Self: ::core::marker::Sized + 'de,
      RB: $crate::__private::ReadBuf + 'de,
      B: $crate::__private::UnknownBuffer<RB, $flavor> + 'de,
    {
      <$bridge as $crate::__private::decode::Decode<'de, $format, RB, B, $flavor>>::decode_length_delimited(context, src).map(|(n, v)| (n, $from(v)))
    }
  };
}

/// A macro emits [`Decode`](super::decode::Decode) implementations for `Self`.
#[macro_export]
macro_rules! try_decode_bridge {
  ($(
    $flavor:ty: $bridge: ty => $output: ty {
      $($ty:ty $([ $( const $g:ident: $gty:ty), +$(,)? ])? as $format:ty {
        convert: $from:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        impl<'de, RB, B> $crate::__private::decode::Decode<'de, $format, RB, B, $flavor> for $ty {
          $crate::try_decode_bridge!(@decode_impl $flavor: $from => $bridge as $format => $output);
        }
      )*
    )*
  };
  ($(
    $flavor:ty: $bridge: ty {
      $($ty:ty $([ $( const $g:ident: $gty:ty), +$(,)? ])? as $format:ty {
        convert: $from:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        impl<'de, RB, B> $crate::__private::decode::Decode<'de, $format, RB, B, $flavor> for $ty {
          $crate::try_decode_bridge!(@decode_impl $flavor: $from => $bridge as $format => $bridge);
        }
      )*
    )*
  };
  (@decode_impl $flavor:ty: $from:expr => $bridge:ty as $format:ty => $output:ty) => {
    fn decode(
      context: &'de <$flavor as $crate::__private::flavors::Flavor>::Context,
      src: RB,
    ) -> ::core::result::Result<(::core::primitive::usize, Self), <$flavor as $crate::__private::flavors::Flavor>::Error>
    where
      Self: ::core::marker::Sized + 'de,
      RB: $crate::__private::ReadBuf + 'de,
      B: $crate::__private::UnknownBuffer<RB, $flavor> + 'de,
    {
      <$bridge as $crate::__private::decode::Decode<'de, $format, RB, B, $flavor>>::decode(context, src).and_then(|(n, v)| $from(v).map(|v| (n, v)))
    }

    fn decode_length_delimited(
      context: &'de <$flavor as $crate::__private::flavors::Flavor>::Context,
      src: RB,
    ) -> ::core::result::Result<(::core::primitive::usize, Self), <$flavor as $crate::__private::flavors::Flavor>::Error>
    where
      Self: ::core::marker::Sized + 'de,
      RB: $crate::__private::ReadBuf + 'de,
      B: $crate::__private::UnknownBuffer<RB, $flavor> + 'de,
    {
      <$bridge as $crate::__private::decode::Decode<'de, $format, RB, B, $flavor>>::decode_length_delimited(context, src).and_then(|(n, v)| $from(v).map(|v| (n, v)))
    }
  };
  (@decode $flavor:ty: $(
    $bridge:ty => $output:ty {
      $($ty:ty $([ $( const $g:ident: $gty:ty), +$(,)? ])? as $format:ty {
        try_from: $from:expr $(;)?
      }), +$(,)?
    }
  ),+$(,)?) => {
    $(
      $(
        impl<'de, RB, B> $crate::__private::decode::Decode<'de, $format, RB, B, $flavor> for $ty {
          $crate::try_decode_bridge!(@decode_impl $flavor: $from => $bridge as $format => $output);
        }
      )*
    )*
  };
}

/// A macro emits [`TryFromPartial`](super::convert::TryFromPartial) implementations for `Self`.
#[macro_export]
macro_rules! try_from_partial {
  (@scalar $flavor:ty: $($ty:ty),+$(,)?) => {
    $(
      impl $crate::__private::convert::TryFromPartial<$flavor> for $ty {
        fn try_from_partial(
          ctx: &<$flavor as $crate::__private::flavors::Flavor>::Context,
          input: <Self as $crate::__private::state::State<crate::__private::state::Partial<$flavor>>>::Output,
        ) -> ::core::result::Result<Self, <$flavor as $crate::__private::flavors::Flavor>::Error>
        where
          Self: ::core::marker::Sized,
          <Self as $crate::__private::state::State<$crate::__private::state::Partial<$flavor>>>::Output: ::core::marker::Sized
        {
          ::core::result::Result::Ok(input)
        }
      }
    )*
  };
}

/// A macro emits [`TryFromRef`](super::convert::TryFromRef) implementations for `Self`.
#[macro_export]
macro_rules! try_from_ref {
  (@scalar &$lifetime:lifetime $flavor:ty: $($ty:ty as $wf:ty),+$(,)?) => {
    $(
      #[allow(non_camel_case_types)]
      impl<$lifetime, __GROST_READ_BUF__, __GROST_BUFFER__> $crate::__private::convert::TryFromRef<$lifetime, $wf, __GROST_READ_BUF__, __GROST_BUFFER__, $flavor> for $ty {
        fn try_from_ref(
          ctx: &$lifetime <$flavor as $crate::__private::flavors::Flavor>::Context,
          input: <Self as $crate::__private::state::State<$crate::__private::state::Ref<$lifetime, $wf, __GROST_READ_BUF__, __GROST_BUFFER__, $flavor>>>::Output,
        ) -> ::core::result::Result<Self, <$flavor as $crate::__private::flavors::Flavor>::Error>
        where
          Self: Sized,
          <Self as $crate::__private::state::State<$crate::__private::state::Ref<$lifetime, $wf, __GROST_READ_BUF__, __GROST_BUFFER__, $flavor>>>::Output: Sized,
          __GROST_READ_BUF__: $crate::__private::buffer::ReadBuf + $lifetime,
          __GROST_BUFFER__: $crate::__private::buffer::UnknownBuffer<__GROST_READ_BUF__, $flavor> + $lifetime,
        {
          ::core::result::Result::Ok(input)
        }
      }
    )*
  };
}

/// A macro emits [`PartialTryFromRef`](super::convert::PartialTryFromRef) implementations for `Self`.
#[macro_export]
macro_rules! try_from_partial_ref {
  (@scalar &$lifetime:lifetime $flavor:ty: $($ty:ty as $wf:ty),+$(,)?) => {
    $(
      impl<$lifetime, __GROST_READ_BUF__, __GROST_BUFFER__> $crate::__private::convert::PartialTryFromRef<$lifetime, $wf, __GROST_READ_BUF__, __GROST_BUFFER__, $flavor> for $ty {
        fn partial_try_from_ref(
          context: &$lifetime <$flavor as $crate::__private::flavors::Flavor>::Context,
          input: <Self as $crate::__private::state::State<$crate::__private::state::PartialRef<$lifetime, $wf, __GROST_READ_BUF__, __GROST_BUFFER__, $flavor>>>::Output,
          selector: &Self::Selector,
        ) -> ::core::result::Result<<Self as $crate::__private::state::State<$crate::__private::state::Partial<$flavor>>>::Output, <$flavor as $crate::__private::flavors::Flavor>::Error>
        where
          <Self as $crate::__private::state::State<$crate::__private::state::Partial<$flavor>>>::Output: Sized,
          <Self as $crate::__private::state::State<$crate::__private::state::PartialRef<'de, $wf, __GROST_READ_BUF__, __GROST_BUFFER__, $flavor>>>::Output: Sized
        {
          ::core::result::Result::Ok(input)
        }
      }
    )*
  };
}

/// A macro emits [`PartialIdentity`](super::convert::PartialIdentity) implementations for `Self`.
#[macro_export]
macro_rules! partial_identity {
  (@scalar $flavor:ty: $($ty:ty $([ $( const $g:ident: $gty:ty), +$(,)? ])?),+$(,)?) => {
    $(
      impl $(< $(const $g:$gty),* >)? $crate::__private::convert::PartialIdentity<$flavor> for $ty {
        fn partial_identity<'a>(
          input: &'a mut Self::Output,
          _: &'a Self::Selector,
        ) -> &'a mut Self::Output {
          input
        }
      }
    )*
  };
}

/// A macro emits [`PartialTryFromRef`](super::convert::PartialTryFromRef) implementations for `Self`.
#[macro_export]
macro_rules! partial_try_from_ref {
  (@scalar &$lifetime:lifetime $flavor:ty: $($ty:ty as $wf:ty),+$(,)?) => {
    $(
      impl<$lifetime, __GROST_READ_BUF__, __GROST_BUFFER__> $crate::__private::convert::PartialTryFromRef<$lifetime, $wf, __GROST_READ_BUF__, __GROST_BUFFER__, $flavor> for $ty {
        fn partial_try_from_ref(
          context: &$lifetime <$flavor as $crate::__private::flavors::Flavor>::Context,
          input: <Self as $crate::__private::state::State<$crate::__private::state::PartialRef<$lifetime, $wf, __GROST_READ_BUF__, __GROST_BUFFER__, $flavor>>>::Output,
          _: &Self::Selector,
        ) -> ::core::result::Result<<Self as $crate::__private::state::State<$crate::__private::state::Partial<$flavor>>>::Output, <$flavor as $crate::__private::flavors::Flavor>::Error>
        where
          <Self as $crate::__private::state::State<$crate::__private::state::Partial<$flavor>>>::Output: Sized,
          <Self as $crate::__private::state::State<$crate::__private::state::PartialRef<'de, $wf, __GROST_READ_BUF__, __GROST_BUFFER__, $flavor>>>::Output: Sized
        {
          ::core::result::Result::Ok(input)
        }
      }
    )*
  };
}
