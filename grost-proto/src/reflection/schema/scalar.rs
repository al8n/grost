mod impls;

/// The Graph protocols
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, derive_more::IsVariant)]
#[non_exhaustive]
pub enum Scalar {
  /// string
  String,
  /// BString
  BString,
  /// bytes
  Bytes,
  /// The byte type, the same as `Scalar::Unsigned(8)`
  #[is_variant(ignore)]
  Byte,
  /// bool
  Boolean,
  /// The char type
  Char,
  /// The complex number type, which based on the signed or unsigned integer type
  #[is_variant(ignore)]
  ComplexInteger {
    /// The number of bits
    bits: usize,
    /// If true, the value is signed
    signed: bool,
  },
  /// The complex number type, which based on the floating point type
  #[is_variant(ignore)]
  ComplexFloat(usize),
  /// The rational number type
  #[is_variant(ignore)]
  Rational {
    /// The number of bits
    bits: usize,
    /// If true, the value is signed
    signed: bool,
  },
  /// Unsigned N-bit integer
  #[is_variant(ignore)]
  Integer {
    /// The number of bits
    bits: usize,
    /// Non-zero, if true, the value must be non-zero
    non_zero: bool,
    /// If true, the value is signed
    signed: bool,
  },
  /// N-bit float
  Float(usize),
  /// Duration
  Duration,
  /// Date
  Date,
  /// Date time
  DateTime,
  /// The timezone
  Timezone,
  /// The time
  Time,
  /// Utc
  Utc,
  /// The uuid type
  Uuid,
  /// The Ipv4 address
  Ipv4,
  /// The Ipv6 address
  Ipv6,
  /// The Ip address
  Ip,
  /// The socket address
  SocketAddr,
  /// The socket address v4
  SocketAddrV4,
  /// The socket address v6
  SocketAddrV6,
  /// The regex type
  Regex,
  /// The bytes regex type
  BytesRegex,
  /// Custom
  Custom {
    name: &'static str,
    description: &'static str,
  },
}

impl core::fmt::Display for Scalar {
  fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match self {
      Self::String => write!(fmt, "string"),
      Self::BString => write!(fmt, "bstring"),
      Self::Byte => write!(fmt, "byte"),
      Self::Bytes => write!(fmt, "bytes"),
      Self::Boolean => write!(fmt, "bool"),
      Self::Date => write!(fmt, "date"),
      Self::DateTime => write!(fmt, "datetime"),
      Self::Time => write!(fmt, "time"),
      Self::Char => write!(fmt, "char"),
      Self::Integer {
        bits,
        non_zero,
        signed,
      } => match (signed, non_zero) {
        (true, false) => write!(fmt, "i{bits}"),
        (false, false) => {
          if *bits == 8 {
            return write!(fmt, "byte");
          }
          write!(fmt, "u{bits}")
        }
        (true, true) => write!(fmt, "NonZeroI{bits}"),
        (false, true) => write!(fmt, "NonZeroU{bits}"),
      },
      Self::ComplexFloat(n) => write!(fmt, "f{n}c"),
      Self::ComplexInteger { signed, bits } => {
        if *signed {
          write!(fmt, "i{bits}c")
        } else {
          write!(fmt, "u{bits}c")
        }
      }
      Self::Rational { signed, bits } => {
        if *signed {
          write!(fmt, "i{bits}r")
        } else {
          write!(fmt, "u{bits}r")
        }
      }
      Self::Float(n) => write!(fmt, "f{n}"),
      Self::Duration => write!(fmt, "duration"),
      Self::Utc => write!(fmt, "utc"),
      Self::Uuid => write!(fmt, "uuid"),
      Self::Timezone => write!(fmt, "timezone"),
      Self::Ipv4 => write!(fmt, "ipv4"),
      Self::Ipv6 => write!(fmt, "ipv6"),
      Self::Ip => write!(fmt, "ip"),
      Self::SocketAddr => write!(fmt, "SocketAddr"),
      Self::SocketAddrV4 => write!(fmt, "SocketAddrV4"),
      Self::SocketAddrV6 => write!(fmt, "SocketAddrV6"),
      Self::Regex => write!(fmt, "regex"),
      Self::BytesRegex => write!(fmt, "bregex"),
      Self::Custom { name, .. } => core::fmt::Debug::fmt(name, fmt),
    }
  }
}

impl Scalar {
  /// Creates a new unsigned integer scalar
  /// with the given number of bits.
  #[inline]
  pub const fn uint(n: usize) -> Self {
    Self::Integer {
      bits: n,
      non_zero: false,
      signed: false,
    }
  }

  /// Creates a new non-zero unsigned integer scalar
  /// with the given number of bits.
  #[inline]
  pub const fn non_zero_uint(n: usize) -> Self {
    Self::Integer {
      bits: n,
      non_zero: true,
      signed: false,
    }
  }

  /// Creates a new signed integer scalar
  /// with the given number of bits.
  #[inline]
  pub const fn int(n: usize) -> Self {
    Self::Integer {
      bits: n,
      non_zero: false,
      signed: true,
    }
  }

  /// Creates a new non-zero signed integer scalar
  /// with the given number of bits.
  #[inline]
  pub const fn non_zero_int(n: usize) -> Self {
    Self::Integer {
      bits: n,
      non_zero: true,
      signed: true,
    }
  }

  /// Creates a new unsigned complex scalar
  /// with the given number of bits.
  #[inline]
  pub const fn unsigned_complex(n: usize) -> Self {
    Self::ComplexInteger {
      bits: n,
      signed: false,
    }
  }

  /// Creates a new signed complex scalar
  /// with the given number of bits.
  #[inline]
  pub const fn signed_complex(n: usize) -> Self {
    Self::ComplexInteger {
      bits: n,
      signed: true,
    }
  }

  /// Creates a new unsigned complex scalar
  /// with the given number of bits.
  #[inline]
  pub const fn float_complex(n: usize) -> Self {
    Self::ComplexFloat(n)
  }

  /// Creates a new rational scalar
  /// with the given number of bits.
  #[inline]
  pub const fn unsigned_rational(n: usize) -> Self {
    Self::Rational {
      bits: n,
      signed: false,
    }
  }

  /// Creates a new rational scalar
  /// with the given number of bits.
  #[inline]
  pub const fn signed_rational(n: usize) -> Self {
    Self::Rational {
      bits: n,
      signed: true,
    }
  }

  /// Creates a new float scalar
  /// with the given number of bits.
  #[inline]
  pub const fn float(n: usize) -> Self {
    Self::Float(n)
  }

  /// Creates a char scalar.
  #[inline]
  pub const fn char() -> Self {
    Self::Char
  }

  /// Creates a bool scalar.
  #[inline]
  pub const fn bool() -> Self {
    Self::Boolean
  }

  /// Creates a new custom scalar with the given name and description.
  #[inline]
  pub const fn custom(name: &'static str, description: &'static str) -> Self {
    Self::Custom { name, description }
  }

  /// Returns the description of the scalar.
  #[inline]
  pub const fn description(&self) -> &'static str {
    match self {
      Self::String => "A string",
      Self::BString => "String which allows non utf-8 characters",
      Self::Bytes => "A bytes",
      Self::Byte => "A byte",
      Self::Boolean => "A boolean",
      Self::Char => "A utf-8 character",
      Self::ComplexInteger { signed, .. } => {
        if *signed {
          "A complex number based on a signed integer"
        } else {
          "A complex number based on an unsigned integer"
        }
      }
      Self::ComplexFloat(_) => "A complex number based on a float",
      Self::Rational { signed, .. } => {
        if *signed {
          "A signed rational number"
        } else {
          "An unsigned rational number"
        }
      }
      Self::Integer {
        signed, non_zero, ..
      } => {
        if *signed {
          if *non_zero {
            "A non-zero signed integer"
          } else {
            "A signed integer"
          }
        } else if *non_zero {
          "A non-zero unsigned integer"
        } else {
          "An unsigned integer"
        }
      }
      Self::Float(_) => "A float",
      Self::Date => "A date",
      Self::DateTime => "A date time",
      Self::Duration => "A duration",
      Self::Time => "A time",
      Self::Utc => "A UTC",
      Self::Uuid => "A UUID",
      Self::Timezone => "A timezone",
      Self::Ipv4 => "An Ipv4 address",
      Self::Ipv6 => "An Ipv6 address",
      Self::Ip => "An Ip address",
      Self::SocketAddr => "A socket address",
      Self::SocketAddrV4 => "A socket address v4",
      Self::SocketAddrV6 => "A socket address v6",
      Self::Regex => "A regex type",
      Self::BytesRegex => "A bytes regex type",
      Self::Custom { description, .. } => description,
    }
  }

  /// Returns `true` if the scalar is a byte or an unsigned 8-bit integer.
  #[inline]
  pub const fn is_byte(self) -> bool {
    matches!(
      self,
      Self::Integer {
        bits: 8,
        non_zero: false,
        signed: false,
      } | Self::Byte
    )
  }

  /// Returns `true` if the scalar is a signed integer.
  #[inline]
  pub const fn is_int(self) -> bool {
    matches!(self, Self::Integer { signed: true, .. })
  }

  /// Returns `true` if the scalar is an unsigned integer.
  #[inline]
  pub const fn is_uint(self) -> bool {
    matches!(self, Self::Integer { signed: false, .. })
  }

  /// Returns `true` if the scalar is a rational number.
  #[inline]
  pub const fn is_rational(self) -> bool {
    matches!(self, Self::Rational { .. })
  }

  /// Returns `true` if the scalar is a complex number.
  #[inline]
  pub const fn is_complex(self) -> bool {
    matches!(self, Self::ComplexInteger { .. } | Self::ComplexFloat(_))
  }
}
