use std::{fs::OpenOptions, io::Write, num::NonZeroU32, path::PathBuf};

use grost_codegen::{field::getter, ty::Ty, *};
use syn::parse_quote;

const ENUM_FILE_NAME: &str = "color_enum";
const STRUCT_FILE_NAME: &str = "user_struct";

fn main() {
  enum_codegen_test(ENUM_FILE_NAME);
  struct_codegen_test(STRUCT_FILE_NAME);

  let mut lib = OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .open("src/lib.rs")
    .unwrap();
  lib
    .write_all(format!("mod {ENUM_FILE_NAME};").as_bytes())
    .unwrap();
  lib
    .write_all(format!("mod {STRUCT_FILE_NAME};").as_bytes())
    .unwrap();
}

fn enum_codegen_test(name: &str) {
  let generator = DefaultGenerator::new();
  let enum_ = Enum::new(
    SafeIdent::new("Color"),
    EnumRepr::U32,
    vec![
      EnumVariant::new(
        SafeIdent::new("Red"),
        EnumVariantValue::U32(NonZeroU32::new(1).unwrap()),
      )
      .with_default(true),
      EnumVariant::new(
        SafeIdent::new("Green"),
        EnumVariantValue::U32(NonZeroU32::new(2).unwrap()),
      ),
      EnumVariant::new(
        SafeIdent::new("Blue"),
        EnumVariantValue::U32(NonZeroU32::new(3).unwrap()),
      ),
    ],
  );

  let generated = generator.generate_enum(&enum_).unwrap();
  let file: syn::File = syn::parse2(generated).unwrap();
  let output = prettyplease::unparse(&file);

  let mut path = PathBuf::from("src");
  path.push(name);
  path.set_extension("rs");

  let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .open(&path)
    .unwrap();

  file.write_all(b"#![no_implicit_prelude]\n\n").unwrap();
  file.write_all(output.as_bytes()).unwrap();
}

fn struct_codegen_test(name: &str) {
  let generator = DefaultGenerator::new();
  let fields = vec![
    Field::new(
      SafeIdent::new("name"),
      Ty::primitive(parse_quote!(::std::string::String), "String!"),
      1,
    ),
    Field::new(
      SafeIdent::new("age"),
      Ty::primitive(parse_quote!(u32), "u32").with_copy(),
      2,
    ),
    Field::new(
      SafeIdent::new("email"),
      Ty::optional(Ty::primitive(parse_quote!(::std::string::String), "String")),
      3,
    ),
  ];
  let struct_ = Struct::new(SafeIdent::new("User"), fields)
    .with_description("A user struct")
    .with_visibility(parse_quote!(pub));

  let generated = generator.generate_struct(&struct_).unwrap();
  let file: syn::File = syn::parse2(generated).unwrap();
  let output = prettyplease::unparse(&file);

  let mut path = PathBuf::from("src");
  path.push(name);
  path.set_extension("rs");

  let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .open(&path)
    .unwrap();

  file.write_all(b"#![no_implicit_prelude]\n\n").unwrap();
  file.write_all(output.as_bytes()).unwrap();

  let fields = vec![
    Field::new(
      SafeIdent::new("user"),
      Ty::struct_(parse_quote!(User), "User!"),
      1,
    ),
    Field::new(
      SafeIdent::new("title"),
      Ty::primitive(parse_quote!(::std::string::String), "String!"),
      2,
    ),
    Field::new(
      SafeIdent::new("content"),
      Ty::optional(Ty::primitive(parse_quote!(::std::string::String), "String")),
      3,
    ),
  ];
  let struct_ = Struct::new(SafeIdent::new("Comment"), fields)
    .with_description("A comment struct")
    .with_visibility(parse_quote!(pub));

  let generated = generator.generate_struct(&struct_).unwrap();
  let ts: syn::File = syn::parse2(generated).unwrap();
  let output = prettyplease::unparse(&ts);
  file.write_all(output.as_bytes()).unwrap();
}
