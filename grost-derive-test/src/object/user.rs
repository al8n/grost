use grost::Object;


#[derive(Object)]
#[grost(
  ref(
    or_default,
    or_default_scalar,
  ),
  partial_ref(
    or_default,
    or_default_scalar,
  ),
  partial(
    or_default,
    or_default_scalar,
  ),
  or_default,
  or_default_bytes,
)]
struct User {
  #[grost(
    tag = 1,
    string,
    or_default,
    partial_ref(
      decode(
        or_else_default,
      ),
    ),
    partial(
      decode(
        or_else_default,
      ),
    ),
  )]
  name: String,
  #[grost(tag = 2, scalar)]
  age: u8,
}

#[test]
fn compile() {
  // This test ensures that the User struct compiles correctly with the grost derive macro.
  // It does not perform any runtime checks or assertions.
}
