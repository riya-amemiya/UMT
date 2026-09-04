# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Common Development Commands

**Build and Test:**

- `cargo build` - Build the library
- `cargo build --release` - Build optimized release version
- `cargo test` - Run all tests (integration tests in `tests/`; `rust-package-ci.yml` does not run this)
- `cargo test test_name` - Run specific test
- `cargo fmt` - Format code according to rustfmt.toml
- `cargo clippy` - Run linter (`cargo clippy -- -D warnings` in CI)
- `cargo doc --open` - Generate and view documentation

Nightly toolchain, edition 2024. After adding a public `umt_*` function, regenerate wasm bindings in `package/umt_wasm` (`bun run gen`).

## Code Architecture

Port of TypeScript `package/main`. Modules are declared in `src/lib.rs`: `advance`, `array`, `async_util`, `color`, `consts`, `crypto`, `data_structure`, `date`, `error`, `function`, `ip`, `iterator`, `map`, `math`, `number`, `object`, `predicate`, `random`, `simple`, `string`, `time`, `tool`, `ua`, `unit`, `url`, `validate`.

Each module typically has:

- Individual function files (e.g. `average.rs`, `gcd.rs`)
- `mod.rs` that re-exports with `pub use`
- Matching tests under `tests/<module>/`

### Code Conventions

- Public functions are prefixed with `umt_` **except** `umt_rust::ip` (`cidr_to_long`, `ip_to_long`, …). Wasm codegen only wraps `pub fn umt_*`.
- Performance-critical functions may use `#[inline]`
- Tests must be in `tests/`, not in `src/`
- Stable Rust only (no `let_chains`)
- IP helpers return `Result` (except `long_to_ip` → `String` and `get_ip_class` → `""` on invalid input)
- Date `DateTime<Utc>` calendar fields are wall-clock values except `umt_from_unix` / `umt_to_unix`

### Testing Patterns

- Each function has a dedicated test file (`test_function_name.rs`)
- Tests include edge cases and random data
- Benchmarks are integration tests in `tests/benchmark/` using `std::time::Instant`

## 言語設定 / Language Settings

This project supports both English and Japanese:

- コード内の説明やドキュメントは英語で記述されています
- 開発者は日本語でのコミュニケーションを希望しています
- When requested, respond in Japanese
