// use crate::{
//   encode::Encode,
//   flavors::{
//     Groto,
//     groto::{Fixed8, Fixed16, Fixed32, Fixed64, Fixed128},
//   },
// };

// macro_rules! tuple2_impl {
//   ($($wf:ty:$merged_wf:ty),+$(,)?) => {
//     $(
//       impl<K, V> Encode<$merged_wf, Groto> for (K, V)
//       where
//         K: Encode<$wf, Groto>,
//         V: Encode<$wf, Groto>,
//       {
//         fn encode_raw<B>(
//           &self,
//           context: &<Groto as crate::flavors::Flavor>::Context,
//           mut buf: &mut B,
//         ) -> Result<usize, <Groto as crate::flavors::Flavor>::Error>
//         where
//           B: crate::buffer::BufMut + ?Sized,
//         {
//           let buf_len = buf.len();
//           if buf_len < <$merged_wf>::SIZE {
//             return Err(<Groto as crate::flavors::Flavor>::Error::buffer_too_small(<$merged_wf>::SIZE, buf_len));
//           }
//           let mut offset = 0;
//           let encoded_len = <K as Encode<$wf, Groto>>::encode(&self.0, context, buf)
//             .map_err(|e| e.update(<Self as Encode<$merged_wf, Groto>>::encoded_len(self, context), buf_len))?;

//           #[cfg(debug_assertions)]
//           {
//             assert_eq!(encoded_len, <$wf>::SIZE, "encoded length of .0 is not {}", <$wf>::SIZE);
//           }

//           offset += encoded_len;

//           let encoded_len = <V as Encode<$wf, Groto>>::encode(&self.1, context, &mut buf.buffer_mut()[offset..])
//             .map_err(|e| e.update(<Self as Encode<$merged_wf, Groto>>::encoded_len(self, context), buf_len))?;
//           #[cfg(debug_assertions)]
//           {
//             assert_eq!(encoded_len, <$wf>::SIZE, "encoded length of .1 is not {}", <$wf>::SIZE);
//           }

//           offset += encoded_len;

//           #[cfg(debug_assertions)]
//           {
//             crate::debug_assert_write_eq::<Self>(offset, <$merged_wf>::SIZE);
//           }
//           Ok(offset)
//         }

//         fn encoded_raw_len(&self, _: &<Groto as crate::flavors::Flavor>::Context) -> usize {
//           <$merged_wf>::SIZE
//         }

//         fn encode<B>(&self, context: &<Groto as crate::flavors::Flavor>::Context, buf: &mut B) -> Result<usize, <Groto as crate::flavors::Flavor>::Error>
//         where
//           B: crate::buffer::BufMut + ?Sized,
//         {
//           <Self as Encode<$merged_wf, Groto>>::encode_raw(self, context, buf)
//         }

//         fn encoded_len(&self, ctx: &<Groto as crate::flavors::Flavor>::Context) -> usize {
//           <Self as Encode<$merged_wf, Groto>>::encoded_raw_len(self, ctx)
//         }
//       }
//     )*
//   };
// }

// tuple2_impl!(
//   Fixed8: Fixed16,
//   Fixed16: Fixed32,
//   Fixed32: Fixed64,
//   Fixed64: Fixed128,
// );
