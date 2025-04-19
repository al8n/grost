use std::{fs::OpenOptions, io::Write, num::NonZeroU32, path::PathBuf};

use grost_codegen::*;

const ENUM_FILE_NAME: &str = "color_enum";
const STRUCT_FILE_NAME: &str = "user_struct";

fn main() {
  enum_codegen_test(ENUM_FILE_NAME);

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
  let enum_ = UnitEnum::new(
    SafeIdent::new("Color"),
    UnitEnumRepr::U32,
    vec![
      UnitEnumVariant::new(SafeIdent::new("Red"), UnitEnumVariantValue::U32(NonZeroU32::new(1).unwrap())).with_default(true),
      UnitEnumVariant::new(SafeIdent::new("Green"), UnitEnumVariantValue::U32(NonZeroU32::new(2).unwrap())),
      UnitEnumVariant::new(SafeIdent::new("Blue"), UnitEnumVariantValue::U32(NonZeroU32::new(3).unwrap())),
    ],
  );

  let generated = generator.generate_unit_enum(&enum_).unwrap();
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
