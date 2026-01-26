//! Color conversion utilities
//!
//! This module provides functions for converting between different color formats:
//! - RGBA (Red, Green, Blue, Alpha)
//! - CMYK (Cyan, Magenta, Yellow, Key/Black)
//! - HSLA (Hue, Saturation, Lightness, Alpha)
//! - Hexadecimal color codes

pub mod cmyk_to_rgba;
pub use cmyk_to_rgba::*;

pub mod hexa_to_rgba;
pub use hexa_to_rgba::*;

pub mod hsla_to_rgba;
pub use hsla_to_rgba::*;

pub mod rgba_to_cmyk;
pub use rgba_to_cmyk::*;

pub mod rgba_to_hexa;
pub use rgba_to_hexa::*;

pub mod rgba_to_hsla;
pub use rgba_to_hsla::*;
