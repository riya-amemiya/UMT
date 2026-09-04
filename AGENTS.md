# AGENTS.md

This file contains instructions and guidelines for AI agents working on the UMT (Universal Math Tool) repository.

## Overview

UMT is a collection of useful utility functions. The primary implementation and source of truth is the TypeScript package located in `package/main`. Other language implementations (`package/umt_python`, `package/umt_rust`) are ports that must maintain strict parity with the TypeScript version's behavior and API where applicable.

## General Rules

1.  **Source of Truth**: `package/main` (TypeScript) defines the expected behavior. When in doubt, consult its implementation and tests.
2.  **No TODOs**: Do not leave TODO comments in the code.
3.  **No Git Conflict Markers**: Ensure all conflict markers (`<<<<<<<`, `=======`, `>>>>>>>`) are resolved and removed. Their presence will fail CI immediately.
4.  **Verification**: Always verify changes by running the appropriate test and lint commands for the specific package.
5.  **No benchmark comments**: Do not put timing numbers, "~Nx faster", ns/call, op counts, or runtime versions in source comments. Put measurements in the PR description only. Comments should explain non-obvious behavior, not recap a microbenchmark.
6.  **Nix is formatter-only**: Use `nix fmt` for Nix (and root YAML) formatting. Do not wrap build, test, or lint in `nix develop`.

## Package: main (TypeScript)

Located in `package/main`.

*   **Runtime**: Uses [Bun](https://bun.sh/).
*   **Setup**: `bun install`
*   **Testing**: `bun run test` (runs Jest)
*   **Linting**: `bun run lint` (runs ESLint, Biome, and TSC)
*   **Formatting**: `bun run format` (runs Biome)
*   **Nix**: `nix fmt` only. Scripts call `make` directly.

**Note**: The `types` directory primarily contains type definitions. Porting logic from `types` is only necessary if it corresponds to runtime logic.

## Package: umt_python (Python)

Located in `package/umt_python`.

### Tooling
*   **Manager**: Uses [uv](https://github.com/astral-sh/uv).
*   **Commands** (run from `package/umt_python`):
    *   `make test`: Runs `uv run pytest`.
    *   `make lint`: Runs `uv run ruff check` and format checks.
    *   `make format`: Runs `uv run ruff format`.
    *   `make typecheck`: Runs `uv run pyright`.

### Coding Standards & Parity
*   **Typing**:
    *   Use `int | float` (Python 3.10+ pipe syntax) for numeric arguments.
    *   Avoid `typing.Any`; use `object` if necessary (Ruff `ANN401`).
    *   Explicitly check `isinstance(n, bool)` and return `False` in numeric validation functions, as Python treats `bool` as `int`.
*   **Math**:
    *   Use `decimal.Decimal` (initialized from strings) for exact arithmetic to match JavaScript's behavior and avoid floating-point errors.
    *   Use `math.isqrt` for integer square roots.
    *   When using `math.comb` or `math.perm`, explicitly cast results to `float` and ensure `NaN` handling matches the TS implementation.
*   **Linting (Ruff)**:
    *   `SIM108`: Use ternary operators for simple conditionals.
    *   `UP035`: Import from `collections.abc` instead of `typing` (e.g., `Callable`, `Iterable`).
    *   `PERF203`: Avoid `try-except` blocks inside loops.
*   **Testing & Benchmarks**:
    *   Unit tests must import from `src` (e.g., `from src.validate import ...`).
    *   Benchmarks reside in `tests/benchmark/` and use `timeit`. Note that `__file__` is not defined in `timeit` strings; resolve paths externally.

## Package: umt_rust (Rust)

Located in `package/umt_rust`.

### Tooling
*   **Manager**: Cargo.
*   **Commands**:
    *   `cargo test`: Run unit and integration tests.
    *   `cargo fmt`: Format code (required for CI).
    *   `cargo clippy`: Run lints.

### Coding Standards & Parity
*   **Core Logic**:
    *   **Stable Rust Only**: Do not use unstable features like `let_chains`.
    *   **Value Enum**: Use `umt_rust::object::Value` with `#[serde(untagged)]` for JSON interoperability. Use the `obj!` macro for construction.
    *   **Regex**: Use `std::sync::OnceLock` for regex compilation in loops.
    *   **Math**:
        *   Implement `apply_currency_exchange` for currency conversion.
        *   Operator precedence: Exp > Mul/Div > Add/Sub.
        *   Rounding: Explicitly round floating-point results (e.g., `(val * 1e10).round() / 1e10`) before string conversion to match TS precision.
*   **Clippy & Lints**:
    *   `for_kv_map`: Iterate over `.keys()` or `.values()` if only one is needed.
    *   `manual_strip`: Use `str::strip_prefix` instead of `starts_with` and slicing.
    *   `module-inception`: File names should match `package/main` (e.g., `calculator.rs`) even if it triggers this lint (allow if necessary, but prefer structural fixes).
*   **Testing**:
    *   Tests must be in the `tests/` directory (integration style), not in `src/`.
    *   Benchmarks are integration tests in `tests/benchmark/` using `std::time::Instant`.
    *   Integration tests in subdirectories must be registered in a root test file (e.g., `tests/integration/mod.rs`).

## Package: umt_wasm (WebAssembly)

Located in `package/umt_wasm`. Auto-generated wasm-bindgen wrappers over `umt_rust`.

* **Generate bindings**: from `package/umt_wasm`, `bun run gen` (or `cargo run --manifest-path codegen/Cargo.toml` then `cargo fmt`).
* **Do not edit** `src/generated.rs` or `doc/generated.md` by hand. After changing `umt_rust` public `umt_*` functions, regenerate and commit both files.
* Codegen only considers `pub fn umt_*`. Modules that omit the prefix (currently `umt_rust::ip`) are invisible — they are neither generated nor listed as skipped.
* Functions whose signatures are not wasm-bindgen-friendly (`DateTime<Utc>`, custom enums, generics, closures) are listed as skipped in `doc/generated.md`. Hand-written adapters go in `src/manual.rs`.
* **CI**: `.github/workflows/wasm-plugin-ci.yml` includes a `codegen-sync` job that fails if generated files drift. Build and test run with cargo / bun; Nix is not used.

## Package: umt_go (Go)

Located in `package/umt_go`. Partial port. Module path `github.com/riya-amemiya/umt-go`; import `github.com/riya-amemiya/umt-go/src/<pkg>` (for example `src/math`, `src/ip`). Go 1.24.1. No GitHub Actions workflow — run Makefile targets locally.

* `make test`: `go test -v -race ./...`
* `make fmt` / `make check` / `make build`
* Tests live under `src/tests/<pkg>/` and import the `src/` packages.
* IP signatures differ from TypeScript: `CidrToLong` takes `"network/prefix"` and returns `[start, end]`; `IsInRange(ip, "network/prefix")`; `GetNetworkAddress` returns a dotted string; `LongToIp` / `CidrToSubnetMask` panic on invalid input.
* Date helpers include `StartOf` / `EndOf` / `AddDuration` / `IsBusinessDay`. There is no `IsBetween`, `AddBusinessDays`, or unix conversion.

## Package: umt_i18n (TypeScript)

Located in `package/umt_i18n`. Nested-key translator (`UMT_i18n`), not a port of `package/main`. npm name `umt-i18n`. Depends on `umt` for types only.

* **Setup**: `bun install`
* **Testing**: `bun run test` (Jest, `src/tests`)
* **Linting**: `bun run lint`
* Placeholders are `{{name}}`. Plural suffixes `_zero` / `_one` / `_other` except locales `ja` / `zh` / `ko`. Lookup: current → fallback locales → default locale → `defaultValue` or the key.

## Algorithms & Specific Implementations
*   **String Distance**: Implement Levenshtein and similar algorithms using O(min(N, M)) space complexity (two-row strategy).
*   **Sorting**: When sorting lists with `NaN`, use a single-pass partition (valid vs. NaN) followed by sorting the valid partition.
*   **Unwrap**: `umt_unwrap` should panic with a message on `None`/`null`.
*   **Equality**: Custom equality checks should strictly distinguish `1` (int) from `True` (bool).
