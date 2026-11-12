import { get } from "./get";
import { pathSegments } from "./pathSegments";
import { set } from "./set";

/**
Sets a value at a deeply nested path from its current value, mutating the
object. Reads the current value like `get` and writes the result like `set`.

The updater receives `undefined` when the path is missing. Intermediate
segments that are missing or not objects are replaced with new objects.
An empty path array leaves the object unchanged and does not call the
updater.

@template T - Type of the source object
@param {T} object - Target object (mutated)
@param {string | string[]} path - Dot-separated string or path segments array
@param {(value: unknown) => unknown} updater - Computes the new value from the current value
@returns {T} The same object reference, for chaining

@remarks
**Prototype pollution warning:** This function does not filter out
prototype-polluting keys (`__proto__`, `constructor`, `prototype`).
Sanitize user-controlled paths before calling.

@example
update({ a: { b: 1 } }, "a.b", (value) => (value as number) + 1); // { a: { b: 2 } }
update({}, ["a", "b"], (value) => value ?? 0); // { a: { b: 0 } }
*/
export const update = <T extends object>(
  object: T,
  path: string | string[],
  updater: (value: unknown) => unknown,
): T => {
  const segments = pathSegments(path);
  return segments.length === 0
    ? object
    : set(object, segments, updater(get(object, segments)));
};
