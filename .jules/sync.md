## 2025-02-19 - Mismatch in Module Location for `to_celsius` **Mismatch:** `to_celsius` (and `to_kelvin`) are located in `Math` module in `package/main`, but were found in `unit` module in `package/umt_python`. **Resolution:** Implemented `to_celsius` in `math` module in `package/umt_python` to ensure parity with `package/main`. The existing implementation in `unit` was left touched to avoid breaking changes, but the canonical location should be `math` as per SoT.

## 2025-02-21 - Logic Mismatch in `gcd` Implementation
**Mismatch:** `gcd` in `package/umt_python` uses `float.as_integer_ratio()` and `math.lcm` for exact floating point handling and performance, whereas `package/main` uses string-based decimal place counting.
**Resolution:** The Python implementation is optimized for performance and correctness using standard library features not present in JS/TS. Functionality remains equivalent for standard floating point inputs.

## 2025-02-23 - Logic Mismatch in `is_double` for Lists
**Mismatch:** `isDouble` in `package/main` (JS) implicitly coerces single-element arrays to numbers (e.g., `[1.5]` -> `1.5`), returning `true`.
**Resolution:** In `package/umt_python`, `is_double` explicitly returns `False` for `list` and `dict` types to maintain idiomatic Python behavior and type safety, aligning with the precedent set by `is_number`.
