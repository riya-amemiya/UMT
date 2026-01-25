//! Cryptographic encoding and decoding utilities.
//!
//! This module provides Base32 and Base58 encoding/decoding functions.
//!
//! # Functions
//!
//! - [`umt_encode_base32`] - Encode bytes to Base32
//! - [`umt_decode_base32`] - Decode Base32 to bytes
//! - [`umt_decode_base32_to_string`] - Decode Base32 to UTF-8 string
//! - [`umt_encode_base58`] - Encode bytes to Base58
//! - [`umt_decode_base58`] - Decode Base58 to bytes
//! - [`umt_decode_base58_to_string`] - Decode Base58 to UTF-8 string

mod decode_base32;
mod decode_base32_to_string;
mod decode_base58;
mod decode_base58_to_string;
mod encode_base32;
mod encode_base58;

pub use decode_base32::{Base32Error, umt_decode_base32};
pub use decode_base32_to_string::umt_decode_base32_to_string;
pub use decode_base58::{Base58Error, umt_decode_base58};
pub use decode_base58_to_string::umt_decode_base58_to_string;
pub use encode_base32::umt_encode_base32;
pub use encode_base58::umt_encode_base58;
