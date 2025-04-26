use uuid_1::Uuid;

use crate::{bridge, flavors::network::Network};

bridge!(
  Network: u128 {
    Uuid {
      from: Uuid::from_u128;
      to: Uuid::as_u128;
    },
  },
);
