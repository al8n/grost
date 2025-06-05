use quote::{ToTokens, quote};
use syn::{Attribute, Generics, Ident, Path, Type, Visibility};

use crate::ast::{FlavorAttribute, object::ConcreteObject as ConcreteObjectAst};

pub use field::*;
pub use partial::*;
pub use partial_decoded::*;
pub use selector::*;

mod field;
mod partial;
mod partial_decoded;
mod selector;

#[derive(Debug, Clone)]
pub struct ConcreteObject {
  path_to_grost: Path,
  attrs: Vec<Attribute>,
  name: Ident,
  vis: Visibility,
  ty: Type,
  reflectable: Type,
  generics: Generics,
  flavor: FlavorAttribute,
  fields: Vec<ConcreteField>,
  default: Option<Path>,
  partial: ConcretePartialObject,
  partial_decoded: ConcretePartialDecodedObject,
  selector: ConcreteSelector,
  selector_iter: ConcreteSelectorIter,
}

impl ToTokens for ConcreteObject {
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

impl ConcreteObject {
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
  pub const fn fields(&self) -> &[ConcreteField] {
    self.fields.as_slice()
  }

  /// Returns the default value of the concrete object, if any.
  #[inline]
  pub fn derive(&self) -> darling::Result<proc_macro2::TokenStream> {
    let default = self.derive_default()?;

    Ok(quote! {
      const _: () = {
        #default
      };
    })
  }

  pub(super) fn from_ast(object: ConcreteObjectAst) -> darling::Result<Self> {
    let fields = object
      .fields()
      .iter()
      .cloned()
      .map(|f| ConcreteField::from_ast(&object, f))
      .collect::<darling::Result<Vec<_>>>()?;

    let partial = ConcretePartialObject::from_ast(&object, &fields)?;
    let partial_decoded = ConcretePartialDecodedObject::from_ast(&object, &fields)?;
    let selector = ConcreteSelector::from_ast(&object, &fields)?;
    let selector_iter = selector.selector_iter(&object)?;

    Ok(Self {
      path_to_grost: object.path_to_grost().clone(),
      attrs: object.attrs().to_vec(),
      name: object.name().clone(),
      vis: object.vis().clone(),
      ty: object.ty().clone(),
      reflectable: object.reflectable().clone(),
      generics: object.generics().clone(),
      flavor: object.flavor().clone(),
      fields,
      default: object.default().cloned(),
      partial,
      partial_decoded,
      selector,
      selector_iter,
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
            #default()
          }
        }
      })
    } else {
      let fields = self.fields().iter().map(|f| {
        let name = f.name();
        let default = f.default();
        quote! {
          #name: #default()
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
