use syn::parse_quote;

/// The Mid-level Intermediate Representation for objects in grost schema,
pub mod object;

fn wire_format_reflection_ty(
  path_to_grost: &syn::Path,
  object_name: &syn::Ident,
  object_generics: &syn::Generics,
  tag: u32,
  flavor_param: &syn::TypeParam,
) -> syn::Type {
  let ident = &flavor_param.ident;
  let (_, tg, _) = object_generics.split_for_impl();
  parse_quote! {
    #path_to_grost::__private::reflection::WireFormatReflection<
      #object_name #tg,
      #ident,
      #tag,
    >
  }
}
