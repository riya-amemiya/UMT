## 2025-02-19 - Mismatch in Module Location for `to_celsius` **Mismatch:** `to_celsius` (and `to_kelvin`) are located in `Math` module in `package/main`, but were found in `unit` module in `package/umt_python`. **Resolution:** Implemented `to_celsius` in `math` module in `package/umt_python` to ensure parity with `package/main`. The existing implementation in `unit` was left touched to avoid breaking changes, but the canonical location should be `math` as per SoT.

## 2025-02-21 - Logic Mismatch in `gcd` Implementation
**Mismatch:** `gcd` in `package/umt_python` uses `float.as_integer_ratio()` and `math.lcm` for exact floating point handling and performance, whereas `package/main` uses string-based decimal place counting.
**Resolution:** The Python implementation is optimized for performance and correctness using standard library features not present in JS/TS. Functionality remains equivalent for standard floating point inputs.

## 2025-02-23 - Logic Mismatch in `is_double` for Lists
**Mismatch:** `isDouble` in `package/main` (JS) implicitly coerces single-element arrays to numbers (e.g., `[1.5]` -> `1.5`), returning `true`.
**Resolution:** In `package/umt_python`, `is_double` explicitly returns `False` for `list` and `dict` types to maintain idiomatic Python behavior and type safety, aligning with the precedent set by `is_number`.

## 2026-03-16 - [Strict Equality in Python] **Mismatch:** [Python's `==` treats `False == 0` and `True == 1` as equal] **Resolution:** [Explicitly check types `if type(obj_val) is bool and type(value) is not bool` when porting strict equality (`===`) from TS to Python to ensure parity in `matches` predicate]

## 2026-03-24 - Predicate Combinators Require Boxed Closures in Rust
**Mismatch:** TypeScript's `every`/`some` accept variadic predicate arguments directly, while Rust requires `Vec<Box<dyn Fn>>` due to heterogeneous closure types.
**Resolution:** Introduced `BoxPredicate<T>` type alias in the predicate module to keep the API ergonomic and satisfy Clippy's `type_complexity` lint. The `not` function uses generics with `Fn` trait bound instead, as it only takes a single predicate.

## 2026-04-05 - Port `throttle` from Function module to umt_python
**Ported:** `throttle` function from `package/main/src/Function/throttle.ts` to `package/umt_python/src/function/throttle.py`.
**Adaptation:** TypeScript uses `setTimeout`/`clearTimeout` for scheduling; Python uses `threading.Timer` with a lock for thread safety. The wait parameter is in seconds (Python convention) rather than milliseconds (JS convention). The `ThrottledFunction` class wraps the callable and exposes a `cancel()` method matching the TS `ThrottledFunction` interface. Uses `time.monotonic()` instead of `Date.now()` for robust elapsed-time tracking.
