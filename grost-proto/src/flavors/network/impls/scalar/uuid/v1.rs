use uuid_1::Uuid;

use crate::{
  bridge, default_wire_format,
  flavors::network::{Fixed128, Network, Varint},
  referenceable_scalar, selectable_bridge,
};

selectable_bridge!(Network:u128[Uuid]);
referenceable_scalar!(Network: Uuid as Fixed128, Uuid as Varint);

bridge!(
  Network: u128 {
    Uuid as Fixed128 {
      from: Uuid::from_u128;
      to: Uuid::as_u128;
    },
    Uuid as Varint {
      from: Uuid::from_u128;
      to: Uuid::as_u128;
    },
  },
);

default_wire_format!(Network: Uuid as Fixed128);
