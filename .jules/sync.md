## 2025-02-19 - Mismatch in Module Location for `to_celsius` **Mismatch:** `to_celsius` (and `to_kelvin`) are located in `Math` module in `package/main`, but were found in `unit` module in `package/umt_python`. **Resolution:** Implemented `to_celsius` in `math` module in `package/umt_python` to ensure parity with `package/main`. The existing implementation in `unit` was left touched to avoid breaking changes, but the canonical location should be `math` as per SoT.

## 2025-02-21 - Logic Mismatch in `gcd` Implementation
**Mismatch:** `gcd` in `package/umt_python` uses `float.as_integer_ratio()` and `math.lcm` for exact floating point handling and performance, whereas `package/main` uses string-based decimal place counting.
**Resolution:** The Python implementation is optimized for performance and correctness using standard library features not present in JS/TS. Functionality remains equivalent for standard floating point inputs.
