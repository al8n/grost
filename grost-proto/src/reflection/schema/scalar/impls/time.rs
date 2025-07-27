impl_reflectable_with_variant!(
  core::time::Duration:Duration,
);

#[cfg(feature = "time_0_3")]
const _: () = {
  use time_0_3::*;

  impl_reflectable_with_variant!(
    Duration:Duration,
    Date:Date,
    UtcDateTime:Utc,
    PrimitiveDateTime:DateTime,
    Time:Time,
  );
};

#[cfg(feature = "chrono_0_4")]
const _: () = {
  use chrono_0_4::*;

  impl_reflectable_with_variant!(
    Duration:Duration,
    DateTime<Utc>:Utc,
    NaiveDate:Date,
    NaiveDateTime:DateTime,
    NaiveTime:Time,
  );
};

#[cfg(feature = "chrono-tz_0_10")]
const _: () = {
  use chrono_tz_0_10::Tz;

  impl_reflectable_with_variant!(
    Tz:Timezone,
  );
};
