## 2025-02-18 - [Insecure Randomness in UUID Generation]
**Vulnerability:** UUID v7 implementation used `Math.random()` which is predictable.
**Learning:** The "No dependencies" policy necessitates careful implementation of cryptographic primitives using platform APIs (`globalThis.crypto`) rather than relying on external libraries or simple `Math.random` fallbacks.
**Prevention:** Always verify if `globalThis.crypto` is available and use it for any security-sensitive randomness (IDs, tokens, keys) before falling back to insecure methods.
