pub use arbitrary_int_1::*;

use crate::{Deserialize, DeserializeOwned, OutputType, Serialize, TypeOwned, TypeRef};

macro_rules! impl_for_arbitrary_int {
  ($($ty:ident),+$(,)?) => {
    wirable!(@outer (varint) <=> ($($ty,)*));

    $(
      impl Serialize for $ty {
        impl_varing_types!(@serialize);
      }

      impl<'de> Deserialize<'de> for $ty {
        impl_varing_types!(@deserialize);
      }

      impl DeserializeOwned for $ty {
        impl_varing_types!(@deserialize_owned);
      }

      impl OutputType for $ty {
        impl_output_type_for_self!();
      }

      impl TypeRef<Self> for $ty {
        impl_varing_types!(@type_ref);
      }

      impl TypeOwned<Self> for $ty {
        impl_varing_types!(@type_owned);
      }
    )*
  };
}

seq_macro::seq!(N in 1..=7 {
  impl_for_arbitrary_int!(#(u~N,)*);
});

seq_macro::seq!(N in 9..=15 {
  impl_for_arbitrary_int!(#(u~N,)*);
});

seq_macro::seq!(N in 17..=31 {
  impl_for_arbitrary_int!(#(u~N,)*);
});

seq_macro::seq!(N in 33..=63 {
  impl_for_arbitrary_int!(#(u~N,)*);
});

seq_macro::seq!(N in 65..=127 {
  impl_for_arbitrary_int!(#(u~N,)*);
});
