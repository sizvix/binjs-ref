//! Tools for manipulating byte-level data.

/// Encoding/decoding booleans.
pub mod bool;

/// Compressing/decompressing from/to common formats.
pub mod compress;

/// Encoding/decoding floating-point numbers.
pub mod float;

/// Determining the length of a stream without actually writing/storing data.
pub mod lengthwriter;

/// Serializing/deserializing traits.
pub mod serialize;

/// Encoding/decoding variable-length numbers.
pub mod varnum;
