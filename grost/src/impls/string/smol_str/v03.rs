use ::smol_str_0_3::SmolStr;

str_bridge!(SmolStr {
  from_str: SmolStr::new,
  to_str: SmolStr::as_str,
},);

str_bridge!(
  @smolstr_message
  SmolStr {
    from_ref: Clone::clone,
    from: Into::into,
  }
);
