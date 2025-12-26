# UMT Bug Report

Deep exploration of the UMT codebase revealed the following bugs.

## Summary

| Status | Bug Type | Count |
|--------|----------|-------|
| **Fixed** | Mathematical Logic Errors | 4 |
| **Fixed** | Empty Input Handling | 1 |
| **Fixed** | Inconsistent Behavior | 1 |
| **Fixed** | Performance Issues | 1 |
| Not Fixed (by design) | Undefined behavior | 3 |
| Potential | Edge Cases | 2 |

---

## Fixed Bugs

### 1. nCr(n, 0) Returns NaN Instead of 1 - **FIXED**
- **File**: `src/Math/nCr.ts`
- **Fix**: Now correctly returns 1 when r=0 (mathematically correct: nCr(n, 0) = 1)

### 2. nPr(n, 0) Returns NaN Instead of 1 - **FIXED**
- **File**: `src/Math/nPr.ts`
- **Fix**: Now correctly returns 1 when r=0 (mathematically correct: nPr(n, 0) = 1)

### 3. gcd(n, 0) Returns 0 Instead of n - **FIXED**
- **File**: `src/Math/gcd.ts`
- **Fix**: Now correctly returns n when one argument is 0 (mathematically correct: gcd(n, 0) = n)

### 4. reduce(0, n) Returns NaN Instead of {x: 0, y: 1} - **FIXED**
- **File**: `src/Math/reduce.ts`
- **Fix**: Now correctly returns {x: 0, y: 1} for zero numerator (0/n = 0/1)

### 5. checkFlagAlignment([]) Crashes - **FIXED**
- **File**: `src/Array/checkFlagAlignment.ts`
- **Fix**: Now returns false for empty matrix instead of crashing

### 6. padStart vs padEnd Inconsistent Behavior - **FIXED**
- **File**: `src/String/padStart.ts`
- **Fix**: Now returns original string for empty padString (consistent with padEnd)

### 7. primeFactorization() Inefficient Algorithm - **FIXED**
- **File**: `src/Math/primeFactorization.ts`
- **Fix**: Optimized from O(n) to O(sqrt(n)) using sqrt-based termination

---

## Not Fixed (By Design)

These are intentionally not fixed as they represent mathematically undefined behavior:

### factorial(-n) Returns 1
- **File**: `src/Math/factorial.ts`
- **Reason**: Factorial of negative numbers is mathematically undefined. Per project policy, errors are not explicitly thrown.

### min()/max() With No Arguments
- **Files**: `src/Math/min.ts`, `src/Math/max.ts`
- **Reason**: Empty input is invalid. Returns Infinity/-Infinity per JavaScript's Math.min/max behavior.

---

## Potential Edge Cases (Not Verified as Bugs)

### isPerfectSquare() Floating Point Precision
- **File**: `src/Validate/isPerfectSquare.ts`
- **Issue**: May fail for very large numbers due to floating point precision
- **Status**: Works correctly for tested values

### quickSort Partition Bounds
- **File**: `src/Array/quickSort.ts`
- **Issue**: Partition loops may theoretically exceed bounds
- **Status**: validateRange helper appears to prevent this

---

*Generated: 2025-12-26*
*Updated: 2025-12-26 (bugs fixed)*
*Branch: claude/bug-exploration-k27Px*
