#[cfg(all(not(feature = "std"), feature = "alloc", feature = "hashbrown_0_15"))]
use hashbrown_0_15::HashMap;

#[cfg(feature = "std")]
use std::collections::HashMap;

use crate::convert::{Flattened, Inner, State};

impl<K, V, S> State<Flattened<Inner>> for HashMap<K, V, S> {
  type Output = (K, V);
}
