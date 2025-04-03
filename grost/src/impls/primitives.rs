zst!((), ::core::marker::PhantomPinned);
phantom!(::core::marker::PhantomData<T>);

mod array;
mod bytes;
mod numbers;
#[cfg(feature = "std")]
mod path;
mod slice;
mod str;
mod string;
mod uuid;
