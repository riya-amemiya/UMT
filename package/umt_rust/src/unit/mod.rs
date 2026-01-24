//! Unit conversion module for UMT Rust.
//!
//! This module provides utilities for converting between different units of measurement.
//! The main component is the `UmtUnitConverter` struct which can be configured with
//! conversion ratios for any unit system.
//!
//! # Example
//!
//! ```
//! use umt_rust::unit::{UmtUnitConverter, umt_unit_converter};
//!
//! // Create a length converter with ratios relative to the base unit
//! let converter = umt_unit_converter(vec![
//!     ("meters", 1.0),       // base unit (ratio = 1)
//!     ("kilometers", 0.001), // 1 meter = 0.001 kilometers
//!     ("centimeters", 100.0), // 1 meter = 100 centimeters
//!     ("millimeters", 1000.0), // 1 meter = 1000 millimeters
//! ]);
//!
//! // Convert 5 kilometers to meters
//! let result = converter.convert(5.0, &"kilometers", &"meters");
//! assert!((result.unwrap() - 5000.0).abs() < 1e-10);
//! ```

pub mod unit_converter;

pub use unit_converter::umt_unit_converter;
pub use unit_converter::UmtUnitConverter;
