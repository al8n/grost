use crate::meta::SchemaFromMeta;

mod flavor;

/// The meta for the object
pub mod object;

impl From<SchemaFromMeta> for SchemaAttribute {
  fn from(meta: SchemaFromMeta) -> Self {
    Self {
      name: meta.name,
      description: meta.description,
    }
  }
}

/// The meta for the schema
#[derive(Default, Debug, Clone)]
pub struct SchemaAttribute {
  name: Option<String>,
  description: Option<String>,
}

impl SchemaAttribute {
  /// Returns the name of the schema
  pub const fn name(&self) -> Option<&str> {
    match self.name.as_ref() {
      Some(name) => Some(name.as_str()),
      None => None,
    }
  }

  /// Returns the description of the schema
  pub const fn description(&self) -> Option<&str> {
    match self.description.as_ref() {
      Some(description) => Some(description.as_str()),
      None => None,
    }
  }
}
