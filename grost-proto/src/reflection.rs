use super::flavors::Flavor;

pub use enum_::*;
pub use struct_::*;

mod enum_;
mod struct_;

/// The type in the Graph protocol schema
#[derive(Debug)]
pub enum Type<F: Flavor + ?Sized> {
  Primitive {
    name: &'static str,
    description: &'static str,
  },
  List(&'static Type<F>),
  Map {
    key: &'static Type<F>,
    value: &'static Type<F>,
  },
  Optional(&'static Type<F>),
  Struct(StructReflection<F>),
  UintEnum(EnumReflection),
  Union(),
  Interface(),
}

impl<F: Flavor + ?Sized> Clone for Type<F> {
  fn clone(&self) -> Self {
    *self
  }
}

impl<F: Flavor + ?Sized> Copy for Type<F> {}
