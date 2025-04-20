use std::num::NonZeroU8;

use grost_proto::{
  Tag,
  reflection::{UnitEnumRepr, UnitEnumVariantValue},
};
use syn::parse_quote;
// use grost_proto::{Tag, WireType};

// use crate::{Field, Struct, field::getter, ty::Ty};

use crate::{Field, Struct, field::getter, ty::Ty};

use super::{DefaultGenerator, Generator, SafeIdent, UnitEnum, UnitEnumVariant};

#[test]
fn test_enum_generate() {
  let generator = DefaultGenerator::new();
  let enum_ = UnitEnum::new(
    SafeIdent::new("Color"),
    UnitEnumRepr::U32,
    vec![
      UnitEnumVariant::new(
        SafeIdent::new("Red"),
        UnitEnumVariantValue::U8(NonZeroU8::new(1).unwrap()),
      ),
      UnitEnumVariant::new(
        SafeIdent::new("Green"),
        UnitEnumVariantValue::U8(NonZeroU8::new(2).unwrap()),
      ),
      UnitEnumVariant::new(
        SafeIdent::new("Blue"),
        UnitEnumVariantValue::U8(NonZeroU8::new(3).unwrap()),
      ),
    ],
  );

  let generated = generator.generate_unit_enum(&enum_).unwrap();
  let file: syn::File = syn::parse2(generated).unwrap();
  let output = prettyplease::unparse(&file);
  println!("{}", output);
}

#[test]
fn t() {
  let t = Ty::optional(Ty::primitive(
    parse_quote!(::core::option::Option<::std::string::String>),
    "String",
  ))
  .to_type_reflection(
    &parse_quote!(grost),
    &super::Flavor::network(&parse_quote!(grost)),
  );
  println!("{}", t);
  let file: syn::File = syn::parse2(t).unwrap();
  let output = prettyplease::unparse(&file);
  println!("{}", output);
}

#[test]
fn test_struct_generate() {
  let generator = DefaultGenerator::new();
  let fields = vec![
    Field::new(
      SafeIdent::new("name"),
      Ty::primitive(parse_quote!(String), "String!"),
      Tag::new(1),
    ),
    Field::new(
      SafeIdent::new("age"),
      Ty::primitive(parse_quote!(u32), "u32!").with_copy(),
      Tag::new(2),
    ),
    Field::new(
      SafeIdent::new("email"),
      Ty::optional(Ty::primitive(
        parse_quote!(::core::option::Option<::std::string::String>),
        "String",
      )),
      Tag::new(3),
    )
    .with_getter(
      None,
      Option::<&str>::None,
      Some(getter::Converter::new(
        parse_quote!(core::option::Option::as_deref),
        parse_quote!(Option<&str>),
      )),
      false,
    ),
  ];
  let struct_ = Struct::new(SafeIdent::new("User"), fields)
    .with_description("A user struct")
    .with_visibility(parse_quote!(pub));

  let generated = generator.generate_struct(&struct_).unwrap();
  let file: syn::File = syn::parse2(generated).unwrap();
  let output = prettyplease::unparse(&file);
  println!("{}", output);
}
