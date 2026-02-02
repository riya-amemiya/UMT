## 2025-02-18 - [Insecure Randomness in UUID Generation]
**Vulnerability:** UUID v7 implementation used `Math.random()` which is predictable.
**Learning:** The "No dependencies" policy necessitates careful implementation of cryptographic primitives using platform APIs (`globalThis.crypto`) rather than relying on external libraries or simple `Math.random` fallbacks.
**Prevention:** Always verify if `globalThis.crypto` is available and use it for any security-sensitive randomness (IDs, tokens, keys) before falling back to insecure methods.

## 2025-02-18 - [Crypto.getRandomValues Size Limit]
**Vulnerability:** Replacing `Math.random` with `crypto.getRandomValues` caused a crash for large requested sizes because the API has a 65536 byte limit (QuotaExceededError).
**Learning:** `crypto.getRandomValues` is not a drop-in replacement for loop-based `Math.random` generation when arbitrary sizes are supported.
**Prevention:** Always implement chunking (e.g. 64KB batches) when using `crypto.getRandomValues` to fill buffers of potentially unlimited size.
