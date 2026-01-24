//! Validate module
//! Provides validation functions for various data types

pub mod array;
pub mod boolean;
pub mod core;
pub mod number;
pub mod object;
pub mod string;

// Re-export core validation functions
pub use self::core::*;

// Standalone validation functions
mod is_array;
mod is_deep_equal;
mod is_dictionary_object;
mod is_double;
mod is_equal;
mod is_not_empty;
mod is_number;
mod is_perfect_square;
mod is_prime_number;
mod is_string;
mod is_value_nan;
mod parse_email;

pub use is_array::*;
pub use is_deep_equal::*;
pub use is_dictionary_object::*;
pub use is_double::*;
pub use is_equal::*;
pub use is_not_empty::*;
pub use is_number::*;
pub use is_perfect_square::*;
pub use is_prime_number::*;
pub use is_string::*;
pub use is_value_nan::*;
pub use parse_email::*;
