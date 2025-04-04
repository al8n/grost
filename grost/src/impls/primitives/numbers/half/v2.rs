use half_2::f16;

fixed!(
  16(f16 {
    from_bytes: |src: &[u8]| {
      Ok(f16::from_le_bytes(src.try_into().unwrap()))
    },
    to_bytes: |this: &Self, buf: &mut [u8]| {
      buf.copy_from_slice(&this.to_le_bytes());
      Ok(())
    },
  }),
);
