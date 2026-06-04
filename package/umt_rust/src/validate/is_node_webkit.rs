//! Node-Webkit environment detection utility

use super::is_browser::umt_is_browser;

/// Determines if the current environment is Node-Webkit (NW.js)
///
/// The TypeScript original returns `isBrowser() && isNode()`, which is true
/// only when both the browser globals (`window`, `document`) and the Node.js
/// globals (`process`, `require`) are present at once — the signature of the
/// NW.js hybrid runtime. Rust resolves the equivalent question through the
/// target configuration: `umt_is_browser` reports true only for a browser
/// (wasm32, non-wasi) target, while the Node.js side corresponds to a native
/// (non-wasm32) target. The two conditions are mutually exclusive under
/// `cfg!`, so the combined check is always false on every build target,
/// faithfully mirroring the source composition.
///
/// # Returns
/// true when the build target is both a browser and a Node environment, which
/// no single target satisfies, so this is false on every build target
///
/// # Examples
/// ```
/// use umt_rust::validate::umt_is_node_webkit;
///
/// // On a native build target this is false.
/// assert!(!umt_is_node_webkit());
/// ```
#[inline]
pub fn umt_is_node_webkit() -> bool {
    umt_is_browser() && cfg!(not(target_arch = "wasm32"))
}
