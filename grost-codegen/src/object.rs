use grost_mir::utils::Output;

#[derive(grost_mir::Field)]
#[grost(attributes(grost))]
pub struct Field;

#[grost_mir::object(attribute = "grost", field = "FieldDeriveInput")]
#[derive(Clone, Debug)]
pub struct Object {
  pub output: Option<Output>,
}
