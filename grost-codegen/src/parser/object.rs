// pub use sealed::ObjectDeriveInput;
use darling::FromDeriveInput;
use syn::DeriveInput;

#[allow(unused)]
mod sealed {
  use super::super::default_path;

  #[derive(grost_darling::ObjectField)]
  #[grost(attributes(grost))]
  pub struct ObjectField;

  #[derive(grost_darling::Object)]
  #[grost(attributes(grost), field = "ObjectFieldDeriveInput")]
  pub struct Object;
}

pub struct Object {
  object: grost_darling::mir::object::Object<sealed::ObjectDeriveInput>,
}

impl Object {
  pub fn from_derive_input(input: DeriveInput) -> darling::Result<Self> {
    let input = sealed::ObjectDeriveInput::from_derive_input(&input)?;

    let object = grost_darling::mir::object::Object::from_meta(
      input,
    )?;

    Ok(Self {
      object,
    })
  }

  pub fn derive_defination(&self) -> proc_macro2::TokenStream {
    self.object.derive_defination()
  }
}
