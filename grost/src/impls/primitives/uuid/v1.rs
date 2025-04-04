use uuid_1::Uuid;

fixed!(128(Uuid {
  from_bytes: |src: &[u8]| {
    let buf: [u8; 16] = src.try_into().unwrap();
    Ok(Uuid::from_bytes(buf))
  },
  to_bytes: |this: &Self, buf: &mut [u8]| {
    buf.copy_from_slice(this.as_bytes());
    Ok(())
  },
}),);
