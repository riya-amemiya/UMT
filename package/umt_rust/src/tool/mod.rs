//! Tool module providing utility functions for functional programming patterns.
//!
//! This module contains:
//! - `create_pipeline` - A functional pipeline for chaining transformations
//! - `parse_json` - JSON parsing utilities
//! - `pipe` - A fluent pipeline builder with rich transformation methods

pub mod create_pipeline;
pub use create_pipeline::*;

pub mod parse_json;
pub use parse_json::*;

pub mod pipe;
pub use pipe::*;

pub use crate::internal::json::{JsonParseError, JsonValue};
