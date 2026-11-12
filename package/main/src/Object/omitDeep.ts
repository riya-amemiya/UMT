import { unset } from "./unset";

import type { OmitDeep } from "$/object/omitDeep";
import type { PickDeepKey } from "$/object/pickDeepKey";

const omitKeys = (
  object: object,
  keys: readonly string[],
  index: number,
): object =>
  index < keys.length
    ? omitKeys(unset(object, keys[index]), keys, index + 1)
    : object;

/**
Creates a new object by deeply removing properties from the source object
based on specified keys. This is the counterpart of `pickDeep`.

Each key is removed with `unset`, so plain objects and arrays along a path are
shallow-copied and the source object is not mutated. Keys that do not exist
are ignored.

@template T - Type of the source object.
@template K - Type of property keys to remove. Must be a subset of PickDeepKey<T>.
@param {T} object - The source object to remove properties from.
@param {...K[]} keys - Property keys to remove. Can use dot notation for nested properties.
@returns {OmitDeep<T, K>} A new object without the specified properties.

@remarks
**Prototype pollution warning:** This function does not filter out
prototype-polluting keys (`__proto__`, `constructor`, `prototype`).
Sanitize user-controlled keys before calling.

@example
```typescript
const obj = { a: { b: { c: 1, d: 2 }, e: 3 }, f: 4 };
const omitted = omitDeep(obj, "a.b.c", "f");
// omitted will be { a: { b: { d: 2 }, e: 3 } }
```
*/
export const omitDeep = <
  T extends object,
  const K extends readonly PickDeepKey<T>[],
>(
  object: T,
  ...keys: K
): OmitDeep<T, K> =>
  omitKeys(unset(object, []), keys as readonly string[], 0) as OmitDeep<T, K>;
