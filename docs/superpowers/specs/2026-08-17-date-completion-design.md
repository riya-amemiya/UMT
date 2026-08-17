# Date Completion Design

Date: 2026-08-17
Status: Approved for implementation (plan A)

## Goal

Fill the remaining dayjs-shaped holes in the TypeScript Date module, then port
the same behavior to Python and Rust.

Local-time semantics stay as they are today. No UTC variants.

## Functions

### `isBetween(date, start, end, unit?, inclusivity?)`

Returns whether `date` lies between `start` and `end`.

- Without `unit`, compare millisecond timestamps (`getTime()`).
- With `unit`, truncate all three dates with `startOf` first. Units are
  `DateBoundaryUnit` (`second` | `minute` | `hour` | `day` | `week` | `month`
  | `quarter` | `year`). Week boundaries are Sunday-start, matching `startOf`.
- `inclusivity` is `"()"` | `"[]"` | `"[)"` | `"(]"`. Default is `"()"`
  (exclusive on both ends), matching dayjs `isBetween`.
- Ranges are not swapped. If `start` is after `end`, the result is false.

### `addBusinessDays(date, amount, holidays?)`

Walks calendar days until `amount` business days have been counted.

- Uses `isBusinessDay` (weekends plus optional holiday list compared with
  `isSameDay`).
- `amount` may be negative. `0` returns a clone of `date` and does not snap
  to a business day.
- The starting date is not counted. Friday + 1 is Monday. Saturday + 1 is
  Monday.
- Does not mutate the input `Date`.

### `subBusinessDays(date, amount, holidays?)`

`addBusinessDays(date, -amount, holidays)`.

### `getQuarter(date)`

Returns `1` | `2` | `3` | `4` from the local month. Matches `startOf(..., "quarter")`:

- Jan–Mar → 1
- Apr–Jun → 2
- Jul–Sep → 3
- Oct–Dec → 4

### `weekOfYear(date)`

Sunday-start week index for `date`'s local year.

- Week 1 is the Sunday-start week that contains January 1 of `date`'s year.
- Computed as the number of Sunday-start weeks from that week's Sunday to
  `startOf(date, "week")`, plus one.
- Day-count rounding is used so DST does not shift the week number.

### `fromUnix(value, unit?)` / `toUnix(date, unit?)`

Unix-time conversion with an explicit unit.

- `unit` is `"s"` | `"ms"`. Default is `"s"`.
- `fromUnix` builds a `Date` from epoch seconds or milliseconds.
- `toUnix(..., "s")` is `Math.floor(date.getTime() / 1000)`.
- `toUnix(..., "ms")` is `date.getTime()`.

## Constraints

- Zero runtime dependencies.
- One runtime export per file. Shared types live in their own files
  (`dateInclusivity.ts`, `unixTimeUnit.ts`), same pattern as `durationUnit.ts`.
- No input validation. Callers pass valid dates and units.
- TypeScript in `package/main` is the source of truth.
- Python and Rust ports match this behavior. Rust keeps treating
  `DateTime<Utc>` fields as wall-clock values, except `fromUnix` / `toUnix`
  which use real epoch timestamps.

## Testing

- Unit tests per function, mirroring existing Date tests.
- dayjs compatibility coverage for `isBetween`, `getQuarter`, and unix
  conversion. `weekOfYear` is Sunday-start by design and is not required to
  match dayjs ISO week numbering.
- Python: `make test` / `make lint` / `make typecheck`.
- Rust: `cargo test` / `cargo fmt` / `cargo clippy`.
