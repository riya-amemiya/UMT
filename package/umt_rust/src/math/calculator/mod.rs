<<<<<<< HEAD
pub mod convert_currency;
pub use convert_currency::*;

pub mod core;
pub use core::*;

pub mod literal_expression;
pub use literal_expression::*;

pub mod calc;
pub use calc::*;

pub mod calculator_initialization;
pub use calculator_initialization::*;
||||||| 55b8153
=======
//! Calculator module providing mathematical expression evaluation.
//!
//! This module contains:
//! - `calculator` - Main calculator function
//! - `calculator_initialization` - Factory for creating calculators with exchange rates
//! - `core` - Core calculation engine
//! - `convert_currency` - Currency conversion utilities
//! - `literal_expression` - Algebraic equation solver

pub mod entry;
pub use entry::*;

pub mod calculator_initialization;
pub use calculator_initialization::*;

pub mod convert_currency;
pub use convert_currency::*;

pub mod core;
pub use core::*;

pub mod literal_expression;
pub use literal_expression::*;
>>>>>>> d916e13f83cf5d012a5cda2956ae6890fbe99788
