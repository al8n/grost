// struct UserSelection {
//   age: Option<<u8 as Selectable>::Selector>,
//   name: Option<<str as Selectable>::Selector>,
//   email: Option<<str as Selectable>::Selector>,
// }

// #[derive(Debug, Copy, Clone, PartialEq, Eq)]
// pub struct SelectionSet<S, UT, UB> {
//   selector: S,
//   unknown_tags: Option<UT>,
//   buffer: Option<UB>,
// }

// impl<S, UT, UB> SelectionSet<S, UT, UB> {
//   /// Creates a new selection set with the given selection and unknown buffer.
//   #[inline]
//   pub const fn new(selector: S, unknown_tags: Option<UT>, buffer: Option<UB>) -> Self {
//     Self {
//       selector,
//       unknown_tags,
//       buffer,
//     }
//   }

//   /// Returns the selection set.
//   #[inline]
//   pub const fn selection(&self) -> &S {
//     &self.selector
//   }

//   /// Returns the unknown tags.
//   #[inline]
//   pub const fn unknown_tags(&self) -> Option<&UT> {
//     self.unknown_tags.as_ref()
//   }

//   /// Returns the unknown buffer.
//   #[inline]
//   pub const fn buffer(&self) -> Option<&UB> {
//     self.buffer.as_ref()
//   }

//   /// Consumes the selection set and returns the components.
//   #[inline]
//   pub fn into_components(self) -> (S, Option<UT>, Option<UB>) {
//     (self.selection, self.unknown_tags, self.buffer)
//   }
// }
