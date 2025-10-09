# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Common Development Commands

**Build and Test:**

- `cargo build` - Build the library
- `cargo build --release` - Build optimized release version
- `cargo test` - Run all tests
- `cargo test test_name` - Run specific test
- `cargo bench` - Run benchmarks using Criterion framework
- `cargo fmt` - Format code according to rustfmt.toml
- `cargo clippy` - Run linter

**Documentation:**

- `cargo doc --open` - Generate and view documentation

## Code Architecture

This is a mathematical utility library (UMT - Ultimate Math Tools) with two main modules:

### Module Structure

- `array/` - Array manipulation utilities (range generation, optimized sorting)
- `math/` - Mathematical functions (statistics, number theory, conversions)

Each module follows the pattern:

- Individual function files (e.g., `average.rs`, `gcd.rs`)
- `mod.rs` that re-exports all functions with `pub use`
- Comprehensive test coverage in `tests/` directory

### Code Conventions

- All public functions prefixed with `umt_` (e.g., `umt_average`, `umt_gcd`)
- Performance-critical functions use `#[inline]` attribute
- Comprehensive documentation with Arguments/Returns sections
- Zero runtime dependencies policy
- Edge case handling (NaN, empty arrays, etc.)

### Testing Patterns

- Each function has dedicated test file (`test_function_name.rs`)
- Tests include edge cases, random data, and performance validation
- Benchmarks for performance-critical functions

## 言語設定 / Language Settings

This project supports both English and Japanese:

- コード内の説明やドキュメントは英語で記述されています
- 開発者は日本語でのコミュニケーションを希望しています
- When requested, respond in Japanese
