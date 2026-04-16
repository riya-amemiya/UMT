# Release v2.15.0

This release delivers a major security hardening pass across the Object, String, Validate, and IP modules, introduces new Object utilities and Validate combinators, and brings sweeping performance improvements throughout the library.

## ✨ New Features

### 🔧 New Functions in Existing Modules

* **Object**: Added `has` for safe prototype-aware property existence checks via path traversal.
* **Object**: Added `keyBy` for transforming arrays into objects keyed by a specified property.
* **Object**: Added `mapValues` for mapping over object values while preserving keys.
* **Object**: Added `getObjectsCommon` for extracting shared entries between two objects.
* **Object**: Added `removePrototype`, `removePrototypeDeep`, `removePrototypeMap`, `removePrototypeMapDeep` — a suite of prototype pollution protection utilities (#775).
* **Validate**: Added `parseEmail` with built-in ReDoS mitigation via input length limiting.
* **Validate**: Added `union` and `intersection` validator combinators for composing schema validators (#785).

## 🛡️ Security Fixes

* **`merge`**: Fixed prototype pollution vulnerability that allowed `__proto__` key injection.
* **`mapKeys`, `mapValues`, `keyBy`**: Fixed prototype pollution via `[HIGH]` audit (#701).
* **`merge`, `pickDeep`, `mergeDeep`**: Added prototype pollution guards to deep Object utilities (#690).
* **`parseQueryString`**: Prevented prototype pollution via crafted query strings (#708).
* **`buildUrl`**: Added prototype pollution guard for query parameter objects (#721).
* **`getValue`**: Fixed prototype pollution in has path traversal (#730).
* **`ipToBinaryString`**: Added input validation to reject invalid IP segments (#733).
* **`uuid`**: Fixed regex injection vulnerability in UUID string validator (#742).
* **`unescapeHtml`**: Fixed unbounded regex quantifiers that enabled ReDoS (#716).
* **`escapeHtml`**: Fixed missing backtick and forward-slash escaping for XSS prevention (#745).
* **`parseJson`**: Fixed prototype pollution through crafted JSON input (#754).
* **`decodeBase58`**: Added strict character validation to reject invalid Base58 input.
* **`longToIp`**: Added input validation to prevent invalid IP address generation (#771).
* **`unescapeHtml`**: Rejected dangerous Unicode code points (`[MEDIUM]`) (#749).
* **`deepClone`**: Added recursion depth limit to prevent stack overflow attacks (#767).
* **`parseEmail`**: Added input length limit to mitigate ReDoS.

## ⚡ Performance Optimizations

* **`zip`, `zipLongest`**: Replaced functional approach with optimized loops for reduced allocations (#691, #694).
* **`arraysJoin`**: Eliminated intermediate array allocations (#699).
* **`getObjectsDiff`**: Improved uniqueness check from O(n²) to O(n) using Set (#696).
* **`levenshteinDistance`**: Replaced `Math.min` spread with inline comparisons and `charCodeAt` (#732).
* **`PriorityQueue`**: Replaced `Math.min(...array)` spread with single-pass loop (#717).
* **`trimStartCharacters`, `trimEndCharacters`**: Used Set for O(1) character lookups (#718).
* **`slugify`**: Consolidated multiple regex passes into one (#722).
* **`getValue`**: Hoisted compiled regex constant outside the loop (#724).
* **`nCr`**: Applied combinatorial symmetry to reduce multiplications (#748).
* **`standardDeviation`**: Switched to single-pass accumulation (#756).
* **`isDeepEqual`**: Replaced O(n²) `splice` with Set-based index tracking (#757).
* **`isPrimeNumber`**: Applied 6k±1 trial division for faster primality testing (#765).
* **`encodeBase32`**: Replaced string concatenation with array join (#769).
* **`mergeDeep`**: Replaced O(n) `Array.shift()` with O(1) index access (#744).
* **`longToIp`**: Rewrote with bitwise operations (#772).
* **`addition`, `subtract`, `multiplication`, `standardDeviation`**: Refactored to avoid `Array.reduce` overhead (#783).
* **`sum`**: Eliminated redundant per-element precision checks (#751).
* **`multiplication`**: Added integer fast-path to skip floating-point handling (#741).

## 🛠️ Bug Fixes

* **`mergeDeep`**: Added recursion depth limit to prevent stack overflow on deeply nested objects.
* **`decodeBase58`**: Fixed silent zero-defaulting on invalid characters — now throws on invalid input.

## 🔄 Refactoring & Maintenance

* Removed redundant input validation from internal functions to improve performance (#782).
* Updated dependencies across all packages (#780, #781, #764, #760, #739, #704).

## 🧪 Testing

* Added `coverageReporters` configuration to Jest for richer coverage output (#736).
* Added ReDoS mitigation test cases for `parseEmail` and `mergeDeep`.
* Added benchmark for `zipLongest` and `formatString`.
* Added adjacent-pivot edge case test for dual-pivot quicksort (#747).
