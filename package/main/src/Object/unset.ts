import { isPlainObject } from "./isPlainObject";
import { pathSegments } from "./pathSegments";

const ARRAY_INDEX_PATTERN = /^(?:0|[1-9]\d*)$/;

const hasSegment = (container: object, key: string): boolean =>
  Array.isArray(container)
    ? ARRAY_INDEX_PATTERN.test(key) && Number(key) < container.length
    : Object.hasOwn(container, key);

const unsetSegments = (value: unknown, segments: string[]): unknown => {
  if (!(Array.isArray(value) || isPlainObject(value))) {
    return value;
  }
  const copy: Record<string, unknown> | unknown[] = Array.isArray(value)
    ? [...value]
    : { ...value };
  const [key, ...rest] = segments;
  if (segments.length === 0 || !hasSegment(value, key)) {
    return copy;
  }
  if (rest.length > 0) {
    (copy as Record<string, unknown>)[key] = unsetSegments(
      (value as Record<string, unknown>)[key],
      rest,
    );
    return copy;
  }
  if (Array.isArray(copy)) {
    copy.splice(Number(key), 1);
    return copy;
  }
  return Object.fromEntries(
    Object.entries(copy).filter(([entryKey]) => entryKey !== key),
  );
};

/**
Removes the property at a deeply nested path without mutating the input.
Uses the same path parsing as `get`, `set` and `has`.

Plain objects and arrays along the path are shallow-copied, while untouched
branches keep their references. Removing an array index removes the element
and shifts the following elements. Any other value on the path, or a missing
segment, stops the traversal and leaves the rest unchanged.

@template T - Type of the source object
@param {T} object - Source object (not mutated)
@param {string | string[]} path - Dot-separated string or path segments array
@returns {T} A new object without the property at the path

@remarks
**Prototype pollution warning:** This function does not filter out
prototype-polluting keys (`__proto__`, `constructor`, `prototype`).
Sanitize user-controlled paths before calling.

@example
unset({ a: { b: 1, c: 2 } }, "a.b"); // { a: { c: 2 } }
unset({ a: [1, 2, 3] }, ["a", "1"]); // { a: [1, 3] }
unset({ a: 1 }, "x.y"); // { a: 1 }
*/
export const unset = <T extends object>(
  object: T,
  path: string | string[],
): T => unsetSegments(object, pathSegments(path)) as T;
