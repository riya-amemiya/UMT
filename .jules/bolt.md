## 2026-02-02 - Object Omission Strategy
**Learning:** Contrary to common V8 optimization advice, using `delete` on small-to-medium objects in Bun was measured to be significantly faster (2x-80x) than reconstructing the object using `Object.keys` and filtering.
**Action:** When optimizing object key removal, prefer `delete` unless working with extremely large objects or specific engines where dictionary mode penalty is proven to be higher than allocation cost.
