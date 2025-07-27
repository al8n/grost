use darling::usage::{IdentSet, LifetimeSet};
use heck::ToUpperCamelCase;
use quote::{format_ident, quote};
use syn::{
  Attribute, Ident, Type, Variant, Visibility,
  parse::{Parse, Parser},
};

pub use convert::*;
pub use selector::SelectorFieldOptions;

use crate::utils::Invokable;

mod convert;
mod selector;

#[derive(Debug, Clone)]
pub struct Index {
  variant: Variant,
  index: usize,
}

impl Index {
  pub(super) fn new(index: usize, field_name: &Ident, tag: u32) -> darling::Result<Self> {
    let variant = format_ident!("{}", field_name.to_string().to_upper_camel_case());
    let variant_doc = format!(" The field indexer for the field `{field_name}`");

    Ok(Self {
      variant: syn::Variant::parse.parse2(quote! {
        #[doc = #variant_doc]
        #variant = #tag
      })?,
      index,
    })
  }

  /// Returns the variant
  #[inline]
  pub const fn variant(&self) -> &Variant {
    &self.variant
  }

  /// Returns the index of the field, this index is the index of the field in the object.
  #[inline]
  pub const fn index(&self) -> usize {
    self.index
  }
}

#[derive(Debug, Clone)]
pub struct SkippedField<M = ()> {
  pub(super) attrs: Vec<Attribute>,
  pub(super) vis: Visibility,
  pub(super) name: Ident,
  pub(super) ty: Type,
  pub(super) lifetimes_usages: LifetimeSet,
  pub(super) type_params_usages: IdentSet,
  pub(super) meta: M,
  pub(super) default: Invokable,
}

impl<M> SkippedField<M> {
  /// Returns the name of the skipped field
  #[inline]
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  /// Returns the visibility of the skipped field
  #[inline]
  pub const fn vis(&self) -> &Visibility {
    &self.vis
  }

  /// Returns the type of the skipped field
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the attributes of the skipped field
  #[inline]
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns the lifetimes used in the skipped field
  #[inline]
  pub const fn lifetimes_usages(&self) -> &LifetimeSet {
    &self.lifetimes_usages
  }

  /// Returns the type parameters used in the skipped field
  #[inline]
  pub const fn type_params_usages(&self) -> &IdentSet {
    &self.type_params_usages
  }

  /// Returns `true` if the skipped field contains any generics.
  #[inline]
  pub fn use_generics(&self) -> bool {
    !self.lifetimes_usages.is_empty() || !self.type_params_usages.is_empty()
  }

  /// Returns the custom metadata associated with the skipped field
  #[inline]
  pub const fn meta(&self) -> &M {
    &self.meta
  }
}

impl SkippedField {
  pub(super) fn with_meta<M>(self, meta: M) -> SkippedField<M> {
    SkippedField {
      attrs: self.attrs,
      vis: self.vis,
      name: self.name,
      ty: self.ty,
      lifetimes_usages: self.lifetimes_usages,
      type_params_usages: self.type_params_usages,
      meta,
      default: self.default,
    }
  }

  pub(super) fn from_raw(
    f: RawSkippedField,
    lifetimes_usages: LifetimeSet,
    type_params_usages: IdentSet,
  ) -> darling::Result<Self> {
    let (f, extra) = f.extract();

    Ok(SkippedField {
      attrs: f.attrs,
      vis: f.vis,
      name: f.name,
      ty: f.ty,
      lifetimes_usages,
      type_params_usages,
      meta: extra,
      default: f.default,
    })
  }
}

#[derive(Debug, Clone)]
pub struct RawSkippedField<M = ()> {
  pub(super) attrs: Vec<Attribute>,
  pub(super) vis: Visibility,
  pub(super) name: Ident,
  pub(super) ty: Type,
  pub(super) extra: M,
  pub(super) default: Invokable,
}

impl<M> RawSkippedField<M> {
  pub(super) fn extract(self) -> (RawSkippedField, M) {
    let Self {
      attrs,
      vis,
      name,
      ty,
      extra,
      default,
    } = self;
    (
      RawSkippedField {
        attrs,
        vis,
        name,
        ty,
        extra: (),
        default,
      },
      extra,
    )
  }

  // /// Returns the name of the skipped field
  // #[inline]
  // pub const fn name(&self) -> &Ident {
  //   &self.name
  // }

  // /// Returns the visibility of the skipped field
  // #[inline]
  // pub const fn vis(&self) -> &Visibility {
  //   &self.vis
  // }

  // /// Returns the type of the skipped field
  // #[inline]
  // pub const fn ty(&self) -> &Type {
  //   &self.ty
  // }

  // /// Returns the attributes of the skipped field
  // #[inline]
  // pub const fn attrs(&self) -> &[Attribute] {
  //   self.attrs.as_slice()
  // }

  // /// Returns the path to the default value function for the skipped field
  // #[inline]
  // pub const fn default(&self) -> &Invokable {
  //   &self.default
  // }

  // /// Returns the extra metadata associated with the skipped field
  // #[inline]
  // pub const fn extra(&self) -> &M {
  //   &self.extra
  // }

  // pub(in crate::object) fn try_new(
  //   f: RawSkippedField<M>,
  // ) -> darling::Result<Self>
  // where
  //   M: Clone,
  // {
  //   let attrs = f.attrs;
  //   let vis = f.vis;
  //   let name = f.name;
  //   let ty = f.ty;
  //   let default = match f.default {
  //     Some(path) => path,
  //     None => {
  //       syn::parse2::<syn::Path>(quote! { ::core::default::Default::default }).map(Into::into)?
  //     }
  //   };

  //   Ok(Self {
  //     attrs,
  //     vis,
  //     name,
  //     ty,
  //     default,
  //     extra: f.extra,
  //   })
  // }
}

// #[derive(Debug, Clone)]
// pub enum FieldOptions<SO, TO> {
//   Skipped {
//     default: Option<Invokable>,
//     extra: SO,
//   },
//   Tagged {
//     label: Label,
//     schema: SchemaOptions,
//     default: Option<Invokable>,
//     tag: NonZeroU32,
//     flavor: Vec<FieldFlavorAttribute>,
//     convert: ConvertAttribute,
//     partial: PartialFieldOptions,
//     partial_ref: PartialRefFieldOptions,
//     selector: SelectorFieldOptions,
//     copy: bool,
//     extra: TO,
//   },
// }

// impl<SO, TO> FieldOptions<SO, TO> {
//   /// Returns the fn which will be used to generate the default value for the field
//   pub const fn default(&self) -> Option<&Invokable> {
//     match self {
//       Self::Skipped { default, .. } => default.as_ref(),
//       Self::Tagged { default, .. } => default.as_ref(),
//     }
//   }
// }

// impl<SO, TO> FieldFromMeta<SO, TO> {
//   pub fn finalize(self) -> darling::Result<FieldOptions<SO, TO>> {
//     Ok(match self {
//       Self::Skipped { default, extra } => FieldOptions::Skipped { default, extra },
//       Self::Tagged {
//         label,
//         schema,
//         default,
//         tag,
//         flavor,
//         convert,
//         partial,
//         partial_ref,
//         selector,
//         copy,
//         extra,
//       } => FieldOptions::Tagged {
//         label,
//         schema: schema.into(),
//         default,
//         tag,
//         flavor: flavor.finalize()?,
//         convert: convert.finalize()?,
//         partial: partial.finalize(),
//         partial_ref: partial_ref.finalize()?,
//         selector: selector.finalize(),
//         copy,
//         extra,
//       },
//     })
//   }
// }

// #[derive(Debug, Clone)]
// pub struct RawTaggedField<E> {
//   pub name: Ident,
//   pub ty: Type,
//   pub vis: Visibility,
//   pub attrs: Vec<Attribute>,
//   pub label: Label,
//   pub schema: SchemaOptions,
//   pub default: Option<Invokable>,
//   pub tag: NonZeroU32,
//   pub flavor: Vec<FieldFlavorAttribute>,
//   pub convert: ConvertAttribute,
//   pub partial: PartialFieldOptions,
//   pub partial_ref: PartialRefFieldOptions,
//   pub selector: SelectorFieldOptions,
//   pub copy: bool,
//   pub extra: E,
// }

// #[derive(Debug, Clone)]
// pub enum RawField<TM = (), SM = ()> {
//   Skipped(RawSkippedField<SM>),
//   Tagged(RawTaggedField<TM>),
// }

// impl<TM, SM> RawField<TM, SM> {
//   /// Creates a new raw field
//   pub fn new(
//     name: Ident,
//     ty: Type,
//     vis: Visibility,
//     attrs: Vec<Attribute>,
//     meta: FieldFromMeta<TM, SM>,
//   ) -> darling::Result<Self> {
//     match meta {
//       FieldFromMeta::Skipped { default, extra } => Ok(Self::Skipped(RawSkippedField {
//         name,
//         ty,
//         vis,
//         attrs,
//         default,
//         extra,
//       })),
//       FieldFromMeta::Tagged {
//         label,
//         schema,
//         default,
//         tag,
//         flavor,
//         convert,
//         partial,
//         partial_ref,
//         selector,
//         copy,
//         extra,
//       } => Ok(Self::Tagged(RawTaggedField {
//         name,
//         ty,
//         vis,
//         attrs,
//         label,
//         schema: schema.into(),
//         default,
//         tag,
//         flavor: flavor.finalize()?,
//         convert: convert.finalize()?,
//         partial: partial.finalize(),
//         partial_ref: partial_ref.finalize()?,
//         selector: selector.finalize(),
//         copy,
//         extra,
//       })),
//     }
//   }
// }
