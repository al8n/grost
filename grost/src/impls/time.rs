// use crate::error::DecodeError;

mod chrono;
mod chrono_tz;
mod duration;
#[allow(clippy::module_inception)]
mod time;

/// Months of the year
#[derive(
  Debug,
  Clone,
  Copy,
  PartialEq,
  Eq,
  PartialOrd,
  Ord,
  Hash,
  derive_more::IsVariant,
  derive_more::Display,
)]
#[repr(u8)]
pub enum Month {
  /// January
  January = 1,
  /// February
  February = 2,
  /// March
  March = 3,
  /// April
  April = 4,
  /// May
  May = 5,
  /// June
  June = 6,
  /// July
  July = 7,
  /// August
  August = 8,
  /// September
  September = 9,
  /// October
  October = 10,
  /// November
  November = 11,
  /// December
  December = 12,
}

impl TryFrom<u8> for Month {
  type Error = DecodeError;

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    Ok(match value {
      1 => Self::January,
      2 => Self::February,
      3 => Self::March,
      4 => Self::April,
      5 => Self::May,
      6 => Self::June,
      7 => Self::July,
      8 => Self::August,
      9 => Self::September,
      10 => Self::October,
      11 => Self::November,
      12 => Self::December,
      _ => return Err(DecodeError::custom("unknown month value")),
    })
  }
}

/// Days of the week
#[derive(
  Debug,
  Clone,
  Copy,
  PartialEq,
  Eq,
  PartialOrd,
  Ord,
  Hash,
  derive_more::IsVariant,
  derive_more::Display,
)]
#[repr(u8)]
pub enum Weekday {
  /// Monday
  Monday = 1,
  /// Tuesday
  Tuesday = 2,
  /// Wednesday
  Wednesday = 3,
  /// Thursday
  Thursday = 4,
  /// Friday
  Friday = 5,
  /// Saturday
  Saturday = 6,
  /// Sunday
  Sunday = 7,
}

impl TryFrom<u8> for Weekday {
  type Error = DecodeError;

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    Ok(match value {
      1 => Self::Monday,
      2 => Self::Tuesday,
      3 => Self::Wednesday,
      4 => Self::Thursday,
      5 => Self::Friday,
      6 => Self::Saturday,
      7 => Self::Sunday,
      _ => return Err(DecodeError::custom("unknown weekday value")),
    })
  }
}

try_from_bridge! {
  u8 {
    Month {
      try_from: |v: u8| Month::try_from(v);
      to: |v: &Self| *v as u8;
    },
    Weekday {
      try_from: |v: u8| Weekday::try_from(v);
      to: |v: &Self| *v as u8;
    },
  },
}
