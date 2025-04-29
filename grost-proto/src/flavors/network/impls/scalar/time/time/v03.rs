use time_0_3::{Date, Duration, PrimitiveDateTime, UtcDateTime};

use crate::network_varint;

network_varint!(Date, Duration, UtcDateTime, PrimitiveDateTime,);
