use uuid_1::Uuid;

use crate::{
  bridge, decoded_state, default_wire_format, flatten_state,
  flavors::network::{Fixed128, Network, Varint},
  reflection::Type,
  selectable, type_reflection,
};

selectable!(@scalar Network:Uuid);
decoded_state!(@scalar &'a Network: Uuid as Fixed128, Uuid as Varint);
flatten_state!(Uuid);
type_reflection! {
  Network:
    Uuid => Type::scalar("Uuid", "Universally Unique Identifier (UUID)."),
}

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
