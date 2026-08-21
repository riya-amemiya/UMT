# UMT Wasm Package

個人的に作った便利関数集
個人用途なので破壊的変更がある可能性があります。
Wasm Packageに依存関係はありません。
JavaScriptからの呼び出しに対応しています。

`umt-plugin-wasm` wraps [`umt_rust`](../umt_rust) with wasm-bindgen. TypeScript `package/main` remains the behavioral source of truth; this package only exposes what the Rust crate can map across the wasm ABI.

## Install

```bash
yarn add umt-plugin-wasm
```

## Layout

| Path | Role |
| --- | --- |
| `codegen/` | Walks `../umt_rust/src`, finds every `pub fn umt_*`, emits wrappers |
| `src/generated.rs` | Auto-generated `#[wasm_bindgen]` functions. **Do not edit.** |
| `doc/generated.md` | Coverage report (generated + skipped). **Do not edit.** |
| `src/manual.rs` | Hand-written adapters for signatures codegen cannot emit |
| `src/lib.rs` | Re-exports `generated` then `manual` |

## Regenerating bindings

After adding or changing public `umt_*` functions in `umt_rust`:

```bash
cd package/umt_wasm
bun run gen
# equivalent:
# cargo run --manifest-path codegen/Cargo.toml
# cargo fmt
```

Commit `src/generated.rs` and `doc/generated.md` together. CI job `codegen-sync` in `.github/workflows/wasm-plugin-ci.yml` regenerates and fails on `git diff`.

The crate uses Rust edition 2024 and the nightly toolchain (same as `umt_rust`). `bun run gen` formats with `cargo +nightly fmt`.

## What gets skipped

Codegen only wraps signatures that map onto the wasm-bindgen ABI (scalars, `String`, `Vec` of those, `Result` of those). Typical skip reasons, listed in `doc/generated.md`:

- `&DateTime<Utc>` / `DateTime<Utc>` — most Date helpers including `umt_is_between`, `umt_add_business_days`, `umt_from_unix`, `umt_week_of_year`
- Custom enums such as `UnixTimeUnit`, `DateInclusivity`, `DurationUnit`
- Generics, closures, async, and reference returns (`&'static str`)

To expose a skipped function, add an adapter in `src/manual.rs` (usually converting through strings, `f64` epoch millis, or serde) and `pub use` it from `src/lib.rs`. Do not try to force it through codegen.

A few Date helpers that already use wasm-friendly types (`umt_is_leap_year`, `umt_day_of_week`, timezone offset strings) are generated automatically.

## Build and test

```bash
bun install
bun run test          # wasm-pack --target nodejs then Jest
cargo test --all
cargo clippy -- -D warnings
cargo fmt --check
```

`wasm-pack` release metadata disables `wasm-opt` so the build does not download a binaryen binary.
