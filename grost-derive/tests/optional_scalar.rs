use grost_derive::Object;

#[derive(Debug, Clone, Object)]
struct OptionalScalar {
  #[grost(tag = 1, optional(scalar))]
  foo: Option<u32>,
}
