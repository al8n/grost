use std::boxed::Box;

use crate::smol_str::SmolStr;

str_bridge!(Box<str> {
  from_str: Box::<str>::from,
  to_str: AsRef::as_ref,
},);

str_bridge!(
  @smolstr_message
  Box::<str> {
    from_ref: |s: &SmolStr| {
      Box::<str>::from(s.as_str())
    },
    from: |s: SmolStr| {
      Box::from(s.as_str())
    },
  },
);
