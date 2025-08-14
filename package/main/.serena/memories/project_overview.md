# UMT Project Overview

UMT (Utility Module Toolkit) is a zero-dependency utility library written in TypeScript.

## Purpose

- Provides a collection of practical functions for arrays, math, strings, dates, cryptography, validation, etc.
- Modular structure organized by functionality, exporting individual utility functions
- Emphasizes lightweight design as a zero-dependency library

## Technology Stack

- **Language**: TypeScript (v5.8.3)
- **Runtime**: Bun (Node.js is not used)
- **Build Target**: ESNext, ESM modules
- **Test Framework**: Jest + SWC
- **Code Quality**: ESLint + Biome
- **Package Manager**: Bun

## Architecture

- Modular design: Each directory is a functional module (Array/, Math/, String/, etc.)
- Barrel exports: Each module re-exports via index.ts
- Type definitions: Shared types in /src/types/ with path alias $/\*
- Main export: /src/index.ts exports all modules
