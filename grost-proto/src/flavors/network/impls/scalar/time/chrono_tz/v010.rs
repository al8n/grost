use crate::{flavors::Network, reflection::Type, schema_type_reflection};

crate::network_varint!(chrono_tz_0_10::Tz,);

schema_type_reflection! {
  Network:
    chrono_tz_0_10::Tz => Type::scalar("Tz", "Time zone information"),
}
