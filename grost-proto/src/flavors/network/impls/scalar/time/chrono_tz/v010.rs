use crate::{flavors::Network, reflection::Type, type_reflection};

crate::network_varint!(chrono_tz_0_10::Tz,);

type_reflection! {
  Network:
    chrono_tz_0_10::Tz => Type::scalar("Tz", "Time zone information"),
}
