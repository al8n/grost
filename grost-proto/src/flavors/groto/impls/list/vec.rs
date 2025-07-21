use std::vec::Vec;

mod packed;
mod repeated;

list!(@length Vec<T>);
list!(@flatten_state Vec<T>);
list!(@partial_state Vec<T> => Vec<T::Output>);
list!(@partial_ref_state(bytes) Vec<u8>);
list!(@partial_ref_state(packed) Vec<T>);
list!(@partial_ref_state(borrow) Vec<T>);
list!(@default_wire_format(packed) Vec<T>);
list!(@default_wire_format(repeated) Vec<T>);
list!(@default_wire_format(bytes) Vec<u8>);
list!(@selectable Vec<T>);
list!(@decode_to_packed_decoder Vec<T>);
list!(@decode_to_packed_decoder(from_bytes) Vec<u8> {
  Vec::from
});
list!(
  @encode(borrow) Vec<&'b T>
);
list!(
  @encode(bytes) Vec<u8>
);
