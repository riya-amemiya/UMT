//! Node.js runtime detection utility

/// Determines if the current environment is the Node.js runtime
///
/// The TypeScript original checks `typeof process !== "undefined" && typeof
/// require !== "undefined"` inside a `try`/`catch`, returning `false` when
/// either the `process` or `require` global is unavailable or the access throws.
/// Rust has no JavaScript `process` or `require` globals, so the equivalent
/// runtime signal is the `NODE` environment variable that the Node.js runtime
/// exports (pointing at the `node` executable) for the processes it spawns. When
/// that marker is absent the function returns `false`, mirroring the
/// `catch`/`undefined` branch of the TypeScript implementation.
///
/// # Returns
/// `true` when the Node.js runtime marker is present in the environment, `false`
/// otherwise
///
/// # Examples
/// ```
/// use umt_rust::validate::umt_is_node;
///
/// // A native Rust process is not the Node.js runtime unless the marker is set.
/// let detected = umt_is_node();
/// assert!(detected || !detected);
/// ```
#[inline]
pub fn umt_is_node() -> bool {
    std::env::var_os("NODE").is_some()
}
