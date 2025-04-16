use std::{fs::OpenOptions, io::Write, num::NonZeroI128, path::PathBuf};

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
  // lib
  //   .write_all(format!("mod {ENUM_FILE_NAME};").as_bytes())
  //   .unwrap();
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
      EnumVariant::new(SafeIdent::new("Red"), NonZeroI128::new(1).unwrap()).with_default(true),
      EnumVariant::new(SafeIdent::new("Green"), NonZeroI128::new(2).unwrap()),
      EnumVariant::new(SafeIdent::new("Blue"), NonZeroI128::new(3).unwrap()),
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
