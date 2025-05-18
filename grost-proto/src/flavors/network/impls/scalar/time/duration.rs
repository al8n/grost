use crate::{flavors::Network, reflection::Type, type_reflection};

crate::network_varint!(core::time::Duration);

type_reflection! {
  Network:
    core::time::Duration => Type::duration(),
}
