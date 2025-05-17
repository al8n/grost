use grost_codegen::{FlavorGenerator, Object, SchemaGeneratorBuilder, network::Network};
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Object, attributes(grost))]
pub fn object(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input_struct = parse_macro_input!(input as DeriveInput);

  let object = match Object::from_derive_input(&input_struct) {
    Ok(input) => input,
    Err(e) => return e.write_errors().into(),
  };

  let network = Network::new(object.path());
  let mut builder = SchemaGeneratorBuilder::new();
  let Ok(_) = builder.add_flavor(network.name(), network) else {
    panic!("failed to add flavor");
  };
  builder.objects_mut().insert(object);
  let generator = builder.build();
  let codegen = match generator.derive() {
    Ok(codegen) => codegen,
    Err(e) => return e.into_compile_error().into(),
  };
  quote!(
    #codegen
  )
  .into()
}
