//! Calculator module providing mathematical expression evaluation.
//!
//! This module contains:
//! - `calculator` - Main calculator function
//! - `calculator_initialization` - Factory for creating calculators with exchange rates
//! - `core` - Core calculation engine
//! - `convert_currency` - Currency conversion utilities
//! - `literal_expression` - Algebraic equation solver

pub mod calculator;
pub use calculator::*;

pub mod calculator_initialization;
pub use calculator_initialization::*;

pub mod convert_currency;
pub use convert_currency::*;

pub mod core;
pub use core::*;

pub mod literal_expression;
pub use literal_expression::*;
