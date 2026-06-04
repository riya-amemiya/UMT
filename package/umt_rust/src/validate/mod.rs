//! Validate module
//! Provides validation functions for various data types

pub mod any;
pub mod array;
pub mod bigint;
pub mod boolean;
pub mod core;
pub mod date;
pub mod file;
pub mod function;
pub mod map;
pub mod number;
pub mod object;
pub mod set;
pub mod string;
pub mod types;
pub mod unknown;

// Re-export core validation functions
pub use self::core::*;

// Standalone validation functions
mod instance_of;
mod is_array;
mod is_browser;
mod is_bun;
mod is_deep_equal;
mod is_dictionary_object;
mod is_double;
mod is_equal;
mod is_node;
mod is_node_webkit;
mod is_not_empty;
mod is_number;
mod is_perfect_square;
mod is_prime_number;
mod is_string;
mod is_value_nan;
mod never;
mod parse_email;
mod template_literal;

pub use instance_of::*;
pub use is_array::*;
pub use is_browser::*;
pub use is_bun::*;
pub use is_deep_equal::*;
pub use is_dictionary_object::*;
pub use is_double::*;
pub use is_equal::*;
pub use is_node::*;
pub use is_node_webkit::*;
pub use is_not_empty::*;
pub use is_number::*;
pub use is_perfect_square::*;
pub use is_prime_number::*;
pub use is_string::*;
pub use is_value_nan::*;
pub use never::*;
pub use parse_email::*;
pub use template_literal::*;
