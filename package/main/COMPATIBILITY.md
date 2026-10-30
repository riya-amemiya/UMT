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

IP helpers (`umt/IP`) are IPv4 only. TypeScript does not validate dotted-decimal strings; the Python and Rust ports raise / return `Err` on malformed addresses and reject non-contiguous subnet masks. `isPrivateIp` is RFC 1918 only (not loopback or link-local). TypeScript, Python, Rust, and Go hoist the three RFC 1918 `(network, mask)` pairs at module load.

`decodeBase32` (`umt/Crypto`) is RFC 4648 (`A-Z` + `2-7`). It skips `=` anywhere and, in TypeScript, maps invalid / lowercase characters to value `0` instead of throwing (`"1A"` → `[0]`). Python / Rust / Go error on those inputs. Wasm skips encode (generic) and decode (`Result`).

`toBaseN` (`umt/Math`) defaults to radix **2** (`toBaseN(10)` → `"1010"`). A radix outside `2`–`36` throws `RangeError` in TypeScript. Python `to_base_n` also defaults to 2. Rust `umt_to_base_n` returns the string `"Invalid radix"`. Go `ToBaseN` panics. Wasm `toBaseN` follows Rust.

`hexaToRgba` (`umt/Color`) does not validate in TypeScript (malformed input can yield `NaN` components). Python, Rust, and Go require `#` plus 3, 6, or 8 hex digits.

`unescapeHtml` (`umt/String`) in TypeScript leaves dangerous numeric character references unchanged: NULL (`0`), C0 controls except TAB / LF / CR, DEL (`127`), C1 (`128`–`159`), surrogates (`0xD800`–`0xDFFF`), and values above `0x10FFFF`. Python / Rust / Go decode numeric entities through the language code-point APIs and do not apply those extra filters.

`escapeHtml` (`umt/String`) in TypeScript also encodes backtick (`&#x60;`) and `/` (`&#x2F;`). Python `escape_html`, Rust `umt_escape_html`, and Go `EscapeHtml` encode only `& < > " '`.

`toFullWidth` (`umt/String`) converts ASCII digits and letters only (`[0-9A-Za-z]`, `+ 0xFEE0`). Ports match. `toHalfWidth` in TypeScript / Rust / Go converts full-width digits and letters only (`０-９Ａ-Ｚａ-ｚ`). Python `to_half_width` also maps `U+FF01`–`U+FF5E` (full-width punctuation) and ideographic space `U+3000` → `" "`.

`slugify` (`umt/String`) uses JavaScript `\w` (`[A-Za-z0-9_]`), so CJK and other non-ASCII letters are dropped (`"Japanese: こんにちは"` → `"japanese"`). Go `Slugify` matches that ASCII `\w` but decomposes only a fixed Latin-1 table instead of full Unicode NFD. Python `slugify` and Rust `umt_slugify` keep Unicode letters (`"japanese-こんにちは"`). Wasm `slugify` follows Rust.

`parseEmail` (`umt/Validate`) rejects strings longer than 320 characters before the RFC regex. Python has no global cap. Rust `umt_parse_email` maps `Rfc822` to the Basic pattern and `Rfc5322` to the Rfc5321 pattern. Go `ParseEmail` is basic-only and returns an error; `ParseEmailWithLevel` applies 998 / 256 limits on some RFC levels.

`mathConverter("1250*1250")` is `"1500*1000+400*100+200*100+50*50"` in TypeScript, Rust, and Go. Python `math_converter` emits float terms (`"1500.0*1000+400.0*100+200.0*100+50*50"`).

UA helpers (`umt/UA`) return coarse labels, not a full parser. Opera (`OPR/`) is `"other"`. Android without a `mobile` token is `"tablet"`. iPhone / iPad / iPod are `"ios"` even when the string contains `like Mac OS X`. Wasm skips the four UA functions (custom enums). Go wraps each field in a struct (`Browser.Name`, `OS.Name`, `Device.Type`).

`sanitizeString` (`umt/String`) keeps printable ASCII plus TAB / LF / CR. It drops CJK, emoji, and accented letters (`"héllo"` → `"hllo"`, `"日本語"` → `""`). It is not `stripAnsi`: ESC bytes are removed but CSI leftovers remain (`"\u001B[31mred\u001B[0m"` → `"[31mred[0m"`). Ports match this ASCII filter.

`joinPath` (`umt/URL`) is a slash-normalizing string join. It does not resolve `.` / `..` and does not use OS separators. Leading `/` is kept only on the first segment; trailing `/` only on the last. Empty segments are dropped. Wasm `joinPath` takes an array of strings, not rest arguments.

`escapeRegExp` (`umt/Tool`) does not escape `-` or `/`. Wasm's generated name is `escapeRegexp` (lowercase `p`); TypeScript and Go use `escapeRegExp` / `EscapeRegExp`.

## Versioning

This package follows [Semantic Versioning (SemVer)](https://semver.org/):

- **MAJOR** version: Breaking changes that may require code updates
- **MINOR** version: New features that are backward compatible
- **PATCH** version: Bug fixes that are backward compatible

Backward compatibility is maintained within the same major version.
