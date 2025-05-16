use darling::{FromDeriveInput, util::Ignored};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(grost), forward_attrs, supports(struct_named))]
pub struct Interface {
  ident: syn::Ident,
  vis: syn::Visibility,
  generics: syn::Generics,
  // data: darling::ast::Data<InterfaceField, Ignored>,
}
