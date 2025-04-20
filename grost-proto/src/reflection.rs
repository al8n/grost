use super::flavors::Flavor;

pub use struct_::*;
pub use unit_enum::*;

mod struct_;
mod unit_enum;

#[derive(Debug)]
pub enum Type<F: Flavor> {
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
  UintEnum(UnitEnumReflection),
  Union(),
  Interface(),
}

impl<F: Flavor> Clone for Type<F> {
  fn clone(&self) -> Self {
    *self
  }
}

impl<F: Flavor> Copy for Type<F> {}
