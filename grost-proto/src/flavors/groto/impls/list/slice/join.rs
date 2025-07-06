use crate::{
  encode::{Encode, PartialEncode},
  flavors::{
    Groto, JoinAscii, JoinChar, WireFormat,
    groto::{Context, Error, LengthDelimited},
  },
  selection::Selectable,
};

macro_rules! blanket_partial_encode_impl {
  ($wf:ty) => {
    fn partial_encode_raw(
      &self,
      context: &Context,
      buf: &mut [u8],
      selector: &Self::Selector,
    ) -> Result<usize, Error> {
      if !*selector {
        return Ok(0);
      }

      <Self as Encode<$wf, Groto>>::encode_raw(self, context, buf)
    }

    fn partial_encoded_raw_len(&self, context: &Context, selector: &Self::Selector) -> usize {
      if *selector {
        <Self as Encode<$wf, Groto>>::encoded_raw_len(self, context)
      } else {
        0
      }
    }

    fn partial_encode(
      &self,
      context: &Context,
      buf: &mut [u8],
      selector: &Self::Selector,
    ) -> Result<usize, Error> {
      if *selector {
        <Self as Encode<$wf, Groto>>::encode(self, context, buf)
      } else {
        Ok(0)
      }
    }

    fn partial_encoded_len(&self, context: &Context, selector: &Self::Selector) -> usize {
      if *selector {
        <Self as Encode<$wf, Groto>>::encoded_len(self, context)
      } else {
        0
      }
    }
  };
}

fn encode_raw<N, F>(
  input: &[N],
  buf: &mut [u8],
  fill_bytes: &[u8],
  get_len: F,
) -> Result<usize, Error>
where
  N: AsRef<[u8]>,
  F: Fn() -> usize,
{
  let buf_len = buf.len();
  let data_len = get_len();

  let mut offset = 0;
  for value in input.iter() {
    if offset >= buf_len {
      return Err(Error::insufficient_buffer(data_len, buf_len));
    }

    if !fill_bytes.is_empty() {
      let len = fill_bytes.len();
      if offset + len >= buf_len {
        return Err(Error::insufficient_buffer(data_len, buf_len));
      }
      buf[offset..offset + len].copy_from_slice(fill_bytes);
      offset += len;
    }

    let bytes = value.as_ref();
    let bytes_len = bytes.len();
    if offset + bytes_len >= buf_len {
      return Err(Error::insufficient_buffer(data_len, buf_len));
    }
    buf[offset..offset + bytes_len].copy_from_slice(bytes);
    offset += bytes_len;
  }

  Ok(offset)
}

fn encoded_raw_len<N>(input: &[N], fill_bytes: &[u8]) -> usize
where
  N: AsRef<[u8]>,
{
  let len = input.len();
  let total = input.iter().map(|n| n.as_ref().len()).sum::<usize>();

  let fill_bytes_len = fill_bytes.len();

  if fill_bytes_len == 0 {
    return total;
  }

  (len * fill_bytes_len) + total
}

fn encode<N, F>(
  input: &[N],
  buf: &mut [u8],
  fill_bytes: &[u8],
  get_data_len: F,
) -> Result<usize, Error>
where
  N: AsRef<[u8]>,
  F: Fn() -> usize,
{
  let buf_len = buf.len();
  let data_len = get_data_len();
  let this_len = varing::encoded_u32_varint_len(data_len as u32) + data_len;
  if buf_len < this_len {
    return Err(Error::insufficient_buffer(this_len, buf_len));
  }

  let mut offset = varing::encode_u32_varint_to(data_len as u32, buf)
    .map_err(|e| Error::from_varint_encode_error(e).update(this_len, buf_len))?;

  for value in input.iter() {
    if offset >= buf_len {
      return Err(Error::insufficient_buffer(this_len, buf_len));
    }

    if !fill_bytes.is_empty() {
      let len = fill_bytes.len();
      if offset + len >= buf_len {
        return Err(Error::insufficient_buffer(data_len, buf_len));
      }
      buf[offset..offset + len].copy_from_slice(fill_bytes);
      offset += len;
    }

    let bytes = value.as_ref();
    let bytes_len = bytes.len();
    if offset + bytes_len >= buf_len {
      return Err(Error::insufficient_buffer(data_len, buf_len));
    }
    buf[offset..offset + bytes_len].copy_from_slice(bytes);
    offset += bytes_len;
  }

  Ok(offset)
}

fn encoded_len<N>(input: &[N], fill_bytes: &[u8]) -> usize
where
  N: AsRef<[u8]>,
{
  let data_len = encoded_raw_len(input, fill_bytes);
  varing::encoded_u32_varint_len(data_len as u32) + data_len
}

seq_macro::seq!(IDX in 0..=63 {
  impl<N, #(const A~IDX: u8,)*> Encode<JoinAscii<LengthDelimited, #(A~IDX,)*>, Groto> for [N]
  where
    N: AsRef<[u8]>,
    JoinAscii<LengthDelimited, #(A~IDX,)*>: WireFormat<Groto>,
  {
    fn encode_raw(
      &self,
      ctx: &Context,
      buf: &mut [u8],
    ) -> Result<usize, Error> {
      encode_raw(self, buf, JoinAscii::<LengthDelimited, #(A~IDX,)*>::BYTES, || <Self as Encode<JoinAscii<LengthDelimited, #(A~IDX,)*>, Groto>>::encoded_raw_len(self, ctx))
    }

    fn encoded_raw_len(&self, _: &Context) -> usize {
      encoded_raw_len(self, JoinAscii::<LengthDelimited, #(A~IDX,)*>::BYTES)
    }

    fn encode(
      &self,
      context: &Context,
      buf: &mut [u8],
    ) -> Result<usize, Error> {
      encode(
        self,
        buf,
        JoinAscii::<LengthDelimited, #(A~IDX,)*>::BYTES,
        || <Self as Encode<JoinAscii<LengthDelimited, #(A~IDX,)*>, Groto>>::encoded_raw_len(self, context),
      )
    }

    fn encoded_len(&self, _: &Context) -> usize {
      encoded_len(self, JoinAscii::<LengthDelimited, #(A~IDX,)*>::BYTES)
    }
  }

  impl<N, #(const A~IDX: u8,)*> PartialEncode<JoinAscii<LengthDelimited, #(A~IDX,)*>, Groto> for [N]
  where
    N: AsRef<[u8]> + Selectable<Groto, Selector = bool>,
    JoinAscii<LengthDelimited, #(A~IDX,)*>: WireFormat<Groto>,
  {
    blanket_partial_encode_impl!(JoinAscii<LengthDelimited, #(A~IDX,)*>);
  }
});

seq_macro::seq!(IDX in 0..=63 {
  impl<N, #(const A~IDX: char,)*> Encode<JoinChar<LengthDelimited, #(A~IDX,)*>, Groto> for [N]
  where
    N: AsRef<[u8]>,
    JoinChar<LengthDelimited, #(A~IDX,)*>: WireFormat<Groto>,
  {
    fn encode_raw(
      &self,
      ctx: &Context,
      buf: &mut [u8],
    ) -> Result<usize, Error> {
      encode_raw(self, buf, JoinChar::<LengthDelimited, #(A~IDX,)*>::BYTES, || <Self as Encode<JoinChar<LengthDelimited, #(A~IDX,)*>, Groto>>::encoded_raw_len(self, ctx))
    }

    fn encoded_raw_len(&self, _: &Context) -> usize {
      encoded_raw_len(self, JoinChar::<LengthDelimited, #(A~IDX,)*>::BYTES)
    }

    fn encode(
      &self,
      context: &Context,
      buf: &mut [u8],
    ) -> Result<usize, Error> {
      encode(
        self,
        buf,
        JoinChar::<LengthDelimited, #(A~IDX,)*>::BYTES,
        || <Self as Encode<JoinChar<LengthDelimited, #(A~IDX,)*>, Groto>>::encoded_raw_len(self, context),
      )
    }

    fn encoded_len(&self, _: &Context) -> usize {
      encoded_len(self, JoinChar::<LengthDelimited, #(A~IDX,)*>::BYTES)
    }
  }

  impl<N, #(const A~IDX: char,)*> PartialEncode<JoinChar<LengthDelimited, #(A~IDX,)*>, Groto> for [N]
  where
    N: AsRef<[u8]> + Selectable<Groto, Selector = bool>,
    JoinChar<LengthDelimited, #(A~IDX,)*>: WireFormat<Groto>,
  {
    blanket_partial_encode_impl!(JoinChar<LengthDelimited, #(A~IDX,)*>);
  }
});
