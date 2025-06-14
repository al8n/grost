use grost_mir::utils::Output;

#[grost_mir::field(attribute = "grost")]
pub struct Field;

#[grost_mir::object(attribute = "grost", field = "Field")]
#[derive(Clone, Debug)]
pub struct Object {
  #[grost(darling(default))]
  pub output: Option<Output>,
}
