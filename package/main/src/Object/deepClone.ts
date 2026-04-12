// Security: cap recursion depth to prevent stack overflow from deeply nested
// objects, which could be used for denial-of-service.
const MAX_CLONE_DEPTH = 100;

/**
 * Recursively clones a value.
 */
const cloneValue = (value: unknown, depth: number): unknown => {
  if (value === null || typeof value !== "object") {
    return value;
  }

  if (depth > MAX_CLONE_DEPTH) {
    throw new Error(
      `deepClone: maximum recursion depth of ${MAX_CLONE_DEPTH} exceeded`,
    );
  }

  if (Array.isArray(value)) {
    const result: unknown[] = [];
    for (const element of value) {
      result.push(cloneValue(element, depth + 1));
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
      result.set(cloneValue(k, depth + 1), cloneValue(v, depth + 1));
    }
    return result;
  }

  if (value instanceof Set) {
    const result = new Set();
    for (const v of value) {
      result.add(cloneValue(v, depth + 1));
    }
    return result;
  }

  // Plain object
  const result: Record<string, unknown> = {};
  for (const key of Object.keys(value as Record<string, unknown>)) {
    result[key] = cloneValue(
      (value as Record<string, unknown>)[key],
      depth + 1,
    );
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
  return cloneValue(value, 0) as T;
};
