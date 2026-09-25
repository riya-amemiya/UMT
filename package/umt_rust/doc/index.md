# UMT Rust

A collection of useful functions that I personally created.

It is for personal use, so there may be destructive changes.

TypeScript `package/main` is the source of truth. Public functions are prefixed with `umt_`, except `umt_rust::ip` (unprefixed; wasm codegen ignores them). Tests live in `tests/` (integration style), not in `src/`. Nightly toolchain, edition 2024. Crate version is in `Cargo.toml` (currently 0.7.0).

```bash
cargo test
cargo fmt
cargo clippy
cargo doc --open
```

`.github/workflows/rust-package-ci.yml` runs `cargo fmt --check`, `cargo clippy -- -D warnings`, and `cargo build`. It does **not** run `cargo test`; run `cargo test` in this directory locally. Wasm CI (`package/umt_wasm`) runs `cargo test --all` for the wasm crate, not this crate's `tests/`.

## Modules

`src/lib.rs` exports:

| Module | Notes |
| --- | --- |
| `advance`, `array`, `async_util`, `color`, `consts`, `crypto` | Same grouping as TypeScript |
| `data_structure` | `LRUCache`, `TTLCache`, `PriorityQueue` |
| `date` | Calendar helpers; wasm skips most (`DateTime<Utc>`) |
| `error`, `function` | Result helpers; debounce / memoize / throttle |
| `ip` | **Not** `umt_`-prefixed. See below |
| `iterator`, `map`, `math`, `number`, `object` | |
| `predicate`, `random`, `simple`, `string`, `time`, `tool`, `ua`, `unit`, `url`, `validate` | |

Runtime dependencies include `chrono`, `regex`, `serde`, `rand`, and others listed in `Cargo.toml` (the crate is not dependency-free).

Fixed regexes are compiled once in `std::sync::LazyLock` statics (`umt_strip_ansi`, `umt_strip_tags`, `umt_words`, `umt_normalize_whitespace`, `umt_unescape_html`, `umt_is_absolute_url`, `umt_hexa_to_rgba`, `umt_format` escaped-text `[...]`, `parse_email`, UA extractors). Patterns built from caller input stay inline (`format_string`, calculator, `umt_regex_match`). Date `umt_format` is not string `format_string`. Prefer `LazyLock` over compiling the same pattern on every call.

## Date helpers

Local-time calendar helpers in `src/date/`. Week boundaries are Sunday-start. There are no UTC variants.

`DateTime<Utc>` **calendar fields** are treated as wall-clock values to match TypeScript local `Date`, except `umt_from_unix` / `umt_to_unix`, which use real epoch timestamps.

| Function | Notes |
| --- | --- |
| `umt_is_between(date, start, end, unit, inclusivity)` | `unit` is `Option<DateBoundaryUnit>`. Inclusivity is the `DateInclusivity` enum (`Exclusive` = `"()"`, default in TS/Python). There is no default argument — pass `DateInclusivity::Exclusive` explicitly. Ranges are not swapped. |
| `umt_add_business_days` / `umt_sub_business_days` | Start date is not counted. Friday + 1 is Monday. `amount == 0` returns a clone. Holidays compared with `umt_is_same_day`. |
| `umt_get_quarter` | Jan–Mar → 1, matching `umt_start_of(..., Quarter)`. |
| `umt_week_of_year` | Sunday-start week index. Week 1 contains January 1. Not ISO-8601. |
| `umt_from_unix` / `umt_to_unix` | `UnixTimeUnit::Second` or `Millisecond`. Seconds are floored. No default unit — pass `UnixTimeUnit::Second` to match TS/Python `"s"`. |
| `umt_format` / `umt_format_iso` | Token formatter. No default pattern — pass `"YYYY-MM-DDTHH:mm:ssZ"` or use `umt_format_iso`. Third argument is `timezone_offset_minutes` (TypeScript `format` reads the `Date`'s own offset). `[text]` is copied as-is. Wasm skips both (`DateTime<Utc>`). |

```rust
use chrono::{Datelike, TimeZone, Utc};
use umt_rust::date::{
    umt_add_business_days, umt_format, umt_from_unix, umt_is_between, DateInclusivity,
    UnixTimeUnit,
};

let start = Utc.with_ymd_and_hms(2025, 4, 10, 10, 0, 0).unwrap();
let mid = Utc.with_ymd_and_hms(2025, 4, 15, 12, 0, 0).unwrap();
let end = Utc.with_ymd_and_hms(2025, 4, 20, 18, 0, 0).unwrap();
assert!(umt_is_between(&mid, &start, &end, None, DateInclusivity::Exclusive));

let friday = Utc.with_ymd_and_hms(2025, 4, 18, 9, 30, 0).unwrap();
assert_eq!(umt_add_business_days(&friday, 1, &[]).day(), 21);

let epoch = umt_from_unix(0.0, UnixTimeUnit::Second);
assert_eq!(epoch.timestamp(), 0);

assert_eq!(umt_format(&friday, "YYYY-MM-DD", 0), "2025-04-18");
assert_eq!(umt_format(&friday, "[Year:] YYYY", 0), "Year: 2025");
```

There is no `umt_is_same` yet (TypeScript `isSame`). Use `umt_is_same_day` or compare `umt_start_of` results.

Wasm bindings for most of these Date helpers are **not** auto-generated (`DateTime<Utc>` / custom enums). See `package/umt_wasm/doc/index.md`.

## String helpers

Unicode-aware string utilities in `src/string/`. Behavior matches TypeScript `package/main/src/String` (verified against the Rust tests added with the `LazyLock` regex cache).

| Function | Notes |
| --- | --- |
| `umt_strip_ansi` | Removes CSI (colors, cursor) and OSC sequences. Incomplete CSI with no final byte is left unchanged (`"\u{001B}[31"` stays). Unlike a full sanitizer, other non-printables are kept. |
| `umt_strip_tags` | Deletes HTML/XML tags, repeating until stable so `"<sc<script>ript>"` becomes `""`. |
| `umt_words(s, pattern)` | Default: split on camelCase / acronym boundaries and non-letter/number separators. Pass `Some(&regex)` to return that pattern's matches instead (empty if nothing matches). |
| `umt_normalize_whitespace` | Collapses Unicode `\s+` to a single space and trims. |
| `umt_unescape_html` | Named entities plus decimal / hex numeric references. Decodes via `char::from_u32` (rejects surrogates and out-of-range). Does **not** apply TypeScript's extra NULL / C0 / DEL / C1 filters. |
| `umt_camel_case` | Lowercases only the first alphanumeric character. Does not split acronyms (`"HELLO"` → `"hELLO"`). |
| `umt_kebab_case` | Inserts dashes on case boundaries. Splits acronyms (`"XMLHttpRequest"` → `"xml-http-request"`). |

```rust
use umt_rust::string::{
    umt_camel_case, umt_kebab_case, umt_normalize_whitespace, umt_strip_ansi, umt_strip_tags,
    umt_unescape_html, umt_words,
};

assert_eq!(umt_strip_ansi("\u{001B}[31mred\u{001B}[0m"), "red");
assert_eq!(umt_strip_tags("<p>Hello <b>World</b></p>"), "Hello World");
assert_eq!(umt_words("XMLHttpRequest", None), vec!["XML", "Http", "Request"]);
assert_eq!(umt_normalize_whitespace("  hello   world \t\n foo "), "hello world foo");
assert_eq!(umt_unescape_html("Tom &amp; Jerry"), "Tom & Jerry");
assert_eq!(umt_camel_case("hello-world"), "helloWorld");
assert_eq!(umt_kebab_case("XMLHttpRequest"), "xml-http-request");
```

## Color and URL helpers

| Function | Notes |
| --- | --- |
| `umt_hexa_to_rgba` | `#` plus 3, 6, or 8 hex digits → `Rgba`. Invalid input is `Err`. Wasm skips this (`Result<Rgba, HexColorError>`). TypeScript `hexaToRgba` does not validate. |
| `umt_is_absolute_url` | RFC 3986 scheme check. Protocol-relative URLs are not absolute. Generated for wasm. |

```rust
use umt_rust::color::umt_hexa_to_rgba;
use umt_rust::url::umt_is_absolute_url;

let red = umt_hexa_to_rgba("#FF0000").unwrap();
assert_eq!((red.r, red.g, red.b, red.a), (255.0, 0.0, 0.0, 1.0));
assert!(umt_hexa_to_rgba("FF0000").is_err());
assert!(umt_is_absolute_url("https://example.com"));
assert!(!umt_is_absolute_url("//example.com"));
```

## IP helpers

IPv4 utilities in `src/ip/`. Names are **not** `umt_`-prefixed (`umt_rust::ip::cidr_to_long`, not `umt_cidr_to_long`). Most return `Result<_, String>`; `long_to_ip` returns `String`, and `get_ip_class` returns `""` for invalid input (matching TypeScript).

| Function | Notes |
| --- | --- |
| `ip_to_long` / `long_to_ip` | Pack or unpack four octets. Invalid dotted-decimal returns `Err`. Leading zeros (`"192.168.01.1"`) are rejected, same as Python. `long_to_ip` always returns `String`. |
| `cidr_to_long` / `cidr_to_subnet_mask` | Prefix `0`–`32`. `cidr > 32` is `Err`. CIDR `0` is `0`. |
| `subnet_mask_to_cidr` | Requires contiguous `1` bits then `0` bits (same as Python). TypeScript only counts set bits. |
| `is_in_range` | `(ip & mask) == (network & mask)`. |
| `is_private_ip` | RFC 1918 only: `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`. Loopback and link-local are not private. |
| `get_ip_class` | Classful first-octet lookup. `0.0.0.0` and malformed input return `""`. |
| `get_network_address` | Returns `u32`, not a dotted string. |
| `ip_to_binary_string` | 32-character `0`/`1` string. |

```rust
use umt_rust::ip::{
    cidr_to_subnet_mask, get_network_address, ip_to_long, is_in_range, is_private_ip,
    long_to_ip,
};

assert_eq!(ip_to_long("192.168.1.1").unwrap(), 3232235777);
assert_eq!(long_to_ip(3232235777), "192.168.1.1");
assert_eq!(cidr_to_subnet_mask(24).unwrap(), "255.255.255.0");
assert!(is_in_range("192.168.1.2", "192.168.1.0", 24).unwrap());
assert!(is_private_ip("10.0.0.1").unwrap());
assert!(!is_private_ip("127.0.0.1").unwrap());
assert_eq!(
    get_network_address("192.168.1.1", "255.255.255.0").unwrap(),
    3232235776
);
```

Wasm codegen only wraps `pub fn umt_*`, so none of these IP helpers appear in `umt-plugin-wasm` (they are absent from both the generated and skipped lists in `package/umt_wasm/doc/generated.md`).
