use rust_decimal_1::Decimal;

fixed!(128(Decimal {
  from_bytes: |src: &[u8]| { Ok((16, Decimal::deserialize(src.try_into().unwrap()))) },
  to_bytes: |this: &Self, buf: &mut [u8]| {
    buf.copy_from_slice(&this.serialize());
    Ok(16)
  },
}),);
