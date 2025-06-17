use uuid_1::Uuid;

use crate::{
  bridge, decoded_state, default_wire_format, flatten_state,
  flavors::network::{Fixed128, Network, Varint},
  identity_partial_transform, identity_transform, selectable,
};

selectable!(@scalar Network:Uuid);
decoded_state!(@scalar &'a Network: Uuid as Fixed128, Uuid as Varint);
flatten_state!(Uuid);

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
identity_transform!(
  Network {
    Uuid as Fixed128,
    Uuid as Varint,
  }
);
identity_partial_transform!(
  Network {
    Uuid as Fixed128,
    Uuid as Varint,
  }
);
