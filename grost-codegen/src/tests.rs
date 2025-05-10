// use std::num::NonZeroU8;

// use grost_proto::reflection::{EnumRepr, EnumVariantValue};
// use syn::parse_quote;
// // use grost_proto::{Tag, WireType};

// // use crate::{Field, Struct, field::getter, ty::Ty};

// use crate::{Field, Struct, field::getter, ty::Ty};

// use super::{SchemaGenerator, Enum, EnumVariant, Generator, SafeIdent};

// #[test]
// fn test_enum_generate() {
//   let generator = SchemaGenerator::new();
//   let enum_ = Enum::new(
//     SafeIdent::new("Color"),
//     EnumRepr::U32,
//     vec![
//       EnumVariant::new(
//         SafeIdent::new("Red"),
//         EnumVariantValue::U8(NonZeroU8::new(1).unwrap()),
//       ),
//       EnumVariant::new(
//         SafeIdent::new("Green"),
//         EnumVariantValue::U8(NonZeroU8::new(2).unwrap()),
//       ),
//       EnumVariant::new(
//         SafeIdent::new("Blue"),
//         EnumVariantValue::U8(NonZeroU8::new(3).unwrap()),
//       ),
//     ],
//   );

//   let generated = generator.generate_enum(&enum_).unwrap();
//   let file: syn::File = syn::parse2(generated).unwrap();
//   let output = prettyplease::unparse(&file);
//   println!("{}", output);
// }

// #[test]
// fn test_struct_generate() {
//   let generator = SchemaGenerator::new();
//   let fields = vec![
//     Field::new(
//       SafeIdent::new("name"),
//       Ty::primitive(parse_quote!(String), "String!"),
//       1,
//     ),
//     Field::new(
//       SafeIdent::new("age"),
//       Ty::primitive(parse_quote!(u32), "u32!").with_copy(),
//       2,
//     ),
//     Field::new(
//       SafeIdent::new("email"),
//       Ty::optional(Ty::primitive(
//         parse_quote!(::core::option::Option<::std::string::String>),
//         "String",
//       )),
//       3,
//     ),
//   ];
//   let user = Struct::new(SafeIdent::new("User"), fields)
//     .with_description("A user struct")
//     .with_visibility(parse_quote!(pub));

//   let fields = vec![
//     Field::new(
//       SafeIdent::new("user"),
//       Ty::struct_(parse_quote!(User), "User!"),
//       1,
//     ),
//     Field::new(
//       SafeIdent::new("replyer"),
//       Ty::optional(Ty::struct_(parse_quote!(User), "User")),
//       2,
//     ),
//     Field::new(
//       SafeIdent::new("title"),
//       Ty::primitive(parse_quote!(::std::string::String), "String!"),
//       3,
//     ),
//     Field::new(
//       SafeIdent::new("content"),
//       Ty::optional(Ty::primitive(parse_quote!(::std::string::String), "String")),
//       4,
//     ),
//   ];
//   let comment = Struct::new(SafeIdent::new("Comment"), fields)
//     .with_description("A comment struct")
//     .with_visibility(parse_quote!(pub));

//   let user_ts = generator.generate_struct(&user).unwrap();
//   let comment_ts = generator.generate_struct(&comment).unwrap();
//   let generated = quote::quote! {
//     #user_ts
//     #comment_ts
//   };
//   let file: syn::File = syn::parse2(generated).unwrap();
//   let output = prettyplease::unparse(&file);
//   println!("{}", output);
// }
