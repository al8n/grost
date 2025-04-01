use crate::smol_str::SmolStr;
use std::string::String;

str_bridge!(String {
  from_str: String::from,
  to_str: String::as_str,
},);

str_bridge!(
  @smolstr_message
  String {
    from_ref: |s: &SmolStr| String::from(s.as_str()),
    from: Into::into,
  },
);
