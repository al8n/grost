use syn::parse_quote;

/// The Mid-level Intermediate Representation for objects in grost schema,
pub mod object;

fn wire_format_reflection_ty(
  path_to_grost: &syn::Path,
  reflection_name: &syn::Ident,
  tag: u32,
  flavor_generic: &syn::Ident,
) -> syn::Type {
  parse_quote! {
    #reflection_name<
      (
        #path_to_grost::__private::reflection::WireFormatReflection,
        #path_to_grost::__private::RawTag<#tag>,
      ),
      #flavor_generic,
    >
  }
}
