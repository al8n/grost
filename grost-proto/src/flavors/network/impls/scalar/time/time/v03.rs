use time_0_3::{Date, Duration, PrimitiveDateTime, UtcDateTime};

use crate::{flavors::Network, network_varint, reflection::Type, type_reflection};

network_varint!(Date, Duration, UtcDateTime, PrimitiveDateTime,);

type_reflection! {
  Network:
    Date => Type::scalar("Date", "Date in the proleptic Gregorian calendar."),
    Duration => Type::duration(),
    UtcDateTime => Type::utc(),
    PrimitiveDateTime => Type::scalar("PrimitiveDateTime", "Combined date and time"),
}
