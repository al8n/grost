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

  /// Returns the fields of the concrete object.
  #[inline]
  pub const fn fields(&self) -> &[ConcreteField] {
    self.fields.as_slice()
  }

  pub(super) fn from_ast(object: ConcreteObjectAst) -> darling::Result<Self> {
    let fields = object
      .fields()
      .iter()
      .cloned()
      .map(|f| ConcreteField::from_ast(&object, f))
      .collect::<darling::Result<Vec<_>>>()?;

    let partial = ConcretePartialObject::from_ast(&object, object.partial(), &fields)?;

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
      partial_decoded: todo!(),
      selector: todo!(),
      selector_iter: todo!(),
    })
  }
}
