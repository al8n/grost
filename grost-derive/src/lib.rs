use grost_codegen::{FlavorGenerator, Object, SchemaGeneratorBuilder, groto::Groto};
use quote::quote;

#[proc_macro_derive(Object, attributes(grost))]
pub fn derive_object(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let object = match Object::from_derive_input(input.into()) {
    Ok(input) => input,
    Err(e) => {
      return e.write_errors().into();
    }
  };

  let groto = Groto::new(object.mir().path_to_grost());
  let mut builder = SchemaGeneratorBuilder::new();
  let Ok(_) = builder.add_flavor(groto.name(), groto) else {
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

#[proc_macro_attribute]
pub fn object(
  args: proc_macro::TokenStream,
  input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
  let object = match Object::from_attribute_input(args.into(), input.into()) {
    Ok(input) => input,
    Err(e) => return e.write_errors().into(),
  };

  let groto = Groto::new(object.mir().path_to_grost());
  let mut builder = SchemaGeneratorBuilder::new();
  let Ok(_) = builder.add_flavor(groto.name(), groto) else {
    panic!("failed to add flavor");
  };
  builder.objects_mut().insert(object);
  let generator = builder.build();
  let codegen = match generator.derive() {
    Ok(codegen) => codegen,
    Err(e) => return e.into_compile_error().into(),
  };

  quote! {
    #codegen
  }
  .into()
}
