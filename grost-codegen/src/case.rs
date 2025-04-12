use std::collections::HashSet;

use heck::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, derive_more::IsVariant)]
pub enum Heck {
  AsciiLower,
  AsciiUpper,
  Lower,
  LowerCamel,
  Upper,
  UpperCamel,
  Snake,
  Kebab,
  ShoutySnake,
  Title,
  ShoutyKebab,
  Train,
}

impl Heck {
  pub fn all_cases(s: &str) -> HashSet<String> {
    [
      Self::AsciiLower.convert(s),
      Self::AsciiUpper.convert(s),
      Self::Lower.convert(s),
      Self::LowerCamel.convert(s),
      Self::Upper.convert(s),
      Self::UpperCamel.convert(s),
      Self::Snake.convert(s),
      Self::Kebab.convert(s),
      Self::ShoutySnake.convert(s),
      Self::Title.convert(s),
      Self::ShoutyKebab.convert(s),
      Self::Train.convert(s),
    ]
    .into_iter()
    .collect()
  }

  pub fn convert(&self, s: &str) -> String {
    match self {
      Self::AsciiLower => s.to_ascii_lowercase(),
      Self::AsciiUpper => s.to_ascii_uppercase(),
      Self::Lower => s.to_lowercase(),
      Self::LowerCamel => s.to_lower_camel_case(),
      Self::Upper => s.to_uppercase(),
      Self::UpperCamel => s.to_upper_camel_case(),
      Self::Snake => s.to_snake_case(),
      Self::Kebab => s.to_kebab_case(),
      Self::ShoutySnake => s.to_shouty_snake_case(),
      Self::Title => s.to_title_case(),
      Self::ShoutyKebab => s.to_shouty_kebab_case(),
      Self::Train => s.to_train_case(),
    }
  }
}
