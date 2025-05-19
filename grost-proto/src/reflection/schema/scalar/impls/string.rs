impl_reflectable_with_variant!(
  str:String,
);

#[cfg(any(feature = "alloc", feature = "std"))]
const _: () = {
  impl_reflectable_with_variant!(
    std::string::String:String,
  );
};

#[cfg(feature = "smol_str_0_3")]
const _: () = {
  impl_reflectable_with_variant!(
    smol_str_0_3::SmolStr:String,
  );
};

#[cfg(feature = "tinystr_0_8")]
const _: () = {
  impl_reflectable_with_variant!(
    tinystr_0_8::TinyAsciiStr<N> [const N: usize]:String,
  );
};

#[cfg(feature = "arrayvec_0_7")]
const _: () = {
  impl_reflectable_with_variant!(
    arrayvec_0_7::ArrayString<N> [const N: usize]:String,
  );
};
