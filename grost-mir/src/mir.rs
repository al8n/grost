use quote::ToTokens;
use syn::parse_quote;

/// The Mid-level Intermediate Representation for objects in grost schema,
pub mod object;

fn wire_format_reflection_ty(
  path_to_grost: &syn::Path,
  object_name: &syn::Ident,
  object_type_generics: &syn::TypeGenerics<'_>,
  tag: u32,
  flavor: impl ToTokens,
) -> syn::Type {
  parse_quote! {
    #path_to_grost::__private::reflection::WireFormatReflection<
      #object_name #object_type_generics,
      #flavor,
      #tag,
    >
  }
}
