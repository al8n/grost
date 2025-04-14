use quote::quote;
use smol_str::{SmolStr, format_smolstr};
use syn::{Attribute, Expr, Visibility};

use crate::{SafeIdent, ty::Ty};
use getter::Getter;
use setter::Setter;

/// The getter related types to a field
pub mod getter;
/// The setter related types to a field
pub mod setter;

pub struct Field {
  name: SafeIdent,
  visibility: Option<Visibility>,
  description: Option<SmolStr>,
  default: Option<Expr>,
  ty: Ty,
  defination_attrs: Vec<Attribute>,
  getter: Option<Getter>,
  mutable_getter: Option<Getter>,
  setter: Option<Setter>,
  movable_setter: Option<Setter>,
  tag: u32,
}

impl Field {
  pub fn new(name: SafeIdent, ty: Ty) -> Self {
    Self {
      getter: None,
      mutable_getter: None,
      setter: None,
      movable_setter: None,
      defination_attrs: Vec::new(),
      name,
      visibility: None,
      description: None,
      default: None,
      ty,
      tag: 0,
    }
      .with_getter(None, Option::<&str>::None, None, false)
      .with_mutable_getter(None, Option::<&str>::None, None, false)
      .with_setter(None, Option::<&str>::None, None, false)
      .with_movable_setter(None, Option::<&str>::None, None, false)
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
      self.accessor_description_field_description().unwrap_or_else(|| SmolStr::new("")),
    ));

    self.getter = Some(
      converter.map(|converter| {
        Getter::new_with_converter(self.name.clone(), converter)
      }).unwrap_or(
        Getter::new(self.name.clone(), self.ty.ty().clone())
      )
      .with_mutable(false)
      .with_const_fn(if self.ty.copy() {
        true
      } else {
        const_fn
      })
      .with_description(doc)
      .with_fn_name(fn_name)
      .with_copy(self.ty.copy())
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
      self.accessor_description_field_description().unwrap_or_else(|| SmolStr::new("")),
    ));

    self.mutable_getter = Some(
      converter.map(|converter| {
        Getter::new_with_converter(self.name.clone(), converter)
      }).unwrap_or(
        Getter::new(self.name.clone(), self.ty.ty().clone())
      )
      .with_mutable(true)
      .with_const_fn(if self.ty.copy() {
        true
      } else {
        const_fn
      })
      .with_description(doc)
      .with_fn_name(fn_name)
      .with_copy(self.ty.copy())
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
      self.accessor_description_field_description().unwrap_or_else(|| SmolStr::new("")),
    ));

    self.setter = Some(
      converter.map(|converter| {
        Setter::new_with_converter(self.name.clone(), converter)
      }).unwrap_or(
        Setter::new(self.name.clone(), self.ty.ty().clone())
      )
      .with_description(doc)
      .with_fn_name(fn_name)
      .with_const_fn(if self.ty.copy() {
        true
      } else {
        const_fn
      })
      .with_take(false)
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
      self.accessor_description_field_description().unwrap_or_else(|| SmolStr::new("")),
    ));

    self.movable_setter = Some(
      converter.map(|converter| {
        Setter::new_with_converter(self.name.clone(), converter)
      }).unwrap_or(
        Setter::new(self.name.clone(), self.ty.ty().clone())
      )
      .with_description(doc)
      .with_fn_name(fn_name)
      .with_const_fn(if self.ty.copy() {
        true
      } else {
        const_fn
      })
      .with_take(true)
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

  pub fn with_tag(mut self, tag: u32) -> Self {
    self.tag = tag;
    self
  }

  pub fn name(&self) -> &SafeIdent {
    &self.name
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

  pub fn tag(&self) -> u32 {
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
