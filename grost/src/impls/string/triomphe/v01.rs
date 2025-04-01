use triomphe_0_1::Arc;

use crate::smol_str::SmolStr;

str_bridge!(Arc<str> {
  from_str: Arc::<str>::from,
  to_str: AsRef::as_ref,
},);

str_bridge!(
  @smolstr_message
  Arc::<str> {
    from_ref: |s: &SmolStr| {
      Arc::<str>::from(s.as_str())
    },
    from: |s: SmolStr| {
      Arc::from(s.as_str())
    },
  },
);
