use std::boxed::Box;

#[cfg(feature = "smol_str_0_3")]
const _: () = {
  use smol_str_0_3::SmolStr;

  str_bridge!(Box<str> {
    from_str: |val: &str| Ok(Box::<str>::from(val));
    to_str: AsRef::as_ref;
  
    type EncodedOwned = SmolStr {
      from_ref: |s: &SmolStr| Ok(Box::<str>::from(s.as_str()));
      from: |s: SmolStr| Ok(Box::from(s.as_str()));
    }
  },);
};
