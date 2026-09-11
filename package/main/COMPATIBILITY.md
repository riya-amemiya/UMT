# Compatibility

## Supported Runtime Environments

This package supports the following runtime environments:

- Node.js 20.x, 22.x, 24.x, 26.x
- Bun 1.2.x

CI uses `oven-sh/setup-bun` with `bun-version: latest` (not a pinned 1.2.x).

## Toolchain (devDependencies)

`typescript` is pinned to **6.0.3** in `package/main`, `package/umt_i18n`, and `package/umt_wasm`. This is a **dev** pin, not a consumer runtime requirement.

Do not merge Dependabot bumps to TypeScript 7.x until both of these work:

- `typescript-eslint` 8.69.x refuses to load under TypeScript 7.0, so `bun run lint:ci` fails.
- `ts-node` 10.9.2 cannot parse `jest.config.ts` under TypeScript 7.0, so `npx jest` on Node 20 fails.

Hold `typescript` at 6.0.3 (revert the version in those three `package.json` files if Dependabot moves it). `biome` formatting-only follow-ups from the same bump are fine.

## Module format (v5)

`umt` v5 is ESM-only (`"type": "module"`). The package `exports` map exposes `import` and `types` only — there is no `require` condition, and the CommonJS / Babel build was removed.

```ts
import { chunk, isBetween } from "umt";
import { isBetween as isBetweenDate } from "umt/Date";
```

```js
// No longer supported
const { chunk } = require("umt");
```

Subpath exports match source modules (`umt/Array`, `umt/Date`, `umt/Validate`, …). Tree-shaking follows those ESM entry points.

Input validation is the caller's responsibility. Functions do not throw on invalid arguments unless throwing is the function's purpose (for example `unwrap`).

IP helpers (`umt/IP`) are IPv4 only. TypeScript does not validate dotted-decimal strings; the Python and Rust ports raise / return `Err` on malformed addresses and reject non-contiguous subnet masks. `isPrivateIp` is RFC 1918 only (not loopback or link-local).

## Versioning

This package follows [Semantic Versioning (SemVer)](https://semver.org/):

- **MAJOR** version: Breaking changes that may require code updates
- **MINOR** version: New features that are backward compatible
- **PATCH** version: Bug fixes that are backward compatible

Backward compatibility is maintained within the same major version.
