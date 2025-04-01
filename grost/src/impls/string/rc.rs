use std::rc::Rc;

use crate::smol_str::SmolStr;

str_bridge!(Rc<str> {
  from_str: Rc::<str>::from,
  to_str: AsRef::as_ref,
},);

str_bridge!(
  @smolstr_message
  Rc::<str> {
    from_ref: |s: &SmolStr| {
      Rc::<str>::from(s.as_str())
    },
    from: |s: SmolStr| {
      Rc::from(s.as_str())
    },
  },
);
