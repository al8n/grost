use core::marker::PhantomData;

use grost::Object;


#[derive(Object)]
struct Generic<I, M> {
  #[grost(tag = 1, generic(marker = "M"))]
  id: I,
  #[grost(skip)]
  _m: PhantomData<M>,
}


#[test]
fn compile() {}
