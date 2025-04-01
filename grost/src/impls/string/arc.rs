use std::sync::Arc;

use crate::smol_str::SmolStr;

str_bridge!(Arc<str> {
  from_str: Arc::<str>::from,
  to_str: AsRef::as_ref,
},);

str_bridge!(
  @smolstr_message
  Arc::<str> {
    from_ref: |s: &SmolStr| {
      s.clone().into()
    },
    from: Into::into,
  },
);
