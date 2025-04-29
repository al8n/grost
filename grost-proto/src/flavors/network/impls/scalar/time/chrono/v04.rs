use chrono_0_4::{DateTime, Duration, NaiveDate, NaiveDateTime, NaiveTime, Utc};

use crate::network_varint;

network_varint!(Duration, NaiveDate, NaiveDateTime, NaiveTime, DateTime<Utc>,);
