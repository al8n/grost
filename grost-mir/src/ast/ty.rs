#[derive(Debug, Clone, Copy, darling::FromMeta)]
#[non_exhaustive]
pub enum Ty {
  Scalar,
  Bytes,
  String,
  Object,
  Enum,
  Union,
  Interface,
}
