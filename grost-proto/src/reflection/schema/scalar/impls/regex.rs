#[cfg(feature = "regex_1")]
const _: () = {
  use regex_1::{Regex, bytes::Regex as BytesRegex};

  impl_reflectable_with_variant!(
    Regex:Regex,
    BytesRegex:BytesRegex,
  );
};
