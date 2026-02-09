# AGENTS.md

This file contains instructions and guidelines for AI agents working on the UMT (Universal Math Tool) repository.

## Overview

UMT is a collection of useful utility functions. The primary implementation and source of truth is the TypeScript package located in `package/main`. Other language implementations (`package/umt_python`, `package/umt_rust`) are ports that must maintain strict parity with the TypeScript version's behavior and API where applicable.

## General Rules

1.  **Source of Truth**: `package/main` (TypeScript) defines the expected behavior. When in doubt, consult its implementation and tests.
2.  **No TODOs**: Do not leave TODO comments in the code.
3.  **No Git Conflict Markers**: Ensure all conflict markers (`<<<<<<<`, `=======`, `>>>>>>>`) are resolved and removed. Their presence will fail CI immediately.
4.  **Verification**: Always verify changes by running the appropriate test and lint commands for the specific package.

## Package: main (TypeScript)

Located in `package/main`.

*   **Runtime**: Uses [Bun](https://bun.sh/).
*   **Setup**: `bun install`
*   **Testing**: `bun run test` (runs Jest)
*   **Linting**: `bun run lint` (runs ESLint, Biome, and TSC)
*   **Formatting**: `bun run format` (runs Biome)

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

## Algorithms & Specific Implementations
*   **String Distance**: Implement Levenshtein and similar algorithms using O(min(N, M)) space complexity (two-row strategy).
*   **Sorting**: When sorting lists with `NaN`, use a single-pass partition (valid vs. NaN) followed by sorting the valid partition.
*   **Unwrap**: `umt_unwrap` should panic with a message on `None`/`null`.
*   **Equality**: Custom equality checks should strictly distinguish `1` (int) from `True` (bool).
