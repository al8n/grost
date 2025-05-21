use grost_mir::ast::Output;
use quote::ToTokens;

#[allow(unused)]
mod sealed {
  #[derive(grost_mir::Field)]
  #[grost(attributes(grost))]
  pub struct Field;

  #[derive(grost_mir::Object)]
  #[grost(attribute = "grost", field = "FieldDeriveInput")]
  pub struct Object;
}

pub struct Object {
  object: grost_mir::object::Object<sealed::ObjectInput>,
}

impl ToTokens for Object {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    self.object.to_tokens(tokens);
  }
}

impl core::ops::Deref for Object {
  type Target = grost_mir::object::Object<sealed::ObjectInput>;

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
  pub fn from_derive_input(input: proc_macro2::TokenStream) -> darling::Result<Self> {
    let input = sealed::ObjectInput::from_derive_input(input)?;
    let object = grost_mir::object::Object::<sealed::ObjectInput>::from_object(input)?;

    Ok(Self { object })
  }

  pub fn from_attribute_input(args: proc_macro2::TokenStream, input: proc_macro2::TokenStream) -> darling::Result<Self> {
    let input = sealed::ObjectInput::from_attribute_input(args, input)?;
    let object = grost_mir::object::Object::<sealed::ObjectInput>::from_object(input)?;

    Ok(Self { object })
  }

  pub fn derive(&self) -> syn::Result<proc_macro2::TokenStream> {
    self.object.derive()
      .map(|t| {
        if self.object.meta().derived() {
          t
        } else {
          let obj = &self.object;
          quote::quote! {
            #obj

            #t
          }
        }
      })
  }

  pub(super) fn output(&self) -> Option<&Output> {
    self.object.meta().output()
  }
}
