use uuid_1::Uuid;

use crate::{
  bridge, default_wire_format, encoded_state,
  flavors::network::{Fixed128, Network, Varint},
  referenceable_scalar, selectable,
};

selectable!(@scalar Network:Uuid);
referenceable_scalar!(Network: Uuid as Fixed128, Uuid as Varint);
encoded_state!(@scalar &'a Network: Uuid as Fixed128, Uuid as Varint);

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
