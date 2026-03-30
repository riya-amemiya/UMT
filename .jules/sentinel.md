## 2025-02-18 - [Insecure Randomness in UUID Generation]
**Vulnerability:** UUID v7 implementation used `Math.random()` which is predictable.
**Learning:** The "No dependencies" policy necessitates careful implementation of cryptographic primitives using platform APIs (`globalThis.crypto`) rather than relying on external libraries or simple `Math.random` fallbacks.
**Prevention:** Always verify if `globalThis.crypto` is available and use it for any security-sensitive randomness (IDs, tokens, keys) before falling back to insecure methods.

## 2025-02-18 - [Crypto.getRandomValues Size Limit]
**Vulnerability:** Replacing `Math.random` with `crypto.getRandomValues` caused a crash for large requested sizes because the API has a 65536 byte limit (QuotaExceededError).
**Learning:** `crypto.getRandomValues` is not a drop-in replacement for loop-based `Math.random` generation when arbitrary sizes are supported.
**Prevention:** Always implement chunking (e.g. 64KB batches) when using `crypto.getRandomValues` to fill buffers of potentially unlimited size.
## 2025-03-02 - [CRITICAL] Fix Prototype Pollution in pickDeep
**Vulnerability:** The `pickDeep` function did not filter out sensitive object keys (`__proto__`, `constructor`, `prototype`), allowing an attacker to traverse the prototype chain and overwrite global properties (Prototype Pollution) by passing malicious paths like `__proto__.polluted`.
**Learning:** Utilities that deeply traverse and construct objects based on dynamic, user-controlled paths must explicitly block access to prototype-related keys to prevent Prototype Pollution attacks.
**Prevention:** Always validate and sanitize keys during deep object traversal or assignment, explicitly ignoring or rejecting keys like `__proto__`, `constructor`, and `prototype`.
## 2025-03-16 - Prototype Pollution in Object Utilities
**Vulnerability:** `getObjectsCommon` and `getObjectsDiff` were vulnerable to Prototype Pollution because they iterated over and copied all keys, including `__proto__`, `constructor`, and `prototype`.
**Learning:** Even simple object diffing or commonality utilities need explicit prototype pollution guards because they dynamically assign keys to a new object based on unsanitized input objects.
**Prevention:** Always explicitly check and ignore `__proto__`, `constructor`, and `prototype` keys during object iteration in any utility that dynamically constructs or merges objects.
## 2026-03-30 - Missing Input Validation in ipToBinaryString
**Vulnerability:** The `ipToBinaryString` function accepted arbitrary strings without validating octet count, numeric format, or value range (0-255). Since it is the foundational function used by `ipToLong`, `isInRange`, `isPrivateIp`, `getNetworkAddress`, and `subnetMaskToCidr`, malformed inputs could cause silent incorrect results in security-critical IP-based access control decisions.
**Learning:** Core parsing functions used by security-sensitive utilities must enforce strict input validation. Silent failures (returning garbage data instead of throwing) can lead to access control bypasses when downstream functions rely on the parsed result for allow/deny decisions.
**Prevention:** Always validate format and value ranges in IP address parsing functions. Throw descriptive errors for malformed inputs rather than silently producing incorrect results.
