# UMT Bug Report

Deep exploration of the UMT codebase revealed the following bugs.

## Summary

| Status | Bug Type | Count |
|--------|----------|-------|
| Confirmed | Mathematical Logic Errors | 5 |
| Confirmed | Empty Input Handling | 3 |
| Confirmed | Inconsistent Behavior | 1 |
| Minor | Performance Issues | 1 |
| Minor | Potential Edge Cases | 2 |

---

## Confirmed Bugs (Verified)

### 1. nCr(n, 0) Returns NaN Instead of 1
- **File**: `src/Math/nCr.ts:14-15`
- **Severity**: Medium
- **Issue**: `nCr(5, 0)` returns `NaN`, but mathematically nCr(n, 0) = 1 (there is exactly one way to choose 0 items from n items)
- **Current Code**:
  ```typescript
  if (n === 0 || r === 0 || n < r) {
    return Number.NaN;  // BUG: r === 0 should return 1
  }
  ```
- **Test Result**: `nCr(5, 0) = NaN` (Expected: `1`)

### 2. nPr(n, 0) Returns NaN Instead of 1
- **File**: `src/Math/nPr.ts:12-13`
- **Severity**: Medium
- **Issue**: `nPr(5, 0)` returns `NaN`, but mathematically nPr(n, 0) = 1 (one way to arrange 0 items)
- **Current Code**:
  ```typescript
  if (n === 0 || r === 0 || n < r) {
    return Number.NaN;  // BUG: r === 0 should return 1
  }
  ```
- **Test Result**: `nPr(5, 0) = NaN` (Expected: `1`)

### 3. gcd(n, 0) Returns 0 Instead of n
- **File**: `src/Math/gcd.ts:52-54`
- **Severity**: Medium
- **Issue**: `gcd(56, 0)` returns `0`, but mathematically gcd(n, 0) = n
- **Current Code**:
  ```typescript
  if (copyX === 0 || copyY === 0) {
    return 0;  // BUG: Should return the non-zero value
  }
  ```
- **Test Result**: `gcd(56, 0) = 0` (Expected: `56`)

### 4. reduce(0, n) Returns NaN Instead of {x: 0, y: 1}
- **File**: `src/Math/reduce.ts:15-17`
- **Severity**: Medium
- **Issue**: `reduce(0, 5)` returns `{x: NaN, y: NaN}`, but 0/5 in reduced form is 0/1
- **Current Code**:
  ```typescript
  if (x === 0 || y === 0) {
    return { x: Number.NaN, y: Number.NaN };  // BUG: 0 numerator should be handled
  }
  ```
- **Test Result**: `reduce(0, 5) = {x: null, y: null}` (Expected: `{x: 0, y: 1}`)

### 5. factorial(-n) Returns 1 Instead of Error/NaN
- **File**: `src/Math/factorial.ts:8`
- **Severity**: Medium
- **Issue**: `factorial(-5)` returns `1`, but factorial of negative numbers is undefined
- **Current Code**:
  ```typescript
  const limit = Math.max(1, x);  // BUG: Silently accepts negative numbers
  ```
- **Test Result**: `factorial(-5) = 1` (Expected: `NaN` or throw)

### 6. min() and max() With No Arguments
- **Files**: `src/Math/min.ts:9`, `src/Math/max.ts:9`
- **Severity**: Low
- **Issue**:
  - `min()` returns `Infinity`
  - `max()` returns `-Infinity`
- **Test Results**:
  - `min() = Infinity` (Should return `NaN` or throw)
  - `max() = -Infinity` (Should return `NaN` or throw)

### 7. checkFlagAlignment([]) Crashes
- **File**: `src/Array/checkFlagAlignment.ts:15`
- **Severity**: High
- **Issue**: Empty matrix causes crash when accessing `matrix[0].length`
- **Test Result**: Throws `undefined is not an object (evaluating 'matrix[0]...')`

### 8. padStart vs padEnd Inconsistent Behavior
- **Files**:
  - `src/String/padStart.ts:20-22` - Throws error for empty padString
  - `src/String/padEnd.ts:14-16` - Returns original string for empty padString
- **Severity**: Low
- **Issue**: Inconsistent error handling for empty `padString` parameter

---

## Performance Issues

### 9. primeFactorization() Inefficient Algorithm
- **File**: `src/Math/primeFactorization.ts:14`
- **Severity**: Performance
- **Issue**: Uses naive trial division up to original number instead of sqrt optimization
- **Current Code**:
  ```typescript
  for (let index = 2; index <= copyX; index++) {  // O(n) instead of O(sqrt(n))
  ```
- **Note**: The `factorize()` function in the same directory correctly uses `factor * factor <= remaining`

---

## Potential Edge Cases (Not Verified as Bugs)

### 10. isPerfectSquare() Floating Point Precision
- **File**: `src/Validate/isPerfectSquare.ts:16-17`
- **Severity**: Potential
- **Issue**: May fail for very large numbers due to floating point precision
- **Test Result**: Works correctly for tested values

### 11. quickSort Partition Bounds
- **File**: `src/Array/quickSort.ts:69-74`
- **Severity**: Potential
- **Issue**: Partition loops may theoretically exceed bounds, though validateRange helper appears to prevent this
- **Note**: Requires deeper investigation with adversarial inputs

---

## Recommendations

1. **High Priority**: Fix mathematical functions (nCr, nPr, gcd, reduce, factorial) to handle edge cases correctly
2. **Medium Priority**: Add empty array/input validation to checkFlagAlignment and similar functions
3. **Low Priority**: Standardize error handling between padStart and padEnd
4. **Performance**: Optimize primeFactorization to use sqrt-based termination

---

*Generated: 2025-12-26*
*Branch: claude/bug-exploration-k27Px*
