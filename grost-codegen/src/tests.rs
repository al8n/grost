use std::num::NonZeroI128;

use grost_types::{Tag, WireType};
use syn::parse_quote;

use crate::{field::getter, ty::Ty, Field, Struct};

use super::{DefaultGenerator, Enum, EnumRepr, EnumVariant, Generator, SafeIdent};

#[test]
fn test_enum_generate() {
  let generator = DefaultGenerator::new();
  let enum_ = Enum::new(
    SafeIdent::new("Color"),
    EnumRepr::U32,
    vec![
      EnumVariant::new(SafeIdent::new("Red"), NonZeroI128::new(1).unwrap()),
      EnumVariant::new(SafeIdent::new("Green"), NonZeroI128::new(2).unwrap()),
      EnumVariant::new(SafeIdent::new("Blue"), NonZeroI128::new(3).unwrap()),
    ],
  );

  let generated = generator.generate_enum(&enum_).unwrap();
  let file: syn::File = syn::parse2(generated).unwrap();
  let output = prettyplease::unparse(&file);
  println!("{}", output);
}

#[test]
fn test_struct_generate() {
  let generator = DefaultGenerator::new();
  let struct_ = Struct::new(SafeIdent::new("User"))
    .with_description("A user struct")
    .with_visibility(parse_quote!(pub))
    .with_fields(vec![
      Field::new(SafeIdent::new("name"), Ty::new(parse_quote!(String), WireType::LengthDelimited), Tag::new(1)),
      Field::new(SafeIdent::new("age"), Ty::new(parse_quote!(u32), WireType::Varint).with_copy(), Tag::new(2)),
      Field::new(SafeIdent::new("email"), Ty::new(parse_quote!(Option<String>), WireType::LengthDelimited), Tag::new(3))
        .with_getter(None, Option::<&str>::None, Some(getter::Converter::new(parse_quote!(core::option::Option::as_deref), parse_quote!(Option<&str>))), false),
    ]);

  let generated = generator.generate_struct(&struct_).unwrap();
  let file: syn::File = syn::parse2(generated).unwrap();
  let output = prettyplease::unparse(&file);
  println!("{}", output);
}
