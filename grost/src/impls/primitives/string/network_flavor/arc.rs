#[cfg(feature = "smol_str_0_3")]
const _: () = {
  use crate::flavors::network::{Network, WireType};
  use smol_str_0_3::SmolStr;
  use std::sync::Arc;

  str_bridge!(Network:(WireType::LengthDelimited):Arc<str> {
    from_str: |val: &str| Ok(Arc::<str>::from(val));
    to_str: AsRef::as_ref;
  
    type EncodedOwned = SmolStr {
      from_ref: |s: &SmolStr| Ok(Arc::<str>::from(s.as_str()));
      from: |s: SmolStr| Ok(Arc::from(s.as_str()));
    }
  },);
};
