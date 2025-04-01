use ::smol_str_0_3::SmolStr;

str_bridge!(SmolStr {
  from_str: |val| Ok(SmolStr::new(val));
  to_str: SmolStr::as_str;

  type SerializedOwned = SmolStr {
    from_ref: |val: &SmolStr| Ok(val.clone());
    from: |val: SmolStr| Ok(val);
  }
},);

