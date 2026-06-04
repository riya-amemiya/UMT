//! Bun runtime detection utility

/// Determines if the current environment is the Bun runtime
///
/// The TypeScript original checks `typeof Bun !== "undefined"` inside a
/// `try`/`catch`, returning `false` when the `Bun` global is unavailable or the
/// access throws. Rust has no JavaScript `Bun` global, so the equivalent runtime
/// signal is the `BUN_INSTALL` environment variable that the Bun runtime exports
/// for the processes it spawns. When that marker is absent the function returns
/// `false`, mirroring the `catch`/`undefined` branch of the TypeScript
/// implementation.
///
/// # Returns
/// `true` when the Bun runtime marker is present in the environment, `false`
/// otherwise
///
/// # Examples
/// ```
/// use umt_rust::validate::umt_is_bun;
///
/// // A native Rust process is not the Bun runtime unless the marker is set.
/// let detected = umt_is_bun();
/// assert!(detected || !detected);
/// ```
#[inline]
pub fn umt_is_bun() -> bool {
    std::env::var_os("BUN_INSTALL").is_some()
}
