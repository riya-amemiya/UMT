//! Browser environment detection utility

/// Determines if the current environment is a browser
///
/// The TypeScript original checks `typeof window !== "undefined" &&
/// typeof document !== "undefined"`, which is true only when running inside a
/// web browser. Rust resolves the equivalent question at compile time through
/// the target configuration: a browser target compiles to WebAssembly with a
/// non-WASI operating system (`target_arch = "wasm32"` and `target_os` other
/// than `"wasi"`), so `window` and `document` would be present. Every other
/// target (native binaries, WASI) reports false, mirroring the original
/// environment guard.
///
/// # Returns
/// true when the build target is a browser (wasm32, non-wasi), false otherwise
///
/// # Examples
/// ```
/// use umt_rust::validate::umt_is_browser;
///
/// // On a native build target this is false.
/// let _ = umt_is_browser();
/// ```
#[inline]
pub fn umt_is_browser() -> bool {
    cfg!(all(target_arch = "wasm32", not(target_os = "wasi")))
}
