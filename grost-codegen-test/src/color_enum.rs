#![no_implicit_prelude]

#[derive(
    ::core::marker::Copy,
    ::core::clone::Clone,
    ::core::fmt::Debug,
    ::core::default::Default,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::hash::Hash
)]
#[repr(u32)]
#[non_exhaustive]
pub enum Color {
    #[default]
    Red,
    Green,
    Blue,
    /// An unknown variant of the enum, used for backwards and forwards compatibility.
    Unknown(::core::primitive::u32),
}
impl Color {
    /// The relection information of the [`Color::Red`] enum variant for [`Network`](::grost::__private::flavors::Network) flavor.
    pub const RED_REFLECTION_NETWORK_FLAVOR: ::grost::__private::reflection::UnitEnumVariantReflection = ::grost::__private::reflection::UnitEnumVariantReflectionBuilder {
        name: "Red",
        schema_name: "Red",
        description: "",
        value: ::grost::__private::reflection::UnitEnumVariantValue::U32(
            ::core::num::NonZeroU32::new(1u32).unwrap(),
        ),
    }
        .build();
    /// The relection information of the [`Color::Green`] enum variant for [`Network`](::grost::__private::flavors::Network) flavor.
    pub const GREEN_REFLECTION_NETWORK_FLAVOR: ::grost::__private::reflection::UnitEnumVariantReflection = ::grost::__private::reflection::UnitEnumVariantReflectionBuilder {
        name: "Green",
        schema_name: "Green",
        description: "",
        value: ::grost::__private::reflection::UnitEnumVariantValue::U32(
            ::core::num::NonZeroU32::new(2u32).unwrap(),
        ),
    }
        .build();
    /// The relection information of the [`Color::Blue`] enum variant for [`Network`](::grost::__private::flavors::Network) flavor.
    pub const BLUE_REFLECTION_NETWORK_FLAVOR: ::grost::__private::reflection::UnitEnumVariantReflection = ::grost::__private::reflection::UnitEnumVariantReflectionBuilder {
        name: "Blue",
        schema_name: "Blue",
        description: "",
        value: ::grost::__private::reflection::UnitEnumVariantValue::U32(
            ::core::num::NonZeroU32::new(3u32).unwrap(),
        ),
    }
        .build();
    /// The relection information of the [`Color`] enum for [`Network`](::grost::__private::flavors::Network) flavor.
    pub const NETWORK_FLAVOR_REFLECTION: ::grost::__private::reflection::UnitEnumReflection = ::grost::__private::reflection::UnitEnumReflectionBuilder {
        name: "Color",
        schema_name: "Color",
        description: "",
        variants: &[
            Self::RED_REFLECTION_NETWORK_FLAVOR,
            Self::GREEN_REFLECTION_NETWORK_FLAVOR,
            Self::BLUE_REFLECTION_NETWORK_FLAVOR,
        ],
        repr: ::grost::__private::reflection::UnitEnumRepr::U32,
    }
        .build();
}
impl ::core::fmt::Display for Color {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        match self.try_as_str() {
            ::core::result::Result::Ok(variant_str) => ::core::write!(f, "{variant_str}"),
            ::core::result::Result::Err(val) => ::core::write!(f, "Unknown({})", val),
        }
    }
}
impl Color {
    /// Try to return the enum variant as a `str`, if the variant is unknown, it will return the value of the unknown variant.
    #[inline]
    pub const fn try_as_str(
        &self,
    ) -> ::core::result::Result<
        &'static ::core::primitive::str,
        ::core::primitive::u32,
    > {
        ::core::result::Result::Ok(
            match self {
                Self::Red => "Red",
                Self::Green => "Green",
                Self::Blue => "Blue",
                Self::Unknown(val) => return ::core::result::Result::Err(*val),
            },
        )
    }
}
#[derive(
    ::core::clone::Clone,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
    ::core::hash::Hash,
)]
///The error type returned when parsing [`Color`]
pub struct ParseColorError {
    _priv: ::std::string::String,
}
impl ::core::fmt::Display for ParseColorError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::write!(f, "Fail to parse `Color`, unknown {}", self._priv)
    }
}
impl ::core::error::Error for ParseColorError {}
impl ::core::str::FromStr for Color {
    type Err = ParseColorError;
    fn from_str(
        src: &::core::primitive::str,
    ) -> ::core::result::Result<Self, Self::Err> {
        match src.trim() {
            "red" | "RED" | "Red" => ::core::result::Result::Ok(Self::Red),
            "green" | "GREEN" | "Green" => ::core::result::Result::Ok(Self::Green),
            "blue" | "BLUE" | "Blue" => ::core::result::Result::Ok(Self::Blue),
            val => {
                if let ::core::option::Option::Some(remaining) = val
                    .strip_prefix("Unknown(")
                    .or_else(|| val.strip_prefix("unknown("))
                {
                    if let ::core::option::Option::Some(remaining) = remaining
                        .strip_suffix(')')
                    {
                        if let ::core::result::Result::Ok(val) = <::core::primitive::u32 as ::core::str::FromStr>::from_str(
                            remaining,
                        ) {
                            return ::core::result::Result::Ok(
                                ::core::convert::From::from(val),
                            );
                        }
                    }
                }
                if let ::core::result::Result::Ok(val) = <::core::primitive::u32 as ::core::str::FromStr>::from_str(
                    val,
                ) {
                    return ::core::result::Result::Ok(::core::convert::From::from(val));
                }
                ::core::result::Result::Err(ParseColorError {
                    _priv: ::std::string::ToString::to_string(src),
                })
            }
        }
    }
}
impl<'a> ::core::convert::TryFrom<&'a ::core::primitive::str> for Color {
    type Error = ParseColorError;
    fn try_from(
        src: &'a ::core::primitive::str,
    ) -> ::core::result::Result<Self, Self::Error> {
        ::core::str::FromStr::from_str(src)
    }
}
impl Color {
    ///Returns `true` if the enum variant is [`Color::Red`]
    #[inline]
    pub const fn is_red(&self) -> ::core::primitive::bool {
        ::core::matches!(self, Self::Red)
    }
    ///Returns `true` if the enum variant is [`Color::Green`]
    #[inline]
    pub const fn is_green(&self) -> ::core::primitive::bool {
        ::core::matches!(self, Self::Green)
    }
    ///Returns `true` if the enum variant is [`Color::Blue`]
    #[inline]
    pub const fn is_blue(&self) -> ::core::primitive::bool {
        ::core::matches!(self, Self::Blue)
    }
}
impl ::core::convert::From<::core::primitive::u32> for Color {
    #[inline]
    fn from(repr: ::core::primitive::u32) -> Self {
        Self::from_u32(repr)
    }
}
impl ::core::convert::From<Color> for ::core::primitive::u32 {
    #[inline]
    fn from(e: Color) -> Self {
        e.as_u32()
    }
}
impl<'a> ::core::convert::TryFrom<&'a [::core::primitive::u8]> for Color {
    type Error = ::grost::__private::varing::DecodeError;
    #[inline]
    fn try_from(
        src: &'a [::core::primitive::u8],
    ) -> ::core::result::Result<Self, Self::Error> {
        Self::const_decode(src).map(|(_, this)| this)
    }
}
impl ::grost::__private::varing::Varint for Color {
    const MAX_ENCODED_LEN: ::core::primitive::usize = 5usize;
    const MIN_ENCODED_LEN: ::core::primitive::usize = 1usize;
    #[inline]
    fn encode(
        &self,
        buf: &mut [::core::primitive::u8],
    ) -> ::core::result::Result<
        ::core::primitive::usize,
        ::grost::__private::varing::EncodeError,
    > {
        self.const_encode_to(buf)
    }
    #[inline]
    fn encoded_len(&self) -> ::core::primitive::usize {
        self.const_encoded_len()
    }
    #[inline]
    fn decode(
        src: &[::core::primitive::u8],
    ) -> ::core::result::Result<
        (::core::primitive::usize, Self),
        ::grost::__private::varing::DecodeError,
    > {
        Self::const_decode(src)
    }
}
impl Color {
    /// The maximum encoded length in bytes.
    pub const MAX_VARINT_ENCODED_LEN: ::core::primitive::usize = 5usize;
    /// The minimum encoded length in bytes.
    pub const MIN_VARINT_ENCODED_LEN: ::core::primitive::usize = 1usize;
    /// The encoded length of the enum variant [`Color::Red`]
    pub const RED_VARINT_ENCODED_LEN: ::core::primitive::usize = Self::RED_VARINT_ENCODED
        .len();
    /// The encoded bytes of the enum variant [`Color::Red`]
    pub const RED_VARINT_ENCODED: ::grost::__private::varing::utils::Buffer<
        { 5usize + 1 },
    > = ::grost::__private::varing::encode_u32_varint(1u32);
    /// The encoded length of the enum variant [`Color::Green`]
    pub const GREEN_VARINT_ENCODED_LEN: ::core::primitive::usize = Self::GREEN_VARINT_ENCODED
        .len();
    /// The encoded bytes of the enum variant [`Color::Green`]
    pub const GREEN_VARINT_ENCODED: ::grost::__private::varing::utils::Buffer<
        { 5usize + 1 },
    > = ::grost::__private::varing::encode_u32_varint(2u32);
    /// The encoded length of the enum variant [`Color::Blue`]
    pub const BLUE_VARINT_ENCODED_LEN: ::core::primitive::usize = Self::BLUE_VARINT_ENCODED
        .len();
    /// The encoded bytes of the enum variant [`Color::Blue`]
    pub const BLUE_VARINT_ENCODED: ::grost::__private::varing::utils::Buffer<
        { 5usize + 1 },
    > = ::grost::__private::varing::encode_u32_varint(3u32);
    /// Returns the enum variant from the raw representation.
    #[inline]
    pub const fn from_u32(repr: ::core::primitive::u32) -> Self {
        match repr {
            1u32 => Self::Red,
            2u32 => Self::Green,
            3u32 => Self::Blue,
            val => Self::Unknown(val),
        }
    }
    /// Returns the raw representation of the enum variant.
    #[inline]
    pub const fn as_u32(&self) -> ::core::primitive::u32 {
        match self {
            Self::Red => 1u32,
            Self::Green => 2u32,
            Self::Blue => 3u32,
            Self::Unknown(val) => *val,
        }
    }
    /// Returns the encoded bytes of the enum variant.
    #[inline]
    pub const fn const_encode(
        &self,
    ) -> ::grost::__private::varing::utils::Buffer<{ 5usize + 1 }> {
        match self {
            Self::Red => Self::RED_VARINT_ENCODED,
            Self::Green => Self::GREEN_VARINT_ENCODED,
            Self::Blue => Self::BLUE_VARINT_ENCODED,
            Self::Unknown(val) => ::grost::__private::varing::encode_u32_varint(*val),
        }
    }
    /// Returns the encoded bytes of the enum variant.
    #[inline]
    pub const fn const_encode_to(
        &self,
        buf: &mut [::core::primitive::u8],
    ) -> ::core::result::Result<
        ::core::primitive::usize,
        ::grost::__private::varing::EncodeError,
    > {
        match self {
            Self::Red => {
                let buf_len = buf.len();
                if buf_len < Self::RED_VARINT_ENCODED_LEN {
                    return ::core::result::Result::Err(
                        ::grost::__private::varing::EncodeError::underflow(
                            Self::RED_VARINT_ENCODED_LEN,
                            buf_len,
                        ),
                    );
                }
                let (b, _) = buf.split_at_mut(Self::RED_VARINT_ENCODED_LEN);
                b.copy_from_slice(Self::RED_VARINT_ENCODED.as_slice());
                ::core::result::Result::Ok(Self::RED_VARINT_ENCODED_LEN)
            }
            Self::Green => {
                let buf_len = buf.len();
                if buf_len < Self::GREEN_VARINT_ENCODED_LEN {
                    return ::core::result::Result::Err(
                        ::grost::__private::varing::EncodeError::underflow(
                            Self::GREEN_VARINT_ENCODED_LEN,
                            buf_len,
                        ),
                    );
                }
                let (b, _) = buf.split_at_mut(Self::GREEN_VARINT_ENCODED_LEN);
                b.copy_from_slice(Self::GREEN_VARINT_ENCODED.as_slice());
                ::core::result::Result::Ok(Self::GREEN_VARINT_ENCODED_LEN)
            }
            Self::Blue => {
                let buf_len = buf.len();
                if buf_len < Self::BLUE_VARINT_ENCODED_LEN {
                    return ::core::result::Result::Err(
                        ::grost::__private::varing::EncodeError::underflow(
                            Self::BLUE_VARINT_ENCODED_LEN,
                            buf_len,
                        ),
                    );
                }
                let (b, _) = buf.split_at_mut(Self::BLUE_VARINT_ENCODED_LEN);
                b.copy_from_slice(Self::BLUE_VARINT_ENCODED.as_slice());
                ::core::result::Result::Ok(Self::BLUE_VARINT_ENCODED_LEN)
            }
            Self::Unknown(val) => {
                ::grost::__private::varing::encode_u32_varint_to(*val, buf)
            }
        }
    }
    /// Decodes the enum variant from the encoded bytes.
    ///
    /// Returns the number of bytes readed and `Self`.
    #[inline]
    pub const fn const_decode(
        src: &[::core::primitive::u8],
    ) -> ::core::result::Result<
        (::core::primitive::usize, Self),
        ::grost::__private::varing::DecodeError,
    > {
        ::core::result::Result::Ok(
            match src {
                [1u8] => ([1u8].len(), Self::Red),
                [2u8] => ([2u8].len(), Self::Green),
                [3u8] => ([3u8].len(), Self::Blue),
                src => {
                    return match ::grost::__private::varing::decode_u32_varint(src) {
                        ::core::result::Result::Ok((read, val)) => {
                            ::core::result::Result::Ok((read, Self::Unknown(val)))
                        }
                        ::core::result::Result::Err(e) => ::core::result::Result::Err(e),
                    };
                }
            },
        )
    }
    /// Returns the encoded length of the enum variant.
    #[inline]
    pub const fn const_encoded_len(&self) -> ::core::primitive::usize {
        match self {
            Self::Red => Self::RED_VARINT_ENCODED_LEN,
            Self::Green => Self::GREEN_VARINT_ENCODED_LEN,
            Self::Blue => Self::BLUE_VARINT_ENCODED_LEN,
            Self::Unknown(val) => {
                ::grost::__private::varing::encoded_u32_varint_len(*val)
            }
        }
    }
}
impl ::grost::__private::Wirable<::grost::__private::flavors::Network> for Color {
    const WIRE_TYPE: <::grost::__private::flavors::Network as ::grost::__private::Flavor>::WireType = <::core::primitive::u32 as ::grost::__private::Wirable<
        ::grost::__private::flavors::Network,
    >>::WIRE_TYPE;
}
impl ::grost::__private::Encode<::grost::__private::flavors::Network> for Color {
    #[inline]
    fn encode(
        &self,
        ctx: &<::grost::__private::flavors::Network as ::grost::__private::Flavor>::Context,
        buf: &mut [::core::primitive::u8],
    ) -> ::core::result::Result<
        ::core::primitive::usize,
        <::grost::__private::flavors::Network as ::grost::__private::Flavor>::EncodeError,
    > {
        <::core::primitive::u32 as ::grost::__private::Encode<
            ::grost::__private::flavors::Network,
        >>::encode(&self.as_u32(), ctx, buf)
    }
    #[inline]
    fn encoded_len(
        &self,
        ctx: &<::grost::__private::flavors::Network as ::grost::__private::Flavor>::Context,
    ) -> ::core::primitive::usize {
        <::core::primitive::u32 as ::grost::__private::Encode<
            ::grost::__private::flavors::Network,
        >>::encoded_len(&self.as_u32(), ctx)
    }
}
impl ::grost::__private::PartialEncode<::grost::__private::flavors::Network> for Color {
    type Selection = ();
    #[inline]
    fn partial_encode(
        &self,
        context: &<::grost::__private::flavors::Network as ::grost::__private::Flavor>::Context,
        buf: &mut [::core::primitive::u8],
        _: &Self::Selection,
    ) -> ::core::result::Result<
        ::core::primitive::usize,
        <::grost::__private::flavors::Network as ::grost::__private::Flavor>::EncodeError,
    > {
        ::grost::__private::Encode::<
            ::grost::__private::flavors::Network,
        >::encode(self, context, buf)
    }
    #[inline]
    fn partial_encoded_len(
        &self,
        context: &<::grost::__private::flavors::Network as ::grost::__private::Flavor>::Context,
        _: &Self::Selection,
    ) -> ::core::primitive::usize {
        ::grost::__private::Encode::<
            ::grost::__private::flavors::Network,
        >::encoded_len(self, context)
    }
}
impl<'de> ::grost::__private::Decode<'de, ::grost::__private::flavors::Network, Self>
for Color {
    #[inline]
    fn decode<UB>(
        ctx: &<::grost::__private::flavors::Network as ::grost::__private::Flavor>::Context,
        src: &'de [::core::primitive::u8],
    ) -> ::core::result::Result<
        (::core::primitive::usize, Self),
        <::grost::__private::flavors::Network as ::grost::__private::Flavor>::DecodeError,
    >
    where
        UB: ::grost::__private::UnknownBuffer<
                ::grost::__private::flavors::Network,
                &'de [::core::primitive::u8],
            > + 'de,
    {
        <::core::primitive::u32 as ::grost::__private::Decode<
            'de,
            ::grost::__private::flavors::Network,
            ::core::primitive::u32,
        >>::decode::<UB>(ctx, src)
            .map(|(read, val)| (read, Self::from_u32(val)))
    }
}
impl ::grost::__private::DecodeOwned<::grost::__private::flavors::Network, Self>
for Color {
    #[inline]
    fn decode_owned<B, UB>(
        ctx: &<::grost::__private::flavors::Network as ::grost::__private::Flavor>::Context,
        src: B,
    ) -> ::core::result::Result<
        (::core::primitive::usize, Self),
        <::grost::__private::flavors::Network as ::grost::__private::Flavor>::DecodeError,
    >
    where
        Self: ::core::marker::Sized + 'static,
        B: ::grost::__private::Buffer + 'static,
        UB: ::grost::__private::UnknownBuffer<::grost::__private::flavors::Network, B>
            + 'static,
    {
        <Self as ::grost::__private::Decode<
            '_,
            ::grost::__private::flavors::Network,
            Self,
        >>::decode::<()>(ctx, src.as_bytes())
    }
}
impl ::grost::__private::IntoTarget<::grost::__private::flavors::Network, Self>
for Color {
    #[inline]
    fn into_target(
        self,
    ) -> ::core::result::Result<
        Self,
        <::grost::__private::flavors::Network as ::grost::__private::Flavor>::DecodeError,
    > {
        ::core::result::Result::Ok(self)
    }
}
impl ::grost::__private::TypeRef<::grost::__private::flavors::Network, Self> for Color {
    #[inline]
    fn to(
        &self,
    ) -> ::core::result::Result<
        Self,
        <::grost::__private::flavors::Network as ::grost::__private::Flavor>::DecodeError,
    > {
        ::core::result::Result::Ok(*self)
    }
}
impl ::grost::__private::TypeOwned<::grost::__private::flavors::Network, Self>
for Color {
    #[inline]
    fn to(
        &self,
    ) -> ::core::result::Result<
        Self,
        <::grost::__private::flavors::Network as ::grost::__private::Flavor>::DecodeError,
    > {
        ::core::result::Result::Ok(*self)
    }
}
impl<'a> ::grost::__private::TypeBorrowed<'a, ::grost::__private::flavors::Network, Self>
for Color {
    fn from_borrow(val: &'a Self) -> Self {
        *val
    }
}
impl<'a> ::core::convert::From<&'a Self> for Color {
    #[inline]
    fn from(e: &'a Self) -> Self {
        *e
    }
}
impl ::grost::__private::PartialMessage<::grost::__private::flavors::Network> for Color {
    type UnknownBuffer<B: ?::core::marker::Sized> = ();
    type Encoded<'a> = Self where Self: ::core::marker::Sized + 'a;
    type Borrowed<'a> = Self where Self: 'a;
    type EncodedOwned = Self where Self: ::core::marker::Sized;
}
impl ::grost::__private::Message<::grost::__private::flavors::Network> for Color {
    type Partial = Self;
    type Encoded<'a> = Self where Self: ::core::marker::Sized + 'a;
    type Borrowed<'a> = Self where Self: 'a;
    type EncodedOwned = Self where Self: ::core::marker::Sized;
}
#[cfg(feature = "quickcheck")]
const _: () = {
    use ::grost::__private::quickcheck::{Arbitrary, Gen};
    impl Color {
        const __QUICKCHECK_VARIANTS: &[Self] = &[Self::Red, Self::Green, Self::Blue];
    }
    impl Arbitrary for Color {
        fn arbitrary(g: &mut Gen) -> Self {
            *g.choose(Self::__QUICKCHECK_VARIANTS).unwrap()
        }
    }
};
#[cfg(test)]
#[allow(non_snake_case)]
mod __quickcheck_fuzzy_network_flavor_color__ {
    use super::Color;
    use ::grost::__private::{Encode, Decode};
    ::grost::__private::quickcheck::quickcheck! {
        fn fuzzy_color(ctx : < ::grost::__private::flavors::Network as
        ::grost::__private::Flavor > ::Context, value : Color) -> bool { extern crate
        std; let encoded_len = value.encoded_len(& ctx); let mut buf = ::std::vec![0u8;
        encoded_len]; let ::core::result::Result::Ok(written) = value.encode(& ctx, & mut
        buf) else { return false; }; if written != encoded_len { return false; } let
        ::core::result::Result::Ok((read, decoded)) = Color::decode:: < () > (& ctx, &
        buf[..encoded_len]) else { return false; }; if decoded != value || read !=
        written { return false; } true }
    }
}
