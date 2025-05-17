use crate::{flavors::Network, reflection::Type, schema_type_reflection};

crate::network_varint!(core::time::Duration);

schema_type_reflection! {
  Network:
    core::time::Duration => Type::duration(),
}
