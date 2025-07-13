/// A state which means the current type is holding an identifier.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WithIdentifier;

/// The generic `S` can be any type representing a state.
pub trait State<S: ?Sized> {
  /// The output state type.
  type Output: ?Sized;
}
