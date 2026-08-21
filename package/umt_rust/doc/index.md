# UMT Rust

A collection of useful functions that I personally created.

It is for personal use, so there may be destructive changes.

TypeScript `package/main` is the source of truth. Public functions are prefixed with `umt_`. Tests live in `tests/` (integration style), not in `src/`. CI uses the nightly toolchain (edition 2024).

```bash
cargo test
cargo fmt
cargo clippy
cargo doc --open
```

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

```rust
use chrono::{Datelike, TimeZone, Utc};
use umt_rust::date::{
    umt_add_business_days, umt_from_unix, umt_is_between, DateInclusivity, UnixTimeUnit,
};

let start = Utc.with_ymd_and_hms(2025, 4, 10, 10, 0, 0).unwrap();
let mid = Utc.with_ymd_and_hms(2025, 4, 15, 12, 0, 0).unwrap();
let end = Utc.with_ymd_and_hms(2025, 4, 20, 18, 0, 0).unwrap();
assert!(umt_is_between(&mid, &start, &end, None, DateInclusivity::Exclusive));

let friday = Utc.with_ymd_and_hms(2025, 4, 18, 9, 30, 0).unwrap();
assert_eq!(umt_add_business_days(&friday, 1, &[]).day(), 21);

let epoch = umt_from_unix(0.0, UnixTimeUnit::Second);
assert_eq!(epoch.timestamp(), 0);
```

There is no `umt_is_same` yet (TypeScript `isSame`). Use `umt_is_same_day` or compare `umt_start_of` results.

Wasm bindings for most of these Date helpers are **not** auto-generated (`DateTime<Utc>` / custom enums). See `package/umt_wasm/doc/index.md`.
