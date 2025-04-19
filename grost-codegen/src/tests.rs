use std::num::NonZeroU8;

use grost_proto::reflection::{UnitEnumRepr, UnitEnumVariantValue};
// use grost_proto::{Tag, WireType};

// use crate::{Field, Struct, field::getter, ty::Ty};

use super::{DefaultGenerator, UnitEnum, UnitEnumVariant, Generator, SafeIdent};

#[test]
fn test_enum_generate() {
  let generator = DefaultGenerator::new();
  let enum_ = UnitEnum::new(
    SafeIdent::new("Color"),
    UnitEnumRepr::U32,
    vec![
      UnitEnumVariant::new(SafeIdent::new("Red"), UnitEnumVariantValue::U8(NonZeroU8::new(1).unwrap())),
      UnitEnumVariant::new(SafeIdent::new("Green"), UnitEnumVariantValue::U8(NonZeroU8::new(2).unwrap())),
      UnitEnumVariant::new(SafeIdent::new("Blue"), UnitEnumVariantValue::U8(NonZeroU8::new(3).unwrap())),
    ],
  );

  let generated = generator.generate_unit_enum(&enum_).unwrap();
  let file: syn::File = syn::parse2(generated).unwrap();
  let output = prettyplease::unparse(&file);
  println!("{}", output);
}

// #[test]
// fn test_struct_generate() {
//   let generator = DefaultGenerator::new();
//   let struct_ = Struct::new(SafeIdent::new("User"))
//     .with_description("A user struct")
//     .with_visibility(parse_quote!(pub))
//     .with_fields(vec![
//       Field::new(
//         SafeIdent::new("name"),
//         Ty::new(
//           parse_quote!(String),
//           "String!".into(),
//           WireType::LengthDelimited,
//         ),
//         Tag::new(1),
//       ),
//       Field::new(
//         SafeIdent::new("age"),
//         Ty::new(parse_quote!(u32), "u32!".into(), WireType::Varint).with_copy(),
//         Tag::new(2),
//       ),
//       Field::new(
//         SafeIdent::new("email"),
//         Ty::new(
//           parse_quote!(Option<String>),
//           "String".into(),
//           WireType::LengthDelimited,
//         ),
//         Tag::new(3),
//       )
//       .with_getter(
//         None,
//         Option::<&str>::None,
//         Some(getter::Converter::new(
//           parse_quote!(core::option::Option::as_deref),
//           parse_quote!(Option<&str>),
//         )),
//         false,
//       ),
//     ]);

//   let generated = generator.generate_struct(&struct_).unwrap();
//   let file: syn::File = syn::parse2(generated).unwrap();
//   let output = prettyplease::unparse(&file);
//   println!("{}", output);
// }
