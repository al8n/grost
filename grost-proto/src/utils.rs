#[cfg(not(feature = "simdutf8"))]
pub use ::core::str::from_utf8;
#[cfg(feature = "simdutf8")]
pub use simdutf8::basic::from_utf8;