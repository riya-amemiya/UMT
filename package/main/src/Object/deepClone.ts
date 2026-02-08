/**
 * Recursively clones a value.
 */
const cloneValue = (value: unknown): unknown => {
  if (value === null || typeof value !== "object") {
    return value;
  }

  if (Array.isArray(value)) {
    const result: unknown[] = [];
    for (const element of value) {
      result.push(cloneValue(element));
    }
    return result;
  }

  if (value instanceof Date) {
    return new Date(value);
  }

  if (value instanceof RegExp) {
    return new RegExp(value.source, value.flags);
  }

  if (value instanceof Map) {
    const result = new Map();
    for (const [k, v] of value) {
      result.set(cloneValue(k), cloneValue(v));
    }
    return result;
  }

  if (value instanceof Set) {
    const result = new Set();
    for (const v of value) {
      result.add(cloneValue(v));
    }
    return result;
  }

  // Plain object
  const result: Record<string, unknown> = {};
  for (const key of Object.keys(value as Record<string, unknown>)) {
    result[key] = cloneValue((value as Record<string, unknown>)[key]);
  }
  return result;
};

/**
 * Creates a deep clone of the given value.
 *
 * @param value - The value to deep clone.
 * @returns A deep clone of the input value.
 *
 * @example
 * ```typescript
 * const original = { a: 1, b: { c: 2 } };
 * const cloned = deepClone(original);
 * cloned.b.c = 99;
 * original.b.c; // still 2
 * ```
 */
export const deepClone = <T>(value: T): T => {
  if (value === null || typeof value !== "object") {
    return value;
  }
  return cloneValue(value) as T;
};
