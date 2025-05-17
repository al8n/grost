use quote::quote;
use syn::DeriveInput;

#[allow(unused)]
mod sealed {
  #[derive(grost_mir::ObjectField)]
  #[grost(attributes(grost))]
  pub struct ObjectField;

  #[derive(grost_mir::Object)]
  #[grost(attributes(grost), field = "ObjectFieldDeriveInput")]
  pub struct Object;
}

pub struct Object {
  object: grost_mir::object::Object<sealed::ObjectDeriveInput>,
}

impl core::ops::Deref for Object {
  type Target = grost_mir::object::Object<sealed::ObjectDeriveInput>;

  fn deref(&self) -> &Self::Target {
    &self.object
  }
}

impl PartialEq for Object {
  fn eq(&self, other: &Self) -> bool {
    self.object.name().eq(other.object.name())
  }
}

impl Eq for Object {}

impl core::hash::Hash for Object {
  fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
    self.object.name().hash(state);
  }
}

impl Object {
  pub fn from_derive_input(input: &DeriveInput) -> darling::Result<Self> {
    let object = grost_mir::object::Object::<sealed::ObjectDeriveInput>::from_derive_input(input)?;

    Ok(Self { object })
  }

  pub fn derive(&self) -> proc_macro2::TokenStream {
    let object = &self.object;

    quote! {
      #object
    }
  }
}
