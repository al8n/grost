use grost_codegen::{Object, ObjectAttributeArgs};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

type Result<T = TokenStream, E = syn::Error> = core::result::Result<T, E>;

#[proc_macro_attribute]
pub fn object(
  args: proc_macro::TokenStream,
  input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
  let mut attrs = ObjectAttributeArgs::default();
  let object_parser = syn::meta::parser(|meta| attrs.parse(meta));
  parse_macro_input!(args with object_parser);
  let input_struct = parse_macro_input!(input as DeriveInput);
  let object = match Object::from_attribute_macro_input(attrs, input_struct) {
    Ok(object) => object,
    Err(e) => return e.into_compile_error().into(),
  };

  quote!().into()
}

fn to_compile_error(res: Result) -> TokenStream {
  match res {
    Ok(t) => t,
    Err(e) => e.to_compile_error(),
  }
}
