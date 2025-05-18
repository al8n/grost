
pub use enum_::*;
pub use object::*;

mod enum_;
mod object;


/// The type in the Graph protocol schema
#[derive(Debug)]
pub enum Type {
  Scalar {
    name: &'static str,
    description: &'static str,
  },
  List(&'static Type),
  Set(&'static Type),
  Map {
    key: &'static Type,
    value: &'static Type,
  },
  Optional(&'static Type),
  Object(&'static Object),
  Enum(&'static Enum),
  Union(),
  Interface(),
}

impl Clone for Type {
  fn clone(&self) -> Self {
    *self
  }
}

impl Copy for Type {}

impl Type {
  /// Construct a scalar type
  pub const fn scalar(name: &'static str, description: &'static str) -> Self {
    Self::Scalar { name, description }
  }

  /// Creates a string schema type
  pub const fn string() -> Self {
    Self::scalar("string", "A string")
  }

  /// Creates a bytes schema type
  pub const fn bytes() -> Self {
    Self::scalar("bytes", "A bytes")
  }

  /// Creates a boolean schema type
  pub const fn duration() -> Self {
    Self::scalar("Duration", "A duration")
  }

  /// Creates a UTC schema type
  pub const fn utc() -> Self {
    Self::scalar("Utc", "A UTC")
  }

  /// Returns `true` if this type is `byte` or `u8`
  pub const fn is_byte(self) -> bool {
    match self {
      Type::Scalar { name, .. } => matches!(name.as_bytes(), b"byte" | b"u8"),
      _ => false,
    }
  }
}
