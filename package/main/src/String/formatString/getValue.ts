/**
 * Retrieves a value from an object using a dot-notation path with array index support.
 *
 * Supports nested properties and array access with positive/negative indices.
 *
 * @param object - The object to retrieve the value from
 * @param path - Dot-notation path (e.g., "user.name", "items[0]", "data[0].items[-1].name")
 * @returns The value at the specified path, or undefined if not found
 *
 * @example
 * // Simple property access
 * getValue({ name: "Alice" }, "name") // → "Alice"
 *
 * @example
 * // Nested property access
 * getValue({ user: { name: "Bob" } }, "user.name") // → "Bob"
 *
 * @example
 * // Array access
 * getValue({ items: ["A", "B", "C"] }, "items[0]") // → "A"
 * getValue({ items: ["A", "B", "C"] }, "items[-1]") // → "C"
 *
 * @example
 * // Complex nested access
 * getValue({ users: [{ name: "Alice" }] }, "users[0].name") // → "Alice"
 */
export function getValue(object: unknown, path: string): unknown {
  const segments: { key: string; index?: number }[] = [];

  const parts = path.split(".");

  for (const part of parts) {
    const arrayMatch = /^(.+?)\[(-?\d+)\]$/.exec(part);
    if (arrayMatch) {
      const [, key, indexString] = arrayMatch;
      segments.push({ key, index: Number(indexString) });
    } else {
      segments.push({ key: part });
    }
  }

  let current = object;

  for (const segment of segments) {
    if (typeof current !== "object" || current == null) {
      return;
    }

    current = (current as Record<string, unknown>)[segment.key];

    if (segment.index !== undefined) {
      if (!Array.isArray(current)) {
        return;
      }
      const index = segment.index;
      current = index < 0 ? current[current.length + index] : current[index];
    }
  }

  return current;
}
