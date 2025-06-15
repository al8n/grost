use indexmap::IndexMap;
use quote::{ToTokens, quote};
use syn::{Attribute, Generics, Ident, LifetimeParam, Path, Type, TypeParam, Visibility};

use crate::utils::Invokable;

use super::{
  super::ast::{GenericObject as GenericObjectAst, Indexer},
  accessors,
};

pub use field::*;
pub use flavor::*;
pub use partial::*;
pub use partial_decoded::*;
pub use reflection::*;
pub use selector::*;

mod field;
mod flavor;
mod indexer;
mod partial;
mod partial_decoded;
mod reflection;
mod selector;

#[derive(Debug, Clone)]
pub struct GenericObject<M, F> {
  path_to_grost: Path,
  attrs: Vec<Attribute>,
  default: Option<Invokable>,
  schema_name: String,
  schema_description: String,
  name: Ident,
  vis: Visibility,
  ty: Type,
  reflectable: Type,
  fields: Vec<GenericField<F>>,
  indexer: Indexer,
  flavor_param: TypeParam,
  unknown_buffer_param: TypeParam,
  lifetime_param: syn::LifetimeParam,
  wire_format_param: TypeParam,
  generics: Generics,
  partial: GenericPartialObject,
  partial_decoded: GenericPartialDecodedObject,
  reflection: GenericObjectReflection,
  selector: GenericSelector,
  selector_iter: GenericSelectorIter,
  flavors: IndexMap<Ident, ObjectFlavor>,
  meta: M,
}

impl<M, F> ToTokens for GenericObject<M, F> {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let name = self.name();
    let vis = self.vis();
    let generics = self.generics();
    let wc = generics.where_clause.as_ref();
    let attrs = self.attrs();

    let fields = self.fields().iter().map(|f| {
      let name = f.name();
      let ty = f.ty();
      let vis = f.vis();
      let attrs = f.attrs();

      quote! {
        #(#attrs)*
        #vis #name: #ty
      }
    });

    tokens.extend(quote! {
      #(#attrs)*
      #vis struct #name #generics #wc {
        #(#fields),*
      }
    });
  }
}

impl<M, F> GenericObject<M, F> {
  /// Returns the path to the `grost` crate.
  #[inline]
  pub const fn path_to_grost(&self) -> &Path {
    &self.path_to_grost
  }

  /// Returns the attributes of the object.
  #[inline]
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns the name of the object.
  #[inline]
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  /// Returns the visibility of the object.
  #[inline]
  pub const fn vis(&self) -> &Visibility {
    &self.vis
  }

  /// Returns the schema name of the object.
  #[inline]
  pub const fn schema_name(&self) -> &str {
    self.schema_name.as_str()
  }

  /// Returns the schema description of the object.
  #[inline]
  pub const fn schema_description(&self) -> &str {
    self.schema_description.as_str()
  }

  /// Returns the type of the object.
  ///
  /// e.g. if the name is `UserObject`, this will return `UserObject<T>`.
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the reflectable type of the object.
  ///
  /// e.g. if the name is `UserObject<T>`, this will return `Reflectable<UserObject<T>>`.
  #[inline]
  pub const fn reflectable(&self) -> &Type {
    &self.reflectable
  }

  /// Returns the generics of the object.
  #[inline]
  pub const fn generics(&self) -> &Generics {
    &self.generics
  }

  /// Returns the fields of the object.
  #[inline]
  pub const fn fields(&self) -> &[GenericField<F>] {
    self.fields.as_slice()
  }

  /// Returns the partial object information.
  #[inline]
  pub const fn partial(&self) -> &GenericPartialObject {
    &self.partial
  }

  /// Returns the partial decoded object information.
  #[inline]
  pub const fn partial_decoded(&self) -> &GenericPartialDecodedObject {
    &self.partial_decoded
  }

  /// Returns the selector information of the object.
  #[inline]
  pub const fn selector(&self) -> &GenericSelector {
    &self.selector
  }

  /// Returns the selector iterator information of the object.
  #[inline]
  pub const fn selector_iter(&self) -> &GenericSelectorIter {
    &self.selector_iter
  }

  /// Returns the generic flavor parameter, which will be used in generated structs or impls.
  #[inline]
  pub const fn flavor_param(&self) -> &TypeParam {
    &self.flavor_param
  }

  /// Returns the generic unknown buffer type parameter, which will be used in generated structs or impls.
  #[inline]
  pub const fn unknown_buffer_param(&self) -> &TypeParam {
    &self.unknown_buffer_param
  }

  /// Returns the generic lifetime parameter, which will be used in generated structs or impls.
  #[inline]
  pub const fn lifetime_param(&self) -> &LifetimeParam {
    &self.lifetime_param
  }

  /// Returns the generic wire format type parameter, which will be used in generated structs or impls.
  #[inline]
  pub const fn wire_format_type_param(&self) -> &TypeParam {
    &self.wire_format_param
  }

  /// Returns the flavors of the object.
  #[inline]
  pub const fn flavors(&self) -> &IndexMap<Ident, ObjectFlavor> {
    &self.flavors
  }

  /// Returns the custom metadata associated with the object.
  #[inline]
  pub const fn meta(&self) -> &M {
    &self.meta
  }

  /// Returns the indexer information of the object
  #[inline]
  pub const fn indexer(&self) -> &Indexer {
    &self.indexer
  }

  pub(super) fn from_ast(object: GenericObjectAst<M, F>) -> darling::Result<Self>
  where
    M: Clone,
    F: Clone,
  {
    let path_to_grost = object.path_to_grost().clone();
    let mut fields = object.fields().to_vec();
    fields.sort_by_key(|f| f.tag().unwrap_or(u32::MAX));

    let fields = fields
      .iter()
      .cloned()
      .enumerate()
      .map(|(idx, f)| GenericField::from_ast(&object, idx, f))
      .collect::<darling::Result<Vec<_>>>()?;

    let partial = GenericPartialObject::from_ast(&object, &fields)?;
    let partial_decoded = GenericPartialDecodedObject::from_ast(&object, &fields)?;
    let flavors = object
      .flavors
      .iter()
      .map(|(name, flavor)| {
        ObjectFlavor::from_ast(&object, name, flavor, &fields).map(|f| (name.clone(), f))
      })
      .collect::<darling::Result<_>>()?;
    let selector = GenericSelector::from_ast(&object, &fields)?;
    let selector_iter = selector.selector_iter(&object)?;
    let reflection = GenericObjectReflection::from_ast(&object, &fields)?;

    Ok(Self {
      path_to_grost,
      reflection,
      attrs: object.attrs,
      default: object.default,
      name: object.name,
      schema_description: object.schema_description,
      schema_name: object.schema_name,
      flavor_param: object.flavor_param,
      vis: object.vis,
      ty: object.ty,
      reflectable: object.reflectable,
      indexer: object.indexer,
      fields,
      generics: object.generics,
      meta: object.meta,
      partial,
      partial_decoded,
      selector,
      selector_iter,
      flavors,
      unknown_buffer_param: object.unknown_buffer_param,
      lifetime_param: object.lifetime_param,
      wire_format_param: object.wire_format_param,
    })
  }

  pub(super) fn derive(&self) -> darling::Result<proc_macro2::TokenStream> {
    let default = self.derive_default()?;
    let accessors = self.derive_accessors()?;

    let indexer = self.derive_indexer_defination();
    let indexer_impl = self.derive_indexer();

    let partial_object = self.derive_partial_object_defination();
    let partial_object_impl = self.derive_partial_object();

    let partial_decoded_object = self.derive_partial_decoded_object_defination();

    let selector = self.derive_selector_defination();
    let selector_impl = self.derive_selector()?;
    let selector_iter = self.derive_selector_iter_defination();
    let selector_iter_impl = self.derive_selector_iter();

    let reflection_impl = self.derive_reflection();

    Ok(quote! {
      #indexer

      #partial_object

      #partial_decoded_object

      #selector

      #selector_iter

      const _: () = {
        #default

        #accessors

        #indexer_impl

        #partial_object_impl

        #reflection_impl

        #selector_impl

        #selector_iter_impl
      };
    })
  }

  fn derive_accessors(&self) -> darling::Result<proc_macro2::TokenStream> {
    let name = self.name();
    let (ig, tg, wc) = self.generics().split_for_impl();

    let accessors = self
      .fields()
      .iter()
      .filter_map(|f| f.try_unwrap_tagged_ref().ok())
      .map(|f| {
        let field_name = f.name();
        let ty = f.ty();
        let copy = f.copy();

        accessors(field_name, f.vis(), ty, copy)
      });

    Ok(quote! {
      impl #ig #name #tg #wc {
        #(#accessors)*
      }
    })
  }

  fn derive_default(&self) -> darling::Result<proc_macro2::TokenStream> {
    let name = self.name();
    let (ig, tg, wc) = self.generics().split_for_impl();

    if let Some(default) = &self.default {
      Ok(quote! {
        impl #ig ::core::default::Default for #name #tg #wc {
          fn default() -> Self {
            Self::new()
          }
        }

        impl #ig ::core::default::Default for #name #tg #wc {
          /// Creates a new instance of the object with default values.
          pub fn new() -> Self {
            (#default)()
          }
        }
      })
    } else {
      let fields = self.fields().iter().map(|f| {
        let name = f.name();
        let default = f.default();
        quote! {
          #name: (#default)()
        }
      });

      Ok(quote! {
        impl #ig ::core::default::Default for #name #tg #wc {
          fn default() -> Self {
            Self::new()
          }
        }

        impl #ig #name #tg #wc {
          /// Creates a new instance of the object with default values.
          pub fn new() -> Self {
            Self {
              #(#fields),*
            }
          }
        }
      })
    }
  }
}
