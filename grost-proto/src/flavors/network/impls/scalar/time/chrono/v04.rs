use chrono_0_4::{DateTime, Duration, NaiveDate, NaiveDateTime, NaiveTime, Utc};

crate::network_varint!(Duration, NaiveDate, NaiveDateTime, NaiveTime, DateTime<Utc>,);
