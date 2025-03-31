pub use arbitrary_int_1::*;

macro_rules! impl_arbitrary_int {
  ($($start:literal..=$end:literal), +$(,)?) => {
    $(
      seq_macro::seq!(N in $start..=$end {
        varing!(#(u~N,)*);
      });
    )*
  };
}

impl_arbitrary_int!(
  1..=7,
  9..=15,
  17..=31,
  33..=63,
  65..=127,
);
