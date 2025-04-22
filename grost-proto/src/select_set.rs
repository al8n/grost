#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SelectionSet<S, UT, UB> {
  selection: S,
  unknown_tags: Option<UT>,
  unknown_buffer: Option<UB>,
}

impl<S, UT, UB> SelectionSet<S, UT, UB> {
  /// Creates a new selection set with the given selection and unknown buffer.
  #[inline]
  pub const fn new(selection: S, unknown_tags: Option<UT>, unknown_buffer: Option<UB>) -> Self {
    Self { selection, unknown_tags, unknown_buffer }
  }

  /// Returns the selection set.
  #[inline]
  pub const fn selection(&self) -> &S {
    &self.selection
  }

  /// Returns the unknown tags.
  #[inline]
  pub const fn unknown_tags(&self) -> Option<&UT> {
    self.unknown_tags.as_ref()
  }

  /// Returns the unknown buffer.
  #[inline]
  pub const fn unknown_buffer(&self) -> Option<&UB> {
    self.unknown_buffer.as_ref()
  }

  /// Consumes the selection set and returns the components.
  #[inline]
  pub fn into_components(self) -> (S, Option<UT>, Option<UB>) {
    (self.selection, self.unknown_tags, self.unknown_buffer)
  }
}
