//! Tool module providing utility functions for functional programming patterns.
//!
//! This module contains:
//! - `create_pipeline` - A functional pipeline for chaining transformations
//! - `parse_json` - JSON parsing utilities
//! - `pipe` - A fluent pipeline builder with rich transformation methods
<<<<<<< HEAD
//! - `unwrap` - Utility to unwrap Option values with a message
||||||| 55b8153
=======
//! - `unwrap` - Utilities for unwrapping optional values
>>>>>>> d916e13f83cf5d012a5cda2956ae6890fbe99788

pub mod create_pipeline;
pub use create_pipeline::*;

pub mod parse_json;
pub use parse_json::*;

pub mod pipe;
pub use pipe::*;

pub mod unwrap;
pub use unwrap::*;
