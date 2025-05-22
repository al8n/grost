#[cfg(feature = "uuid_1")]
const _: () = {
  impl_reflectable_with_variant!(
    uuid_1::Uuid:Uuid,
  );
};
