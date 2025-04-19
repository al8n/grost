/// The value of an unit enum variant
#[derive(
  Clone,
  Copy,
  Debug,
  PartialEq,
  Eq,
  Hash,
  derive_more::IsVariant,
  derive_more::Unwrap,
  derive_more::TryUnwrap,
)]
pub enum UnitEnumVariantReflectionValue {
  /// `u8` value, `repr(u8)`
  U8(u8),
  /// `u16` value, `repr(u16)`
  U16(u16),
  /// `u32` value, `repr(u32)`
  U32(u32),
  /// `u64` value, `repr(u64)`
  U64(u64),
  /// `i8` value, `repr(i8)`
  I8(i8),
  /// `i16` value, `repr(i16)`
  I16(i16),
  /// `i32` value, `repr(i32)`
  I32(i32),
  /// `i64` value, `repr(i64)`
  I64(i64),
}

#[doc(hidden)]
pub struct UnitEnumVariantReflectionReflectionBuilder {
  pub name: &'static str,
  pub schema_name: &'static str,
  pub description: &'static str,
  pub value: UnitEnumVariantReflectionValue,
}

impl UnitEnumVariantReflectionReflectionBuilder {
  #[inline]
  pub const fn build(self) -> UnitEnumVariantReflectionReflection {
    UnitEnumVariantReflectionReflection {
      name: self.name,
      schema_name: self.schema_name,
      description: self.description,
      value: self.value,
    }
  }
}

/// The information of a variant of enum in the Graph protocol buffer
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UnitEnumVariantReflectionReflection {
  name: &'static str,
  schema_name: &'static str,
  description: &'static str,
  value: UnitEnumVariantReflectionValue,
}

impl UnitEnumVariantReflectionReflection {
  /// Get the name of the variant
  #[inline]
  pub const fn name(&self) -> &'static str {
    self.name
  }

  /// Get the schema name of the variant
  #[inline]
  pub const fn schema_name(&self) -> &'static str {
    self.schema_name
  }

  /// Get the value of the variant
  #[inline]
  pub const fn value(&self) -> UnitEnumVariantReflectionValue {
    self.value
  }

  /// Get the description of the variant
  #[inline]
  pub const fn description(&self) -> &'static str {
    self.description
  }
}

#[doc(hidden)]
pub struct UnitEnumReflectionBuilder {
  pub name: &'static str,
  pub schema_name: &'static str,
  pub variants: &'static [UnitEnumVariantReflectionReflection],
  pub description: &'static str,
}

impl UnitEnumReflectionBuilder {
  #[inline]
  pub const fn build(self) -> UnitEnumReflection {
    UnitEnumReflection {
      name: self.name,
      schema_name: self.schema_name,
      variants: self.variants,
      description: self.description,
    }
  }
}

/// The information of an enum in the Graph protocol buffer
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UnitEnumReflection {
  name: &'static str,
  schema_name: &'static str,
  description: &'static str,
  variants: &'static [UnitEnumVariantReflectionReflection],
}

impl UnitEnumReflection {
  /// Get the name of the enum
  #[inline]
  pub const fn name(&self) -> &'static str {
    self.name
  }

  /// Get the schema name of the enum
  #[inline]
  pub const fn schema_name(&self) -> &'static str {
    self.schema_name
  }

  /// Get the variants of the enum
  #[inline]
  pub const fn variants(&self) -> &'static [UnitEnumVariantReflectionReflection] {
    self.variants
  }

  /// Get the description of the enum
  #[inline]
  pub const fn description(&self) -> &'static str {
    self.description
  }
}
