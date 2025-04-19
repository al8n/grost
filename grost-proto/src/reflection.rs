use super::flavors::Flavor;

pub use struct_::*;
pub use unit_enum::*;

mod struct_;
mod unit_enum;

#[derive(Debug, Clone, Copy)]
pub enum TypeReflection<F: Flavor> {
  Primitive {
    name: &'static str,
    schema_name: &'static str,
    description: &'static str,
  },
  List(&'static TypeReflection<F>),
  Map {
    key: &'static TypeReflection<F>,
    value: &'static TypeReflection<F>,
  },
  Struct(StructReflection<F>),
  UintEnum(UnitEnumReflection),
  NamedEnum(),
}
