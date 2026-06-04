//! instanceof validation utility
//!
//! Checks whether a value is an instance of a specific type, mirroring the
//! TypeScript `instanceOf` validator. JavaScript decides this at runtime with
//! the `instanceof` operator; the Rust analog is a runtime type check on a
//! `&dyn Any` trait object via [`std::any::Any::is`].

use std::any::Any;

/// Determines whether a value is an instance of the type `T`.
///
/// This mirrors the TypeScript `instanceOf` validator, which checks a value
/// against a constructor at runtime. Because Rust has no class inheritance,
/// the check is concrete-type identity rather than subclass matching.
///
/// # Arguments
/// * `value` - The value to check, erased behind a `&dyn Any` trait object
///
/// # Returns
/// `true` if `value` holds a concrete `T`, `false` otherwise
///
/// # Examples
/// ```
/// use umt_rust::validate::umt_instance_of;
///
/// struct Animal {
///     name: String,
/// }
///
/// let pet = Animal {
///     name: String::from("rex"),
/// };
/// assert!(umt_instance_of::<Animal>(&pet));
/// assert!(!umt_instance_of::<String>(&pet));
/// ```
#[inline]
pub fn umt_instance_of<T: Any>(value: &dyn Any) -> bool {
    value.is::<T>()
}
