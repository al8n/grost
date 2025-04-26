use std::num::NonZeroU8;

use grost_proto::{
  Tag,
  reflection::{EnumRepr, EnumVariantValue},
};
use syn::parse_quote;
// use grost_proto::{Tag, WireType};

// use crate::{Field, Struct, field::getter, ty::Ty};

use crate::{Field, Struct, field::getter, ty::Ty};

use super::{DefaultGenerator, Enum, EnumVariant, Generator, SafeIdent};

#[test]
fn test_enum_generate() {
  let generator = DefaultGenerator::new();
  let enum_ = Enum::new(
    SafeIdent::new("Color"),
    EnumRepr::U32,
    vec![
      EnumVariant::new(
        SafeIdent::new("Red"),
        EnumVariantValue::U8(NonZeroU8::new(1).unwrap()),
      ),
      EnumVariant::new(
        SafeIdent::new("Green"),
        EnumVariantValue::U8(NonZeroU8::new(2).unwrap()),
      ),
      EnumVariant::new(
        SafeIdent::new("Blue"),
        EnumVariantValue::U8(NonZeroU8::new(3).unwrap()),
      ),
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
    ),
    // .with_getter(
    //   None,
    //   Option::<&str>::None,
    //   Some(getter::Converter::new(
    //     parse_quote!(core::option::Option::as_deref),
    //     parse_quote!(Option<&str>),
    //   )),
    //   false,
    // ),
  ];
  let struct_ = Struct::new(SafeIdent::new("User"), fields)
    .with_description("A user struct")
    .with_visibility(parse_quote!(pub));

  let generated = generator.generate_struct(&struct_).unwrap();
  let file: syn::File = syn::parse2(generated).unwrap();
  let output = prettyplease::unparse(&file);
  println!("{}", output);
}
