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
| [`umt-i18n`](package/umt_i18n/README.md) | TypeScript | `package/umt_i18n` | Nested-key translator (`UMT_i18n`). Not a port of `umt`. npm name `umt-i18n`. |
| [`umt-go`](package/umt_go/README.md) | Go | `package/umt_go` | Partial Go port. Import `github.com/riya-amemiya/umt-go/src/<pkg>`. No CI workflow. |

## Development

Per-package commands live in [AGENTS.md](AGENTS.md). Cloud Agent bootstrap is [`.cursor/install.sh`](.cursor/install.sh): it installs Bun, uv, Rust nightly, and Nix for `nix fmt`, then `bun install` / `uv sync` / `cargo +nightly fetch` for `package/main`, `package/umt_python`, and `package/umt_rust`. It does not set up Go, wasm, or i18n.

Build, test, and lint use each package's native toolchain. Do not wrap those commands in `nix develop`. Nix is only used for `nix fmt` (and the path-filtered [nix-fmt.yml](.github/workflows/nix-fmt.yml) workflow).

```bash
# TypeScript (Bun)
cd package/main
bun install
bun run test
bun run lint
bun run build

# Python (uv)
cd package/umt_python
make install
make test

# Rust (nightly, edition 2024)
cd package/umt_rust
cargo test
cargo fmt
cargo clippy

# Go
cd package/umt_go
make test

# i18n
cd package/umt_i18n
bun install
bun run test

# Format Nix files
nix fmt
```

## CI and troubleshooting

Workflows are path-filtered. Typical failures:

| Check | Workflow | What to do |
| --- | --- | --- |
| Main lint / build / 100% coverage | `main-package-bun.yml` | `cd package/main && bun run lint:ci && bun run test`. Coverage must be 100% statements, branches, functions, and lines or the job fails and comments on the PR. |
| Node runtime matrix | `main-package-node.yml` | Node 20 / 22 / 24 / 26 via `npx jest --coverage`. |
| Python format / lint / typecheck / test | `python-package-ci.yml` | `cd package/umt_python && make all`. Matrix 3.10–3.15. |
| Rust format / clippy / build | `rust-package-ci.yml` | `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo build`. **Does not run `cargo test`**. Run `cd package/umt_rust && cargo test` locally. |
| Wasm codegen drift | `wasm-plugin-ci.yml` `codegen-sync` | `cd package/umt_wasm && bun run gen` and commit `src/generated.rs` + `doc/generated.md`. |
| Nix formatting | `nix-fmt.yml` | `nix fmt` at repo root and in `package/main`, `package/umt_i18n`, `package/umt_wasm`. |

There is no Go CI. `package/umt_go` is validated only by `make test` locally.

## Compatibility

- TypeScript `umt` v5 is ESM-only. See [package/main/COMPATIBILITY.md](package/main/COMPATIBILITY.md). CI runs Node 20 / 22 / 24 / 26.
- Python requires 3.10+. CI currently runs 3.10–3.15.
- Go IP / CIDR signatures are not the TypeScript ones (CIDR strings, dotted `GetNetworkAddress`). See [package/umt_go/README.md](package/umt_go/README.md).
