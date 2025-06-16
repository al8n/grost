use quote::{ToTokens, quote};
use syn::{Attribute, Generics, Ident, LifetimeParam, Path, Type, TypeParam, Visibility};

use super::{super::ast::ConcreteObject as ConcreteObjectAst, accessors};
use crate::{flavor::FlavorAttribute, object::ast::Indexer, utils::Invokable};

pub use field::*;
pub use partial::*;
pub use partial_decoded::*;
pub use reflection::*;
pub use selector::*;

mod decode;
mod encode;
mod field;
mod indexer;
mod partial;
mod partial_decoded;
mod reflection;
mod selector;

#[derive(Debug, Clone)]
pub struct ConcreteObject<M = (), F = ()> {
  path_to_grost: Path,
  attrs: Vec<Attribute>,
  name: Ident,
  schema_name: String,
  schema_description: String,
  vis: Visibility,
  ty: Type,
  reflectable: Type,
  generics: Generics,
  flavor: FlavorAttribute,
  unknown_buffer_param: TypeParam,
  lifetime_param: LifetimeParam,
  read_buffer_param: TypeParam,
  write_buffer_param: TypeParam,
  fields: Vec<ConcreteField<F>>,
  indexer: Indexer,
  default: Option<Invokable>,
  partial: ConcretePartialObject,
  partial_decoded: ConcretePartialDecodedObject,
  selector: ConcreteSelector,
  selector_iter: ConcreteSelectorIter,
  reflection: ConcreteObjectReflection,
  meta: M,
}

impl<M, F> ToTokens for ConcreteObject<M, F> {
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

impl<M, F> ConcreteObject<M, F> {
  /// Returns the path to the grost crate.
  #[inline]
  pub const fn path_to_grost(&self) -> &Path {
    &self.path_to_grost
  }

  /// Returns the attributes of the concrete object.
  #[inline]
  pub const fn attrs(&self) -> &[Attribute] {
    self.attrs.as_slice()
  }

  /// Returns the name of the concrete object.
  #[inline]
  pub const fn name(&self) -> &Ident {
    &self.name
  }

  /// Returns the schema name of the concrete object.
  #[inline]
  pub const fn schema_name(&self) -> &str {
    self.schema_name.as_str()
  }

  /// Returns the schema description of the concrete object.
  #[inline]
  pub const fn schema_description(&self) -> &str {
    self.schema_description.as_str()
  }

  /// Returns the visibility of the concrete object.
  #[inline]
  pub const fn vis(&self) -> &Visibility {
    &self.vis
  }

  /// Returns the type of the concrete object.
  #[inline]
  pub const fn ty(&self) -> &Type {
    &self.ty
  }

  /// Returns the reflectable type of the concrete object.
  #[inline]
  pub const fn reflectable(&self) -> &Type {
    &self.reflectable
  }

  /// Returns the generics of the concrete object.
  #[inline]
  pub const fn generics(&self) -> &Generics {
    &self.generics
  }

  /// Returns the flavor type of the concrete object.
  #[inline]
  pub const fn flavor_type(&self) -> &Type {
    self.flavor.ty()
  }

  /// Returns the generic unknown buffer type parameter, which will be used in generated structs or impls
  #[inline]
  pub const fn unknown_buffer_param(&self) -> &TypeParam {
    &self.unknown_buffer_param
  }

  /// Returns the lifetime generic parameter, which will be used in generated structs or impls
  #[inline]
  pub const fn lifetime_param(&self) -> &LifetimeParam {
    &self.lifetime_param
  }

  /// Returns the read buffer generic parameter, which will be used in generated `Decode` and `PartialDecode` impls
  #[inline]
  pub const fn read_buffer_param(&self) -> &TypeParam {
    &self.read_buffer_param
  }

  /// Returns the write buffer generic parameter, which will be used in generated `Encode` and `PartialDecode` impls
  #[inline]
  pub const fn write_buffer_param(&self) -> &TypeParam {
    &self.write_buffer_param
  }

  /// Returns the wire format of the concrete object.
  #[inline]
  pub const fn wire_format(&self) -> &Type {
    self.flavor.wire_format()
  }

  /// Returns the indexer information of the object.
  #[inline]
  pub const fn indexer(&self) -> &Indexer {
    &self.indexer
  }

  /// Returns the partial object information.
  #[inline]
  pub const fn partial(&self) -> &ConcretePartialObject {
    &self.partial
  }

  /// Returns the partial decoded object information.
  #[inline]
  pub const fn partial_decoded(&self) -> &ConcretePartialDecodedObject {
    &self.partial_decoded
  }

  /// Returns the selector information of the concrete object.
  #[inline]
  pub const fn selector(&self) -> &ConcreteSelector {
    &self.selector
  }

  /// Returns the selector iterator of the concrete object.
  #[inline]
  pub const fn selector_iter(&self) -> &ConcreteSelectorIter {
    &self.selector_iter
  }

  /// Returns the fields of the concrete object.
  #[inline]
  pub const fn fields(&self) -> &[ConcreteField<F>] {
    self.fields.as_slice()
  }

  /// Returns the custom metadata associated with the object.
  #[inline]
  pub const fn meta(&self) -> &M {
    &self.meta
  }

  /// Returns the default value of the concrete object, if any.
  #[inline]
  pub fn derive(&self) -> darling::Result<proc_macro2::TokenStream> {
    let default = self.derive_default()?;
    let accessors = self.derive_accessors()?;

    let indexer_defination = self.derive_indexer_defination();
    let indexer_impl = self.derive_indexer();

    let partial_def = self.partial.derive_defination(self)?;
    let partial_impl = self.partial.derive(self)?;

    let partial_decoded_def = self.derive_partial_decoded_object_defination();
    let partial_decoded_impl = self.derive_partial_decoded_object();

    let reflection_impl = self.reflection.derive(self)?;

    let selector = self.derive_selector_defination();
    let selector_impl = self.derive_selector()?;

    let selector_iter_def = self.derive_selector_iter_defination();
    let selector_iter_impl = self.derive_selector_iter();

    let decode_impl = self.derive_decode()?;

    Ok(quote! {
      #indexer_defination
      #selector
      #selector_iter_def
      #partial_def
      #partial_decoded_def

      const _: () = {
        #default

        #accessors

        #partial_impl

        #partial_decoded_impl

        #reflection_impl

        #indexer_impl

        #selector_impl

        #selector_iter_impl

        #decode_impl
      };
    })
  }

  pub(super) fn from_ast(object: ConcreteObjectAst<M, F>) -> darling::Result<Self>
  where
    M: Clone,
    F: Clone,
  {
    let mut fields = object.fields().to_vec();
    fields.sort_by_key(|f| f.tag().unwrap_or(u32::MAX));

    let fields = fields
      .iter()
      .cloned()
      .enumerate()
      .map(|(idx, f)| ConcreteField::from_ast(&object, idx, f))
      .collect::<darling::Result<Vec<_>>>()?;

    let partial = ConcretePartialObject::from_ast(&object, &fields)?;
    let partial_decoded = ConcretePartialDecodedObject::from_ast(&object, &fields)?;
    let selector = ConcreteSelector::from_ast(&object, &fields)?;
    let selector_iter = selector.selector_iter(&object)?;
    let reflection = ConcreteObjectReflection::from_ast(&object, &fields)?;

    Ok(Self {
      path_to_grost: object.path_to_grost().clone(),
      attrs: object.attrs().to_vec(),
      name: object.name().clone(),
      schema_description: object.schema_description().to_string(),
      schema_name: object.schema_name().to_string(),
      vis: object.vis().clone(),
      ty: object.ty().clone(),
      reflectable: object.reflectable().clone(),
      generics: object.generics().clone(),
      flavor: object.flavor().clone(),
      unknown_buffer_param: object.unknown_buffer_param,
      lifetime_param: object.lifetime_param,
      read_buffer_param: object.read_buffer_param,
      write_buffer_param: object.write_buffer_param,
      reflection,
      fields,
      default: object.default,
      partial,
      partial_decoded,
      selector,
      selector_iter,
      meta: object.meta,
      indexer: object.indexer,
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
