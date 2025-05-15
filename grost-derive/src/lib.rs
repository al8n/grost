use grost_codegen::parser::Object;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Object, attributes(grost))]
pub fn object(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input_struct = parse_macro_input!(input as DeriveInput);

  let object = match Object::from_derive_input(&input_struct) {
    Ok(input) => input,
    Err(e) => return e.write_errors().into(),
  };

  let definations = object.derive_defination();
  quote!(
    #definations
  )
  .into()
}
