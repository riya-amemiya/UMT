# UMT

Amemiya Riya's useful functions collection

UMT = Useful My Tools

[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/riya-amemiya/UMT)

## Implementation Policy

- No dependencies
- Implement only the minimum functionality necessary to ensure extensibility

> [!NOTE]
> This project has multiple language implementations under `package`, but most of the language implementations other than the `main` package written in TypeScript were written by AI.

## Packages

TypeScript (`package/main`) is the source of truth. Other language ports should match its behavior and API where applicable.

| Package | Language | Path | Notes |
| --- | --- | --- | --- |
| [`umt`](package/main/README.md) | TypeScript | `package/main` | ESM-only from v5. Wiki docs are generated from JSDoc. |
| [`umt-python`](package/umt_python/README.md) | Python | `package/umt_python` | Port of the TypeScript API (`snake_case`). |
| [`umt_rust`](package/umt_rust/doc/index.md) | Rust | `package/umt_rust` | Port with `umt_`-prefixed functions. Nightly toolchain (edition 2024). |
| [`umt-plugin-wasm`](package/umt_wasm/doc/index.md) | WebAssembly | `package/umt_wasm` | wasm-bindgen wrappers over `umt_rust`. |
| [`umt_i18n`](package/umt_i18n/README.md) | TypeScript | `package/umt_i18n` | Separate i18n package. |
| [`umt-go`](package/umt_go/README.md) | Go | `package/umt_go` | Partial Go port. |

## Development

Per-package commands live in [AGENTS.md](AGENTS.md). Cloud Agent bootstrap is [`.cursor/install.sh`](.cursor/install.sh).

```bash
# TypeScript (Bun inside the package flake)
cd package/main
bun install          # run inside `nix develop` if bun/node are not on PATH
bun run test
bun run lint

# Python (uv)
cd package/umt_python
make install
make test

# Rust (nightly)
cd package/umt_rust
cargo test
cargo fmt
cargo clippy
```

`package/main` npm scripts (`bun run test`, `bun run lint`, `bun run build`) invoke `nix develop --command make …`, so Nix flakes must be available for those entry points.

## Compatibility

- TypeScript `umt` v5 is ESM-only. See [package/main/COMPATIBILITY.md](package/main/COMPATIBILITY.md).
- Python requires 3.10+. CI currently runs 3.10–3.15.
