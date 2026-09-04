# CLAUDE.md

Guidance for Claude Code when working in `package/umt_i18n`.

**Don't use Node.js for package scripts.** Use Bun (`bun run test`, `bun run lint`, `bun run build`).

## Project Overview

`umt-i18n` is a nested-key translator (`UMT_i18n`), not a port of `package/main`. It depends on `umt` only for shared types (`DeepPartial`, `PickDeepKey`, `ShallowObjectValue`). Public API is `src/umtI18n.ts`, re-exported from `src/index.ts`.

## Commands

```bash
bun install
bun run test      # Jest + SWC, roots `src/tests`
bun run lint      # ESLint + Biome + tsc
bun run format    # Biome
bun run build     # tsc + tsc-alias → `module/`
```

## Behavior to preserve (see `src/tests/UMT_i18n.test.ts`)

- Placeholders are `{{name}}`. Missing params leave the token in the string.
- Constructor formatters are keyed by placeholder name.
- Plural suffixes `_zero` / `_one` / `_other` apply except for locales `ja`, `zh`, `ko` (those use the key as-is).
- Lookup: current locale → `fallbackLocales` → default locale → `defaultValue` or the key.
- Nested objects flatten with `.` (`nested.deep.message`).
- `t()` aliases `translate()`. `setLocale` does not change the default locale.
