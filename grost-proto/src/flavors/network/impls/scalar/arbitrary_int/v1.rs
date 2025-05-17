use crate::{flavors::Network, network_varint, reflection::Type, schema_type_reflection};
pub use arbitrary_int_1::*;

macro_rules! impl_arbitrary_int {
  ($($start:literal..=$end:literal), +$(,)?) => {
    $(
      seq_macro::seq!(N in $start..=$end {
        network_varint!(#(u~N,)*);

        schema_type_reflection! {
          Network:
            #(u~N => Type::scalar(stringify!(u~N), concat!(stringify!(N), "-bit unsigned integer")),)*
        }
      });
    )*
  };
}

impl_arbitrary_int!(1..=7, 9..=15, 17..=31, 33..=63, 65..=127,);
