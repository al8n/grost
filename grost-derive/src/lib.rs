use darling::FromDeriveInput;
use grost_codegen::{ObjectAttributeArgs, parser::ObjectDeriveInput};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

type Result<T = TokenStream, E = syn::Error> = core::result::Result<T, E>;

#[proc_macro_derive(Object, attributes(grost))]
pub fn object(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input_struct = parse_macro_input!(input as DeriveInput);

  let input = match ObjectDeriveInput::from_derive_input(&input_struct) {
    Ok(input) => input,
    Err(e) => return e.write_errors().into(),
  };

  quote!().into()
}

fn to_compile_error(res: Result) -> TokenStream {
  match res {
    Ok(t) => t,
    Err(e) => e.to_compile_error(),
  }
}
