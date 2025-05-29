use syn::{Ident, Type};


pub struct PartialDecodedField {
  name: Ident,
  ty: Type,
}
