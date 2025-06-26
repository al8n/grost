use chrono_0_4::{DateTime, Duration, NaiveDate, NaiveDateTime, NaiveTime, Utc};

crate::groto_varint!(Duration, NaiveDate, NaiveDateTime, NaiveTime, DateTime<Utc>,);
