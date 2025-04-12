use std::num::NonZeroI128;

use super::{DefaultGenerator, Enum, EnumVariant, EnumRepr, Generator, SafeIdent};


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
