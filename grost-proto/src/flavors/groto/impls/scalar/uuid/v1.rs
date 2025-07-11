use uuid_1::Uuid;

use crate::{
  bridge, default_scalar_wire_format, flatten_state,
  flavors::groto::{Fixed128, Groto, Varint},
  groto_identity_transform, partial_ref_state, partial_state, ref_state, selectable,
};

selectable!(@scalar Groto:Uuid);
ref_state!(@scalar &'a Groto:
  Uuid as Fixed128,
  Uuid as Varint,
);
partial_ref_state!(@scalar &'a Groto:
  Uuid as Fixed128,
  Uuid as Varint,
);
partial_state!(@scalar Groto: Uuid);
flatten_state!(Uuid);

bridge!(
  Groto: u128 {
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

default_scalar_wire_format!(Groto: Uuid as Fixed128);
groto_identity_transform!(Uuid as Fixed128, Uuid as Varint,);
identity_partial_transform!(
  Groto {
    Uuid as Fixed128,
    Uuid as Varint,
  }
);
