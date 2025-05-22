use tinyvec_1::{Array, ArrayVec};

repeated!(@tinyvec Array:ArrayVec<A>);

#[cfg(any(feature = "std", feature = "alloc"))]
repeated!(@tinyvec Array:tinyvec_1::TinyVec<A>);
