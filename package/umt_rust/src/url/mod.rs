//! URL utility module for UMT Rust.
//!
//! This module provides URL-related utility functions:
//! - `build_url` - Build a URL with query parameters appended
//! - `is_absolute_url` - Check if a URL string is absolute (RFC 3986)
//! - `join_path` - Join multiple path segments into a normalized path
//! - `parse_query_string` - Parse a query string into a key-value map

pub mod build_url;
pub use build_url::*;

pub mod is_absolute_url;
pub use is_absolute_url::*;

pub mod join_path;
pub use join_path::*;

pub mod parse_query_string;
pub use parse_query_string::*;
