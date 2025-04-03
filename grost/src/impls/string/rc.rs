use std::rc::Rc;

#[cfg(feature = "smol_str_0_3")]
const _: () = {
  use smol_str_0_3::SmolStr;

  str_bridge!(Rc<str> {
    from_str: |val: &str| Ok(Rc::<str>::from(val));
    to_str: AsRef::as_ref;
  
    type EncodedOwned = SmolStr {
      from_ref: |s: &SmolStr| Ok(Rc::<str>::from(s.as_str()));
      from: |s: SmolStr| Ok(Rc::from(s.as_str()));
    }
  },);
};
