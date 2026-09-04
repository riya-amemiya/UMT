# Compatibility

## Supported Runtime Environments

This package supports the following runtime environments:

- Node.js 20.x, 22.x, 24.x, 26.x
- Bun 1.2.x

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
