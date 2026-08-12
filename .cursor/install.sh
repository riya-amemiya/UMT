#!/usr/bin/env bash
# Idempotent Cloud Agent bootstrap for the UMT monorepo.
#
# Prepares the three primary packages documented in AGENTS.md:
#   - package/main       TypeScript (Bun via Nix flake dev shell)
#   - package/umt_python Python (uv)
#   - package/umt_rust   Rust (cargo, nightly toolchain for edition 2024)
#
# Safe to run repeatedly: every toolchain check is guarded and the per-package
# steps only refresh dependencies against the committed lockfiles.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

log() { printf '\n\033[1;34m==> %s\033[0m\n' "$1"; }

# --- Nix (single-user) -------------------------------------------------------
# The main package's package.json scripts and CI drive everything through
# `nix develop`, which provides pinned bun/node/gnumake from the flake.
log "Ensuring Nix is available"
if [ ! -e "$HOME/.nix-profile/etc/profile.d/nix.sh" ]; then
  sh <(curl -L https://nixos.org/nix/install) --no-daemon --yes
fi
mkdir -p "$HOME/.config/nix"
if ! grep -qs 'experimental-features' "$HOME/.config/nix/nix.conf" 2>/dev/null; then
  echo "experimental-features = nix-command flakes" >> "$HOME/.config/nix/nix.conf"
fi
# shellcheck disable=SC1091
. "$HOME/.nix-profile/etc/profile.d/nix.sh"

# --- uv (Python) -------------------------------------------------------------
log "Ensuring uv is available"
export PATH="$HOME/.local/bin:$PATH"
if ! command -v uv >/dev/null 2>&1; then
  curl -LsSf https://astral.sh/uv/install.sh | sh
  export PATH="$HOME/.local/bin:$PATH"
fi

# --- Rust nightly (edition 2024 + clippy/rustfmt) ----------------------------
log "Ensuring Rust nightly toolchain and components"
export PATH="/usr/local/cargo/bin:$PATH"
if command -v rustup >/dev/null 2>&1; then
  rustup toolchain install nightly --profile minimal --component clippy rustfmt >/dev/null 2>&1 || \
    rustup component add clippy rustfmt --toolchain nightly
fi

# --- package/main (TypeScript / Bun) ----------------------------------------
log "Installing package/main dependencies (bun)"
( cd package/main && nix develop -c bun install )

# --- package/umt_python (Python / uv) ---------------------------------------
log "Installing package/umt_python dependencies (uv)"
( cd package/umt_python && uv sync )

# --- package/umt_rust (Rust / cargo) ----------------------------------------
log "Fetching package/umt_rust dependencies (cargo, nightly)"
( cd package/umt_rust && cargo +nightly fetch )

log "Cloud Agent environment ready"
