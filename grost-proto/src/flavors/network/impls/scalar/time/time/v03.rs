use time_0_3::{Date, Duration, PrimitiveDateTime, Time, UtcDateTime};

crate::network_varint!(Date, Duration, UtcDateTime, PrimitiveDateTime, Time);
