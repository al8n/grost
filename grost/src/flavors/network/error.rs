use super::Network;
pub use grost_proto::EncodeError;

/// The decode error type for the [`Network`] flavor
pub type DecodeError = grost_proto::DecodeError<Network>;
