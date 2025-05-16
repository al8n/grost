#[cfg(feature = "smol_str_0_3")]
const _: () = {
  use crate::flavors::network::Network;
  use smol_str_0_3::SmolStr;
  use std::sync::Arc;

  use crate::{into_target, type_owned, type_ref};

  into_target!(Network: SmolStr => Arc<str> {
    |val: SmolStr| Ok(Arc::from(val.as_ref()))
  });
  into_target!(Network: &str => Arc<str> {
    |val: &str| Ok(Arc::from(val))
  });
  type_ref!(Network: &str => Arc<str> {
    |val: &str| Ok(Arc::from(val))
  });
  type_owned!(Network: SmolStr => Arc<str> {
    |val: &SmolStr| Ok(Arc::from(val.as_ref()))
  });

  str_message!(Arc<str> => SmolStr);
};
