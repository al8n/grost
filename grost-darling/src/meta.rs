use darling::FromMeta;

/// The meta for the object
pub mod object;

/// The meta for the schema
#[derive(Default, Debug, Clone, FromMeta)]
pub struct SchemaMeta {
  #[darling(default)]
  name: Option<String>,
  #[darling(default)]
  description: Option<String>,
}

impl SchemaMeta {
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
