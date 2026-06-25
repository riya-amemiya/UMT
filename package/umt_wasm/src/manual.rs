//! Hand-written wasm-bindgen adapters.
//!
//! `generated.rs` covers every umt_rust function whose signature maps onto the
//! wasm-bindgen ABI directly. Functions that take or return custom types
//! (`Value`, the validator structs, `DateTime`, color types, and so on) need an
//! adapter written by hand here, typically converting through `serde`. See
//! `doc/generated.md` for the list of functions not yet covered.
