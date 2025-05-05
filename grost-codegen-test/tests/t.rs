use grost::{flavors::Network, reflection::FieldReflection};
use grost_codegen_test::user_struct::*;

#[test]
fn t() {
  let indexer = UserFieldIndexer::Age;
  let reflection = indexer.field_reflection::<Network>();
  println!("{:?} {}", reflection, size_of::<FieldReflection<Network>>());
  let tag = indexer.tag::<Network>();
  println!("{}", tag);
  let wire_type = indexer.wire_type::<Network>();
  println!("{}", wire_type);
  let identifier = indexer.identifier::<Network>();
  println!("{}", identifier);

  let s = CommentSelector::<Network>::all();
  let r = s[CommentFieldIndexer::User];
  println!("{:?}", s.without_title());
}
