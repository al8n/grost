use crate::{
  flavors::{
    Borrowed, Flatten, Groto, Packed, WireFormat,
    groto::{Context, Error},
  },
  reflection::{Reflectable, SchemaType, SchemaTypeReflection},
};

mod join;
mod packed;
mod repeated;

fn check_list_type<T>() -> Result<(), Error>
where
  T: ?Sized,
  SchemaTypeReflection<T>: Reflectable<T, Reflection = SchemaType>,
{
  if !SchemaTypeReflection::<T>::REFLECTION.is_list() {
    return Err(Error::custom(
      "cannot encode a non-nested list type as flatten format",
    ));
  }

  Ok(())
}

#[cfg(test)]
mod tests {
  use crate::{
    encode::{Encode, EquivalentEncode},
    flavors::groto::Varint,
  };

  use super::*;

  #[test]
  fn t() {
    fn equivalent_encoded_len<T, W>(input: &T, ctx: &Context) -> usize
    where
      W: WireFormat<Groto>,
      T: Encode<W, Groto> + ?Sized,
      [u16]: EquivalentEncode<T, W, Groto, WireFormat = Packed<Varint>, Flavor = Groto>,
    {
      input.encoded_len(ctx)
    }

    let a: &[&[u16]] = &[&[1u16, 2, 3], &[4, 5, 6]];
    let flatten_a: &[u16] = &[1u16, 2, 3, 4, 5, 6];

    let context = Context::default();
    let encoded_len =
      equivalent_encoded_len::<_, Flatten<Borrowed<'_, Packed<Varint>>, Varint>>(a, &context);
    let flatten_encoded_len = equivalent_encoded_len(flatten_a, &context);
    assert_eq!(encoded_len, flatten_encoded_len);

    let mut buf = [0u8; 100];
    let mut flatten_buf = [0u8; 100];

    let encoded_len =
      <[&[u16]] as Encode<Flatten<Borrowed<'_, Packed<Varint>>, Varint>, Groto>>::encode(
        a, &context, &mut buf,
      )
      .unwrap();
    let flatten_encoded_len =
      <[u16] as Encode<Packed<Varint>, Groto>>::encode(flatten_a, &context, &mut flatten_buf)
        .unwrap();
    assert_eq!(encoded_len, flatten_encoded_len);
    assert_eq!(&buf[..encoded_len], &flatten_buf[..flatten_encoded_len]);
  }
}
