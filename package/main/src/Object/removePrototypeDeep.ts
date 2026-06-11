import { isPlainObject } from "@/Object/isPlainObject";

/**
 * Creates a new object with the same properties as the given object, but with
 * the prototype polluting properties removed recursively.
 * ("__proto__", "constructor", "prototype" are excluded from nested objects
 * and objects inside arrays)
 *
 * @param object - The object to remove the prototype polluting properties from.
 * @returns A new object with the prototype polluting properties removed
 * recursively.
 */
export const removePrototypeDeep = <T extends Record<string, unknown>>(
  object: T,
): T => {
  const result: Record<string, unknown> = Object.create(null);
  const stack: [
    Record<string, unknown> | unknown[],
    Record<string, unknown> | unknown[],
  ][] = [[object, result]];

  while (stack.length > 0) {
    // biome-ignore lint/style/noNonNullAssertion: stack.length > 0 guarantees pop() returns a value
    const [source, destination] = stack.pop()!;

    if (Array.isArray(source)) {
      const array = destination as unknown[];
      for (const value of source) {
        if (Array.isArray(value)) {
          const child: unknown[] = [];
          array.push(child);
          stack.push([value, child]);
        } else if (isPlainObject(value)) {
          const child: Record<string, unknown> = Object.create(null);
          array.push(child);
          stack.push([value, child]);
        } else {
          array.push(value);
        }
      }
      continue;
    }

    const target = destination as Record<string, unknown>;
    for (const key of Object.keys(source)) {
      if (key === "__proto__" || key === "constructor" || key === "prototype") {
        continue;
      }
      const value = source[key];
      if (Array.isArray(value)) {
        const child: unknown[] = [];
        target[key] = child;
        stack.push([value, child]);
      } else if (isPlainObject(value)) {
        const child: Record<string, unknown> = Object.create(null);
        target[key] = child;
        stack.push([value, child]);
      } else {
        target[key] = value;
      }
    }
  }

  return result as T;
};
