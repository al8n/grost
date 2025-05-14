#[derive(grost_darling::ObjectField)]
#[grost(attributes(grost))]
pub struct ObjectField {
  #[grost(darling(rename = "crate", default = "super::super::default_path"))]
  path_to_crate: syn::Path,
}

// #[derive(Debug, FromField)]
// #[darling(attributes(grost), forward_attrs)]
// pub struct ObjectFieldDeriveInput {
//   ident: Option<Ident>,
//   ty: Type,
//   vis: Visibility,
//   attrs: Vec<Attribute>,
//   #[darling(default)]
//   schema: SchemaMeta,
//   #[darling(default)]
//   default: Option<syn::Path>,
//   tag: NonZeroU32,
//   #[darling(default)]
//   wire: Option<Type>,
//   #[darling(default)]
//   partial: PartialFieldMeta,
//   #[darling(default)]
//   partial_ref: PartialRefFieldMeta,
//   #[darling(default)]
//   select: Selection,
//   #[darling(default)]
//   copy: bool,
//   #[darling(flatten)]
//   hint: TypeHintMeta,
// }
