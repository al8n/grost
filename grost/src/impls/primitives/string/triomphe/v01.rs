use triomphe_0_1::Arc;

#[cfg(feature = "smol_str_0_3")]
const _: () = {
  use smol_str_0_3::SmolStr;

  str_bridge!(Arc<str> {
    from_str: |val: &str| Ok(Arc::<str>::from(val));
    to_str: AsRef::as_ref;
  
    type EncodedOwned = SmolStr {
      from_ref: |s: &SmolStr| Ok(Arc::<str>::from(s.as_str()));
      from: |s: SmolStr| Ok(Arc::from(s.as_str()));
    }
  },);
};
