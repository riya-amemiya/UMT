//! Predicate combinators module
//! Provides higher-order functions for composing and combining predicates

mod every;
mod is_not_nullish;
mod is_nullish;
mod matches;
mod not;
mod some;

/// A boxed predicate function that takes a reference to `T` and returns `bool`
pub type BoxPredicate<T> = Box<dyn Fn(&T) -> bool>;

pub use every::*;
pub use is_not_nullish::*;
pub use is_nullish::*;
pub use matches::*;
pub use not::*;
pub use some::*;
