# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

UMT (Utility Module Toolkit) is a TypeScript utility library organized into functional modules. It's a zero-dependency library that exports individual utility functions grouped by category (Array, Math, String, Date, etc.).

## Build/Test/Lint Commands

- Build: `bun run build` (TypeScript compilation with path alias resolution)
- Lint: `bun run lint` (ESLint + Biome + TypeScript type checking)
- Test all: `bun run test` (Jest with SWC transform)
- Test single file: `bun run test src/tests/unit/path/to/test.test.ts`
- Format: `bun run format` (Biome formatter)

## Architecture & Module Organization

### Source Structure (`/src/`)

- **Modular design**: Each directory is a functional module (`Array/`, `Math/`, `String/`, etc.)
- **Barrel exports**: Each module has an `index.ts` that re-exports all functions
- **Type definitions**: Shared types in `/src/types/` with path alias `$/*`
- **Main export**: `/src/index.ts` exports all modules using barrel pattern

### Test Structure (`/src/tests/`)

- **Unit tests**: `/unit/` - mirrors source structure, tests individual functions
- **Integration tests**: `/integration/` - tests cross-module functionality
- **Benchmarks**: `/benchmark/` - performance testing for array sorting algorithms

### Key Path Aliases

- `@/*` → `src/*` (internal imports)
- `$/*` → `src/types/*` (type imports)

## Code Style Guidelines

- **File naming**: `camelCase` or `PascalCase` (enforced by ESLint unicorn plugin)
- **Imports**: Ordered by builtin → external → internal → parent → sibling → index (auto-sorted by ESLint)
- **Formatting**:
  - Indentation: 2 spaces
  - Line width: 80 characters
  - No default exports (enforced by Biome `noDefaultExport`)
- **TypeScript**:
  - Strict mode enabled with all strict flags
  - Path aliases must be resolved with `tsc-alias` after compilation
  - Consistent array type notation (`T[]` preferred over `Array<T>`)
- **Error handling**:
  - Use descriptive error messages (enforced by Biome `useErrorMessage`)
  - Throw only Error objects (enforced by Biome `useThrowOnlyError`)

## Development Notes

- **Runtime**: Uses Bun for development, but outputs standard ES modules
- **Build target**: ESNext with ESM modules, compiled to `./module/`
- **No dependencies**: Main package is dependency-free (dev dependencies only)
- **Testing framework**: Jest with SWC transformer for fast TypeScript compilation
- **Coverage**: Enabled by default, excludes environment detection utilities
- **Language**: Please respond in Japanese
