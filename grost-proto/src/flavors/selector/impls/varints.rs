use super::super::{SelectorFlavor, Varint};
use crate::varint;

varint!(SelectorFlavor:Varint {
  u8,
  u16,
  u32,
  u64,
  u128,
  i8,
  i16,
  i32,
  i64,
  i128,
});
