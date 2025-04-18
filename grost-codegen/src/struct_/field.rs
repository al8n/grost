use grost_types::Tag;
use heck::ToShoutySnakeCase;
use quote::{ToTokens, format_ident, quote};
use smol_str::{SmolStr, format_smolstr};
use syn::{Attribute, Expr, Visibility};

use crate::{SafeIdent, WireTypeExt, ty::Ty};
use getter::Getter;
use setter::Setter;

/// The getter related types to a field
pub mod getter;
/// The setter related types to a field
pub mod setter;

#[derive(Clone)]
pub struct Field {
  name: SafeIdent,
  schema_name: SmolStr,
  visibility: Option<Visibility>,
  description: Option<SmolStr>,
  default: Option<Expr>,
  ty: Ty,
  defination_attrs: Vec<Attribute>,
  getter: Option<Getter>,
  mutable_getter: Option<Getter>,
  setter: Option<Setter>,
  movable_setter: Option<Setter>,
  tag: Tag,
}

impl Field {
  pub fn new(name: SafeIdent, ty: Ty, tag: Tag) -> Self {
    Self {
      getter: None,
      mutable_getter: None,
      schema_name: name.original_str().into(),
      setter: None,
      movable_setter: None,
      defination_attrs: Vec::new(),
      name,
      visibility: None,
      description: None,
      default: None,
      ty,
      tag,
    }
    .with_getter(None, Option::<&str>::None, None, false)
    .with_mutable_getter(None, Option::<&str>::None, None, false)
    .with_setter(None, Option::<&str>::None, None, false)
    .with_movable_setter(None, Option::<&str>::None, None, false)
  }

  /// Sets the schema name of the field
  ///
  /// By default, the schema name is the [`self.name.original_str()`](SafeIdent::original_str) of the field
  pub fn with_schema_name(mut self, name: impl Into<SmolStr>) -> Self {
    self.schema_name = name.into();
    self
  }

  pub fn with_defination_attrs(mut self, attrs: Vec<Attribute>) -> Self {
    self.defination_attrs = attrs;
    self
  }

  /// Clear the getter
  pub fn deny_getter(mut self) -> Self {
    self.getter = None;
    self
  }

  /// Clear the mutable getter
  pub fn deny_mutable_getter(mut self) -> Self {
    self.mutable_getter = None;
    self
  }

  /// Clear the setter
  pub fn deny_setter(mut self) -> Self {
    self.setter = None;
    self
  }

  /// Clear the movable setter
  pub fn deny_movable_setter(mut self) -> Self {
    self.movable_setter = None;
    self
  }

  pub fn with_getter(
    mut self,
    fn_name: Option<SafeIdent>,
    description: Option<impl Into<SmolStr>>,
    converter: Option<getter::Converter>,
    const_fn: bool,
  ) -> Self {
    let name_str = self.name.name_str();
    let doc = description.map(Into::into).unwrap_or(format_smolstr!(
      r#"
      Gets the reference of the field `{name_str}`.

      {}
      "#,
      self
        .accessor_description_field_description()
        .unwrap_or_else(|| SmolStr::new("")),
    ));

    self.getter = Some(
      converter
        .map(|converter| Getter::new_with_converter(self.name.clone(), converter))
        .unwrap_or(Getter::new(self.name.clone(), self.ty.ty().clone()))
        .with_mutable(false)
        .with_const_fn(if self.ty.copy() { true } else { const_fn })
        .with_description(doc)
        .with_fn_name(fn_name)
        .with_copy(self.ty.copy()),
    );
    self
  }

  pub fn with_mutable_getter(
    mut self,
    fn_name: Option<SafeIdent>,
    description: Option<impl Into<SmolStr>>,
    converter: Option<getter::Converter>,
    const_fn: bool,
  ) -> Self {
    let name_str = self.name.name_str();
    let doc = description.map(Into::into).unwrap_or(format_smolstr!(
      r###"
      Gets the mutable reference of the field `{name_str}`.

      {}
      "###,
      self
        .accessor_description_field_description()
        .unwrap_or_else(|| SmolStr::new("")),
    ));

    self.mutable_getter = Some(
      converter
        .map(|converter| Getter::new_with_converter(self.name.clone(), converter))
        .unwrap_or(Getter::new(self.name.clone(), self.ty.ty().clone()))
        .with_mutable(true)
        .with_const_fn(if self.ty.copy() { true } else { const_fn })
        .with_description(doc)
        .with_fn_name(fn_name)
        .with_copy(self.ty.copy()),
    );
    self
  }

  pub fn with_setter(
    mut self,
    fn_name: Option<SafeIdent>,
    description: Option<impl Into<SmolStr>>,
    converter: Option<setter::Converter>,
    const_fn: bool,
  ) -> Self {
    let name_str = self.name.name_str();
    let doc = description.map(Into::into).unwrap_or(format_smolstr!(
      r###"
      Sets the `{name_str}`.

      {}
      "###,
      self
        .accessor_description_field_description()
        .unwrap_or_else(|| SmolStr::new("")),
    ));

    self.setter = Some(
      converter
        .map(|converter| Setter::new_with_converter(self.name.clone(), converter))
        .unwrap_or(Setter::new(self.name.clone(), self.ty.ty().clone()))
        .with_description(doc)
        .with_fn_name(fn_name)
        .with_const_fn(if self.ty.copy() { true } else { const_fn })
        .with_take(false),
    );
    self
  }

  pub fn with_movable_setter(
    mut self,
    fn_name: Option<SafeIdent>,
    description: Option<impl Into<SmolStr>>,
    converter: Option<setter::Converter>,
    const_fn: bool,
  ) -> Self {
    let name_str = self.name.name_str();
    let doc = description.map(Into::into).unwrap_or(format_smolstr!(
      r###"
      Sets the `{name_str}`.

      {}
      "###,
      self
        .accessor_description_field_description()
        .unwrap_or_else(|| SmolStr::new("")),
    ));

    self.movable_setter = Some(
      converter
        .map(|converter| Setter::new_with_converter(self.name.clone(), converter))
        .unwrap_or(Setter::new(self.name.clone(), self.ty.ty().clone()))
        .with_description(doc)
        .with_fn_name(fn_name)
        .with_const_fn(if self.ty.copy() { true } else { const_fn })
        .with_take(true),
    );
    self
  }

  pub fn with_visibility(mut self, visibility: Visibility) -> Self {
    self.visibility = Some(visibility);
    self
  }

  pub fn with_description(mut self, description: SmolStr) -> Self {
    self.description = Some(description);
    self
  }

  pub fn with_default(mut self, default: Expr) -> Self {
    self.default = Some(default);
    self
  }

  pub fn with_tag(mut self, tag: Tag) -> Self {
    self.tag = tag;
    self
  }

  pub fn name(&self) -> &SafeIdent {
    &self.name
  }

  pub fn schema_name(&self) -> &str {
    &self.schema_name
  }

  pub fn description(&self) -> Option<&str> {
    self.description.as_deref()
  }

  pub fn default(&self) -> Option<&Expr> {
    self.default.as_ref()
  }

  pub fn ty(&self) -> &Ty {
    &self.ty
  }

  pub fn tag(&self) -> Tag {
    self.tag
  }

  pub fn visibility(&self) -> Option<&Visibility> {
    self.visibility.as_ref()
  }

  pub fn getter(&self) -> Option<&Getter> {
    self.getter.as_ref()
  }

  pub fn mutable_getter(&self) -> Option<&Getter> {
    self.mutable_getter.as_ref()
  }

  pub(crate) fn field_defination(&self) -> proc_macro2::TokenStream {
    let name = &self.name;
    let description = self.description.as_ref().map(|d| {
      let s = d.as_str();
      quote! {
        #[doc = #s]
      }
    });
    let ty = &self.ty;
    let visibility = self.visibility.as_ref();
    let attrs = &self.defination_attrs;

    quote! {
      #(#attrs)*
      #description
      #visibility #name: #ty
    }
  }

  pub(crate) fn field_accessors(&self) -> proc_macro2::TokenStream {
    let getter = self.getter.as_ref();
    let mutable_getter = self.mutable_getter.as_ref();
    let setter = self.setter.as_ref();
    let movable_setter = self.movable_setter.as_ref();
    quote! {
      #getter

      #mutable_getter

      #setter

      #movable_setter
    }
  }

  pub(crate) fn field_consts(&self, path_to_grost: &syn::Path) -> proc_macro2::TokenStream {
    let name = self.name.name_str().to_shouty_snake_case();
    let field_name = self.name.name_str();
    let tag_const_name = format_ident!("__{}_TAG__", name);
    let field_info_name = format_ident!("{}_REFLECTION", name);
    let field_info_doc = format!(" The reflection information of the `{field_name}` field");
    let tag = self.tag.get();
    let ty_name = self.ty.ty().to_token_stream().to_string().replace(" ", "");
    let schema_name = self.schema_name();
    let schema_ty_name = self.ty.schema_type();

    let identifier_name = format_ident!("__{}_IDENTIFIER__", name);
    let identifier_encoded_len_name = format_ident!("__{}_IDENTIFIER_ENCODED_LEN__", name);
    let identifier_encode_name = format_ident!("__ENCODED_{}_IDENTIFIER__", name);
    let wt_tokens = self.ty.wire_type().to_tokens(path_to_grost);

    quote! {
      const #tag_const_name: #path_to_grost::__private::Tag = #path_to_grost::__private::Tag::new(#tag);
      const #identifier_name: #path_to_grost::__private::Identifier = #path_to_grost::__private::Identifier::new(#wt_tokens, Self::#tag_const_name);
      const #identifier_encoded_len_name: ::core::primitive::usize = Self::#identifier_name.encoded_len();
      const #identifier_encode_name: &[::core::primitive::u8] = Self::#identifier_name.encode().as_slice();

      #[doc = #field_info_doc]
      pub const #field_info_name: #path_to_grost::__private::FieldRelection = #path_to_grost::__private::FieldRelectionBuilder {
        identifier: Self::#identifier_name,
        encoded_identifier_len: Self::#identifier_encoded_len_name,
        encoded_identifier: Self::#identifier_encode_name,
        name: #field_name,
        ty: #ty_name,
        schema_name: #schema_name,
        schema_type: #schema_ty_name,
      }.build();
    }
  }

  fn accessor_description_field_description(&self) -> Option<SmolStr> {
    self.description.as_ref().map(|d| {
      let s = d.as_str();
      format_smolstr! {
        r###"
        ## Field description

        {s}
        "###,
      }
    })
  }
}
