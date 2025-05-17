use chrono_0_4::{DateTime, Duration, NaiveDate, NaiveDateTime, NaiveTime, Utc};

use crate::{flavors::Network, network_varint, reflection::Type, schema_type_reflection};

network_varint!(Duration, NaiveDate, NaiveDateTime, NaiveTime, DateTime<Utc>,);

schema_type_reflection! {
  Network:
    Duration => Type::duration(),
    NaiveDate => Type::scalar("NaiveDate", "ISO 8601 calendar date without timezone. Allows for every [proleptic Gregorian date] from Jan 1, 262145 BCE to Dec 31, 262143 CE. Also supports the conversion from ISO 8601 ordinal and week date."),
    NaiveDateTime => Type::scalar("NaiveDateTime", "ISO 8601 combined date and time without timezone."),
    NaiveTime => Type::scalar("NaiveTime", "ISO 8601 time without timezone. Allows for the nanosecond precision and optional leap second representation."),
    DateTime<Utc> => Type::utc(),
}
