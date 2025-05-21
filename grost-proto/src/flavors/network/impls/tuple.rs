use crate::{
  encode::Encode,
  flavors::{
    Network,
    network::{Fixed8, Fixed16, Fixed32, Fixed64, Fixed128},
  },
};

macro_rules! tuple2_impl {
  ($($wf:ty:$merged_wf:ty),+$(,)?) => {
    $(
      impl<K, V> Encode<Network, $merged_wf> for (K, V)
      where
        K: Encode<Network, $wf>,
        V: Encode<Network, $wf>,
      {
        fn encode(&self, context: &<Network as crate::flavors::Flavor>::Context, buf: &mut [u8]) -> Result<usize, <Network as crate::flavors::Flavor>::Error> {
          let buf_len = buf.len();
          if buf_len < <$merged_wf>::SIZE {
            return Err(<Network as crate::flavors::Flavor>::Error::insufficient_buffer(<$merged_wf>::SIZE, buf_len));
          }
          let mut offset = 0;
          let encoded_len = <K as Encode<Network, $wf>>::encode(&self.0, context, buf)
            .map_err(|e| e.update(<Self as Encode<Network, $merged_wf>>::encoded_len(self, context), buf_len))?;

          #[cfg(debug_assertions)]
          {
            assert_eq!(encoded_len, <$wf>::SIZE, "encoded length of .0 is not {}", <$wf>::SIZE);
          }

          offset += encoded_len;

          let encoded_len = <V as Encode<Network, $wf>>::encode(&self.1, context, &mut buf[offset..])
            .map_err(|e| e.update(<Self as Encode<Network, $merged_wf>>::encoded_len(self, context), buf_len))?;
          #[cfg(debug_assertions)]
          {
            assert_eq!(encoded_len, <$wf>::SIZE, "encoded length of .1 is not {}", <$wf>::SIZE);
          }

          offset += encoded_len;

          #[cfg(debug_assertions)]
          {
            crate::debug_assert_write_eq::<Self>(offset, <$merged_wf>::SIZE);
          }
          Ok(offset)
        }

        fn encoded_len(&self, _: &<Network as crate::flavors::Flavor>::Context) -> usize {
          <$merged_wf>::SIZE
        }

        fn encoded_length_delimited_len(&self, context: &<Network as crate::flavors::Flavor>::Context) -> usize {
          <Self as Encode<Network, $merged_wf>>::encoded_len(self, context)
        }

        fn encode_length_delimited(
          &self,
          context: &<Network as crate::flavors::Flavor>::Context,
          buf: &mut [u8],
        ) -> Result<usize, <Network as crate::flavors::Flavor>::Error> {
          <Self as Encode<Network, $merged_wf>>::encode(self, context, buf)
        }
      }
    )*
  };
}

tuple2_impl!(
  Fixed8: Fixed16,
  Fixed16: Fixed32,
  Fixed32: Fixed64,
  Fixed64: Fixed128,
);
