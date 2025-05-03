use super::Network;

wire_type!(
  enum WireType<Network> {
    /// The zero-sized type wire format, this wire format requires no information included.
    "zst" = 0,
    /// The varint encoding/decoding wire format
    "varint" = 1,
    /// The length-delimited encoding/decoding wire format
    "length-delimited" = 2,
    /// The fixed 8-bit length encoding/decoding wire format
    "fixed8" = 3,
    /// The fixed 16-bit length encoding/decoding wire format
    "fixed16" = 4,
    /// The fixed 32-bit length encoding/decoding wire format
    "fixed32" = 5,
    /// The fixed 64-bit length encoding/decoding wire format
    "fixed64" = 6,
    /// The fixed 128-bit length encoding/decoding wire format
    "fixed128" = 7,
  }
);
