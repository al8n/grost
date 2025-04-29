#[cfg(feature = "smol_str_0_3")]
const _: () = {
  use crate::flavors::network::Network;
  use smol_str_0_3::SmolStr;
  use std::sync::Arc;

  use crate::{into_target, type_owned, type_ref};

  str_bridge!(Network: Arc<str> {
    from_str: |val: &str| Arc::<str>::from(val);
    as_str: AsRef::as_ref;
  
    type EncodedOwned = SmolStr;
  },);

  into_target!(Network: SmolStr => Arc<str> {
    |val: SmolStr| Ok(Arc::from(val.as_ref()))
  });
  into_target!(Network: &str => Arc<str> {
    |val: &str| Ok(Arc::from(val))
  });
  into_target!(@self Network: Arc<str>);
  type_ref!(@mapping Network: &str => Arc<str> {
    |val: &str| Ok(Arc::from(val))
  });
  type_owned!(@mapping Network: SmolStr => Arc<str> {
    |val: &SmolStr| Ok(Arc::from(val.as_ref()))
  });
  type_owned!(@clone Network: Arc<str>);
};
