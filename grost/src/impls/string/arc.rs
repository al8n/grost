use crate::smol_str::SmolStr;
use std::sync::Arc;

str_bridge!(Arc<str> {
  from_str: |val: &str| Ok(Arc::<str>::from(val));
  to_str: AsRef::as_ref;

  type EncodedOwned = SmolStr {
    from_ref: |s: &SmolStr| Ok(Arc::<str>::from(s.as_str()));
    from: |s: SmolStr| Ok(Arc::from(s.as_str()));
  }
},);
